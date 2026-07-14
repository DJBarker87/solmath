# SolMath — Usage Guide

Worked examples for every major runtime feature. All code compiles standalone
with `solmath = { version = "0.2", features = ["full"] }`. Offline table
generation requires the additional `table-gen` feature.

See the generated rustdoc API and the function table in `README.md` for
complete signatures, CU costs, and ULP accuracy. For Anchor/Solana instruction
templates with compute-budget guidance, see `INTEGRATION.md`.

---

## Fixed-point encoding

Every value is a `u128` or `i128` integer scaled by `SCALE = 1_000_000_000_000`
(1e12). In clients, tests, scripts, and off-chain config, use `fp` / `fp_i`:

```rust
use solmath::{fp, fp_i};

let spot = fp("100")?;
let rate = fp("0.05")?;
let rho = fp_i("-0.70")?;

// Non-zero digits beyond 12 decimal places are rejected.
assert!(fp("1.0000000000001").is_err());
# Ok::<(), solmath::SolMathError>(())
```

On-chain programs should receive already-validated integers across instruction
data. To encode a real number by hand, multiply by SCALE:

```rust
use solmath::SCALE;

let one       = SCALE;                // 1.0
let half      = SCALE / 2;           // 0.5
let hundred   = 100 * SCALE;         // 100.0
let five_pct  = 50_000_000_000u128;  // 0.05
let twenty_pct = 200_000_000_000u128; // 0.20
let neg_half  = -(SCALE as i128) / 2; // -0.5  (use SCALE_I for signed)
```

To decode back to a human-readable float (off-chain only):

```rust
let value: u128 = 1_500_000_000_000; // 1.5 at SCALE
let real = value as f64 / 1e12;       // 1.5
```

---

## Core arithmetic

```rust
use solmath::*;

// Multiply: 2.5 * 3.0 = 7.5
let a = 2 * SCALE + SCALE / 2; // 2.5
let b = 3 * SCALE;             // 3.0
let product = fp_mul(a, b)?;   // 7_500_000_000_000 = 7.5
assert_eq!(product, 7 * SCALE + SCALE / 2);

// Signed multiply: -1.5 * 4.0 = -6.0
let x = fp_mul_i(-3 * SCALE_I / 2, 4 * SCALE_I)?;
assert_eq!(x, -6 * SCALE_I);

// Divide: 7.5 / 2.5 = 3.0
let quotient = fp_div(product, a)?;
assert_eq!(quotient, 3 * SCALE);

// Square root: sqrt(2.0) = 1.41421356...
let root = fp_sqrt(2 * SCALE)?;
// root ≈ 1_414_213_562_373  (within 1 ULP)
# Ok::<(), SolMathError>(())
```

### Overflow-safe multiply-divide

When the intermediate `a * b` might exceed u128, use the mul_div family:

```rust
use solmath::*;

// u64 floor(a * b / c)
let out = mul_div_floor(1_000_000, 999_999, 1_000_000)?;
assert_eq!(out, 999_999);

// u128 ceil(a * b / c) — uses U256 intermediate
let big = mul_div_ceil_u128(u128::MAX / 2, 2, 3)?;
# Ok::<(), SolMathError>(())
```

---

## Transcendentals

```rust
use solmath::*;

// ln(2.0) ≈ 0.693147...
let ln2 = ln_fixed_i(2 * SCALE)?;
assert!((ln2 - 693_147_180_560i128).abs() <= 3); // max 3 ULP

// exp(1.0) ≈ 2.71828...
let e = exp_fixed_i(SCALE_I)?;
assert_eq!(e, 2_718_281_828_459i128); // this input rounds exactly

// pow(2.0, 0.5) = sqrt(2) ≈ 1.41421...
let root2 = pow_fixed(2 * SCALE, SCALE / 2)?;

// expm1(0.001) — cancellation-safe for small x
let tiny = expm1_fixed(1_000_000_000)?; // 0.001 at SCALE
// More accurate than exp_fixed_i(x)? - SCALE_I for |x| near 0

// Integer power: 1.05^10 ≈ 1.62889...
let compound = pow_int(SCALE + 50_000_000_000, 10)?;
# Ok::<(), SolMathError>(())
```

---

## Trigonometry

```rust
use solmath::*;

// sin(π/6) ≈ 0.5
let pi_over_6 = 523_598_775_598i128; // π/6 at SCALE
let s = sin_fixed(pi_over_6)?;
assert!((s - SCALE_I / 2).abs() <= 2); // max 2 ULP

// cos(0) = 1.0
let c = cos_fixed(0)?;
assert_eq!(c, SCALE_I);

// sincos — single reduction, returns (sin, cos)
let (sn, cs) = sincos_fixed(pi_over_6)?;
# Ok::<(), SolMathError>(())
```

Any `i128` angle is accepted — reduction via `rem_euclid` handles the full range.

---

## Normal distribution

```rust
use solmath::*;

// PDF: φ(0) ≈ 0.39894...
let pdf = norm_pdf(0)?;
assert!((pdf - 398_942_280_401i128).abs() <= 2);

// CDF: Φ(0) = 0.5
let cdf = norm_cdf_poly(0)?;
assert!((cdf - SCALE_I / 2).abs() <= 2); // proved for every i128 input

// Quantile: Φ⁻¹(0.975) ≈ 1.96
let z = inverse_norm_cdf(975_000_000_000)?; // 0.975 at SCALE
assert!((z - 1_959_963_984_540i128).abs() <= 6);

// Combined CDF + PDF (saves ~338 CU on average vs separate final-SBF calls)
let (cdf_val, pdf_val) = norm_cdf_and_pdf(SCALE_I)?; // at x = 1.0
# Ok::<(), SolMathError>(())
```

---

## Bivariate normal CDF

```rust
use solmath::*;

// Phi2(-0.5, 0.3; 0.85)
let a = fp_i("-0.5")?;
let b = fp_i("0.3")?;
let rho = fp_i("0.85")?;
let prob = bvn_cdf(a, b, rho)?;
// prob is an i128 probability at SCALE.

// Fixed-rho hot path: embed a precomputed 64x64 SCALE_6 table.
// Replace this placeholder with values generated offline.
let table = Phi2Table::from_array([[0i32; 64]; 64]);
let fast_prob = table.eval(a, b)?;
# let _ = (prob, fast_prob);
# Ok::<(), SolMathError>(())
```

Use `bvn_cdf` for general correlation. Exact endpoints and certified separated
near-singular limits are analytic; unresolved near-equal `|rho|>.99` inputs
return `NoConvergence`. Use `Phi2Table::eval` only when a protocol accepts its
measured 64x64 bilinear-table error (max 0.001326764088 in the audit corpus).
The final affected-path SBF sample averaged/maxed at 100,498/135,944 CU for
`bvn_cdf`; the broader retained branch-grid audit reached 208,693 CU. Runtime-
backed table lookup averaged/maxed at 1,439/1,440 CU.
`Phi2Table::generate` is offline-only.

---

## Two-asset rainbow options

The `rainbow` feature prices calls on the minimum or maximum of two assets with
the analytic Stulz formulas:

```rust
use solmath::{best_of_call, worst_of_call, SCALE};

let s1 = 100 * SCALE;
let s2 = 105 * SCALE;
let strike = 100 * SCALE;
let rate = 50_000_000_000;
let q1 = 10_000_000_000;
let q2 = 20_000_000_000;
let sigma1 = 250_000_000_000;
let sigma2 = 300_000_000_000;
let rho = 400_000_000_000i128;
let time = SCALE;

let worst = worst_of_call(
    s1, s2, strike, rate, q1, q2, sigma1, sigma2, rho, time,
)?;
let best = best_of_call(
    s1, s2, strike, rate, q1, q2, sigma1, sigma2, rho, time,
)?;

assert!(best >= worst);
# Ok::<(), solmath::SolMathError>(())
```

Both functions evaluate three bivariate-normal terms on-chain and support
positive or negative return correlation.

---

## Black-Scholes (HP)

The HP path gives roughly 10-14 significant figures on non-tiny outputs and
averaged 113,177 CU for prices plus all 5 Greeks. The standard `bs_full` path
averaged 24,717 CU and maxed at 25,650 CU when lower precision is acceptable:

```rust
use solmath::*;

let s     = 100 * SCALE;           // spot = $100
let k     = 105 * SCALE;           // strike = $105
let r     = 50_000_000_000u128;    // 5%
let sigma = 200_000_000_000u128;   // 20%
let t     = SCALE;                 // 1 year

// Prices only (84,528 CU average / 122,380 max)
let (call, put) = black_scholes_price_hp(s, k, r, sigma, t)?;
// call ≈ $8.02, put ≈ $7.90

// Full Greeks (113,177 CU average / 149,925 max)
let g = bs_full_hp(s, k, r, sigma, t)?;
// g.call, g.put          — option prices
// g.call_delta, g.put_delta — deltas (signed)
// g.gamma                — gamma (same for call/put)
// g.vega                 — vega
// g.call_theta, g.put_theta — thetas
// g.call_rho, g.put_rho — rhos

// Individual Greeks at standard precision (~14K-18K CU average each)
let (cd, pd) = bs_delta(s, k, r, sigma, t)?;
let gamma = bs_gamma(s, k, r, sigma, t)?;
let vega = bs_vega(s, k, r, sigma, t)?;
# Ok::<(), SolMathError>(())
```

Use `bs_full_hp` when price and Greek accuracy are the priority; use `bs_full`
when the smaller standard-precision path is sufficient.

---

## American options: Kim Boundary Integration

Enable `american-kbi` when the early-exercise premium must be computed entirely
inside the program:

```rust
use solmath::{american_kbi_price, AmericanKbiKind, SCALE};

let put = american_kbi_price(
    100 * SCALE,
    100 * SCALE,
    50_000_000_000,  // r = 5%
    30_000_000_000,  // q = 3%
    300_000_000_000, // sigma = 30%
    SCALE,           // T = 1 year
    AmericanKbiKind::Put,
)?;
# let _ = put;
# Ok::<(), solmath::SolMathError>(())
```

KBI reconstructs the nonlinear smooth-pasting boundary from the live inputs and
then evaluates Kim's early-exercise-premium integral. Its embedded artifact is
parameter-independent geometry plus nine global cubature weights, not a price
table, per-contract coefficient set, or uploaded operator. The
validated domain and QuantLib QdFp results are in
[`docs/AMERICAN_KBI.md`](docs/AMERICAN_KBI.md).

---

## Exponential NIG options

The `nig` feature provides a European call/put pair under the exponential NIG
model. The result includes the quote-local absolute-error allowance and the
execution tier selected by the engine:

```rust
use solmath::{nig_price_certified, NigParams, SCALE};

let quote = nig_price_certified(
    100 * SCALE,
    100 * SCALE,
    50_000_000_000,       // rate = 5%
    20_000_000_000,       // dividend yield = 2%
    SCALE,                // one year
    NigParams {
        alpha: 15 * SCALE,
        beta: -2 * SCALE as i128,
        delta_per_year: SCALE,
    },
    5_000_000_000,        // requested absolute error = 0.005
)?;

// tier 0 = expiry, 1 = Chernoff tail, 15 = Gauss-Kronrod integration
let _ = (quote.call, quote.put, quote.max_abs_error, quote.tier);
# Ok::<(), solmath::SolMathError>(())
```

The martingale correction, scaled-Bessel density, quadrature, and parity step
all execute in the runtime. See [`docs/NIG.md`](docs/NIG.md) for the parameter
domain, reference campaigns, and compute distribution.

---

## Implied volatility

```rust
use solmath::*;

let s = 100 * SCALE;
let k = 105 * SCALE;
let r = 50_000_000_000u128;
let t = SCALE;

// First compute a reference price at known sigma
let sigma_true = 200_000_000_000u128; // 20%
let (call, _) = black_scholes_price_hp(s, k, r, sigma_true, t)?;

// Recover sigma from the observed price
let sigma_recovered = implied_vol(call, s, k, r, t)?;
// sigma_recovered ≈ 200_000_000_000 (within a few ULP)
# Ok::<(), SolMathError>(())
```

The solver uses Li rational initial guess + Halley refinement + Jaeckel
rational. Accepted final-artifact cases used 82,917 CU median / 88,707 average,
282,132 P99, and 328,660 max. Request at least 500K for the math call plus
surrounding program headroom, and handle `NoConvergence`.

---

## Barrier options

```rust
use solmath::*;

let s     = 100 * SCALE;           // spot
let k     = 100 * SCALE;           // strike
let h     = 90 * SCALE;            // barrier at $90
let r     = 50_000_000_000u128;    // 5%
let sigma = 250_000_000_000u128;   // 25%
let t     = SCALE;                 // 1 year

let result = barrier_option(
    s, k, h, r, sigma, t,
    true,                           // is_call
    BarrierType::DownAndOut,        // knocked out if S ≤ $90
)?;

// result.price  — barrier option price (≤ vanilla)
// result.vanilla — plain vanilla BS reference price

// In/Out conservation: in_price + out_price == vanilla
let result_in = barrier_option(
    s, k, h, r, sigma, t, true, BarrierType::DownAndIn,
)?;
assert_eq!(result.price + result_in.price, result.vanilla);

// Production contracts must persist historical breach state:
let state_aware = barrier_option_with_state(
    s, k, h, r, sigma, t, true, BarrierType::DownAndOut, false,
)?;
# let _ = state_aware;
# Ok::<(), SolMathError>(())
```

The state-aware unbreached path maxed at 415,531 CU; request at least 500K
before adding account/oracle/CPI work.

---

## Arithmetic-Asian / TWAP settlement

`twap_option_price` prices a continuously sampled arithmetic average, including
the portion of an in-progress TWAP that is already fixed:

```rust
use solmath::*;

let minutes = |n: u128| n * SCALE / (365 * 24 * 60);

// 12 of 30 settlement minutes have fixed at $99.50; 18 remain.
let result = twap_option_price(
    100 * SCALE,         // current spot
    100 * SCALE,         // strike
    50_000_000_000,      // rate: 5%
    20_000_000_000,      // continuous yield: 2%
    600_000_000_000,     // volatility: 60%
    minutes(18),         // time to expiry/payment
    minutes(18),         // remaining averaging time
    99_500_000_000_000,  // fixed average
    400_000_000_000,     // fixed weight = 12/30
)?;

// result.call / result.put — discounted option marks
// result.expected_average — risk-neutral expected final TWAP
// result.log_variance — variance parameter of the matched lognormal
# Ok::<(), SolMathError>(())
```

Before the averaging window begins, use the full window length with
`fixed_average = fixed_weight = 0`. Once it is completely fixed, use
`averaging_time = 0` and `fixed_weight = SCALE`. Persist the observation
accumulator on-chain and construct [`TwapInputs`] at the instruction boundary;
derive the fixed average and weight from that authenticated observation state.

The continuous arithmetic moments are exact under constant-parameter GBM, but
the option price uses a two-moment lognormal approximation. See
[`docs/ASIAN_TWAP.md`](docs/ASIAN_TWAP.md) for the equations, validation, and
limitations. The 100K production and 10K adversarial accuracy corpora all
complete without rejection and record their deviations from 60-digit mpmath
references; the adversarial maximum is `$0.000019587949` in a near-deterministic,
near-ATM partially fixed contract. The 2,000-input deployed-SBF sweep measured
137,997 average, 180,029 P99, and 182,458 maximum math CU. A separate
10,000-case adversarial CU sweep maxed at 185,590 math CU and 186,610 CU for the complete benchmark
instruction. Additional account, oracle, logging, or CPI work still requires
explicit budget headroom.

---

## Weighted pool swap (AMM)

```rust
use solmath::*;

// Two-token equal-weight pool: 10,000 of each token
let bal_in  = 10_000 * SCALE;
let bal_out = 10_000 * SCALE;
let w_in    = SCALE / 2;           // 50%
let w_out   = SCALE / 2;           // 50%
let amt_in  = 100 * SCALE;         // swap 100 tokens
let fee     = 3_000_000_000u128;   // 0.3%

let (net_out, fee_amt) = weighted_pool_swap(
    bal_in, bal_out, w_in, w_out, amt_in, fee,
)?;
// net_out ≈ 98.7 tokens (after fee, constant-product pricing)
// fee_amt ≈ 0.3 tokens
# Ok::<(), SolMathError>(())
```

### Token conversion helpers

```rust
use solmath::*;

// USDC (6 decimals): 1,000,000 lamports = 1.0 USDC
let fp = token_to_fp(1_000_000, 6)?;
assert_eq!(fp, SCALE); // 1.0 at SCALE

// Convert back: floor for user payouts, ceil for protocol fees
let raw_floor = fp_to_token_floor(fp, 6)?;
let raw_ceil  = fp_to_token_ceil(fp, 6)?;
assert_eq!(raw_floor, 1_000_000);
assert_eq!(raw_ceil, 1_000_000);
# Ok::<(), SolMathError>(())
```

---

## SABR stochastic volatility

```rust
use solmath::*;

let f     = 100 * SCALE;            // forward
let k     = 105 * SCALE;            // strike
let t     = SCALE;                   // 1 year
let alpha = 200_000_000_000u128;     // 0.20
let beta  = SCALE / 2;              // 0.50 (CEV exponent)
let rho   = -300_000_000_000i128;    // -0.30
let nu    = 400_000_000_000u128;     // 0.40

// Implied Black vol for a single strike
let vol = sabr_implied_vol(f, k, t, alpha, beta, rho, nu)?;

// Full prices (call, put) via SABR vol + BS
let s = 100 * SCALE;
let r = 50_000_000_000u128;
let (call, put) = sabr_price(s, k, r, t, alpha, beta, rho, nu)?;

// Batch pricing: precompute once, then per-strike
let pre = sabr_precompute(f, t, alpha, beta, rho, nu)?;
for &strike in &[95 * SCALE, 100 * SCALE, 105 * SCALE, 110 * SCALE] {
    let vol_k = sabr_vol_at(&pre, strike)?;
    // use vol_k with BS for the price
}
# Ok::<(), SolMathError>(())
```

For a complete executable surface, pass the strike/maturity grid through
`certify_sabr_surface` and consume its typed `CertifiedSabrQuote` nodes. The
accepted guarded price/Greek paths maxed at 603,172/603,160 CU; a 700K math
budget covers those measured paths before surrounding instruction work.

---

## Deterministic Heston limit

```rust
use solmath::*;

let s     = 100 * SCALE;
let k     = 100 * SCALE;
let r     = 50_000_000_000u128;       // 5%
let t     = SCALE;                     // 1 year
let v0    = 40_000_000_000u128;        // initial variance 0.04 (σ₀ = 20%)
let kappa = 2 * SCALE;                // mean reversion speed
let theta = 40_000_000_000u128;        // long-run variance 0.04
let xi    = 0u128;                     // only deterministic variance is executable
let rho   = -700_000_000_000i128;      // spot-vol correlation -0.70

let (call, put) = heston_price(s, k, r, t, v0, kappa, theta, xi, rho)?;
// 200,704-case max call/put errors: 200/372 raw SCALE units.
// Final SBF: 118,523 CU average / 183,239 max.
# Ok::<(), SolMathError>(())
```

This feature implements the deterministic Heston limit. At positive expiry,
`xi > 0` returns `NoConvergence`; stochastic-distribution pricing is available
separately through the exponential NIG engine documented in `docs/NIG.md`.

---

## Complex arithmetic

These primitives are available for general-purpose complex fixed-point work:

```rust
use solmath::*;

let a = Complex::new(SCALE_I, SCALE_I / 2);     // 1.0 + 0.5i
let b = Complex::new(2 * SCALE_I, -SCALE_I);    // 2.0 - 1.0i

let product = complex_mul(a, b)?;               // (2.5 + 0.0i)
let quotient = complex_div(a, b)?;
let exp_z = complex_exp(Complex::new(0, SCALE_I))?; // e^(i) ≈ cos(1) + i·sin(1)
let sqrt_z = complex_sqrt(Complex::new(-SCALE_I, 0))?; // √(-1) = i
# Ok::<(), SolMathError>(())
```

---

## Error handling

All fallible functions return `Result<_, SolMathError>`. Use `?` in Solana
programs to propagate errors as instruction failures:

```rust
use solmath::*;

fn my_instruction(spot: u128, strike: u128) -> Result<u128, SolMathError> {
    // Domain error if either is zero
    let ratio = fp_div(spot, strike)?;
    let ln_ratio = ln_fixed_i(ratio)?;
    Ok(ln_ratio.unsigned_abs())
}

// Matching on specific errors:
match ln_fixed_i(0) {
    Err(SolMathError::DomainError) => { /* expected: ln(0) is undefined */ }
    _ => unreachable!(),
}
```

---

## Cargo.toml feature selection

Pick features to control binary size. Each feature pulls in its dependencies:

```toml
# Minimal: core arithmetic only (mul, div, sqrt)
solmath = { version = "0.2", default-features = false }

# AMM pool math (needs transcendental for pow)
solmath = { version = "0.2", default-features = false, features = ["pool"] }

# Options pricing + Greeks
solmath = { version = "0.2", default-features = false, features = ["bs"] }

# Options + implied vol solver
solmath = { version = "0.2", default-features = false, features = ["iv"] }

# Arithmetic-Asian / partially fixed TWAP settlement
solmath = { version = "0.2", default-features = false, features = ["asian"] }

# Fully on-chain American Kim Boundary Integration
solmath = { version = "0.2", default-features = false, features = ["american-kbi"] }

# Fully on-chain exponential NIG pricing
solmath = { version = "0.2", default-features = false, features = ["nig"] }

# Two-asset best-of / worst-of options
solmath = { version = "0.2", default-features = false, features = ["rainbow"] }

# Every stable runtime module
solmath = { version = "0.2", default-features = false, features = ["full"] }

# Bivariate CDF only
solmath = { version = "0.2", default-features = false, features = ["bivariate"] }

# Offline Phi2 table generation
solmath = { version = "0.2", default-features = false, features = ["table-gen"] }
```

Feature dependency graph:

```
core (always on)
├── transcendental ← complex
│                  ← bs ← iv ← pade-iv
│                  ←    ← heston
│                  ← barrier
│                  ← asian
│                  ← american-kbi
│                  ← nig
│                  ← sabr
│                  ← pool
│                  ← bivariate ← table-gen
│                              ← rainbow
```

`full` enables every stable runtime module, including `american-kbi`,
exponential NIG pricing, two-asset rainbow options, and the deterministic
Heston limit. It does not enable offline `table-gen` or experimental
`pade-iv`; opt into those explicitly.
