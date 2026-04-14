# SolMath

Financial math that fits on Solana.

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

- **9-22x faster** than `rust_decimal` for transcendentals, **10-23x faster** than `brine-fp`
- **Black Scholes Option Price + all 5 Greeks in ~50K CU** — one Solana instruction, room to spare
- **10-14 sig figs** vs QuantLib on the HP Black-Scholes path
- **Proved error bounds** for core primitives ([PROOFS.md](PROOFS.md))
- **European barrier options** — all 4 types (down/up × in/out), ~263K CU, validated against QuantLib on 443K vectors
- **Reproducible validation** — 2.5M+ vectors checked against mpmath, scipy, and QuantLib, with bundled fixture files for the crate test suite

`no_std` | zero dependencies | pure integer arithmetic

## The Problem

`rust_decimal` with its `maths` feature costs **97,188 CU (median) for one ln()** — a 4-token weighted pool needs 4 ln calls minimum, burning ~400K CU on logarithms alone. Solana programs have a hard 200,000 CU limit per instruction.

SolMath computes ln() in **3,500-5,200 CU**.

Measured on-chain (50,000 production vectors, Solana localnet):

| Operation | rust_decimal | brine-fp | SolMath | vs rust_decimal | vs brine-fp |
|-----------|-------------|----------|---------|-----------------|-------------|
| ln(x) | 97,188 med CU | 41,815 med CU | 4,362 med CU | **22x** | **10x** |
| exp(x) | 29,172 med CU | 18,972 med CU† | 5,145 med CU | **6x** | **4x** |
| sqrt(x) | 19,883 med CU | 77,322 med CU | 3,007 med CU | **7x** | **26x** |
| Full BS + all Greeks | — | — | ~50,000 CU | — | — |

†brine-fp exp only handles non-negative inputs.

## Usage

```rust
use solmath::*;

// All values are i128/u128 scaled by SCALE (1e12).
// 1.5 → 1_500_000_000_000.  0.05 → 50_000_000_000.

let s = 100 * SCALE;                    // spot = $100
let k = 105 * SCALE;                    // strike = $105
let r = 50_000_000_000u128;             // risk-free rate = 5%
let sigma = 200_000_000_000u128;        // volatility = 20%
let t = SCALE;                          // time to expiry = 1 year

let greeks = bs_full_hp(s, k, r, sigma, t);
// greeks.call  ≈ $8.02
// greeks.gamma ≈ 0.0198
// greeks.vega  ≈ 39.67
```

```toml
[dependencies]
solmath = "0.1"
```

### Feature Flags

Default features are `transcendental + complex`. For on-chain programs that only need specific functionality, disable defaults and pick what you need:

```toml
# AMM pool math only — smallest binary
solmath = { version = "0.1", default-features = false, features = ["pool"] }

# Black-Scholes pricing + IV
solmath = { version = "0.1", default-features = false, features = ["iv"] }

# Heston stochastic vol
solmath = { version = "0.1", default-features = false, features = ["heston"] }
```

| Feature | Modules | Dependencies |
|---------|---------|--------------|
| *(core)* | arithmetic, mul_div, overflow, constants, error, double_word | — |
| `transcendental` | ln, exp, pow, sin, cos, norm_cdf, norm_pdf, HP variants | — |
| `complex` | complex arithmetic | transcendental |
| `bs` | Black-Scholes pricing + Greeks | transcendental |
| `iv` | implied volatility solver | bs |
| `barrier` | European barrier options | transcendental |
| `nig` | NIG fat-tail pricing | transcendental, complex |
| `heston` | Heston stochastic vol | bs, complex |
| `sabr` | SABR stochastic vol | transcendental |
| `pool` | weighted pool swap math | transcendental |
| `bivariate` | bivariate normal CDF (GL6 + table lookup) | transcendental |
| `table-gen` | offline Φ₂ table generation | bivariate |
| `full` | everything above | all |
| `pade-iv` | experimental Padé IV guess | iv |

Default features: **core + transcendental + complex** — everything needed for general-purpose fixed-point math, logarithms, exponentials, trigonometry, and normal distribution. Pricing models (BS, IV, Heston, SABR, barrier, NIG) and pool math are opt-in. Use `features = ["full"]` for everything, or `default-features = false` for core arithmetic only.

### Binary Size

Deployed `.so` sizes measured against an Anchor baseline (151 KB). Rent rate: 6,960 lamports/byte (2-year rent-exempt).

| Feature | Adds | Rent |
|---------|------|------|
| Core arithmetic (mul, div, sqrt) | +15 KB | 0.10 SOL |
| Pool math (weighted swap) | +50 KB | 0.35 SOL |
| SABR vol surface | +68 KB | 0.47 SOL |
| Black-Scholes + Greeks (HP) | +77 KB | 0.54 SOL |
| Transcendentals (ln, exp, pow, CDF) | +83 KB | 0.58 SOL |
| Heston stochastic vol | +120 KB | 0.83 SOL |
| Implied volatility solver | +161 KB | 1.12 SOL |
| Full library | +261 KB | 1.82 SOL |

All well under Solana's 10 MB program limit. LTO strips unused code paths even within enabled features. Rent is a one-time refundable deposit.

## Use Cases

- **Options protocols** — Black-Scholes pricing + Greeks + IV in a single instruction
- **Exotic options** — European barrier options (knock-in/out) on-chain
- **AMMs / weighted pools** — Balancer-style swap math with overflow-safe division
- **Structured products** — fat-tail pricing (NIG) for skew-aware valuation
- **Risk engines** — HP path gives 10+ sig figs for settlement and margin calculations
- **Any on-chain math** — ln, exp, pow, sqrt, sin, cos, CDF all fit in tight CU budgets

## Safety Model

- **No panics** in any public function for valid-range inputs. Internal assertions are guarded by input clamping
- **No silent sentinels** — every fallible function returns `Result<T, SolMathError>`
- **Error variants:** `DomainError` (invalid input), `Overflow` (result too large), `DivisionByZero`, `NoConvergence` (iterative methods)
- **Total functions** (valid for all inputs, e.g. `sin_fixed`, `norm_cdf_poly`) return bare types
- **Overflow detection:** `fp_mul`, `fp_mul_i`, `fp_mul_round`, `fp_mul_i_round` return `Err(Overflow)` on overflow — no silent saturation or wrap-around. Use `checked_mul_div_i` for an exact multiply-then-divide in one step
- **Internal arithmetic:** Remez polynomials for ln/exp, boundary-constrained CDF — all validated on 100K+ vectors

## At a Glance

| Function | Median err | Max $ error | Avg CU | Max CU |
|----------|-----------|-------------|--------|--------|
| **bs_full_hp** | **0** | **$0.000000000004** | **118K** | **165K** |
| bs_price_hp | 0 | $0.000000000004 | ~60K | ~80K |
| bs_full | 209 | $0.000003 | 50K | 68K |
| barrier_option | 1 | $0.000002 | 263K | 385K ¹ |
| implied_vol | 4 ⁴ | — | 157K | 396K ¹ |
| pow_fixed_hp | 0 | — | 27K | 35K |
| pow_product_hp | 1 | — | 16K | 20K |
| nig_64 | 2,520 | $0.06 | 344K | 386K ¹ |
| **bvn_cdf** | **2** ⁵ | **—** | **129K** | **153K** |
| Phi2Table.eval | 2 ⁵ | — | 943 | 943 |
| ln_fixed_i | 1 | — | 4.5K | 5.2K |
| ln_fixed_hp | 0 | — | 19K | 20K |
| exp_fixed_i | 1 ² | — | 5K | 5K |
| norm_cdf_poly | 0 | — | 6K | 15K |
| fp_sqrt | 0 | — | 3K | 9K |

¹ Requires `ComputeBudgetProgram.setComputeUnitLimit()`. Request 500K for `barrier_option`, `implied_vol`, and `nig_64`. All other functions fit within the default 200K CU budget.

² exp max error of 473M occurs at the i128 overflow boundary (|x| ≈ 40). Within the financial domain (|x| < 20), max error is 1 ULP.

⁵ bvn_cdf and Phi2Table.eval: median error 2 ULP. Max error 92K ULP at |ρ| > 0.95 (= 9.2×10⁻⁵ absolute probability). For |ρ| ≤ 0.90, max error < 1 ULP. Validated on 590K vectors (mpmath 50-digit reference) + 20K on-chain CU measurements.

Accuracy from 100K stratified offline vectors (mpmath 50-digit reference). CU from 50K on-chain vectors (NUC localnet, `BENCH_CONCURRENCY=32`).

<details>
<summary>HP Black-Scholes — 100K vectors, outputs >= $0.01</summary>

| Greek | % Exact | Worst SF | Median SF | Max abs err |
|-------|---------|----------|-----------|-------------|
| Call | 74.5% | 9.6 | 13.6 | 3 |
| Put | 73.1% | 9.9 | 13.6 | 4 |
| Call Delta | 99.9% | 10.1 | 11.8 | 1 |
| Put Delta | 99.9% | 10.3 | 11.7 | 1 |
| Gamma | **100%** | 10.5 | 10.5 | 1 |
| Vega | 84.2% | 10.0 | 13.8 | 6 |
| Call Theta | 95.1% | 10.0 | 13.5 | 2 |
| Put Theta | 94.9% | 10.1 | 13.3 | 2 |
| Call Rho | 73.7% | 9.9 | 14.0 | 11 |
| Put Rho | 75.3% | 10.0 | 14.2 | 11 |

</details>

<details>
<summary>vs QuantLib 1.41 — 5,000 HP Black-Scholes vectors</summary>

Cross-checked against [QuantLib](https://www.quantlib.org/) 1.41's BlackCalculator (IEEE 754 f64).

| Greek | Median agreement (sig figs) |
|-------|---------------------------|
| Call | 14.2 |
| Put | 14.1 |
| Delta | 12.2 |
| Gamma | 10.1 |
| Vega | 14.3 |
| Theta | 13.6 |
| Rho | 14.5 |

</details>

<details>
<summary>Barrier Options — 443K vectors vs QuantLib 1.41</summary>

Validated against QuantLib's AnalyticBarrierEngine (Rubinstein-Reiner closed form). All 4 barrier types × call/put = 8 configurations.

| Type | Vectors | Max ULP | P99 | Median |
|------|---------|---------|-----|--------|
| down_out_call | 60,480 | 26 | 14 | 1 |
| down_in_call | 60,480 | 23 | 10 | 0 |
| down_out_put | 50,400 | 48 | 22 | 1 |
| down_in_put | 50,400 | 63 | 27 | 1 |
| up_out_call | 50,400 | 1,654 | 27 | 1 |
| up_in_call | 50,400 | 1,654 | 33 | 1 |
| up_out_put | 60,480 | 551 | 13 | 1 |
| up_in_put | 60,480 | 552 | 12 | 0 |
| **conservation** | **443,520** | **26** | **15** | **1** |

Conservation: in + out = vanilla, verified to ≤ 26 ULP across all 443K vectors.

On-chain CU (10K vectors on Solana localnet): avg **263K**, median 262K, P99 321K, max 385K.

</details>

## Performance

Measured on Solana BPF with runtime inputs (no constant folding). Median CU from 50,000 on-chain vectors per function (NUC localnet, `BENCH_CONCURRENCY=32`); avg/P99/max from earlier 100K run where not superseded.

| Function | Avg CU | Median CU | P95 CU | P99 CU | Max CU |
|----------|--------|-----------|--------|--------|--------|
| fp_sqrt | 3,598 | 3,007 | — | 5,930 | 9,402 |
| sin_fixed | 4,654 | 4,029 | — | 5,159 | 5,170 |
| cos_fixed | 4,578 | 4,027 | — | 5,168 | 5,181 |
| exp_fixed_i | 4,935 | 5,145 | — | 5,205 | 5,212 |
| norm_cdf_poly | 6,844 | 6,186 | — | 15,311 | 15,333 |
| ln_fixed_i | 4,562 | 4,362 | 5,143 | 5,189 | 5,207 |
| pow_fixed_hp | 27,408 | 27,408 | — | — | — |
| ln_fixed_hp | 19,175 | 18,889 | 19,471 | 19,537 | 19,764 |
| norm_cdf_poly_hp | 24,234 | 19,708 | — | 40,668 | 40,691 |
| **bs_full** | **50,191** | **50,015** | — | **65,762** | **68,418** |
| **bs_full_hp** | **118,202** | **116,628** | — | **163,359** | **164,961** |
| barrier_option | 262,906 | 261,773 | 320,907 | 320,907 | 385,456 |
| implied_vol | 156,563 | 148,575 | — | 339,535 | 395,940 |
| nig_64 | 344,273 | 346,648 | — | 382,667 | 386,010 |
| **bvn_cdf** | **128,614** | **129,700** | **147,771** | — | **153,090** |
| Phi2Table.eval | 943 | 943 | 943 | 943 | 943 |

A full Black-Scholes price + all 5 Greeks fits in **50K CU average**. The HP variant with every Greek at 10+ sig figs fits in **118K CU average**. Both leave room for protocol logic within the default 200K budget. European barrier options (all 4 types) average **263K CU** with a 400K compute budget.

### NUC Arithmetic Rerun

Measured on NUC localnet (`BENCH_CONCURRENCY=32`), 50,000 vectors per function.

| Function | Avg CU | Median CU | P99 CU | Max CU | Max ULP |
|----------|--------|-----------|--------|--------|---------|
| fp_mul | 557 | 530 | 744 | 744 | 1 |
| fp_mul_i | 587 | 561 | 774 | 775 | 0 |
| fp_div | 625 | 655 | 684 | 690 | 1 |
| fp_div_i | 652 | 676 | 718 | 724 | 0 |
| fp_mul_hp_i | 103 | 103 | 103 | 103 | 0 |
| fp_div_hp | 1,376 | 1,345 | 1,480 | 1,486 | 1 |
| checked_mul_div_i | 883 | 883 | 1,106 | 3,807 | 0 |

## Bivariate Normal CDF

First fixed-point bivariate normal CDF on any blockchain. Two tiers: general (any ρ) and fast (fixed ρ with precomputed table). Feature-gated behind `bivariate`.

```toml
solmath = { version = "0.1", features = ["bivariate"] }
```

### `bvn_cdf` — general, any ρ

```rust
use solmath::{bvn_cdf, SCALE};

// Φ₂(-0.5, 0.3; 0.85) — all i128 at SCALE, like everything else in SolMath
let a   = -500_000_000_000i128;
let b   =  300_000_000_000i128;
let rho =  850_000_000_000i128;
let prob = bvn_cdf(a, b, rho)?; // ≈ 0.271 × SCALE
```

6-point Gauss-Legendre quadrature (Drezner-Wesolowsky). Validated against mpmath 50-digit reference on 590K production + adversarial vectors. 20K on-chain CU measurements.

| Metric | Value |
|--------|-------|
| CU median | 129K |
| CU max | 153K |
| Accuracy (|ρ| ≤ 0.90) | max error < 4×10⁻⁷ |
| Accuracy (|ρ| ≤ 0.95) | max error < 5×10⁻⁶ |
| Accuracy (|ρ| ≤ 0.99) | max error < 10⁻⁴ |
| Properties | monotone, symmetric (exact), non-negative, bounded |

### `Phi2Table` — fast, fixed ρ

```rust
use solmath::Phi2Table;

// Embed a precomputed table as const (generated offline with `table-gen` feature)
const MY_TABLE: Phi2Table = Phi2Table::from_array(/* 64×64 i32 array */);

let prob = MY_TABLE.eval(-500_000_000_000i128, 300_000_000_000i128)?;
```

Catmull-Rom bicubic interpolation on a 64×64 lookup table. Pure i64 arithmetic — no `norm_cdf`, no i128 in the hot path.

| Metric | Value |
|--------|-------|
| CU | 943 (constant) |
| Accuracy | max error < 9.0×10⁻⁵ |
| Storage | 64 KB const per table |
| Properties | non-negative, bounded, monotone to 10⁻⁴ |

Enable `table-gen` for the offline `Phi2Table::generate(rho, 64)` constructor. Not needed for on-chain evaluation.

## Accuracy

Validated against 3M+ offline test vectors (100K stratified production per function + 443K barrier vectors from QuantLib + 10K adversarial + 1.35M original suite) plus 1M on-chain vectors on Solana localnet. References computed with mpmath at 50-digit precision, cross-checked against scipy and QuantLib.

### Full accuracy table (100K production vectors)

| Function | Max err | P99 | P95 | Median | % Exact | Max $ err ² |
|----------|---------|-----|-----|--------|---------|------------|
| fp_mul_i | 0 | 0 | 0 | 0 | 100% | — |
| fp_div_i | 0 | 0 | 0 | 0 | 100% | — |
| checked_mul_div_i | 0 | 0 | 0 | 0 | 100% | — |
| fp_sqrt | 1 | 1 | 1 | 0 | 50.0% | — |
| fp_mul_hp_i | 0 | 0 | 0 | 0 | 100% | — |
| fp_div_hp_safe | 1 | 1 | 1 | 1 | 49.6% | — |
| ln_fixed_i | 3 | 2 | 2 | 1 | 44.2% | — |
| ln_fixed_hp | 2 | 1 | 1 | 0 | 71.7% | — |
| exp_fixed_i | 473M ³ | 127M | 3.5M | 1 | 32.0% | — |
| sin_fixed | 2 | 1 | 1 | 1 | 48.9% | — |
| cos_fixed | 2 | 1 | 1 | 1 | 44.6% | — |
| norm_cdf_poly | 4 | 3 | 2 | 0 | 50.2% | — |
| norm_cdf_poly_hp | 5 | 3 | 2 | 1 | 42.8% | — |
| norm_pdf | 2 | 1 | 1 | 1 | 23.8% | — |
| pow_fixed_hp | 21.5M | 648 | 0 | 0 | 96.1% | — |
| pow_product_hp | 3K | 1K | 518 | 1 | 45.3% | — |
| bs_full.call | 3K | 2K | 1K | 209 | 1.6% | $0.000003 |
| bs_full.put | 3K | 2K | 1K | 213 | 2.2% | $0.000003 |
| bs_full_hp.call | 3 | 1 | 1 | 0 | 74.5% | $0.000000000003 |
| bs_full_hp.put | 4 | 2 | 1 | 0 | 73.1% | $0.000000000004 |
| bs_full_hp.delta | 1 | 0 | 0 | 0 | 99.9% | — |
| bs_full_hp.gamma | 1 | 0 | 0 | 0 | 100% | — |
| bs_full_hp.vega | 6 | 1 | 1 | 0 | 84.2% | — |
| bs_full_hp.call_theta | 2 | 1 | 0 | 0 | 95.1% | — |
| bs_full_hp.put_theta | 2 | 1 | 1 | 0 | 94.9% | — |
| bs_full_hp.call_rho | 11 | 2 | 1 | 0 | 73.7% | — |
| bs_full_hp.put_rho | 11 | 2 | 1 | 0 | 75.3% | — |
| barrier (down call) | 26 | 14 | 8 | 1 | — | $0.000000000026 |
| barrier (down put) | 63 | 27 | 13 | 1 | — | $0.000000000063 |
| barrier (up call) | 1,654 | 33 | 17 | 1 | — | $0.000000001654 |
| barrier (up put) | 552 | 13 | 7 | 1 | — | $0.000000000552 |
| nig_64 | 64K | 49K | 16K | 2,520 | — | $0.06 |
| implied_vol | 17M ⁴ | 20.5K | 47 | 4 | — | — |
| bvn_cdf | 92K ⁵ | 9.6K | 864 | 2 | — | — |
| Phi2Table.eval | 90K ⁵ | — | — | 2 | — | — |

² Dollar errors assume a ~$10 option. 1 ULP = $0.000000000001.

³ exp max error 473M occurs at the i128 overflow boundary (|x| ≈ 40). Within the financial domain (|x| < 20), exp achieves 10+ significant figures. The relative error remains < 1.7 × 10⁻¹¹ across the full range.

⁴ IV ULP measured via round-trip: σ_true → BS price (mpmath) → quantize to SCALE → `implied_vol` → compare to σ_true. Offline Rust measurement on 100K production + 10K adversarial vectors; 108,494 converging inputs (98.6%). 1,506 inputs return `Err(NoConvergence)` — deep ITM/OTM where extrinsic value is below 1 ULP and there is no invertible signal. 96.2% of converging inputs are within the 100 ULP design tolerance; the tail (max 17M ULP, 0.24% of inputs) occurs near the convergence boundary where price quantization limits recoverable precision. CU from 50K on-chain vectors (NUC localnet).

Accuracy from 100K stratified offline vectors (mpmath 50-digit reference).

<details>
<summary>Formal error bounds</summary>

| Function | Proved bound | Observed max |
|----------|-------------|-------------|
| fp_mul_i | < 1 (proved) | 0 |
| fp_mul_round | ≤ 0.5 (by construction) | 0 |
| fp_mul_i_round | ≤ 0.5 (by construction) | 0 |
| fp_div_round | ≤ 0.5 (by construction) | — |
| fp_sqrt | < 1 (proved) | 1 |
| checked_mul_div_i | 0 exact (proved) | 0 |
| ln_fixed_i | <= 15 (proved) | 3 |
| ln_fixed_hp | <= 15 (proved) | 2 |
| norm_cdf_poly | <= 5 (certified) | 4 |

See [PROOFS.md](PROOFS.md) for complete proofs.

</details>

<details>
<summary><h2>Functions</h2></summary>

### Pricing and Greeks

```rust
// Price + all 5 Greeks in one call — ~50K CU
bs_full(s, k, r, sigma, t) -> Result<BsFull, SolMathError>

// HP price only (no Greeks) — ~60K CU
black_scholes_price_hp(s, k, r, sigma, t) -> Result<(u128, u128), SolMathError>

// High-precision variant — ~118K CU, 10+ sig figs on every Greek
bs_full_hp(s, k, r, sigma, t) -> Result<BsFull, SolMathError>

// Implied volatility — Li (2006) rational guess → Halley → Jäckel fallback, ~157K CU avg / 148K median
// Returns Err(NoConvergence) for sub-ULP extrinsic (deep ITM) or zero-vega cases
implied_vol(market_price, s, k, r, t) -> Result<u128, SolMathError>

// NIG fat-tail pricing (i64/1e6 scale, ~344K CU on-chain)
nig_call_64(s, k, r, t, alpha, beta, delta) -> Result<i64, SolMathError>
nig_put_64(s, k, r, t, alpha, beta, delta) -> Result<i64, SolMathError>

// NIG i128 variant — offline/high-precision only (~302K CU native, exceeds on-chain budget)
nig_call_price(s, k, r, t, alpha, beta: i128, delta) -> Result<u128, SolMathError>

// European barrier options — ~263K CU, 4 types × call/put
barrier_option(s, k, h, r, sigma, t, is_call, barrier_type) -> Result<BarrierResult, SolMathError>
// BarrierResult { price: u128, vanilla: u128 }
// BarrierType: DownAndOut, DownAndIn, UpAndOut, UpAndIn

// Individual Greeks (all return Result, all need sigma > 0 and t > 0)
black_scholes_price(s, k, r, sigma, t) -> Result<(u128, u128), SolMathError>
bs_delta(s, k, r, sigma, t) -> Result<(i128, i128), SolMathError>  // (call_delta, put_delta)
bs_gamma(s, k, r, sigma, t) -> Result<i128, SolMathError>
bs_vega(s, k, r, sigma, t) -> Result<i128, SolMathError>
bs_theta(s, k, r, sigma, t) -> Result<(i128, i128), SolMathError>  // (call_theta, put_theta)
bs_rho(s, k, r, sigma, t) -> Result<(i128, i128), SolMathError>    // (call_rho, put_rho)
```

### Transcendentals

```rust
ln_fixed_i(x: u128) -> Result<i128, SolMathError>   // 4.5K CU, 3 ULP max (table-assisted)
exp_fixed_i(x: i128) -> Result<i128, SolMathError>   // 5K CU, 1 ULP median (see accuracy table)
pow_fixed(base, exp) -> Result<u128, SolMathError>    // via exp(exp * ln(base))
pow_fixed_hp(base, exp) -> Result<u128, SolMathError> // 1 ULP median, ~27K CU, tested up to 100×SCALE
pow_int(base: u128, n: u128) -> Result<u128, SolMathError> // integer power, split recursion
pow_fixed_i(base: i128, exp: i128) -> Result<i128, SolMathError> // signed power
ln_fixed_hp(x: i128) -> Result<i128, SolMathError>   // HP variant, 2 ULP max, ~19K CU (compensated DW)
exp_fixed_hp(x: i128) -> Result<i128, SolMathError>   // HP variant at 1e15 scale
sin_fixed(x: i128) -> i128               // 2 ULP max, ~5K CU
cos_fixed(x: i128) -> i128               // 2 ULP max, ~5K CU
sincos_fixed(x: i128) -> (i128, i128)    // both at once, shared reduction
```

### Normal Distribution

```rust
norm_cdf_poly(x: i128) -> i128            // Phi(x), piecewise minimax, ~7K CU, 4 ULP
norm_pdf(x: i128) -> i128                 // phi(x) = exp(-x^2/2)/sqrt(2pi), 2 ULP
norm_cdf_and_pdf(x) -> (i128, i128)       // both at once
norm_cdf_poly_hp(x: i128) -> i128         // HP variant, 5 ULP at 1e15 scale, ~24K CU
```

### Arithmetic

```rust
fp_mul(a: u128, b: u128) -> Result<u128, SolMathError>          // truncating
fp_mul_round(a: u128, b: u128) -> Result<u128, SolMathError>    // rounding (≤ 0.5 ULP)
fp_mul_i(a: i128, b: i128) -> Result<i128, SolMathError>        // truncating
fp_mul_i_round(a: i128, b: i128) -> Result<i128, SolMathError>  // rounding (≤ 0.5 ULP)
fp_mul_i_round_dw(a: i128, b: i128) -> DoubleWord      // rounding + sub-ULP remainder
fp_div(a: u128, b: u128) -> Result<u128, SolMathError>  // truncating, overflow-safe via U256
fp_div_round(a: u128, b: u128) -> Result<u128, SolMathError> // rounding (≤ 0.5 ULP)
fp_div_i(a: i128, b: i128) -> Result<i128, SolMathError> // signed, overflow-safe
fp_div_floor(a, b) -> Result<u128, SolMathError>
fp_div_ceil(a, b) -> Result<u128, SolMathError>
checked_mul_div_i(a, b, c) -> Result<i128, SolMathError> // (a * b) / c, exact via U256
checked_mul_div_floor_i(a, b, c) -> Result<i128, SolMathError> // floor rounding
checked_mul_div_ceil_i(a, b, c) -> Result<i128, SolMathError>  // ceil rounding
mul_div_floor(a: u64, b: u64, c: u64) -> Result<u64, SolMathError> // u64 mul-div, floor
mul_div_ceil(a: u64, b: u64, c: u64) -> Result<u64, SolMathError>  // u64 mul-div, ceil
mul_div_floor_u128(a: u128, b: u128, c: u128) -> Result<u128, SolMathError> // u128 mul-div via U256
mul_div_ceil_u128(a: u128, b: u128, c: u128) -> Result<u128, SolMathError>  // u128 mul-div via U256
fp_sqrt(x: u128) -> u128                   // Newton-Raphson, 1 ULP
fp_mul_hp_i(a: i128, b: i128) -> i128      // HP multiply at 1e15 scale
fp_mul_hp_u(a: u128, b: u128) -> u128      // HP multiply unsigned
fp_div_hp_safe(a: i128, b: i128) -> Result<i128, SolMathError> // HP division
```

### Compensated Arithmetic

```rust
// DoubleWord: hi + lo/SCALE — tracks sub-ULP remainders through multiply chains
DoubleWord { hi: i128, lo: i128 }
DoubleWord::from_hi(v: i128) -> DoubleWord   // wrap standard value (lo = 0)
DoubleWord::to_i128(self) -> i128             // collapse with rounding
DoubleWord::add(self, other) -> DoubleWord    // exact addition with carry

// Split LN2 constants for sub-ULP range reduction in ln/exp
LN2_LO: i128           // true_ln2 × SCALE ≈ LN2_I + LN2_LO / SCALE
LN2_HP_LO: i128        // same at HP scale
LN_REMEZ_COEFFS: [i128; 8]    // ln polynomial as array
LN_REMEZ_HP_COEFFS: [i128; 10] // HP ln polynomial as array
```

### Pool Math

```rust
weighted_pool_swap(
    balance_in, balance_out,
    weight_in, weight_out,
    amount_in, fee_rate,
) -> Result<(u128, u128), SolMathError>  // (net_output, fee)

pow_product_hp(x, w) -> Result<u128, SolMathError> // x^w * x^(1-w) pool invariant, 13+ sig figs
token_to_fp(raw_amount: u64, decimals: u8) -> u128
fp_to_token_floor(fp_amount: u128, decimals: u8) -> u64
fp_to_token_ceil(fp_amount: u128, decimals: u8) -> u64
```

### Complex Arithmetic

```rust
complex_mul(a: Complex, b: Complex) -> Complex
complex_div(a: Complex, b: Complex) -> Result<Complex, SolMathError>
complex_exp(z: Complex) -> Result<Complex, SolMathError>
complex_sqrt(z: Complex) -> Result<Complex, SolMathError>
```

</details>

<details>
<summary><h2>How It Works</h2></summary>

**Fixed-point, not floating-point.** Everything is integer arithmetic on `u128`/`i128` with an implicit 1e12 denominator. No floats touch the runtime.

**Range reduction.** Transcendentals are computed on small intervals and scaled back:
- **ln:** 16-entry split-constant lookup table + degree-3 Remez polynomial via arctanh substitution. Table narrows polynomial range from [0, 1/3] to [0, 1/33], cutting Horner steps from 7→3. Sub-ULP residuals on table values and LN2 constant. 3 ULP max at ~4.5K CU. HP variant uses compensated DW Horner (degree-9) for 2 ULP max.
- **exp:** Decompose x = k*ln(2) + r, Remez rational approximation on the remainder, scale by 2^k. ~half the CU of Taylor.
- **sin/cos:** Cody-Waite two-word 2pi reduction, then minimax Taylor polynomials. 2 ULP max.
- **sqrt:** Newton-Raphson with bit-length initial guess. 1 ULP.

**Minimax polynomial CDF.** 6 piecewise degree-11 polynomials + CF8 asymptotic tail, boundary-constrained, coordinate-descent optimized. 4 ULP max, fully monotone.

**High-precision path.** HP functions compute at 1e15 internal scale, then round to 1e12 on output. The extra 3 digits of internal precision drown truncation noise — all HP Greeks hold 10+ significant figures at ~2.7x the CU cost.

**Overflow-safe division.** `fp_div` and `fp_div_i` use U256 widened arithmetic when `a * SCALE` would overflow u128. Fast path: ~660 CU. Widened path: ~1,650 CU. Both exact to the truncation remainder.

**Compensated arithmetic.** `DoubleWord` tracks sub-ULP remainders: `fp_mul_i_round_dw` returns both the rounded quotient and the exact residual. `horner_compensated` (internal) propagates these remainders through polynomial evaluation, reducing accumulated error from O(n × 0.5 ULP) to O(0.5 ULP). Split LN2 constants (`LN2_LO`, `LN2_HP_LO`) enable sub-ULP range reduction corrections in ln/exp.

**Shared intermediates.** `bs_full` computes d1, d2, Phi(d1), sigma*sqrt(T) once and reuses across price + all 5 Greeks.

**Implied volatility.** Three-stage solver: (1) Li (2006) bivariate rational polynomial for the initial guess when |x| < 0.5 and the normalised price has meaningful digits, (2) bracketed Halley refinement (up to 4 iterations), (3) Jäckel "Let's Be Rational" normalised-space fallback with Householder(3) for out-of-Li-domain cases. Deep OTM tails use a two-step asymptotic guess: A = −2·ln(β) − ln(2π), A₂ = A − ln(A), σ√T ≈ |x|/√A₂. Deep ITM cases where the OTM-equivalent extrinsic value rounds to zero at SCALE return `NoConvergence` rather than a garbage answer.

</details>


<details>
<summary><h2>vs Other Libraries</h2></summary>

Measured on-chain, 50,000 production vectors, Solana localnet (median CU):

| Function | SolMath | rust_decimal | brine-fp | SolMath vs rust_decimal | SolMath vs brine-fp |
|----------|---------|-------------|----------|------------------------|---------------------|
| ln | 4,362 | 97,188 | 41,815 | **22x faster** | **10x faster** |
| exp | 5,145 | 29,172 | 18,972† | **6x faster** | **4x faster** |
| sqrt | 3,007 | 19,883 | 77,322 | **7x faster** | **26x faster** |

†brine-fp exp skips negative inputs.

**Accuracy** (Max ULP, 50K vectors): SolMath ≤2 ULP on all three. brine-fp ≤1 ULP on all three.

**Feature gap**: brine-fp has no Black-Scholes, Greeks, IV solver, normal CDF/PDF/inverse CDF, barrier options, NIG distribution, or pool math. rust_decimal has no transcendentals within the CU budget.

vs **fermat-math**: fermat-math handles decimal accounting with 7 IEEE rounding modes; SolMath handles computational finance — transcendentals, distribution functions, pricing models. They're complementary.

</details>

## Testing

Every accuracy number is independently reproducible. References computed with [mpmath](https://mpmath.org/) at 50 decimal digits, cross-checked against scipy and [QuantLib](https://www.quantlib.org/) 1.41.

100K stratified production vectors per function (regime-bucketed, not uniform random). 443K barrier vectors from QuantLib's AnalyticBarrierEngine. 10K adversarial vectors targeting cancellation regions and overflow boundaries. Formal proofs for core primitives in [PROOFS.md](PROOFS.md).

```bash
pip install -r scripts/requirements.txt
python3 scripts/generate_production_vectors.py
python3 scripts/generate_adversarial_vectors.py
python3 scripts/generate_barrier_vectors.py
python3 scripts/crosscheck_quantlib.py
```

## License

MIT OR Apache-2.0
