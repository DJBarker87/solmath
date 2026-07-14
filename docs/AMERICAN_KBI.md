# Kim Boundary Integration (KBI)

`american-kbi` is SolMath's fully on-chain American-option engine. It accepts
only `(S, K, r, q, sigma, T)` plus the call/put selector and performs every
parameter-dependent calculation in the program. It does not consume a price
surface, uploaded operator, account, matrix, oracle-built coefficient set, or
trusted off-chain result.

## Method

KBI combines Kim's early-exercise-premium representation with a directly
reconstructed smooth-pasting exercise boundary:

1. Normalize the put problem by strike. Calls use exact American put-call
   duality, swapping `(S, K, r, q)`.
2. Reconstruct the nonlinear put exercise boundary on an 18-node graded time
   grid.
3. Substitute `lag = t*y^2`, cancelling the square-root singularity exactly,
   and evaluate both boundary-history integrals at six fixed Gaussian nodes.
   All 18 positive boundary values remain available for log interpolation.
4. Integrate the early-exercise premium at nine globally transformed nodes.
   Their positive weights are QdFp-regularized empirical cubature weights fit
   once on the 48-contract training corpus.

The embedded artifact contains only parameter-independent grid geometry, nine
global cubature weights, and cubic-Hermite normal-kernel coefficients. It
contains no sampled option prices or per-contract coefficients. The live
boundary, discount factors, normal kernels, exercise decision, and premium are
computed from the six quote inputs on-chain.

## API and runtime domain

All numeric inputs and the returned price use SolMath's `1e12` scale.

```rust
use solmath::{american_kbi_price, AmericanKbiKind, SCALE};

let price = american_kbi_price(
    100 * SCALE,          // spot
    100 * SCALE,          // strike
    50_000_000_000,       // r = 5%
    30_000_000_000,       // q = 3%
    300_000_000_000,      // sigma = 30%
    SCALE,                // T = 1 year
    AmericanKbiKind::Put,
).expect("inputs are inside the KBI domain");
```

The runtime contract covers the following parameter box; inputs outside it
return `DomainError`:

- `0 <= r,q <= 12%`
- `10% <= sigma <= 120%`
- `30/365 <= T <= 2` years
- `|ln(S/K)| <= 0.75`

Enable it independently with:

```toml
solmath = { version = "0.2", default-features = false, features = ["american-kbi"] }
```

## Accuracy against QuantLib QdFp

Errors below are dollars at a `$100` strike. The reference is QuantLib 1.41
`QdFpAmericanEngine` with `accurateScheme`.

| Corpus | Leg | Comparisons | Median | P95 | P99 | Maximum |
|---|---:|---:|---:|---:|---:|---:|
| Held-out | Call | 792 | $0.000089 | $0.001077 | $0.001727 | $0.001885 |
| Held-out | Put | 792 | $0.000094 | $0.001618 | $0.002030 | $0.002095 |
| Unseen deterministic | Call | 6,336 | $0.000113 | $0.001404 | $0.002059 | $0.002502 |
| Unseen deterministic | Put | 6,336 | $0.000097 | $0.001305 | $0.001919 | $0.002698 |

The unseen corpus contains 192 newly sampled contracts and 33 log-moneyness
points per contract per leg. Reports:

- [`american_kbi_runtime_accuracy_report.json`](https://github.com/DJBarker87/solmath/blob/v0.2.0/benchmark/american_kbi_runtime_accuracy_report.json)
- [`american_kbi_unseen_accuracy_report.json`](https://github.com/DJBarker87/solmath/blob/v0.2.0/benchmark/american_kbi_unseen_accuracy_report.json)

The release comparison also executed every row of the existing 100,000-vector
production corpus and every row of the 10,000-vector adversarial corpus. There
was no sampling. The table reports KBI error on every input inside its declared
domain; all such inputs were accepted.

| Corpus | Leg | Requested | In domain / accepted | Median | P95 | P99 | Maximum | Within $0.001 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Production 100K | Call | 100,000 | 78,047 | $0.0000156 | $0.0006946 | $0.0013619 | $0.0027441 | 97.57% |
| Production 100K | Put | 100,000 | 78,047 | $0.0000160 | $0.0007078 | $0.0014208 | $0.0032638 | 97.46% |
| Adversarial 10K | Call | 10,000 | 5,528 | $0.0000234 | $0.0011672 | $0.0017998 | $0.0029456 | 92.37% |
| Adversarial 10K | Put | 10,000 | 5,528 | $0.0000279 | $0.0013621 | $0.0019915 | $0.0027250 | 91.08% |

Every accepted KBI quote was within `$0.01` per `$100` strike. In the
production corpus, the 21,953 excluded inputs per leg consist exactly of 5,094
maturities below 30 days and 16,859 above two years. In the adversarial corpus,
5,528 inputs per leg are in-domain and 4,472 intentionally target at least one
declared domain boundary. No supported input was rejected.

## Deployed compute and footprint

The retained 2,000-quote-per-leg Agave campaign exercised KBI with generated
inputs. Every quote was accepted.

| Leg | Accepted | Average math CU | Median | P95 | P99 | Max math CU | Max full instruction CU |
|---|---:|---:|---:|---:|---:|---:|---:|
| Call | 2,000 / 2,000 | 381,096 | 382,636 | 387,937 | 388,749 | 389,587 | 390,628 |
| Put | 2,000 / 2,000 | 371,876 | 382,652 | 387,882 | 388,879 | 389,742 | 390,786 |

The isolated KBI deployment measured 293,360 bytes against a 184,848-byte
Anchor baseline: a 108,512-byte linked delta. The measured full composite was
1,083,408 bytes with SHA-256
`685d49886179ca0bec80e31d9e9b878f3d044a47869cfd2f70cc2cf194c05161`. See
[`american_kbi_release_report.json`](https://github.com/DJBarker87/solmath/blob/v0.2.0/benchmark/american_kbi_release_report.json).

## Reproducibility

The embedded Q40 artifact is identified by SHA-256
`6c0e7857669b9913770de45da32d5cfdeb1d89faf78e7bcdad55d3ee27a218ca`.
`scripts/generate_american_kbi_data.py` regenerates the parameter-independent
payload, `scripts/fit_american_kbi_price_weights.py --check` reproduces the
nine positive cubature weights from the fixed 48-contract training corpus, and
`scripts/validate_american_kbi_runtime.py` drives the compiled Rust batch
executable against QuantLib. These are release/validation tools only; none
participates in a live quote.

Together, the generator, artifact identity, QdFp corpora, and deployed-CU run
bind the documented algorithm to the measured `0.2.0` implementation.
