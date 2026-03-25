#!/usr/bin/env python3
"""
Fit Remez polynomial for exp via the rational reconstruction formula.

From FreeBSD msun/e_exp.c:
  c(r) = r - (P1*r^2 + P2*r^4 + ... + P5*r^10)
  exp(r) = 1 + r + r*c/(2-c)

where r in [-ln(2)/2, ln(2)/2] ~ [-0.3466, 0.3466]

The polynomial P approximates: (exp(r)+1)/(exp(r)-1)/r - 2/r^2
rewritten as R(z) where z = r^2:
  R(z) = P1*z + P2*z^2 + P3*z^3 + P4*z^4 + P5*z^5

The target function: f(z) = (2*atanh(sqrt(z)/2) - sqrt(z)) / z^(3/2)
...actually it's simpler. The polynomial P(z) is defined by:
  c(r) = r - P(r^2) * r^2
  exp(r) = 1 + 2*r / (R(r) - r)
  where R(r) = 2 + r^2/6 - r^4/360 + ...

The coefficients are for:
  R(z) ~ 2 + P1*z + P2*z^2 + P3*z^3 + P4*z^4 + P5*z^5

where R(z) = r*(exp(r)+1)/(exp(r)-1) and z = r^2.

Let me just directly compute the Remez coefficients for the c(r) formulation.
Actually, it's easier to just scale the brine-fp coefficients from 1e18 to 1e12.
The coefficients approximate the SAME mathematical function — only the scale changes.
"""

from mpmath import mp, mpf, exp, sqrt, fabs, log
mp.dps = 60

# The brine-fp coefficients at 1e18 scale:
P_1e18 = [
    166666666666666019,   # P1 (positive) ~ 1/6
    -2777777777701559,    # P2 (negative) ~ -1/360
    66137563214379,       # P3 (positive)
    -1653390220546,       # P4 (negative)
    41381367970,          # P5 (positive)
]

# These are P(z) coefficients where z = r^2, and the formula is:
#   c = r - (P1*z + P2*z^2 + P3*z^3 + P4*z^4 + P5*z^5)  [all at their respective scales]
#   exp(r) = 1 + (r*c/(2-c) - lo + hi)
#
# The real (unscaled) coefficients:
P_real = [p / 1e18 for p in P_1e18]
print("Brine-fp coefficients (unscaled):")
for i, p in enumerate(P_real):
    print(f"  P{i+1} = {p:.18e}")
print()

# Scale to 1e12:
SCALE_12 = mpf(10)**12
SCALE_15 = mpf(10)**15

print("=== At SCALE = 1e12 ===")
for i, p in enumerate(P_real):
    scaled = mpf(p) * SCALE_12
    rounded = int(round(float(scaled)))
    err = float(fabs(scaled - rounded))
    sign = "" if rounded >= 0 else "-"
    print(f"  pub const EXP_REMEZ_P{i+1}: i128 = {rounded:>20d}; // qerr {err:.4f}")
print()

print("=== At SCALE = 1e15 ===")
for i, p in enumerate(P_real):
    scaled = mpf(p) * SCALE_15
    rounded = int(round(float(scaled)))
    err = float(fabs(scaled - rounded))
    print(f"  pub const EXP_REMEZ_HP_P{i+1}: i128 = {rounded:>20d}; // qerr {err:.4f}")
print()

# Verify: compute exp(r) for a few test values using these coefficients
print("=== Verification ===")
r_max = float(log(2)) / 2

for r_val in [0.0, 0.01, 0.1, 0.3, 0.34, -0.1, -0.34]:
    r = mpf(r_val)
    z = r * r

    # Using brine-fp formula:
    # c = r - (P1*z + P2*z^2 + P3*z^3 + P4*z^4 + P5*z^5)
    poly = sum(mpf(P_real[i]) * z**(i+1) for i in range(5))
    c = r - poly

    if c == 2:
        exp_approx = mpf(1)  # avoid division by zero at r=0
    else:
        exp_approx = 1 + r * c / (2 - c)

    exp_true = exp(r)
    err = float(fabs(exp_approx - exp_true))
    err_ulp_12 = err * float(SCALE_12)
    err_ulp_15 = err * float(SCALE_15)

    print(f"  r={r_val:+.3f}: true={float(exp_true):.15f} approx={float(exp_approx):.15f} "
          f"err_1e12={err_ulp_12:.4f} err_1e15={err_ulp_15:.4f}")

# Count operations:
# Current Taylor: 12 iterations × (fp_mul_i_round + divide_by_n) = 24 rounding ops
# Remez rational: compute z (1 mul), Horner for P (4 muls), c = r - P*z (1 mul),
#                 r*c (1 mul), 2-c (sub), r*c/(2-c) (1 div), final add = ~8 ops + 1 div
print()
print("Operation count:")
print("  Current Taylor: ~24 rounding operations (12 × mul + 12 × div)")
print("  Remez rational: ~8 rounding operations (7 mul + 1 div)")
