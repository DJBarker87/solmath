#!/usr/bin/env python3
"""
Remez refit of Li IV coefficients optimising for Halley convergence speed.

The objective is NOT approximation error — it's the number of Halley iterations
needed to reach 100 ULP from the initial guess. Low-vega points get higher weight
because they converge slower per iteration.

Usage:
  python3 remez_li_convergence.py              # full run (hours)
  python3 remez_li_convergence.py --quick      # quick test (~10 min)
  python3 remez_li_convergence.py --grid 300   # custom grid density

Output: Rust constants to stdout, JSON to li_convergence_result.json
"""

import numpy as np
from scipy.optimize import differential_evolution
from scipy.stats import norm
import sys, time, json, argparse

SCALE = 10**12
INV_SQRT_2PI = 398_942_280_401  # 1/sqrt(2pi) at SCALE

# ============================================================
# Fixed-point arithmetic matching Rust mul_fast
# ============================================================

def mul_fast(a, b):
    p = a * b
    return p // SCALE if p >= 0 else -((-p) // SCALE)

def fp_div_i(a, b):
    if b == 0: return 0
    return (a * SCALE) // b

def fp_sqrt_py(x):
    if x <= 0: return 0
    return int(round(np.sqrt(x / SCALE) * SCALE))

# ============================================================
# BS normalised call (f64 reference)
# ============================================================

def bs_norm_call_f64(x, s):
    if s <= 1e-14:
        return max(np.exp(x/2) - np.exp(-x/2), 0) if x > 0 else 0.0
    h = x / s; t = s / 2
    return max(norm.cdf(h+t)*np.exp(x/2) - norm.cdf(h-t)*np.exp(-x/2), 0.0)

# ============================================================
# Li rational form in mul_fast arithmetic
# ============================================================

def li_eval_int(x_sc, c_sc, P):
    """Evaluate Li's rational form using integer mul_fast (matches Rust exactly)."""
    p1, p2, p3 = P[0], P[1], P[2]
    N = P[3:17]; M = P[17:31]

    sc = int(round(np.sqrt(c_sc / SCALE) * SCALE))
    sc3 = mul_fast(sc, c_sc); sc4 = mul_fast(c_sc, c_sc)

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
    linear = mul_fast(p1, x_sc) + mul_fast(p2, sc) + mul_fast(p3, c_sc)
    return linear + (num * SCALE) // den

# ============================================================
# Full Halley iteration in integer arithmetic (matches Rust iv_price_and_greeks)
# ============================================================

def norm_cdf_f64(x_sc):
    """norm_cdf_poly approximation — use scipy for reference."""
    return int(round(norm.cdf(x_sc / SCALE) * SCALE))

def norm_pdf_f64(x_sc):
    return int(round(norm.pdf(x_sc / SCALE) * SCALE))

def iv_price_and_greeks_int(x_i, ln_fk, s_i, k_disc, solve_as_put):
    """Matches Rust iv_price_and_greeks in integer arithmetic."""
    d1 = fp_div_i(ln_fk, x_i) + x_i // 2
    d2 = d1 - x_i

    phi_d1 = norm_cdf_f64(d1)
    pdf_d1 = norm_pdf_f64(d1)
    phi_d2 = norm_cdf_f64(d2)

    if solve_as_put:
        p = mul_fast(k_disc, SCALE - phi_d2) - mul_fast(s_i, SCALE - phi_d1)
        price_i = max(p, 0)
    else:
        c = mul_fast(s_i, phi_d1) - mul_fast(k_disc, phi_d2)
        price_i = max(c, 0)

    vega_x = mul_fast(s_i, pdf_d1)

    if x_i > 0 and vega_x > 0:
        volga_x = fp_div_i(mul_fast(vega_x, mul_fast(d1, d2)), x_i)
    else:
        volga_x = 0

    return price_i, vega_x, volga_x

def halley_step_int(x_u, f, vega_x, volga_x, x_lo, x_hi):
    """Matches Rust halley_step_bracketed."""
    if vega_x <= 1000:
        return (x_lo + x_hi) // 2
    two_f_fp = 2 * mul_fast(f, vega_x)
    denom = 2 * mul_fast(vega_x, vega_x) - mul_fast(f, volga_x)
    if abs(denom) > 1000:
        step = fp_div_i(two_f_fp, denom)
    else:
        step = fp_div_i(f, vega_x)
    new_x = x_u - step
    if new_x > x_lo and new_x < x_hi:
        return new_x
    return (x_lo + x_hi) // 2

def count_halley_iterations(guess_fp, true_sst_fp, ln_fk, s_i, k_disc, solve_as_put,
                             target_i, max_iter=6, tol=100):
    """Run actual Halley iterations and count how many needed to reach tol ULP."""
    sqrt_t = SCALE  # normalised: T=1 in sigma*sqrt(T) space

    x_lo = max(1, guess_fp // 10)
    x_hi = max(guess_fp * 5, SCALE * 5)
    x_u = max(1, min(guess_fp, x_hi - 1))

    for i in range(max_iter):
        x_i = x_u
        if x_i <= 1: return max_iter

        price_i, vega_x, volga_x = iv_price_and_greeks_int(x_i, ln_fk, s_i, k_disc, solve_as_put)
        f = price_i - target_i

        if abs(f) <= tol:
            return i  # converged at iteration i

        if f > 0 and x_u < x_hi: x_hi = x_u
        elif f < 0 and x_u > x_lo: x_lo = x_u

        x_u = halley_step_int(x_u, f, vega_x, volga_x, x_lo, x_hi)

    return max_iter

# ============================================================
# Generate training data with convergence metadata
# ============================================================

def generate_data(n_grid):
    """Generate (x_fp, c_fp, sst_fp, ln_fk, s_i, k_disc, solve_as_put, target_i, vega_real)."""
    data = []
    # In Li domain: |x| < 0.5 means |ln(F/K)| < 0.5
    # We model S=100, K varies, r=0 (so F=S, ln(F/K) = ln(S/K) = x)
    s_real = 100.0
    s_i = int(s_real * SCALE)

    xs = np.linspace(-0.48, 0.48, n_grid)
    sigmas = np.linspace(0.03, 0.97, n_grid)

    for x_real in xs:
        k_real = s_real * np.exp(-x_real)  # K = S/exp(x) so ln(S/K) = x
        k_i = int(round(k_real * SCALE))
        k_disc = k_i  # r=0
        ln_fk = int(round(x_real * SCALE))

        for sst_real in sigmas:
            c_real = bs_norm_call_f64(x_real, sst_real)
            if c_real < 0.001 or c_real > 0.95:
                continue

            c_fp = int(round(c_real * SCALE))
            sst_fp = int(round(sst_real * SCALE))

            # Determine solve_as_put and target
            solve_as_put = s_i > k_disc + k_disc // 20
            if solve_as_put:
                # compute put price from BS
                call_price = int(round(bs_norm_call_f64(x_real, sst_real) * s_real * SCALE))
                target_i = call_price - s_i + k_disc
                if target_i <= 0: target_i = 1
            else:
                target_i = int(round(bs_norm_call_f64(x_real, sst_real) * s_real * SCALE))

            # Vega for weighting
            if sst_real > 0.001:
                d1 = x_real / sst_real + sst_real / 2
                vega_real = norm.pdf(d1) * s_real
            else:
                vega_real = 0.001

            data.append((int(round(x_real * SCALE)), c_fp, sst_fp,
                         ln_fk, s_i, k_disc, solve_as_put, target_i,
                         vega_real))

    return data

# ============================================================
# Convergence objective
# ============================================================

def convergence_objective(params_flat, data_subset):
    """
    Objective: minimise max Halley iterations + weighted average.
    Primary: max_iters (want ≤ 3)
    Secondary: sum of (iters / vega) — penalises slow convergence in low-vega regions.
    """
    P = [int(round(p)) for p in params_flat]
    max_iters = 0
    weighted_sum = 0.0
    penalty = 0

    for (x_fp, c_fp, sst_fp, ln_fk, s_i, k_disc, solve_as_put, target_i, vega_real) in data_subset:
        try:
            guess = li_eval_int(x_fp, c_fp, P)
        except:
            penalty += 1
            continue

        if guess <= 0 or guess > 2 * SCALE:
            penalty += 1
            continue

        iters = count_halley_iterations(
            guess, sst_fp, ln_fk, s_i, k_disc, solve_as_put, target_i,
            max_iter=6, tol=100)

        max_iters = max(max_iters, iters)
        # Weight: 1/vega for low-vega emphasis
        weight = 1.0 / max(vega_real, 0.01)
        weighted_sum += iters * weight

    # Objective: max_iters dominates, weighted_sum breaks ties
    return max_iters * 1e15 + weighted_sum + penalty * 1e18

# ============================================================
# Current (original Li) coefficients
# ============================================================

ORIGINAL = [-969_271_876_255, 97_428_338_274, 1_750_081_126_685,
    -68_098_378_725, 440_639_436_211, -263_473_754_689, -5_792_537_721_792, -5_267_481_008_429,
    4_714_393_825_758, 3_529_944_137_559, -23_636_495_876_611, -9_020_361_771_283,
    14_749_084_301_452, -32_570_660_102_526, 76_398_155_779_133, 41_855_161_781_749, -12_150_611_865_704,
    6_268_456_292_246, -6_284_840_445_036, 30_068_281_276_567, -11_780_036_995_036, -2_310_966_989_723,
    -11_473_184_324_152, -230_101_682_610_568, 86_127_219_899_668, 3_730_181_294_225,
    -13_954_993_561_151, 261_950_288_864_225, 20_090_690_444_187, -50_117_067_019_539, 13_723_711_519_422]

# ============================================================
# Main
# ============================================================

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--quick", action="store_true")
    parser.add_argument("--grid", type=int, default=None)
    parser.add_argument("--de-iters", type=int, default=None)
    args = parser.parse_args()

    quick = args.quick
    n_grid = args.grid or (100 if quick else 200)
    de_iters = args.de_iters or (30 if quick else 500)
    de_pop = 15 if quick else 25
    n_subset = 1000 if quick else 5000

    print(f"{'QUICK' if quick else 'FULL'} mode: grid={n_grid}, DE iters={de_iters}, pop={de_pop}", file=sys.stderr)

    # Generate data
    print(f"Generating {n_grid}×{n_grid} training grid...", file=sys.stderr)
    t0 = time.time()
    data = generate_data(n_grid)
    print(f"  {len(data)} points ({time.time()-t0:.1f}s)", file=sys.stderr)

    # Baseline: count iterations with original coefficients
    print("\nBaseline (original Li)...", file=sys.stderr)
    iter_counts = []
    for pt in data:
        x_fp, c_fp, sst_fp, ln_fk, s_i, k_disc, solve_as_put, target_i, vega = pt
        guess = li_eval_int(x_fp, c_fp, ORIGINAL)
        if guess <= 0 or guess > 2 * SCALE:
            iter_counts.append(6)
            continue
        iters = count_halley_iterations(guess, sst_fp, ln_fk, s_i, k_disc, solve_as_put, target_i)
        iter_counts.append(iters)
    iter_counts.sort()
    n = len(iter_counts)
    print(f"  Halley iters: max={max(iter_counts)} P99={iter_counts[int(n*0.99)]} "
          f"P95={iter_counts[int(n*0.95)]} P50={iter_counts[n//2]} "
          f"mean={sum(iter_counts)/n:.2f}", file=sys.stderr)
    print(f"  Distribution: {[iter_counts.count(i) for i in range(7)]}", file=sys.stderr)

    # Subsample for DE
    np.random.seed(42)
    idx = np.random.choice(len(data), min(n_subset, len(data)), replace=False)
    subset = [data[i] for i in idx]

    # Baseline objective on subset
    base_obj = convergence_objective(ORIGINAL, subset)
    print(f"  Baseline objective (subset): {base_obj:.0f}", file=sys.stderr)

    # DE optimisation
    print(f"\nDE optimisation: {de_iters} iter, pop={de_pop}, {len(subset)} points...", file=sys.stderr)
    x0 = np.array(ORIGINAL, dtype=float)
    bounds = []
    for v in x0:
        mag = max(abs(v) * 0.30, 1e9)
        bounds.append((v - mag, v + mag))

    t1 = time.time()
    best_so_far = [None]
    def callback(xk, convergence):
        obj = convergence_objective(xk, subset)
        max_it = int(obj // 1e15)
        elapsed = time.time() - t1
        print(f"  DE: max_iters={max_it} obj={obj:.0f} ({elapsed:.0f}s)", file=sys.stderr)
        return False

    result = differential_evolution(
        convergence_objective, bounds, args=(subset,),
        x0=x0, seed=42, maxiter=de_iters, popsize=de_pop,
        tol=0, mutation=(0.5, 1.5), recombination=0.9,
        workers=-1, updating='deferred',
        callback=callback,
    )
    C_de = [int(round(v)) for v in result.x]
    print(f"  DE done in {time.time()-t1:.0f}s", file=sys.stderr)

    # Coordinate descent polish
    print("\nCoordinate descent polish...", file=sys.stderr)
    best = list(C_de)
    cur_obj = convergence_objective(best, subset)

    for rnd in range(15):
        improved = False
        for i in range(31):
            for delta in [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000]:
                for sign in [1, -1]:
                    trial = list(best)
                    trial[i] += sign * delta
                    trial_obj = convergence_objective(trial, subset)
                    if trial_obj < cur_obj:
                        best = trial
                        cur_obj = trial_obj
                        improved = True
        max_it = int(cur_obj // 1e15)
        print(f"  CD round {rnd+1}: max_iters={max_it} obj={cur_obj:.0f} ({time.time()-t1:.0f}s)", file=sys.stderr)
        if not improved:
            break

    # Full validation
    print("\nFull validation on all data...", file=sys.stderr)
    iter_counts_new = []
    approx_errs = []
    for pt in data:
        x_fp, c_fp, sst_fp, ln_fk, s_i, k_disc, solve_as_put, target_i, vega = pt
        guess = li_eval_int(x_fp, c_fp, best)
        approx_errs.append(abs(guess - sst_fp))
        if guess <= 0 or guess > 2 * SCALE:
            iter_counts_new.append(6)
            continue
        iters = count_halley_iterations(guess, sst_fp, ln_fk, s_i, k_disc, solve_as_put, target_i)
        iter_counts_new.append(iters)

    iter_counts_new.sort()
    approx_errs.sort()
    n = len(iter_counts_new)
    na = len(approx_errs)

    print(f"\n  ORIGINAL:", file=sys.stderr)
    print(f"    Halley iters: max={max(iter_counts)} P99={iter_counts[int(len(iter_counts)*0.99)]} "
          f"P50={iter_counts[len(iter_counts)//2]} mean={sum(iter_counts)/len(iter_counts):.2f}", file=sys.stderr)

    print(f"  NEW:", file=sys.stderr)
    print(f"    Halley iters: max={max(iter_counts_new)} P99={iter_counts_new[int(n*0.99)]} "
          f"P50={iter_counts_new[n//2]} mean={sum(iter_counts_new)/n:.2f}", file=sys.stderr)
    print(f"    Approx error: max={approx_errs[-1]} P99={approx_errs[int(na*0.99)]} "
          f"P50={approx_errs[na//2]}", file=sys.stderr)
    print(f"    Distribution: {[iter_counts_new.count(i) for i in range(7)]}", file=sys.stderr)

    # Output
    print(f"// Convergence-optimised Li IV guess coefficients")
    print(f"// Halley iters: max={max(iter_counts_new)} P99={iter_counts_new[int(n*0.99)]} "
          f"P50={iter_counts_new[n//2]} mean={sum(iter_counts_new)/n:.2f}")
    print(f"// Approx error: max={approx_errs[-1]} ULP, P50={approx_errs[na//2]} ULP")
    print(f"// Training: {len(data)} points, grid={n_grid}, DE iters={de_iters}")
    print(f"pub const LI_P1: i128 = {best[0]};")
    print(f"pub const LI_P2: i128 = {best[1]};")
    print(f"pub const LI_P3: i128 = {best[2]};")
    print(f"pub const LI_N: [i128; 14] = [")
    for i in range(14): print(f"    {best[3+i]:>25},")
    print(f"];")
    print(f"pub const LI_M: [i128; 14] = [")
    for i in range(14): print(f"    {best[17+i]:>25},")
    print(f"];")

    with open("li_convergence_result.json", "w") as f:
        json.dump({
            "coefficients": best,
            "halley_max": max(iter_counts_new),
            "halley_p99": iter_counts_new[int(n*0.99)],
            "halley_p50": iter_counts_new[n//2],
            "halley_mean": sum(iter_counts_new)/n,
            "halley_distribution": [iter_counts_new.count(i) for i in range(7)],
            "approx_max_ulp": approx_errs[-1],
            "approx_p50_ulp": approx_errs[na//2],
            "n_training": len(data),
            "original_halley_max": max(iter_counts),
            "original_halley_mean": sum(iter_counts)/len(iter_counts),
        }, f, indent=2)
    print(f"\nSaved to li_convergence_result.json", file=sys.stderr)
