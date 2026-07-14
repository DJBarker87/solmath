#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$repo_root/target/critical-invariants}"

cargo test --offline --test critical_invariants --no-default-features
cargo test --offline --test critical_invariants --all-features
RUSTFLAGS="-C overflow-checks=on" cargo test --offline --release --test critical_invariants --all-features
RUSTFLAGS="-C overflow-checks=off" cargo test --offline --release --test critical_invariants --all-features
