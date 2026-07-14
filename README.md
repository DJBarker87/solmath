# SolMath

Deterministic fixed-point mathematics and quantitative finance for Rust and
Solana.

[![Crates.io](https://img.shields.io/crates/v/solmath.svg)](https://crates.io/crates/solmath)
[![Docs.rs](https://docs.rs/solmath/badge.svg)](https://docs.rs/solmath)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

SolMath is a `no_std`, zero-dependency library for deterministic decimal math,
probability functions, derivatives pricing, and DeFi calculations. Runtime
code uses integer arithmetic with 12 decimal places; there is no floating point,
heap allocation, network access, or off-chain pricing service.

The crate spans the complete path from arithmetic primitives to on-chain
pricing engines:

- checked fixed-point multiply, divide, square root, rounding, and U256-backed
  intermediate arithmetic;
- `ln`, `ln(1+x)`, `exp`, `exp(x)-1`, powers, trigonometry, and high-precision
  variants;
- normal PDF/CDF/inverse CDF, bivariate normal probabilities, and two-asset
  rainbow options;
- Black–Scholes prices and Greeks, implied volatility, barrier options,
  arithmetic-Asian/TWAP settlement, American options, exponential NIG,
  deterministic Heston, and SABR;
- weighted-pool swaps, token conversion, and explicit settlement rounding.

`no_std` · zero dependencies · pure integer runtime · `SCALE = 1e12`

## Quick start

```toml
[dependencies]
solmath = "0.2"
```

Values are integers scaled by `1_000_000_000_000`. The parsing helpers are
useful in tests and clients; Solana instructions normally receive encoded
integers directly.

```rust
use solmath::{fp, fp_div, fp_mul, fp_sqrt};

let amount = fp("1250.50")?;
let rate = fp("0.035")?;

let interest = fp_mul(amount, rate)?;          // 43.7675
let one_third = fp_div(fp("1")?, fp("3")?)?; // 0.333333333333
let root_two = fp_sqrt(fp("2")?)?;            // 1.414213562373
```

Pricing models use the same representation:

```rust
use solmath::{bs_full_hp, SCALE};

let greeks = bs_full_hp(
    100 * SCALE,          // spot
    105 * SCALE,          // strike
    50_000_000_000,       // rate = 5%
    200_000_000_000,      // volatility = 20%
    SCALE,                // one year
)?;

// greeks.call, greeks.put, greeks.call_delta, greeks.gamma, greeks.vega, ...
```

## What is included

| Area | Main APIs |
|---|---|
| Fixed-point arithmetic | `fp_mul`, `fp_div`, `fp_sqrt`, rounded/floor/ceil variants, `checked_mul_div_*`, `DoubleWord` |
| Transcendentals | `ln_fixed_i`, `ln_1p_fixed`, `exp_fixed_i`, `expm1_fixed`, powers, sine, cosine |
| Probability | normal PDF/CDF/inverse CDF, bivariate normal CDF, certified fixed-correlation tables |
| European options | Black–Scholes prices, all Greeks, implied volatility, barriers |
| Path-dependent options | partially fixed arithmetic-Asian and TWAP settlement |
| American options | Kim boundary reconstruction plus early-exercise-premium integration |
| Alternative distributions | European exponential-NIG pricing with call, put, error allowance, and execution tier |
| Volatility models | deterministic Heston reduction, SABR implied volatility/pricing/surface certification |
| Multi-asset options | best-of and worst-of calls via the Stulz bivariate-normal formula |
| DeFi math | weighted-pool swaps, token conversion, explicit payout/collection rounding |

## Feature selection

The default is core arithmetic plus `transcendental`. Financial models and
complex arithmetic are opt-in so downstream programs compile only the
capabilities they use.

```toml
# Core arithmetic only
solmath = { version = "0.2", default-features = false }

# Black–Scholes and Greeks
solmath = { version = "0.2", default-features = false, features = ["bs"] }

# Fully on-chain American pricing
solmath = { version = "0.2", default-features = false, features = ["american-kbi"] }

# Fully on-chain exponential NIG pricing
solmath = { version = "0.2", default-features = false, features = ["nig"] }

# All stable runtime modules
solmath = { version = "0.2", features = ["full"] }
```

| Feature | Capability | Pulls in |
|---|---|---|
| `transcendental` | logs, exponentials, powers, trig, normal distribution, HP kernels | — |
| `complex` | fixed-point complex arithmetic | `transcendental` |
| `bs` | Black–Scholes prices and Greeks | `transcendental` |
| `iv` | implied-volatility solver | `bs` |
| `barrier` | continuous European barriers | `transcendental` |
| `asian` | arithmetic-Asian and partially fixed TWAP settlement | `transcendental` |
| `american-kbi` | American call/put pricing | `transcendental` |
| `nig` | exponential-NIG call/put pricing | `transcendental` |
| `heston` | deterministic-variance Heston reduction | `bs` |
| `sabr` | SABR analytics, prices, Greeks, and surface certificates | `transcendental` |
| `pool` | weighted-pool and token math | `transcendental` |
| `bivariate` | bivariate normal CDF and table evaluators | `transcendental` |
| `rainbow` | two-asset best-of/worst-of options | `bivariate` |
| `full` | every stable runtime capability above | all runtime features |
| `table-gen` | offline bivariate table generation | `bivariate` |
| `pade-iv` | alternate experimental IV initializer | `iv` |

`table-gen` and `pade-iv` are intentionally outside `full`. For size-sensitive
Solana programs, prefer `default-features = false` with one or two explicit
capabilities. Final linked size also depends on which functions the downstream
program actually calls and what LTO removes.

## On-chain pricing engines

### American options: Kim Boundary Integration

`american_kbi_price` accepts only `(S, K, r, q, sigma, T)` and the option side.
It reconstructs the smooth-pasting exercise boundary and evaluates Kim's
early-exercise-premium integral inside the program. The embedded artifact is
parameter-independent quadrature geometry and normal-kernel coefficients—not
a grid of prices or per-contract lookup data.

```rust
use solmath::{american_kbi_price, AmericanKbiKind, SCALE};

let put = american_kbi_price(
    100 * SCALE,
    100 * SCALE,
    50_000_000_000,
    30_000_000_000,
    300_000_000_000,
    SCALE,
    AmericanKbiKind::Put,
)?;
```

The production corpus measured maximum call/put errors of
`$0.002744 / $0.003264` per `$100` strike against QuantLib QdFp. The deployed
quote instructions maxed at `390,628 / 390,786` CU. See
[Kim Boundary Integration](docs/AMERICAN_KBI.md).

### Exponential NIG

`nig_price_certified` prices the smaller out-of-the-money leg through an
embedded 15/7 Gauss–Kronrod density integral, obtains the other leg by exact
fixed-point put-call parity, and returns the quote's numerical allowance and
execution tier.

```rust
use solmath::{nig_price_certified, NigParams, SCALE};

let quote = nig_price_certified(
    100 * SCALE,
    100 * SCALE,
    50_000_000_000,
    20_000_000_000,
    SCALE,
    NigParams {
        alpha: 15 * SCALE,
        beta: -2 * SCALE as i128,
        delta_per_year: SCALE,
    },
    5_000_000_000, // requested absolute error = 0.005
)?;

// quote.call, quote.put, quote.max_abs_error, quote.tier
```

Across the production/adversarial reference campaigns, maximum normalized
errors were `$0.000565 / $0.001397` per `$100`. The deployed instruction maxed
at `382,441` CU. See [Exponential NIG](docs/NIG.md).

### Arithmetic-Asian and TWAP settlement

`twap_option_price` combines the authenticated fixed portion of an in-progress
average with exact continuous-GBM first and second moments for the remaining
window, then prices the moment-matched distribution. It handles unseasoned,
partially fixed, and fully fixed averages through one API. The practical
deployed sweep maxed at `182,458` math CU. See
[Arithmetic-Asian / TWAP](docs/ASIAN_TWAP.md).

### The rest of the model surface

- `bs_full` provides a compact standard-precision price-and-Greeks path;
  `bs_full_hp` provides the highest-accuracy European path.
- `implied_vol` combines a rational initializer, bracketed Halley refinement,
  and a normalized-space fallback.
- `barrier_option_with_state` prices continuous down/up, in/out barriers while
  incorporating the contract's persisted breach state.
- `best_of_call` and `worst_of_call` implement the two-asset Stulz formulas.
- `certify_sabr_surface` validates a complete strike/maturity grid and returns
  typed certified quote nodes for execution.
- `heston_price` implements the exact deterministic-variance (`xi = 0`)
  reduction to high-precision Black–Scholes.

## Performance and footprint

Measurements below are from deployed SBF artifacts; CU figures refer to the
math call or benchmark instruction stated, not an application's entire
transaction.

| Operation | Typical CU | Observed maximum |
|---|---:|---:|
| `ln_fixed_i` | 705 average | 808 |
| `exp_fixed_i` | 961 average | 992 |
| `norm_cdf_poly` | 960 average | 993 |
| `bs_full` price + Greeks | 24,717 average | 25,650 |
| `bs_full_hp` price + Greeks | 113,177 average | 149,925 |
| arithmetic-Asian / TWAP | 137,997 average | 182,458 math / 186,610 adversarial instruction |
| deterministic Heston | 118,523 average | 190,756 retained branch-grid max |
| exponential NIG | 129,872 average | 382,441 full instruction |
| American KBI call / put | 381,096 / 371,876 average | 390,628 / 390,786 full instruction |

Isolated linked SBF footprints against the same `184,848`-byte Anchor baseline:

| Capability | Total SBF | Linked delta |
|---|---:|---:|
| `exp_fixed_i` | 191,824 bytes | 6,976 bytes |
| `expm1_fixed` + `ln_1p_fixed` | 225,800 bytes | 40,952 bytes |
| American KBI | 293,360 bytes | 108,512 bytes |
| Exponential NIG | 311,464 bytes | 126,616 bytes |

Run `scripts/measure_sbf_footprint.sh` and the composite harness against the
exact downstream program before setting deployment and transaction budgets.

## Accuracy and numerical design

SolMath uses range reduction, low-degree minimax/piecewise kernels, compensated
fixed-point arithmetic, and widened intermediates. Parameter-independent
coefficients and quadrature geometry are generated offline and embedded as
integers; option prices remain functions of live inputs.

Representative release evidence:

| Path | Release result |
|---|---|
| Core integer arithmetic | 545/545 bit-precise Kani checks across 13 harnesses: U256 carry/division bounds, exact sqrt Newton/bisection transitions, truncation `< 1 ULP`, and nearest rounding `<= 0.5 ULP` |
| `ln_fixed_i` | all-input Arb/exact-integer certificate: at most 3 ULP |
| `norm_cdf_poly` | all-`i128` certificate: at most 2 ULP, exact symmetry, monotone |
| HP Black–Scholes | max call/put error 3/4 raw units in the 100K corpus |
| Barrier options | 443,520 QuantLib comparisons across all eight type/side combinations |
| Arithmetic-Asian / TWAP | 100K production + 10K adversarial mpmath references; production max `$2.258e-8` |
| American KBI | 100K production + 10K adversarial plus held-out/unseen QuantLib QdFp surfaces |
| Exponential NIG | 100K production + 10K adversarial references plus independent 50-digit density/Lewis checks |
| SABR | 500-case QuantLib corpus plus whole-grid parity, bound, vertical, butterfly, and calendar checks |

The detailed corpora, commands, certificate identities, and model-specific
domains are indexed in [VALIDATION.md](VALIDATION.md) and
[PROOFS.md](https://github.com/DJBarker87/solmath/blob/v0.2.0/PROOFS.md).

## Runtime contract

- Public computations are infallible or return `Result<_, SolMathError>`.
- `DomainError`, `Overflow`, `DivisionByZero`, and `NoConvergence` make the
  numerical outcome explicit.
- The crate forbids unsafe Rust, has no runtime dependencies, and does not
  allocate.
- U256-backed paths preserve results whose final value fits even when a
  `u128 × u128` intermediate does not.
- Validated input types such as `EuropeanInputs`, `TwapInputs`, and
  `PoolSwapInputs` let programs establish model bounds once at the instruction
  boundary.
- Settlement helpers expose floor and ceiling explicitly so protocols can
  choose their economic rounding policy.

See [SECURITY.md](SECURITY.md) for the precise arithmetic and model contracts,
and [INTEGRATION.md](INTEGRATION.md) for Anchor patterns and compute-budget
guidance.

## Documentation

- [Usage guide](USAGE.md) — worked examples for the runtime APIs.
- [Solana integration](INTEGRATION.md) — validated inputs, Anchor error mapping,
  account examples, rounding, and CU budgets.
- [Architecture](docs/ARCHITECTURE.md) — precision tiers, modules, generated
  kernels, feature gates, and rounding conventions.
- [Kim Boundary Integration](docs/AMERICAN_KBI.md) — method, domain, QuantLib
  accuracy, CU, and artifact identity.
- [Exponential NIG](docs/NIG.md) — model convention, API, domain, accuracy, and
  compute tiers.
- [Arithmetic-Asian / TWAP](docs/ASIAN_TWAP.md) — settlement state, moment
  equations, validation, and usage.
- [Validation](VALIDATION.md) — release matrix and reproducibility commands.
- [Security model](SECURITY.md) — arithmetic guarantees and integration
  boundaries.
- [API reference](https://docs.rs/solmath) — generated Rust documentation.

## License

MIT OR Apache-2.0
