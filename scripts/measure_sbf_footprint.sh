#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/benchmark/sbf-footprint/Cargo.toml"
artifact="$repo_root/benchmark/sbf-footprint/target/deploy/solmath_sbf_footprint.so"
log_dir="$(mktemp -d)"
trap 'rm -rf "$log_dir"' EXIT

cargo metadata --manifest-path "$manifest" --locked --no-deps --format-version 1 \
    >"$log_dir/metadata.json"

build_size() {
    local label="$1"
    local feature="$2"
    local -a command=(
        cargo build-sbf
        --manifest-path "$manifest"
        --no-default-features
    )
    if [[ -n "$feature" ]]; then
        command+=(--features "$feature")
    fi
    if ! "${command[@]}" >"$log_dir/$label.log" 2>&1; then
        cat "$log_dir/$label.log" >&2
        return 1
    fi
    wc -c <"$artifact" | tr -d ' '
}

baseline="$(build_size baseline "")"
exp="$(build_size exp exp)"
expm1="$(build_size expm1 expm1)"
ln1p="$(build_size ln1p ln1p)"
both="$(build_size both both)"
legacy_expm1="$(build_size legacy-expm1 legacy-expm1)"
kbi="$(build_size kbi kbi)"
nig="$(build_size nig nig)"

exp_delta=$((exp - baseline))
expm1_delta=$((expm1 - baseline))
ln1p_delta=$((ln1p - baseline))
both_delta=$((both - baseline))
legacy_expm1_delta=$((legacy_expm1 - baseline))
kbi_delta=$((kbi - baseline))
nig_delta=$((nig - baseline))

printf '%-16s %12s %12s\n' "variant" "SBF bytes" "delta"
printf '%-16s %12d %12d\n' "baseline" "$baseline" 0
printf '%-16s %12d %12d\n' "exp" "$exp" "$exp_delta"
printf '%-16s %12d %12d\n' "expm1" "$expm1" "$expm1_delta"
printf '%-16s %12d %12d\n' "ln1p" "$ln1p" "$ln1p_delta"
printf '%-16s %12d %12d\n' "both" "$both" "$both_delta"
printf '%-16s %12d %12d\n' "legacy-expm1" "$legacy_expm1" "$legacy_expm1_delta"
printf '%-16s %12d %12d\n' "kbi" "$kbi" "$kbi_delta"
printf '%-16s %12d %12d\n' "nig" "$nig" "$nig_delta"

status=0
check_delta() {
    local name="$1"
    local actual="$2"
    local budget="$3"
    if ((actual > budget)); then
        echo "$name linked delta $actual exceeds budget $budget" >&2
        status=1
    fi
}

check_delta expm1 "$expm1_delta" $((22 * 1024))
check_delta ln1p "$ln1p_delta" $((34 * 1024))
check_delta both "$both_delta" $((50 * 1024))
check_delta exp "$exp_delta" $((10 * 1024))
check_delta kbi "$kbi_delta" $((140 * 1024))
check_delta nig "$nig_delta" $((125 * 1024))
if ((expm1 >= legacy_expm1)); then
    echo "hybrid expm1 ($expm1 bytes) is not smaller than legacy expm1 ($legacy_expm1 bytes)" >&2
    status=1
fi
exit "$status"
