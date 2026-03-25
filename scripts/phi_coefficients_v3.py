#!/usr/bin/env python3
"""
Refit norm_cdf_poly with boundary continuity + rounding Horner + coord descent.

Strategy: pin LEFT boundary of each piece via C0 adjustment.
Right boundary is owned by the next piece's left pin.
Piece 0's left boundary is Φ(0) = 0.5 exactly.
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

# ── Exact boundary targets ──

BOUNDARIES = [0.0, 0.5, 1.5, 3.0, 5.0, 8.0]
BOUNDARY_TARGETS = {}
for x in BOUNDARIES:
    val = int(mpmath.nint(mpmath.ncdf(mpmath.mpf(x)) * SCALE))
    BOUNDARY_TARGETS[x] = val
    print(f"  Φ({x}) = {val}")
print()

# ── Fixed-point arithmetic (rounding) ──

def fp_mul_round(a, b):
    p = a * b
    if p >= 0:
        return (p + SCALE // 2) // SCALE
    else:
        return -((-p + SCALE // 2) // SCALE)

def horner_round(coeffs, t):
    result = coeffs[-1]
    for i in range(len(coeffs) - 2, -1, -1):
        result = fp_mul_round(result, t) + coeffs[i]
    return result

def map_t_round(ax_scaled, mid_scaled, hw_scaled):
    num = (ax_scaled - mid_scaled) * SCALE
    if num >= 0:
        return (num + hw_scaled // 2) // hw_scaled
    else:
        return -((-num + hw_scaled // 2) // hw_scaled)

# ── Grid generation ──

def gen_grid(x_low, x_high, n=50000):
    pts = set()
    width = x_high - x_low
    n_uni = int(n * 0.8)
    for i in range(n_uni):
        pts.add(x_low + width * i / max(n_uni - 1, 1))
    n_left = int(n * 0.1)
    for i in range(n_left):
        pts.add(x_low + width * 0.01 * i / max(n_left - 1, 1))
    n_right = n - n_uni - n_left
    for i in range(n_right):
        pts.add(x_high - width * 0.01 * (n_right - 1 - i) / max(n_right - 1, 1))
    pts = sorted(pts)
    grid = []
    for x in pts:
        x_scaled = int(round(x * SCALE))
        ref = int(mpmath.nint(mpmath.ncdf(mpmath.mpf(x)) * SCALE))
        grid.append((x_scaled, ref))
    return grid

# ── Evaluation ──

def eval_piece(coeffs, grid, mid_s, hw_s):
    errors = []
    for x_s, ref in grid:
        t = map_t_round(x_s, mid_s, hw_s)
        actual = max(0, min(SCALE, horner_round(coeffs, t)))
        errors.append(abs(actual - ref))
    return errors

def stats(errors):
    s = sorted(errors)
    n = len(s)
    if n == 0: return 0, 0, 0, 0
    return s[-1], s[int(n*0.99)], s[n//2], sum(1 for e in s if e == 0)/n*100

# ── C0 pinning ──

def compute_c0_for_boundary(coeffs, target_val, mid_s, hw_s, boundary_x_scaled):
    """Compute what C0 must be so that horner_round(coeffs, t_boundary) = target_val.
    We evaluate the polynomial WITHOUT C0 contribution, then set C0 = target - rest."""
    t = map_t_round(boundary_x_scaled, mid_s, hw_s)
    # Horner without C0: evaluate c11*t^11 + c10*t^10 + ... + c1*t
    # The Horner chain: start at c[11], multiply by t and add c[10], ..., multiply by t and add c[1]
    # Then the final step would be: multiply by t and add c[0]
    # So: rest = horner_of_c1_to_c11 * t (in fixed point)
    temp = coeffs[-1]
    for i in range(len(coeffs) - 2, 0, -1):  # down to c[1]
        temp = fp_mul_round(temp, t) + coeffs[i]
    # Now temp = the value after adding c[1]. Next step: temp * t + c[0]
    rest = fp_mul_round(temp, t)
    # We want rest + c0 = target_val
    return target_val - rest

# ── Initial fit ──

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
    f_nodes = np.array([float(mpmath.ncdf(mpmath.mpf(x))) for x in nodes])
    V = np.column_stack([t_nodes**j for j in range(DEGREE + 1)])
    coeffs, _, _, _ = np.linalg.lstsq(V, f_nodes, rcond=None)
    return [int(mpmath.nint(mpmath.mpf(c) * SCALE)) for c in coeffs]

# ── Coordinate descent with boundary pin ──

def coord_descent_pinned(coeffs, grid, mid_s, hw_s, left_x_scaled, left_target, max_rounds=10):
    """Optimize C1..C11 via coord descent. After each move, recompute C0 to pin left boundary."""
    # Pin C0 initially
    coeffs[0] = compute_c0_for_boundary(coeffs, left_target, mid_s, hw_s, left_x_scaled)

    errors = eval_piece(coeffs, grid, mid_s, hw_s)
    best_max, best_p99, best_med, best_exact = stats(errors)
    print(f"    After pin: max={best_max} P99={best_p99} med={best_med}")

    improved = True
    round_num = 0
    while improved and round_num < max_rounds:
        improved = False
        round_num += 1
        for i in range(1, len(coeffs)):  # skip C0 — it's pinned
            for delta in [-2, -1, +1, +2]:
                trial = coeffs[:]
                trial[i] += delta
                # Recompute C0 to maintain pin
                trial[0] = compute_c0_for_boundary(trial, left_target, mid_s, hw_s, left_x_scaled)
                errs = eval_piece(trial, grid, mid_s, hw_s)
                mx, p99, med, exact = stats(errs)
                if mx < best_max or (mx == best_max and p99 < best_p99):
                    coeffs = trial
                    best_max = mx
                    best_p99 = p99
                    best_med = med
                    best_exact = exact
                    improved = True
                    print(f"    C{i:>2} {delta:+d} → max={best_max} P99={best_p99}")

    print(f"    Final: max={best_max} P99={best_p99} med={best_med} exact={best_exact:.1f}% ({round_num} rounds)")
    return coeffs, best_max

# ── Main ──

if __name__ == '__main__':
    all_results = []

    for idx, piece in enumerate(PIECES):
        a, b = piece["x_low"], piece["x_high"]
        mid_s = int(round(piece["midpoint"] * SCALE))
        hw_s = int(round(piece["half_width"] * SCALE))
        left_x_s = int(round(a * SCALE))
        left_target = BOUNDARY_TARGETS[a]

        print(f"{'='*60}")
        print(f"Piece {idx}: [{a}, {b}]  left_pin=Φ({a})={left_target}")
        print(f"{'='*60}")

        t0 = time.time()
        grid = gen_grid(a, b, 50000)
        print(f"  Grid: {len(grid)} points ({time.time()-t0:.1f}s)")

        coeffs = fit_initial(piece)

        # Check boundary before pin
        t_left = map_t_round(left_x_s, mid_s, hw_s)
        pre_val = horner_round(coeffs, t_left)
        print(f"  Pre-pin left boundary: {pre_val} (target {left_target}, err={pre_val - left_target})")

        print(f"  Running pinned coordinate descent...")
        t0 = time.time()
        opt_coeffs, opt_max = coord_descent_pinned(coeffs, grid, mid_s, hw_s, left_x_s, left_target)
        print(f"  Took {time.time()-t0:.1f}s")

        # Verify boundary
        t_left = map_t_round(left_x_s, mid_s, hw_s)
        final_left = horner_round(opt_coeffs, t_left)
        t_right = map_t_round(int(round(b * SCALE)), mid_s, hw_s)
        final_right = horner_round(opt_coeffs, t_right)
        right_target = BOUNDARY_TARGETS[b]
        print(f"  Left  boundary: actual={final_left} target={left_target} err={final_left - left_target}")
        print(f"  Right boundary: actual={final_right} target={right_target} err={final_right - right_target}")

        all_results.append({
            "idx": idx, "bounds": f"[{a}, {b}]",
            "max_ulp": opt_max, "coeffs": opt_coeffs,
            "left_err": final_left - left_target,
            "right_err": final_right - right_target,
        })

    # ── Summary ──
    print(f"\n{'='*60}")
    print("SUMMARY")
    print(f"{'='*60}")
    print(f"  {'Piece':8} {'Domain':14} {'Max ULP':>8} {'Left err':>9} {'Right err':>10}")
    print(f"  {'-'*54}")
    for r in all_results:
        print(f"  Piece {r['idx']}   {r['bounds']:14} {r['max_ulp']:>8} {r['left_err']:>+9} {r['right_err']:>+10}")

    # ── Rust constants ──
    print(f"\n{'='*60}")
    print("RUST CONSTANTS")
    print(f"{'='*60}")
    for r in all_results:
        print(f"\n// Piece {r['idx']}: {r['bounds']} — max ULP = {r['max_ulp']}, left_err={r['left_err']}, right_err={r['right_err']}")
        print(f"pub const POLY_V2_I{r['idx']}: [i128; 12] = [")
        for i, c in enumerate(r["coeffs"]):
            comma = "," if i < 11 else ""
            print(f"    {c}{comma}")
        print("];")
