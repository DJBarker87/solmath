# Arithmetic-Asian / TWAP Option Pricing

## Overview

`arithmetic_asian_price` and its protocol-oriented alias
`twap_option_price` provide constant-time pricing for an option settled against
a continuous arithmetic average. The functions support an averaging window
that starts in the future and an average that is already partially fixed.

SolMath computes the arithmetic average's first two GBM moments analytically
and prices their matched lognormal distribution, following the
Levy/Turnbull-Wakeman family of arithmetic-Asian approximations.

## Settlement state

The final average is

```text
A = w A_fixed + (1-w) B
B = 1/tau integral_[T-tau,T] S(u) du
```

where:

- `T` is `t`, the time remaining until payment;
- `tau` is `averaging_time`, the remaining continuous averaging-window length;
- `w` is `fixed_weight`, the fraction already observed;
- `A_fixed` is `fixed_average`, the average of those observations.

For a 30-minute settlement TWAP:

| State | `t` | `averaging_time` | `fixed_weight` | `fixed_average` |
|---|---:|---:|---:|---:|
| Before the window | time to expiry | 30 minutes | 0 | 0 |
| 12 minutes observed | 18 minutes | 18 minutes | 12/30 | observed 12-minute TWAP |
| Fully fixed | time to payment | 0 | 1 | final TWAP |

Times are year fractions at `SCALE = 1e12`. Derive all four state fields from
one canonical observation accumulator so the quote and persisted fixing state
remain coherent.

## Moment calculation

Under risk-neutral GBM with carry `b = r - q`, let

```text
a = T - tau
B0 = b tau
V = sigma^2 tau
phi1(x) = expm1(x) / x
```

Then the future average has exact first moment

```text
E[B] = S exp(b a) phi1(B0)
```

and exact second moment

```text
E[B^2] = S^2 exp((2b + sigma^2) a) J(B0,V)

J(B0,V) = 2/B0 [exp(B0) phi1(B0+V) - phi1(2B0+V)].
```

The implementation uses the continuous limit at `B0 = 0` and a bivariate
series near the origin, avoiding cancellation for minute-scale TWAP windows.
The fixed portion is deterministic at quote time, so

```text
E[A]   = w A_fixed + (1-w) E[B]
Var[A] = (1-w)^2 Var[B].
```

The matched lognormal variance is

```text
v = ln(1 + Var[A] / E[A]^2).
```

Calls and puts are priced from `(E[A], v)` and discounted to payment. The
public outputs satisfy discounted average put-call parity exactly after
fixed-point rounding.

## API

```rust
use solmath::{twap_option_price, SCALE};

let minutes = |n: u128| n * SCALE / (365 * 24 * 60);
let quote = twap_option_price(
    100 * SCALE,         // spot
    100 * SCALE,         // strike
    50_000_000_000,      // r = 5%
    20_000_000_000,      // q = 2%
    600_000_000_000,     // sigma = 60%
    minutes(18),         // time to payment
    minutes(18),         // remaining averaging window
    99_500_000_000_000,  // fixed average = 99.50
    400_000_000_000,     // fixed weight = 12/30
)?;

let _ = (quote.call, quote.put, quote.expected_average, quote.log_variance);
# Ok::<(), solmath::SolMathError>(())
```

Use `TwapInputs::from_raw` at an instruction boundary to validate prices,
rates, volatility, times, the averaging-window relation, and fixing-state
coherence once.

## Validation

- Six committed 80-decimal mpmath vectors cover unseasoned, future-starting,
  partially fixed, tiny-price, long-maturity, and 18-minute TWAP cases.
- `benchmark/prod_asian_vectors.json` contains exactly 100,000 stratified
  production vectors, and `benchmark/adv_asian_vectors.json` contains exactly
  10,000 adversarial vectors spanning tiny windows, the moment-series seam,
  raw carry/fixing seams, future starts, deep tails, high variance, and
  near-ATM partially fixed contracts with very little residual variance. All
  110,000 compiled calls completed without rejection. Against 60-digit mpmath, production
  call/put P99 was `1,180` raw and maximum was `22,580` raw (`$2.258e-8`);
  adversarial call/put P99 was `1,220,288` raw and maximum was `19,587,949`
  raw (`$0.000019587949`). The adversarial maximum has only `16` raw matched
  log variance, so SCALE-resolution log/CDF inputs are strongly amplified at
  the near-ATM transition. Generate and validate the corpora with
  `scripts/generate_asian_vectors.py` and
  `scripts/validate_asian_corpora.py`.
- `benchmark/asian_quantlib_vectors.json` commits 10,000 prices produced
  directly by QuantLib 1.41 `ContinuousArithmeticAsianLevyEngine`; regenerate
  it with `scripts/generate_asian_quantlib_vectors.py`. A 500-vector generated
  subset runs under Cargo. Across all 10,000 rows the call/put median raw
  deviations are `110/112`; maxima are `$0.000594854/$0.000594852` in a
  cancellation-sensitive two-day QuantLib case.
- The optimized deployed composite SBF artifact was measured on 2,000
  practical runtime inputs: `137,997` average, `161,471` median, `177,422`
  P95, `180,029` P99, and `182,458` maximum math CU. Every call succeeded.
- A separate 10,000-input full-domain/branch-seam sweep, including 3,332
  expected domain rejections, measured every call and maxed at `185,590`
  math CU. Complete Anchor transactions maxed at `186,610` CU, leaving
  `13,390` CU below the 200K target. The raw report is
  `benchmark/asian_cu_report.json`.

These checks validate the fixed-point implementation against the stated
moment-match model and characterize its behavior at the important fixed-point
and near-deterministic seams.

Reproduce the randomized implementation check with:

```bash
python3 scripts/validate_asian_runtime.py --cases 500
python3 scripts/generate_asian_vectors.py
python3 scripts/validate_asian_corpora.py
python3 scripts/generate_asian_quantlib_vectors.py
```

## Model scope

- Continuous sampling is an approximation to a discrete oracle feed.
- The model assumes GBM with constant `r`, `q`, and `sigma`.
- `fixed_average` and `fixed_weight` must come from persisted, authenticated
  oracle state.
- Protocols with a discrete observation feed can compare their exact schedule
  with Monte Carlo when calibrating the continuous-sampling approximation.
- No Greeks or implied-volatility inversion are included in the first version.
