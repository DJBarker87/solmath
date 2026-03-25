#!/usr/bin/env python3
"""
Fit a Remez minimax polynomial for ln via the arctanh substitution.

We approximate: arctanh(t)/t = 1 + t^2/3 + t^4/5 + ...
on t in [0, t_max] where t_max = (sqrt(2) - 1) / (sqrt(2) + 1) ~ 0.17157

Equivalently, fit P(u) on u in [0, u_max] where u = t^2, u_max ~ 0.02944
such that: 2*t*P(t^2) ~ ln((1+t)/(1-t))

The target function is: f(u) = arctanh(sqrt(u)) / sqrt(u)
                              = 1 + u/3 + u^2/5 + u^3/7 + ...
"""

from mpmath import mp, mpf, log, sqrt, atanh, fabs, cos, pi, matrix, lu_solve
import sys

mp.dps = 60

SCALE = mpf(10)**12
t_max = (sqrt(2) - 1) / (sqrt(2) + 1)
u_max = t_max**2

print(f"t_max = {float(t_max):.15f}")
print(f"u_max = {float(u_max):.15f}")
print()

def target(u):
    """arctanh(sqrt(u)) / sqrt(u) for u > 0, limit 1 at u=0"""
    if u == 0:
        return mpf(1)
    su = sqrt(u)
    return atanh(su) / su

# ============================================================
# Chebyshev-node interpolation (near-minimax)
# ============================================================

print("=" * 70)
print("Chebyshev-node interpolation for P(u) = arctanh(sqrt(u))/sqrt(u)")
print("Domain: u in [0, {:.10f}]".format(float(u_max)))
print("=" * 70)
print()

best_degree = None
best_coeffs = None
best_max_err = None

for degree in range(3, 10):
    n = degree + 1
    # Chebyshev nodes on [0, u_max]
    nodes = [(u_max/2) * (1 - cos(pi * (2*k + 1) / (2*n))) for k in range(n)]

    # Vandermonde system
    A = matrix(n, n)
    b_vec = matrix(n, 1)
    for i in range(n):
        for j in range(n):
            A[i, j] = nodes[i]**j
        b_vec[i] = target(nodes[i])

    coeffs = lu_solve(A, b_vec)
    coeffs = [coeffs[i] for i in range(n)]

    # Dense error evaluation
    max_err = mpf(0)
    worst_u = mpf(0)
    for i in range(200001):
        u = u_max * mpf(i) / 200000
        p_val = sum(coeffs[j] * u**j for j in range(n))
        true_val = target(u)
        err = fabs(p_val - true_val)
        if err > max_err:
            max_err = err
            worst_u = u

    # Output error: result = 2*t*P(u)*SCALE, so error = 2*t*err*SCALE
    # Worst case at t = t_max
    max_output_ulp = 2 * float(t_max) * float(max_err) * float(SCALE)

    print(f"Degree {degree}: max |P(u) - f(u)| = {float(max_err):.6e}")
    print(f"  max output error = {max_output_ulp:.4f} ULP (at t=t_max)")
    print(f"  worst at u = {float(worst_u):.10f}")

    # Print scaled coefficients
    print(f"  Coefficients at SCALE=1e12:")
    for j, c in enumerate(coeffs):
        c_scaled = c * SCALE
        c_rounded = int(round(float(c_scaled)))
        c_err = float(fabs(c_scaled - c_rounded))
        print(f"    c{j} = {c_rounded:>25d}  (quantization err: {c_err:.6f})")

    # Count operations: degree fp_mul_i in Horner + 1 final fp_mul_i(t, p)
    total_muls = degree + 1
    print(f"  Horner fp_mul_i calls: {degree} + 1 final = {total_muls} total")
    print()

    if max_output_ulp < 0.5 and (best_degree is None or degree < best_degree):
        best_degree = degree
        best_coeffs = coeffs
        best_max_err = max_output_ulp

print("=" * 70)
if best_degree is not None:
    print(f"RECOMMENDED: degree {best_degree} (approx error = {best_max_err:.4f} ULP)")
    print(f"  Total fp_mul_i calls: {best_degree + 1}")
    print(f"  (Current Mercator: ~16 fp_mul_i + ~8 rounding divides)")
    print()
    print("  Rust constants:")
    print()
    for j, c in enumerate(best_coeffs):
        c_scaled = c * SCALE
        c_rounded = int(round(float(c_scaled)))
        sign = "" if c_rounded >= 0 else "-"
        abs_val = abs(c_rounded)
        # Format with underscores for readability
        formatted = f"{abs_val:_d}".replace("_", "_")
        print(f"    pub const LN_REMEZ_C{j}: i128 = {sign}{formatted};")
    print()
    print("  Horner evaluation (unrolled, highest degree first):")
    print(f"    let u = t2; // = t^2")
    print(f"    let mut p = LN_REMEZ_C{best_degree};")
    for j in range(best_degree - 1, -1, -1):
        print(f"    p = fp_mul_i(p, u) + LN_REMEZ_C{j};")
    print(f"    let series_result = 2 * fp_mul_i(t, p);")
else:
    print("NO degree <= 9 achieved < 0.5 ULP approximation error.")
    print("Try a true Remez exchange algorithm.")
