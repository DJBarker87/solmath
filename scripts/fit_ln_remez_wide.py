#!/usr/bin/env python3
"""
Fit Remez polynomial for ln WITHOUT sqrt(2) secondary reduction.
Domain: m in [SCALE, 2*SCALE), so t = (m-1)/(m+1) in [0, 1/3].
u = t^2 in [0, 1/9].
"""

from mpmath import mp, mpf, sqrt, atanh, fabs, cos, pi, matrix, lu_solve
mp.dps = 60

SCALE = mpf(10)**12

# Without sqrt(2) reduction: m in [SCALE, 2*SCALE)
# t = (m/SCALE - 1) / (m/SCALE + 1) in [0, 1/3]
t_max = mpf(1) / 3
u_max = t_max**2  # 1/9

print(f"t_max = {float(t_max):.15f}")
print(f"u_max = {float(u_max):.15f}")
print()

def target(u):
    if u == 0:
        return mpf(1)
    su = sqrt(u)
    return atanh(su) / su

print("=" * 70)
print("Wide domain: no sqrt(2) reduction, u in [0, 1/9]")
print("=" * 70)
print()

best_degree = None
best_coeffs = None
best_max_err = None

for degree in range(4, 12):
    n = degree + 1
    nodes = [(u_max/2) * (1 - cos(pi * (2*k + 1) / (2*n))) for k in range(n)]

    A = matrix(n, n)
    b_vec = matrix(n, 1)
    for i in range(n):
        for j in range(n):
            A[i, j] = nodes[i]**j
        b_vec[i] = target(nodes[i])

    coeffs = lu_solve(A, b_vec)
    coeffs = [coeffs[i] for i in range(n)]

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

    max_output_ulp = 2 * float(t_max) * float(max_err) * float(SCALE)
    total_muls = degree + 1

    print(f"Degree {degree}: max |P(u) - f(u)| = {float(max_err):.6e}")
    print(f"  max output error = {max_output_ulp:.4f} ULP (at t=1/3)")
    print(f"  Horner fp_mul_i calls: {degree} + 1 final = {total_muls} total")

    if max_output_ulp < 0.5:
        print(f"  *** PASSES < 0.5 ULP threshold ***")

    print(f"  Coefficients at SCALE=1e12:")
    for j, c in enumerate(coeffs):
        c_scaled = c * SCALE
        c_rounded = int(round(float(c_scaled)))
        c_err = float(fabs(c_scaled - c_rounded))
        print(f"    c{j} = {c_rounded:>25d}  (qerr: {c_err:.6f})")
    print()

    if max_output_ulp < 0.5 and (best_degree is None or degree < best_degree):
        best_degree = degree
        best_coeffs = coeffs
        best_max_err = max_output_ulp

print("=" * 70)
if best_degree is not None:
    print(f"RECOMMENDED: degree {best_degree} (approx error = {best_max_err:.4f} ULP)")
    print(f"  Total fp_mul_i calls: {best_degree + 1}")
    print()
    print("  Rust constants:")
    for j, c in enumerate(best_coeffs):
        c_scaled = c * SCALE
        c_rounded = int(round(float(c_scaled)))
        print(f"    pub const LN_REMEZ_W{j}: i128 = {abs(c_rounded):_d};")
    print()
    print("  Horner (unrolled):")
    print(f"    let u = fp_mul_i_round(t, t);")
    print(f"    let p = LN_REMEZ_W{best_degree};")
    for j in range(best_degree - 1, -1, -1):
        print(f"    let p = fp_mul_i_round(p, u) + LN_REMEZ_W{j};")
    print(f"    let series_result = 2 * fp_mul_i_round(t, p);")
    print(f"    // reconstruction: series_result + k * LN2_I  (no half_ln2_adj!)")
