#!/usr/bin/env python3
"""
Regenerate adversarial vectors targeting current (post-rewrite) implementations.
Exactly 10K vectors per function, focused on actual error-prone regions.
Reference values from mpmath at 60-digit precision.
"""

import json, os, random, sys
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
    lut_step = SCALE // 1024

    def ln_vec(x, cat):
        if x <= 0 or x > U128_MAX: return None
        ref = nint(mpmath.log(mpmath.mpf(x) / SCALE) * SCALE)
        if abs(ref) > I128_MAX: return None
        return {"x": s(x), "expected": s(ref), "category": cat}

    # Every raw increment around one plus both sides of the correctly-rounded
    # direct-return seam at |x-1| = 1e-6.
    for delta in range(-1_000, 1_001):
        vecs.append(ln_vec(SCALE + delta, "near_one_raw"))
    for sign in (-1, 1):
        for offset in range(-128, 129):
            vecs.append(ln_vec(SCALE + sign * (1_000_000 + offset), "near_one_seam"))

    # Cross every normalized midpoint-table boundary at k=0.
    for j in range(1025):
        boundary = SCALE + j * lut_step
        side = -1 if j % 2 == 0 else 1
        for x in (boundary, boundary + side):
            v = ln_vec(x, "lut_table_boundary")
            if v is not None:
                vecs.append(v)

    # Exact powers of two and neighboring raw values across the public range.
    for k in range(-40, 89):
        base = SCALE << k if k >= 0 else SCALE >> -k
        for offset in (0, -1, 1, -2, 2):
            v = ln_vec(base + offset, "power_of_two")
            if v is not None:
                vecs.append(v)

    # Mantissa interiors across every reachable exponent, including k=88 for
    # the upper half of the u128 domain.
    offsets = (
        -488_281_249, -366_210_937, -244_140_625, -122_070_312,
        -1, 0, 1, 122_070_312, 244_140_625, 366_210_937, 488_281_249,
    )
    for k in range(-40, 89):
        for lane in range(36):
            j = (73 * k + 29 * lane) % 1024
            midpoint = SCALE + j * lut_step + lut_step // 2
            mantissa = midpoint + offsets[lane % len(offsets)]
            x = mantissa << k if k >= 0 else mantissa >> -k
            v = ln_vec(x, "reduction_k_sweep")
            if v is not None:
                vecs.append(v)

    for x in (1, 2, U128_MAX - 1, U128_MAX):
        vecs.append(ln_vec(x, "domain_extreme"))

    pad_to(vecs, 10000, lambda: ln_vec(
        random.randint(1, U128_MAX), "full_width_fill"))
    vecs = [v for v in vecs if v is not None][:10000]

    save("adv_ln_vectors.json",
         {"suite": "adversarial", "function": "ln_fixed_i",
          "zones": "near-one raw/seam, every LUT boundary, powers of two, all reachable exponents, u128 extremes, full-width fill",
          "reference": "mpmath.log at 60 decimal digits"}, vecs)


# ============================================================
# LN_1P — 10,000 vectors
# ============================================================

def gen_ln_1p():
    print("ln_1p_fixed...")
    vecs = []

    def ln_1p_vec(x, cat):
        if x <= -SCALE or x > I128_MAX:
            return None
        ref = nint(mpmath.log1p(mpmath.mpf(x) / SCALE) * SCALE)
        return {"x": s(x), "expected": s(ref), "category": cat}

    # Domain edge: make 1+x range from one raw unit to 1e-2.
    for one_plus_x in range(1, 257):
        vecs.append(ln_1p_vec(-SCALE + one_plus_x, "domain_edge_raw"))
    pad_to(vecs, 768, lambda: ln_1p_vec(
        -SCALE + max(1, int(10 ** random.uniform(0, 10))), "domain_edge_log"))

    # Every raw increment in the central proof interval sample.
    for x in range(-1_000, 1_001):
        vecs.append(ln_1p_vec(x, "near_zero_raw"))

    # Exercise both sides of the direct-return seam at |x| = 1e-6.
    for sign in (-1, 1):
        for offset in range(-128, 129):
            vecs.append(ln_1p_vec(sign * (1_000_000 + offset), "near_zero_seam"))

    # Cross every table boundary at k=0. For each boundary include the exact
    # value and alternate one adjacent raw unit.
    LUT_STEP = SCALE // 1024
    for j in range(1025):
        side = -1 if j % 2 == 0 else 1
        normalized = SCALE + j * LUT_STEP
        for one_plus_x in (normalized, normalized + side):
            vecs.append(ln_1p_vec(one_plus_x - SCALE, "lut_table_boundary"))

    # Permanent regressions discovered by the production corpus. These are
    # reduction/rounding combinations that attained its current 2-ULP maximum.
    for x in (
        -999_999_993_148,
        -997_601_045_585,
        4_477_046_948_564_625,
        1_252_655_015_786_461,
        4_255_737_139_032_047,
        1_211_973_668_939_694,
        5_811_452_000_770_234,
        2_065_446_030_829_052,
    ):
        vecs.append(ln_1p_vec(x, "production_worst_regression"))

    # Exercise mantissa interiors across every reachable binary exponent.
    # This targets the interaction between local rounding and k*ln(2), which
    # the former two-octave corpus missed.
    offsets = (
        -488_281_249, -366_210_937, -244_140_625, -122_070_312,
        -1, 0, 1, 122_070_312, 244_140_625, 366_210_937, 488_281_249,
    )
    for k in range(-40, 88):
        for lane in range(36):
            j = (73 * k + 29 * lane) % 1024
            midpoint = SCALE + j * LUT_STEP + LUT_STEP // 2
            mantissa = midpoint + offsets[lane % len(offsets)]
            one_plus_x = mantissa << k if k >= 0 else mantissa >> -k
            v = ln_1p_vec(one_plus_x - SCALE, "reduction_k_sweep")
            if v is not None:
                vecs.append(v)

    pad_to(vecs, 10000, lambda: ln_1p_vec(
        random.randint(-900_000_000_000, 10 * SCALE), "mixed_fill"))

    vecs = [v for v in vecs if v is not None][:10000]
    save("adv_ln_1p_vectors.json",
         {"suite": "adversarial", "function": "ln_1p_fixed",
          "zones": "domain edge, near-zero raw units, fast-path seams, every LUT boundary, all reachable binary exponents, production-worst regressions, signed fill",
          "reference": "mpmath.log1p at 60 decimal digits"}, vecs)


# ============================================================
# EXPM1 — 10,000 vectors
# ============================================================

def gen_expm1():
    print("expm1_fixed...")
    vecs = []
    def expm1_vec(x, cat):
        if x >= 40 * SCALE or x < I128_MIN:
            return None
        ref = nint(mpmath.expm1(mpmath.mpf(x) / SCALE) * SCALE)
        if ref < I128_MIN or ref > I128_MAX:
            return None
        return {"x": s(x), "expected": s(ref), "category": cat}

    # Exact raw-unit neighborhood and both sides of the direct-return seam.
    for x in range(-1_000, 1_001):
        vecs.append(expm1_vec(x, "near_zero_raw"))
    for sign in (-1, 1):
        for offset in range(-128, 129):
            vecs.append(expm1_vec(sign * (1_000_000 + offset), "near_zero_seam"))

    # Half-ln2 reduction seams for every reachable exponent.
    for k in range(-58, 59):
        center = nint((mpmath.mpf(k) + mpmath.mpf("0.5")) * mpmath.log(2) * SCALE)
        for offset in (-2, -1, 0, 1, 2):
            v = expm1_vec(center + offset, "ln2_reduction_seam")
            if v is not None:
                vecs.append(v)

    # Every power-of-two LUT boundary at k=0, exact and one adjacent raw unit.
    r_min = -346_573_590_280
    step = 1 << 29
    half_ln2 = 346_573_590_280
    for j in range(1293):
        boundary = r_min + j * step
        if boundary > half_ln2:
            break
        side = -1 if j % 2 == 0 else 1
        vecs.append(expm1_vec(boundary, "lut_boundary"))
        vecs.append(expm1_vec(boundary + side, "lut_boundary"))

    # Mantissa interiors across the complete k range.
    offsets = (-268_435_455, -201_326_592, -134_217_728, -67_108_864,
               -1, 0, 1, 67_108_864, 134_217_728, 201_326_592, 268_435_455)
    for k in range(-58, 59):
        k_ln2 = nint(mpmath.mpf(k) * mpmath.log(2) * SCALE)
        for lane in range(32):
            j = (71 * k + 31 * lane) % 1292
            midpoint = r_min + j * step + step // 2
            x = k_ln2 + midpoint + offsets[lane % len(offsets)]
            v = expm1_vec(x, "reduction_k_sweep")
            if v is not None:
                vecs.append(v)

    # Public saturation/overflow seams; overflow inputs are excluded because
    # this corpus measures successful numerical results.
    for center in (-40 * SCALE, 40 * SCALE):
        for offset in range(-20, 21):
            v = expm1_vec(center + offset, "domain_limit_seam")
            if v is not None:
                vecs.append(v)

    # Permanent regression attaining the production corpus's full-domain
    # absolute-error maximum on the retained implementation.
    vecs.append(expm1_vec(39_981_510_191_812, "production_worst_regression"))

    pad_to(vecs, 10000, lambda: expm1_vec(
        random.randint(-40 * SCALE, 40 * SCALE - 1), "mixed_fill"))
    vecs = [v for v in vecs if v is not None][:10000]
    save("adv_expm1_vectors.json",
         {"suite": "adversarial", "function": "expm1_fixed",
          "zones": "near-zero raw/seam, all ln2 reduction seams, every LUT boundary, all reachable exponents, domain limits, signed fill",
          "reference": "mpmath.expm1 at 60 decimal digits"}, vecs)

# ============================================================
# EXP — 10,000 vectors
# ============================================================

def gen_exp():
    print("exp_fixed_i...")
    vecs = []
    LN2 = mpmath.log(2)
    phases = 32

    def exp_vec(x, cat):
        if x <= -40 * SCALE:
            return {"x": s(x), "expected": "0", "category": cat}
        if x >= 40 * SCALE:
            return None
        ref = nint(mpmath.exp(mpmath.mpf(x) / SCALE) * SCALE)
        if ref > I128_MAX or ref < 0: return None
        return {"x": s(x), "expected": s(ref), "category": cat}

    # Bracket every nearest-cell decision seam in the full successful domain.
    # floor/ceil are the two adjacent raw inputs surrounding the exact
    # half-cell boundary, so the adversarial suite cannot miss a phase-table
    # discontinuity or hide the amplified positive-tail worst case.
    step = LN2 / phases
    for cell in range(-2_000, 2_001):
        boundary = (mpmath.mpf(cell) + mpmath.mpf("0.5")) * step * SCALE
        if not (-40 * SCALE < boundary < 40 * SCALE):
            continue
        lower = int(mpmath.floor(boundary))
        upper = int(mpmath.ceil(boundary))
        vecs.append(exp_vec(lower, "cell_seam"))
        vecs.append(exp_vec(upper, "cell_seam"))

    # Sample cell interiors across every phase and reachable octave.
    for cell in range(-1_840, 1_841, 4):
        x = nint(mpmath.mpf(cell) * step * SCALE)
        v = exp_vec(x, "cell_interior")
        if v is not None:
            vecs.append(v)

    # Exact octave points and adjacent raw inputs retain the previous ln2
    # regressions while the cell sweep above targets the new implementation.
    for k in range(-58, 58):
        x_scaled = nint(mpmath.mpf(k) * LN2 * SCALE)
        for offset in (-1, 0, 1):
            v = exp_vec(x_scaled + offset, "ln2_multiple")
            if v is not None:
                vecs.append(v)

    # Correctly-rounded direct-return seams and raw values around zero.
    for sign in (-1, 1):
        for offset in range(-8, 9):
            vecs.append(exp_vec(sign * 1_000_000 + offset, "tiny_direct_seam"))
    for x in range(-2_048, 2_049, 17):
        vecs.append(exp_vec(x, "near_zero_raw"))

    # Saturation/overflow guards and a dense successful strip immediately
    # below +40, where power-of-two reconstruction amplifies absolute error.
    for offset in range(-16, 17):
        v = exp_vec(-40 * SCALE + offset, "negative_domain_seam")
        if v is not None:
            vecs.append(v)
        v = exp_vec(40 * SCALE + offset, "positive_domain_seam")
        if v is not None:
            vecs.append(v)
    for offset in range(1, 257):
        vecs.append(exp_vec(40 * SCALE - offset, "positive_limit_raw"))

    # Permanent regressions from the old and phased kernels.
    for x in (19_998_424_170_953, 38_468_881_341_913, 39_996_758_403_249):
        vecs.append(exp_vec(x, "worst_regression"))

    pad_to(vecs, 10000, lambda: exp_vec(
        random.randint(-40 * SCALE, 40 * SCALE - 1), "full_domain_fill"))
    vecs = [v for v in vecs if v is not None][:10000]

    save("adv_exp_vectors.json",
         {"suite": "adversarial", "function": "exp_fixed_i",
          "zones": "all ln2/32 cell seams, phased interiors, ln2 multiples, tiny-direct seam, raw zero, domain guards, dense +40 tail, permanent worst points, full-domain fill",
          "reference": "mpmath.exp at 60 decimal digits; contract saturation at x <= -40"}, vecs)

# ============================================================
# NORM_CDF — 10,000 vectors
# ============================================================

def gen_norm_cdf():
    print("norm_cdf_poly...")
    vecs = []
    rng = random.Random(0xCDF2026)
    cutoff = 7_130_506_848_171
    boundaries = [
        SCALE // 2, SCALE, 3 * SCALE // 2, 2 * SCALE,
        5 * SCALE // 2, 3 * SCALE, 7 * SCALE // 2, 4 * SCALE,
        9 * SCALE // 2, 5 * SCALE, 11 * SCALE // 2, 6 * SCALE,
        13 * SCALE // 2, 7 * SCALE,
        cutoff, 8 * SCALE,
    ]

    def cdf_ref(x_scaled):
        x_mp = mpmath.mpf(x_scaled) / SCALE
        return nint((1 + mpmath.erf(x_mp / mpmath.sqrt(2))) / 2 * SCALE)

    def cdf_vec(x, cat):
        return {"x": s(x), "expected": s(cdf_ref(x)), "category": cat}

    # Every body/tail/saturation seam, on both signs, with dense raw offsets.
    raw_offsets = [0]
    for off in (1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233,
                377, 610, 987, 1_597, 2_584, 4_096):
        raw_offsets.extend((off, -off))
    for boundary in boundaries:
        for off in raw_offsets:
            for sign in [1, -1]:
                vecs.append(cdf_vec(sign * (boundary + off),
                                    f"seam_{boundary}"))

    # Raw values around zero exercise exact symmetry and the center branch.
    for x in range(-4_096, 4_097, 17):
        vecs.append(cdf_vec(x, "near_zero_raw"))

    # Interior coverage for every actual polynomial and exact-tail interval.
    pieces = [
        (0, SCALE // 2), (SCALE // 2, SCALE),
        (SCALE, 3 * SCALE // 2), (3 * SCALE // 2, 2 * SCALE),
        (2 * SCALE, 5 * SCALE // 2), (5 * SCALE // 2, 3 * SCALE),
        (3 * SCALE, 7 * SCALE // 2), (7 * SCALE // 2, 4 * SCALE),
        (4 * SCALE, 9 * SCALE // 2), (9 * SCALE // 2, 5 * SCALE),
        (5 * SCALE, 11 * SCALE // 2), (11 * SCALE // 2, 6 * SCALE),
        (6 * SCALE, 13 * SCALE // 2), (13 * SCALE // 2, 7 * SCALE),
        (7 * SCALE, cutoff), (cutoff, 8 * SCALE),
    ]
    lane = 0
    while len(vecs) < 7_800:
        lo, hi = pieces[lane % len(pieces)]
        x = rng.randint(lo, hi)
        sign = 1 if (lane // len(pieces)) % 2 == 0 else -1
        vecs.append(cdf_vec(sign * x, f"interior_{lo}_{hi}"))
        lane += 1

    # Hit true-reference half-ULP transitions throughout the direct tail.
    # These are stronger than uniform random tails because a one-raw-input
    # movement can change the correctly rounded output.
    for index in range(600):
        exponent = mpmath.mpf(index) / 599
        tail_raw = mpmath.power(10, exponent * mpmath.log10(286_000))
        probability = (mpmath.floor(tail_raw) + mpmath.mpf("0.5")) / SCALE
        root = mpmath.sqrt(2) * mpmath.erfinv(1 - 2 * probability)
        x0 = nint(root * SCALE)
        for off in (-1, 0, 1):
            sign = 1 if (index + off) % 2 == 0 else -1
            vecs.append(cdf_vec(sign * (x0 + off), "tail_rounding_transition"))

    # Preserve independently observed worst production points as regressions.
    for x in (-1_016_809_046_576, -1_291_830_207_302,
              1_016_809_046_576, 1_291_830_207_302):
        vecs.append(cdf_vec(x, "observed_worst_regression"))

    # Zero, exact saturation points, and enormous public-domain inputs.
    vecs.append(cdf_vec(0, "zero"))
    for x in (8 * SCALE, 8 * SCALE + 1, 9 * SCALE, 10 * SCALE,
              20 * SCALE, 37 * SCALE, I128_MAX):
        vecs.append(cdf_vec(x, "extreme_tail"))
        vecs.append(cdf_vec(-x, "extreme_tail_neg"))
    vecs.append(cdf_vec(I128_MIN, "i128_min"))

    # Fill to exactly 10000
    while len(vecs) < 10000:
        x = rng.randint(-8 * SCALE, 8 * SCALE)
        vecs.append(cdf_vec(x, "fill"))
    vecs = vecs[:10000]

    save("adv_norm_cdf_vectors.json",
         {"suite": "adversarial", "function": "norm_cdf_poly",
          "boundaries_raw": boundaries,
          "zones": "all implementation seams, raw center, every piece interior, tail rounding transitions, saturation, i128 extrema",
          "reference": "mpmath.ncdf at 60 decimal digits"}, vecs)

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
        # pow_product_hp evaluates x^w * x^(1-w), whose exact reference is x.
        # The previous generator accidentally emitted x^w and made every
        # adversarial identity check compare against the wrong function.
        return {"x": s(x), "w": s(w), "expected": s(x), "category": cat}

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

    if "--ln-1p-only" in sys.argv:
        gen_ln_1p()
        sys.exit(0)
    if "--expm1-only" in sys.argv:
        gen_expm1()
        sys.exit(0)
    if "--exp-only" in sys.argv:
        gen_exp()
        sys.exit(0)
    if "--ln-only" in sys.argv:
        gen_ln()
        sys.exit(0)
    if "--norm-cdf-only" in sys.argv:
        gen_norm_cdf()
        sys.exit(0)

    gen_ln()
    gen_ln_1p()
    gen_expm1()
    gen_exp()
    gen_norm_cdf()
    gen_pow()
    gen_sincos()
    gen_bs()
    gen_exp_hp()
    gen_pow_product_hp()

    print("\nDone.")
