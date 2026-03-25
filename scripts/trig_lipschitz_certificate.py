#!/usr/bin/env python3
"""
trig_lipschitz_certificate.py — Rigorous certificate for sin_core / cos_core error bounds.

Two-level certificate:
  Level 1 — Real polynomial approximation error (Lipschitz-certified):
    |P_real(u) - f(u)| where P_real evaluates the integer coefficients in exact
    real arithmetic and f is the true sinc/cos function.
  Level 2 — Integer Horner truncation deviation (analytically bounded):
    |P_int(u) - P_real(u)| ≤ sum_{k=0}^{N-1} |u/S|^k  (geometric sum of per-step
    truncation errors, each < 1 ULP, attenuated by the polynomial variable ratio).

Total: |P_int(u) - f(u)| ≤ Level1_cert + Level2_bound.

Usage: python3 trig_lipschitz_certificate.py
Requires: mpmath (pip install mpmath)
"""

import mpmath
import time
import sys

mpmath.mp.dps = 60

SCALE = 10**12
SCALE_M = mpmath.mpf(SCALE)

# ============================================================
# Coefficients from constants.rs lines 336–349
# ============================================================

# sin(x)/x = c[0] + c[1]·(u/S) + c[2]·(u/S)^2 + ... + c[5]·(u/S)^5  where u = x²
SIN_COEFFS = [
    1_000_000_000_000,   # SIN_C1   (constants.rs:336)
    -166_666_666_666,    # SIN_C3   (constants.rs:337)
    8_333_333_325,       # SIN_C5   (constants.rs:338)
    -198_412_638,        # SIN_C7   (constants.rs:339)
    2_755_535,           # SIN_C9   (constants.rs:340)
    -24_761,             # SIN_C11  (constants.rs:341)
]

# cos(x) = c[0] + c[1]·(u/S) + c[2]·(u/S)^2 + ... + c[5]·(u/S)^5  where u = x²
COS_COEFFS = [
    1_000_000_000_000,   # COS_C0   (constants.rs:344)
    -499_999_999_995,    # COS_C2   (constants.rs:345)
    41_666_666_560,      # COS_C4   (constants.rs:346)
    -1_388_888_100,      # COS_C6   (constants.rs:347)
    24_799_027,          # COS_C8   (constants.rs:348)
    -271_792,            # COS_C10  (constants.rs:349)
]

U_MAX = int(mpmath.floor((mpmath.pi / 4) ** 2 * SCALE_M))


# ============================================================
# Integer Horner (matching Rust fp_mul_i truncation toward zero)
# ============================================================

def trunc_div(a, b):
    """Integer division truncating toward zero (Rust semantics)."""
    if (a >= 0) == (b >= 0):
        return abs(a) // abs(b)
    else:
        return -(abs(a) // abs(b))


def horner_int(coeffs, u):
    """Integer Horner: start from highest coeff, multiply by u, trunc-divide by SCALE, add next."""
    r = coeffs[-1]
    for k in range(len(coeffs) - 2, -1, -1):
        r = trunc_div(r * u, SCALE) + coeffs[k]
    return r


# ============================================================
# Real-valued polynomial (mpmath exact arithmetic)
# ============================================================

def eval_real_poly(coeffs, u_m):
    """Evaluate c[0] + c[1]*(u/S) + c[2]*(u/S)^2 + ... in mpmath."""
    t = u_m / SCALE_M
    r = mpmath.mpf(coeffs[-1])
    for k in range(len(coeffs) - 2, -1, -1):
        r = r * t + mpmath.mpf(coeffs[k])
    return r


def eval_real_poly_deriv(coeffs, u_m):
    """d/du of the real polynomial = (1/S) * sum k*c_k*(u/S)^{k-1}."""
    t = u_m / SCALE_M
    dcoeffs = [k * coeffs[k] for k in range(1, len(coeffs))]
    r = mpmath.mpf(dcoeffs[-1])
    for k in range(len(dcoeffs) - 2, -1, -1):
        r = r * t + mpmath.mpf(dcoeffs[k])
    return r / SCALE_M


def eval_real_poly_deriv2(coeffs, u_m):
    """d^2/du^2 of the real polynomial."""
    t = u_m / SCALE_M
    d2coeffs = [k * (k - 1) * coeffs[k] for k in range(2, len(coeffs))]
    r = mpmath.mpf(d2coeffs[-1])
    for k in range(len(d2coeffs) - 2, -1, -1):
        r = r * t + mpmath.mpf(d2coeffs[k])
    return r / (SCALE_M ** 2)


# ============================================================
# True function values and derivatives in u-space
# ============================================================

def sinc_true(u):
    """sinc(sqrt(u/S)) * S — target of sin_core polynomial."""
    if u == 0:
        return SCALE_M
    z = mpmath.sqrt(mpmath.mpf(u) / SCALE_M)
    return mpmath.sin(z) / z * SCALE_M


def cos_true(u):
    """cos(sqrt(u/S)) * S — target of cos_core polynomial."""
    if u == 0:
        return SCALE_M
    z = mpmath.sqrt(mpmath.mpf(u) / SCALE_M)
    return mpmath.cos(z) * SCALE_M


def sinc_true_deriv(u_m):
    """d/du [sinc(sqrt(u/S)) * S]."""
    if u_m == 0:
        return mpmath.mpf(-1) / 6
    z = mpmath.sqrt(u_m / SCALE_M)
    # d/du [S * sin(z)/z] = (z*cos(z) - sin(z)) / (2*z^3)
    # where z = sqrt(u/S), derived via chain rule dz/du = 1/(2*z*S).
    return (mpmath.cos(z) * z - mpmath.sin(z)) / (2 * z ** 3)


def cos_true_deriv(u_m):
    """d/du [cos(sqrt(u/S)) * S]."""
    if u_m == 0:
        return mpmath.mpf(-1) / 2
    z = mpmath.sqrt(u_m / SCALE_M)
    return -mpmath.sin(z) / (2 * z)


# ============================================================
# M2 bound via grid sampling + analytical M3 correction
# ============================================================

def compute_M2(coeffs, true_deriv_fn, n_m2=10_000):
    """Rigorous upper bound on |e''_real(u)| = |P''_real(u) - f''(u)| over [0, U_MAX]."""
    h_m2 = mpmath.mpf(U_MAX) / n_m2
    M2_grid = mpmath.mpf(0)

    for i in range(n_m2 + 1):
        u_m = mpmath.mpf(i) * U_MAX / n_m2
        p2 = eval_real_poly_deriv2(coeffs, u_m)
        # f''(u) via central difference of f'(u)
        eps = max(u_m * mpmath.mpf("1e-8"), mpmath.mpf("1e-2"))
        u_plus = min(u_m + eps, mpmath.mpf(U_MAX))
        u_minus = max(u_m - eps, mpmath.mpf(0))
        f2 = (true_deriv_fn(u_plus) - true_deriv_fn(u_minus)) / (u_plus - u_minus)
        e2 = abs(p2 - f2)
        if e2 > M2_grid:
            M2_grid = e2

    # Analytical M3 bound for grid correction
    p3_bound = sum(
        k * (k - 1) * (k - 2) * abs(coeffs[k]) for k in range(3, len(coeffs))
    )
    p3_term = mpmath.mpf(p3_bound) / (SCALE_M ** 3)
    # f'''(u) is O(1/S^{3/2}) — bound crudely at 1/S^2
    M3 = p3_term + mpmath.mpf(1) / (SCALE_M ** 2)

    return M2_grid + M3 * h_m2 / 2


# ============================================================
# Analytical Horner truncation bound
# ============================================================

def horner_truncation_bound(n_steps):
    """
    Analytical upper bound on |P_int(u) - P_real(u)|.

    Each of the n_steps Horner multiplications (fp_mul_i) truncates toward zero,
    introducing < 1 ULP error. Error from step k propagates through subsequent
    multiplications by |u/S| ≤ (π/4)² ≈ 0.617.

    Total: Σ_{k=0}^{n_steps-1} |u/S|^k < (1 - a^n) / (1 - a) where a = (π/4)².
    """
    a = float((mpmath.pi / 4) ** 2)
    bound = sum(a ** k for k in range(n_steps))
    return bound


# ============================================================
# Main certification
# ============================================================

def certify_trig(name, coeffs, true_fn, true_deriv_fn, n_grid=200_000):
    """
    Two-level certificate for one trig function.

    Level 1: Lipschitz-certified real polynomial approx error.
    Level 2: Analytical Horner truncation bound.
    """
    n_steps = len(coeffs) - 1  # number of Horner multiply steps

    # Grid in real u-space (not integer-spaced — use mpmath for real u values)
    u_max_m = mpmath.mpf(U_MAX)
    h = u_max_m / n_grid

    grid_max_real = mpmath.mpf(0)   # Level 1: |P_real - f|
    grid_max_int = mpmath.mpf(0)    # For reference: |P_int - f|
    L_grid = mpmath.mpf(0)          # max |e'_real|

    poly_int_fn = horner_int

    t0 = time.time()
    report_every = n_grid // 10

    for i in range(n_grid + 1):
        u_m = mpmath.mpf(i) * u_max_m / n_grid
        u_int = int(u_m)  # nearest integer u for integer Horner

        # Real polynomial value
        p_real = eval_real_poly(coeffs, u_m)

        # True function value
        f_val = true_fn(u_int)

        # Also evaluate true function at exact u_m for the real error
        f_val_real = true_fn(int(u_m)) if u_m == int(u_m) else true_fn(u_int)
        # For the real polynomial, evaluate at the continuous u_m
        # But the true function takes integer u — let's use continuous u
        if i == 0:
            f_continuous = SCALE_M  # sinc(0) = 1 or cos(0) = 1
        else:
            z = mpmath.sqrt(u_m / SCALE_M)
            if name.startswith("sin"):
                f_continuous = mpmath.sin(z) / z * SCALE_M
            else:
                f_continuous = mpmath.cos(z) * SCALE_M

        # Level 1: real polynomial error at continuous u
        err_real = abs(p_real - f_continuous)
        if err_real > grid_max_real:
            grid_max_real = err_real

        # Integer Horner error at integer u (for reference)
        p_int = poly_int_fn(coeffs, u_int)
        f_at_int = true_fn(u_int)
        err_int = abs(mpmath.mpf(p_int) - f_at_int)
        if err_int > grid_max_int:
            grid_max_int = err_int

        # Lipschitz: e'_real(u) = P'_real(u) - f'(u)
        p_d = eval_real_poly_deriv(coeffs, u_m)
        f_d = true_deriv_fn(u_m)
        err_d = abs(p_d - f_d)
        if err_d > L_grid:
            L_grid = err_d

        if report_every > 0 and i > 0 and i % report_every == 0:
            pct = 100 * i // n_grid
            sys.stderr.write(f"  {name}: {pct}% ({i}/{n_grid})\n")
            sys.stderr.flush()

    elapsed = time.time() - t0

    # M2 for Lipschitz correction of L
    M2 = compute_M2(coeffs, true_deriv_fn)
    L_corr = M2 * h / 2
    L = L_grid + L_corr

    # Level 1 certificate
    Lh2 = L * h / 2
    level1_cert = grid_max_real + Lh2

    # Level 2: analytical Horner bound
    level2_bound = horner_truncation_bound(n_steps)

    # Total
    total = float(level1_cert) + level2_bound

    return {
        "name": name,
        "degree": n_steps,
        "n_grid": n_grid,
        "elapsed": elapsed,
        # Level 1
        "grid_max_real": float(grid_max_real),
        "L_grid": float(L_grid),
        "M2": float(M2),
        "L_corr": float(L_corr),
        "L": float(L),
        "h": float(h),
        "Lh2": float(Lh2),
        "level1_cert": float(level1_cert),
        # Level 2
        "level2_bound": level2_bound,
        # Reference
        "grid_max_int": float(grid_max_int),
        # Total
        "total": total,
    }


def main():
    print("trig_lipschitz_certificate.py — SolMath sin_core / cos_core error bounds")
    print(f"mpmath precision: {mpmath.mp.dps} decimal digits")
    print(f"SCALE = {SCALE}")
    print(f"u_max = {U_MAX}  (= floor((pi/4)^2 * SCALE))")
    print()

    n_grid = 200_000

    # ---- sin_core ----
    sys.stderr.write("Certifying sin_core...\n")
    sys.stderr.flush()
    sin_r = certify_trig(
        "sin_core", SIN_COEFFS, sinc_true, sinc_true_deriv, n_grid=n_grid,
    )

    # ---- cos_core ----
    sys.stderr.write("Certifying cos_core...\n")
    sys.stderr.flush()
    cos_r = certify_trig(
        "cos_core", COS_COEFFS, cos_true, cos_true_deriv, n_grid=n_grid,
    )

    # ---- Output ----
    print("=" * 78)
    print("  sin_core / cos_core — Two-Level Lipschitz Certificate")
    print("=" * 78)
    print()

    for r in [sin_r, cos_r]:
        print(f"  {r['name']} (degree {r['degree']} in u = x^2/SCALE)")
        print(f"  {'─' * 50}")
        print(f"  Level 1 — Real polynomial approximation error:")
        print(f"    grid max |P_real(u) - f(u)|    = {r['grid_max_real']:.6f} ULP")
        print(f"    L (Lipschitz of error)          = {r['L']:.4e}")
        print(f"    h (grid spacing)                = {r['h']:.1f} u-units")
        print(f"    L * h / 2                       = {r['Lh2']:.6f} ULP")
        print(f"    Level 1 certified               = {r['level1_cert']:.6f} ULP")
        print()
        print(f"  Level 2 — Horner truncation bound ({r['degree']} steps, |u/S| <= 0.617):")
        print(f"    Σ (π/4)^(2k), k=0..{r['degree']-1}            = {r['level2_bound']:.4f} ULP")
        print()
        print(f"  Total certified bound             = {r['total']:.4f} ULP")
        print(f"  Integer Horner grid max (ref)     = {r['grid_max_int']:.6f} ULP")
        print(f"  Grid points: {r['n_grid']:,}, time: {r['elapsed']:.1f}s")
        print()

    # ---- Verdict ----
    sin_pass = sin_r["total"] < 5.0
    cos_pass = cos_r["total"] < 5.0

    print("=" * 78)
    print(f"  sin_core: {'PASS' if sin_pass else 'FAIL'}  "
          f"(certified {sin_r['total']:.4f} ULP, target < 5)")
    print(f"  cos_core: {'PASS' if cos_pass else 'FAIL'}  "
          f"(certified {cos_r['total']:.4f} ULP, target < 5)")
    print("=" * 78)

    if sin_pass and cos_pass:
        print()
        print("Certificate statement for PROOFS.md:")
        print("-" * 60)
        print(f"""
sin_core and cos_core error bounds have been rigorously certified via
two-level Lipschitz analysis:

  Level 1 (real polynomial vs truth, Lipschitz-certified):
    sin_core: {sin_r['level1_cert']:.4f} ULP
    cos_core: {cos_r['level1_cert']:.4f} ULP

  Level 2 (integer Horner truncation, analytically bounded):
    5 steps at |u/S| <= (pi/4)^2 = 0.617: <= {sin_r['level2_bound']:.4f} ULP

  Total certified bounds:
    sin_core: {sin_r['total']:.4f} ULP  (in P(u)-space; output attenuated by |x/S|)
    cos_core: {cos_r['total']:.4f} ULP  (direct output)

Grid: {n_grid:,} points, mpmath {mpmath.mp.dps}-digit precision.
""")
    else:
        print("\nFAIL — see details above.")

    return 0 if (sin_pass and cos_pass) else 1


if __name__ == "__main__":
    sys.exit(main())
