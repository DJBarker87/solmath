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

The crate also exposes `fp("...") -> Result<u128, SolMathError>` and
`fp_i("...") -> Result<i128, SolMathError>` for clients, tests, scripts, and
off-chain config. These helpers reject non-zero digits beyond 12 decimal places.
On-chain programs should generally receive already-validated fixed-point
integers in instruction data.

### Signedness

- **Prices, volatilities, times, and conventional model rates** — generally
  `u128`.
- **Intermediate results, Greeks, correlation, and NIG rates/yields** — `i128`
  where the quantity can be signed.
- The convention is enforced by function signatures: if a parameter is `u128`,
  the caller cannot pass a negative value.

---

## Two-tier precision model

The library operates at two scales:

| Tier | Constant | Scale | Precision | Used by |
|------|----------|-------|-----------|---------|
| **Standard** | `SCALE` / `SCALE_I` | 1e12 | 12 decimal places | Core arithmetic, transcendentals, trig, standard BS |
| **High-Precision (HP)** | `SCALE_HP` / `SCALE_HP_U` | 1e15 | 15 decimal places | HP ln/exp, HP BS, barrier, Asian/TWAP moments, HP CDF |

### Why two tiers?

Standard precision (1e12) is the default Solana tier:
- `u128` can hold values up to ~3.4e38, so two SCALE-valued numbers can always
  be multiplied without overflow: `1e12 * 1e12 = 1e24 << 3.4e38`.
- 12 decimal places provide fine-grained quote, rate, and probability inputs.
- CU costs are low: simple multiplies are ~50 CU.

HP (1e15) is needed when errors compound through chains of operations. A single
`ln` at 1e12 has a proved 3-ULP all-input bound and measured max 2 ULP error.
Pass that through `exp`, `norm_cdf`, and two
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
├── encoding.rs       fp/fp_i decimal parsing helpers
├── checked.rs        Validated financial input types
│
├── transcendental.rs ln_fixed_i, exp_fixed_i, pow_fixed, expm1_fixed
├── exp_coeffs.rs     generated Q22 minimax / fractional-power constants
├── trig.rs           sin_fixed, cos_fixed, sincos_fixed
├── normal.rs         norm_pdf, norm_cdf_poly, inverse_norm_cdf
├── norm_cdf_coeffs.rs generated guarded CDF coefficients
├── hp.rs             HP variants: ln/exp/pow/mul/div/BS at 1e15
│
├── complex.rs        Complex type, mul/div/exp/sqrt
├── bs.rs             Standard-precision Black-Scholes
├── iv.rs             Implied volatility solver
├── barrier.rs        European barrier options (Rubinstein-Reiner)
├── asian.rs          Continuous arithmetic-Asian / partially fixed TWAP approximation
├── american_kbi.rs   Fully on-chain Kim boundary integration for American options
├── american_kbi_data.rs  Generated parameter-independent KBI quadrature geometry
├── heston.rs         Exact deterministic Heston (`xi = 0`) reduction
├── sabr.rs           SABR implied vol + pricing
├── nig.rs            Bounded exponential-NIG OTM integration + parity
├── i64_math.rs       Signed 1e6-scale NIG compatibility wrappers
├── i64_cf.rs         Test-only Heston characteristic-function research
├── pool.rs           Weighted pool swap + token conversion
├── bvn_cdf.rs        Bivariate normal CDF
├── phi2table.rs      Fixed-correlation Φ₂ tables and certificates
├── rainbow.rs        Stulz best-of / worst-of two-asset calls
└── lib.rs            Feature-gated re-exports
```

### Feature gates

Every module beyond `core` is feature-gated. The dependency graph:

```
core (always on)
  arithmetic, overflow, mul_div, double_word, encoding, constants, error

transcendental (default)
  transcendental, trig, normal, hp

complex
  complex — requires transcendental

bs
  bs — requires transcendental

iv
  iv — requires bs

barrier
  barrier — requires transcendental (uses HP internally)

asian
  asian — requires transcendental (uses HP internally)

american-kbi
  american_kbi — requires transcendental

heston
  heston — requires bs

nig
  nig, i64_math — requires transcendental

sabr
  sabr — requires transcendental

pool
  pool — requires transcendental (uses pow_fixed_hp)

bivariate
  bvn_cdf, phi2table — requires transcendental

rainbow
  rainbow — requires bivariate

table-gen
  offline phi2table generation — requires bivariate

pade-iv
  alternate IV initializer — requires iv
```

The default feature is `transcendental`. Use `features = ["full"]` for all
stable runtime modules, or `default-features = false` for core arithmetic
only. Complex arithmetic and every pricing model are opt-in. Offline
`table-gen` and experimental `pade-iv` are explicit opt-ins and are
deliberately excluded from `full`.

---

## Validation assets

The crate ships measured summaries in its model documentation. Machine-readable
reports, generated corpora, integration tests, SBF harnesses, and Python
generators stay repository-only so the crates.io package remains small. See
`VALIDATION.md` for the exact split and release commands.

---

## Approximation strategy and table budget

Transcendental functions begin with range reduction and use polynomial or
rational kernels on narrow subintervals. Small reduced-domain midpoint tables
are reserved for cases where the final SBF artifact demonstrates a material CU
improvement:

| Function | Reduction | Polynomial | Domain after reduction |
|----------|-----------|------------|-----------------------|
| `ln` / `ln1p` | Normalize to [1, 2) + 1,024 midpoint/reciprocal anchors | Q42 cubic correction | Half of one midpoint segment |
| `exp` | Octave + 32-way fractional split, division-free Q63 residual | Degree-5 Q22 minimax | `r ∈ [-ln2/64, ln2/64]` |
| `expm1` | Rounded `k·ln(2)` + 1,292 midpoint anchors | Q43 cubic correction | Half of one midpoint segment |
| `sin`/`cos` | Cody-Waite `rem_euclid` to `(-π, π]` then octant | Degree-11/10 minimax | `x ∈ [-π/4, π/4]` |
| `norm_cdf` | Ten half-sigma body pieces + four half-sigma tails | Q23 degree 8/7 body; Q39-evaluated degree 6/5/4/3 tail | Q44 `t ∈ [-1, 1]` |

The Remez coefficients are precomputed by offline Python scripts (in `scripts/`)
and baked into generated coefficient modules or `constants.rs` as integer
literals — no runtime fitting.
`ln`, `ln1p`, and `expm1` share a single rounded `k·ln(2)` table. The
`ln1p`+`expm1` combined raw payload is 27,944 bytes, down from 29,800 bytes
with duplicate reduction tables.

The standard normal CDF is not table-driven. Its generated coefficient module
contains 936 bytes of coefficients/cutoff data and is compile-time capped at
2 KiB. Direct tail polynomials replace the former exponential, Mills-ratio
continued fraction, and divisions. An Arb/exact-integer certificate proves at
most 2 ULP, exact symmetry, integer safety, and nondecreasing output for every
`i128`; the Q39 tail guard uses the same stored Q23 coefficients and adds no
payload.

The standard exponential is calculation-first rather than answer-table-driven:
six Q22 coefficients and 32 rounded Q62 fractional-power constants total 304
bytes. Split-i64 Q63 reduction avoids wide division and the phase product is
rounded only once at final reconstruction. Its source-bound Arb/exact-integer
certificate proves monotonicity, relative error below 1.55×10^-16, and raw
error below 41,159 for `|x| < 20`; the retained 100K production corpus measured
33,622 max / 7,881 P99 / zero median. The deployed kernel measured 961 average /
992 max CU, and its isolated linked path is 21,272 bytes smaller than the
former rational implementation.

The enforced optimization order is:

1. range reduction and algebraic simplification;
2. Remez/minimax or another low-degree polynomial/rational kernel;
3. a small reduced-domain anchor table only after CU, accuracy, and linked SBF
   measurements justify it.

Compile-time assertions cap the `exp` payload at 512 bytes, `expm1` at 16 KiB,
`ln1p` at 20 KiB, and the latter pair's combined payload at 32 KiB. The
repository-only SBF footprint harness caps the linked `exp` increase at 10 KiB
and `expm1`/`ln1p`/combined increases at 22/34/50 KiB respectively. It also
verifies that the hybrid `expm1` remains smaller than its former
calculation-heavy path. Run `scripts/measure_sbf_footprint.sh` after changing a
table or kernel.

---

## Deterministic Heston architecture

`heston_price` has three public outcomes:

1. **Expiry** (`t=0`) — intrinsic value.
2. **Deterministic variance** (`t>0`, `xi=0`) — an exact reduction using
   cancellation-safe integrated CIR variance and HP Black-Scholes. The final
   exp-final 2K SBF sample measured 118,523 CU average and 183,239 max; the
   broader 2,004-case branch grid remains the conservative maximum at 190,756.
3. **Stochastic variance** (`t>0`, `xi>0`) — the public deterministic model
   returns `NoConvergence`. Characteristic-function experiments remain confined
   to test-only code and are not part of the runtime API.

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
