#!/usr/bin/env python3
"""
lipschitz_certificate.py — Rigorous numerical certificate for SolMath CDF error bounds.

For each polynomial piece in norm_cdf_poly (SCALE = 1e12) and norm_cdf_poly_hp
(SCALE_HP = 1e15), this script:

1. Evaluates the real-valued polynomial (exact mpmath arithmetic) vs mpmath.ncdf
   at 100,000+ grid points per piece.
2. Records grid_max = max |p(u(x)) - Phi(x) * SCALE| over the grid.
3. Computes a rigorous Lipschitz constant L for the error function e(x) by:
   a. Sampling |e'(x)| on the same grid.
   b. Adding an analytical correction from a proved upper bound on |e''(x)|.
4. Reports the certificate bound = grid_max + L * h / 2 (where h is grid spacing).

If certificate_bound <= claimed_bound for all pieces: PASS.

This bounds the real-valued polynomial approximation error. The integer Horner
evaluation error is separately bounded in PROOFS.md at < 18 ULP (standard) and
< 10 ULP (HP).

Usage: python3 lipschitz_certificate.py
Requires: mpmath (pip install mpmath)
"""

import mpmath
import time
import sys

mpmath.mp.dps = 60  # 60 decimal digits

# ============================================================
# Constants
# ============================================================

SCALE = mpmath.mpf(10) ** 12
SCALE_HP = mpmath.mpf(10) ** 15

CLAIMED_BOUND_STD = 5   # ULP at SCALE (v2 rounding Horner, 6 pieces + CF tail)
CLAIMED_BOUND_HP = 5    # ULP at SCALE_HP (v2 rounding Horner, 6 pieces + CF tail)

# ============================================================
# Polynomial piece definitions — from constants.rs (v2)
# ============================================================

# Standard-scale v2 pieces (degree 11 each, 6 pieces on [0, 5], tail for (5, 8])
# Rounding Horner (fp_mul_i_round), boundary-constrained, coordinate-descent optimized.
STD_PIECES = [
    {
        "name": "I0", "lo": 0.0, "hi": 0.5, "mid": 0.25, "hw": 0.25,
        "coeffs": [
            598_706_325_685, 96_667_029_200, -3_020_844_663, -944_013_957,
            46_217_351, 8_272_417, -471_323, -57_350, 3_612, 332, -25, -5,
        ],
    },
    {
        "name": "I1", "lo": 0.5, "hi": 1.5, "mid": 1.0, "hw": 0.5,
        "coeffs": [
            841_344_746_069, 120_985_362_259, -30_246_340_579, -9,
            1_260_264_311, -126_026_344, -31_506_998, 6_001_018,
            469_437, -171_562, -2_238, 3_366,
        ],
    },
    {
        "name": "I2", "lo": 1.5, "hi": 2.25, "mid": 1.875, "hw": 0.375,
        "coeffs": [
            969_603_638_235, 25_794_853_434, -9_068_503_163, 1_520_863_556,
            -54_796_234, -24_375_030, 3_883_819, 18_049,
            -60_032, 4_327, 413, -29,
        ],
    },
    {
        "name": "I3", "lo": 2.25, "hi": 3.0, "mid": 2.625, "hw": 0.375,
        "coeffs": [
            995_667_551_638, 4_771_568_097, -2_348_506_182, 658_769_951,
            -107_075_994, 7_184_753, 828_766, -237_082,
            16_850, 1_684, -422, -92,
        ],
    },
    {
        "name": "I4", "lo": 3.0, "hi": 4.0, "mid": 3.5, "hw": 0.5,
        "coeffs": [
            999_767_370_921, 436_341_347, -381_798_695, 204_535_012,
            -73_575_654, 18_081_420, -2_821_622, 167_292,
            39_482, -11_790, 931, 114,
        ],
    },
    {
        "name": "I5", "lo": 4.0, "hi": 5.0, "mid": 4.5, "hw": 0.5,
        "coeffs": [
            999_996_602_327, 7_991_870, -8_990_869, 6_410_140,
            -3_230_973, 1_213_662, -347_720, 75_398,
            -11_582, 1_318, -130, -93,
        ],
    },
]

# HP-scale v2 pieces (deg 13/15/17, 6 polynomial pieces on [0, 5], tail for (5, 8])
# Rounding Horner (fp_mul_hp_i), boundary-constrained, coordinate-descent optimized.
HP_PIECES = [
    {
        "name": "HP_I0", "lo": 0.0, "hi": 0.5, "mid": 0.25, "hw": 0.25,
        "coeffs": [
            598706325682924, 96667029200713, -3020844662519, -944013957034,
            46217349936, 8272413929, -471315375, -57342341,
            3603759, 323049, -21723, -1210, 4, -100,
        ],
    },
    {
        "name": "HP_I1", "lo": 0.5, "hi": 1.5, "mid": 1.0, "hw": 0.5,
        "coeffs": [
            841344746068544, 120985362259572, -30246340565023, -33,
            1260264191835, -126026418616, -31506612696, 6001256127,
            468867241, -171906606, -1847116, 3593915, -100208, -55795,
        ],
    },
    {
        "name": "HP_I2A", "lo": 1.5, "hi": 2.25, "mid": 1.875, "hw": 0.375,
        "coeffs": [
            969603638234739, 25794853435008, -9068503160759, 1520863550899,
            -54796253049, -24374992132, 3883873087, 17940493,
            -60092044, 4454506, 433779, -86366, 3277, 5989, -982, -1491,
        ],
    },
    {
        "name": "HP_I2B", "lo": 2.25, "hi": 3.0, "mid": 2.625, "hw": 0.375,
        "coeffs": [
            995667551636987, 4771568098811, -2348506173644, 658769960927,
            -107076056464, 7184669478, 828940223, -236847082,
            16656613, 1411389, -351615, 22524, 5391, -5946, -829, 1606,
        ],
    },
    {
        "name": "HP_I3A", "lo": 3.0, "hi": 4.0, "mid": 3.5, "hw": 0.5,
        "coeffs": [
            999767370920965, 436341347522, -381798679096, 204535006650,
            -73575786827, 18081462775, -2821236035, 167169387,
            39009455, -11645489, 1152149, 49566, -6912, 5795,
            -13949, -1724, 3518, 416,
        ],
    },
    {
        "name": "HP_I3B", "lo": 4.0, "hi": 5.0, "mid": 4.5, "hw": 0.5,
        "coeffs": [
            999996602326874, 7991870552, -8990854373, 6410146188,
            -3231088291, 1213608926, -347400516, 75547720,
            -11941734, 1139090, 6868, -3204, 4445, -25983,
            39, 15785, -15, -3944,
        ],
    },
    # HP tail ([5.0, 8.0]) uses PDF * Mills ratio — not a direct CDF polynomial; skipped.
]


# ============================================================
# Polynomial evaluation
# ============================================================

def eval_poly(coeffs, u):
    """Evaluate c[0] + c[1]*u + ... + c[N]*u^N via Horner in mpmath."""
    r = mpmath.mpf(coeffs[-1])
    for k in range(len(coeffs) - 2, -1, -1):
        r = r * u + mpmath.mpf(coeffs[k])
    return r


def eval_poly_deriv(coeffs, u):
    """Evaluate d/du [c[0] + c[1]*u + ... + c[N]*u^N]."""
    dcoeffs = [k * coeffs[k] for k in range(1, len(coeffs))]
    if not dcoeffs:
        return mpmath.mpf(0)
    return eval_poly(dcoeffs, u)


def eval_poly_deriv2(coeffs, u):
    """Evaluate d^2/du^2 [c[0] + c[1]*u + ... + c[N]*u^N]."""
    d2coeffs = [k * (k - 1) * coeffs[k] for k in range(2, len(coeffs))]
    if not d2coeffs:
        return mpmath.mpf(0)
    return eval_poly(d2coeffs, u)


# ============================================================
# Second-derivative bound (for Lipschitz rigour)
# ============================================================

def compute_M2(piece, scale, n_m2=10_000):
    """
    Rigorous upper bound on |e''(x)| for x in [lo, hi].

    e''(x) = p''(u(x)) / hw^2 + x * phi(x) * scale

    We sample |e''(x)| on a grid and add an analytical bound on |e'''(x)| * h/2
    to correct for grid aliasing. Since e''' is one more derivative of a
    bounded polynomial + Gaussian, the correction is small on a 10K grid.
    """
    lo = mpmath.mpf(piece["lo"])
    hi = mpmath.mpf(piece["hi"])
    mid = mpmath.mpf(piece["mid"])
    hw = mpmath.mpf(piece["hw"])
    coeffs = piece["coeffs"]

    h_m2 = (hi - lo) / n_m2
    M2_grid = mpmath.mpf(0)

    for i in range(n_m2 + 1):
        x = lo + i * h_m2
        u = (x - mid) / hw
        # e''(x) = p''(u) / hw^2 - phi'(x) * scale
        #        = p''(u) / hw^2 + x * phi(x) * scale
        p2_val = eval_poly_deriv2(coeffs, u) / (hw ** 2)
        phi_d_val = x * mpmath.npdf(x) * scale  # -phi'(x) * scale = x*phi(x)*scale
        e2 = abs(p2_val + phi_d_val)
        if e2 > M2_grid:
            M2_grid = e2

    # Analytical bound on |e'''(x)| for grid correction.
    # e'''(x) = p'''(u)/hw^3 + (1 - x^2)*phi(x)*scale
    # Use triangle inequality: |p'''(u)| <= sum k(k-1)(k-2)|c_k| for |u|<=1
    p3_bound = sum(
        k * (k - 1) * (k - 2) * abs(coeffs[k]) for k in range(3, len(coeffs))
    )
    p3_term = mpmath.mpf(p3_bound) / (hw ** 3)
    # |(1-x^2)*phi(x)| <= phi(0) = 1/sqrt(2*pi) for all x (crude but safe)
    gauss_term = mpmath.mpf(1) / mpmath.sqrt(2 * mpmath.pi) * scale
    M3 = p3_term + gauss_term

    return M2_grid + M3 * h_m2 / 2


# ============================================================
# Per-piece certification
# ============================================================

def certify_piece(piece, scale, n_grid=100_000):
    """
    Certify the real-valued polynomial approximation error for one piece.

    Returns a dict:
      grid_max  — max |p(u(x)) - Phi(x)*SCALE| on the grid
      L         — rigorous Lipschitz constant for the error function
      h         — grid spacing
      Lh2       — L * h / 2  (the certificate correction)
      cert      — grid_max + Lh2  (the certified bound)
    """
    lo = mpmath.mpf(piece["lo"])
    hi = mpmath.mpf(piece["hi"])
    mid = mpmath.mpf(piece["mid"])
    hw = mpmath.mpf(piece["hw"])
    coeffs = piece["coeffs"]
    degree = len(coeffs) - 1

    h = (hi - lo) / n_grid

    grid_max_err = mpmath.mpf(0)
    L_grid = mpmath.mpf(0)

    t0 = time.time()
    report_interval = n_grid // 10

    for i in range(n_grid + 1):
        x = lo + i * h
        u = (x - mid) / hw

        # Real-valued polynomial (no integer rounding)
        p_val = eval_poly(coeffs, u)

        # True CDF
        true_val = mpmath.ncdf(x) * scale

        err = abs(p_val - true_val)
        if err > grid_max_err:
            grid_max_err = err

        # Error derivative: e'(x) = p'(u)/hw - phi(x)*scale
        p_d = eval_poly_deriv(coeffs, u) / hw
        phi_val = mpmath.npdf(x) * scale
        err_d = abs(p_d - phi_val)
        if err_d > L_grid:
            L_grid = err_d

        if report_interval > 0 and i > 0 and i % report_interval == 0:
            pct = 100 * i // n_grid
            sys.stderr.write(f"  {piece['name']}: {pct}% ({i}/{n_grid})\n")
            sys.stderr.flush()

    elapsed = time.time() - t0

    # Second-derivative bound for Lipschitz correction
    M2 = compute_M2(piece, scale)

    # Rigorous L: sampled max + correction for grid aliasing of e'
    L_corr = M2 * h / 2
    L = L_grid + L_corr

    # Certificate
    Lh2 = L * h / 2
    cert = grid_max_err + Lh2

    return {
        "name": piece["name"],
        "degree": degree,
        "interval": f"[{piece['lo']}, {piece['hi']}]",
        "grid_max": float(grid_max_err),
        "L_grid": float(L_grid),
        "M2": float(M2),
        "L_corr": float(L_corr),
        "L": float(L),
        "h": float(h),
        "Lh2": float(Lh2),
        "cert": float(cert),
        "n_grid": n_grid,
        "elapsed": elapsed,
    }


# ============================================================
# Main
# ============================================================

def print_header(title, claimed):
    print(f"\n{'=' * 72}")
    print(f"  {title}")
    print(f"  Claimed bound: {claimed} ULP")
    print(f"{'=' * 72}\n")


def print_table(results, claimed):
    # Header
    fmt = "{:<10s} {:>4s} {:>14s}  {:>12s} {:>12s} {:>12s}  {:>10s} {:>6s}"
    print(fmt.format(
        "Piece", "Deg", "Interval",
        "grid_max", "L*h/2", "cert_bound",
        "Status", "Time",
    ))
    print("-" * 92)

    all_pass = True
    fmt_row = "{:<10s} {:>4d} {:>14s}  {:>12.4f} {:>12.6f} {:>12.4f}  {:>10s} {:>5.1f}s"

    for r in results:
        status = "PASS" if r["cert"] <= claimed else "FAIL"
        if status == "FAIL":
            all_pass = False
        print(fmt_row.format(
            r["name"], r["degree"], r["interval"],
            r["grid_max"], r["Lh2"], r["cert"],
            status, r["elapsed"],
        ))

    print()
    return all_pass


def main():
    print("lipschitz_certificate.py — SolMath CDF polynomial error bounds")
    print(f"mpmath precision: {mpmath.mp.dps} decimal digits")
    print()

    n_grid_std = 100_000
    n_grid_hp = 100_000

    # ------------------------------------------------------------------
    # Standard scale: norm_cdf_poly
    # ------------------------------------------------------------------
    print_header("norm_cdf_poly v2 (SCALE = 1e12, degree 11, 6 pieces on [0,5])", CLAIMED_BOUND_STD)

    std_results = []
    for piece in STD_PIECES:
        sys.stderr.write(f"Certifying {piece['name']}...\n")
        sys.stderr.flush()
        r = certify_piece(piece, SCALE, n_grid=n_grid_std)
        std_results.append(r)

    std_pass = print_table(std_results, CLAIMED_BOUND_STD)

    # Overall max
    std_max_cert = max(r["cert"] for r in std_results)
    std_max_grid = max(r["grid_max"] for r in std_results)
    print(f"Overall grid max:        {std_max_grid:.4f} ULP")
    print(f"Overall certificate max: {std_max_cert:.4f} ULP")
    print(f"Claimed bound:           {CLAIMED_BOUND_STD} ULP")
    print(f"Verdict:                 {'PASS' if std_pass else 'FAIL'}")

    # ------------------------------------------------------------------
    # HP scale: norm_cdf_poly_hp
    # ------------------------------------------------------------------
    print_header(
        "norm_cdf_poly_hp v2 (SCALE_HP = 1e15, degrees 13-17, 6 polynomial pieces on [0,5])",
        CLAIMED_BOUND_HP,
    )
    print("NOTE: HP piece I4 ([5.0, 8.0]) uses PDF * Mills ratio, not a direct")
    print("CDF polynomial. It is not certified here. The tail CDF is > 0.9999997,")
    print("so absolute error there is bounded by the PDF magnitude (~10^-6 * SCALE_HP).\n")

    hp_results = []
    for piece in HP_PIECES:
        sys.stderr.write(f"Certifying {piece['name']}...\n")
        sys.stderr.flush()
        r = certify_piece(piece, SCALE_HP, n_grid=n_grid_hp)
        hp_results.append(r)

    hp_pass = print_table(hp_results, CLAIMED_BOUND_HP)

    hp_max_cert = max(r["cert"] for r in hp_results)
    hp_max_grid = max(r["grid_max"] for r in hp_results)
    print(f"Overall grid max:        {hp_max_grid:.4f} ULP")
    print(f"Overall certificate max: {hp_max_cert:.4f} ULP")
    print(f"Claimed bound:           {CLAIMED_BOUND_HP} ULP")
    print(f"Verdict:                 {'PASS' if hp_pass else 'FAIL'}")

    # ------------------------------------------------------------------
    # Summary
    # ------------------------------------------------------------------
    print(f"\n{'=' * 72}")
    print("  SUMMARY")
    print(f"{'=' * 72}\n")

    both_pass = std_pass and hp_pass

    if both_pass:
        print("ALL PIECES PASS.\n")
        print("Certificate statement for PROOFS.md:")
        print("-" * 60)
        print(f"""
The real-valued polynomial approximation error (the polynomial evaluated
in exact arithmetic vs the true CDF) has been rigorously certified via
dense-grid evaluation with Lipschitz interpolation correction:

  norm_cdf_poly:    max |p(u(x)) - Phi(x)*SCALE|    = {std_max_cert:.2f} ULP
  norm_cdf_poly_hp: max |p(u(x)) - Phi(x)*SCALE_HP| = {hp_max_cert:.2f} ULP

These bounds cover the minimax approximation error and coefficient
quantization error. The Horner evaluation rounding error (< 18 ULP
standard, < 10 ULP HP) is additive and proved separately.

Grid: {n_grid_std:,} points/piece, mpmath {mpmath.mp.dps}-digit precision.
Certificate method: grid max + L*h/2 with analytical M2 Lipschitz correction.
""")
    else:
        print("SOME PIECES FAILED — see details above.")
        if not std_pass:
            for r in std_results:
                if r["cert"] > CLAIMED_BOUND_STD:
                    excess = r["cert"] - CLAIMED_BOUND_STD
                    print(f"  {r['name']}: cert={r['cert']:.4f}, exceeds by {excess:.4f} ULP")
        if not hp_pass:
            for r in hp_results:
                if r["cert"] > CLAIMED_BOUND_HP:
                    excess = r["cert"] - CLAIMED_BOUND_HP
                    print(f"  {r['name']}: cert={r['cert']:.4f}, exceeds by {excess:.4f} ULP")

    return 0 if both_pass else 1


if __name__ == "__main__":
    sys.exit(main())
