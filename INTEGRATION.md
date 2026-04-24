# SolMath Integration Guide

Copy-paste starting points for Solana programs. Keep decimal parsing off-chain:
clients/tests can use `solmath::fp("0.05")`, but on-chain instructions should
receive already-validated integers scaled by `SCALE = 1_000_000_000_000`.

## Dependencies

```toml
[dependencies]
anchor-lang = "0.31"
solmath = { version = "0.1", default-features = false, features = ["transcendental"] }
```

For pool math and token conversion, use:

```toml
[dependencies]
anchor-lang = "0.31"
solmath = { version = "0.1", default-features = false, features = ["pool"] }
```

## Shared Error Mapping

Use one mapper at your program boundary. Do not unwrap math results.

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

## 1. Options Quote

Use `bs_full_hp` when accuracy matters. Current benchmark: ~118K CU average,
~165K max for price + all 5 Greeks. A quote-only instruction fits under the
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

## 2. Weighted Pool Swap Quote

Use `weighted_pool_swap` for Balancer-style weighted pools. It returns
`(net_out, fee)` at `SCALE`. Convert raw token amounts to fixed-point before
calling it, and convert the output back to raw token units with floor rounding.

Compute budget: the swap path uses one HP power calculation. Start with the
default 200K CU budget for quote-only instructions, and benchmark your full
instruction if you add token CPIs or multi-hop routing.

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
| `bs_full_hp` quote | 200K quote-only, 250K+ with extra logic | Benchmarked ~118K avg / ~165K max. |
| Weighted pool quote | 200K quote-only | Benchmark if combined with token CPIs or routing. |
| Token conversion | Default budget | Helpers are simple integer conversions. |

For higher-cost models such as `implied_vol`, `barrier_option`, and `nig_64`,
request a larger budget in the client before calling the instruction.
