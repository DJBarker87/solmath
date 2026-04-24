# SolMath Validation

This file is the reproducibility checklist for SolMath releases. It separates
the small crates.io package from the larger repository validation assets so the
published crate stays usable while the full audit trail remains reproducible.

## Toolchains

The `0.1.4` release was checked with:

- Rust/Cargo release toolchain: `rustc 1.93.0`, `cargo 1.93.0`
- MSRV library check: `rustc 1.79.0`
- Package metadata: `rust-version = "1.79"`

The test suite uses dev-dependencies such as `proptest`; run the full tests on
stable. The MSRV job intentionally checks the library surface only:
`cargo +1.79.0 check --lib --all-features`.

## CI

GitHub Actions runs `.github/workflows/ci.yml` on push and pull requests:

```bash
cargo check --lib --all-features       # Rust 1.79.0 MSRV library check
git diff --check
cargo test --no-default-features
cargo test
cargo test --all-features
cargo check --examples --all-features
cargo package
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
cargo +stable package
```

Then verify the generated package, not only the working tree:

```bash
rm -rf /tmp/solmath-package
mkdir -p /tmp/solmath-package
tar -xzf target/package/solmath-0.1.4.crate -C /tmp/solmath-package
cd /tmp/solmath-package/solmath-0.1.4
cargo test --all-features
```

## Reference Assets

Included in the crates.io package:

- `benchmark/iv_vectors.json` — implied-vol recovery regression vectors.
- `test_data/heston_reference_tests.rs` — generated Heston reference tests.
- `test_data/sabr_reference_tests.rs` — generated SABR reference tests.
- `PROOFS.md` — analytical error-bound notes for core approximations.

Repository-only assets:

- `scripts/` — Python generators and QuantLib/mpmath cross-check scripts.
- `tests/reference/mul_div_vectors.json` — large generated mul-div corpus.

The repository-only assets are excluded from the crate tarball to keep install
and docs.rs builds small. CI and the crate test suite still cover mul-div using
exact edge vectors plus property tests.

From a full repository checkout, run the large mul-div corpus explicitly:

```bash
SOLMATH_FULL_VECTORS=1 cargo test --all-features validate_against_full_python_vectors_when_requested
```

To regenerate the larger offline assets from a full repository checkout:

```bash
python3 -m venv .venv
. .venv/bin/activate
pip install -r scripts/requirements.txt
python3 scripts/generate_production_vectors.py
python3 scripts/generate_adversarial_vectors.py
python3 scripts/generate_barrier_vectors.py
python3 scripts/crosscheck_quantlib.py
```

## Production Readiness Matrix

| Area | Status | Use today? | Notes |
|------|--------|------------|-------|
| Core fixed-point arithmetic | Internally tested, property-tested | Yes, with protocol review | Overflow returns `Err`, no silent wrapping. |
| Token conversion helpers | Internally tested | Yes, with explicit floor/ceil policy | Use floor for payouts and ceil for collections. |
| Weighted pool math | Internally tested | Candidate | Validate economic invariants for your pool parameters. |
| HP Black-Scholes | QuantLib cross-checked | Candidate | Best-supported pricing path; still unaudited. |
| Standard Black-Scholes | Internally tested | Candidate | Lower precision than HP path. |
| Implied volatility | Vector-tested | Caution | Solver returns `NoConvergence`; callers need fallback policy. |
| Barrier options | QuantLib cross-checked | Caution | Requires higher compute budget. |
| Heston, SABR, NIG | Reference-tested | Research/caution | Model risk dominates arithmetic risk; validate assumptions independently. |
| Bivariate CDF/table lookup | mpmath-vector tested | Research/caution | Accuracy degrades near extreme correlations; see README notes. |

## Audit Status

No independent third-party audit is claimed. Treat SolMath as unaudited
financial infrastructure until your integration has its own review or an
external audit covers the exact version and feature set you use.
