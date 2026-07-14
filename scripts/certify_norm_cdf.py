#!/usr/bin/env python3
"""Rigorous certificate for the current Q44/Q23 ``norm_cdf_poly`` kernel.

This script reads the coefficients and fixed-point parameters from the Rust
sources, then proves four separate claims:

1. each exact, quantized polynomial approximates the corresponding normal CDF
   (or survival probability) on its complete continuous interval;
2. Q44 coordinate mapping and rounded integer Horner evaluation preserve an
   all-input error bound below 2.5 raw output units;
3. every dispatch seam and the half-raw tail cutoff are correctly ordered; and
4. the actual integer output is nondecreasing over the complete discrete i128
   input domain.

Continuous bounds use Arb balls (python-flint).  They are not sampling claims:
sampled values are elevated to interval-wide bounds with the mean-value
theorem, using a purely analytical third-derivative majorant.  Discrete
monotonicity is reduced to a finite set of possibly ambiguous final-rounding
cells.  A generated Rust verifier exhausts every raw input pair in those cells
using the same i128 rounding recurrences as the production kernel.

Reproduce with:

    python3 -m pip install --target /tmp/solmath-proof-deps python-flint==0.8.0
    PYTHONPATH=/tmp/solmath-proof-deps python3 scripts/certify_norm_cdf.py

The generated verifier and candidate-cell file live in a temporary directory;
the repository is not modified by running this certificate.
"""

from __future__ import annotations

import argparse
import hashlib
import math
import re
import struct
import subprocess
import tempfile
import time
from dataclasses import dataclass
from pathlib import Path

try:
    import flint
    from flint import arb, ctx, fmpq
except ImportError as exc:  # pragma: no cover - exercised by dependency failure
    raise SystemExit(
        "python-flint==0.8.0 is required; install it into an isolated directory "
        "and put that directory on PYTHONPATH"
    ) from exc


ROOT = Path(__file__).resolve().parents[1]
COEFF_SOURCE = ROOT / "src" / "norm_cdf_coeffs.rs"
KERNEL_SOURCE = ROOT / "src" / "normal.rs"
CONSTANT_SOURCE = ROOT / "src" / "constants.rs"
SCALE = 10**12
Q = 1 << 44
GUARD = 1 << 23
TAIL_EXTRA_Q = 16
HW_RAW = SCALE // 4
RECIPROCAL = 1_237_940_039_285_380
TAIL_CUTOFF = 7_130_506_848_171
ARB_BITS = 256
ERROR_GRID = 100_000
DERIVATIVE_GRID = 100_000
SECOND_GRID = 10_000
SIGN_GRID = 4096
EXPECTED_COEFFICIENT_SHA256 = "12e6200c0985ebc9f5a73b3a6585ca1c294a0ec34b1af85338e44f2bc41340ef"
EXPECTED_MODELED_KERNEL_SHA256 = "dece876f81c6e35c67768878ef89eb88429c8e04beb63a64996e5f9e68f78994"
MODELED_FUNCTIONS = (
    "round_shift_cdf",
    "horner_guard_q44",
    "horner_tail_guard_q44",
    "poly_map_t_q44",
    "norm_cdf_positive_tail",
    "norm_cdf_poly",
)


@dataclass(frozen=True)
class Piece:
    name: str
    lo_num: int  # exact sigma endpoint numerator over 2
    hi_num: int
    tail: bool
    coefficients: tuple[int, ...]

    @property
    def lo_raw(self) -> int:
        return self.lo_num * SCALE // 2

    @property
    def hi_raw(self) -> int:
        return self.hi_num * SCALE // 2

    @property
    def mid_raw(self) -> int:
        return (self.lo_raw + self.hi_raw) // 2

    @property
    def degree(self) -> int:
        return max(i for i, value in enumerate(self.coefficients) if value != 0)


PIECE_LAYOUT = (
    ("NORM_CDF_0_05_Q23", 0, 1, False),
    ("NORM_CDF_05_10_Q23", 1, 2, False),
    ("NORM_CDF_10_15_Q23", 2, 3, False),
    ("NORM_CDF_15_20_Q23", 3, 4, False),
    ("NORM_CDF_20_25_Q23", 4, 5, False),
    ("NORM_CDF_25_30_Q23", 5, 6, False),
    ("NORM_CDF_30_35_Q23", 6, 7, False),
    ("NORM_CDF_35_40_Q23", 7, 8, False),
    ("NORM_CDF_40_45_Q23", 8, 9, False),
    ("NORM_CDF_45_50_Q23", 9, 10, False),
    ("NORM_TAIL_50_55_Q23", 10, 11, True),
    ("NORM_TAIL_55_60_Q23", 11, 12, True),
    ("NORM_TAIL_60_65_Q23", 12, 13, True),
    ("NORM_TAIL_65_70_Q23", 13, 14, True),
)


def extract_rust_function(source: str, name: str) -> str:
    """Extract one complete Rust function, including its inline attribute."""
    match = re.search(
        rf"(?m)^(?:pub\(crate\) |pub )?(?:#\[[^\n]+\]\n)*fn {name}"
        rf"(?:<[^\n]+>)?\s*\(",
        source,
    )
    assert match, f"cannot find modeled Rust function {name}"
    brace = source.find("{", match.end())
    assert brace >= 0
    depth = 0
    for index in range(brace, len(source)):
        if source[index] == "{":
            depth += 1
        elif source[index] == "}":
            depth -= 1
            if depth == 0:
                return source[match.start() : index + 1]
    raise AssertionError(f"unterminated modeled Rust function {name}")


def parse_sources() -> tuple[list[Piece], str, str, str]:
    coefficient_text = COEFF_SOURCE.read_text()
    kernel_text = KERNEL_SOURCE.read_text()
    constant_text = CONSTANT_SOURCE.read_text()
    arrays: dict[str, tuple[int, ...]] = {}
    pattern = re.compile(
        r"const (NORM_(?:CDF|TAIL)_[A-Z0-9_]+): \[i64; \d+\] = \[(.*?)\];",
        re.S,
    )
    for name, body in pattern.findall(coefficient_text):
        arrays[name] = tuple(
            int(token.replace("_", ""))
            for token in re.findall(r"-?[0-9][0-9_]*", body)
        )

    assert "pub(crate) const CDF_T_Q: u32 = 44;" in coefficient_text
    assert "pub(crate) const CDF_COEFF_GUARD_Q: u32 = 23;" in coefficient_text
    assert "pub(crate) const CDF_TAIL_EVAL_EXTRA_Q: u32 = 16;" in coefficient_text
    cutoff_match = re.search(
        r"NORM_TAIL_HALF_RAW_CUTOFF: i128 = ([0-9_]+);", coefficient_text
    )
    assert cutoff_match
    assert int(cutoff_match.group(1).replace("_", "")) == TAIL_CUTOFF
    assert "round_shift_cdf(result as i128 * t as i128) + coefficient" in kernel_text
    assert "tail.clamp(0, SCALE_I as i64)" in kernel_text
    assert "if x < -8 * SCALE_I" in kernel_text
    assert "if x > 8 * SCALE_I" in kernel_text

    # Bind the certificate to the exact production recurrences and dispatch.
    # A source edit must deliberately update this digest and rerun the proof;
    # a few token-level assertions are not accepted as equivalence evidence.
    modeled_kernel = "\n\n".join(
        extract_rust_function(kernel_text, name) for name in MODELED_FUNCTIONS
    ) + "\n"
    assert sha256_text(coefficient_text) == EXPECTED_COEFFICIENT_SHA256
    assert sha256_text(modeled_kernel) == EXPECTED_MODELED_KERNEL_SHA256
    scale_match = re.search(r"pub const SCALE_I: i128 = ([0-9_]+)i128;", constant_text)
    assert scale_match
    assert int(scale_match.group(1).replace("_", "")) == SCALE

    pieces = [
        Piece(name, lo, hi, tail, arrays[name])
        for name, lo, hi, tail in PIECE_LAYOUT
    ]
    return pieces, coefficient_text, kernel_text, modeled_kernel


def certify_integer_safety(pieces: list[Piece]) -> None:
    """Machine-check every width assumption used by the integer model."""
    i64_max = (1 << 63) - 1
    i128_max = (1 << 127) - 1
    i128_min = -(1 << 127)
    q_half = Q // 2

    # The public guards execute before x.abs().  In particular i128::MIN takes
    # the first return, while every value reaching abs is in [-8S, 8S].
    assert i128_min < -8 * SCALE < 8 * SCALE < i128_max
    assert abs(-8 * SCALE) < i128_max

    # Every current dispatch call uses |ax-mid| <= SCALE/4.  The mapping
    # multiplication, sign-aware negation, and half addition all fit i128.
    assert max(abs(piece.lo_raw - piece.mid_raw) for piece in pieces) == HW_RAW
    assert max(abs(piece.hi_raw - piece.mid_raw) for piece in pieces) == HW_RAW
    map_product = HW_RAW * RECIPROCAL
    assert map_product + q_half <= i128_max
    assert -map_product > i128_min
    assert -i64_max > -(1 << 63)
    for piece in pieces:
        assert map_t(piece.lo_raw, piece.mid_raw) == -Q
        assert map_t(piece.hi_raw, piece.mid_raw) == Q

        # With |t| <= Q, rounded multiplication cannot increase accumulator
        # magnitude. Summed absolute coefficients therefore bound every cast,
        # add, final rounding, and i128 multiplication in every Horner step.
        extra_q = TAIL_EXTRA_Q if piece.tail else 0
        bound = abs(piece.coefficients[-1]) << extra_q
        assert bound <= i64_max
        for coefficient in reversed(piece.coefficients[:-1]):
            assert bound * Q + q_half <= i128_max
            assert bound <= i64_max  # round_shift_cdf result -> i64 cast
            bound += abs(coefficient) << extra_q
            assert bound <= i64_max  # i64 coefficient addition
        output_guard = GUARD << extra_q
        assert bound + output_guard // 2 <= i128_max
        assert bound + output_guard // 2 <= i64_max
        assert (bound + output_guard // 2) // output_guard <= i64_max

    # Tail subtraction, clamp/cast, symmetry, and all midpoint expressions fit.
    assert 0 <= SCALE <= i64_max
    assert 27 * SCALE <= i128_max


def rn_div(value: int, divisor: int) -> int:
    """Round to nearest with half ties away from zero, exactly as Rust."""
    if value >= 0:
        return (value + divisor // 2) // divisor
    return -((-value + divisor // 2) // divisor)


def map_t(raw_x: int, midpoint: int) -> int:
    return rn_div((raw_x - midpoint) * RECIPROCAL, Q)


def horner_integer(coefficients: tuple[int, ...], t: int, extra_q: int = 0) -> int:
    result = coefficients[-1] << extra_q
    for coefficient in reversed(coefficients[:-1]):
        result = rn_div(result * t, Q) + (coefficient << extra_q)
    return result


def rounded_piece_value(piece: Piece, raw_x: int) -> int:
    extra_q = TAIL_EXTRA_Q if piece.tail else 0
    guarded = horner_integer(
        piece.coefficients, map_t(raw_x, piece.mid_raw), extra_q
    )
    return rn_div(guarded, GUARD << extra_q)


def evaluation_guard(piece: Piece) -> int:
    return GUARD << (TAIL_EXTRA_Q if piece.tail else 0)


def arb_poly(coefficients: tuple[int, ...], t: arb, derivative: int = 0) -> arb:
    derived = list(coefficients)
    for _ in range(derivative):
        derived = [i * derived[i] for i in range(1, len(derived))]
    if not derived:
        return arb(0)
    result = arb(derived[-1]) / GUARD
    for coefficient in reversed(derived[:-1]):
        result = result * t + arb(coefficient) / GUARD
    return result


def phi(x: arb) -> arb:
    return (-x * x / 2).exp() / (arb(2) * arb.pi()).sqrt()


def target(piece: Piece, t: arb, derivative: int = 0) -> arb:
    midpoint = arb(piece.lo_num + piece.hi_num) / 4
    half_width = arb(piece.hi_num - piece.lo_num) / 4
    x = midpoint + half_width * t
    scale = arb(SCALE)
    sign = -1 if piece.tail else 1
    if derivative == 0:
        if piece.tail:
            return scale * (x / arb(2).sqrt()).erfc() / 2
        return scale * (arb(1) + (x / arb(2).sqrt()).erf()) / 2
    density = phi(x)
    if derivative == 1:
        return sign * scale * half_width * density
    if derivative == 2:
        return -sign * scale * half_width**2 * x * density
    if derivative == 3:
        return sign * scale * half_width**3 * (x * x - 1) * density
    raise ValueError("only derivatives 0..3 are used")


def upper_abs(value: arb) -> arb:
    return abs(value).abs_upper()


def grid_max(piece: Piece, derivative: int, count: int) -> arb:
    maximum = arb(0)
    for index in range(count + 1):
        t = arb(-1) + arb(2 * index) / count
        error = arb_poly(piece.coefficients, t, derivative) - target(
            piece, t, derivative
        )
        maximum = maximum.max(upper_abs(error))
    return maximum


def analytical_third_bound(piece: Piece) -> arb:
    coefficients = piece.coefficients
    polynomial = arb(0)
    for i in range(3, len(coefficients)):
        polynomial += arb(abs(i * (i - 1) * (i - 2) * coefficients[i])) / GUARD

    lo = arb(piece.lo_num) / 2
    hi = arb(piece.hi_num) / 2
    lo_x2_minus_one = abs(fmpq(piece.lo_num * piece.lo_num, 4) - 1)
    hi_x2_minus_one = abs(fmpq(piece.hi_num * piece.hi_num, 4) - 1)
    max_x2_minus_one = (
        lo_x2_minus_one
        if lo_x2_minus_one > hi_x2_minus_one
        else hi_x2_minus_one
    )
    # phi is decreasing for x >= 0, so phi(lo) is a rigorous interval-wide max.
    target_bound = (
        arb(SCALE)
        * (arb(1) / 4) ** 3
        * (arb(int(max_x2_minus_one.numerator)) / int(max_x2_minus_one.denominator))
        * phi(lo)
    )
    del hi  # documents that the complete interval was considered above
    return polynomial + target_bound.abs_upper()


def derivative_sign_bound(piece: Piece) -> arb:
    """Prove the exact polynomial has the required sign on all t in [-1, 1]."""
    minimum_oriented = None
    orientation = -1 if piece.tail else 1
    for index in range(SIGN_GRID):
        lo = arb(-1) + arb(2 * index) / SIGN_GRID
        hi = arb(-1) + arb(2 * (index + 1)) / SIGN_GRID
        interval = lo.union(hi)
        oriented = orientation * arb_poly(piece.coefficients, interval, 1)
        assert oriented > 0, f"unproved derivative sign in {piece.name} cell {index}"
        lower = oriented.lower()
        minimum_oriented = lower if minimum_oriented is None else minimum_oriented.min(lower)
    assert minimum_oriented is not None and minimum_oriented > 0
    return minimum_oriented


@dataclass
class PieceCertificate:
    piece: Piece
    approximation: arb
    map_error: arb
    integer_error: arb
    total_error: arb
    derivative_min_guard: arb
    monotone_by_margin: bool


def certify_piece(piece: Piece) -> PieceCertificate:
    m3 = analytical_third_bound(piece)
    second_sample = grid_max(piece, 2, SECOND_GRID)
    m2 = second_sample + m3 / SECOND_GRID
    derivative_sample = grid_max(piece, 1, DERIVATIVE_GRID)
    lipschitz = derivative_sample + m2 / DERIVATIVE_GRID
    error_sample = grid_max(piece, 0, ERROR_GRID)
    approximation = error_sample + lipschitz / ERROR_GRID

    derivative_min_raw = derivative_sign_bound(piece)
    eval_guard = evaluation_guard(piece)
    derivative_min_guard = derivative_min_raw * eval_guard

    reciprocal_residual = abs(RECIPROCAL * HW_RAW - Q * Q)
    map_delta_t = (arb(1) / 2 + arb(reciprocal_residual) / Q) / Q
    polynomial_derivative_bound = arb(0)
    for i in range(1, len(piece.coefficients)):
        polynomial_derivative_bound += arb(i * abs(piece.coefficients[i])) / GUARD
    map_error = polynomial_derivative_bound * map_delta_t

    integer_error = arb(1) / 2 + arb(piece.degree) / (2 * eval_guard)
    total = approximation + map_error + integer_error

    minimum_t_step = RECIPROCAL // Q
    guarded_step_lower = derivative_min_guard * minimum_t_step / Q
    monotone_by_margin = guarded_step_lower > piece.degree
    return PieceCertificate(
        piece,
        approximation,
        map_error,
        integer_error,
        total,
        derivative_min_guard,
        monotone_by_margin,
    )


def exact_poly_num_den(piece: Piece, t: int) -> tuple[int, int]:
    """Exact rational guarded polynomial at z=t/Q."""
    numerator = piece.coefficients[piece.degree]
    denominator = 1
    for coefficient in reversed(piece.coefficients[: piece.degree]):
        numerator = numerator * t + coefficient * denominator * Q
        denominator *= Q
    return numerator, denominator


def exact_poly_fmpq(piece: Piece, raw_x: int) -> fmpq:
    """Exact guarded polynomial, evaluated by FLINT rational arithmetic."""
    z = fmpq(map_t(raw_x, piece.mid_raw), Q)
    result = fmpq(piece.coefficients[piece.degree])
    for coefficient in reversed(piece.coefficients[: piece.degree]):
        result = result * z + coefficient
    return result


def compare_poly_to_twice_guarded(piece: Piece, raw_x: int, rhs_twice: int) -> int:
    """Compare exact polynomial in evaluation-guard units with rhs_twice / 2."""
    extra_scale = 1 << (TAIL_EXTRA_Q if piece.tail else 0)
    value = exact_poly_fmpq(piece, raw_x) * extra_scale
    difference = 2 * value - rhs_twice
    return (difference > 0) - (difference < 0)


def float_poly(piece: Piece, t: float) -> tuple[float, float]:
    coefficients = [value / GUARD for value in piece.coefficients[: piece.degree + 1]]
    value = coefficients[-1]
    derivative = 0.0
    for coefficient in reversed(coefficients[:-1]):
        derivative = derivative * t + value
        value = value * t + coefficient
    return value, derivative


def candidate_intervals(
    piece_index: int,
    certificate: PieceCertificate,
) -> tuple[list[tuple[int, int, int]], int]:
    """Return every raw interval where final rounding could reverse direction."""
    piece = certificate.piece
    degree = piece.degree
    # Every production polynomial owns (lo, hi]. The exact seam at lo belongs
    # to the preceding piece (and x=0 has a special return); certify_seams
    # checks those cross-dispatch pairs separately.
    lo = piece.lo_raw + 1
    hi = piece.hi_raw
    extra_scale = 1 << (TAIL_EXTRA_Q if piece.tail else 0)
    eval_guard = evaluation_guard(piece)
    lo_exact = exact_poly_fmpq(piece, lo) * extra_scale
    hi_exact = exact_poly_fmpq(piece, hi) * extra_scale
    value_min_exact = lo_exact if lo_exact < hi_exact else hi_exact
    value_max_exact = hi_exact if lo_exact < hi_exact else lo_exact

    # A final raw-output transition k-1 -> k occurs at (k-1/2)*eval_guard.
    # Include exactly every k whose guarded uncertainty band
    # [(k-1/2)G-d/2, (k-1/2)G+d/2] intersects the polynomial endpoint range.
    # These floor/ceil operations are FLINT exact rationals; binary floats are
    # used only later to propose a root center whose bracket is exact-checked.
    first_k = int(
        ((2 * value_min_exact - degree + eval_guard) / (2 * eval_guard)).ceil()
    )
    last_k = int(
        ((2 * value_max_exact + degree + eval_guard) / (2 * eval_guard)).floor()
    )
    ordered_k = range(first_k, last_k + 1)
    if piece.tail:
        ordered_k = range(last_k, first_k - 1, -1)

    min_guard_per_raw = (
        float(certificate.derivative_min_guard) * (RECIPROCAL // Q) / Q
    )
    base_radius = max(16, math.ceil((degree + 2) / min_guard_per_raw) + 16)
    intervals: list[tuple[int, int, int]] = []
    t_guess = -1.0
    total_thresholds = last_k - first_k + 1
    print(
        f"discrete-brackets {piece.name}: candidate threshold span={total_thresholds}",
        flush=True,
    )

    for sequence, k in enumerate(ordered_k):
        threshold_twice = (2 * k - 1) * eval_guard
        low_twice = threshold_twice - degree
        high_twice = threshold_twice + degree

        # Skip threshold bands disjoint from the exact polynomial endpoint range.
        left_cmp_low = (2 * lo_exact > low_twice) - (2 * lo_exact < low_twice)
        left_cmp_high = (2 * lo_exact > high_twice) - (2 * lo_exact < high_twice)
        right_cmp_low = (2 * hi_exact > low_twice) - (2 * hi_exact < low_twice)
        right_cmp_high = (2 * hi_exact > high_twice) - (2 * hi_exact < high_twice)
        if not piece.tail:
            if right_cmp_low < 0 or left_cmp_high > 0:
                continue
        else:
            if left_cmp_low < 0 or right_cmp_high > 0:
                continue

        target_raw = k - 0.5
        # Floating point only proposes a center. Exact inequalities below make
        # the resulting bracket independent of floating-point accuracy.
        for _ in range(4):
            value, derivative = float_poly(piece, t_guess)
            if derivative == 0:
                break
            t_guess -= (value - target_raw) / derivative
            t_guess = min(1.0, max(-1.0, t_guess))
        center = round(piece.mid_raw + HW_RAW * t_guess)
        radius = base_radius

        while True:
            left = max(lo, center - radius)
            right = min(hi, center + radius)
            if not piece.tail:
                left_ok = left == lo or compare_poly_to_twice_guarded(
                    piece, left, low_twice
                ) < 0
                right_ok = right == hi or compare_poly_to_twice_guarded(
                    piece, right, high_twice
                ) > 0
            else:
                left_ok = left == lo or compare_poly_to_twice_guarded(
                    piece, left, high_twice
                ) > 0
                right_ok = right == hi or compare_poly_to_twice_guarded(
                    piece, right, low_twice
                ) < 0
            if left_ok and right_ok:
                break
            radius *= 2
            assert radius <= hi - lo + 1

        intervals.append((piece_index, left, right))
        if sequence and sequence % 1_000_000 == 0:
            print(
                f"  {piece.name}: {sequence}/{total_thresholds} thresholds",
                flush=True,
            )

    # Adjacent/overlapping bands can be merged without weakening exhaustion.
    merged: list[tuple[int, int, int]] = []
    for record in sorted(intervals, key=lambda item: item[1]):
        if merged and record[1] <= merged[-1][2] + 1:
            old = merged[-1]
            merged[-1] = (piece_index, old[1], max(old[2], record[2]))
        else:
            merged.append(record)
    checked_pairs = sum(right - left for _, left, right in merged)
    print(
        f"  {piece.name}: records={len(merged)} exact-pairs={checked_pairs}",
        flush=True,
    )
    return merged, checked_pairs


def rust_array(values: tuple[int, ...]) -> str:
    padded = list(values) + [0] * (9 - len(values))
    return "[" + ",".join(str(value) for value in padded) + "]"


def render_rust_verifier(pieces: list[Piece]) -> str:
    coefficients = ",\n".join(rust_array(piece.coefficients) for piece in pieces)
    lengths = ",".join(str(len(piece.coefficients)) for piece in pieces)
    mids = ",".join(str(piece.mid_raw) for piece in pieces)
    tails = ",".join("true" if piece.tail else "false" for piece in pieces)
    return f"""
use std::env;
use std::fs;

const Q: i128 = 1i128 << 44;
const GUARD: i128 = {GUARD};
const TAIL_EXTRA_Q: u32 = {TAIL_EXTRA_Q};
const RECIP: i128 = {RECIPROCAL};
const COEFFS: [[i64; 9]; {len(pieces)}] = [
{coefficients}
];
const LENGTHS: [usize; {len(pieces)}] = [{lengths}];
const MIDS: [i128; {len(pieces)}] = [{mids}];
const TAIL: [bool; {len(pieces)}] = [{tails}];

fn rn(value: i128, divisor: i128) -> i128 {{
    if value >= 0 {{ (value + divisor / 2) / divisor }}
    else {{ -((-value + divisor / 2) / divisor) }}
}}

fn value(piece: usize, x: i128) -> i128 {{
    let t = rn((x - MIDS[piece]) * RECIP, Q);
    let n = LENGTHS[piece];
    let extra_q = if TAIL[piece] {{ TAIL_EXTRA_Q }} else {{ 0 }};
    let mut result = COEFFS[piece][n - 1] << extra_q;
    for index in (0..n - 1).rev() {{
        result = rn(result as i128 * t, Q) as i64 + (COEFFS[piece][index] << extra_q);
    }}
    rn(result as i128, GUARD << extra_q)
}}

fn get_i64(bytes: &[u8]) -> i64 {{
    i64::from_le_bytes(bytes.try_into().unwrap())
}}

fn main() {{
    let path = env::args().nth(1).expect("candidate file");
    let bytes = fs::read(path).unwrap();
    assert_eq!(bytes.len() % 24, 0);
    let mut pairs: u128 = 0;
    let mut records: u64 = 0;
    for record in bytes.chunks_exact(24) {{
        let piece = record[0] as usize;
        let left = get_i64(&record[8..16]) as i128;
        let right = get_i64(&record[16..24]) as i128;
        let mut previous = value(piece, left);
        let mut x = left + 1;
        while x <= right {{
            let current = value(piece, x);
            if (!TAIL[piece] && current < previous) || (TAIL[piece] && current > previous) {{
                panic!("monotonicity failure piece={{}} x={{}} prev={{}} current={{}}", piece, x, previous, current);
            }}
            previous = current;
            x += 1;
            pairs += 1;
        }}
        records += 1;
    }}
    println!("records={{records}} pairs={{pairs}}");
}}
"""


def run_ambiguous_cell_verifier(
    pieces: list[Piece], certificates: list[PieceCertificate]
) -> tuple[int, int, str]:
    all_intervals: list[tuple[int, int, int]] = []
    expected_pairs = 0
    for index, certificate in enumerate(certificates):
        if certificate.monotone_by_margin:
            continue
        intervals, pair_count = candidate_intervals(index, certificate)
        all_intervals.extend(intervals)
        expected_pairs += pair_count

    with tempfile.TemporaryDirectory(prefix="solmath-cdf-proof-") as directory:
        directory_path = Path(directory)
        records_path = directory_path / "ambiguous.bin"
        verifier_path = directory_path / "verify.rs"
        binary_path = directory_path / "verify"
        with records_path.open("wb") as output:
            for piece, left, right in all_intervals:
                output.write(struct.pack("<B7xqq", piece, left, right))
        verifier_path.write_text(render_rust_verifier(pieces))
        subprocess.run(
            [
                "rustc",
                "--edition=2021",
                "-O",
                "-C",
                "overflow-checks=yes",
                str(verifier_path),
                "-o",
                str(binary_path),
            ],
            check=True,
        )
        completed = subprocess.run(
            [str(binary_path), str(records_path)],
            text=True,
            capture_output=True,
        )
        if completed.returncode != 0:
            raise AssertionError(
                "generated exact integer verifier failed:\n" + completed.stderr
            )
    output = completed.stdout.strip()
    match = re.fullmatch(r"records=(\d+) pairs=(\d+)", output)
    assert match
    records = int(match.group(1))
    pairs = int(match.group(2))
    assert records == len(all_intervals)
    assert pairs == expected_pairs
    return records, pairs, output


def positive_kernel_value(pieces: list[Piece], raw_x: int) -> int:
    if raw_x == 0:
        return SCALE // 2
    if raw_x <= 5 * SCALE:
        index = min((raw_x - 1) // (SCALE // 2), 9)
        return rounded_piece_value(pieces[index], raw_x)
    if raw_x <= 7 * SCALE:
        index = 10 + min((raw_x - 5 * SCALE - 1) // (SCALE // 2), 3)
        tail = rounded_piece_value(pieces[index], raw_x)
        return SCALE - max(0, min(SCALE, tail))
    if raw_x <= TAIL_CUTOFF:
        return SCALE - 1
    return SCALE


def certify_seams_and_cutoff(
    pieces: list[Piece],
) -> tuple[list[tuple[int, int, int]], arb, arb, arb, arb]:
    # Include the exact x=0 special return and its first positive raw neighbor,
    # then every dispatch seam and the half-raw tail transition.
    seams = [0] + [i * SCALE // 2 for i in range(1, 15)] + [TAIL_CUTOFF]
    seam_values: list[tuple[int, int, int]] = []
    for seam in seams:
        at = positive_kernel_value(pieces, seam)
        after = positive_kernel_value(pieces, seam + 1)
        assert after >= at, f"nonmonotone seam at raw {seam}"
        seam_values.append((seam, at, after))

    sqrt_two = arb(2).sqrt()
    tail_at_seven = arb(SCALE) * (arb(7) / sqrt_two).erfc() / 2
    cutoff_x = arb(TAIL_CUTOFF) / SCALE
    cutoff_tail = arb(SCALE) * (cutoff_x / sqrt_two).erfc() / 2
    after_x = arb(TAIL_CUTOFF + 1) / SCALE
    after_tail = arb(SCALE) * (after_x / sqrt_two).erfc() / 2
    tail_at_eight = arb(SCALE) * (arb(8) / sqrt_two).erfc() / 2
    assert tail_at_seven < arb("1.5")
    assert cutoff_tail > arb("0.5")
    assert after_tail < arb("0.5")
    assert tail_at_eight < arb("0.001")
    assert 7 * SCALE < TAIL_CUTOFF < 8 * SCALE
    return seam_values, tail_at_seven, cutoff_tail, after_tail, tail_at_eight


def fmt(value: arb, digits: int = 12) -> str:
    return value.str(digits, radius=False)


def sha256_text(text: str) -> str:
    return hashlib.sha256(text.encode()).hexdigest()


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--skip-discrete",
        action="store_true",
        help="run continuous accuracy proof only (not a release certificate)",
    )
    args = parser.parse_args()
    assert flint.__version__ == "0.8.0", flint.__version__
    ctx.prec = ARB_BITS
    started = time.monotonic()
    pieces, coefficient_text, kernel_text, modeled_kernel = parse_sources()
    certify_integer_safety(pieces)

    reciprocal_residual = abs(RECIPROCAL * HW_RAW - Q * Q)
    assert 2 * reciprocal_residual <= HW_RAW
    assert map_t(0, HW_RAW) == -Q
    assert map_t(2 * HW_RAW, HW_RAW) == Q

    certificates: list[PieceCertificate] = []
    print("piece approximation map integer total derivative-margin")
    for piece in pieces:
        certificate = certify_piece(piece)
        certificates.append(certificate)
        assert certificate.total_error < arb("2.5")
        print(
            piece.name,
            fmt(certificate.approximation),
            fmt(certificate.map_error),
            fmt(certificate.integer_error),
            fmt(certificate.total_error),
            "analytic" if certificate.monotone_by_margin else "cell-audit",
        )

    seam_values, tail_at_seven, cutoff_tail, after_tail, tail_at_eight = (
        certify_seams_and_cutoff(pieces)
    )
    if args.skip_discrete:
        records = pairs = 0
        discrete = "SKIPPED"
    else:
        records, pairs, discrete = run_ambiguous_cell_verifier(pieces, certificates)

    maximum = arb(0)
    worst = ""
    for certificate in certificates:
        if not certificate.total_error < maximum:
            maximum = maximum.max(certificate.total_error)
            worst = certificate.piece.name
    assert maximum < arb("2.5")
    elapsed = time.monotonic() - started
    print(f"worst={worst} bound={fmt(maximum, 16)}")
    print(
        "tail_at_7=" + fmt(tail_at_seven, 16),
        "cutoff_tail=" + fmt(cutoff_tail, 16),
        "after_cutoff_tail=" + fmt(after_tail, 16),
        "tail_at_8=" + fmt(tail_at_eight, 16),
    )
    print(f"seams={len(seam_values)} discrete={discrete}")
    print(f"coefficient_sha256={sha256_text(coefficient_text)}")
    print(f"modeled_kernel_sha256={sha256_text(modeled_kernel)}")
    print(f"kernel_sha256={sha256_text(kernel_text)}")
    print(f"python_flint={flint.__version__} arb_bits={ARB_BITS} elapsed={elapsed:.1f}s")
    if not args.skip_discrete:
        print(
            "CERTIFIED: real error < 2.5 raw units; nearest-integer reference "
            "error <= 2 ULP; exact symmetry and all-i128 discrete monotonicity"
        )


if __name__ == "__main__":
    main()
