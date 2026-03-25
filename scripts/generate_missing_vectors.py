#!/usr/bin/env python3
"""
Generate 50K production vectors for all functions missing from the existing benchmark.
mpmath 50+ digits for transcendental refs, exact arithmetic for fp ops.
"""

import json, os, random
import mpmath

mpmath.mp.dps = 50
random.seed(99)

SCALE = 10**12
SCALE_HP = 10**15
U128_MAX = 2**128 - 1
I128_MAX = 2**127 - 1
U64_MAX = 2**64 - 1

OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'benchmark')
N = 50_000

def save(name, meta, vecs):
    meta["n"] = len(vecs)
    with open(os.path.join(OUT, name), 'w') as f:
        json.dump({"meta": meta, "vectors": vecs}, f)
    print(f"  {len(vecs):,} -> {name}")

def s(x): return str(int(x))
def nint(x): return int(mpmath.nint(x))

def pad(vecs, target, gen):
    while len(vecs) < target:
        v = gen()
        if v: vecs.append(v)
    return vecs[:target]

# ============================================================
# Arithmetic rounding variants
# ============================================================

def gen_fp_mul_round():
    print("fp_mul_round...")
    def make():
        a = random.randint(1, 10**24)
        b = random.randint(1, 10**24)
        if a > U128_MAX or b > U128_MAX: return None
        p = a * b
        expected = (p + SCALE // 2) // SCALE
        if expected > U128_MAX: return None
        return {"a": s(a), "b": s(b), "expected": s(expected), "category": "random"}
    vecs = []
    pad(vecs, N, make)
    save("prod_fp_mul_round_vectors.json", {"function": "fp_mul_round"}, vecs)

def gen_fp_mul_i_round():
    print("fp_mul_i_round...")
    def make():
        a = random.randint(-10**24, 10**24)
        b = random.randint(-10**24, 10**24)
        p = a * b
        if p >= 0:
            expected = (p + SCALE // 2) // SCALE
        else:
            expected = (p - SCALE // 2) // SCALE
        if abs(expected) > I128_MAX: return None
        return {"a": s(a), "b": s(b), "expected": s(expected), "category": "random"}
    vecs = []
    pad(vecs, N, make)
    save("prod_fp_mul_i_round_vectors.json", {"function": "fp_mul_i_round"}, vecs)

def gen_fp_div_variants():
    print("fp_div_round, fp_div_floor, fp_div_ceil...")
    round_v, floor_v, ceil_v = [], [], []
    def make():
        a = random.randint(1, 10**20)
        b = random.randint(1, 10**20)
        num = a * SCALE
        q = num // b
        r = num % b
        if q > U128_MAX: return None
        rnd = q + (1 if 2 * r >= b else 0)
        flr = q
        cel = q + (1 if r > 0 else 0)
        return (
            {"a": s(a), "b": s(b), "expected": s(rnd), "category": "random"},
            {"a": s(a), "b": s(b), "expected": s(flr), "category": "random"},
            {"a": s(a), "b": s(b), "expected": s(cel), "category": "random"},
        )
    while len(round_v) < N:
        r = make()
        if r:
            round_v.append(r[0]); floor_v.append(r[1]); ceil_v.append(r[2])
    save("prod_fp_div_round_vectors.json", {"function": "fp_div_round"}, round_v[:N])
    save("prod_fp_div_floor_vectors.json", {"function": "fp_div_floor"}, floor_v[:N])
    save("prod_fp_div_ceil_vectors.json", {"function": "fp_div_ceil"}, ceil_v[:N])

def gen_fp_mul_hp_u():
    print("fp_mul_hp_u...")
    def make():
        a = random.randint(1, 10**24)
        b = random.randint(1, 10**24)
        expected = a * b // SCALE_HP
        if expected > U128_MAX: return None
        return {"a": s(a), "b": s(b), "expected": s(expected), "category": "random"}
    vecs = []
    pad(vecs, N, make)
    save("prod_fp_mul_hp_u_vectors.json", {"function": "fp_mul_hp_u"}, vecs)

def gen_checked_mul_div_variants():
    print("checked_mul_div_floor_i, checked_mul_div_ceil_i...")
    floor_v, ceil_v = [], []
    def make():
        a = random.randint(-10**20, 10**20)
        b = random.randint(-10**20, 10**20)
        c = random.randint(1, 10**20)
        num = a * b
        if num >= 0:
            flr = num // c
            cel = (num + c - 1) // c
        else:
            flr = -((-num + c - 1) // c)
            cel = -((-num) // c)
        if abs(flr) > I128_MAX or abs(cel) > I128_MAX: return None
        return (
            {"a": s(a), "b": s(b), "c": s(c), "expected": s(flr), "category": "random"},
            {"a": s(a), "b": s(b), "c": s(c), "expected": s(cel), "category": "random"},
        )
    while len(floor_v) < N:
        r = make()
        if r: floor_v.append(r[0]); ceil_v.append(r[1])
    save("prod_checked_mul_div_floor_i_vectors.json", {"function": "checked_mul_div_floor_i"}, floor_v[:N])
    save("prod_checked_mul_div_ceil_i_vectors.json", {"function": "checked_mul_div_ceil_i"}, ceil_v[:N])

def gen_mul_div_u128():
    print("mul_div_floor_u128, mul_div_ceil_u128...")
    floor_v, ceil_v = [], []
    def make():
        a = random.randint(1, U64_MAX)
        b = random.randint(1, U64_MAX)
        c = random.randint(1, U64_MAX)
        num = a * b
        flr = num // c
        cel = (num + c - 1) // c
        if flr > U128_MAX or cel > U128_MAX: return None
        return (
            {"a": s(a), "b": s(b), "c": s(c), "expected": s(flr)},
            {"a": s(a), "b": s(b), "c": s(c), "expected": s(cel)},
        )
    while len(floor_v) < N:
        r = make()
        if r: floor_v.append(r[0]); ceil_v.append(r[1])
    save("prod_mul_div_floor_u128_vectors.json", {"function": "mul_div_floor_u128"}, floor_v[:N])
    save("prod_mul_div_ceil_u128_vectors.json", {"function": "mul_div_ceil_u128"}, ceil_v[:N])

# ============================================================
# Transcendentals
# ============================================================

def gen_expm1():
    print("expm1...")
    def make():
        x = int(random.uniform(-2, 2) * SCALE)
        ref = nint((mpmath.exp(mpmath.mpf(x) / SCALE) - 1) * SCALE)
        if abs(ref) > I128_MAX: return None
        return {"x": s(x), "expected": s(ref), "category": "random"}
    vecs = []
    pad(vecs, N, make)
    save("prod_expm1_vectors.json", {"function": "expm1_fixed"}, vecs)

def gen_pow_fixed_i():
    print("pow_fixed_i...")
    def make():
        base = int(random.uniform(0.5, 2.0) * SCALE)
        exp = int(random.uniform(-5, 5) * SCALE)
        if base <= 0: return None
        ref = nint(mpmath.power(mpmath.mpf(base) / SCALE, mpmath.mpf(exp) / SCALE) * SCALE)
        if ref > I128_MAX or ref <= 0: return None
        return {"base": s(base), "exp": s(exp), "expected": s(ref), "category": "random"}
    vecs = []
    pad(vecs, N, make)
    save("prod_pow_fixed_i_vectors.json", {"function": "pow_fixed_i"}, vecs)

def gen_sincos():
    print("sincos...")
    vecs = []
    def make():
        x = int(random.uniform(-20, 20) * SCALE)
        x_mp = mpmath.mpf(x) / SCALE
        return {"x": s(x),
                "expected_sin": s(nint(mpmath.sin(x_mp) * SCALE)),
                "expected_cos": s(nint(mpmath.cos(x_mp) * SCALE)),
                "category": "random"}
    pad(vecs, N, make)
    save("prod_sincos_vectors.json", {"function": "sincos_fixed"}, vecs)

def gen_cdf_pdf():
    print("cdf_pdf...")
    vecs = []
    def make():
        x = int(random.uniform(-8, 8) * SCALE)
        x_mp = mpmath.mpf(x) / SCALE
        cdf = nint((1 + mpmath.erf(x_mp / mpmath.sqrt(2))) / 2 * SCALE)
        pdf = nint(mpmath.exp(-x_mp**2 / 2) / mpmath.sqrt(2 * mpmath.pi) * SCALE)
        return {"x": s(x), "expected_cdf": s(cdf), "expected_pdf": s(pdf), "category": "random"}
    pad(vecs, N, make)
    save("prod_cdf_pdf_vectors.json", {"function": "norm_cdf_and_pdf"}, vecs)

# ============================================================
# Complex
# ============================================================

def gen_complex():
    print("complex_mul, complex_div, complex_exp, complex_sqrt...")
    mul_v, div_v, exp_v, sqrt_v = [], [], [], []
    def make():
        a_re = int(random.uniform(-10, 10) * SCALE)
        a_im = int(random.uniform(-10, 10) * SCALE)
        b_re = int(random.uniform(-10, 10) * SCALE)
        b_im = int(random.uniform(-10, 10) * SCALE)
        a = mpmath.mpc(mpmath.mpf(a_re)/SCALE, mpmath.mpf(a_im)/SCALE)
        b = mpmath.mpc(mpmath.mpf(b_re)/SCALE, mpmath.mpf(b_im)/SCALE)
        m = a * b
        mul_vec = {"a_re": s(a_re), "a_im": s(a_im), "b_re": s(b_re), "b_im": s(b_im),
                    "expected_re": s(nint(m.real * SCALE)), "expected_im": s(nint(m.imag * SCALE))}
        div_vec = None
        if abs(b) > 1e-10:
            d = a / b
            div_vec = {"a_re": s(a_re), "a_im": s(a_im), "b_re": s(b_re), "b_im": s(b_im),
                        "expected_re": s(nint(d.real * SCALE)), "expected_im": s(nint(d.imag * SCALE))}
        e = mpmath.exp(a)
        exp_vec = None
        if abs(e.real) < 1e30 and abs(e.imag) < 1e30:
            exp_vec = {"re": s(a_re), "im": s(a_im),
                       "expected_re": s(nint(e.real * SCALE)), "expected_im": s(nint(e.imag * SCALE))}
        sq = mpmath.sqrt(a)
        sqrt_vec = {"re": s(a_re), "im": s(a_im),
                    "expected_re": s(nint(sq.real * SCALE)), "expected_im": s(nint(sq.imag * SCALE))}
        return mul_vec, div_vec, exp_vec, sqrt_vec

    while len(mul_v) < N:
        m, d, e, sq = make()
        mul_v.append(m)
        if d: div_v.append(d)
        if e: exp_v.append(e)
        sqrt_v.append(sq)
    save("prod_complex_mul_vectors.json", {"function": "complex_mul"}, mul_v[:N])
    save("prod_complex_div_vectors.json", {"function": "complex_div"}, div_v[:N])
    save("prod_complex_exp_vectors.json", {"function": "complex_exp"}, exp_v[:N])
    save("prod_complex_sqrt_vectors.json", {"function": "complex_sqrt"}, sqrt_v[:N])

# ============================================================
# Token conversion
# ============================================================

def gen_token():
    print("token_to_fp, fp_to_token...")
    to_fp, from_fp_floor, from_fp_ceil = [], [], []
    for _ in range(N):
        amount = random.randint(0, 10**15)
        decimals = random.choice([6, 8, 9, 12, 18])
        fp = amount * SCALE // (10 ** decimals)
        to_fp.append({"amount": s(amount), "decimals": s(decimals), "expected": s(fp)})
        fp_val = random.randint(0, 10**24)
        dec = random.choice([6, 8, 9, 12])
        mult = 10 ** dec
        flr = fp_val * mult // SCALE
        cel = (fp_val * mult + SCALE - 1) // SCALE
        from_fp_floor.append({"fp": s(fp_val), "decimals": s(dec), "expected": s(flr)})
        from_fp_ceil.append({"fp": s(fp_val), "decimals": s(dec), "expected": s(cel)})
    save("prod_token_to_fp_vectors.json", {"function": "token_to_fp"}, to_fp)
    save("prod_fp_to_token_floor_vectors.json", {"function": "fp_to_token_floor"}, from_fp_floor)
    save("prod_fp_to_token_ceil_vectors.json", {"function": "fp_to_token_ceil"}, from_fp_ceil)

# ============================================================
# BS price (non-full, just call+put)
# ============================================================

def gen_bs_price():
    print("black_scholes_price...")
    vecs = []
    def make():
        S = int(random.uniform(50, 200) * SCALE)
        K = int(random.uniform(50, 200) * SCALE)
        r = int(random.uniform(0, 0.1) * SCALE)
        sigma = int(random.uniform(0.05, 1.0) * SCALE)
        T = int(random.uniform(0.01, 2.0) * SCALE)
        S_mp, K_mp = mpmath.mpf(S)/SCALE, mpmath.mpf(K)/SCALE
        r_mp, sig_mp, T_mp = mpmath.mpf(r)/SCALE, mpmath.mpf(sigma)/SCALE, mpmath.mpf(T)/SCALE
        if sig_mp <= 0 or T_mp <= 0: return None
        sqrtT = mpmath.sqrt(T_mp)
        d1 = (mpmath.log(S_mp/K_mp) + (r_mp + sig_mp**2/2)*T_mp) / (sig_mp * sqrtT)
        d2 = d1 - sig_mp * sqrtT
        Nd1 = (1+mpmath.erf(d1/mpmath.sqrt(2)))/2
        Nd2 = (1+mpmath.erf(d2/mpmath.sqrt(2)))/2
        disc = mpmath.exp(-r_mp * T_mp)
        call = S_mp * Nd1 - K_mp * disc * Nd2
        put = call - S_mp + K_mp * disc
        return {"s": s(S), "k": s(K), "r": s(r), "sigma": s(sigma), "t": s(T),
                "call": s(max(0, nint(call * SCALE))), "put": s(max(0, nint(put * SCALE)))}
    pad(vecs, N, make)
    save("prod_black_scholes_price_vectors.json", {"function": "black_scholes_price"}, vecs)
    save("prod_black_scholes_price_hp_vectors.json", {"function": "black_scholes_price_hp"}, vecs)

# ============================================================
# SABR z/chi
# ============================================================

def gen_sabr_z_over_chi():
    print("sabr_z_over_chi...")
    vecs = []
    def make():
        z = int(random.uniform(-3, 3) * SCALE)
        rho = int(random.uniform(-0.95, 0.95) * SCALE)
        z_mp, rho_mp = mpmath.mpf(z)/SCALE, mpmath.mpf(rho)/SCALE
        inner = mpmath.sqrt(1 - 2*rho_mp*z_mp + z_mp**2)
        chi = mpmath.log((inner + z_mp - rho_mp) / (1 - rho_mp))
        if abs(chi) < 1e-30: return None
        ref = nint(z_mp / chi * SCALE)
        if abs(ref) > I128_MAX: return None
        return {"z": s(z), "rho": s(rho), "expected": s(ref)}
    pad(vecs, N, make)
    save("prod_sabr_z_over_chi_vectors.json", {"function": "sabr_z_over_chi_pade"}, vecs)

# ============================================================

if __name__ == '__main__':
    print(f"Generating 50K production vectors for missing functions\n")
    gen_fp_mul_round()
    gen_fp_mul_i_round()
    gen_fp_div_variants()
    gen_fp_mul_hp_u()
    gen_checked_mul_div_variants()
    gen_mul_div_u128()
    gen_expm1()
    gen_pow_fixed_i()
    gen_sincos()
    gen_cdf_pdf()
    gen_complex()
    gen_token()
    gen_bs_price()
    gen_sabr_z_over_chi()
    print("\nDone.")
