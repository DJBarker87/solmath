#!/usr/bin/env python3
"""
Refit norm_cdf_poly_hp coefficients with coordinate descent.

HP already uses rounding multiply (fp_mul_hp_i rounds via +SCALE_HP/2).
But coefficients may not be optimally rounded. Run coord descent on each piece.

HP pieces:
  I0: [0, 0.5]     deg 10
  I1: [0.5, 1.5]   deg 13
  I2A: [1.5, 2.25] deg 15
  I2B: [2.25, 3.0] deg 15
  I3: [3.0, 5.0]   deg 17  ← split to [3,4] + [4,5] if needed
  I4: [5.0, 8.0]   tail (skip — uses PDF×Mills)

Strategy: first measure baseline, then split I3 and refit all polynomial pieces.
"""

import mpmath
from mpmath import mpf, cos, pi, matrix, lu_solve, nint, sqrt, erf
import numpy as np
import time

mpmath.mp.dps = 60
SCALE_HP = 10**15

def phi_exact_int(x):
    return int(nint((1 + erf(mpf(x) / sqrt(2))) / 2 * SCALE_HP))

def phi_exact_mp(x):
    return (1 + erf(mpf(x) / sqrt(2))) / 2

# ── HP fixed-point rounding (matches fp_mul_hp_u → fp_mul_hp_i) ──

def fp_mul_hp_u(a, b):
    """Matches Rust fp_mul_hp_u: split multiply with rounding on ll term."""
    S = SCALE_HP
    hi_a = a // S; lo_a = a % S
    hi_b = b // S; lo_b = b % S
    hh = hi_a * hi_b * S
    hl = hi_a * lo_b
    lh = lo_a * hi_b
    ll = (lo_a * lo_b + S // 2) // S
    return hh + hl + lh + ll

def fp_mul_hp_i(a, b):
    neg = (a < 0) != (b < 0)
    raw = fp_mul_hp_u(abs(a), abs(b))
    return -raw if neg else raw

def map_t_hp(ax_s, mid_s, hw_s):
    """Matches poly_map_t_hp: truncating division."""
    return (ax_s - mid_s) * SCALE_HP // hw_s

def horner_hp(coeffs, t):
    r = coeffs[-1]
    for i in range(len(coeffs) - 2, -1, -1):
        r = fp_mul_hp_i(r, t) + coeffs[i]
    return r

# ── Pieces (split I3 into [3,4] and [4,5]) ──

PIECES = [
    {"name": "I0",  "a": 0.0,  "b": 0.5,  "deg": 13},
    {"name": "I1",  "a": 0.5,  "b": 1.5,  "deg": 13},
    {"name": "I2A", "a": 1.5,  "b": 2.25, "deg": 15},
    {"name": "I2B", "a": 2.25, "b": 3.0,  "deg": 15},
    {"name": "I3A", "a": 3.0,  "b": 4.0,  "deg": 17},
    {"name": "I3B", "a": 4.0,  "b": 5.0,  "deg": 17},
]

# Boundary targets
BOUNDS = sorted(set(p["a"] for p in PIECES) | set(p["b"] for p in PIECES))
BT = {b: phi_exact_int(b) for b in BOUNDS}
BT[0.0] = SCALE_HP // 2
print("HP Boundary targets:")
for b in BOUNDS:
    print(f"  Φ({b:5.2f}) = {BT[b]}")
print()

# ── Constrained fitting (null-space in normalized τ) ──

def fit_constrained(a, b, deg, lt, rt):
    mid = (a + b) / 2.0
    hw = (b - a) / 2.0
    L = mpf(lt); R = mpf(rt)
    half_sum = (L + R) / 2
    half_diff = (R - L) / 2
    n_free = deg - 1  # C2..Cdeg
    nodes = [cos(mpf(2*i + 1) / (2*n_free) * pi) for i in range(n_free)]
    A = matrix(n_free, n_free)
    b_vec = matrix(n_free, 1)
    for i, tau in enumerate(nodes):
        x_real = mpf(mid) + mpf(hw) * tau
        target = phi_exact_mp(x_real) * SCALE_HP
        constant = half_sum + half_diff * tau
        b_vec[i] = target - constant
        for j_idx in range(n_free):
            j = j_idx + 2
            if j % 2 == 0:
                A[i, j_idx] = tau**j - 1
            else:
                A[i, j_idx] = tau**j - tau
    c_free = lu_solve(A, b_vec)
    coeffs = [mpf(0)] * (deg + 1)
    coeffs[0] = half_sum
    coeffs[1] = half_diff
    for j_idx in range(n_free):
        j = j_idx + 2
        coeffs[j] = c_free[j_idx]
        if j % 2 == 0:
            coeffs[0] -= c_free[j_idx]
        else:
            coeffs[1] -= c_free[j_idx]
    return [float(c) for c in coeffs]

# ── Round and pin (same approach as standard, adapted for HP) ──

def round_and_pin(coeffs_float, deg, lt, rt):
    from math import floor, ceil
    coeffs = [0] * (deg + 1)
    for j in range(2, deg + 1):
        coeffs[j] = round(coeffs_float[j])
    even_sum = sum(coeffs[j] for j in range(2, deg + 1, 2))
    odd_sum = sum(coeffs[j] for j in range(3, deg + 1, 2))
    c0_exact = (lt + rt) / 2.0 - even_sum
    c1_exact = (rt - lt) / 2.0 - odd_sum
    c0_opts = sorted(set([int(floor(c0_exact)), int(ceil(c0_exact))]))
    c1_opts = sorted(set([int(floor(c1_exact)), int(ceil(c1_exact))]))

    # t at boundaries = ±SCALE_HP. fp_mul_hp_i(r, SCALE_HP) = r (exact for small r).
    # But for large r, the split multiply may introduce error.
    # Evaluate in actual Horner to find best combo.
    # For HP, t_left = -SCALE_HP, t_right = +SCALE_HP
    t_left = -SCALE_HP
    t_right = SCALE_HP

    best = None; best_err = float('inf')
    for c0 in c0_opts:
        for c1 in c1_opts:
            trial = [c0, c1] + coeffs[2:]
            vl = horner_hp(trial, t_left)
            vr = horner_hp(trial, t_right)
            # Directional: vl >= lt, vr <= rt
            if vl < lt or vr > rt:
                continue
            err = abs(vl - lt) + abs(vr - rt)
            if err < best_err:
                best_err = err; best = trial[:]
    if best is None:
        # Fallback
        for c0 in c0_opts:
            for c1 in c1_opts:
                trial = [c0, c1] + coeffs[2:]
                vl = horner_hp(trial, t_left)
                vr = horner_hp(trial, t_right)
                err = abs(vl - lt) + abs(vr - rt)
                if err < best_err:
                    best_err = err; best = trial[:]
    return best

# ── Grid and evaluation ──

def gen_grid(a, b, n=30000):
    pts = set()
    w = b - a
    for i in range(int(n*0.8)):
        pts.add(a + w * i / max(int(n*0.8)-1, 1))
    for i in range(int(n*0.1)):
        pts.add(a + w * 0.02 * i / max(int(n*0.1)-1, 1))
    for i in range(int(n*0.1)):
        pts.add(b - w * 0.02 * i / max(int(n*0.1)-1, 1))
    grid = []
    for x in sorted(pts):
        x_s = int(round(x * SCALE_HP))
        ref = phi_exact_int(x)
        grid.append((x_s, ref))
    return grid

def eval_max(coeffs, grid, mid_s, hw_s):
    mx = 0
    for x_s, ref in grid:
        t = map_t_hp(x_s, mid_s, hw_s)
        actual = max(0, min(SCALE_HP, horner_hp(coeffs, t)))
        mx = max(mx, abs(actual - ref))
    return mx

def eval_stats(coeffs, grid, mid_s, hw_s):
    errs = []
    for x_s, ref in grid:
        t = map_t_hp(x_s, mid_s, hw_s)
        actual = max(0, min(SCALE_HP, horner_hp(coeffs, t)))
        errs.append(abs(actual - ref))
    errs.sort()
    n = len(errs)
    return errs[-1], errs[int(n*0.99)], errs[n//2], sum(1 for e in errs if e==0)/n*100

# ── Coordinate descent ──

def coord_descent(coeffs, deg, grid, mid_s, hw_s, lt, rt, max_rounds=10):
    from math import floor, ceil
    t_left = -SCALE_HP
    t_right = SCALE_HP

    def repin(c):
        even_sum = sum(c[j] for j in range(2, deg+1, 2))
        odd_sum = sum(c[j] for j in range(3, deg+1, 2))
        c0_exact = (lt + rt) / 2.0 - even_sum
        c1_exact = (rt - lt) / 2.0 - odd_sum
        c0_opts = sorted(set([int(floor(c0_exact)), int(ceil(c0_exact))]))
        c1_opts = sorted(set([int(floor(c1_exact)), int(ceil(c1_exact))]))
        best = None; best_err = float('inf')
        for c0 in c0_opts:
            for c1 in c1_opts:
                trial = [c0, c1] + c[2:]
                vl = horner_hp(trial, t_left)
                vr = horner_hp(trial, t_right)
                if vl < lt or vr > rt:
                    continue
                err = abs(vl - lt) + abs(vr - rt)
                if err < best_err:
                    best_err = err; best = trial[:]
        if best is None:
            for c0 in c0_opts:
                for c1 in c1_opts:
                    trial = [c0, c1] + c[2:]
                    vl = horner_hp(trial, t_left)
                    vr = horner_hp(trial, t_right)
                    err = abs(vl - lt) + abs(vr - rt)
                    if err < best_err:
                        best_err = err; best = trial[:]
        return best

    best_max = eval_max(coeffs, grid, mid_s, hw_s)
    mx, p99, med, ex = eval_stats(coeffs, grid, mid_s, hw_s)
    print(f"    Start: max={mx} P99={p99} med={med} exact={ex:.1f}%")

    improved = True; rnd = 0
    while improved and rnd < max_rounds:
        improved = False; rnd += 1
        for i in range(2, deg + 1):
            for delta in [-2, -1, +1, +2]:
                trial_base = coeffs[:]
                trial_base[i] += delta
                trial = repin(trial_base)
                if trial is None:
                    continue
                vl = horner_hp(trial, t_left)
                vr = horner_hp(trial, t_right)
                if vl < lt or vr > rt:
                    continue
                trial_mx = eval_max(trial, grid, mid_s, hw_s)
                if trial_mx < best_max:
                    coeffs = trial; best_max = trial_mx; improved = True
                    print(f"    C{i:>2} {delta:+d} → max={best_max}")

    mx, p99, med, ex = eval_stats(coeffs, grid, mid_s, hw_s)
    print(f"    Final: max={mx} P99={p99} med={med} exact={ex:.1f}% ({rnd} rds)")
    return coeffs, mx

# ── Main ──

if __name__ == '__main__':
    results = []

    for piece in PIECES:
        a, b, deg = piece["a"], piece["b"], piece["deg"]
        name = piece["name"]
        mid = (a + b) / 2.0
        hw = (b - a) / 2.0
        mid_s = int(round(mid * SCALE_HP))
        hw_s = int(round(hw * SCALE_HP))
        lt = BT[a]; rt = BT[b]

        print(f"\n{'='*60}")
        print(f"{name}: [{a}, {b}]  deg={deg}  mid={mid}  hw={hw}")
        print(f"{'='*60}")

        t0 = time.time()
        grid = gen_grid(a, b, 30000)
        print(f"  Grid: {len(grid)} pts ({time.time()-t0:.1f}s)")

        print("  Fitting...")
        cf = fit_constrained(a, b, deg, lt, rt)
        coeffs = round_and_pin(cf, deg, lt, rt)

        tl = horner_hp(coeffs, -SCALE_HP)
        tr = horner_hp(coeffs, SCALE_HP)
        print(f"  Boundaries: left_err={tl-lt} right_err={tr-rt}")

        print("  Coord descent...")
        t0 = time.time()
        opt, opt_max = coord_descent(coeffs, deg, grid, mid_s, hw_s, lt, rt)
        print(f"  Took {time.time()-t0:.1f}s")

        tl = horner_hp(opt, -SCALE_HP)
        tr = horner_hp(opt, SCALE_HP)
        print(f"  Final boundaries: left_err={tl-lt} right_err={tr-rt}")

        results.append({"name": name, "a": a, "b": b, "deg": deg,
                         "max": opt_max, "coeffs": opt, "le": tl-lt, "re": tr-rt})

    # ── Summary ──
    print(f"\n{'='*60}")
    print("SUMMARY")
    print(f"{'='*60}")
    print(f"  {'Name':6} {'Domain':14} {'Deg':>4} {'Max':>6} {'L err':>6} {'R err':>6}")
    for r in results:
        print(f"  {r['name']:6} [{r['a']},{r['b']}]{'':4} {r['deg']:>4} {r['max']:>6} {r['le']:>+6} {r['re']:>+6}")
    print(f"  Overall max: {max(r['max'] for r in results)}")

    # ── Rust constants ──
    print(f"\n{'='*60}")
    print("RUST CONSTANTS")
    print(f"{'='*60}")
    for r in results:
        n = r["deg"] + 1
        mid_s = int(round(((r["a"]+r["b"])/2) * SCALE_HP))
        hw_s = int(round(((r["b"]-r["a"])/2) * SCALE_HP))
        print(f"\n// {r['name']}: [{r['a']},{r['b']}] deg={r['deg']} max={r['max']} ULP")
        print(f"pub const POLY_HP_V2_{r['name']}: [i128; {n}] = [")
        for i, c in enumerate(r["coeffs"]):
            comma = "," if i < n - 1 else ""
            print(f"    {c}{comma}")
        print("];")
    # Boundaries and midpoints
    print()
    for i, r in enumerate(results[:-1]):
        hi = int(round(r["b"] * SCALE_HP))
        print(f"pub const POLY_HP_V2_{r['name']}_HI: i128 = {hi};")
    print()
    for r in results:
        mid_s = int(round(((r["a"]+r["b"])/2) * SCALE_HP))
        hw_s = int(round(((r["b"]-r["a"])/2) * SCALE_HP))
        print(f"pub const POLY_HP_V2_{r['name']}_MID: i128 = {mid_s};")
        print(f"pub const POLY_HP_V2_{r['name']}_HW: i128 = {hw_s};")
