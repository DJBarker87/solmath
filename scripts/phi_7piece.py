#!/usr/bin/env python3
"""
7-piece norm_cdf_poly with boundary continuity, rounding Horner, coord descent.

Null-space reduction: C0, C1 derived from boundary constraints.
C2–C11 free, fitted at 10 Chebyshev nodes in normalized τ ∈ [-1, 1].
Coordinate descent on rounded integer C2–C11 with C0/C1 recomputed.
"""

import mpmath
from mpmath import mpf, cos, pi, matrix, lu_solve, nint, sqrt, erf
import time, json

mpmath.mp.dps = 60
SCALE = 10**12
DEGREE = 11

PIECES = [
    (0.0,  0.5),
    (0.5,  1.5),
    (1.5,  2.25),
    (2.25, 3.0),
    (3.0,  4.0),
    (4.0,  5.0),
]

def phi_exact_int(x):
    x_mp = mpf(x)
    return int(nint((1 + erf(x_mp / sqrt(2))) / 2 * SCALE))

def phi_exact_mp(x):
    x_mp = mpf(x)
    return (1 + erf(x_mp / sqrt(2))) / 2

# Boundary targets
BOUNDS = sorted(set(b for a, b in PIECES) | set(a for a, b in PIECES))
BT = {b: phi_exact_int(b) for b in BOUNDS}
BT[0.0] = SCALE // 2
print("Boundary targets:")
for b in BOUNDS:
    print(f"  Φ({b:5.2f}) = {BT[b]}")
print()

# ── Fixed-point rounding (matches Rust exactly) ──

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

# ── Constrained fitting in normalized τ ∈ [-1, 1] ──

def fit_constrained_normalized(a, b, left_target, right_target):
    """Fit degree-11 poly with P(τ=-1) = left_target, P(τ=+1) = right_target.
    Uses null-space: C0, C1 from constraints, C2–C11 at 10 Chebyshev nodes.
    All in normalized τ ∈ [-1, 1] (well-conditioned). Returns float coefficients."""

    mid = (a + b) / 2.0
    hw = (b - a) / 2.0
    L = mpf(left_target)
    R = mpf(right_target)

    # In normalized τ:
    # P(-1) = Σ C_j * (-1)^j = L
    # P(+1) = Σ C_j * (+1)^j = R
    # Sum:  2*(C0 + C2 + C4 + C6 + C8 + C10) = L + R
    # Diff: 2*(C1 + C3 + C5 + C7 + C9 + C11) = R - L
    # C0 = (L+R)/2 - C2 - C4 - C6 - C8 - C10
    # C1 = (R-L)/2 - C3 - C5 - C7 - C9 - C11

    half_sum = (L + R) / 2
    half_diff = (R - L) / 2

    # 10 Chebyshev nodes on (-1, 1)
    n_free = 10
    nodes = [cos(mpf(2*i + 1) / (2*n_free) * pi) for i in range(n_free)]

    # Build system: for each node τ, express P(τ) in terms of C2–C11
    # P(τ) = C0 + C1*τ + C2*τ^2 + ... + C11*τ^11
    # Substituting C0 and C1:
    # P(τ) = (half_sum - C2 - C4 - ...) + (half_diff - C3 - C5 - ...)*τ + C2*τ^2 + ... + C11*τ^11
    # For even j: C_j contributes (τ^j - 1) to P(τ) (from the -C_j in C0)
    # For odd j:  C_j contributes (τ^j - τ)  to P(τ) (from the -C_j in C1)

    A = matrix(n_free, n_free)
    b_vec = matrix(n_free, 1)

    for i, tau in enumerate(nodes):
        # Target: Φ at this node
        x_real = mpf(mid) + mpf(hw) * tau
        target = phi_exact_mp(x_real) * SCALE

        # Constant part from C0, C1 expressions
        constant = half_sum + half_diff * tau
        b_vec[i] = target - constant

        for j_idx in range(n_free):
            j = j_idx + 2
            if j % 2 == 0:
                A[i, j_idx] = tau**j - 1
            else:
                A[i, j_idx] = tau**j - tau

    c_free = lu_solve(A, b_vec)

    # Recover all 12 coefficients
    coeffs = [mpf(0)] * 12
    coeffs[0] = half_sum
    coeffs[1] = half_diff
    for j_idx in range(n_free):
        j = j_idx + 2
        coeffs[j] = c_free[j_idx]
        if j % 2 == 0:
            coeffs[0] -= c_free[j_idx]
        else:
            coeffs[1] -= c_free[j_idx]

    # Verify boundaries
    val_l = sum(coeffs[j] * mpf(-1)**j for j in range(12))
    val_r = sum(coeffs[j] for j in range(12))
    assert abs(float(val_l - L)) < 0.01, f"Left: {float(val_l)} vs {float(L)}"
    assert abs(float(val_r - R)) < 0.01, f"Right: {float(val_r)} vs {float(R)}"

    return [float(c) for c in coeffs]

# ── Round and recompute C0, C1 for fixed-point boundary pin ──

def round_and_pin(coeffs_float, mid_s, hw_s, left_target, right_target):
    """Round C2–C11 to int. Compute exact real-valued C0, C1 from boundary
    equations with rounded C2–C11. Try all 4 floor/ceil combos for C0/C1,
    evaluate each in actual rounding Horner at both boundaries, pick best.
    If best still misses, widen search by adjusting C2–C11 ±1."""

    coeffs = [0] * 12
    for j in range(2, 12):
        coeffs[j] = round(coeffs_float[j])

    t_left = map_t_round(mid_s - hw_s, mid_s, hw_s)
    t_right = map_t_round(mid_s + hw_s, mid_s, hw_s)

    def horner_without_c0(c, t):
        """Evaluate Horner from c[11] down to c[1]*t, return the 'rest' before adding c[0]."""
        r = c[11]
        for i in range(10, 0, -1):
            r = fp_mul_round(r, t) + c[i]
        return fp_mul_round(r, t)

    def try_c0_c1_directed(c, lt, rt):
        """Try all 4 floor/ceil combos for C0/C1.
        Enforce: vl >= lt (left not below target) and vr <= rt (right not above target).
        This ensures piece_k(right) <= target <= piece_{k+1}(left) at every join.
        Among valid combos, pick the one with smallest max(|vl-lt|, |vr-rt|).
        Returns (best_coeffs, boundary_ok)."""
        even_sum = sum(c[j] for j in range(2, 12, 2))
        odd_sum = sum(c[j] for j in range(3, 12, 2))
        c0_exact = (lt + rt) / 2.0 - even_sum
        c1_exact = (rt - lt) / 2.0 - odd_sum

        from math import floor, ceil
        c0_opts = sorted(set([int(floor(c0_exact)), int(ceil(c0_exact))]))
        c1_opts = sorted(set([int(floor(c1_exact)), int(ceil(c1_exact))]))

        best = None
        best_err = float('inf')
        # First pass: only combos satisfying directional constraint
        for c0 in c0_opts:
            for c1 in c1_opts:
                trial = [c0, c1] + c[2:]
                vl = horner_round(trial, t_left)
                vr = horner_round(trial, t_right)
                if vl < lt or vr > rt:
                    continue  # violates monotonicity direction
                err = abs(vl - lt) + abs(vr - rt)
                if err < best_err:
                    best_err = err
                    best = trial[:]
        # Fallback: if no combo satisfies direction, pick least-bad
        if best is None:
            for c0 in c0_opts:
                for c1 in c1_opts:
                    trial = [c0, c1] + c[2:]
                    vl = horner_round(trial, t_left)
                    vr = horner_round(trial, t_right)
                    err = abs(vl - lt) + abs(vr - rt)
                    if err < best_err:
                        best_err = err
                        best = trial[:]
        return best, best_err

    best, best_err = try_c0_c1_directed(coeffs, left_target, right_target)

    # If boundaries not exact, widen: try C2–C11 ±1
    if best_err > 0:
        for j in range(2, 12):
            for delta in [-1, +1]:
                trial_base = coeffs[:]
                trial_base[j] += delta
                cand, err = try_c0_c1_directed(trial_base, left_target, right_target)
                if err < best_err:
                    best = cand
                    best_err = err
                    coeffs = trial_base
                    if best_err == 0:
                        break
            if best_err == 0:
                break

    return best

# ── Coordinate descent ──

def gen_grid(a, b, n=50000):
    pts = set()
    w = b - a
    for i in range(int(n*0.8)):
        pts.add(a + w * i / max(int(n*0.8)-1, 1))
    for i in range(int(n*0.1)):
        pts.add(a + w * 0.01 * i / max(int(n*0.1)-1, 1))
    for i in range(int(n*0.1)):
        pts.add(b - w * 0.01 * i / max(int(n*0.1)-1, 1))
    grid = []
    for x in sorted(pts):
        x_s = int(round(x * SCALE))
        ref = phi_exact_int(x)
        grid.append((x_s, ref))
    return grid

def eval_max(coeffs, grid, mid_s, hw_s):
    mx = 0
    for x_s, ref in grid:
        t = map_t_round(x_s, mid_s, hw_s)
        actual = max(0, min(SCALE, horner_round(coeffs, t)))
        mx = max(mx, abs(actual - ref))
    return mx

def eval_stats(coeffs, grid, mid_s, hw_s):
    errs = []
    for x_s, ref in grid:
        t = map_t_round(x_s, mid_s, hw_s)
        actual = max(0, min(SCALE, horner_round(coeffs, t)))
        errs.append(abs(actual - ref))
    errs.sort()
    n = len(errs)
    return errs[-1], errs[int(n*0.99)], errs[n//2], sum(1 for e in errs if e==0)/n*100

def coord_descent(coeffs, grid, mid_s, hw_s, left_target, right_target, max_rounds=10):
    t_left = map_t_round(mid_s - hw_s, mid_s, hw_s)
    t_right = map_t_round(mid_s + hw_s, mid_s, hw_s)

    def try_repin(c):
        """Recompute C0/C1 with directional constraint. Returns (coeffs, ok)."""
        even_sum = sum(c[j] for j in range(2, 12, 2))
        odd_sum = sum(c[j] for j in range(3, 12, 2))
        c0_exact = (left_target + right_target) / 2.0 - even_sum
        c1_exact = (right_target - left_target) / 2.0 - odd_sum
        from math import floor, ceil
        c0_opts = sorted(set([int(floor(c0_exact)), int(ceil(c0_exact))]))
        c1_opts = sorted(set([int(floor(c1_exact)), int(ceil(c1_exact))]))
        best = None
        best_err = float('inf')
        for c0 in c0_opts:
            for c1 in c1_opts:
                trial = [c0, c1] + c[2:]
                vl = horner_round(trial, t_left)
                vr = horner_round(trial, t_right)
                if vl < left_target or vr > right_target:
                    continue
                err = abs(vl - left_target) + abs(vr - right_target)
                if err < best_err:
                    best_err = err
                    best = trial[:]
        if best is None:
            # Fallback: least-bad
            for c0 in c0_opts:
                for c1 in c1_opts:
                    trial = [c0, c1] + c[2:]
                    vl = horner_round(trial, t_left)
                    vr = horner_round(trial, t_right)
                    err = abs(vl - left_target) + abs(vr - right_target)
                    if err < best_err:
                        best_err = err
                        best = trial[:]
        return best, best_err

    best_max = eval_max(coeffs, grid, mid_s, hw_s)
    mx, p99, med, ex = eval_stats(coeffs, grid, mid_s, hw_s)
    print(f"    Start: max={mx} P99={p99} med={med} exact={ex:.1f}%")

    improved = True
    rnd = 0
    while improved and rnd < max_rounds:
        improved = False
        rnd += 1
        for i in range(2, 12):
            for delta in [-2, -1, +1, +2]:
                trial_base = coeffs[:]
                trial_base[i] += delta
                trial, berr = try_repin(trial_base)
                if trial is None:
                    continue
                # Verify directional constraint
                vl = horner_round(trial, t_left)
                vr = horner_round(trial, t_right)
                if vl < left_target or vr > right_target:
                    continue
                trial_mx = eval_max(trial, grid, mid_s, hw_s)
                if trial_mx < best_max:
                    coeffs = trial
                    best_max = trial_mx
                    improved = True
                    print(f"    C{i:>2} {delta:+d} → max={best_max}")

    mx, p99, med, ex = eval_stats(coeffs, grid, mid_s, hw_s)
    print(f"    Final: max={mx} P99={p99} med={med} exact={ex:.1f}% ({rnd} rds)")
    return coeffs, mx

# ── Main ──

if __name__ == '__main__':
    results = []

    for idx, (a, b) in enumerate(PIECES):
        mid = (a + b) / 2.0
        hw = (b - a) / 2.0
        mid_s = int(round(mid * SCALE))
        hw_s = int(round(hw * SCALE))
        lt = BT[a]
        rt = BT[b]

        print(f"\n{'='*60}")
        print(f"Piece {idx}: [{a}, {b}]  mid={mid}  hw={hw}")
        print(f"  left={lt}  right={rt}")
        print(f"{'='*60}")

        t0 = time.time()
        grid = gen_grid(a, b, 50000)
        print(f"  Grid: {len(grid)} pts ({time.time()-t0:.1f}s)")

        # Constrained fit
        print("  Fitting (null-space constrained)...")
        cf = fit_constrained_normalized(a, b, lt, rt)

        # Round and pin
        coeffs = round_and_pin(cf, mid_s, hw_s, lt, rt)

        # Verify boundaries
        tl = map_t_round(mid_s - hw_s, mid_s, hw_s)
        tr = map_t_round(mid_s + hw_s, mid_s, hw_s)
        vl = horner_round(coeffs, tl)
        vr = horner_round(coeffs, tr)
        print(f"  Boundaries: left_err={vl-lt} right_err={vr-rt}")

        # Coord descent
        print("  Optimizing...")
        t0 = time.time()
        opt, opt_max = coord_descent(coeffs, grid, mid_s, hw_s, lt, rt)
        print(f"  Took {time.time()-t0:.1f}s")

        # Final boundary check
        vl = horner_round(opt, tl)
        vr = horner_round(opt, tr)
        print(f"  Final boundaries: left_err={vl-lt} right_err={vr-rt}")

        results.append({"idx": idx, "a": a, "b": b, "mid": mid, "hw": hw,
                         "max": opt_max, "coeffs": opt, "le": vl-lt, "re": vr-rt})

    # ── Monotonicity check ──
    print(f"\n{'='*60}")
    print("MONOTONICITY CHECK")
    print(f"{'='*60}")
    for i in range(len(results) - 1):
        r = results[i]
        rn = results[i+1]
        bx = r["b"]
        bx_s = int(round(bx * SCALE))
        mid_l = int(round(r["mid"] * SCALE)); hw_l = int(round(r["hw"] * SCALE))
        mid_r = int(round(rn["mid"] * SCALE)); hw_r = int(round(rn["hw"] * SCALE))
        # Left piece at boundary
        v_at = horner_round(r["coeffs"], map_t_round(bx_s, mid_l, hw_l))
        # Right piece just above
        v_above = horner_round(rn["coeffs"], map_t_round(bx_s + 1, mid_r, hw_r))
        gap = v_above - v_at
        mono = "OK" if gap >= 0 else "FAIL"
        print(f"  x={bx}: left_piece={v_at} right_piece(+1)={v_above} gap={gap} {mono}")

    # ── Per-piece monotonicity sweep (100K points each) ──
    print(f"\n{'='*60}")
    print("PER-PIECE MONOTONICITY (100K pts each)")
    print(f"{'='*60}")
    for r in results:
        mid_s = int(round(r["mid"] * SCALE))
        hw_s = int(round(r["hw"] * SCALE))
        left_x = int(round(r["a"] * SCALE))
        right_x = int(round(r["b"] * SCALE))
        n_pts = 100000
        prev_val = -1
        reversals = 0
        for i in range(n_pts + 1):
            x_s = left_x + (right_x - left_x) * i // n_pts
            t = map_t_round(x_s, mid_s, hw_s)
            val = max(0, min(SCALE, horner_round(r["coeffs"], t)))
            if val < prev_val:
                reversals += 1
            prev_val = val
        status = "OK" if reversals == 0 else f"FAIL ({reversals} reversals)"
        print(f"  Piece {r['idx']} [{r['a']},{r['b']}]: {status}")

    # ── Summary ──
    print(f"\n{'='*60}")
    print("SUMMARY")
    print(f"{'='*60}")
    print(f"  {'Piece':8} {'Domain':14} {'Max':>6} {'L err':>6} {'R err':>6}")
    for r in results:
        print(f"  Piece {r['idx']}   [{r['a']},{r['b']}]{'':4} {r['max']:>6} {r['le']:>+6} {r['re']:>+6}")
    print(f"  Overall max: {max(r['max'] for r in results)}")

    # ── Rust constants ──
    print(f"\n{'='*60}")
    print("RUST CONSTANTS")
    print(f"{'='*60}")
    for r in results:
        mid_s = int(round(r["mid"] * SCALE))
        hw_s = int(round(r["hw"] * SCALE))
        print(f"\n// Piece {r['idx']}: [{r['a']},{r['b']}] mid={mid_s} hw={hw_s} max={r['max']} ULP")
        arr_name = f"POLY_V2_I{r['idx']}"
        print(f"pub const {arr_name}: [i128; 12] = [")
        for i, c in enumerate(r["coeffs"]):
            comma = "," if i < 11 else ""
            print(f"    {c}{comma}")
        print("];")

    # Boundaries
    print("\n// Piece boundaries (HI values)")
    for r in results[:-1]:
        hi = int(round(r["b"] * SCALE))
        print(f"pub const POLY_V2_I{r['idx']}_HI: i128 = {hi};")
    print("\n// Piece midpoints and half-widths")
    for r in results:
        mid_s = int(round(r["mid"] * SCALE))
        hw_s = int(round(r["hw"] * SCALE))
        print(f"pub const POLY_V2_I{r['idx']}_MID: i128 = {mid_s};")
        print(f"pub const POLY_V2_I{r['idx']}_HW: i128 = {hw_s};")
