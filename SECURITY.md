# SolMath security model

SolMath is designed for deterministic, value-bearing arithmetic in constrained
Rust runtimes. Its security model is based on explicit numeric domains, checked
intermediates, bounded execution, typed validation at program boundaries, and
reproducible numerical evidence.

## Reporting

Report a vulnerability privately through the repository's GitHub
**Security → Report a vulnerability** flow. The maintainer will acknowledge a
report within five business days and coordinate disclosure. The latest
published release and the current main branch receive fixes.

## Enforced runtime properties

| Property | Enforcement |
|---|---|
| Safe Rust | `#![forbid(unsafe_code)]` at the crate root |
| Deterministic runtime | integer-only implementation; no floating point |
| `no_std` | `#![no_std]` with zero dependencies |
| No allocation | fixed-size values and arrays; no allocator |
| Explicit failure | public computations are infallible or return `Result` |
| Bounded execution | iterative methods have fixed iteration caps |
| Overflow handling | checked primitives plus U256 widened paths |
| Profile consistency | debug and release tests run with overflow checks both on and off |
| Production panic gate | CI rejects `unwrap`, `expect`, `panic`, and `unreachable` in the library |

The public error contract has four variants:

| Error | Meaning |
|---|---|
| `DomainError` | The input does not define a supported mathematical problem |
| `Overflow` | The final value cannot be represented safely |
| `DivisionByZero` | A required divisor is zero |
| `NoConvergence` | An iterative/error-gated computation cannot return the requested result |

This keeps numerical outcomes in the normal control flow of a Solana
instruction. Integrations can map each variant to their own program error and
retry, reject, or select another quote path as appropriate.

## Arithmetic and overflow

SolMath uses three layers of arithmetic protection.

### Checked operations

`checked_add`, `checked_sub`, `checked_mul`, and checked conversions are the
default throughout the implementation. A non-representable result becomes
`SolMathError::Overflow`.

### Widened intermediates

Multiplication followed by division frequently has a representable result even
when `a * b` does not fit in `u128`. `fp_mul`, `fp_div`, and the
`checked_mul_div_*` / `mul_div_*_u128` families use software U256 arithmetic for
that case. The widened path preserves the exact quotient/remainder relationship
before the requested rounding rule is applied.

### Entry-domain validation

Pricing and DeFi functions validate their numeric and relational domains before
entering sensitive kernels. The `checked` module turns those constraints into
types such as `EuropeanInputs`, `ImpliedVolInputs`, `BarrierInputs`,
`TwapInputs`, and `PoolSwapInputs`. Programs can validate raw instruction data
once and carry a valid domain through the rest of the calculation.

## Internal optimized kernels

A small number of `pub(crate)` multiply helpers omit repeated checks inside
already reduced polynomial domains. They are not callable by downstream code.
Their callers establish bounds before entry:

| Internal path | Established bound |
|---|---|
| Trig polynomial multiplication | angle has been reduced to the certified core interval |
| HP log/exp multiplication | operands are bounded by the range-reduction contract |
| IV rational initializer | price-like inputs and normalized variables are capped before evaluation |

Changing one of these domains requires the associated invariant, accuracy, and
overflow tests to change with it.

## Model contracts

Each higher-level model exposes the domain it actually implements. A contract
boundary is returned as an error rather than extended by silent extrapolation.

| Model | Runtime contract |
|---|---|
| Black–Scholes | Positive price, strike, volatility, and time within the public numeric bounds |
| Implied volatility | Bounded solver with `NoConvergence` for prices without a resolvable volatility at `SCALE` |
| American KBI | `0–12%` rates/yields, `10–120%` volatility, `30/365–2` years, and `abs(ln(S/K)) <= 0.75` |
| Exponential NIG | Published alpha/beta, elapsed-scale, rate, time, and moneyness domain plus caller-selected error allowance |
| Arithmetic-Asian/TWAP | Coherent fixed average/weight and averaging times; fixing state supplied by the integration |
| Barrier options | State-aware API accepts the persisted historical breach flag |
| Deterministic Heston | Exact `xi = 0` integrated-variance reduction; other positive-expiry `xi` values return `NoConvergence` |
| SABR | Individual analytics plus an atomic whole-grid certificate for parity, bounds, verticals, butterflies, and calendars |
| Bivariate normal | Analytic endpoint handling and guarded quadrature; unresolved near-singular boundary cases return `NoConvergence` |
| Fixed-correlation Phi2 | Raw analytics lookup or certificate-ID/error-budget-bound evaluation |

The detailed KBI, NIG, and TWAP contracts are in
[`docs/AMERICAN_KBI.md`](docs/AMERICAN_KBI.md),
[`docs/NIG.md`](docs/NIG.md), and
[`docs/ASIAN_TWAP.md`](docs/ASIAN_TWAP.md).

## Rounding and settlement

The crate exposes truncating, nearest, floor, and ceiling variants rather than
hiding an economic rounding choice. `fp_to_token_floor` and
`fp_to_token_ceil` make payout and collection policy explicit at the conversion
boundary. Weighted-pool execution rounds output down and the fee up.

For barriers and in-progress averages, mathematical parameters are only part of
the contract state. Integrations should derive breach/fixing state from their
persisted accounts or oracle observations before calling the model.

## Verification

The release workflow exercises several independent layers:

- no-default, default, all-feature, MSRV, and embedded `no_std` builds;
- debug and release tests with overflow checks forced both on and off;
- deterministic five-million-iteration checked-input and financial-invariant
  sweeps;
- strict production Clippy gates for panic-like constructs;
- thirteen Kani harnesses covering 545 bit-precise checks, including full-width
  U256 carry/division bounds, exact square-root Newton/bisection transitions, truncation,
  nearest-rounding, overflow, and double-word ULP invariants;
- source-digest-gated Arb/exact-integer certificates for `ln`, `exp`, and the
  standard normal CDF;
- high-precision and QuantLib comparison corpora for pricing models;
- deployed-SBF footprint and compute campaigns;
- dependency, package-surface, and secret/history checks.

The standard `ln` certificate establishes at most 3 ULP over its valid `u128`
domain. The normal-CDF certificate establishes at most 2 ULP, exact symmetry,
and monotonicity for every `i128`. Model-specific empirical and analytical
results are indexed in [VALIDATION.md](VALIDATION.md) and
[PROOFS.md](https://github.com/DJBarker87/solmath/blob/v0.2.0/PROOFS.md).

## Integration boundary

SolMath supplies numerical functions; a consuming program supplies the
surrounding protocol controls. Account ownership, signer/PDA authorization,
oracle provenance and freshness, volatility calibration, slippage, transaction
composition, and upgrade policy remain part of that program.

Measure the exact linked program before choosing compute limits. The repository
figures isolate SolMath calls and benchmark instructions; account
serialization, logs, oracle reads, and CPIs add to the final transaction.

## Assurance record

Release evidence is retained as generated corpora, machine-readable reports,
source-bound certificates, deterministic tests, and SBF artifacts. That
reproducible record defines the assurance scope of `0.2.0`; downstream reviews
can extend it to the exact accounts, oracles, and economic rules of an
integrating program.
