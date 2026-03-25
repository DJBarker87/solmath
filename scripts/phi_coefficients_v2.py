#!/usr/bin/env python3
"""
Refit norm_cdf_poly coefficients for ROUNDING Horner evaluation.

Key changes from phi_coefficients.py:
1. horner_i128 uses rounding (fp_mul_i_round) instead of truncating
2. map_t_i128 uses rounding division
3. Coordinate descent on integer coefficients after initial fit
4. Dense grid with boundary clustering for optimization
"""

import mpmath
import numpy as np
import time

mpmath.mp.dps = 50
SCALE = 10**12
DEGREE = 11

PIECES = [
    {"x_low": 0.0, "x_high": 0.5},
    {"x_low": 0.5, "x_high": 1.5},
    {"x_low": 1.5, "x_high": 3.0},
    {"x_low": 3.0, "x_high": 5.0},
    {"x_low": 5.0, "x_high": 8.0},
]
for p in PIECES:
    p["midpoint"] = (p["x_low"] + p["x_high"]) / 2.0
    p["half_width"] = (p["x_high"] - p["x_low"]) / 2.0

def phi_mpmath(x):
    return mpmath.ncdf(x)

# ── Fixed-point arithmetic (ROUNDING — matches Rust fp_mul_i_round) ──

def fp_mul_i_round(a, b):
    p = a * b
    if p >= 0:
        return (p + SCALE // 2) // SCALE
    else:
        return -( (-p + SCALE // 2) // SCALE )

def horner_i128_round(coeffs, t):
    """Rounding Horner — matches horner_11_round in Rust."""
    result = coeffs[-1]
    for i in range(len(coeffs) - 2, -1, -1):
        result = fp_mul_i_round(result, t) + coeffs[i]
    return result

def map_t_round(ax_scaled, mid_scaled, hw_scaled):
    """Rounding map — matches poly_map_t_round in Rust."""
    num = (ax_scaled - mid_scaled) * SCALE
    if num >= 0:
        return (num + hw_scaled // 2) // hw_scaled
    else:
        return -( (-num + hw_scaled // 2) // hw_scaled )

# ── Grid generation ──

def gen_grid(x_low, x_high, n=50000):
    """Dense grid with boundary clustering."""
    pts = []
    width = x_high - x_low

    # 80% uniform
    n_uni = int(n * 0.8)
    for i in range(n_uni):
        x = x_low + width * i / (n_uni - 1) if n_uni > 1 else x_low
        pts.append(x)

    # 10% near left boundary
    n_left = int(n * 0.1)
    for i in range(n_left):
        x = x_low + width * 0.01 * i / max(n_left - 1, 1)
        pts.append(x)

    # 10% near right boundary
    n_right = n - n_uni - n_left
    for i in range(n_right):
        x = x_high - width * 0.01 * (n_right - 1 - i) / max(n_right - 1, 1)
        pts.append(x)

    # Remove duplicates, sort
    pts = sorted(set(pts))

    # Compute reference values from mpmath
    grid = []
    for x in pts:
        x_scaled = int(round(x * SCALE))
        ref = int(mpmath.nint(phi_mpmath(mpmath.mpf(x)) * SCALE))
        grid.append((x_scaled, ref))
    return grid

# ── Evaluation ──

def evaluate_piece(coeffs_int, grid, mid_scaled, hw_scaled):
    """Evaluate on grid, return list of (actual, expected, error)."""
    results = []
    for x_scaled, ref in grid:
        t = map_t_round(x_scaled, mid_scaled, hw_scaled)
        actual = horner_i128_round(coeffs_int, t)
        actual = max(0, min(SCALE, actual))
        err = abs(actual - ref)
        results.append(err)
    return results

def stats(errors):
    errors_s = sorted(errors)
    n = len(errors_s)
    if n == 0:
        return 0, 0, 0, 0
    mx = errors_s[-1]
    p99 = errors_s[int(n * 0.99)]
    med = errors_s[n // 2]
    exact = sum(1 for e in errors_s if e == 0) / n * 100
    return mx, p99, med, exact

# ── Initial fit (same as phi_coefficients.py) ──

def fit_initial(piece):
    a, b = piece["x_low"], piece["x_high"]
    mid, hw = piece["midpoint"], piece["half_width"]

    n_sample = 500
    nodes = []
    for k in range(n_sample):
        tk = float(mpmath.cos(mpmath.pi * (2*k + 1) / (2*n_sample)))
        xk = (a + b) / 2.0 + (b - a) / 2.0 * tk
        nodes.append(xk)

    t_nodes = np.array([(x - mid) / hw for x in nodes])
    f_nodes = np.array([float(phi_mpmath(mpmath.mpf(x))) for x in nodes])

    V = np.column_stack([t_nodes**j for j in range(DEGREE + 1)])
    coeffs, _, _, _ = np.linalg.lstsq(V, f_nodes, rcond=None)

    # Round to integer
    coeffs_int = [int(mpmath.nint(mpmath.mpf(c) * SCALE)) for c in coeffs]
    return coeffs_int

# ── Coordinate descent ──

def coordinate_descent(coeffs_int, grid, mid_scaled, hw_scaled, label, max_rounds=10):
    """Optimize integer coefficients via coordinate descent."""
    errors = evaluate_piece(coeffs_int, grid, mid_scaled, hw_scaled)
    best_max, best_p99, best_med, best_exact = stats(errors)
    print(f"    Initial: max={best_max} P99={best_p99} med={best_med} exact={best_exact:.1f}%")

    improved = True
    round_num = 0
    while improved and round_num < max_rounds:
        improved = False
        round_num += 1
        for i in range(len(coeffs_int)):
            for delta in [-2, -1, +1, +2]:
                trial = coeffs_int[:]
                trial[i] += delta
                errs = evaluate_piece(trial, grid, mid_scaled, hw_scaled)
                mx, p99, med, exact = stats(errs)
                if mx < best_max or (mx == best_max and p99 < best_p99):
                    coeffs_int = trial
                    best_max = mx
                    best_p99 = p99
                    best_med = med
                    best_exact = exact
                    improved = True
                    print(f"    C{i:>2} {delta:+d} → max={best_max} P99={best_p99}")

    print(f"    Final:   max={best_max} P99={best_p99} med={best_med} exact={best_exact:.1f}% ({round_num} rounds)")
    return coeffs_int, best_max

# ── Main ──

if __name__ == '__main__':
    all_results = []

    for idx, piece in enumerate(PIECES):
        a, b = piece["x_low"], piece["x_high"]
        mid_scaled = int(round(piece["midpoint"] * SCALE))
        hw_scaled = int(round(piece["half_width"] * SCALE))

        print(f"\n{'='*60}")
        print(f"Piece {idx}: [{a}, {b}]")
        print(f"{'='*60}")

        # Generate grid
        t0 = time.time()
        grid = gen_grid(a, b, 50000)
        print(f"  Grid: {len(grid)} points ({time.time()-t0:.1f}s)")

        # Initial fit
        coeffs_int = fit_initial(piece)

        # Evaluate with TRUNCATING (old) for comparison
        old_errors = []
        for x_scaled, ref in grid:
            num = (x_scaled - mid_scaled) * SCALE
            t_trunc = num // hw_scaled  # truncating
            result = coeffs_int[-1]
            for i in range(len(coeffs_int) - 2, -1, -1):
                result = (result * t_trunc) // SCALE + coeffs_int[i]  # truncating
            result = max(0, min(SCALE, result))
            old_errors.append(abs(result - ref))
        old_mx, old_p99, old_med, old_exact = stats(old_errors)
        print(f"  Truncating eval (baseline): max={old_mx} P99={old_p99} med={old_med}")

        # Evaluate with ROUNDING (before optimization)
        errors = evaluate_piece(coeffs_int, grid, mid_scaled, hw_scaled)
        mx, p99, med, exact = stats(errors)
        print(f"  Rounding eval (pre-opt):    max={mx} P99={p99} med={med}")

        # Coordinate descent
        print(f"  Running coordinate descent...")
        t0 = time.time()
        opt_coeffs, opt_max = coordinate_descent(coeffs_int, grid, mid_scaled, hw_scaled, f"Piece {idx}")
        elapsed = time.time() - t0
        print(f"  Descent took {elapsed:.1f}s")

        all_results.append({
            "idx": idx,
            "bounds": f"[{a}, {b}]",
            "old_max": old_mx,
            "new_max": opt_max,
            "coeffs": opt_coeffs,
        })

    # ── Summary ──
    print(f"\n{'='*60}")
    print("SUMMARY")
    print(f"{'='*60}")
    print(f"  {'Piece':8} {'Domain':14} {'Old Max':>8} {'New Max':>8} {'Change':>8}")
    print(f"  {'-'*50}")
    for r in all_results:
        change = r["new_max"] - r["old_max"]
        print(f"  Piece {r['idx']}   {r['bounds']:14} {r['old_max']:>8} {r['new_max']:>8} {change:>+8}")

    # ── Rust constants ──
    print(f"\n{'='*60}")
    print("RUST CONSTANTS (for constants.rs)")
    print(f"{'='*60}")
    for r in all_results:
        print(f"\n// Piece {r['idx']}: {r['bounds']} — max ULP = {r['new_max']} (rounding Horner, coord descent)")
        print(f"pub const POLY_V2_I{r['idx']}: [i128; 12] = [")
        for i, c in enumerate(r["coeffs"]):
            comma = "," if i < 11 else ""
            print(f"    {c}{comma}")
        print("];")
