#!/usr/bin/env python3
"""Rigorous all-input certificate for the current ``ln_fixed_i`` kernel.

This is a proof checker, not a sampler.  It combines exact integer/rational
arithmetic with Arb ball arithmetic (via python-flint) and covers every
``u128`` input symbolically by binary exponent and LUT segment.

The checker is intentionally bound to the exact Rust functions and generated
tables that it proves.  A change to one of those function bodies makes the
kernel digest assertion fail until the proof is reviewed and refreshed.

Install the pinned proof dependency with::

    python3 -m pip install python-flint==0.8.0

Then run::

    python3 scripts/certify_ln_fixed.py
    python3 scripts/certify_ln_fixed.py --json

The certified error is relative to ``1e12 * ln(x / 1e12)``.  The final ULP
claim compares the Rust integer result with that real value rounded to the
nearest integer (either tie rule is covered).
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.metadata
import json
import re
from fractions import Fraction
from pathlib import Path

try:
    from flint import arb, ctx
except ImportError as exc:  # pragma: no cover - dependency failure path
    raise SystemExit(
        "ln certificate requires python-flint==0.8.0; install it with "
        "`python3 -m pip install python-flint==0.8.0`"
    ) from exc


REPO = Path(__file__).resolve().parents[1]
TRANS_SOURCE = REPO / "src" / "transcendental.rs"
CONSTANTS_SOURCE = REPO / "src" / "constants.rs"
LN_LUT_SOURCE = REPO / "src" / "ln_lut.rs"
LN2_LUT_SOURCE = REPO / "src" / "ln2_lut.rs"

PYTHON_FLINT_VERSION = "0.8.0"
ARB_DPS = 100
EXPECTED_KERNEL_SHA256 = "8c1200e8c3caff7185a4d12051f999ab608952996adafc7094b5c92b571a6a8e"
KERNEL_FUNCTIONS = (
    "round_shift_signed",
    "round_shift_i64",
    "mul_q42",
    "ln_mantissa_lut",
    "normalize_ln_fallback",
    "normalize_ln",
    "ln_fixed_i",
)

SCALE = 10**12
Q42 = 1 << 42
RECIP_GUARD = 1 << 32
SEGMENTS = 1024
STEP = SCALE // SEGMENTS
HALF_STEP = STEP // 2
NEAR_ONE_RAW = 1_000_000
REACHABLE_K_MIN = -40
REACHABLE_K_MAX = 88
I64_MAX = (1 << 63) - 1
I128_MAX = (1 << 127) - 1

# Outward-rounded public certificate thresholds.  Every candidate is checked
# against these values with Arb interval comparisons; the long diagnostic
# enclosures printed by the script are evidence, not assumptions.
ctx.dps = ARB_DPS
REGULAR_REAL_ERROR_LT = arb("2.925564")
SPECIAL_REAL_ERROR_LT = arb("1.499803")
NEAR_ONE_REAL_ERROR_LT = arb("0.5")
ROUNDED_ULP_LE = 3


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def extract_function(source: str, name: str) -> str:
    """Extract one Rust function body, including its signature."""

    match = re.search(rf"(?m)^(?:pub )?fn {re.escape(name)}\b", source)
    if match is None:
        raise AssertionError(f"missing Rust function {name}")
    brace = source.index("{", match.start())
    depth = 0
    for index in range(brace, len(source)):
        if source[index] == "{":
            depth += 1
        elif source[index] == "}":
            depth -= 1
            if depth == 0:
                return source[match.start() : index + 1]
    raise AssertionError(f"unterminated Rust function {name}")


def kernel_sha256() -> str:
    source = TRANS_SOURCE.read_text()
    bound_source = "\n".join(
        f"{name}\0{extract_function(source, name)}" for name in KERNEL_FUNCTIONS
    )
    return hashlib.sha256(bound_source.encode()).hexdigest()


def parse_rust_array(path: Path, name: str) -> list[int]:
    source = path.read_text()
    match = re.search(
        rf"const\s+{re.escape(name)}\s*:.*?=\s*\[(.*?)\];",
        source,
        flags=re.DOTALL,
    )
    if match is None:
        raise AssertionError(f"missing Rust array {name}")
    return [int(value) for value in re.findall(r"-?\d+", match.group(1))]


def parse_rust_integer(path: Path, name: str) -> int:
    source = path.read_text()
    match = re.search(
        rf"const\s+{re.escape(name)}\s*:.*?=\s*(-?[\d_]+)"
        rf"(?:u128|i128|u64|i64|usize|i32)?\s*;",
        source,
    )
    if match is None:
        raise AssertionError(f"missing Rust integer {name}")
    return int(match.group(1).replace("_", ""))


def round_nearest_away(numerator: int, denominator: int) -> int:
    """Exact model of the signed Rust round-shift helpers."""

    assert denominator > 0 and denominator % 2 == 0
    half = denominator // 2
    if numerator >= 0:
        return (numerator + half) // denominator
    return -((-numerator + half) // denominator)


def arb_fraction(value: Fraction) -> arb:
    return arb(value.numerator) / value.denominator


def strict_less(left: arb, right: arb, description: str) -> None:
    if not left < right:
        raise AssertionError(f"could not prove {description}: {left} < {right}")


def prove_normalization() -> tuple[int, int, int]:
    """Cover the fallback's 128 bit-length classes by monotone intervals.

    For a bit length ``b``, the initial exponent is ``b - 40`` and the first
    shifted mantissa is in ``[2^39, 2^40)``.  The only possible correction is
    therefore the ``m < SCALE`` branch.  The intervals below split exactly at
    that branch condition and check their endpoint images; monotonicity of a
    shift proves every integer inside each interval has the same bounds.
    """

    exponents: list[int] = []
    intervals = 0
    for bit_length in range(1, 129):
        x_low = 1 << (bit_length - 1)
        x_high = (1 << bit_length) - 1
        k0 = bit_length - 40
        covered = 0

        if k0 >= 0:
            divisor = 1 << k0
            low_branch_high = min(x_high, SCALE * divisor - 1)
            high_branch_low = max(x_low, SCALE * divisor)

            if x_low <= low_branch_high:
                k = k0 - 1
                assert k >= 0 or bit_length <= 40
                if k >= 0:
                    m_low = x_low >> k
                    m_high = low_branch_high >> k
                else:
                    m_low = x_low << -k
                    m_high = low_branch_high << -k
                assert SCALE <= m_low <= m_high < 2 * SCALE
                exponents.append(k)
                intervals += 1
                covered += low_branch_high - x_low + 1

            if high_branch_low <= x_high:
                k = k0
                m_low = high_branch_low >> k
                m_high = x_high >> k
                assert SCALE <= m_low <= m_high < 2 * SCALE
                exponents.append(k)
                intervals += 1
                covered += x_high - high_branch_low + 1

            if x_low <= low_branch_high and high_branch_low <= x_high:
                assert low_branch_high + 1 == high_branch_low
        else:
            multiplier = 1 << -k0
            low_branch_high = min(x_high, (SCALE - 1) // multiplier)
            high_branch_low = max(x_low, (SCALE + multiplier - 1) // multiplier)

            if x_low <= low_branch_high:
                k = k0 - 1
                m_low = x_low << -k
                m_high = low_branch_high << -k
                assert SCALE <= m_low <= m_high < 2 * SCALE
                exponents.append(k)
                intervals += 1
                covered += low_branch_high - x_low + 1

            if high_branch_low <= x_high:
                k = k0
                m_low = high_branch_low << -k
                m_high = x_high << -k
                assert SCALE <= m_low <= m_high < 2 * SCALE
                exponents.append(k)
                intervals += 1
                covered += x_high - high_branch_low + 1

            if x_low <= low_branch_high and high_branch_low <= x_high:
                assert low_branch_high + 1 == high_branch_low

        assert covered == x_high - x_low + 1

    assert min(exponents) == REACHABLE_K_MIN
    assert max(exponents) == REACHABLE_K_MAX
    assert all(REACHABLE_K_MIN <= k <= REACHABLE_K_MAX for k in exponents)

    # The two hot branches bypass the fallback but are the same normalized
    # relation: (x,0) on [S,2S), and (2x,-1) on [S/2,S).
    assert SCALE < 2 * SCALE <= (1 << 128)
    assert 2 * (SCALE // 2) == SCALE
    assert SCALE <= 2 * (SCALE - 1) < 2 * SCALE
    return min(exponents), max(exponents), intervals


def main() -> dict[str, object]:
    installed = importlib.metadata.version("python-flint")
    if installed != PYTHON_FLINT_VERSION:
        raise AssertionError(
            f"proof requires python-flint {PYTHON_FLINT_VERSION}, found {installed}"
        )

    # 100 decimal digits is far more than needed to separate every rounding
    # boundary in the current tables.  Arb propagates a rigorous radius.
    ctx.dps = ARB_DPS

    digest = kernel_sha256()
    assert digest == EXPECTED_KERNEL_SHA256, (
        "the certified ln Rust functions changed; review the proof and refresh "
        f"EXPECTED_KERNEL_SHA256 (found {digest})"
    )

    # SCALE and SCALE_I are imported into the certified functions from a
    # separate module, so bind their values explicitly as part of the proof.
    assert parse_rust_integer(CONSTANTS_SOURCE, "SCALE") == SCALE
    assert parse_rust_integer(CONSTANTS_SOURCE, "SCALE_I") == SCALE
    assert parse_rust_integer(LN_LUT_SOURCE, "LN_LUT_SEGMENTS") == SEGMENTS
    assert parse_rust_integer(LN_LUT_SOURCE, "LN_LUT_STEP") == STEP
    assert parse_rust_integer(LN_LUT_SOURCE, "LN_LUT_HALF_STEP") == HALF_STEP
    assert parse_rust_integer(LN2_LUT_SOURCE, "K_LN2_MIN") == -64
    assert parse_rust_integer(LN2_LUT_SOURCE, "K_LN2_MAX") == 88

    midpoint_logs = parse_rust_array(LN_LUT_SOURCE, "LN_LUT_MID_LOG")
    reciprocals = parse_rust_array(LN_LUT_SOURCE, "LN_Q42_RECIP_G32")
    k_ln2 = parse_rust_array(LN2_LUT_SOURCE, "K_LN2_RAW")
    assert len(midpoint_logs) == len(reciprocals) == SEGMENTS
    assert len(k_ln2) == 153

    k_min, k_max, normalization_intervals = prove_normalization()

    half = arb(1) / 2
    ln2 = arb(2).log()
    table_errors: list[arb] = []
    for j, stored in enumerate(midpoint_logs):
        midpoint = SCALE + j * STEP + HALF_STEP
        truth = arb(SCALE) * (arb(midpoint) / SCALE).log()
        strict_less(arb(stored) - half, truth, f"midpoint[{j}] lower rounding edge")
        strict_less(truth, arb(stored) + half, f"midpoint[{j}] upper rounding edge")
        table_errors.append(arb(stored) - truth)

        # The reciprocal is nearest-integer Q42 with 32 extra guard bits.
        reciprocal_residual = abs(
            reciprocals[j] * midpoint - Q42 * RECIP_GUARD
        )
        assert 2 * reciprocal_residual <= midpoint

    k_errors: dict[int, arb] = {}
    for index, stored in enumerate(k_ln2):
        k = index - 64
        truth = arb(k * SCALE) * ln2
        strict_less(arb(stored) - half, truth, f"k_ln2[{k}] lower rounding edge")
        strict_less(truth, arb(stored) + half, f"k_ln2[{k}] upper rounding edge")
        if REACHABLE_K_MIN <= k <= REACHABLE_K_MAX:
            k_errors[k] = arb(stored) - truth

    # Near-one branch.  The error is monotone in |x-SCALE| on each side, and
    # the negative side is the larger endpoint.
    near_delta = NEAR_ONE_RAW - 1
    near_positive = arb(near_delta) - arb(SCALE) * (
        arb(1) + arb(near_delta) / SCALE
    ).log()
    near_negative = -arb(near_delta) - arb(SCALE) * (
        arb(1) - arb(near_delta) / SCALE
    ).log()
    near_error = near_positive.max(near_negative)
    strict_less(near_error, NEAR_ONE_REAL_ERROR_LT, "near-one real error")
    strict_less(near_error + half, arb(4), "near-one correctly-rounded ULP < 4")

    max_d_times_recip = 0
    max_q_abs = 0
    max_q_square = 0
    max_q2_times_q = 0
    max_local_times_scale = 0
    largest_local_bound = arb(0)
    largest_regular_bound = arb(0)
    regular_maximizer = (0, 0)
    regular_maximizer_midpoint = -1.0

    for j, (stored_log, reciprocal, table_error) in enumerate(
        zip(midpoint_logs, reciprocals, table_errors)
    ):
        midpoint = SCALE + j * STEP + HALF_STEP
        d_low = -HALF_STEP + (1 if j == 0 else 0)  # m=SCALE is special-cased
        d_high = HALF_STEP - 1
        d_abs = max(abs(d_low), abs(d_high))

        endpoint_q = [
            round_nearest_away(d_low * reciprocal, RECIP_GUARD),
            round_nearest_away(d_high * reciprocal, RECIP_GUARD),
        ]
        q_abs = max(abs(q) for q in endpoint_q)
        q_ratio = Fraction(q_abs, Q42)
        t_ratio = Fraction(d_abs, midpoint)
        z_ratio = max(q_ratio, t_ratio)

        reciprocal_residual = abs(reciprocal * midpoint - Q42 * RECIP_GUARD)
        q_error_units = Fraction(1, 2) + Fraction(
            d_abs * reciprocal_residual, midpoint * RECIP_GUARD
        )

        # Integer cubic evaluation.  q2 contributes at most 3/4 Q42 unit
        # after /2.  q3 contributes at most 5/6 + |q|/(6Q42) after /3.
        integer_cubic_units = Fraction(19, 12) + q_ratio / 6

        # |p'(z)| for p(z)=z-z^2/2+z^3/3 is <= 1+Z+Z^2 on
        # the interval connecting the exact d/midpoint and quantized q/Q42.
        cubic_derivative = 1 + z_ratio + z_ratio * z_ratio

        # The log-series tail is sum_{n>=4} (-1)^(n+1)t^n/n.
        series_tail = (
            SCALE * t_ratio**4 / (4 * (1 - t_ratio))
        )
        local_bound = (
            Fraction(1, 2)
            + Fraction(SCALE, Q42) * integer_cubic_units
            + Fraction(SCALE, Q42) * cubic_derivative * q_error_units
            + series_tail
        )
        local_bound_arb = arb_fraction(local_bound)
        largest_local_bound = largest_local_bound.max(local_bound_arb)

        # Overflow proof.  Endpoint checks suffice because q is a monotone
        # rounded linear function of d inside each segment.
        d_times_recip = max(abs(d_low * reciprocal), abs(d_high * reciprocal))
        q2_abs = round_nearest_away(q_abs * q_abs, Q42)
        q3_product_abs = q2_abs * q_abs
        q3_abs = round_nearest_away(q3_product_abs, Q42)
        local_abs = q_abs + (q2_abs + 1) // 2 + (q3_abs + 2) // 3
        max_d_times_recip = max(max_d_times_recip, d_times_recip)
        max_q_abs = max(max_q_abs, q_abs)
        max_q_square = max(max_q_square, q_abs * q_abs)
        max_q2_times_q = max(max_q2_times_q, q3_product_abs)
        max_local_times_scale = max(max_local_times_scale, local_abs * SCALE)

        assert d_times_recip <= I64_MAX
        assert q_abs * q_abs <= I64_MAX
        assert q3_product_abs <= I64_MAX
        assert local_abs * SCALE <= I128_MAX
        # The helpers add half a denominator before shifting.  Check those
        # intermediate numerators too, including the negation path: none of
        # the certified values can be the signed minimum, so abs(value)+half
        # covers both signs exactly.
        assert d_times_recip + RECIP_GUARD // 2 <= I64_MAX
        assert q_abs * q_abs + Q42 // 2 <= I64_MAX
        assert q3_product_abs + Q42 // 2 <= I64_MAX
        assert local_abs * SCALE + Q42 // 2 <= I128_MAX
        assert local_abs <= I64_MAX
        assert 0 <= j < SEGMENTS
        assert -(1 << 63) <= midpoint + d_low <= midpoint + d_high <= I64_MAX

        # For k>0, x=m*2^k+r and normalization discards r.  This contributes
        # N=S*ln(1+r/(m*2^k)), bounded at the smallest regular m in the
        # segment and r=2^k-1.  For k<=0 normalization is exact.
        for k in range(REACHABLE_K_MIN, REACHABLE_K_MAX + 1):
            constant_error = table_error + k_errors[k]
            constant_and_normalization = abs(constant_error).upper()
            if k > 0:
                m_min = SCALE + j * STEP + (1 if j == 0 else 0)
                normalization_max = arb(SCALE) * (
                    arb(1)
                    + arb((1 << k) - 1) / arb(m_min * (1 << k))
                ).log()
                other_endpoint = abs(constant_error - normalization_max).upper()
                constant_and_normalization = constant_and_normalization.max(
                    other_endpoint
                )

            total_bound = constant_and_normalization + local_bound_arb
            strict_less(
                total_bound,
                REGULAR_REAL_ERROR_LT,
                f"regular real error at segment={j}, k={k}",
            )
            largest_regular_bound = largest_regular_bound.max(total_bound)
            midpoint_float = float(total_bound)
            if midpoint_float > regular_maximizer_midpoint:
                regular_maximizer_midpoint = midpoint_float
                regular_maximizer = (j, k)

    # m=SCALE bypasses the midpoint table and cubic.  Only the pre-rounded
    # whole k*ln(2) constant and (for k>0) discarded normalization bits remain.
    largest_special_bound = arb(0)
    special_maximizer = 0
    special_maximizer_midpoint = -1.0
    for k in range(REACHABLE_K_MIN, REACHABLE_K_MAX + 1):
        special_bound = abs(k_errors[k]).upper()
        if k > 0:
            normalization_max = arb(SCALE) * (
                arb(1) + arb((1 << k) - 1) / arb(SCALE * (1 << k))
            ).log()
            special_bound = special_bound.max(
                abs(k_errors[k] - normalization_max).upper()
            )
        strict_less(
            special_bound,
            SPECIAL_REAL_ERROR_LT,
            f"m=SCALE real error at k={k}",
        )
        largest_special_bound = largest_special_bound.max(special_bound)
        midpoint_float = float(special_bound)
        if midpoint_float > special_maximizer_midpoint:
            special_maximizer_midpoint = midpoint_float
            special_maximizer = k

    strict_less(
        largest_special_bound + half,
        arb(4),
        "m=SCALE correctly-rounded ULP < 4",
    )

    # Nearest-integer reference error is <= 0.5.  Since both Rust and the
    # reference are integers, a strict difference below 4 proves <= 3 ULP.
    rounded_reference_bound = largest_regular_bound + half
    strict_less(rounded_reference_bound, arb(4), "correctly-rounded ULP < 4")

    max_output_abs = (
        max(abs(value) for value in midpoint_logs)
        + max(abs(value) for value in k_ln2)
        + (max_local_times_scale + Q42 // 2) // Q42
    )
    assert max_output_abs <= I128_MAX

    return {
        "schema": "solmath-ln-fixed-certificate-v1",
        "checker_sha256": sha256(Path(__file__).resolve()),
        "proof_engine": f"python-flint {installed} / Arb at {ctx.dps} decimal digits",
        "kernel_function_sha256": digest,
        "source_sha256": {
            "src/transcendental.rs": sha256(TRANS_SOURCE),
            "src/constants.rs": sha256(CONSTANTS_SOURCE),
            "src/ln_lut.rs": sha256(LN_LUT_SOURCE),
            "src/ln2_lut.rs": sha256(LN2_LUT_SOURCE),
        },
        "domain": {
            "valid_x": "1..=u128::MAX",
            "x_zero": "DomainError",
            "normalization_k": [k_min, k_max],
            "normalization_monotone_intervals": normalization_intervals,
            "segments": SEGMENTS,
            "segment_exponent_pairs_checked": SEGMENTS
            * (REACHABLE_K_MAX - REACHABLE_K_MIN + 1),
        },
        "table_checks": {
            "midpoint_logs_correctly_rounded": len(midpoint_logs),
            "reciprocals_nearest_integer": len(reciprocals),
            "k_ln2_correctly_rounded": len(k_ln2),
        },
        "real_error": {
            "near_one_computed_enclosure": str(near_error),
            "near_one_proved_lt": "0.5",
            "m_equals_scale_computed_enclosure": str(largest_special_bound),
            "m_equals_scale_maximizer_k": special_maximizer,
            "m_equals_scale_proved_lt": "1.499803",
            "largest_local_computed_enclosure": str(largest_local_bound),
            "regular_computed_enclosure": str(largest_regular_bound),
            "regular_maximizer_segment_k": list(regular_maximizer),
            "regular_proved_lt": "2.925564",
        },
        "correctly_rounded_reference": {
            "computed_triangle_enclosure": str(rounded_reference_bound),
            "integer_ulp_bound": ROUNDED_ULP_LE,
        },
        "overflow_maxima": {
            "abs_d_times_recip_i64": max_d_times_recip,
            "abs_d_times_recip_plus_rounding_half_i64": max_d_times_recip
            + RECIP_GUARD // 2,
            "abs_q_q42": max_q_abs,
            "q_square_i64": max_q_square,
            "q_square_plus_rounding_half_i64": max_q_square + Q42 // 2,
            "abs_q2_times_q_i64": max_q2_times_q,
            "abs_q2_times_q_plus_rounding_half_i64": max_q2_times_q + Q42 // 2,
            "abs_local_times_scale_i128": max_local_times_scale,
            "abs_local_times_scale_plus_rounding_half_i128": max_local_times_scale
            + Q42 // 2,
            "abs_return_conservative_i128": max_output_abs,
        },
        "result": "PASS",
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true", help="emit the certificate as JSON")
    args = parser.parse_args()
    certificate = main()
    if args.json:
        print(json.dumps(certificate, indent=2, sort_keys=True))
    else:
        error = certificate["real_error"]
        rounded = certificate["correctly_rounded_reference"]
        print("ln_fixed_i all-input certificate: PASS")
        print(f"  kernel sha256: {certificate['kernel_function_sha256']}")
        print(f"  regular real error: < {error['regular_proved_lt']} raw units")
        print(f"  near-one real error: < {error['near_one_proved_lt']} raw units")
        print(
            "  correctly-rounded reference: <= "
            f"{rounded['integer_ulp_bound']} ULP"
        )
