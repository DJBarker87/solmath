#!/usr/bin/env python3
"""
Refit norm_cdf_poly with C1 boundary continuity + rounding Horner + coord descent.

Strategy:
1. Use the ORIGINAL constrained LS fitting in float (normalized t ∈ [-1,1])
2. Round coefficients to integer at SCALE
3. Recompute C0 (and optionally C1) for exact boundary pinning in fixed-point
4. Coordinate descent on C2–C11 with C0/C1 recomputation after each step
"""

import mpmath
from mpmath import mpf, nint
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

BT = {}
for x in [0.0, 0.5, 1.5, 3.0, 5.0, 8.0]:
    BT[x] = int(nint(mpmath.ncdf(mpf(x)) * SCALE))
BT[8.0] = min(BT[8.0], SCALE)
print("Boundary targets:", {k: v for k, v in BT.items()})
print()

# ── Fixed-point rounding ──

def fp_mul_round(a, b):
    p = a * b
    if p >= 0: return (p + SCALE // 2) // SCALE
    else: return -((-p + SCALE // 2) // SCALE)

def horner_round(coeffs, t):
    r = coeffs[-1]
    for i in range(len(coeffs) - 2, -1, -1):
        r = fp_mul_round(r, t) + coeffs[i]
    return r

def map_t_round(ax_s, mid_s, hw_s):
    num = (ax_s - mid_s) * SCALE
    if num >= 0: return (num + hw_s // 2) // hw_s
    else: return -((-num + hw_s // 2) // hw_s)

# ── Constrained LS in float (from original phi_coefficients.py) ──

def fit_c1_constrained_float(piece, left_val, left_deriv_t, n_sample=500):
    """C1-constrained fit: pin value AND derivative at left boundary.
    Returns float64 coefficients for P(τ), τ ∈ [-1, 1]."""
    a, b = piece["x_low"], piece["x_high"]
    mid, hw = piece["midpoint"], piece["half_width"]
    d = DEGREE

    nodes = []
    for k in range(n_sample):
        tk = float(mpmath.cos(mpmath.pi * (2*k + 1) / (2*n_sample)))
        xk = (a + b) / 2.0 + (b - a) / 2.0 * tk
        nodes.append(xk)

    t_nodes = np.array([(x - mid) / hw for x in nodes])
    f_nodes = np.array([float(mpmath.ncdf(mpmath.mpf(x))) for x in nodes])

    # Constraints: P(-1) = left_val, P'(-1) = left_deriv_t
    # Eliminate C0, C1 via substitution (same as original script)
    rhs = f_nodes - left_val - left_deriv_t * (1.0 + t_nodes)
    V = np.zeros((len(t_nodes), d - 1))
    for j_idx, j in enumerate(range(2, d + 1)):
        sign_j = (-1.0)**j
        deriv_sign_j = j * (-1.0)**(j - 1)
        V[:, j_idx] = t_nodes**j - sign_j - deriv_sign_j * (1.0 + t_nodes)

    c_free, _, _, _ = np.linalg.lstsq(V, rhs, rcond=None)
    c1 = left_deriv_t - sum(j * c_free[j-2] * (-1.0)**(j-1) for j in range(2, d+1))
    c0 = left_val + c1 - sum(c_free[j-2] * (-1.0)**j for j in range(2, d+1))

    coeffs = np.zeros(d + 1)
    coeffs[0] = c0
    coeffs[1] = c1
    coeffs[2:] = c_free
    return coeffs

def generate_coefficients_float():
    """Generate all 5 pieces with sequential C1 continuity (same as original)."""
    left_val = float(mpmath.ncdf(mpmath.mpf(0)))  # 0.5
    left_deriv_x = float(mpmath.mpf(1) / mpmath.sqrt(2 * mpmath.pi))

    all_float = []
    for idx, piece in enumerate(PIECES):
        hw = piece["half_width"]
        left_deriv_t = left_deriv_x * hw
        coeffs = fit_c1_constrained_float(piece, left_val, left_deriv_t)
        all_float.append(coeffs)
        # Right boundary for next piece
        left_val = sum(coeffs)
        right_deriv_t = sum(j * coeffs[j] for j in range(1, DEGREE + 1))
        left_deriv_x = right_deriv_t / hw

    return all_float

# ── Recompute C0 for fixed-point boundary pin ──

def recompute_c0_fp(coeffs, mid_s, hw_s, left_x_s, left_target):
    """Set C0 so that horner_round(coeffs, t_left) = left_target."""
    t = map_t_round(left_x_s, mid_s, hw_s)
    # Horner without C0: evaluate c[11]*t^11 + ... + c[1]*t (skipping C0 addition)
    temp = coeffs[-1]
    for i in range(len(coeffs) - 2, 0, -1):
        temp = fp_mul_round(temp, t) + coeffs[i]
    rest = fp_mul_round(temp, t)
    # rest + C0 should equal left_target
    coeffs[0] = left_target - rest
    return coeffs

# ── Grid ──

def gen_grid(x_low, x_high, n=50000):
    pts = set()
    w = x_high - x_low
    for i in range(int(n*0.8)):
        pts.add(x_low + w * i / max(int(n*0.8)-1, 1))
    for i in range(int(n*0.1)):
        pts.add(x_low + w * 0.02 * i / max(int(n*0.1)-1, 1))
    for i in range(int(n*0.1)):
        pts.add(x_high - w * 0.02 * i / max(int(n*0.1)-1, 1))
    grid = []
    for x in sorted(pts):
        x_s = int(round(x * SCALE))
        ref = int(nint(mpmath.ncdf(mpf(x)) * SCALE))
        grid.append((x_s, ref))
    return grid

def eval_piece(coeffs, grid, mid_s, hw_s):
    return [abs(max(0, min(SCALE, horner_round(coeffs, map_t_round(x_s, mid_s, hw_s)))) - ref)
            for x_s, ref in grid]

def stats(errors):
    s = sorted(errors)
    n = len(s)
    if n == 0: return 0, 0, 0, 0
    return s[-1], s[int(n*0.99)], s[n//2], sum(1 for e in s if e==0)/n*100

# ── Coordinate descent on C2–C11, recompute C0 ──

def coord_descent(coeffs, grid, mid_s, hw_s, left_x_s, left_target, max_rounds=10):
    coeffs = recompute_c0_fp(coeffs, mid_s, hw_s, left_x_s, left_target)
    errors = eval_piece(coeffs, grid, mid_s, hw_s)
    best_max, best_p99, best_med, best_exact = stats(errors)
    print(f"    After pin: max={best_max} P99={best_p99} med={best_med}")

    improved = True
    rnd = 0
    while improved and rnd < max_rounds:
        improved = False
        rnd += 1
        for i in range(2, 12):
            for delta in [-2, -1, +1, +2]:
                trial = coeffs[:]
                trial[i] += delta
                trial = recompute_c0_fp(trial, mid_s, hw_s, left_x_s, left_target)
                errs = eval_piece(trial, grid, mid_s, hw_s)
                mx, p99, med, ex = stats(errs)
                if mx < best_max or (mx == best_max and p99 < best_p99):
                    coeffs = trial
                    best_max = mx; best_p99 = p99; best_med = med; best_exact = ex
                    improved = True
                    print(f"    C{i:>2} {delta:+d} → max={best_max} P99={best_p99}")

    print(f"    Final: max={best_max} P99={best_p99} med={best_med} exact={best_exact:.1f}% ({rnd} rds)")
    return coeffs, best_max

# ── Main ──

if __name__ == '__main__':
    print("Generating C1-constrained float coefficients...")
    all_float = generate_coefficients_float()

    all_results = []

    for idx, piece in enumerate(PIECES):
        a, b = piece["x_low"], piece["x_high"]
        mid_s = int(round(piece["midpoint"] * SCALE))
        hw_s = int(round(piece["half_width"] * SCALE))
        left_x_s = int(round(a * SCALE))
        left_target = BT[a]
        right_target = BT[b]

        print(f"\n{'='*60}")
        print(f"Piece {idx}: [{a}, {b}]  left={left_target}  right={right_target}")
        print(f"{'='*60}")

        grid = gen_grid(a, b, 50000)
        print(f"  Grid: {len(grid)} points")

        # Round float coefficients to integer (these represent P(τ) at SCALE)
        coeffs_int = [int(nint(mpf(c) * SCALE)) for c in all_float[idx]]
        print(f"  Float→int coeffs: {coeffs_int[:3]}...")

        # Coordinate descent with left-boundary pin
        t0 = time.time()
        opt, opt_max = coord_descent(coeffs_int, grid, mid_s, hw_s, left_x_s, left_target)
        print(f"  Took {time.time()-t0:.1f}s")

        # Verify boundaries
        tl = map_t_round(left_x_s, mid_s, hw_s)
        tr = map_t_round(mid_s + hw_s, mid_s, hw_s)
        fl = horner_round(opt, tl)
        fr = horner_round(opt, tr)
        print(f"  Left:  {fl} target={left_target} err={fl - left_target}")
        print(f"  Right: {fr} target={right_target} err={fr - right_target}")

        all_results.append({"idx": idx, "bounds": f"[{a},{b}]", "max": opt_max,
                            "coeffs": opt, "le": fl - left_target, "re": fr - right_target})

    print(f"\n{'='*60}")
    print("SUMMARY")
    print(f"{'='*60}")
    print(f"  {'Piece':8} {'Domain':10} {'Max':>6} {'L err':>6} {'R err':>6}")
    for r in all_results:
        print(f"  Piece {r['idx']}   {r['bounds']:10} {r['max']:>6} {r['le']:>+6} {r['re']:>+6}")

    print(f"\n{'='*60}")
    print("RUST CONSTANTS")
    print(f"{'='*60}")
    for r in all_results:
        print(f"\n// Piece {r['idx']}: {r['bounds']} max={r['max']} ULP, boundary=({r['le']},{r['re']})")
        print(f"pub const POLY_V2_I{r['idx']}: [i128; 12] = [")
        for i, c in enumerate(r["coeffs"]):
            comma = "," if i < 11 else ""
            print(f"    {c}{comma}")
        print("];")
