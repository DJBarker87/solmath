#!/usr/bin/env python3
"""
Fit Remez polynomial for ln_fixed_hp at SCALE_HP = 1e15.
Wide domain (no sqrt2 reduction): t in [0, 1/3], u = t^2 in [0, 1/9].
"""

from mpmath import mp, mpf, sqrt, atanh, fabs, cos, pi, matrix, lu_solve
mp.dps = 60

SCALE = mpf(10)**15  # HP scale

t_max = mpf(1) / 3
u_max = t_max**2

print(f"SCALE_HP = 1e15")
print(f"t_max = {float(t_max):.15f}")
print(f"u_max = {float(u_max):.15f}")
print()

def target(u):
    if u == 0:
        return mpf(1)
    su = sqrt(u)
    return atanh(su) / su

for degree in range(6, 12):
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
    for i in range(200001):
        u = u_max * mpf(i) / 200000
        p_val = sum(coeffs[j] * u**j for j in range(n))
        true_val = target(u)
        err = fabs(p_val - true_val)
        if err > max_err:
            max_err = err

    max_output_ulp = 2 * float(t_max) * float(max_err) * float(SCALE)
    total_muls = degree + 1

    marker = "  *** PASSES ***" if max_output_ulp < 0.5 else ""
    print(f"Degree {degree}: approx error = {max_output_ulp:.4f} ULP, {total_muls} fp_mul_i{marker}")

    if max_output_ulp < 0.5:
        print(f"  Coefficients at SCALE_HP=1e15:")
        for j, c in enumerate(coeffs):
            c_scaled = c * SCALE
            c_rounded = int(round(float(c_scaled)))
            c_err = float(fabs(c_scaled - c_rounded))
            print(f"    pub const LN_REMEZ_HP{j}: i128 = {abs(c_rounded):_d}; // qerr {c_err:.4f}")
        print()
