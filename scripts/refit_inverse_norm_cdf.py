#!/usr/bin/env python3
"""
Refit inverse_norm_cdf AS241 coefficients at SCALE = 1e12.
Precomputes all reference values, then does fast coordinate descent.
"""
import mpmath
from mpmath import mp, mpf, erfinv, sqrt
import sys

mp.dps = 60
SCALE = 10**12
SCALE_I = SCALE
HALF = SCALE // 2

# ── Fixed-point arithmetic matching Rust ──

def fp_mul_i_round(a, b):
    prod = a * b
    if prod >= 0:
        return (prod + HALF) // SCALE
    else:
        return -((-prod + HALF) // SCALE)

def fp_mul_i(a, b):
    return (a * b) // SCALE

def fp_div_i(a, b):
    if b == 0: return None
    # Matching Rust: truncation toward zero
    return (a * SCALE) // b

def fp_sqrt_precise(x):
    if x <= 0: return 0
    val = x * SCALE
    s = int(val ** 0.5)
    while s * s > val: s -= 1
    while (s + 1) * (s + 1) <= val: s += 1
    return s

def ln_fixed_i_mp(x_u):
    if x_u <= 0: return None
    return int(mp.nint(mp.log(mpf(x_u) / SCALE) * SCALE))

# ── Horner evaluation ──

def horner_7_round(c, r):
    acc = c[7]
    for i in range(6, -1, -1):
        acc = fp_mul_i_round(acc, r) + c[i]
    return acc

def horner_7_den_round(c, r):
    acc = c[6]
    for i in range(5, -1, -1):
        acc = fp_mul_i_round(acc, r) + c[i]
    acc = fp_mul_i_round(acc, r) + SCALE_I
    return acc

# ── Branch evaluation ──

AS241_SPLIT1 = 425_000_000_000
AS241_CONST1 = 180_625_000_000
AS241_SPLIT2 = 5_000_000_000_000
AS241_CONST2 = 1_600_000_000_000

def eval_branch1(q, A, B):
    r = AS241_CONST1 - fp_mul_i(q, q)
    num = horner_7_round(A, r)
    den = horner_7_den_round(B, r)
    if den == 0: return None
    return fp_div_i(fp_mul_i_round(q, num), den)

def eval_branch2(r, C, D):
    r_adj = r - AS241_CONST2
    num = horner_7_round(C, r_adj)
    den = horner_7_den_round(D, r_adj)
    if den == 0: return None
    return fp_div_i(num, den)

def eval_branch3(r, E, F):
    r_adj = r - AS241_SPLIT2
    num = horner_7_round(E, r_adj)
    den = horner_7_den_round(F, r_adj)
    if den == 0: return None
    return fp_div_i(num, den)

def inverse_normal_ref(p_scaled):
    p = mpf(p_scaled) / SCALE
    z = erfinv(2 * p - 1) * sqrt(2)
    return int(mp.nint(z * SCALE))

# ── Current coefficients ──

CUR_A = [3387132872796, 133141667891784, 1971590950306551, 13731693765509461,
         45921953931549871, 67265770927008701, 33430575583588128, 2509080928730123]
CUR_B = [42313330701601, 687187007492058, 5394196021424751, 21213794301586596,
         39307895800092711, 28729085735721943, 5226495278852855]
CUR_C = [1423437110750, 4630337846157, 5769497221461, 3647848324763,
         1270458252452, 241780725177, 22723844989, 774545014]
CUR_D = [2053191626638, 1676384830184, 689767334985, 148103976427,
         15198666564, 547593808, 1051]
CUR_E = [6657904643501, 5463784911164, 1784826539917, 296560571829,
         26532189527, 1242660947, 27115556, 201033]
CUR_F = [599832206556, 136929880923, 14875361291, 786869131, 18463183, 142151, 0]

# ── Build grids with precomputed references ──

def build_branch1_grid(n=4000):
    """Returns list of (q_scaled, z_ref)."""
    grid = []
    for i in range(n + 1):
        q_f = -0.425 + 0.85 * i / n
        q_scaled = int(round(q_f * SCALE))
        if abs(q_scaled) > AS241_SPLIT1:
            continue
        p = q_scaled + SCALE_I // 2
        if p <= 0 or p >= SCALE_I:
            continue
        z_ref = inverse_normal_ref(p)
        grid.append((q_scaled, z_ref))
    # Extra density near q=0 and boundaries
    for q_f in [0.0, 0.001, -0.001, 0.424, -0.424, 0.425, -0.425,
                0.42, -0.42, 0.1, -0.1, 0.2, -0.2, 0.3, -0.3]:
        q_scaled = int(round(q_f * SCALE))
        if abs(q_scaled) <= AS241_SPLIT1:
            p = q_scaled + SCALE_I // 2
            if 0 < p < SCALE_I:
                z_ref = inverse_normal_ref(p)
                grid.append((q_scaled, z_ref))
    return grid

def build_branch2_grid(n=3000):
    """Returns list of (r_scaled, sign, z_ref)."""
    grid = []
    # Lower tail: p < 0.075
    for i in range(n):
        log_p = -25 + (25 + float(mp.log(mpf('0.075')))) * i / n
        p_f = float(mp.exp(mpf(log_p)))
        ps = max(1, int(round(p_f * SCALE)))
        if ps <= 0 or ps >= SCALE_I:
            continue
        tail = ps  # lower tail
        ln_tail = ln_fixed_i_mp(tail)
        if ln_tail is None:
            continue
        neg_ln = -ln_tail
        if neg_ln < 0:
            continue
        r = fp_sqrt_precise(neg_ln)
        if r >= AS241_SPLIT2:
            continue
        z_ref = inverse_normal_ref(ps)
        grid.append((r, -1, z_ref))  # sign=-1 for lower tail
    # Upper tail too (symmetric)
    for i in range(n):
        log_p = -25 + (25 + float(mp.log(mpf('0.075')))) * i / n
        p_f = float(mp.exp(mpf(log_p)))
        ps_lower = max(1, int(round(p_f * SCALE)))
        ps = SCALE_I - ps_lower
        if ps <= 0 or ps >= SCALE_I:
            continue
        tail = SCALE_I - ps  # = ps_lower
        ln_tail = ln_fixed_i_mp(tail)
        if ln_tail is None:
            continue
        neg_ln = -ln_tail
        if neg_ln < 0:
            continue
        r = fp_sqrt_precise(neg_ln)
        if r >= AS241_SPLIT2:
            continue
        z_ref = inverse_normal_ref(ps)
        grid.append((r, 1, z_ref))
    return grid

def build_branch3_grid(n=2000):
    """Returns list of (r_scaled, sign, z_ref)."""
    grid = []
    for ps in range(1, n + 1):
        tail = ps
        ln_tail = ln_fixed_i_mp(tail)
        if ln_tail is None:
            continue
        neg_ln = -ln_tail
        if neg_ln < 0:
            continue
        r = fp_sqrt_precise(neg_ln)
        if r < AS241_SPLIT2:
            continue
        z_ref = inverse_normal_ref(ps)
        grid.append((r, -1, z_ref))
        # Upper mirror
        ps_upper = SCALE_I - ps
        if 0 < ps_upper < SCALE_I:
            z_ref_upper = inverse_normal_ref(ps_upper)
            grid.append((r, 1, z_ref_upper))
    return grid

# ── Max ULP computation (fast, no mpmath) ──

def branch1_max_ulp(A, B, grid):
    max_ulp = 0
    for q, z_ref in grid:
        z_test = eval_branch1(q, A, B)
        if z_test is None: continue
        ulp = abs(z_test - z_ref)
        if ulp > max_ulp: max_ulp = ulp
    return max_ulp

def branch2_max_ulp(C, D, grid):
    max_ulp = 0
    for r, sign, z_ref in grid:
        ret = eval_branch2(r, C, D)
        if ret is None: continue
        z_test = ret * sign
        ulp = abs(z_test - z_ref)
        if ulp > max_ulp: max_ulp = ulp
    return max_ulp

def branch3_max_ulp(E, F, grid):
    max_ulp = 0
    for r, sign, z_ref in grid:
        ret = eval_branch3(r, E, F)
        if ret is None: continue
        z_test = ret * sign
        ulp = abs(z_test - z_ref)
        if ulp > max_ulp: max_ulp = ulp
    return max_ulp

def branch_ulp_stats(max_ulp_fn, coeffs, grid, label=""):
    """Get full ULP stats."""
    ulps = []
    for item in grid:
        if len(item) == 2:
            q, z_ref = item
            z_test = eval_branch1(q, coeffs[0], coeffs[1])
        else:
            r, sign, z_ref = item
            if label == "B2":
                ret = eval_branch2(r, coeffs[0], coeffs[1])
            else:
                ret = eval_branch3(r, coeffs[0], coeffs[1])
            if ret is None: continue
            z_test = ret * sign
        if z_test is None: continue
        ulps.append(abs(z_test - z_ref))
    if not ulps:
        return
    ulps.sort()
    n = len(ulps)
    exact = sum(1 for u in ulps if u == 0)
    print(f"  {label}: n={n:5d} max={ulps[-1]:4d} P99={ulps[int(0.99*n)]:3d} "
          f"P95={ulps[int(0.95*n)]:3d} med={ulps[n//2]:3d} exact={exact}/{n}")

# ── Coordinate descent polish ──

def polish(coeffs_list, eval_fn, grid, n_rounds=8, label=""):
    """
    coeffs_list: list of coefficient arrays to polish, e.g. [A, B]
    eval_fn: function(num, den, grid) → max_ulp
    """
    best_max = eval_fn(coeffs_list[0], coeffs_list[1], grid)
    print(f"  {label} initial max ULP: {best_max}")

    for rnd in range(n_rounds):
        improved = False
        for ci, coeffs in enumerate(coeffs_list):
            for idx in range(len(coeffs)):
                for delta in [1, -1, 2, -2, 5, -5, 10, -10, 50, -50,
                              100, -100, 500, -500, 1000, -1000,
                              5000, -5000, 10000, -10000]:
                    old = coeffs[idx]
                    coeffs[idx] = old + delta
                    m = eval_fn(coeffs_list[0], coeffs_list[1], grid)
                    if m < best_max:
                        best_max = m
                        improved = True
                    else:
                        coeffs[idx] = old
        print(f"  {label} round {rnd+1}: max ULP = {best_max}")
        if not improved:
            break
    return coeffs_list

# ── Rational fitting via least-squares ──

def fit_rational_77_for_branch1(grid):
    """Fit P[7]/Q[7] for branch 1 in real arithmetic, then quantize."""
    import numpy as np

    # For branch 1: z = q * P(r) / Q(r), so P(r)/Q(r) = z/q
    # Generate target values in r-space
    rs = []
    targets = []
    for q_scaled, z_ref in grid:
        if q_scaled == 0:
            continue
        r = AS241_CONST1 - fp_mul_i(q_scaled, q_scaled)
        r_real = r / SCALE
        target = (z_ref / SCALE) / (q_scaled / SCALE)
        rs.append(r_real)
        targets.append(target)

    rs = np.array(rs)
    targets = np.array(targets)

    # Least-squares: P(r) - target*Q(r) = target (since Q has const term 1)
    n = len(rs)
    A = np.zeros((n, 15))
    rhs = np.array(targets)
    for i in range(n):
        x = rs[i]
        y = targets[i]
        for j in range(8):
            A[i, j] = x ** j
        for j in range(7):
            A[i, 8 + j] = -y * x ** (j + 1)

    sol, _, _, _ = np.linalg.lstsq(A, rhs, rcond=None)
    a_coeffs = [int(round(sol[j] * SCALE)) for j in range(8)]
    b_coeffs = [int(round(sol[8 + j] * SCALE)) for j in range(7)]
    return a_coeffs, b_coeffs

def fit_rational_77_for_tail(grid, eval_fn, const_offset):
    """Fit P[7]/Q[7] for tail branches in real arithmetic."""
    import numpy as np

    rs_adj = []
    targets = []
    for r_scaled, sign, z_ref in grid:
        r_adj = (r_scaled - const_offset) / SCALE
        target = abs(z_ref) / SCALE  # positive
        rs_adj.append(r_adj)
        targets.append(target)

    rs_adj = np.array(rs_adj)
    targets = np.array(targets)

    n = len(rs_adj)
    A = np.zeros((n, 15))
    rhs = np.array(targets)
    for i in range(n):
        x = rs_adj[i]
        y = targets[i]
        for j in range(8):
            A[i, j] = x ** j
        for j in range(7):
            A[i, 8 + j] = -y * x ** (j + 1)

    sol, _, _, _ = np.linalg.lstsq(A, rhs, rcond=None)
    num_coeffs = [int(round(sol[j] * SCALE)) for j in range(8)]
    den_coeffs = [int(round(sol[8 + j] * SCALE)) for j in range(7)]
    return num_coeffs, den_coeffs

def print_rust_constants(A, B, C, D, E, F):
    print("\n// ── Refitted AS241 coefficients (SCALE = 1e12, mpmath refit) ──\n")
    def fmt(name, arr, n):
        lines = [f"pub const {name}: [i128; {n}] = ["]
        for i, v in enumerate(arr):
            lines.append(f"    {v:>25},")
        lines.append("];")
        return "\n".join(lines)
    for name, arr, n in [("AS241_A", A, 8), ("AS241_B", B, 7),
                          ("AS241_C", C, 8), ("AS241_D", D, 7),
                          ("AS241_E", E, 8), ("AS241_F", F, 7)]:
        print(fmt(name, arr, n))
        print()

if __name__ == '__main__':
    print("=" * 60)
    print("Inverse Normal CDF Coefficient Refit")
    print(f"SCALE = {SCALE}, mpmath dps = {mp.dps}")
    print("=" * 60)

    # Step 1: Build grids with precomputed references
    print("\nBuilding reference grids (mpmath)...")
    sys.stdout.flush()
    g1 = build_branch1_grid(4000)
    print(f"  Branch 1: {len(g1)} points")
    sys.stdout.flush()
    g2 = build_branch2_grid(2000)
    print(f"  Branch 2: {len(g2)} points")
    sys.stdout.flush()
    g3 = build_branch3_grid(1000)
    print(f"  Branch 3: {len(g3)} points")
    sys.stdout.flush()

    # Step 2: Profile current
    print("\n── Current coefficients ──")
    branch_ulp_stats(None, (CUR_A, CUR_B), g1, "B1")
    branch_ulp_stats(None, (CUR_C, CUR_D), g2, "B2")
    branch_ulp_stats(None, (CUR_E, CUR_F), g3, "B3")
    sys.stdout.flush()

    # Step 3: Refit Branch 2 only (branches 1 and 3 are already ≤5 ULP)
    A_new, B_new = list(CUR_A), list(CUR_B)
    E_new, F_new = list(CUR_E), list(CUR_F)

    print("\n── Refitting Branch 2 (the 140 ULP bottleneck) ──")
    sys.stdout.flush()
    try:
        C_new, D_new = fit_rational_77_for_tail(g2, eval_branch2, AS241_CONST2)
        print(f"  LS fit done, initial max ULP: {branch2_max_ulp(C_new, D_new, g2)}")
    except Exception as e:
        print(f"  LS fit failed ({e}), starting from current coefficients")
        C_new, D_new = list(CUR_C), list(CUR_D)

    # Also try starting from current coefficients and see which is better
    C_cur, D_cur = list(CUR_C), list(CUR_D)
    m_ls = branch2_max_ulp(C_new, D_new, g2)
    m_cur = branch2_max_ulp(C_cur, D_cur, g2)
    print(f"  LS fit max: {m_ls}, current max: {m_cur}")
    if m_cur < m_ls:
        print("  Starting polish from current coefficients (better baseline)")
        C_new, D_new = C_cur, D_cur
    else:
        print("  Starting polish from LS fit (better baseline)")

    polish([C_new, D_new], branch2_max_ulp, g2, n_rounds=12, label="B2")

    # Step 4: Profile refitted
    print("\n── Refitted coefficients ──")
    branch_ulp_stats(None, (A_new, B_new), g1, "B1")
    branch_ulp_stats(None, (C_new, D_new), g2, "B2")
    branch_ulp_stats(None, (E_new, F_new), g3, "B3")

    # Step 5: Output
    print_rust_constants(A_new, B_new, C_new, D_new, E_new, F_new)
