#!/usr/bin/env python3
"""
Remez refit of Li (2006) IV rational guess for fixed-point mul_fast arithmetic.

Two-phase approach:
  Phase 1: L2 fit in f64 (scipy least_squares, fast) → good starting point
  Phase 2: Differential evolution in integer mul_fast space (slow, parallelisable)
  Phase 3: Coordinate descent polish on the DE result

Run on a multi-core machine:
  python3 remez_li_iv.py            # full run (hours)
  python3 remez_li_iv.py --quick    # quick test (minutes)

Output: prints Rust constants to stdout. Pipe to file:
  python3 remez_li_iv.py > li_coefficients.txt
"""

import numpy as np
from scipy.optimize import least_squares, differential_evolution
from scipy.stats import norm
import sys, time, json

SCALE = 10**12

# ============================================================
# Black-Scholes normalised call (reference)
# ============================================================

def bs_norm_call(x, s):
    """Normalised BS call: b(x,s) = Phi(x/s+s/2)*exp(x/2) - Phi(x/s-s/2)*exp(-x/2)"""
    if s <= 1e-14:
        return max(np.exp(x/2) - np.exp(-x/2), 0) if x > 0 else 0.0
    h = x / s; t = s / 2
    return max(norm.cdf(h+t)*np.exp(x/2) - norm.cdf(h-t)*np.exp(-x/2), 0.0)

# ============================================================
# Li rational form evaluation
# ============================================================

def mul_fast(a, b):
    """Python equivalent of Rust mul_fast: (a*b) // SCALE, truncation toward zero."""
    p = a * b
    return p // SCALE if p >= 0 else -((-p) // SCALE)

def li_eval_f64(x, c, params):
    """Li rational form in f64 (params are real-valued, not scaled)."""
    p1, p2, p3 = params[0], params[1], params[2]
    N = params[3:17]; M = params[17:31]
    sc = np.sqrt(c); sc3 = sc * c; sc4 = c * c

    h0_n = x * (N[1] + x*(N[4] + x*(N[8] + x*N[13])))
    h0_m = x * (M[1] + x*(M[4] + x*(M[8] + x*M[13])))
    h1_n = sc * (N[0] + x*(N[3] + x*(N[7] + x*N[12])))
    h1_m = sc * (M[0] + x*(M[3] + x*(M[7] + x*M[12])))
    h2_n = c * (N[2] + x*(N[6] + x*N[11]))
    h2_m = c * (M[2] + x*(M[6] + x*M[11]))
    h3_n = sc3 * (N[5] + x*N[10])
    h3_m = sc3 * (M[5] + x*M[10])
    h4_n = sc4 * N[9]; h4_m = sc4 * M[9]

    num = h0_n + h1_n + h2_n + h3_n + h4_n
    den = 1.0 + h0_m + h1_m + h2_m + h3_m + h4_m
    if abs(den) < 1e-15: return np.sqrt(2*np.pi) * sc
    return p1*x + p2*sc + p3*c + num/den

def li_eval_int(x_sc, c_sc, P):
    """Li rational form in integer mul_fast arithmetic (matches Rust exactly)."""
    sc = int(round(np.sqrt(c_sc / SCALE) * SCALE))
    sc3 = mul_fast(sc, c_sc); sc4 = mul_fast(c_sc, c_sc)
    N = P[3:17]; M = P[17:31]

    h0_n = mul_fast(x_sc, N[1] + mul_fast(x_sc, N[4] + mul_fast(x_sc, N[8] + mul_fast(x_sc, N[13]))))
    h0_m = mul_fast(x_sc, M[1] + mul_fast(x_sc, M[4] + mul_fast(x_sc, M[8] + mul_fast(x_sc, M[13]))))
    h1_n = mul_fast(sc, N[0] + mul_fast(x_sc, N[3] + mul_fast(x_sc, N[7] + mul_fast(x_sc, N[12]))))
    h1_m = mul_fast(sc, M[0] + mul_fast(x_sc, M[3] + mul_fast(x_sc, M[7] + mul_fast(x_sc, M[12]))))
    h2_n = mul_fast(c_sc, N[2] + mul_fast(x_sc, N[6] + mul_fast(x_sc, N[11])))
    h2_m = mul_fast(c_sc, M[2] + mul_fast(x_sc, M[6] + mul_fast(x_sc, M[11])))
    h3_n = mul_fast(sc3, N[5] + mul_fast(x_sc, N[10]))
    h3_m = mul_fast(sc3, M[5] + mul_fast(x_sc, M[10]))
    h4_n = mul_fast(sc4, N[9]); h4_m = mul_fast(sc4, M[9])

    num = h0_n + h1_n + h2_n + h3_n + h4_n
    den = SCALE + h0_m + h1_m + h2_m + h3_m + h4_m
    if abs(den) < 100: return mul_fast(2506628274631, sc)
    linear = mul_fast(P[0], x_sc) + mul_fast(P[1], sc) + mul_fast(P[2], c_sc)
    return linear + (num * SCALE) // den

# ============================================================
# Objective functions
# ============================================================

def max_err_int(P_list, data):
    """L-infinity error in integer arithmetic."""
    mx = 0
    for x_sc, c_sc, s_sc in data:
        try:
            err = abs(li_eval_int(int(x_sc), int(c_sc), P_list) - int(s_sc))
            if err > mx: mx = err
        except: return 10**18
    return mx

def de_objective(params, data):
    """DE objective: takes float params, rounds to int, evaluates."""
    P = [int(round(p)) for p in params]
    return float(max_err_int(P, data))

# ============================================================
# Current coefficients
# ============================================================

CURRENT = [-969271876255, 97428338274, 1750081126685,
    -68098378725, 440639436211, -263473754689, -5792537721792, -5267481008429,
    4714393825758, 3529944137559, -23636495876611, -9020361771283,
    14749084301452, -32570660102526, 76398155779133, 41855161781749, -12150611865704,
    6268456292246, -6284840445036, 30068281276567, -11780036995036, -2310966989723,
    -11473184324152, -230101682610568, 86127219899668, 3730181294225,
    -13954993561151, 261950288864225, 20090690444187, -50117067019539, 13723711519422]

# ============================================================
# Main
# ============================================================

if __name__ == "__main__":
    quick = "--quick" in sys.argv
    print(f"{'QUICK' if quick else 'FULL'} mode", file=sys.stderr)

    # Generate training data
    N_grid = 80 if quick else 150
    print(f"Generating {N_grid}x{N_grid} training grid...", file=sys.stderr)
    data_real = []
    for x in np.linspace(-0.48, 0.48, N_grid):
        for s in np.linspace(0.03, 0.97, N_grid):
            c = bs_norm_call(x, s)
            if 0.001 < c < 0.95:
                data_real.append((x, c, s))
    data_real = np.array(data_real)
    data_int = [(int(round(d[0]*SCALE)), int(round(d[1]*SCALE)), int(round(d[2]*SCALE)))
                for d in data_real]
    print(f"  {len(data_int)} points", file=sys.stderr)

    # Baseline
    errs_cur = sorted([abs(li_eval_int(x, c, CURRENT) - s) for x, c, s in data_int])
    print(f"  Current: P50={errs_cur[len(errs_cur)//2]:.0f} P95={errs_cur[int(len(errs_cur)*0.95)]:.0f} Max={errs_cur[-1]:.0f}", file=sys.stderr)

    # ── Phase 1: L2 fit in f64 ──
    print("\nPhase 1: L2 f64 fit...", file=sys.stderr)
    x_d, c_d, s_d = data_real[:,0], data_real[:,1], data_real[:,2]
    C0 = np.array(CURRENT, dtype=float)

    def residuals(params):
        return np.array([li_eval_f64(x, c, params/SCALE) - s
                         for x, c, s in zip(x_d, c_d, s_d)])

    res1 = least_squares(residuals, C0, method='lm', max_nfev=20000)
    C_f64 = res1.x
    C_rounded = [int(round(v)) for v in C_f64]
    errs_f64 = sorted([abs(li_eval_int(x, c, C_rounded) - s) for x, c, s in data_int])
    print(f"  L2 f64 max residual: {np.max(np.abs(res1.fun)):.8f}", file=sys.stderr)
    print(f"  Rounded int: P50={errs_f64[len(errs_f64)//2]:.0f} P95={errs_f64[int(len(errs_f64)*0.95)]:.0f} Max={errs_f64[-1]:.0f}", file=sys.stderr)

    # Pick better starting point
    if errs_f64[-1] < errs_cur[-1]:
        best_start = C_rounded
        print("  → L2 fit improved over current, using as start", file=sys.stderr)
    else:
        best_start = list(CURRENT)
        print("  → Current is better, keeping as start", file=sys.stderr)

    # ── Phase 2: DE in integer space ──
    N_de_points = 1500 if quick else 4000
    de_maxiter = 50 if quick else 500
    de_popsize = 15 if quick else 25

    np.random.seed(42)
    idx = np.random.choice(len(data_int), min(N_de_points, len(data_int)), replace=False)
    subset = [data_int[i] for i in idx]

    print(f"\nPhase 2: DE on {len(subset)} points, {de_maxiter} iter, pop {de_popsize}...", file=sys.stderr)
    x0 = np.array(best_start, dtype=float)
    bounds = []
    for v in x0:
        mag = max(abs(v) * 0.20, 1e9)
        bounds.append((v - mag, v + mag))

    t0 = time.time()
    res2 = differential_evolution(
        de_objective, bounds, args=(subset,),
        x0=x0, seed=42, maxiter=de_maxiter, popsize=de_popsize,
        tol=0, mutation=(0.5, 1.5), recombination=0.9,
        workers=-1, updating='deferred',
        callback=lambda xk, convergence: print(f"  DE iter: err={de_objective(xk, subset):.0f} ({time.time()-t0:.0f}s)", file=sys.stderr) or False,
    )
    C_de = [int(round(v)) for v in res2.x]
    print(f"  DE done in {time.time()-t0:.0f}s, best={res2.fun:.0f}", file=sys.stderr)

    # ── Phase 3: Coordinate descent ──
    print("\nPhase 3: Coordinate descent polish...", file=sys.stderr)
    best = list(C_de)
    cur_max = max_err_int(best, subset)

    for rnd in range(20):
        improved = False
        for i in range(31):
            for delta in [1, 5, 10, 50, 100, 500, 1000, 5000, 10000, 50000, 100000, 1000000]:
                for sign in [1, -1]:
                    trial = list(best)
                    trial[i] += sign * delta
                    trial_err = max_err_int(trial, subset)
                    if trial_err < cur_max:
                        best = trial
                        cur_max = trial_err
                        improved = True
        print(f"  Round {rnd+1}: max={cur_max:.0f} ({time.time()-t0:.0f}s)", file=sys.stderr)
        if not improved: break

    # Full validation
    errs_new = sorted([abs(li_eval_int(x, c, best) - s) for x, c, s in data_int])
    print(f"\nFinal:", file=sys.stderr)
    print(f"  Current: P50={errs_cur[len(errs_cur)//2]:.0f} P95={errs_cur[int(len(errs_cur)*0.95)]:.0f} Max={errs_cur[-1]:.0f}", file=sys.stderr)
    print(f"  New:     P50={errs_new[len(errs_new)//2]:.0f} P95={errs_new[int(len(errs_new)*0.95)]:.0f} Max={errs_new[-1]:.0f}", file=sys.stderr)
    print(f"  Improvement: {errs_cur[-1]/max(errs_new[-1],1):.2f}x max, {errs_cur[len(errs_cur)//2]/max(errs_new[len(errs_new)//2],1):.2f}x median", file=sys.stderr)

    # Output to stdout (Rust constants)
    print(f"// Remez-optimised Li IV guess coefficients")
    print(f"// Max error: {errs_new[-1]} ULP ({errs_new[-1]/SCALE:.8f} in sigma*sqrt(T))")
    print(f"// P50: {errs_new[len(errs_new)//2]} P95: {errs_new[int(len(errs_new)*0.95)]} P99: {errs_new[int(len(errs_new)*0.99)]}")
    print(f"// Training: {len(data_int)} points, x in [-0.48, 0.48], c in (0.001, 0.95)")
    print(f"pub const LI_P1: i128 = {best[0]}; // {best[0]/SCALE:.12f}")
    print(f"pub const LI_P2: i128 = {best[1]}; // {best[1]/SCALE:.12f}")
    print(f"pub const LI_P3: i128 = {best[2]}; // {best[2]/SCALE:.12f}")
    print(f"pub const LI_N: [i128; 14] = [")
    for i in range(14): print(f"    {best[3+i]:>25},  // n{i+1}")
    print(f"];")
    print(f"pub const LI_M: [i128; 14] = [")
    for i in range(14): print(f"    {best[17+i]:>25},  // m{i+1}")
    print(f"];")

    # Also dump JSON for programmatic use
    with open("li_remez_result.json", "w") as f:
        json.dump({"coefficients": best,
                    "max_ulp": int(errs_new[-1]),
                    "p50_ulp": int(errs_new[len(errs_new)//2]),
                    "p95_ulp": int(errs_new[int(len(errs_new)*0.95)]),
                    "n_training": len(data_int)}, f, indent=2)
    print(f"\nSaved to li_remez_result.json", file=sys.stderr)
