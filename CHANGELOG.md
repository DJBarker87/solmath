# Changelog

## 0.2.0 - 2026-07-14

- Made the feature graph an explicit release contract. The default surface is
  now core arithmetic plus `transcendental`; complex arithmetic and every
  pricing model are opt-in, while `full` includes stable runtime capabilities
  but deliberately excludes offline `table-gen` and experimental `pade-iv`.
  Removed the inert root `idl-build` flag. These feature/API changes are
  intentionally released as `0.2.0`, rather than a semver-invalid `0.1.6`.
- Kept generated validation reports in the tagged repository instead of the
  crates.io source archive. The published crate contains the implementation,
  user documentation, examples, and measured summaries without spending its
  package-size budget on machine-readable release evidence.
- Expanded bit-precise formal verification from 2 to 13 Kani harnesses and from
  90 to 545 successful checks. In addition to complete-domain ULP rounding and
  double-word invariants, production-linked proofs now cover exact U256 product
  carry assembly, u64 quotient-digit fit, Knuth-D3's two-correction bound, and
  exact square-root Newton and bracket transitions. Fixed-step square-root
  results now use a one-square release certificate and restart through the
  exact monotonically convergent integer-Newton kernel if it rejects.

- Replaced the temporary fail-closed NIG stubs with a fully on-chain,
  domain-gated exponential-NIG call/put engine. It prices the OTM leg through a
  15/7 direct-density rule, derives the other leg by parity, uses a capped
  Chernoff tail certificate, and exposes signed rates/dividends plus a
  quote-local error allowance. A generated piecewise scaled-Bessel kernel cut
  the initial 31-node prototype from roughly 1.03M CU to a 381,385-CU deployed
  maximum. The final 100K/10K campaign has zero allowance/request violations
  and `$0.000565/$0.001397` per `$100` normalized maxima; independent 50-digit
  density and Lewis oracles agree to `1.8e-16` dollars. Isolated linked size is
  311,464 bytes, a 126,616-byte delta over the Anchor baseline. Repository-only
  generators and toolchain reports stay out of the published source archive,
  which remains below 259 KiB against a 260 KiB CI ceiling.

- Completed a release red-team pass: removed the unpublished Volterra
  compatibility feature/API/example, removed the BS1993/BS2002/CRR American
  feature, API, validation harnesses, and 96 KiB bivariate table, removed the
  public KBI profiling hook, reduced the old NIG implementations to temporary
  fail-closed stubs before the replacement above, and stopped deterministic Heston from pulling
  the complex/stochastic research engine into production builds. KBI is now
  the crate's only American-option implementation. Rebuilding the full-feature
  SBF composite reduced it from 1,040,392 to 1,012,056 bytes; at that
  intermediate fail-closed point the isolated KBI artifact was 291,544 bytes. A
  clean package rebuild was roughly 33 KiB smaller.
- Fixed the published-source test boundary by moving the generated SABR corpus
  to a repository-only integration test. The unpacked crate now passes its own
  all-feature test suite without shipping the large generated corpus.
- Bumped the release candidate from the already-published `0.1.5` to `0.2.0`,
  added docs.rs metadata, and tightened the CI package/feature checks.

- Removed the experimental `american-rom` runtime, feature, generated payload,
  example, and benchmark instructions. KBI is now the sole reduced-cost
  accuracy-first American integration path. The published package excludes
  repository-only validation corpora and is guarded by a 260 KiB CI ceiling.

- Added `american-kbi`, an accuracy-first, fully on-chain American call/put
  pricer using nonlinear smooth-pasting boundary reconstruction and Kim's
  early-exercise-premium integration. A singularity-cancelling six-node
  Gaussian history rule and nine positive QdFp-regularized cubature weights
  retain all six-input-dependent work in-program. Against QuantLib 1.41 QdFp,
  unseen maxima are $0.002502/$0.002698 per $100 strike for calls/puts. A full,
  unsampled 100K production plus 10K adversarial campaign accepted every
  in-domain quote and measured production maxima of $0.002744/$0.003264.
  Deployed full-instruction maxima are 390,628/390,786 CU, with a 108,512-byte
  isolated linked delta.

- Added constant-time continuous arithmetic-Asian / TWAP option pricing with
  future-starting windows and partially fixed averages. The implementation
  evaluates the first two GBM average moments at HP precision, uses a
  cancellation-safe short-window series, exposes exact discounted put-call
  parity, and labels the final lognormal moment match as an approximation.
  Added `TwapInputs`, exactly 100,000 stratified production and 10,000
  adversarial 60-digit mpmath vectors, 10,000 separately generated QuantLib
  1.41 vectors, examples, documentation, and an Anchor/SBF composite path. The
  full corpus exposed catastrophic cancellation at raw carry seams; a stable
  second-order small-carry expansion fixed it. Production then exposed a gap in
  the adversarial design: near-ATM, heavily fixed contracts with very little
  residual variance. That regime is now retained explicitly. All 110K compiled
  accuracy calls succeed, with production/adversarial price maxima of
  22,580/19,587,949 raw.
  Reusing the certified SCALE log/CDF kernels reduced deployed compute: the 2K
  practical sweep measured 137,997 average / 180,029 P99 / 182,458 max math
  CU, while the 10K adversarial sweep maxed at 185,590 math / 186,610 complete-
  instruction CU, both below 200K.

- Replaced the standard `ln_fixed_i` 16-anchor/Remez path with the shared
  1,024-segment Q42 midpoint kernel. The full 100K production and 10K
  full-width adversarial corpora now measure 2 ULP max, 1 ULP P99, and zero
  median error. Narrowing the exact LUT index from `u128` to `u64` removed a
  wide-division helper. The final exp-linked composite measured 705 average /
  808 maximum CU; the earlier isolated ln harness measured 712 / 813.
- Refit `norm_cdf_poly` as ten half-sigma guarded body polynomials and four
  direct tail polynomials with a balanced dispatch tree. Standalone CDF now
  performs no exponential or division, has no answer lookup table, measures
  2 ULP max on both 100K/10K corpora, and measured 960 average / 993 maximum
  CU on the final deployed composite artifact.
- Replaced `exp_fixed_i`'s division-heavy full-ln(2) rational with a
  calculation-first, division-free ln(2)/32 kernel: five Q22 minimax Horner
  products, a split-i64 Q63 residual, and 32 rounded Q62 fractional power-of-two
  reconstruction constants (304 bytes, no sampled-answer table). Direct SBF
  compute fell from 6,027 average / 6,365 max to 961 / 992 CU, while the
  isolated linked path became 21,272 bytes smaller. Production max/P99/median
  improved from 449,129,270 / 121,816,490 / 1 raw units to
  33,622 / 7,881 / 0; a regenerated 10K corpus brackets every new reduction
  seam and retains the amplified positive-tail worst zone, measuring
  15,727,361,334,177 max / 6,704,999,717,817 P99 / zero median raw error.
- Reran every retained exp-dependent reference campaign. PDF exactness rose
  from 23.778% to 42.560%; deterministic-Heston call/put maxima improved from
  245/486 to 200/372; BVN, Phi2, SABR, and fail-closed contracts held. A few
  composed power/BS maxima increased because the former exp error had
  accidentally cancelled upstream approximation error. Production IV accepts
  64 fewer rows due fixed-iteration verification-threshold sensitivity even
  though the new discount is correctly rounded on every investigated lost row;
  this is disclosed in the exp downstream report.
- Added reproducible, source-bound Arb/exact-integer release certificates for
  the new logarithm, exponential, and normal-CDF kernels. The logarithm
  certificate proves less than 2.925564 raw real error and at most 3 integer
  ULP for every valid `u128` input. The exponential certificate proves relative
  error below 1.55×10^-16, raw error below 41,159 over `|x| < 20`, and
  monotonicity across every raw input; its full `(-40,40)` raw bound is
  2.21×10^13 because absolute error scales with the result. The CDF certificate
  proves less than 2.393619 raw real error, at most 2 integer ULP, exact
  symmetry, and nondecreasing output for every `i128`.
  Its exact discrete analysis found two real one-ULP inversions missed by dense
  sweeps; Q23 body coefficients and a payload-free Q39 tail evaluation guard
  remove both while retaining the 936-byte coefficient/cutoff payload.
- Reran 2,719,550 downstream output checks across power, inverse CDF, fused
  CDF/PDF, standard Black-Scholes/Greeks, IV, and SABR ratio paths. Added an
  explicit near-one/large-exponent HP fallback after the improved log exposed
  a composition-sensitive power regression.
- Closed the remaining reference gaps with fresh 22,500-case BVN, 20,000-case
  Phi2 off-grid, 100,000-case full SABR, 200,704-case deterministic-Heston,
  100,000-case stochastic-Heston rejection, and public NIG rejection
  campaigns. GL20 BVN slightly improved; SABR price's maximum tail error moved
  from 879 to 1,130 raw units while P99 and median stayed at 126 and zero.
- Added a locked composite SBF harness and remeasured every `ln`/CDF-affected
  executable path in a 55,400-call, zero-harness-error deployed campaign,
  including power, inverse
  CDF, fused CDF/PDF, Black-Scholes and every Greek, IV, SABR, BVN, Heston,
  NIG rejection, and Phi2 lookup. Average CU fell 51.9% for standard BS price,
  36.8% for `bs_full`, 35.5% for IV, and 27.3% for guarded SABR price.

- Added `ln_1p_fixed` for signed, cancellation-safe `ln(1+x)`: a dedicated
  division-free Q42/table kernel measured at 2 raw-unit production maximum,
  zero median error, and 542 average / 811 production-maximum CU over the
  retained native and deployed-SBF campaigns.
- Replaced `expm1_fixed`'s degree-11 Taylor/general-exp paths with a
  division-free raw-Q22/Q43 midpoint kernel: ordinary-domain error improved
  from 11 to 3 raw units, while deployed SBF compute fell to 783 average /
  1,041 production maximum CU.
- Shared the rounded `k·ln(2)` reduction table between `ln_1p_fixed` and
  `expm1_fixed`, reducing their combined raw LUT payload from 29,800 to 27,944
  bytes. Added compile-time table caps and a locked SBF footprint harness that
  guards the linked binary deltas against regression.
- Fixed signed/unsigned half rounding, maximum-input square root, DoubleWord
  tie/overflow behavior, power exponent conversion, large-angle reduction,
  complex wide arithmetic, BS parity/bounds, IV preflight, and token/pool
  rounding/domain defects.
- Added persisted-breach barrier pricing and protocol-favouring weighted-pool
  execution inside a certified ratio/weight domain.
- Made positive-expiry stochastic Heston and NIG public pricing fail closed;
  added a cancellation-safe exact deterministic-Heston reduction.
- Added SABR executable-domain/arbitrage guards and near-singular BVN
  fail-closed behavior; replaced overshooting Phi2 cubic interpolation with
  monotone bilinear interpolation and disclosed its measured precision.
- Hardened reference scripts, dependencies, CI permissions/action pins,
  overflow-profile testing, and release guidance.
- Reran final native accuracy corpora and deployed-SBF CU distributions for
  every changed executable path; evidence is under `.superstack/`.

## 0.1.5 - 2026-04-24

- Added `VALIDATION.md` with release commands, package checks, reference asset
  policy, and production-readiness guidance.
- Documented the CI-ready release check sequence for MSRV library checking, full
  feature tests, examples, and package verification.
- Added `fp` and `fp_i` decimal parsing helpers for tests, examples, clients, and
  off-chain configuration.
- Added examples for options pricing, weighted pool swaps, safe token conversion,
  and an Anchor options-pricing template.
- Added `INTEGRATION.md` with copy-paste Solana instruction patterns for options
  quotes, weighted pool quotes, token conversion, compute budgets, and error
  handling.
- Removed all dev-dependencies by replacing `proptest`/`serde_json`-based tests
  with deterministic in-crate sweeps and compact edge-vector tests.
- Slimmed the crate package by excluding repository-only validation scripts and
  the large generated mul-div vector corpus from crates.io.

## 0.1.3 - 2026-04-24

- Published the bivariate normal CDF release to crates.io.
- Included required docs and generated reference-test assets in the package.
- Corrected README signatures for fallible APIs.
- Removed unsupported audit-history claims and stale architecture references.
