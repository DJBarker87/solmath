#![cfg(feature = "sabr")]

// Keep the large generated QuantLib corpus outside the library unit-test
// build. This preserves repository coverage while allowing `cargo test` on
// the published crate, whose compact allowlist deliberately excludes
// repository-only generated corpora.
include!("../test_data/sabr_reference_tests.rs");
