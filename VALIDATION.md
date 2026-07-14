# SolMath Validation

This file is the reproducibility checklist for SolMath releases. It separates
the small crates.io package from the larger repository validation assets so the
published crate stays usable while the full audit trail remains reproducible.

## Toolchains

The `0.2.0` release was checked with:

- Rust/Cargo release toolchain: `rustc 1.93.0`, `cargo 1.93.0`
- MSRV library check: `rustc 1.79.0`
- Package metadata: `rust-version = "1.79"`

The crate has no runtime or dev dependencies. Run the full tests on stable.
The MSRV job intentionally checks the library surface only:
`cargo +1.79.0 check --lib --all-features`.

## Release Checks

Run this sequence locally before publishing. It is also the expected CI
sequence enforced by CI:

```bash
cargo check --lib --all-features       # Rust 1.79.0 MSRV library check
git diff --check
cargo fmt --check
cargo clippy --lib --all-features -- -A warnings -D clippy::correctness -D clippy::suspicious -D clippy::perf
cargo test --no-default-features
cargo test
cargo test --all-features
cargo test --release --all-features    # repeat with overflow checks on and off
cargo check --target thumbv7em-none-eabihf --lib --all-features
python3 scripts/verify_feature_contract.py
cargo check --examples --all-features
./scripts/verify_critical_invariants.sh
python3 scripts/generate_exp_coeffs.py --check
python3 scripts/certify_ln_fixed.py
python3 scripts/certify_exp_fixed.py
python3 scripts/certify_norm_cdf.py
rm -f target/package/solmath-0.2.0.crate
cargo package
cargo publish --dry-run
# In the integration harness: anchor build, deploy to local validator, then
# rerun the changed-path CU matrix before updating any published CU figure.
# With Solana's cargo-build-sbf installed, enforce the linked LUT/kernel budget:
./scripts/measure_sbf_footprint.sh
```

## Local Release Checklist

Run from the repository root:

```bash
rustup toolchain install 1.79.0 stable
cargo +1.79.0 check --lib --all-features
cargo +stable test --no-default-features
cargo +stable test
cargo +stable test --all-features
cargo +stable check --examples --all-features
rm -f target/package/solmath-0.2.0.crate
cargo +stable package
cargo +stable publish --dry-run
```

Then verify the generated package, not only the working tree:

```bash
rm -rf /tmp/solmath-package
mkdir -p /tmp/solmath-package
tar -xzf target/package/solmath-0.2.0.crate -C /tmp/solmath-package
cd /tmp/solmath-package/solmath-0.2.0
cargo test --all-features
```

## Reference Assets

Included in the crates.io package:

- `INTEGRATION.md` — copy-paste Solana integration patterns.
- `USAGE.md`, `VALIDATION.md`, `SECURITY.md`, `CHANGELOG.md`, architecture, and
  the KBI, Asian/TWAP, and NIG guides — usage, release, security, and model
  guidance.

Repository-only assets:

- `scripts/` — Python generators and QuantLib/mpmath cross-check scripts.
- `examples/README.md` and `examples/anchor_options_pricing.md` —
  repository/developer guidance not required to build the published crate.
- `benchmark/iv_vectors.json`, `test_data/`, and `tests/` — generated and
  integration-test corpora executed in the repository CI/test checkout but not
  shipped in the compact crate tarball.
- `benchmark/prod_asian_vectors.json` and `benchmark/adv_asian_vectors.json` —
  exact 100K stratified production and 10K seam/tail adversarial arithmetic-
  Asian corpora from 60-digit mpmath. `benchmark/asian_accuracy_report.json`
  records the compiled 110K sweep.
- `tests/reference/mul_div_vectors.json` — large generated mul-div corpus.
- `benchmark/prod_ln_1p_vectors.json` and `benchmark/adv_ln_1p_vectors.json` —
  100K/10K `mpmath.log1p` reference corpora generated on demand.
- `benchmark/prod_expm1_vectors.json` and `benchmark/adv_expm1_vectors.json` —
  100K/10K `mpmath.expm1` full-domain and reduction-seam reference corpora.
- `benchmark/prod_exp_vectors.json` and `benchmark/adv_exp_vectors.json` — exact
  100K production and 10K adversarial exponential corpora. The latter brackets
  every ln(2)/32 cell seam and retains the positive-tail amplification zone.
- `benchmark/prod_ln_vectors.json` and `benchmark/adv_ln_vectors.json` —
  100K/10K logarithm corpora spanning every midpoint boundary, every reachable
  binary exponent, and `u128` extrema.
- `benchmark/prod_norm_cdf_vectors.json` and
  `benchmark/adv_norm_cdf_vectors.json` — 100K/10K CDF corpora covering every
  body/tail seam, raw neighborhoods, tail rounding transitions, saturation,
  and signed extrema.
- `benchmark/sbf-footprint/` — isolated, locked Anchor harness for comparing
  baseline, hybrid, combined, and former calculation-heavy linked SBF sizes.
- `benchmark/sbf-composite/` — locked deployed-SBF harness for bracketing the
  complete `ln`/CDF/exp-affected composite call matrix with runtime inputs.
- `benchmark/nig_cu_report.json` and `benchmark/nig_footprint_report.json` —
  toolchain-specific NIG deployment measurements retained with their SBF
  harnesses in the repository.
- `benchmark/nig_independent_oracle_report.json` — the separate 50-digit NIG
  Bessel-density/Lewis cross-check retained alongside its reference script.
- `benchmark/american_kbi_{runtime,unseen}_accuracy_report.json`,
  `benchmark/american_kbi_release_report.json`, and
  `benchmark/nig_release_report.json` — machine-readable release evidence. The
  tagged implementation docs contain their measured summaries; the JSON does
  not ship in the crate archive.
- `scripts/generate_american_kbi_data.py` — deterministic generator for KBI's
  parameter-independent Q40 quadrature geometry and normal-kernel coefficients.
- `scripts/fit_american_kbi_price_weights.py` — reproduces KBI's nine positive
  QdFp-regularized empirical cubature weights from the fixed 48-contract
  training corpus; the weights are global integration coefficients, not prices.
- `scripts/validate_american_kbi_runtime.py` and
  `benchmark/american_kbi_{runtime,unseen}_accuracy_report.json` — compiled
  Rust KBI comparisons against QuantLib QdFp. No generated price is consumed
  by the live API.
- `scripts/certify_ln_fixed.py`, `scripts/certify_exp_fixed.py`, and
  `scripts/certify_norm_cdf.py` —
  source-digest-gated Arb/exact-integer release certificates. All three require
  the pinned `python-flint==0.8.0` dependency.

The repository-only assets are excluded from the crate tarball to keep install
and docs.rs builds small. CI rejects a release archive above 260 KiB. The crate
test suite still covers mul-div using exact edge vectors plus deterministic
property-style sweeps.

## Publish and provenance gate

`0.2.0` intentionally contains breaking feature/API changes relative to the
published `0.1.5`. Before publishing, run `cargo semver-checks check-release
--baseline-version 0.1.5 --all-features` and confirm that Cargo derives a
breaking release rather than treating this as a patch. Publish only from a
clean commit whose CI is green:

```bash
cargo publish --dry-run
git tag -s v0.2.0 -m "solmath 0.2.0"
git push origin v0.2.0
cargo publish
```

Push the signed tag first because documentation inside the crate links to
versioned repository evidence. The tag must point at the exact source commit
used by `cargo publish`. Verify the
crate checksum and docs.rs build after crates.io accepts the release. Registry
MFA, crate ownership, and the signing key are external controls and cannot be
verified from this repository.

## LUT and Linked-Binary Budgets

The reduced-domain `ln1p` and `expm1` tables have compile-time raw-payload
limits, so an oversized generated table fails normal compilation. The current
shared payload is 27,944 bytes. Normal CDF has no answer table: its 936 bytes
of coefficient/cutoff data have a separate 2 KiB compile-time cap. Its Q39
tail evaluation promotes the same stored Q23 coefficients at runtime and adds
no payload. The calculation-first exponential stores 304 bytes of Q22 minimax
coefficients and rounded Q62 fractional power-of-two reconstruction constants
under a 512-byte compile-time cap; it has no sampled-answer table. Linked size is
checked separately because code, helper selection,
alignment, and dead-section elimination cannot be inferred from Rust source
size:

```bash
./scripts/measure_sbf_footprint.sh
```

The harness currently measures linked increases of 19,104 bytes for `expm1`,
25,968 for `ln1p`, 40,952 for both, and 6,976 for `exp_fixed_i`. The final exp
path is 21,272 linked bytes smaller than its former rational implementation.
Its caps allow limited toolchain drift but reject exp growth beyond 10 KiB and
`expm1`/`ln1p`/combined growth beyond 22/34/50 KiB. Any new large lookup
structure requires the same three-way evidence: accuracy corpus, deployed CU
distribution, and linked SBF delta against an identical baseline.

To regenerate the larger offline assets from a full repository checkout:

```bash
python3 -m venv .venv
. .venv/bin/activate
pip install -r scripts/requirements.txt
python3 scripts/generate_production_vectors.py
python3 scripts/generate_adversarial_vectors.py
python3 scripts/generate_exp_coeffs.py --check
python3 scripts/generate_norm_cdf_coeffs.py
python3 scripts/certify_ln_fixed.py
python3 scripts/certify_exp_fixed.py
python3 scripts/certify_norm_cdf.py
python3 scripts/generate_bvn_phi2_references.py
python3 scripts/generate_american_kbi_data.py --check src/american_kbi_data.rs
python3 scripts/generate_barrier_vectors.py
python3 scripts/validate_asian_runtime.py --cases 500
python3 scripts/crosscheck_quantlib.py
```

## Capability evidence matrix

| Capability | Runtime status | Primary evidence | Current measured characteristic |
|---|---|---|---|
| Core fixed-point arithmetic | General runtime API | Deterministic property sweeps and thirteen Kani harnesses | Checked overflow and exact widened mul-div/sqrt fallback; 545/545 bit-precise checks, including U256 limb and sqrt-transition bounds plus `<1` and `<=0.5` ULP rounding lemmas |
| `ln` and normal CDF | All-input source-certified kernels | Arb intervals plus exact-integer monotonicity/symmetry analysis | `ln_fixed_i <= 3` ULP; `norm_cdf_poly <= 2` ULP for every `i128` |
| `exp` | Source-certified runtime kernel | Arb/exact-integer certificate and 100K/10K corpora | Relative error `<1.55e-16`; 961 average / 992 max CU |
| Token conversion and pool math | General runtime APIs | Exact rounding tests and pool-domain invariant sweeps | Explicit floor/ceil conversion; pool quote max 33,685 math CU |
| Black–Scholes and Greeks | Standard and HP runtime paths | 100K high-precision corpus and QuantLib BlackCalculator comparison | HP call/put max error 3/4 raw; 149,925 max CU for all Greeks |
| Implied volatility | Bounded iterative runtime solver | 100K/10K price-to-volatility round trips | 82,917 median / 328,660 max CU on accepted sampled quotes |
| Barrier options | State-aware runtime API | 443,520 QuantLib AnalyticBarrierEngine comparisons | All eight type/side combinations; 415,579 max math CU |
| Arithmetic-Asian / TWAP | Continuous-moment, partially fixed runtime model | 100K/10K mpmath plus 10K QuantLib references | Production max `$2.258e-8`; 182,458 practical max math CU |
| American KBI | Fully on-chain runtime model | Complete 100K/10K comparison plus held-out/unseen QuantLib QdFp surfaces | Production call/put max `$0.002744/$0.003264`; 390,628/390,786 max instruction CU |
| Exponential NIG | Fully on-chain bounded runtime model | 100K/10K CDF-density references and independent 50-digit density/Lewis checks | Max `$0.000565/$0.001397` per `$100`; 382,441 max instruction CU |
| Deterministic Heston (`xi = 0`) | Exact integrated-variance reduction | 200,704-case independent sweep | Call/put max 200/372 raw; 190,756 retained max CU |
| SABR | Analytics plus whole-grid certified execution | QuantLib references and parity/bounds/vertical/butterfly/calendar tests | Stored certified-node reads are 251 CU; largest tested certificate 607,665 CU |
| Bivariate normal and Phi2 tables | Guarded quadrature plus fixed-correlation tables | Independent quadrature corpus and certificate-ID/error-budget tests | `bvn_cdf` 135,944 sampled max CU; table lookup 1,440 max CU |
| Two-asset rainbow options | Analytic Stulz formulas using `bvn_cdf` | Stulz reference and Monte Carlo comparisons | Best-of and worst-of calls for positive/negative correlation |

The matrix describes the implementation and evidence that ship with `0.2.0`.
Model assumptions and runtime domains are part of each API contract; callers
receive `SolMathError` when a quote cannot be represented by that contract.

## Evidence index

The repository retains the full measurement trail rather than embedding large
reports in the crate archive:

- `.superstack/ln-proof-certificate-2026-07-12.md`,
  `.superstack/exp-proof-certificate-2026-07-12.md`, and
  `.superstack/norm-cdf-proof-2026-07-14-release.md` bind the source-certified
  kernels to their exact implementations.
- `.superstack/kani-arithmetic-verification-2026-07-14.md` records the current
  thirteen-harness, 545-check integer proof layer;
  `.superstack/kani-ulp-verification-2026-07-14.md` retains the preceding
  eight-harness ULP layer and `.superstack/kani-verification-2026-07-12.md`
  retains the original two-harness baseline.
- `.superstack/composite-cu-revalidation-2026-07-12.md` and
  `.superstack/release-cu-revalidation-2026-07-14.json` retain deployed-SBF CU
  measurements.
- `benchmark/american_kbi_*_report.json`, `benchmark/nig_*_report.json`, and
  `benchmark/asian_*_report.json` retain the model-specific accuracy, footprint,
  and compute results.

See [SECURITY.md](SECURITY.md) for runtime guarantees and integration
boundaries, and the model guides under `docs/` for equations and domains.
