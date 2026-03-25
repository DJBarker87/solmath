#!/usr/bin/env python3
"""
Regenerate adversarial vectors targeting current (post-rewrite) implementations.
Exactly 10K vectors per function, focused on actual error-prone regions.
Reference values from mpmath at 60-digit precision.
"""

import json, os, random
import mpmath

mpmath.mp.dps = 60
random.seed(42)

SCALE = 10**12
SCALE_HP = 10**15
U128_MAX = 2**128 - 1
I128_MAX = 2**127 - 1
I128_MIN = -(2**127)

OUTPUT_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'benchmark')

def save(filename, meta, vectors):
    path = os.path.join(OUTPUT_DIR, filename)
    meta["n"] = len(vectors)
    with open(path, 'w') as f:
        json.dump({"meta": meta, "vectors": vectors}, f)
    print(f"  {len(vectors):,} vectors -> {filename}")

def s(x):
    return str(int(x))

def nint(x):
    return int(mpmath.nint(x))

def pad_to(vecs, target, gen_one):
    """Keep generating until we have exactly `target` vectors."""
    while len(vecs) < target:
        v = gen_one()
        if v is not None:
            vecs.append(v)
    return vecs[:target]

# ============================================================
# LN — 10,000 vectors
# ============================================================

def gen_ln():
    print("ln_fixed_i...")
    vecs = []
    TABLE_STEP = SCALE // 16
    TABLE_HALF_STEP = SCALE // 32

    def ln_vec(x, cat):
        if x <= 0 or x > U128_MAX: return None
        ref = nint(mpmath.log(mpmath.mpf(x) / SCALE) * SCALE)
        if abs(ref) > I128_MAX: return None
        return {"x": s(x), "expected": s(ref), "category": cat}

    # Zone 1: near x=1.0 (2500)
    pad_to(vecs, 2500, lambda: ln_vec(int((1.0 + random.uniform(-0.001, 0.001)) * SCALE), "near_1"))

    # Zone 2: table boundaries (fixed ~204, deterministic)
    for j in range(17):
        boundary = SCALE + j * TABLE_STEP
        for offset in [0, 1, -1, 2, -2, 10, -10, 100, -100, TABLE_HALF_STEP - 1, TABLE_HALF_STEP, TABLE_HALF_STEP + 1]:
            v = ln_vec(boundary + offset, f"table_boundary_j{j}")
            if v: vecs.append(v)

    # Zone 3: direct/table seam (101, deterministic)
    for off in range(-50, 51):
        v = ln_vec(SCALE + TABLE_HALF_STEP + off, "direct_table_seam")
        if v: vecs.append(v)

    # Zone 4: powers of 2 (~490, deterministic)
    for k in range(-30, 40):
        base = int(mpmath.power(2, k) * SCALE)
        for off in [0, 1, -1, 10, -10, 1000, -1000]:
            v = ln_vec(base + off, f"pow2_k{k}")
            if v: vecs.append(v)

    # Zone 5: very small (fill to 6000)
    pad_to(vecs, 6000, lambda: ln_vec(int(10 ** random.uniform(-12, -3) * SCALE), "very_small"))

    # Zone 6: very large (fill to 8000)
    pad_to(vecs, 8000, lambda: ln_vec(int(10 ** random.uniform(3, 15) * SCALE), "very_large"))

    # Zone 7: dense sweep (fill to 10000)
    for i in range(10000 - len(vecs)):
        x = int((1.0 + i / (10000 - len(vecs) + 1)) * SCALE)
        v = ln_vec(x, "dense_sweep")
        if v: vecs.append(v)
    pad_to(vecs, 10000, lambda: ln_vec(int(random.uniform(0.5, 3.0) * SCALE), "fill"))

    save("adv_ln_vectors.json",
         {"suite": "adversarial", "function": "ln_fixed_i",
          "zones": "near_1, table_boundaries, direct_table_seam, pow2, very_small, very_large, dense_sweep"}, vecs)

# ============================================================
# EXP — 10,000 vectors
# ============================================================

def gen_exp():
    print("exp_fixed_i...")
    vecs = []
    LN2 = mpmath.log(2)

    def exp_vec(x, cat):
        ref = nint(mpmath.exp(mpmath.mpf(x) / SCALE) * SCALE)
        if ref > U128_MAX or ref <= 0: return None
        return {"x": s(x), "expected": s(ref), "category": cat}

    # Zone 1: ln2 multiples (~880, deterministic)
    for k in range(-40, 40):
        x_scaled = int(float(k * LN2) * SCALE)
        for off in [0, 1, -1, 2, -2, 5, -5, 50, -50, 500, -500]:
            v = exp_vec(x_scaled + off, f"ln2_k{k}")
            if v: vecs.append(v)

    # Zone 2: near zero (fill to 3500)
    pad_to(vecs, 3500, lambda: exp_vec(int(random.uniform(-0.001, 0.001) * SCALE), "near_zero"))

    # Zone 3: large negative (fill to 5500)
    pad_to(vecs, 5500, lambda: exp_vec(int(random.uniform(-25, -5) * SCALE), "large_negative"))

    # Zone 4: moderate positive (fill to 7500)
    pad_to(vecs, 7500, lambda: exp_vec(int(random.uniform(0.01, 2.0) * SCALE), "moderate_positive"))

    # Zone 5: near overflow (fill to 9000)
    pad_to(vecs, 9000, lambda: exp_vec(int(random.uniform(25.0, 38.5) * SCALE), "near_overflow"))

    # Zone 6: reduction midpoints (deterministic, ~120)
    for k in range(-20, 20):
        for frac in [-0.25, 0.0, 0.25]:
            v = exp_vec(int(float(k * LN2 + frac * LN2) * SCALE), "reduction_midpoint")
            if v: vecs.append(v)

    # Fill remainder
    pad_to(vecs, 10000, lambda: exp_vec(int(random.uniform(-20, 20) * SCALE), "fill"))

    save("adv_exp_vectors.json",
         {"suite": "adversarial", "function": "exp_fixed_i",
          "zones": "ln2_multiples, near_zero, large_negative, moderate_positive, near_overflow, reduction_midpoints"}, vecs)

# ============================================================
# NORM_CDF — 10,000 vectors
# ============================================================

def gen_norm_cdf():
    print("norm_cdf_poly...")
    vecs = []
    boundaries = [0.5, 1.5, 3.0, 5.0]

    def cdf_ref(x_scaled):
        x_mp = mpmath.mpf(x_scaled) / SCALE
        return nint((1 + mpmath.erf(x_mp / mpmath.sqrt(2))) / 2 * SCALE)

    def cdf_vec(x, cat):
        return {"x": s(x), "expected": s(cdf_ref(x)), "category": cat}

    # Zone 1: boundaries (~608, deterministic)
    for b in boundaries:
        b_scaled = int(b * SCALE)
        for off in [0, 1, -1, 2, -2, 5, -5, 10, -10, 50, -50, 100, -100, 500, -500, 1000, -1000, 5000, -5000]:
            for sign in [1, -1]:
                vecs.append(cdf_vec(sign * (b_scaled + off), f"boundary_{b}"))

    # Zone 2: piece interiors (fill to 7000)
    pieces = [(0, 0.5), (0.5, 1.5), (1.5, 3.0), (3.0, 5.0), (5.0, 8.5)]
    per_piece = (7000 - len(vecs)) // (len(pieces) * 2)
    for (lo, hi) in pieces:
        for _ in range(per_piece):
            x = int(random.uniform(lo, hi) * SCALE)
            vecs.append(cdf_vec(x, f"interior_{lo}_{hi}"))
            vecs.append(cdf_vec(-x, f"interior_neg_{lo}_{hi}"))

    # Zone 3: deep tails (fill to 9000)
    while len(vecs) < 9000:
        x = int(random.uniform(5.0, 8.5) * SCALE)
        vecs.append(cdf_vec(x, "deep_right_tail"))
        vecs.append(cdf_vec(-x, "deep_left_tail"))

    # Zone 4: zero + extreme tails
    vecs.append(cdf_vec(0, "zero"))
    for x_real in [8.0, 8.5, 9.0, 10.0, 20.0, 37.0]:
        x = int(x_real * SCALE)
        vecs.append(cdf_vec(x, "extreme_tail"))
        vecs.append(cdf_vec(-x, "extreme_tail_neg"))

    # Fill to exactly 10000
    while len(vecs) < 10000:
        x = int(random.uniform(-8.0, 8.0) * SCALE)
        vecs.append(cdf_vec(x, "fill"))
    vecs = vecs[:10000]

    save("adv_norm_cdf_vectors.json",
         {"suite": "adversarial", "function": "norm_cdf_poly",
          "boundaries": boundaries,
          "zones": "boundaries, piece_interiors, deep_tails, zero, extreme_tails"}, vecs)

# ============================================================
# POW — 10,000 vectors
# ============================================================

def gen_pow():
    print("pow_fixed...")
    vecs = []

    def pow_vec(base_lo, base_hi, exp_lo, exp_hi, cat):
        base = int(random.uniform(base_lo, base_hi) * SCALE)
        exp = int(random.uniform(exp_lo, exp_hi) * SCALE)
        if base <= 0: return None
        ref = nint(mpmath.power(mpmath.mpf(base) / SCALE, mpmath.mpf(exp) / SCALE) * SCALE)
        if ref > U128_MAX or ref <= 0: return None
        return {"base": s(base), "exp": s(exp), "expected": s(ref), "category": cat}

    # Zone 1: near_1 (3000)
    pad_to(vecs, 3000, lambda: pow_vec(0.998, 1.002, 5, 100, "near_1"))
    # Zone 2: small_base (fill to 5000)
    pad_to(vecs, 5000, lambda: pow_vec(0.05, 0.3, 2, 10, "small_base"))
    # Zone 3: fractional (fill to 8000)
    pad_to(vecs, 8000, lambda: pow_vec(0.5, 2.0, 0.1, 0.9, "fractional"))
    # Zone 4: large_base (fill to 10000)
    pad_to(vecs, 10000, lambda: pow_vec(10, 1000, 0.001, 0.1, "large_base"))

    save("adv_pow_fixed_vectors.json",
         {"suite": "adversarial", "function": "pow_fixed",
          "zones": "near_1, small_base, fractional, large_base"}, vecs)

# ============================================================
# SINCOS — 10,000 vectors
# ============================================================

def gen_sincos():
    print("sincos_fixed...")
    vecs = []
    PI = mpmath.pi

    def sc_vec(x, cat):
        if abs(x) > I128_MAX: return None
        x_mp = mpmath.mpf(x) / SCALE
        return {"x": s(x), "expected_sin": s(nint(mpmath.sin(x_mp) * SCALE)),
                "expected_cos": s(nint(mpmath.cos(x_mp) * SCALE)), "category": cat}

    # Zone 1: quadrant boundaries (~55, deterministic)
    for q_mult in [0, 0.5, 1, 1.5, 2]:
        q_scaled = int(float(q_mult * PI) * SCALE)
        for off in [0, 1, -1, 2, -2, 10, -10, 100, -100, 1000, -1000]:
            v = sc_vec(q_scaled + off, "quadrant_boundary")
            if v: vecs.append(v)

    # Zone 2: pi/4 seam (201, deterministic)
    piq_scaled = int(float(PI / 4) * SCALE)
    for off in range(-100, 101):
        v = sc_vec(piq_scaled + off, "pi_quarter_seam")
        if v: vecs.append(v)

    # Zone 3: large argument (fill to 5000)
    pad_to(vecs, 5000, lambda: sc_vec(int(random.uniform(100, 5000) * float(PI) * SCALE), "large_argument"))

    # Zone 4: near zero (fill to 7500)
    pad_to(vecs, 7500, lambda: sc_vec(int(random.uniform(-0.001, 0.001) * SCALE), "near_zero"))

    # Zone 5: negative (fill to 10000)
    pad_to(vecs, 10000, lambda: sc_vec(int(-random.uniform(0.1, 20.0) * SCALE), "negative"))

    save("adv_sincos_vectors.json",
         {"suite": "adversarial", "function": "sincos_fixed",
          "zones": "quadrant_boundaries, pi_quarter_seam, large_argument, near_zero, negative"}, vecs)

# ============================================================
# BS — 10,000 vectors
# ============================================================

def gen_bs():
    print("bs_full...")
    vecs = []

    def bs_ref(S, K, r, sigma, T):
        S_mp = mpmath.mpf(S) / SCALE
        K_mp = mpmath.mpf(K) / SCALE
        r_mp = mpmath.mpf(r) / SCALE
        sig_mp = mpmath.mpf(sigma) / SCALE
        T_mp = mpmath.mpf(T) / SCALE
        if sig_mp <= 0 or T_mp <= 0: return None
        sqrtT = mpmath.sqrt(T_mp)
        sig_sqrtT = sig_mp * sqrtT
        d1 = (mpmath.log(S_mp / K_mp) + (r_mp + sig_mp**2 / 2) * T_mp) / sig_sqrtT
        d2 = d1 - sig_sqrtT
        Nd1 = (1 + mpmath.erf(d1 / mpmath.sqrt(2))) / 2
        Nd2 = (1 + mpmath.erf(d2 / mpmath.sqrt(2))) / 2
        Nnd1 = (1 + mpmath.erf(-d1 / mpmath.sqrt(2))) / 2
        Nnd2 = (1 + mpmath.erf(-d2 / mpmath.sqrt(2))) / 2
        disc = mpmath.exp(-r_mp * T_mp)
        pdf_d1 = mpmath.exp(-d1**2 / 2) / mpmath.sqrt(2 * mpmath.pi)
        call = S_mp * Nd1 - K_mp * disc * Nd2
        put = call - S_mp + K_mp * disc
        delta_call = Nd1
        delta_put = Nd1 - 1
        gamma = pdf_d1 / (S_mp * sig_sqrtT)
        vega = S_mp * pdf_d1 * sqrtT
        theta_call = -(S_mp * pdf_d1 * sig_mp) / (2 * sqrtT) - r_mp * K_mp * disc * Nd2
        theta_put = -(S_mp * pdf_d1 * sig_mp) / (2 * sqrtT) + r_mp * K_mp * disc * Nnd2
        rho_call = K_mp * T_mp * disc * Nd2
        rho_put = -K_mp * T_mp * disc * Nnd2
        return {
            "call": max(0, nint(call * SCALE)), "put": max(0, nint(put * SCALE)),
            "call_delta": nint(delta_call * SCALE), "put_delta": nint(delta_put * SCALE),
            "gamma": nint(gamma * SCALE), "vega": nint(vega * SCALE),
            "call_theta": nint(theta_call * SCALE), "put_theta": nint(theta_put * SCALE),
            "call_rho": nint(rho_call * SCALE), "put_rho": nint(rho_put * SCALE),
        }

    def bs_vec(s_lo, s_hi, k_lo, k_hi, r_lo, r_hi, sig_lo, sig_hi, t_lo, t_hi, cat):
        S = int(random.uniform(s_lo, s_hi) * SCALE)
        K = int(random.uniform(k_lo, k_hi) * SCALE)
        r = int(random.uniform(r_lo, r_hi) * SCALE)
        sigma = int(random.uniform(sig_lo, sig_hi) * SCALE)
        T = int(random.uniform(t_lo, t_hi) * SCALE)
        g = bs_ref(S, K, r, sigma, T)
        if g is None: return None
        return {"s": s(S), "k": s(K), "r": s(r), "sigma": s(sigma), "t": s(T),
                "call": s(g["call"]), "put": s(g["put"]),
                "call_delta": s(g["call_delta"]), "put_delta": s(g["put_delta"]),
                "gamma": s(g["gamma"]), "vega": s(g["vega"]),
                "call_theta": s(g["call_theta"]), "put_theta": s(g["put_theta"]),
                "call_rho": s(g["call_rho"]), "put_rho": s(g["put_rho"]),
                "category": cat}

    # Zone 1: deep OTM (3000)
    pad_to(vecs, 3000, lambda: bs_vec(50, 200, 75, 500, 0, 0.1, 0.05, 0.5, 0.01, 2.0, "deep_otm"))
    # Zone 2: low vol (fill to 5500)
    pad_to(vecs, 5500, lambda: bs_vec(100, 100.001, 80, 120, 0.05, 0.05001, 0.005, 0.05, 0.01, 1.0, "low_vol"))
    # Zone 3: short maturity (fill to 8000)
    pad_to(vecs, 8000, lambda: bs_vec(100, 100.001, 95, 105, 0.05, 0.05001, 0.1, 0.5, 0.001, 0.01, "short_maturity"))
    # Zone 4: extreme vol (fill to 10000)
    pad_to(vecs, 10000, lambda: bs_vec(100, 100.001, 50, 200, 0, 0.1, 1.0, 3.0, 0.1, 2.0, "extreme_vol"))

    save("adv_bs_full_vectors.json",
         {"suite": "adversarial", "function": "bs_full",
          "zones": "deep_otm, low_vol, short_maturity, extreme_vol",
          "note": "mpmath 60 digits"}, vecs)

# ============================================================
# HP variants — 10,000 each
# ============================================================

def gen_exp_hp():
    print("exp_fixed_hp...")
    vecs = []
    LN2 = mpmath.log(2)

    def exp_hp_vec(x, cat):
        ref = nint(mpmath.exp(mpmath.mpf(x) / SCALE_HP) * SCALE_HP)
        if ref > U128_MAX or ref <= 0: return None
        return {"x": s(x), "expected": s(ref), "category": cat}

    # Zone 1: ln2 multiples (~560, deterministic)
    for k in range(-40, 40):
        x_scaled = int(float(k * LN2) * SCALE_HP)
        for off in [0, 1, -1, 5, -5, 50, -50]:
            v = exp_hp_vec(x_scaled + off, f"ln2_k{k}")
            if v: vecs.append(v)

    # Zone 2: near zero (fill to 3000)
    pad_to(vecs, 3000, lambda: exp_hp_vec(int(random.uniform(-0.001, 0.001) * SCALE_HP), "near_zero"))
    # Zone 3: large negative (fill to 6000)
    pad_to(vecs, 6000, lambda: exp_hp_vec(int(random.uniform(-25, -5) * SCALE_HP), "large_negative"))
    # Zone 4: moderate positive (fill to 8500)
    pad_to(vecs, 8500, lambda: exp_hp_vec(int(random.uniform(0.01, 2.0) * SCALE_HP), "moderate_positive"))
    # Zone 5: near overflow (fill to 10000)
    pad_to(vecs, 10000, lambda: exp_hp_vec(int(random.uniform(25.0, 38.5) * SCALE_HP), "near_overflow"))

    save("adv_exp_hp_vectors.json",
         {"suite": "adversarial", "function": "exp_fixed_hp", "scale": SCALE_HP,
          "zones": "ln2_multiples, near_zero, large_negative, moderate_positive, near_overflow"}, vecs)

def gen_pow_product_hp():
    print("pow_product_hp...")
    vecs = []

    def pp_vec(x_lo, x_hi, w_lo, w_hi, cat):
        x = int(random.uniform(x_lo, x_hi) * SCALE)
        w = int(random.uniform(w_lo, w_hi) * SCALE)
        if x <= 0: return None
        ref = nint(mpmath.power(mpmath.mpf(x) / SCALE, mpmath.mpf(w) / SCALE) * SCALE)
        if ref > U128_MAX or ref <= 0: return None
        return {"x": s(x), "w": s(w), "expected": s(ref), "category": cat}

    pad_to(vecs, 2500, lambda: pp_vec(0.5, 2.0, 0.001, 0.01, "extreme_low_w"))
    pad_to(vecs, 5000, lambda: pp_vec(0.5, 2.0, 0.99, 0.999, "extreme_high_w"))
    pad_to(vecs, 7500, lambda: pp_vec(10, 1000, 0.1, 0.9, "large_x"))
    pad_to(vecs, 10000, lambda: pp_vec(0.998, 1.002, 0.1, 0.9, "x_near_1"))

    save("adv_pow_product_hp_vectors.json",
         {"suite": "adversarial", "function": "pow_product_hp",
          "zones": "extreme_low_w, extreme_high_w, large_x, x_near_1"}, vecs)

# ============================================================

if __name__ == '__main__':
    print(f"SolMath Adversarial Vector Generator (post-rewrite)")
    print(f"SCALE={SCALE}, mpmath precision={mpmath.mp.dps} digits\n")

    gen_ln()
    gen_exp()
    gen_norm_cdf()
    gen_pow()
    gen_sincos()
    gen_bs()
    gen_exp_hp()
    gen_pow_product_hp()

    print("\nDone.")
