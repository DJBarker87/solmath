#!/usr/bin/env python3
"""Source-bound numerical certificate for the N32/Q63 ``exp_fixed_i`` kernel.

The certificate combines exact integer reasoning with Arb interval arithmetic:

* the checked-in coefficient source must be byte-identical to the output of
  ``generate_exp_coeffs.py``;
* a 120-decimal-digit Remez exchange is checked for equioscillation, while Arb
  rigorously bounds the *quantized* polynomial over its whole interval;
* the split-i64 Q63 reduction, all integer widths, all reduction seams, and the
  tiny/domain branches are checked against the modeled Rust recurrence; and
* a derivative margin proves monotonicity inside cells, while every actual
  cell seam is checked at each raw input in a +/-8 window.

Reproduce with:

    python3 -m pip install --target /tmp/solmath-proof-deps python-flint==0.8.0
    PYTHONPATH=/tmp/solmath-proof-deps python3 scripts/certify_exp_fixed.py
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import math
import re
from pathlib import Path
from typing import Any

import mpmath as mp

try:
    from flint import arb, ctx
except ImportError as exc:  # pragma: no cover - dependency failure
    raise SystemExit(
        "python-flint==0.8.0 is required; put it on PYTHONPATH"
    ) from exc


ROOT = Path(__file__).resolve().parents[1]
COEFF_SOURCE = ROOT / "src" / "exp_coeffs.rs"
KERNEL_SOURCE = ROOT / "src" / "transcendental.rs"
CONSTANT_SOURCE = ROOT / "src" / "constants.rs"
EXPM1_SOURCE = ROOT / "src" / "expm1_lut.rs"
GENERATOR_SOURCE = ROOT / "scripts" / "generate_exp_coeffs.py"
CERTIFICATE_SCRIPT = Path(__file__).resolve()
JSON_OUTPUT = ROOT / ".superstack" / "exp-proof-certificate-2026-07-12.json"
MARKDOWN_OUTPUT = ROOT / ".superstack" / "exp-proof-certificate-2026-07-12.md"
PRODUCTION_VECTORS = ROOT / "benchmark" / "prod_exp_vectors.json"
ADVERSARIAL_VECTORS = ROOT / "benchmark" / "adv_exp_vectors.json"

SCALE = 10**12
LIMIT = 40 * SCALE
Q63 = 1 << 63
Q64 = 1 << 64
Q96 = 1 << 96
COEFF_GUARD = SCALE << 22
PHASE_Q = 1 << 62
ARB_BITS = 256
ERROR_GRID = 100_000
SEAM_RADIUS_RAW = 8
DATE = "2026-07-12"

MODELED_FUNCTIONS = (
    "round_shift_signed",
    "round_shift_i64",
    "mul_q63_i64",
    "exp_fixed_i",
)

# Filled after the modeled functions and generated coefficient source were
# frozen.  These fail closed if the Rust recurrence or constants drift.
EXPECTED_COEFFICIENT_SHA256 = "a5adbf73f726a1d347c03acf7d732aff2d48fec93499ba308bcc5d9af13c97ac"
EXPECTED_MODELED_KERNEL_SHA256 = "04b8f5eb1e543e2cff99dc936d19ba179cefbe4c74ca1bb2af5ba94ad6a481ed"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def extract_rust_function(source: str, name: str) -> str:
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
    raise AssertionError(f"unterminated Rust function {name}")


def parse_scalar(source: str, name: str) -> int:
    match = re.search(
        rf"\bconst {name}\s*:[^=]+?=\s*(-?[0-9][0-9_]*)(?:[iu][0-9]+)?\s*;",
        source,
    )
    assert match, f"cannot parse {name}"
    return int(match.group(1).replace("_", ""))


def parse_array(source: str, name: str) -> tuple[int, ...]:
    match = re.search(
        rf"\bconst {name}\s*:\s*\[[^\]]+\]\s*=\s*\[(.*?)\];", source, re.S
    )
    assert match, f"cannot parse {name}"
    return tuple(
        int(token.replace("_", ""))
        for token in re.findall(r"-?[0-9][0-9_]*", match.group(1))
    )


def import_generator():
    spec = importlib.util.spec_from_file_location("generate_exp_coeffs", GENERATOR_SOURCE)
    assert spec and spec.loader
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def source_binding() -> tuple[dict[str, Any], Any, dict[str, int], tuple[int, ...], tuple[int, ...]]:
    coefficient_text = COEFF_SOURCE.read_text()
    kernel_text = KERNEL_SOURCE.read_text()
    constant_text = CONSTANT_SOURCE.read_text()
    expm1_text = EXPM1_SOURCE.read_text()
    generator = import_generator()
    assert generator.render() == coefficient_text, "exp_coeffs.rs is not generator-reproducible"

    modeled_text = "\n\n".join(
        extract_rust_function(kernel_text, name) for name in MODELED_FUNCTIONS
    )
    modeled_sha = hashlib.sha256(modeled_text.encode()).hexdigest()
    coefficient_sha = hashlib.sha256(coefficient_text.encode()).hexdigest()
    if EXPECTED_COEFFICIENT_SHA256 != "TO_FILL":
        assert coefficient_sha == EXPECTED_COEFFICIENT_SHA256
    if EXPECTED_MODELED_KERNEL_SHA256 != "TO_FILL":
        assert modeled_sha == EXPECTED_MODELED_KERNEL_SHA256

    constants = {
        "SCALE_I": parse_scalar(constant_text, "SCALE_I"),
        "LN2_I": parse_scalar(constant_text, "LN2_I"),
        "EXPM1_INV_LN2_Q56": parse_scalar(expm1_text, "EXPM1_INV_LN2_Q56"),
        "EXP_RAW_TO_Q63_HI": parse_scalar(coefficient_text, "EXP_RAW_TO_Q63_HI"),
        "EXP_RAW_TO_Q63_FRAC_Q28": parse_scalar(
            coefficient_text, "EXP_RAW_TO_Q63_FRAC_Q28"
        ),
        "EXP_LN2_RESIDUAL_Q96": parse_scalar(
            coefficient_text, "EXP_LN2_RESIDUAL_Q96"
        ),
        "EXP_STEP_Q63": parse_scalar(coefficient_text, "EXP_STEP_Q63"),
        "EXP_POLY_GUARD": parse_scalar(coefficient_text, "EXP_POLY_GUARD"),
        "EXP_PHASE_BITS": parse_scalar(coefficient_text, "EXP_PHASE_BITS"),
        # Retained only to reproduce the pre-optimization rational kernel in
        # the before/after corpus comparison; they are not used by the proof.
        "LN2_LO": parse_scalar(constant_text, "LN2_LO"),
        "EXP_REMEZ_P1": parse_scalar(constant_text, "EXP_REMEZ_P1"),
        "EXP_REMEZ_P2": parse_scalar(constant_text, "EXP_REMEZ_P2"),
        "EXP_REMEZ_P3": parse_scalar(constant_text, "EXP_REMEZ_P3"),
        "EXP_REMEZ_P4": parse_scalar(constant_text, "EXP_REMEZ_P4"),
        "EXP_REMEZ_P5": parse_scalar(constant_text, "EXP_REMEZ_P5"),
    }
    coefficients = parse_array(coefficient_text, "EXP_REMEZ_Q22")
    phases = parse_array(coefficient_text, "EXP2_PHASE_Q62")

    assert constants == {
        "SCALE_I": SCALE,
        "LN2_I": 693_147_180_560,
        "EXPM1_INV_LN2_Q56": 103_957,
        "EXP_RAW_TO_Q63_HI": 18_446_744,
        "EXP_RAW_TO_Q63_FRAC_Q28": 19_786_257,
        "EXP_LN2_RESIDUAL_Q96": -4_333_034_379_533_306,
        "EXP_STEP_Q63": 199_786_072_581_291_495,
        "EXP_POLY_GUARD": 22,
        "EXP_PHASE_BITS": 5,
        "LN2_LO": -54_690_582_768,
        "EXP_REMEZ_P1": 166_666_666_667,
        "EXP_REMEZ_P2": -2_777_777_778,
        "EXP_REMEZ_P3": 66_137_563,
        "EXP_REMEZ_P4": -1_653_390,
        "EXP_REMEZ_P5": 41_381,
    }
    assert len(coefficients) == 6
    assert len(phases) == 32
    assert all(0 < value <= (1 << 63) - 1 for value in coefficients)
    assert all(0 < value <= (1 << 63) - 1 for value in phases)
    assert "(-1_000_000..1_000_000).contains(&x)" in modeled_text
    assert "round_shift_i64(scaled_residual, 1)" in modeled_text
    assert "(56 - EXP_PHASE_BITS)" in modeled_text
    assert "EXP_POLY_GUARD + 62" in modeled_text

    binding = {
        "coefficient_source_sha256": coefficient_sha,
        "modeled_kernel_sha256": modeled_sha,
        "modeled_functions": list(MODELED_FUNCTIONS),
        "full_source_sha256": {
            "src/exp_coeffs.rs": sha256(COEFF_SOURCE),
            "src/transcendental.rs": sha256(KERNEL_SOURCE),
            "src/constants.rs": sha256(CONSTANT_SOURCE),
            "src/expm1_lut.rs": sha256(EXPM1_SOURCE),
            "scripts/generate_exp_coeffs.py": sha256(GENERATOR_SOURCE),
            "scripts/certify_exp_fixed.py": sha256(CERTIFICATE_SCRIPT),
        },
        "generator_byte_identical": True,
    }
    return binding, generator, constants, coefficients, phases


def round_shift(value: int, shift: int) -> int:
    assert shift > 0
    half = 1 << (shift - 1)
    if value >= 0:
        return (value + half) >> shift
    return -((-value + half) >> shift)


def reduce_model(x: int, constants: dict[str, int]) -> dict[str, int]:
    x64 = x
    octave = round_shift(x64 * constants["EXPM1_INV_LN2_Q56"], 56)
    raw = x64 - octave * constants["LN2_I"]
    scaled = raw * constants["EXP_RAW_TO_Q63_HI"] + round_shift(
        raw * constants["EXP_RAW_TO_Q63_FRAC_Q28"], 28
    )
    octave_q63 = round_shift(scaled, 1) - round_shift(
        octave * constants["EXP_LN2_RESIDUAL_Q96"], 33
    )
    subcell = round_shift(
        raw * constants["EXPM1_INV_LN2_Q56"],
        56 - constants["EXP_PHASE_BITS"],
    )
    r_q63 = octave_q63 - subcell * constants["EXP_STEP_Q63"]
    half_step = (constants["EXP_STEP_Q63"] + 1) // 2
    if r_q63 > half_step:
        subcell += 1
        r_q63 -= constants["EXP_STEP_Q63"]
    elif r_q63 < -half_step:
        subcell -= 1
        r_q63 += constants["EXP_STEP_Q63"]
    phases = 1 << constants["EXP_PHASE_BITS"]
    cell = octave * phases + subcell
    return {
        "octave_estimate": octave,
        "raw_residual": raw,
        "scaled_residual": scaled,
        "subcell": subcell,
        "r_q63": r_q63,
        "cell": cell,
        "octave": cell >> constants["EXP_PHASE_BITS"],
        "phase": cell & (phases - 1),
    }


def exp_model(
    x: int,
    constants: dict[str, int],
    coefficients: tuple[int, ...],
    phases: tuple[int, ...],
) -> tuple[str, int | str]:
    if x <= -LIMIT:
        return "ok", 0
    if x >= LIMIT:
        return "error", "Overflow"
    if -1_000_000 <= x < 1_000_000:
        return "ok", SCALE + x

    reduction = reduce_model(x, constants)
    r_q63 = reduction["r_q63"]
    polynomial = coefficients[0]
    for coefficient in coefficients[1:]:
        polynomial = round_shift(polynomial * r_q63, 63) + coefficient

    phase = reduction["phase"]
    if phase == 0:
        guarded = polynomial
        guard = constants["EXP_POLY_GUARD"]
    else:
        guarded = polynomial * phases[phase]
        guard = constants["EXP_POLY_GUARD"] + 62
    shift = guard - reduction["octave"]
    if shift >= 128:
        return "ok", 0
    if shift > 0:
        return "ok", round_shift(guarded, shift)
    if shift == 0:
        return "ok", guarded
    result = guarded << -shift
    assert result <= (1 << 127) - 1
    return "ok", result


def trunc_div(numerator: int, denominator: int) -> int:
    """Rust-style signed integer division (toward zero)."""
    assert denominator != 0
    negative = (numerator < 0) != (denominator < 0)
    quotient = abs(numerator) // abs(denominator)
    return -quotient if negative else quotient


def legacy_exp_model(x: int, constants: dict[str, int]) -> tuple[str, int | str]:
    """Exact simulator for the replaced SCALE/Remez rational recurrence."""
    if x <= -LIMIT:
        return "ok", 0
    if x >= LIMIT:
        return "error", "Overflow"
    if x == 0:
        return "ok", SCALE

    octave = trunc_div(x, constants["LN2_I"])
    raw_correction = octave * constants["LN2_LO"]
    if raw_correction >= 0:
        ln2_correction = trunc_div(raw_correction + SCALE // 2, SCALE)
    else:
        ln2_correction = trunc_div(raw_correction - SCALE // 2, SCALE)
    residual = x - octave * constants["LN2_I"] - ln2_correction
    half_ln2 = constants["LN2_I"] // 2
    if residual > half_ln2:
        octave += 1
        residual -= constants["LN2_I"]
    elif residual < -half_ln2:
        octave -= 1
        residual += constants["LN2_I"]

    def mul_round(left: int, right: int) -> int:
        product = left * right
        quotient = trunc_div(product, SCALE)
        remainder = product - quotient * SCALE
        if abs(remainder) < SCALE // 2:
            return quotient
        return quotient + (1 if product >= 0 else -1)

    xx = mul_round(residual, residual)
    polynomial = mul_round(xx, constants["EXP_REMEZ_P5"]) + constants["EXP_REMEZ_P4"]
    polynomial = mul_round(xx, polynomial) + constants["EXP_REMEZ_P3"]
    polynomial = mul_round(xx, polynomial) + constants["EXP_REMEZ_P2"]
    polynomial = mul_round(xx, polynomial) + constants["EXP_REMEZ_P1"]
    correction = residual - mul_round(polynomial, xx)
    rc = mul_round(residual, correction)
    rational = trunc_div(rc * SCALE, 2 * SCALE - correction)
    reduced = SCALE + residual + rational
    if octave >= 0:
        return "ok", reduced << octave
    return "ok", reduced >> -octave


def arb_text(value: arb, digits: int = 40) -> str:
    return value.str(digits, radius=False)


def upper_abs(value: arb) -> arb:
    return abs(value).abs_upper()


def certify_polynomial(generator, coefficients: tuple[int, ...]) -> dict[str, Any]:
    ctx.prec = ARB_BITS
    mp.mp.dps = generator.WORK_DPS
    real_coefficients, alternation_error, extrema = generator.derive_remez()
    quantized_ascending = tuple(reversed(coefficients))
    regenerated = tuple(
        generator.round_away(value * COEFF_GUARD) for value in real_coefficients
    )
    assert regenerated == quantized_ascending

    alternation_values = [
        sum(value * x**power for power, value in enumerate(real_coefficients))
        - mp.exp(x)
        for x in extrema
    ]
    assert all(left * right < 0 for left, right in zip(alternation_values, alternation_values[1:]))
    alternation_spread = max(abs(value) for value in alternation_values) - min(
        abs(value) for value in alternation_values
    )
    assert alternation_spread < mp.mpf("1e-100")

    radius = arb(2).log() / 64
    degree = len(quantized_ascending) - 1
    derivative_error_bound = arb(0)
    radius_power = arb(1)
    for power in range(degree):
        delta = (
            arb((power + 1) * quantized_ascending[power + 1]) / COEFF_GUARD
            - arb(1) / math.factorial(power)
        )
        derivative_error_bound += upper_abs(delta) * radius_power
        radius_power *= radius
    derivative_error_bound += radius**degree * radius.exp().abs_upper() / math.factorial(degree)

    maximum_node_error = arb(0)
    maximum_index = 0
    for index in range(ERROR_GRID + 1):
        x = -radius + 2 * radius * index / ERROR_GRID
        polynomial = arb(quantized_ascending[-1]) / COEFF_GUARD
        for coefficient in reversed(quantized_ascending[:-1]):
            polynomial = polynomial * x + arb(coefficient) / COEFF_GUARD
        error = upper_abs(polynomial - x.exp())
        if error > maximum_node_error:
            maximum_node_error = error
            maximum_index = index
    continuous_bound = maximum_node_error + derivative_error_bound * radius / ERROR_GRID
    assert continuous_bound < arb("7.05e-17")
    return {
        "degree": degree,
        "interval": "[-ln(2)/64, ln(2)/64]",
        "remez_alternation_error": mp.nstr(alternation_error, 80),
        "remez_extrema": [mp.nstr(value, 40) for value in extrema],
        "alternation_spread": mp.nstr(alternation_spread, 8),
        "quantized_continuous_bound": arb_text(continuous_bound, 55),
        "maximum_grid_node_bound": arb_text(maximum_node_error, 55),
        "maximum_grid_node_index": maximum_index,
        "derivative_error_bound": arb_text(derivative_error_bound, 55),
        "arb_bits": ARB_BITS,
        "mvt_grid_intervals": ERROR_GRID,
        "coefficients_descending_q22": list(coefficients),
        "_continuous": continuous_bound,
        "_derivative_error": derivative_error_bound,
    }


def ceil_div(numerator: int, denominator: int) -> int:
    return -((-numerator) // denominator)


def exact_raw_residual_bound(constants: dict[str, int]) -> tuple[int, dict[str, int]]:
    reciprocal = constants["EXPM1_INV_LN2_Q56"]
    denominator = 1 << 56
    half = 1 << 55
    maximum = 0
    witness: dict[str, int] = {}
    for octave in range(0, 100):
        lo = max(0, ceil_div(octave * denominator - half, reciprocal))
        hi = min(LIMIT - 1, ((octave + 1) * denominator - half - 1) // reciprocal)
        if lo > hi:
            continue
        for x in (lo, hi):
            residual = x - octave * constants["LN2_I"]
            if abs(residual) > maximum:
                maximum = abs(residual)
                witness = {"x": x, "octave": octave, "raw_residual": residual}
    assert maximum == 346_624_802_184
    return maximum, witness


def reduction_and_width_certificate(
    constants: dict[str, int], coefficients: tuple[int, ...], phases: tuple[int, ...]
) -> dict[str, Any]:
    ctx.prec = ARB_BITS
    ln2 = arb(2).log()
    phase_count = 1 << constants["EXP_PHASE_BITS"]
    reciprocal = arb(constants["EXPM1_INV_LN2_Q56"]) / (1 << 56)
    exact_reciprocal = arb(1) / (SCALE * ln2)
    candidate_cell_error = (
        arb(1) / 2
        + phase_count * LIMIT * upper_abs(reciprocal - exact_reciprocal)
        + phase_count
        * 58
        * upper_abs(arb(1) - constants["LN2_I"] * reciprocal)
    )
    assert candidate_cell_error < arb("0.51")

    max_raw, raw_witness = exact_raw_residual_bound(constants)
    exact_raw_multiplier = arb(2) ** 64 / SCALE
    split_raw_multiplier = arb(constants["EXP_RAW_TO_Q63_HI"]) + arb(
        constants["EXP_RAW_TO_Q63_FRAC_Q28"]
    ) / (1 << 28)
    split_q64_error = arb(1) / 2 + max_raw * upper_abs(
        split_raw_multiplier - exact_raw_multiplier
    )
    q63_conversion_error = split_q64_error / 2 + arb(1) / 2

    exact_ln2_residual_q96 = (ln2 - arb(constants["LN2_I"]) / SCALE) * Q96
    ln2_residual_error = arb(1) / 2 + 58 * upper_abs(
        arb(constants["EXP_LN2_RESIDUAL_Q96"]) - exact_ln2_residual_q96
    ) / (1 << 33)
    step_error_per_subcell = upper_abs(
        arb(constants["EXP_STEP_Q63"]) - ln2 / phase_count * Q63
    )
    # The raw proposal is in [-16,16]; allowing one correction gives 17.
    subcell_bound = 17
    step_error = subcell_bound * step_error_per_subcell
    reduction_error_q63 = q63_conversion_error + ln2_residual_error + step_error
    assert reduction_error_q63 < arb(72)
    reduction_error_real = reduction_error_q63 / Q63
    ambiguous_raw_radius = reduction_error_real * SCALE
    assert ambiguous_raw_radius < arb("0.00001")

    i64_max = (1 << 63) - 1
    i128_max = (1 << 127) - 1
    raw_hi_product = max_raw * constants["EXP_RAW_TO_Q63_HI"]
    raw_frac_product = max_raw * constants["EXP_RAW_TO_Q63_FRAC_Q28"]
    scaled_residual_bound = raw_hi_product + round_shift(raw_frac_product, 28)
    octave_residual_product = 58 * abs(constants["EXP_LN2_RESIDUAL_Q96"])
    subcell_proposal_product = max_raw * constants["EXPM1_INV_LN2_Q56"]
    r_bound = (constants["EXP_STEP_Q63"] + 1) // 2 + 1

    assert max(raw_hi_product, raw_frac_product, scaled_residual_bound) <= i64_max
    assert octave_residual_product <= i64_max
    assert subcell_proposal_product <= i64_max
    assert r_bound <= i64_max

    accumulator_bound = abs(coefficients[0])
    maximum_horner_product = 0
    accumulator_bounds = [accumulator_bound]
    for coefficient in coefficients[1:]:
        product_bound = accumulator_bound * r_bound
        maximum_horner_product = max(maximum_horner_product, product_bound)
        accumulator_bound = (product_bound + (1 << 62)) // Q63 + abs(coefficient)
        accumulator_bounds.append(accumulator_bound)
    assert accumulator_bound <= i64_max
    assert maximum_horner_product <= i128_max
    maximum_phase = max(phases)
    maximum_phase_product = accumulator_bound * maximum_phase
    assert maximum_phase <= i64_max
    assert maximum_phase_product <= i128_max

    max_input = LIMIT - 1
    status, maximum_output = exp_model(max_input, constants, coefficients, phases)
    assert status == "ok" and isinstance(maximum_output, int)
    assert maximum_output < i128_max

    return {
        "candidate_cell_error_bound": arb_text(candidate_cell_error, 55),
        "candidate_within_one_correction": True,
        "maximum_exact_raw_octave_residual": max_raw,
        "raw_residual_witness": raw_witness,
        "split_q64_error_bound": arb_text(split_q64_error, 55),
        "q63_conversion_error_bound": arb_text(q63_conversion_error, 55),
        "ln2_residual_error_bound_q63": arb_text(ln2_residual_error, 55),
        "step_error_bound_q63": arb_text(step_error, 55),
        "total_reduction_error_bound_q63": arb_text(reduction_error_q63, 55),
        "total_reduction_error_bound_real": arb_text(reduction_error_real, 55),
        "ambiguous_raw_radius": arb_text(ambiguous_raw_radius, 30),
        "subcell_bound_including_correction": subcell_bound,
        "widths": {
            "i64_max": i64_max,
            "i128_max": i128_max,
            "raw_times_hi": raw_hi_product,
            "raw_times_frac_q28": raw_frac_product,
            "scaled_residual": scaled_residual_bound,
            "octave_times_ln2_residual_q96": octave_residual_product,
            "raw_times_inv_ln2_q56": subcell_proposal_product,
            "maximum_abs_r_q63": r_bound,
            "horner_accumulator_bounds": accumulator_bounds,
            "maximum_horner_product": maximum_horner_product,
            "maximum_phase_factor_q62": maximum_phase,
            "maximum_phase_product": maximum_phase_product,
            "maximum_output_at_40_minus_one": maximum_output,
            "horner_product_fraction_i128": maximum_horner_product / i128_max,
            "phase_product_fraction_i128": maximum_phase_product / i128_max,
        },
        "_reduction_real": reduction_error_real,
    }


def exact_cell(x: int) -> int:
    return int(mp.floor(mp.mpf(x) * 32 / (SCALE * mp.log(2)) + mp.mpf("0.5")))


def seam_and_monotonicity_certificate(
    constants: dict[str, int],
    coefficients: tuple[int, ...],
    phases: tuple[int, ...],
    reduction: dict[str, Any],
    polynomial: dict[str, Any],
) -> dict[str, Any]:
    mp.mp.dps = 100
    seams = []
    checked_inputs = 0
    monotone_pairs = 0
    maximum_jump = 0
    minimum_boundary_distance = None
    for boundary_cell in range(-2000, 2000):
        boundary = (mp.mpf(boundary_cell) + mp.mpf("0.5")) * mp.log(2) / 32 * SCALE
        center = int(mp.floor(boundary + mp.mpf("0.5")))
        if center - SEAM_RADIUS_RAW <= -LIMIT or center + SEAM_RADIUS_RAW >= LIMIT:
            continue
        boundary_ball = (arb(boundary_cell) + arb(1) / 2) * arb(2).log() / 32 * SCALE
        boundary_distance = abs(boundary_ball - center).lower()
        if minimum_boundary_distance is None or boundary_distance < minimum_boundary_distance:
            minimum_boundary_distance = boundary_distance
        seams.append(boundary_cell)
        xs = list(range(center - SEAM_RADIUS_RAW, center + SEAM_RADIUS_RAW + 1))
        outputs: list[int] = []
        for x in xs:
            reduction_result = reduce_model(x, constants)
            if arb(x) < boundary_ball:
                interval_exact_cell = boundary_cell
            elif arb(x) > boundary_ball:
                interval_exact_cell = boundary_cell + 1
            else:  # pragma: no cover - 256-bit Arb separates every checked raw x
                raise AssertionError("seam boundary overlaps an integer raw input")
            assert interval_exact_cell == exact_cell(x)
            assert reduction_result["cell"] == interval_exact_cell
            status, output = exp_model(x, constants, coefficients, phases)
            assert status == "ok" and isinstance(output, int)
            outputs.append(output)
        checked_inputs += len(xs)
        for left, right in zip(outputs, outputs[1:]):
            assert right >= left
            maximum_jump = max(maximum_jump, right - left)
            monotone_pairs += 1
    assert len(seams) == 3_694
    assert minimum_boundary_distance is not None
    # Outside each +/-8 window the nearest possible raw input is separated
    # from its seam by more than seven units, versus <1e-5 raw reduction
    # uncertainty. Inside the windows the Arb comparisons above are exact.
    assert arb(reduction["ambiguous_raw_radius"]) < arb(7)

    # The cheap full-octave proposal changes at points near half-ln2 cell
    # centers, not at N32 cell boundaries.  Its two algebraic decompositions
    # represent the same final cell, but rounded split conversion can differ
    # by a few Q63 units, so check every such internal transition explicitly.
    proposal_transitions: set[int] = set()
    proposal_denominator = 1 << 56
    proposal_half = 1 << 55
    proposal_reciprocal = constants["EXPM1_INV_LN2_Q56"]
    for new_octave in range(1, 59):
        positive = ceil_div(
            new_octave * proposal_denominator - proposal_half,
            proposal_reciprocal,
        )
        proposal_transitions.add(positive)
        proposal_transitions.add(-positive)
    assert len(proposal_transitions) == 116
    proposal_transition_inputs = 0
    proposal_transition_pairs = 0
    for center in sorted(proposal_transitions):
        xs = list(range(center - SEAM_RADIUS_RAW, center + SEAM_RADIUS_RAW + 1))
        outputs = []
        for x in xs:
            assert -LIMIT < x < LIMIT
            assert reduce_model(x, constants)["cell"] == exact_cell(x)
            status, output = exp_model(x, constants, coefficients, phases)
            assert status == "ok" and isinstance(output, int)
            outputs.append(output)
        proposal_transition_inputs += len(xs)
        for left, right in zip(outputs, outputs[1:]):
            assert right >= left
            proposal_transition_pairs += 1

    # Inside one cell the exact quantized polynomial rises by millions of
    # guarded units per raw input.  This dominates both Horner evaluations'
    # complete rounding envelopes, so the integer polynomial cannot reverse.
    ctx.prec = ARB_BITS
    radius = arb(2).log() / 64 + reduction["_reduction_real"]
    ascending = tuple(reversed(coefficients))
    derivative_lower = (
        arb(ascending[1]) / COEFF_GUARD
        - 2 * arb(ascending[2]) / COEFF_GUARD * radius
        - 4 * arb(ascending[4]) / COEFF_GUARD * radius**3
    )
    assert derivative_lower > arb("0.98")
    minimum_q63_step = constants["EXP_RAW_TO_Q63_HI"] // 2
    horner_rounding_guarded = arb(1) / 2 * sum(
        radius**power for power in range(len(coefficients) - 1)
    )
    guarded_step_margin = (
        derivative_lower * minimum_q63_step / Q63 * COEFF_GUARD
        - 2 * horner_rounding_guarded
    )
    assert guarded_step_margin > arb(1_000_000)

    # Tiny direct-return seams and the complete domain contract.
    tiny_xs = list(range(-1_000_008, -999_991)) + list(range(999_992, 1_000_009))
    tiny_outputs = []
    for x in tiny_xs:
        status, output = exp_model(x, constants, coefficients, phases)
        assert status == "ok" and isinstance(output, int)
        expected = int(mp.floor(mp.exp(mp.mpf(x) / SCALE) * SCALE + mp.mpf("0.5")))
        assert abs(output - expected) <= 1
        tiny_outputs.append(output)
    assert all(right >= left for left, right in zip(tiny_outputs, tiny_outputs[1:]))

    tiny_radius = arb(999_999) / SCALE
    tiny_remainder = SCALE * tiny_radius**2 * tiny_radius.exp() / 2
    assert tiny_remainder < arb("0.5")

    domain_cases = {}
    for x in (-LIMIT - 1, -LIMIT, -LIMIT + 1, LIMIT - 1, LIMIT, LIMIT + 1):
        status, value = exp_model(x, constants, coefficients, phases)
        domain_cases[str(x)] = {"status": status, "value": value}
    assert domain_cases[str(-LIMIT - 1)] == {"status": "ok", "value": 0}
    assert domain_cases[str(-LIMIT)] == {"status": "ok", "value": 0}
    assert domain_cases[str(LIMIT)] == {"status": "error", "value": "Overflow"}
    assert domain_cases[str(LIMIT + 1)] == {"status": "error", "value": "Overflow"}

    return {
        "actual_cell_seams": len(seams),
        "raw_radius_per_seam": SEAM_RADIUS_RAW,
        "seam_input_checks": checked_inputs,
        "seam_monotone_pairs": monotone_pairs,
        "cell_mismatches": 0,
        "seam_reversals": 0,
        "maximum_observed_adjacent_jump": maximum_jump,
        "minimum_seam_to_integer_distance_raw": str(minimum_boundary_distance),
        "outside_window_distance_lower_raw": 7,
        "octave_proposal_internal_seams": len(proposal_transitions),
        "octave_proposal_input_checks": proposal_transition_inputs,
        "octave_proposal_monotone_pairs": proposal_transition_pairs,
        "octave_proposal_reversals": 0,
        "interior_derivative_lower": arb_text(derivative_lower, 45),
        "minimum_q63_increment_per_raw_input": minimum_q63_step,
        "horner_rounding_envelope_guarded": arb_text(horner_rounding_guarded, 45),
        "interior_guarded_step_margin": arb_text(guarded_step_margin, 45),
        "interior_monotonicity_proved": True,
        "tiny_taylor_remainder_bound_raw": arb_text(tiny_remainder, 45),
        "tiny_seam_checks": len(tiny_xs),
        "domain_cases": domain_cases,
    }


def final_error_certificate(
    constants: dict[str, int],
    phases: tuple[int, ...],
    polynomial: dict[str, Any],
    reduction: dict[str, Any],
) -> dict[str, Any]:
    ctx.prec = ARB_BITS
    radius = arb(2).log() / 64
    extended_radius = radius + reduction["_reduction_real"]
    continuous = polynomial["_continuous"] + polynomial["_derivative_error"] * reduction[
        "_reduction_real"
    ]
    input_exp_error = extended_radius.exp() * reduction["_reduction_real"]
    degree = len(polynomial["coefficients_descending_q22"]) - 1
    horner_rounding = arb(1) / (2 * COEFF_GUARD) * sum(
        extended_radius**power for power in range(degree)
    )
    phase_max = arb(2) ** (arb(31) / 32)
    phase_quantization = arb(1) / (2 * PHASE_Q)
    polynomial_magnitude = extended_radius.exp() + continuous + input_exp_error + horner_rounding
    local_error = phase_max * (continuous + input_exp_error + horner_rounding) + (
        polynomial_magnitude * phase_quantization
    )
    relative_error = local_error / (-radius).exp()

    full_raw = local_error * SCALE * (1 << 57) + arb(1) / 2
    financial_raw = local_error * SCALE * (1 << 28) + arb(1) / 2
    assert relative_error < arb("2e-16")
    assert financial_raw < arb(50_000)
    assert full_raw < arb("3e13")

    return {
        "extended_residual_radius": arb_text(extended_radius, 55),
        "continuous_bound_on_extended_radius": arb_text(continuous, 55),
        "input_reduction_exp_contribution": arb_text(input_exp_error, 55),
        "integer_horner_rounding_contribution": arb_text(horner_rounding, 55),
        "phase_quantization_contribution": arb_text(
            polynomial_magnitude * phase_quantization, 55
        ),
        "combined_local_absolute_bound": arb_text(local_error, 55),
        "combined_relative_bound": arb_text(relative_error, 55),
        "financial_domain": "|x| < 20*SCALE, maximum reconstruction octave 28",
        "financial_raw_ulp_bound": arb_text(financial_raw, 45),
        "full_domain": "-40*SCALE < x < 40*SCALE, maximum reconstruction octave 57",
        "full_raw_ulp_bound": arb_text(full_raw, 45),
    }


def percentile(sorted_values: list[int], fraction: float) -> int:
    return sorted_values[int(fraction * (len(sorted_values) - 1))]


def retained_corpus_stats(
    path: Path,
    constants: dict[str, int],
    coefficients: tuple[int, ...],
    phases: tuple[int, ...],
    legacy: bool = False,
) -> dict[str, Any]:
    if not path.exists():
        return {"present": False}
    payload = json.loads(path.read_text())
    errors: list[int] = []
    worst: dict[str, Any] | None = None
    for vector in payload["vectors"]:
        x = int(vector["x"])
        status, output = (
            legacy_exp_model(x, constants)
            if legacy
            else exp_model(x, constants, coefficients, phases)
        )
        assert status == "ok" and isinstance(output, int)
        expected = int(vector["expected"])
        error = abs(output - expected)
        errors.append(error)
        if worst is None or error > worst["error"]:
            worst = {
                "error": error,
                "x": x,
                "category": vector.get("category"),
                "output": output,
                "expected": expected,
            }
    errors.sort()
    count = len(errors)
    return {
        "present": True,
        "path": str(path.relative_to(ROOT)),
        "sha256": sha256(path),
        "count": count,
        "max": errors[-1],
        "p99": percentile(errors, 0.99),
        "p95": percentile(errors, 0.95),
        "median": errors[count // 2],
        "exact": sum(error == 0 for error in errors),
        "worst": worst,
        "meta": payload.get("meta", {}),
        "kernel": "legacy SCALE/Remez rational simulator" if legacy else "frozen N32/Q63",
    }


def strip_private(data: Any) -> Any:
    if isinstance(data, dict):
        return {key: strip_private(value) for key, value in data.items() if not key.startswith("_")}
    if isinstance(data, list):
        return [strip_private(value) for value in data]
    return data


def render_markdown(certificate: dict[str, Any]) -> str:
    source = certificate["source_binding"]
    poly = certificate["polynomial"]
    reduction = certificate["reduction"]
    monotone = certificate["monotonicity"]
    errors = certificate["error_bounds"]
    prod = certificate["retained_corpora"]["production"]
    adv = certificate["retained_corpora"]["adversarial"]
    legacy_prod = certificate["legacy_comparison"]["production"]
    legacy_adv = certificate["legacy_comparison"]["adversarial"]
    widths = reduction["widths"]
    return f"""# `exp_fixed_i` proof certificate ({DATE})

## Result

The exact N32/Q63 Rust recurrence bound by this certificate is division-free,
uses a degree-5 quantized Remez polynomial, and is monotone on every valid raw
input. Arb proves the quantized local polynomial error is at most
`{poly['quantized_continuous_bound']}`. Including split range reduction,
integer Horner rounding and Q62 phase reconstruction gives:

| Bound | Certified value |
|---|---:|
| relative error over `(-40,40)` | `{errors['combined_relative_bound']}` |
| raw error for `|x| < 20*SCALE` | `{errors['financial_raw_ulp_bound']}` |
| raw error for the full valid domain | `{errors['full_raw_ulp_bound']}` |

## Source binding

- `src/exp_coeffs.rs`: `{source['coefficient_source_sha256']}`
- modeled Rust functions: `{source['modeled_kernel_sha256']}`
- functions: {', '.join(source['modeled_functions'])}
- generator output is byte-identical to the checked-in coefficient source.

## Approximation

- high-precision Remez alternation error: `{poly['remez_alternation_error']}`
- exact quantized coefficient bound: `{poly['quantized_continuous_bound']}`
- Arb precision/grid: {poly['arb_bits']} bits / {poly['mvt_grid_intervals']:,} intervals
- coefficients, descending Q22: `{poly['coefficients_descending_q22']}`

The Remez exchange establishes the origin and equioscillation of the real
coefficients. The stated error theorem does not trust sampled Remez error: it
uses Arb balls on the exact quantized coefficients plus a mean-value enclosure.

## Reduction and monotonicity

- candidate-cell error: `{reduction['candidate_cell_error_bound']}` cell units
- complete Q63 reduction error: `{reduction['total_reduction_error_bound_q63']}` units
- equivalent ambiguous raw radius: `{reduction['ambiguous_raw_radius']}`
- actual cell seams: {monotone['actual_cell_seams']:,}
- raw checks: {monotone['seam_input_checks']:,} (`+/-{monotone['raw_radius_per_seam']}` each)
- cell mismatches / output reversals: {monotone['cell_mismatches']} / {monotone['seam_reversals']}
- internal octave-proposal seams: {monotone['octave_proposal_internal_seams']:,}
  ({monotone['octave_proposal_input_checks']:,} raw checks, {monotone['octave_proposal_reversals']} reversals)
- within-cell guarded step margin: `{monotone['interior_guarded_step_margin']}`
- tiny direct-return remainder: `{monotone['tiny_taylor_remainder_bound_raw']}` raw units

N32 has 3,694 actual reduction seams. The earlier 7,386-seam count applied to
the rejected N64 candidate; this certificate checks every seam of the frozen
N32 source, with 62,798 raw evaluations.

## Integer widths

| Intermediate | Maximum/bound |
|---|---:|
| raw residual | {reduction['maximum_exact_raw_octave_residual']:,} |
| split scaled residual | {widths['scaled_residual']:,} |
| Q63 residual | {widths['maximum_abs_r_q63']:,} |
| Horner product | {widths['maximum_horner_product']:,} |
| phase product | {widths['maximum_phase_product']:,} |
| phase product / `i128::MAX` | {widths['phase_product_fraction_i128']:.6f} |
| output at `40*SCALE-1` | {widths['maximum_output_at_40_minus_one']:,} |

## Retained corpora

| Corpus | N | Max | P99 | P95 | Median | Exact |
|---|---:|---:|---:|---:|---:|---:|
| production | {prod.get('count', 0):,} | {prod.get('max', 0):,} | {prod.get('p99', 0):,} | {prod.get('p95', 0):,} | {prod.get('median', 0):,} | {prod.get('exact', 0):,} |
| structural adversarial | {adv.get('count', 0):,} | {adv.get('max', 0):,} | {adv.get('p99', 0):,} | {adv.get('p95', 0):,} | {adv.get('median', 0):,} | {adv.get('exact', 0):,} |

For an exact before/after comparison, the certificate also simulates the
replaced SCALE/Remez rational recurrence on these same vector files:

| Kernel/corpus | Max | P99 | P95 | Median | Exact |
|---|---:|---:|---:|---:|---:|
| legacy / production | {legacy_prod.get('max', 0):,} | {legacy_prod.get('p99', 0):,} | {legacy_prod.get('p95', 0):,} | {legacy_prod.get('median', 0):,} | {legacy_prod.get('exact', 0):,} |
| N32/Q63 / production | {prod.get('max', 0):,} | {prod.get('p99', 0):,} | {prod.get('p95', 0):,} | {prod.get('median', 0):,} | {prod.get('exact', 0):,} |
| legacy / adversarial | {legacy_adv.get('max', 0):,} | {legacy_adv.get('p99', 0):,} | {legacy_adv.get('p95', 0):,} | {legacy_adv.get('median', 0):,} | {legacy_adv.get('exact', 0):,} |
| N32/Q63 / adversarial | {adv.get('max', 0):,} | {adv.get('p99', 0):,} | {adv.get('p95', 0):,} | {adv.get('median', 0):,} | {adv.get('exact', 0):,} |

These corpora are empirical cross-checks and are not used to establish the
continuous theorem.

## Scope

This is a source-bound proof of the mathematical and exact-integer recurrence.
It does not prove Rust compiler, LLVM/SBF VM, operating-system or hardware
correctness. Deployed CU and linked-size measurements are separate artifacts.
"""


def build_certificate() -> dict[str, Any]:
    binding, generator, constants, coefficients, phases = source_binding()
    polynomial = certify_polynomial(generator, coefficients)
    reduction = reduction_and_width_certificate(constants, coefficients, phases)
    monotonicity = seam_and_monotonicity_certificate(
        constants, coefficients, phases, reduction, polynomial
    )
    errors = final_error_certificate(constants, phases, polynomial, reduction)
    certificate = {
        "schema": "solmath.exp_fixed_i.proof.v1",
        "date": DATE,
        "classification": "Arb interval proof plus exact integer/seam verification",
        "source_binding": binding,
        "constants": constants,
        "polynomial": polynomial,
        "reduction": reduction,
        "monotonicity": monotonicity,
        "error_bounds": errors,
        "retained_corpora": {
            "production": retained_corpus_stats(
                PRODUCTION_VECTORS, constants, coefficients, phases
            ),
            "adversarial": retained_corpus_stats(
                ADVERSARIAL_VECTORS, constants, coefficients, phases
            ),
        },
        "legacy_comparison": {
            "description": (
                "Exact simulator for the replaced SCALE/Remez rational kernel, "
                "evaluated on the identical retained vector files."
            ),
            "production": retained_corpus_stats(
                PRODUCTION_VECTORS, constants, coefficients, phases, legacy=True
            ),
            "adversarial": retained_corpus_stats(
                ADVERSARIAL_VECTORS, constants, coefficients, phases, legacy=True
            ),
        },
        "limitations": (
            "Source-bound numerical/integer proof; compiler, LLVM/SBF VM, OS and "
            "hardware correctness are outside scope."
        ),
    }
    return strip_private(certificate)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true", help="compare with checked-in certificates")
    parser.add_argument(
        "--print-source-digests", action="store_true", help="print hashes even before pinning"
    )
    args = parser.parse_args()
    certificate = build_certificate()
    if args.print_source_digests:
        print(json.dumps(certificate["source_binding"], indent=2, sort_keys=True))
        return
    json_text = json.dumps(certificate, indent=2, sort_keys=True) + "\n"
    markdown_text = render_markdown(certificate)
    if args.check:
        assert JSON_OUTPUT.read_text() == json_text, f"{JSON_OUTPUT} is stale"
        assert MARKDOWN_OUTPUT.read_text() == markdown_text, f"{MARKDOWN_OUTPUT} is stale"
        print("verified exp_fixed_i proof certificates")
    else:
        JSON_OUTPUT.parent.mkdir(parents=True, exist_ok=True)
        JSON_OUTPUT.write_text(json_text)
        MARKDOWN_OUTPUT.write_text(markdown_text)
        print(f"wrote {JSON_OUTPUT}")
        print(f"wrote {MARKDOWN_OUTPUT}")


if __name__ == "__main__":
    main()
