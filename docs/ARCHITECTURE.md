# SolMath — Architecture

How the library represents numbers, computes with them, and scales precision.

---

## Fixed-point encoding

SolMath uses **integer-scaled fixed-point** — no floats, no heap, no dependencies.
Every real number is represented as an integer multiplied by a power of 10.

```
real value   = integer / SCALE
SCALE        = 1_000_000_000_000  (1e12, u128)
SCALE_I      = 1_000_000_000_000  (1e12, i128)
```

This gives 12 decimal places of sub-unit precision. One ULP (unit in the last
place) = 1 in the integer representation = 10^-12 in real terms.

### Encoding examples

| Real value | Integer at SCALE | Literal |
|------------|-----------------|---------|
| 1.0 | 1,000,000,000,000 | `SCALE` |
| 100.0 | 100,000,000,000,000 | `100 * SCALE` |
| 0.05 (5%) | 50,000,000,000 | `50_000_000_000` |
| 0.20 (20%) | 200,000,000,000 | `200_000_000_000` |
| -0.7 | -700,000,000,000 | `-700_000_000_000i128` |

### Signedness

- **Prices, rates, volatilities, times** — `u128` (always non-negative).
- **Intermediate results, Greeks, correlation** — `i128` (signed).
- The convention is enforced by function signatures: if a parameter is `u128`,
  the caller cannot pass a negative value.

---

## Two-tier precision model

The library operates at two scales:

| Tier | Constant | Scale | Precision | Used by |
|------|----------|-------|-----------|---------|
| **Standard** | `SCALE` / `SCALE_I` | 1e12 | 12 decimal places | Core arithmetic, transcendentals, trig, standard BS |
| **High-Precision (HP)** | `SCALE_HP` / `SCALE_HP_U` | 1e15 | 15 decimal places | HP ln/exp, HP BS, barrier, HP CDF |

### Why two tiers?

Standard precision (1e12) is optimal for Solana:
- `u128` can hold values up to ~3.4e38, so two SCALE-valued numbers can always
  be multiplied without overflow: `1e12 * 1e12 = 1e24 << 3.4e38`.
- 12 decimal places exceed the precision of any real-world financial parameter.
- CU costs are low: simple multiplies are ~50 CU.

HP (1e15) is needed when errors compound through chains of operations. A single
`ln` at 1e12 has max 3 ULP error. Pass that through `exp`, `norm_cdf`, and two
more multiplies (as Black-Scholes does), and errors can amplify to 100+ ULP at
standard precision. By computing the chain at 1e15 and rounding back to 1e12 at
the end, the library preserves 10-14 significant figures.

### HP functions: input/output convention

HP functions with the `_hp` suffix accept and return values **at standard scale
(1e12)**. The upscale/downscale is internal:

```
caller passes u128 at 1e12
  └── upscale_std_to_hp: × 1000 → i128 at 1e15
        └── all arithmetic at 1e15
              └── downscale_hp_to_std: ÷ 1000 (rounded) → u128 at 1e12
                    └── returned to caller
```

This means `black_scholes_price_hp`, `bs_full_hp`, `pow_fixed_hp`, and
`barrier_option` all have the same calling convention as their standard-precision
counterparts — the caller never needs to think about 1e15.

Exception: functions like `ln_fixed_hp`, `exp_fixed_hp`, `fp_mul_hp_i`, and
`norm_cdf_poly_hp` operate entirely at 1e15. These are primarily used internally
by the HP pricing functions.

---

## U256 software arithmetic

When a `u128 × u128` product exceeds `u128::MAX` (~3.4e38), the library
widens to a software 256-bit integer. The `U256` type in `constants.rs`
uses four `u64` limbs:

```
U256 { limbs: [u64; 4] }
```

Key operations:
- `U256::mul_u128(a, b)` — full 256-bit product of two u128 values
- `U256::div_rem_u128(c)` — divide U256 by u128, returning (quotient, remainder)
- `div_rem_u256_long` — long-division fallback (used by `debug_assert` only)

This powers the `checked_mul_div_*` family, which computes `(a × b) / c`
exactly for any u128 inputs (as long as the quotient fits in u128).

---

## DoubleWord type

`DoubleWord` carries a sub-ULP residual alongside a standard-precision value:

```
true_value = hi + lo / SCALE
```

where `|lo| < SCALE`. This enables error-free propagation through multiply
chains — instead of discarding rounding remainders, they accumulate in `lo`
and can be folded back when precision matters.

Used internally by:
- `fp_mul_i_round_dw` — returns the exact remainder from a fixed-point multiply
- HP `ln` compensated polynomial — carries sub-ULP residuals through Horner evaluation
- HP `pow` — accumulates the exp(sum) chain with sub-ULP precision

---

## Module structure

```
solmath/src/
├── constants.rs      SCALE, SCALE_HP, U256, polynomial coefficients, BsFull
├── error.rs          SolMathError enum
├── arithmetic.rs     fp_mul, fp_div, fp_sqrt (core fixed-point ops)
├── overflow.rs       U256, checked_mul_div_* (wide arithmetic)
├── mul_div.rs        mul_div_floor/ceil at u64 and u128
├── double_word.rs    DoubleWord sub-ULP accumulator
│
├── transcendental.rs ln_fixed_i, exp_fixed_i, pow_fixed, expm1_fixed
├── trig.rs           sin_fixed, cos_fixed, sincos_fixed
├── normal.rs         norm_pdf, norm_cdf_poly, inverse_norm_cdf
├── hp.rs             HP variants: ln/exp/pow/mul/div/BS at 1e15
│
├── complex.rs        Complex type, mul/div/exp/sqrt
├── bs.rs             Standard-precision Black-Scholes
├── iv.rs             Implied volatility solver
├── barrier.rs        European barrier options (Rubinstein-Reiner)
├── heston.rs         Heston stochastic vol (three-path architecture)
├── sabr.rs           SABR implied vol + pricing
├── nig.rs            NIG fat-tail pricing (COS method)
├── i64_math.rs       i64-scale NIG for on-chain use
├── i64_cf.rs         i64-scale Heston characteristic function
├── pool.rs           Weighted pool swap + token conversion
├── gamma.rs          Regularised incomplete gamma (for IL premium)
└── lib.rs            Feature-gated re-exports
```

### Feature gates

Every module beyond `core` is feature-gated. The dependency graph:

```
core (always on)
  arithmetic, overflow, mul_div, double_word, constants, error

transcendental (default)
  transcendental, trig, normal, hp, i64_math

complex (default)
  complex — requires transcendental

bs
  bs — requires transcendental

iv
  iv — requires bs

barrier
  barrier — requires transcendental (uses HP internally)

heston
  heston, i64_cf — requires bs + complex

nig
  nig — requires transcendental + complex

sabr
  sabr — requires transcendental

pool
  pool — requires transcendental (uses pow_fixed_hp)
```

Default features: `transcendental + complex`. Use `features = ["full"]`
for everything, or `default-features = false` for core arithmetic only.

---

## Validation assets

The crate ships the reference assets needed by its test suite:

- `tests/reference/mul_div_vectors.json` — floor/ceil mul-div cross-check vectors
- `benchmark/iv_vectors.json` — implied-vol recovery vectors used by the crate test suite
- `test_data/heston_reference_tests.rs` — generated Heston reference cases
- `test_data/sabr_reference_tests.rs` — generated SABR reference cases

These are included directly from crate-local paths in the `#[cfg(test)]`
modules so `cargo test` can run from this repository without depending on a
separate checkout.

---

## Polynomial approximation strategy

All transcendental functions use **minimax (Remez) polynomials** fitted to
narrow subintervals after range reduction:

| Function | Reduction | Polynomial | Domain after reduction |
|----------|-----------|------------|-----------------------|
| `ln` | Binary shift to [1, 2) + 16-entry table lookup | Degree-3 arctanh Remez | `t ∈ [-1/32, 1/32]` |
| `exp` | Integer/fractional split via `k = round(x/ln2)` | Degree-5 Remez rational | `r ∈ [-ln2/2, ln2/2]` |
| `sin`/`cos` | Cody-Waite `rem_euclid` to `(-π, π]` then octant | Degree-11/10 minimax | `x ∈ [-π/4, π/4]` |
| `norm_cdf` | 6-piece piecewise by `|x|` range | Degree-11 per piece | Per-piece mapped `t` |

The Remez coefficients are precomputed by offline Python scripts (in `scripts/`)
and baked into `constants.rs` as integer literals — no runtime fitting.

---

## Heston three-path architecture

`heston_price` selects one of three code paths based on input characteristics:

1. **Degenerate** (`t=0`, `s=0`, or `k=0`) — returns intrinsic value. ~100 CU.

2. **BS fallback** (`ξ²T < 0.01`) — when vol-of-vol is negligible, the Heston
   model reduces to Black-Scholes with effective variance `σ̄²`. Uses
   `bs_full_hp`. ~130K CU.

3. **Control-variate quadrature** — the full path. Computes a BS reference
   price at an effective σ, then corrects it with a 21-node double-exponential
   quadrature of the difference between the Heston and BS characteristic
   functions. The Heston CF evaluates at `i64` scale (~1e6) for speed; the
   BS CF evaluates at `i128`. ~410-430K CU.

---

## Rounding conventions

| Context | Direction | Rationale |
|---------|-----------|-----------|
| `fp_mul`, `fp_div`, `fp_div_i` | Truncate toward zero | Default for Rust integer division |
| `fp_mul_round`, `fp_mul_i_round` | Round to nearest | Sub-ULP accuracy (0.5 ULP max) |
| `fp_div_floor` | Floor (toward -∞) | Conservative for unsigned payout calculations |
| `fp_div_ceil` | Ceil (toward +∞) | Protocol-safe fee collection |
| `fp_to_token_floor` | Floor | User receives no more than owed |
| `fp_to_token_ceil` | Ceil | Protocol collects at least the fee |
| `weighted_pool_swap` gross_out | Floor | Trader receives less (protocol-favorable) |
| `weighted_pool_swap` fee | Ceil | Protocol collects at least the fee |

In DeFi the general rule is: round against the external party to prevent
economic exploits. SolMath follows this convention where the rounding direction
is financially meaningful.
