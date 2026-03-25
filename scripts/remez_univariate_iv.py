#!/usr/bin/env python3
"""
Univariate Padé IV initial guess: Brenner-Subrahmanyam approach.

Approximation:
    σ√T ≈ β × R(z)
where:
    β = c × √(2π)          (Brenner-Subrahmanyam ATM approximation)
    z = x / β              (normalised log-moneyness)
    R(z) = P(z) / Q(z)     ([4/4] or [6/6] Padé)

Fitting is done in integer mul_fast arithmetic to match the Rust solver exactly.

Two phases:
  Phase 1: L2 fit in f64 → starting point for DE
  Phase 2: Differential evolution minimising max absolute error in mul_fast space
  Phase 3: Coordinate descent polish

Usage:
  python3 remez_univariate_iv.py              # [4/4] fit (default)
  python3 remez_univariate_iv.py --order 6    # [6/6] fit
  python3 remez_univariate_iv.py --quick      # fast test (smaller grid)

Output: prints Rust constants to stdout.
"""

import numpy as np
from scipy.optimize import differential_evolution, minimize
from scipy.stats import norm
import sys, time, json, argparse

SCALE = 10**12

# ============================================================
# Black-Scholes normalised call (high-precision reference)
# ============================================================

def bs_norm_call(x, s):
    """Normalised BS call: b(x,s) = Φ(x/s+s/2)·exp(x/2) - Φ(x/s-s/2)·exp(-x/2)"""
    if s <= 1e-14:
        return max(np.exp(x/2) - np.exp(-x/2), 0) if x > 0 else 0.0
    h = x / s
    t = s / 2
    return max(norm.cdf(h + t) * np.exp(x / 2) - norm.cdf(h - t) * np.exp(-x / 2), 0.0)

# ============================================================
# Try mpmath for higher precision training data
# ============================================================

try:
    import mpmath
    mpmath.mp.dps = 50
    HAS_MPMATH = True

    def bs_norm_call_hp(x, s):
        """High-precision normalised BS call using mpmath."""
        x = mpmath.mpf(x)
        s = mpmath.mpf(s)
        if s <= 1e-14:
            return float(max(mpmath.exp(x/2) - mpmath.exp(-x/2), 0))
        h = x / s
        t = s / 2
        v = mpmath.ncdf(h + t) * mpmath.exp(x / 2) - mpmath.ncdf(h - t) * mpmath.exp(-x / 2)
        return float(max(v, 0))
except ImportError:
    HAS_MPMATH = False
    bs_norm_call_hp = bs_norm_call

# ============================================================
# Fixed-point arithmetic (matches Rust exactly)
# ============================================================

def mul_fast(a, b):
    """Python equivalent of Rust mul_fast: (a*b) // SCALE, truncation toward zero."""
    p = a * b
    return p // SCALE if p >= 0 else -((-p) // SCALE)

def fp_div_i(a, b):
    """Python equivalent of Rust fp_div_i: (a * SCALE) // b, truncation toward zero."""
    if b == 0:
        return 0  # shouldn't happen with valid inputs
    p = a * SCALE
    return p // b if p >= 0 else -((-p) // b)

def isqrt(n):
    """Integer square root via Newton's method."""
    if n < 0:
        raise ValueError("isqrt of negative")
    if n == 0:
        return 0
    x = n
    y = (x + 1) // 2
    while y < x:
        x = y
        y = (x + n // x) // 2
    return x

def fp_sqrt(a):
    """Fixed-point sqrt matching Rust: sqrt(a * SCALE)."""
    return isqrt(a * SCALE)

# ============================================================
# Training data generation
# ============================================================

def generate_training_data(n_x=500, n_s=500, quick=False):
    """Generate (x, c, σ√T_true) training grid.

    x ∈ [-0.49, 0.49], σ√T ∈ [0.01, 1.5]
    Filter to c ∈ [0.001, 0.95] (the domain where the guess is used).
    """
    if quick:
        n_x, n_s = 100, 100

    bs_fn = bs_norm_call_hp if HAS_MPMATH else bs_norm_call

    # Log-spaced near boundaries, linear in middle
    x_grid = np.linspace(-0.49, 0.49, n_x)
    s_grid = np.linspace(0.01, 1.5, n_s)

    data = []
    for x in x_grid:
        for s in s_grid:
            c = bs_fn(x, s)
            if 0.001 <= c <= 0.95:
                data.append((x, c, s))

    print(f"Generated {len(data)} training points", file=sys.stderr)
    return data

# ============================================================
# Padé evaluation in f64 (for Phase 1)
# ============================================================

def pade_eval_f64(x, c, coeffs, order=4):
    """Evaluate Padé approximation in f64."""
    sqrt_2pi = np.sqrt(2 * np.pi)
    beta = c * sqrt_2pi
    if beta <= 1e-15:
        return 0.0
    z = x / beta

    p = coeffs[:order + 1]  # p0..p_order
    q = coeffs[order + 1:]  # q1..q_order

    # Horner for numerator: p0 + z(p1 + z(p2 + ...))
    num = p[order]
    for i in range(order - 1, -1, -1):
        num = num * z + p[i]

    # Horner for denominator: 1 + z(q1 + z(q2 + ...))
    den = q[order - 1]
    for i in range(order - 2, -1, -1):
        den = den * z + q[i]
    den = den * z + 1.0

    if abs(den) < 1e-15:
        return beta

    return beta * num / den

# ============================================================
# Padé evaluation in fixed-point (for Phase 2 — matches Rust)
# ============================================================

SQRT_2PI_FP = 2_506_628_274_631  # √(2π) × SCALE

def pade_eval_int(x_fp, c_fp, coeffs_fp, order=4):
    """Evaluate Padé in integer mul_fast arithmetic (matches Rust exactly)."""
    beta = mul_fast(c_fp, SQRT_2PI_FP)
    if beta <= 0:
        return 0

    z = fp_div_i(x_fp, beta)

    n_p = order + 1
    p = coeffs_fp[:n_p]
    q = coeffs_fp[n_p:]

    # Horner for numerator
    num = p[order]
    for i in range(order - 1, -1, -1):
        num = mul_fast(num, z) + p[i]

    # Horner for denominator
    den = q[order - 1]
    for i in range(order - 2, -1, -1):
        den = mul_fast(den, z) + q[i]
    den = mul_fast(den, z) + SCALE

    if abs(den) < 1000:
        return beta

    return mul_fast(beta, fp_div_i(num, den))

# ============================================================
# Phase 1: L2 fit in f64
# ============================================================

def phase1_fit(data, order=4):
    """L2 fit of Padé coefficients in floating point."""
    n_params = 2 * order + 1  # (order+1) numerator + order denominator

    def residuals(params):
        errs = []
        for x, c, s_true in data:
            s_pred = pade_eval_f64(x, c, params, order)
            errs.append(s_pred - s_true)
        return errs

    # Initial guess: p0=1.0, rest=0 (exact at ATM)
    x0 = np.zeros(n_params)
    x0[0] = 1.0  # p0

    print("Phase 1: L2 fit in f64...", file=sys.stderr)
    t0 = time.time()

    from scipy.optimize import least_squares
    result = least_squares(residuals, x0, method='lm', max_nfev=5000)

    print(f"  Phase 1 done in {time.time()-t0:.1f}s, "
          f"max_err={np.max(np.abs(result.fun)):.6f}, "
          f"rms={np.sqrt(np.mean(np.array(result.fun)**2)):.6f}", file=sys.stderr)

    return result.x

# ============================================================
# Phase 2: Differential evolution in integer space
# ============================================================

# Module-level state for DE objective (must be picklable for workers=-1)
_de_data_fp = []
_de_order = 4

def _de_objective(params_float):
    """Max absolute error in mul_fast arithmetic. Module-level for pickling."""
    if len(_de_data_fp) == 0:
        raise RuntimeError("Training data empty — fork bug")
    coeffs_fp = [int(round(p)) for p in params_float]
    max_err = 0
    for x_fp, c_fp, s_fp in _de_data_fp:
        s_pred = pade_eval_int(x_fp, c_fp, coeffs_fp, _de_order)
        err = abs(s_pred - s_fp)
        if err > max_err:
            max_err = err
    return max_err

def phase2_de(data, f64_coeffs, order=4, quick=False):
    """Differential evolution minimising max |error| in mul_fast arithmetic."""
    global _de_data_fp, _de_order
    _de_order = order
    n_params = 2 * order + 1

    # Convert f64 coefficients to scaled integers
    center = np.round(f64_coeffs * SCALE).astype(int)

    # Pre-compute fixed-point training data
    _de_data_fp = []
    for x, c, s_true in data:
        x_fp = int(round(x * SCALE))
        c_fp = int(round(c * SCALE))
        s_fp = int(round(s_true * SCALE))
        _de_data_fp.append((x_fp, c_fp, s_fp))

    # Search bounds: ±50% around f64 solution, wider for small coefficients.
    # Integer-optimal coefficients can be far from f64-optimal ones.
    bounds = []
    for c in center:
        width = max(abs(c) // 2, SCALE // 10)
        bounds.append((float(c - width), float(c + width)))

    popsize = 20 if quick else 40
    maxiter = 200 if quick else 1000

    print(f"Phase 2: DE in mul_fast space (popsize={popsize}, maxiter={maxiter})...",
          file=sys.stderr)
    t0 = time.time()

    result = differential_evolution(
        _de_objective, bounds,
        seed=42,
        popsize=popsize,
        maxiter=maxiter,
        tol=1e-3,
        mutation=(0.5, 1.5),
        recombination=0.9,
        workers=1,  # single-process — correct on all platforms
        x0=center.astype(float),
        disp=True,
    )

    best = [int(round(p)) for p in result.x]
    print(f"  Phase 2 done in {time.time()-t0:.1f}s, max_err={result.fun/SCALE:.6f} σ√T",
          file=sys.stderr)

    return best

# ============================================================
# Phase 3: Coordinate descent polish
# ============================================================

def phase3_polish(data, coeffs_fp, order=4, rounds=3):
    """Coordinate-wise polish: try ±δ for each coefficient."""
    data_fp = []
    for x, c, s_true in data:
        x_fp = int(round(x * SCALE))
        c_fp = int(round(c * SCALE))
        s_fp = int(round(s_true * SCALE))
        data_fp.append((x_fp, c_fp, s_fp))

    def max_err(coeffs):
        me = 0
        for x_fp, c_fp, s_fp in data_fp:
            s_pred = pade_eval_int(x_fp, c_fp, coeffs, order)
            err = abs(s_pred - s_fp)
            if err > me:
                me = err
        return me

    best = list(coeffs_fp)
    best_err = max_err(best)

    print(f"Phase 3: coordinate descent polish (start err={best_err/SCALE:.6f})...",
          file=sys.stderr)
    t0 = time.time()

    for round_idx in range(rounds):
        improved = False
        for i in range(len(best)):
            for delta in [1, -1, 10, -10, 100, -100, 1000, -1000,
                          10000, -10000, 100000, -100000]:
                trial = list(best)
                trial[i] += delta
                err = max_err(trial)
                if err < best_err:
                    best = trial
                    best_err = err
                    improved = True
        print(f"  Round {round_idx+1}: max_err={best_err/SCALE:.6f}", file=sys.stderr)
        if not improved:
            break

    print(f"  Phase 3 done in {time.time()-t0:.1f}s", file=sys.stderr)
    return best

# ============================================================
# Error analysis and reporting
# ============================================================

def error_analysis(data, coeffs_fp, order=4):
    """Detailed error analysis."""
    errs = []
    for x, c, s_true in data:
        x_fp = int(round(x * SCALE))
        c_fp = int(round(c * SCALE))
        s_fp = int(round(s_true * SCALE))
        s_pred = pade_eval_int(x_fp, c_fp, coeffs_fp, order)
        errs.append((s_pred - s_fp) / SCALE)

    errs = np.array(errs)
    print(f"\n{'='*60}", file=sys.stderr)
    print(f"Error Analysis ({len(errs)} points):", file=sys.stderr)
    print(f"  Max absolute error: {np.max(np.abs(errs)):.6f} σ√T", file=sys.stderr)
    print(f"  Mean absolute error: {np.mean(np.abs(errs)):.6f} σ√T", file=sys.stderr)
    print(f"  RMS error:           {np.sqrt(np.mean(errs**2)):.6f} σ√T", file=sys.stderr)
    print(f"  Max positive error:  {np.max(errs):.6f}", file=sys.stderr)
    print(f"  Max negative error:  {np.min(errs):.6f}", file=sys.stderr)

    # Percentiles
    abs_errs = np.abs(errs)
    for p in [50, 90, 95, 99, 99.9, 100]:
        print(f"  P{p:>5.1f}: {np.percentile(abs_errs, p):.6f}", file=sys.stderr)

    # Error by region
    for label, filt in [
        ("Near-ATM |x|<0.05", lambda d: abs(d[0]) < 0.05),
        ("Mid  0.05<|x|<0.2", lambda d: 0.05 <= abs(d[0]) < 0.2),
        ("Wing 0.2<|x|<0.5",  lambda d: 0.2 <= abs(d[0]) < 0.5),
    ]:
        idx = [i for i, d in enumerate(data) if filt(d)]
        if idx:
            sub = abs_errs[idx]
            print(f"  {label}: max={np.max(sub):.6f}, mean={np.mean(sub):.6f}",
                  file=sys.stderr)
    print(f"{'='*60}\n", file=sys.stderr)

# ============================================================
# Output Rust constants
# ============================================================

def print_rust_constants(coeffs_fp, order=4):
    """Print Rust constant declarations."""
    n_p = order + 1
    p = coeffs_fp[:n_p]
    q = coeffs_fp[n_p:]

    print(f"// Univariate Padé [{order}/{order}] IV guess coefficients")
    print(f"// Brenner-Subrahmanyam variable: β = c·√(2π), z = x/β")
    print(f"// σ√T ≈ β × P(z) / Q(z)")
    print(f"// Generated by scripts/remez_univariate_iv.py")
    for i, val in enumerate(p):
        print(f"pub const PADE_P{i}: i128 = {val:>20};")
    for i, val in enumerate(q):
        print(f"pub const PADE_Q{i+1}: i128 = {val:>20};")

    # Also print sqrt(2pi) for reference
    print(f"pub const SQRT_2PI: i128 = {SQRT_2PI_FP:>20}; // √(2π) × SCALE")

# ============================================================
# Main
# ============================================================

def main():
    parser = argparse.ArgumentParser(description="Fit univariate Padé IV guess")
    parser.add_argument("--order", type=int, default=4, choices=[4, 5, 6],
                        help="Padé order [N/N] (default: 4)")
    parser.add_argument("--quick", action="store_true",
                        help="Quick test with smaller grid")
    parser.add_argument("--grid", type=int, default=500,
                        help="Grid size per dimension (default: 500)")
    args = parser.parse_args()

    order = args.order
    n = args.grid if not args.quick else 100

    print(f"Fitting [{order}/{order}] Padé for IV initial guess", file=sys.stderr)
    print(f"Grid: {n}×{n}, mpmath={'yes' if HAS_MPMATH else 'no'}", file=sys.stderr)

    # Generate training data
    data = generate_training_data(n_x=n, n_s=n, quick=args.quick)

    # Subsample for DE if grid is very large
    if len(data) > 50000 and not args.quick:
        rng = np.random.RandomState(42)
        idx = rng.choice(len(data), 50000, replace=False)
        data_de = [data[i] for i in idx]
        print(f"Subsampled to {len(data_de)} points for DE", file=sys.stderr)
    else:
        data_de = data

    # Phase 1
    f64_coeffs = phase1_fit(data_de, order)
    error_analysis(data, [int(round(c * SCALE)) for c in f64_coeffs], order)

    # Phase 2
    int_coeffs = phase2_de(data_de, f64_coeffs, order, quick=args.quick)
    error_analysis(data, int_coeffs, order)

    # Phase 3
    final_coeffs = phase3_polish(data, int_coeffs, order, rounds=5)
    error_analysis(data, final_coeffs, order)

    # Output
    print_rust_constants(final_coeffs, order)

    # Also save as JSON for later use
    json_path = f"pade_{order}_{order}_coefficients.json"
    with open(json_path, 'w') as f:
        json.dump({
            "order": order,
            "coefficients": final_coeffs,
            "n_points": len(data),
        }, f, indent=2)
    print(f"\nCoefficients saved to {json_path}", file=sys.stderr)

if __name__ == "__main__":
    main()
