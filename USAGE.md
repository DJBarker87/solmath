# SolMath — Usage Guide

Worked examples for every major feature. All code compiles standalone with
`solmath-core = { version = "0.1", features = ["full"] }`.

See [docs/API_REFERENCE.md](docs/API_REFERENCE.md) for the complete function
listing with signatures, CU costs, and ULP accuracy.

---

## Fixed-point encoding

Every value is a `u128` or `i128` integer scaled by `SCALE = 1_000_000_000_000`
(1e12). To encode a real number, multiply by SCALE:

```rust
use solmath_core::SCALE;

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
use solmath_core::*;

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
use solmath_core::*;

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
use solmath_core::*;

// ln(2.0) ≈ 0.693147...
let ln2 = ln_fixed_i(2 * SCALE)?;
assert!((ln2 - 693_147_180_560i128).abs() <= 3); // max 3 ULP

// exp(1.0) ≈ 2.71828...
let e = exp_fixed_i(SCALE_I)?;
assert!((e - 2_718_281_828_459i128).abs() <= 1); // max 1 ULP

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
use solmath_core::*;

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
use solmath_core::*;

// PDF: φ(0) ≈ 0.39894...
let pdf = norm_pdf(0)?;
assert!((pdf - 398_942_280_401i128).abs() <= 2);

// CDF: Φ(0) = 0.5
let cdf = norm_cdf_poly(0)?;
assert!((cdf - SCALE_I / 2).abs() <= 4);

// Quantile: Φ⁻¹(0.975) ≈ 1.96
let z = inverse_norm_cdf(975_000_000_000)?; // 0.975 at SCALE
assert!((z - 1_959_963_984_540i128).abs() <= 6);

// Combined CDF + PDF (saves ~2K CU vs calling separately)
let (cdf_val, pdf_val) = norm_cdf_and_pdf(SCALE_I)?; // at x = 1.0
# Ok::<(), SolMathError>(())
```

---

## Black-Scholes (HP)

The HP path gives 10-14 significant figures in ~50K CU (prices + all 5 Greeks):

```rust
use solmath_core::*;

let s     = 100 * SCALE;           // spot = $100
let k     = 105 * SCALE;           // strike = $105
let r     = 50_000_000_000u128;    // 5%
let sigma = 200_000_000_000u128;   // 20%
let t     = SCALE;                 // 1 year

// Prices only (~60K CU)
let (call, put) = black_scholes_price_hp(s, k, r, sigma, t)?;
// call ≈ $8.02, put ≈ $7.90

// Full Greeks (~130K CU)
let g = bs_full_hp(s, k, r, sigma, t)?;
// g.call, g.put          — option prices
// g.call_delta, g.put_delta — deltas (signed)
// g.gamma                — gamma (same for call/put)
// g.vega                 — vega
// g.call_theta, g.put_theta — thetas
// g.call_rho, g.put_rho — rhos

// Individual Greeks at standard precision (~20K CU each)
let (cd, pd) = bs_delta(s, k, r, sigma, t)?;
let gamma = bs_gamma(s, k, r, sigma, t)?;
let vega = bs_vega(s, k, r, sigma, t)?;
# Ok::<(), SolMathError>(())
```

For on-chain use, prefer `bs_full_hp` — it gives production-grade accuracy in a
single instruction.

---

## Implied volatility

```rust
use solmath_core::*;

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
rational. Typical cost: 30-60K CU.

---

## Barrier options

```rust
use solmath_core::*;

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
# Ok::<(), SolMathError>(())
```

---

## Weighted pool swap (AMM)

```rust
use solmath_core::*;

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
use solmath_core::*;

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
use solmath_core::*;

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

---

## Heston stochastic volatility

```rust
use solmath_core::*;

let s     = 100 * SCALE;
let k     = 100 * SCALE;
let r     = 50_000_000_000u128;       // 5%
let t     = SCALE;                     // 1 year
let v0    = 40_000_000_000u128;        // initial variance 0.04 (σ₀ = 20%)
let kappa = 2 * SCALE;                // mean reversion speed
let theta = 40_000_000_000u128;        // long-run variance 0.04
let xi    = 500_000_000_000u128;       // vol of vol 0.50
let rho   = -700_000_000_000i128;      // spot-vol correlation -0.70

let (call, put) = heston_price(s, k, r, t, v0, kappa, theta, xi, rho)?;
// Accuracy: $0.002-$0.007 typical vs QuantLib
// CU cost: ~410-430K (CV path), ~130K (BS fallback)
# Ok::<(), SolMathError>(())
```

---

## Complex arithmetic

Used internally by Heston and NIG, but available for general use:

```rust
use solmath_core::*;

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
use solmath_core::*;

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
# Minimal: core arithmetic only (mul, div, sqrt) — +15 KB
solmath-core = { version = "0.1", default-features = false }

# AMM pool math (needs transcendental for pow)
solmath-core = { version = "0.1", default-features = false, features = ["pool"] }

# Options pricing + Greeks
solmath-core = { version = "0.1", default-features = false, features = ["bs"] }

# Options + implied vol solver
solmath-core = { version = "0.1", default-features = false, features = ["iv"] }

# Everything
solmath-core = { version = "0.1", features = ["full"] }
```

Feature dependency graph:

```
core (always on)
├── transcendental ← complex ← nig
│                  ← bs ← iv
│                  ←    ← heston (also needs complex)
│                  ← barrier
│                  ← sabr
│                  ← pool
```
