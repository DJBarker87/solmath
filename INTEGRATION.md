# SolMath Integration Guide

Copy-paste starting points for Solana programs. Keep decimal parsing off-chain:
clients/tests can use `solmath::fp("0.05")`, but on-chain instructions should
receive already-validated integers scaled by `SCALE = 1_000_000_000_000`.

## Dependencies

```toml
[dependencies]
anchor-lang = "0.32"
solmath = { version = "0.2", default-features = false, features = ["transcendental"] }
```

For pool math and token conversion, use:

```toml
[dependencies]
anchor-lang = "0.32"
solmath = { version = "0.2", default-features = false, features = ["pool"] }
```

American KBI and exponential NIG can be linked independently:

```toml
solmath = { version = "0.2", default-features = false, features = ["american-kbi"] }
# or: features = ["nig"]
```

## Shared Error Mapping

Use one mapper at the program boundary so every numerical outcome becomes a
stable program error.

```rust
use anchor_lang::prelude::*;
use solmath::SolMathError;

#[error_code]
pub enum MathIntegrationError {
    #[msg("input outside the supported mathematical domain")]
    MathDomain,
    #[msg("fixed-point arithmetic overflow")]
    MathOverflow,
    #[msg("division by zero")]
    MathDivisionByZero,
    #[msg("iterative solver did not converge")]
    MathNoConvergence,
    #[msg("slippage limit exceeded")]
    SlippageExceeded,
}

pub fn map_math_error(err: SolMathError) -> anchor_lang::error::Error {
    match err {
        SolMathError::DomainError => MathIntegrationError::MathDomain.into(),
        SolMathError::Overflow => MathIntegrationError::MathOverflow.into(),
        SolMathError::DivisionByZero => MathIntegrationError::MathDivisionByZero.into(),
        SolMathError::NoConvergence => MathIntegrationError::MathNoConvergence.into(),
    }
}
```

## Validate Inputs Once (Recommended)

The raw pricing functions accept integer inputs and return domain/arithmetic
errors through `Result`. The
`checked` module goes further: it makes the valid financial domain a **type**.
Validate untrusted instruction data once at your program boundary into
`Price` / `Rate` / `Vol` / `Time`, bundle them, and every downstream pricing
call is guaranteed not to panic or silently wrap—the internal bounds the
kernels assume are established by construction. A degenerate-but-in-range input
returns its normal `Err` variant without aborting the process.

```rust
use solmath::{EuropeanInputs, ImpliedVolInputs};

// At the boundary: one fallible validation of raw instruction data.
let quote = EuropeanInputs::from_raw(s, k, r, sigma, t).map_err(map_math_error)?;
let greeks = quote.full().map_err(map_math_error)?; // cannot panic
let call = greeks.call;

// Implied volatility from an observed price, same pattern.
let iv = ImpliedVolInputs::from_raw(market_price, s, k, r, t)
    .map_err(map_math_error)?
    .solve()
    .map_err(map_math_error)?;
```

The same pattern covers the other value-bearing entry points:

```rust
use solmath::{BarrierInputs, PoolSwapInputs, TwapInputs};
use solmath::barrier::BarrierType;

let knock = BarrierInputs::from_raw(s, k, h, r, sigma, t).map_err(map_math_error)?;
let out = knock.price(true, BarrierType::DownAndOut).map_err(map_math_error)?; // cannot panic

// TWAP validity is relational too: averaging_time <= t, and the fixed
// average/weight must describe one coherent observation state.
let twap = TwapInputs::from_raw(
    s, k, r, q, sigma, t, averaging_time, fixed_average, fixed_weight,
)
.map_err(map_math_error)?
.price()
.map_err(map_math_error)?;

// Pool validity is relational (weight ratio ≤ 20, post-trade balance ratio ≥ 0.01),
// so from_raw checks the whole certified domain at once.
let (net_out, fee) = PoolSwapInputs::from_raw(
    balance_in, balance_out, weight_in, weight_out, amount_in, fee_rate,
)
.map_err(map_math_error)?
.quote()
.map_err(map_math_error)?;
```

Certified domain (generous — rescale larger economics homogeneously):
`Price ≤ 100,000`, `Rate ≤ 1000%`, `0 < Vol ≤ 10000%`, `0 < Time ≤ 100` years;
pool swaps use the kernel's relational domain. The no-panic guarantee over the
whole domain is verified continuously by the `tests/checked_layer.rs` sweep
(European, IV, barrier, TWAP, and pool), which CI runs in a scaled soak with overflow
checks on. Prefer this layer for value-bearing paths; drop to the raw functions
only when you need an input outside the certified box and have your own bound.

## 1. Options Quote

Use `bs_full_hp` when accuracy matters. Final-artifact benchmark: 113,177 CU
average and 149,925 max for price + all 5 Greeks. A quote-only instruction fits under the
default 200K CU budget; request 250K+ if the same instruction also settles,
writes multiple accounts, or performs CPIs.

Client-side budget:

```ts
import { ComputeBudgetProgram } from "@solana/web3.js";

const budgetIx = ComputeBudgetProgram.setComputeUnitLimit({ units: 200_000 });
```

Instruction body:

```rust
use anchor_lang::prelude::*;
use solmath::bs_full_hp;

#[account]
pub struct OptionQuote {
    pub call: u128,
    pub put: u128,
    pub call_delta: i128,
    pub put_delta: i128,
    pub gamma: i128,
    pub vega: i128,
    pub call_theta: i128,
    pub put_theta: i128,
    pub call_rho: i128,
    pub put_rho: i128,
}

impl OptionQuote {
    pub const LEN: usize = 2 * 16 + 8 * 16;
}

#[derive(Accounts)]
pub struct QuoteOption<'info> {
    #[account(mut)]
    pub quote: Account<'info, OptionQuote>,
}

pub fn quote_option(
    ctx: Context<QuoteOption>,
    spot: u128,
    strike: u128,
    risk_free_rate: u128,
    volatility: u128,
    years_to_expiry: u128,
) -> Result<()> {
    let greeks = bs_full_hp(
        spot,
        strike,
        risk_free_rate,
        volatility,
        years_to_expiry,
    )
    .map_err(map_math_error)?;

    let quote = &mut ctx.accounts.quote;
    quote.call = greeks.call;
    quote.put = greeks.put;
    quote.call_delta = greeks.call_delta;
    quote.put_delta = greeks.put_delta;
    quote.gamma = greeks.gamma;
    quote.vega = greeks.vega;
    quote.call_theta = greeks.call_theta;
    quote.put_theta = greeks.put_theta;
    quote.call_rho = greeks.call_rho;
    quote.put_rho = greeks.put_rho;

    Ok(())
}
```

### American quote with KBI

For an American settlement mark, enable `american-kbi` and pass only the six
scaled market/contract inputs. The boundary and Kim early-exercise-premium
integral are evaluated in the instruction; there is no operator account or
off-chain quote payload.

```rust
use solmath::{american_kbi_price, AmericanKbiKind};

let price = american_kbi_price(
    spot,
    strike,
    risk_free_rate,
    dividend_yield,
    volatility,
    years_to_expiry,
    AmericanKbiKind::Put,
)
.map_err(map_math_error)?;
```

The retained full-instruction maxima are 390,628 CU for calls and 390,786 CU
for puts. A 400K limit covers the measured quote-only instruction; add measured
headroom for account writes, oracle reads, or CPIs. The parameter domain and
QuantLib QdFp evidence are in `docs/AMERICAN_KBI.md`.

### Exponential NIG quote

Enable `nig` to return the European call/put pair, quote-local numerical
allowance, and selected execution tier in one call:

```rust
use solmath::{nig_price_certified, NigParams, SCALE};

let quote = nig_price_certified(
    100 * SCALE,
    100 * SCALE,
    50_000_000_000,       // rate = 5%
    20_000_000_000,       // dividend yield = 2%
    SCALE,
    NigParams {
        alpha: 15 * SCALE,
        beta: -2 * SCALE as i128,
        delta_per_year: SCALE,
    },
    5_000_000_000,        // requested absolute error = 0.005
)
.map_err(map_math_error)?;
```

`quote.tier` distinguishes expiry, the inexpensive Chernoff tail, and the full
15-node quadrature path. The current deployed maximum is 382,441 CU; the full
domain and accuracy evidence are in `docs/NIG.md`.

## 2. Weighted Pool Swap Quote

Use `weighted_pool_swap` for Balancer-style weighted pools. It returns
`(net_out, fee)` at `SCALE`. Convert raw token amounts to fixed-point before
calling it, and convert the output back to raw token units with floor rounding.
The safe power-error proof requires the post-trade balance ratio to be at least
1% and `weight_in / weight_out <= 20`; unsupported shapes return
`DomainError`.

Compute budget: the swap path uses one HP power calculation. Start with the
default 200K CU budget for quote-only instructions, and benchmark your full
instruction if you add token CPIs or multi-hop routing.
The final math-only sweep over 900 certified-domain cases averaged 25,608 CU
and maxed at 33,685 CU.

Client-side budget:

```ts
import { ComputeBudgetProgram } from "@solana/web3.js";

const budgetIx = ComputeBudgetProgram.setComputeUnitLimit({ units: 200_000 });
```

Instruction body:

```rust
use anchor_lang::prelude::*;
use solmath::{fp_to_token_floor, token_to_fp, weighted_pool_swap};

#[account]
pub struct SwapQuote {
    pub net_out_raw: u64,
    pub fee_raw: u64,
}

#[derive(Accounts)]
pub struct QuoteWeightedSwap<'info> {
    #[account(mut)]
    pub quote: Account<'info, SwapQuote>,
}

#[allow(clippy::too_many_arguments)]
pub fn quote_weighted_swap(
    ctx: Context<QuoteWeightedSwap>,
    balance_in_raw: u64,
    balance_out_raw: u64,
    amount_in_raw: u64,
    token_in_decimals: u8,
    token_out_decimals: u8,
    weight_in: u128,
    weight_out: u128,
    fee_rate: u128,
    min_out_raw: u64,
) -> Result<()> {
    let balance_in = token_to_fp(balance_in_raw, token_in_decimals).map_err(map_math_error)?;
    let balance_out = token_to_fp(balance_out_raw, token_out_decimals).map_err(map_math_error)?;
    let amount_in = token_to_fp(amount_in_raw, token_in_decimals).map_err(map_math_error)?;

    let (net_out, fee) = weighted_pool_swap(
        balance_in,
        balance_out,
        weight_in,
        weight_out,
        amount_in,
        fee_rate,
    )
    .map_err(map_math_error)?;

    let net_out_raw = fp_to_token_floor(net_out, token_out_decimals).map_err(map_math_error)?;
    let fee_raw = fp_to_token_floor(fee, token_out_decimals).map_err(map_math_error)?;

    require!(
        net_out_raw >= min_out_raw,
        MathIntegrationError::SlippageExceeded
    );

    let quote = &mut ctx.accounts.quote;
    quote.net_out_raw = net_out_raw;
    quote.fee_raw = fee_raw;

    Ok(())
}
```

## 3. Token Conversion And Rounding Policy

Use floor when the protocol pays users, and ceil when the protocol collects
fees, repayments, or penalties. This prevents dust from leaking against the
protocol. These helpers are cheap; no compute-budget increase is needed for
conversion-only instructions.

Instruction body:

```rust
use anchor_lang::prelude::*;
use solmath::{fp_to_token_ceil, fp_to_token_floor, token_to_fp};

#[account]
pub struct SettlementAmounts {
    pub deposit_fp: u128,
    pub user_payout_raw: u64,
    pub protocol_fee_raw: u64,
}

#[derive(Accounts)]
pub struct SettleAmounts<'info> {
    #[account(mut)]
    pub amounts: Account<'info, SettlementAmounts>,
}

pub fn settle_amounts(
    ctx: Context<SettleAmounts>,
    deposit_raw: u64,
    payout_fp: u128,
    fee_fp: u128,
    token_decimals: u8,
) -> Result<()> {
    let deposit_fp = token_to_fp(deposit_raw, token_decimals).map_err(map_math_error)?;

    let user_payout_raw =
        fp_to_token_floor(payout_fp, token_decimals).map_err(map_math_error)?;
    let protocol_fee_raw =
        fp_to_token_ceil(fee_fp, token_decimals).map_err(map_math_error)?;

    let amounts = &mut ctx.accounts.amounts;
    amounts.deposit_fp = deposit_fp;
    amounts.user_payout_raw = user_payout_raw;
    amounts.protocol_fee_raw = protocol_fee_raw;

    Ok(())
}
```

## Compute Budget Summary

| Use case | Suggested limit | Notes |
|----------|-----------------|-------|
| `bs_full_hp` quote | 200K quote-only, 250K+ with extra logic | Benchmarked 113,177 avg / 149,925 max. |
| `american_kbi_price` | 400K quote-only; more with account/CPI work | Fully on-chain boundary and premium integration. Full-instruction call/put maxima 390,628/390,786 CU on 2,000 quotes per leg. |
| `nig_price_certified` | 400K quote-only | 2K deployed sweep: 367,321 P99 / 381,385 max math CU and 382,441 max full-instruction CU. Deep Chernoff-tail quotes are much cheaper; domain/error-budget misses return `SolMathError`. |
| Weighted pool quote | 200K quote-only | Benchmark if combined with token CPIs or routing. |
| Token conversion | Default budget | Helpers are simple integer conversions. |
| `exp_fixed_i` | Default budget | 961 average / 992 max CU. Certified relative error is <1.55×10^-16; absolute raw error grows with the result. |
| `barrier_option_with_state` | 500K minimum | Math-only max 415,579 CU. Persist historical breach state. |
| `twap_option_price` | Fits the default 200K benchmark instruction | 100K production + 10K adversarial accuracy calls complete without rejection; deviations are reported in `benchmark/asian_accuracy_report.json`. 2K practical CU calls: 137,997 average / 180,029 P99 / 182,458 math max. The 10K adversarial CU sweep maxed at 185,590 math / 186,610 complete-instruction CU. Persist authenticated fixing state; extra oracle/account/CPI work needs added budget. |
| `implied_vol` | 500K minimum | Fresh math-only P99 282,132 / max 328,660 CU; 17/2,000 sampled inputs returned an expected error. |
| deterministic `heston_price` (`xi = 0`) | 250K minimum | Fresh 2K math-only max 183,239 CU; the broader 2,004-case grid maxed at 190,756. The API returns `NoConvergence` for `xi > 0`. |
| guarded `sabr_price` / `sabr_greeks` | 700K minimum | Fresh accepted-case math-only maxima 603,172 / 603,160 CU; rejected shapes return an error. |
| `bvn_cdf` / `bvn_cdf_hp` | 250K / 550K minimum | Conservative broader branch-grid maxima remain 208,693 / 468,417 CU; the fresh 2K exp-affected samples measured 100,498 / 248,294 average and 135,944 / 307,310 max. |
| `Phi2Table::eval` | Default budget | Runtime-backed math-only max 1,440 CU. |

These are SolMath-call measurements, not complete instruction budgets. Add
headroom for Anchor dispatch, account serialization, logs, oracle work, and
CPIs, and gate regressions against your deployed artifact. NIG exposes its
documented domain and requested-error contract through `DomainError` and
`NoConvergence`. `heston_price` implements the deterministic positive-expiry
`xi == 0` limit.

## Required release profile

SolMath returns arithmetic failures as `SolMathError`, but an on-chain
consumer should still keep Rust overflow checks enabled as defense in depth.
Add this to the **workspace-root** `Cargo.toml` (a dependency cannot impose
the setting on its caller):

```toml
[profile.release]
overflow-checks = true
```

CI should test release builds both with checks enabled and disabled; SolMath's
own workflow does this to catch profile-dependent behavior.
