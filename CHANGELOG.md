# Changelog

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
