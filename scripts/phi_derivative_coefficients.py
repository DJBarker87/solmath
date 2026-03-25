#!/usr/bin/env python3
"""
Compute derivative coefficients for the phi piecewise minimax polynomial.

The phi polynomial evaluates Φ(x) = c₀ + c₁t + c₂t² + ... + c₁₁t¹¹
where t = (|x| - mid) / hw.

The derivative dΦ/dx = φ(x) = poly'(t) / hw
where poly'(t) = c₁ + 2c₂t + 3c₃t² + ... + 11c₁₁t¹⁰

So the derivative coefficients are: d[k] = (k+1) × c[k+1]  for k = 0..10.

This script:
1. Computes derivative coefficients for all 5 pieces
2. Verifies against analytical φ(x) = exp(-x²/2)/√(2π) using mpmath
3. Generates Rust constants
"""

import mpmath
import numpy as np
import json
import os
import sys

mpmath.mp.dps = 50
SCALE = 10**12


def trunc_div(a, b):
    if b == 0: raise ZeroDivisionError
    if (a >= 0 and b > 0) or (a < 0 and b < 0):
        return abs(a) // abs(b)
    else:
        return -(abs(a) // abs(b))


def fp_mul_i(a, b):
    return trunc_div(a * b, SCALE)


# The 5 phi polynomial pieces (from the Rust source)
PIECES = [
    {
        "x_low": 0.0, "x_high": 0.5, "mid": 0.25, "hw": 0.25,
        "coeffs_i128": [598706325683, 96667029201, -3020844663, -944013957, 46217350,
                        8272414, -471315, -57343, 3604, 323, -22, -2]
    },
    {
        "x_low": 0.5, "x_high": 1.5, "mid": 1.0, "hw": 0.5,
        "coeffs_i128": [841344746069, 120985362260, -30246340569, -5, 1260264234,
                        -126026382, -31506790, 6001127, 469208, -171688, -2149, 3416]
    },
    {
        "x_low": 1.5, "x_high": 3.0, "mid": 2.25, "hw": 0.75,
        "coeffs_i128": [987775527335, 23804738880, -20085247720, 9066257895, -1941843776,
                        -109595589, 176496701, -35201720, -3277957, 2527460, -204238, -75313]
    },
    {
        "x_low": 3.0, "x_high": 5.0, "mid": 4.0, "hw": 1.0,
        "coeffs_i128": [999968328759, 133830187, -267660506, 334576629, -289965085,
                        181777815, -82529050, 25545312, -3919688, -783934, 653222, -140313]
    },
    {
        "x_low": 5.0, "x_high": 8.0, "mid": 6.5, "hw": 1.5,
        "coeffs_i128": [999999999968, 375, -2558, 6884, -7459,
                        20651, -66379, 62892, 5917, 13646, -72811, 38883]
    },
]


def compute_derivative_coeffs(piece):
    """Derivative of poly(t) = c0 + c1*t + ... + c11*t^11 is:
    poly'(t) = c1 + 2*c2*t + 3*c3*t^2 + ... + 11*c11*t^10
    So deriv_coeffs[k] = (k+1) * original_coeffs[k+1] for k=0..10.
    """
    c = piece["coeffs_i128"]
    return [(k + 1) * c[k + 1] for k in range(11)]


def eval_deriv_i128(piece, x_scaled):
    """Evaluate dΦ/dx = poly'(t) / hw at a scaled x value."""
    mid_s = int(round(piece["mid"] * SCALE))
    hw_s = int(round(piece["hw"] * SCALE))
    deriv = piece["deriv_i128"]

    # Map to t
    t_s = trunc_div((x_scaled - mid_s) * SCALE, hw_s)

    # Horner for degree 10
    r = deriv[10]
    for i in range(9, -1, -1):
        r = fp_mul_i(r, t_s) + deriv[i]

    # Divide by hw (in SCALE) to get d/dx from d/dt
    return trunc_div(r * SCALE, hw_s)


def eval_phi_pdf_i128(x_float):
    """Evaluate the polynomial-derived PDF at a float x, returning float."""
    x_scaled = int(round(abs(x_float) * SCALE))
    if x_scaled >= 8 * SCALE:
        return 0.0

    for piece in PIECES:
        hi = int(round(piece["x_high"] * SCALE))
        if x_scaled <= hi:
            result = eval_deriv_i128(piece, x_scaled)
            return result / SCALE
    return 0.0


def main():
    print("=" * 70)
    print("  PHI DERIVATIVE COEFFICIENT GENERATION")
    print("=" * 70)

    # Step 1: Compute derivative coefficients
    for p in PIECES:
        p["deriv_i128"] = compute_derivative_coeffs(p)
        print(f"\nPiece [{p['x_low']}, {p['x_high']}]:")
        print(f"  Derivative coeffs: {p['deriv_i128']}")

    # Step 2: Verify against analytical PDF
    print(f"\n{'='*70}")
    print(f"  TEST A: Derivative vs analytical PDF")
    print(f"{'='*70}")

    n_points = 10000
    xs = np.linspace(0, 7.99, n_points)
    max_err = 0.0
    max_rel_err = 0.0
    worst_x = 0.0

    for x in xs:
        analytical = float(mpmath.npdf(x))
        poly_pdf = eval_phi_pdf_i128(x)

        err = abs(poly_pdf - analytical)
        if analytical > 1e-12:
            rel = err / analytical
            if rel > max_rel_err:
                max_rel_err = rel
        if err > max_err:
            max_err = err
            worst_x = x

    print(f"  Max absolute error: {max_err:.4e} (at x={worst_x:.4f})")
    print(f"  Max relative error: {max_rel_err:.4e}")
    pass_a = max_err < 5e-11
    print(f"  {'PASS' if pass_a else 'FAIL'} (target: max abs < 5e-11 for |x| < 5)")

    # Check specifically for |x| < 5
    xs5 = np.linspace(0, 4.99, 5000)
    max_err5 = 0.0
    for x in xs5:
        analytical = float(mpmath.npdf(x))
        poly_pdf = eval_phi_pdf_i128(x)
        err = abs(poly_pdf - analytical)
        if err > max_err5:
            max_err5 = err
    print(f"  Max abs error |x|<5: {max_err5:.4e}")

    # Step 3: Known values
    print(f"\n{'='*70}")
    print(f"  TEST D: PDF properties")
    print(f"{'='*70}")

    pdf_0 = eval_phi_pdf_i128(0.0)
    analytical_0 = 1.0 / float(mpmath.sqrt(2 * mpmath.pi))
    print(f"  φ(0): poly={pdf_0:.12f}  analytical={analytical_0:.12f}  err={abs(pdf_0-analytical_0):.4e}")

    # Monotone decrease for x > 0
    prev = eval_phi_pdf_i128(0.0)
    mono_violations = 0
    for x in np.linspace(0.01, 7.99, 5000):
        cur = eval_phi_pdf_i128(x)
        if cur > prev + 1e-12:
            mono_violations += 1
        prev = cur
    print(f"  Monotonicity violations (x>0): {mono_violations}")

    # Non-negativity
    neg_count = 0
    for x in np.linspace(0, 7.99, 10000):
        if eval_phi_pdf_i128(x) < -1e-15:
            neg_count += 1
    print(f"  Negative PDF values: {neg_count}")

    # Step 4: Boundary continuity
    print(f"\n{'='*70}")
    print(f"  TEST C: Boundary continuity of derivative")
    print(f"{'='*70}")

    for i in range(len(PIECES) - 1):
        boundary = PIECES[i]["x_high"]
        left = eval_deriv_i128(PIECES[i], int(round(boundary * SCALE)))
        right = eval_deriv_i128(PIECES[i + 1], int(round(boundary * SCALE)))
        # Convert both to d/dx by dividing by respective hw
        hw_l = int(round(PIECES[i]["hw"] * SCALE))
        hw_r = int(round(PIECES[i + 1]["hw"] * SCALE))
        pdf_l = trunc_div(left * SCALE, hw_l)
        pdf_r = trunc_div(right * SCALE, hw_r)
        diff = abs(pdf_l - pdf_r)
        print(f"  x={boundary}: left={pdf_l} right={pdf_r} diff={diff}")

    # Generate Rust constants
    print(f"\n{'='*70}")
    print(f"  RUST CONSTANTS")
    print(f"{'='*70}")

    for i, p in enumerate(PIECES):
        d = p["deriv_i128"]
        print(f"const POLY_DERIV_I{i}: [i128; 11] = [")
        for j, c in enumerate(d):
            print(f"    {c:>25},  // {j+1} * c[{j+1}]")
        print(f"];")
        print()

    # Save
    out_dir = os.path.join(os.path.dirname(__file__), '..', 'outputs')
    os.makedirs(out_dir, exist_ok=True)

    all_pass = pass_a and mono_violations == 0 and neg_count == 0

    report = f"PHI DERIVATIVE VERIFICATION\n{'='*60}\n"
    report += f"Max abs error: {max_err:.4e}\n"
    report += f"Max abs error |x|<5: {max_err5:.4e}\n"
    report += f"Max rel error: {max_rel_err:.4e}\n"
    report += f"φ(0) error: {abs(pdf_0-analytical_0):.4e}\n"
    report += f"Monotonicity violations: {mono_violations}\n"
    report += f"Negative values: {neg_count}\n"
    report += f"\nOVERALL: {'PASS' if all_pass else 'FAIL'}\n"

    with open(os.path.join(out_dir, 'phi_derivative_verification.txt'), 'w') as f:
        f.write(report)
    print(f"\n  Report saved. OVERALL: {'PASS' if all_pass else 'FAIL'}")

    return all_pass


if __name__ == '__main__':
    success = main()
    sys.exit(0 if success else 1)
