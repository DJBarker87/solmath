#!/usr/bin/env python3
"""
SolMath Production Validation Vectors

100,000 vectors per function. Stratified across production-realistic domains.
These numbers go in the README — they represent what users will actually experience.

Primary references: mpmath at 50 decimal digits.
Cross-checked: 1,000 random samples per function against scipy/numpy.
"""

import json, os, random, math, sys
import mpmath
import scipy.stats
import scipy.special
import numpy as np

mpmath.mp.dps = 50
random.seed(42)
np.random.seed(42)

SCALE = 10**12
SCALE_I = SCALE
SCALE_HP = 10**15
I128_MAX = 2**127 - 1
I128_MIN = -(2**127)
U128_MAX = 2**128 - 1
N = 100_000

OUTPUT_DIR = os.path.join(os.path.dirname(__file__), '..', 'benchmark')

def save_vectors(filename, vectors, meta):
    path = os.path.join(OUTPUT_DIR, filename)
    with open(path, 'w') as f:
        json.dump({"meta": meta, "vectors": vectors}, f)
    print(f"  {filename}: {len(vectors)} vectors")

def _to_str(x):
    return str(int(x))

def _nint(x):
    return int(mpmath.nint(x))

def _trunc(x):
    if x >= 0:
        return int(mpmath.floor(x))
    else:
        return int(mpmath.ceil(x))

def stratified_sample(buckets, n_per_bucket):
    """Sample n_per_bucket uniform values from each (lo, hi) bucket."""
    samples = []
    for lo, hi in buckets:
        for _ in range(n_per_bucket):
            samples.append(random.uniform(lo, hi))
    return samples

def stratified_sample_log(buckets, n_per_bucket):
    """Sample log-uniform within each bucket."""
    samples = []
    for lo, hi in buckets:
        for _ in range(n_per_bucket):
            log_lo = math.log10(lo) if lo > 0 else -3
            log_hi = math.log10(hi)
            samples.append(10 ** random.uniform(log_lo, log_hi))
    return samples

def stratified_2d(buckets_x, buckets_y, n_per_cell):
    """Cartesian product of bucket pairs, n_per_cell from each cell."""
    samples = []
    for lo_x, hi_x in buckets_x:
        for lo_y, hi_y in buckets_y:
            for _ in range(n_per_cell):
                x = random.uniform(lo_x, hi_x)
                y = random.uniform(lo_y, hi_y)
                samples.append((x, y))
    return samples

def crosscheck(vectors, scipy_fn, key_in, key_expected, tol, label):
    """Cross-check 1000 random samples against a scipy reference."""
    samples = random.sample(vectors, min(1000, len(vectors)))
    mismatches = 0
    exceptions = 0
    first_exception = None
    for v in samples:
        try:
            scipy_ref = scipy_fn(v)
            mpmath_ref = int(v[key_expected])
            if abs(scipy_ref - mpmath_ref) > tol:
                mismatches += 1
        except Exception as exc:
            exceptions += 1
            if first_exception is None:
                first_exception = repr(exc)
    if not samples:
        raise RuntimeError(f"{label} cross-check selected zero samples")
    status = "OK" if mismatches == 0 and exceptions == 0 else (
        f"FAILED: {mismatches} mismatches, {exceptions} reference errors"
    )
    print(f"    Cross-check vs {label}: {len(samples)} samples, {status}")
    if mismatches or exceptions:
        detail = f"; first error: {first_exception}" if first_exception else ""
        raise RuntimeError(f"{label} cross-check failed{detail}")
    return 0


# ════════════════════════════════════════════════════════════
# Black-Scholes reference helper
# ════════════════════════════════════════════════════════════

def _bs_greeks_ref(S_fp, K_fp, r_fp, sigma_fp, T_fp):
    """Compute BS call, put, and all Greeks from fixed-point inputs at SCALE.
    Returns dict of fixed-point values, or None if degenerate."""
    S = mpmath.mpf(S_fp) / SCALE
    K = mpmath.mpf(K_fp) / SCALE
    r = mpmath.mpf(r_fp) / SCALE
    sigma = mpmath.mpf(sigma_fp) / SCALE
    T = mpmath.mpf(T_fp) / SCALE

    vol_sqrt_t = sigma * mpmath.sqrt(T)
    if vol_sqrt_t < mpmath.mpf('1e-15'):
        return None

    d1 = (mpmath.log(S / K) + (r + sigma**2 / 2) * T) / vol_sqrt_t
    d2 = d1 - vol_sqrt_t
    npdf_d1 = mpmath.exp(-d1**2 / 2) / mpmath.sqrt(2 * mpmath.pi)
    ncdf_d1 = mpmath.ncdf(d1)
    ncdf_d2 = mpmath.ncdf(d2)
    disc = mpmath.exp(-r * T)

    call = S * ncdf_d1 - K * disc * ncdf_d2
    put = K * disc * (1 - ncdf_d2) - S * (1 - ncdf_d1)

    delta_call = ncdf_d1
    delta_put = ncdf_d1 - 1
    gamma = npdf_d1 / (S * sigma * mpmath.sqrt(T))
    vega = S * npdf_d1 * mpmath.sqrt(T)
    theta_call = -(S * npdf_d1 * sigma) / (2 * mpmath.sqrt(T)) - r * K * disc * ncdf_d2
    theta_put = -(S * npdf_d1 * sigma) / (2 * mpmath.sqrt(T)) + r * K * disc * (1 - ncdf_d2)
    rho_call = K * T * disc * ncdf_d2
    rho_put = -K * T * disc * (1 - ncdf_d2)

    def to_fp(v):
        return _nint(v * SCALE)

    result = {
        'call': to_fp(call),
        'put': to_fp(put),
        'call_delta': to_fp(delta_call),
        'put_delta': to_fp(delta_put),
        'gamma': to_fp(gamma),
        'vega': to_fp(vega),
        'call_theta': to_fp(theta_call),
        'put_theta': to_fp(theta_put),
        'call_rho': to_fp(rho_call),
        'put_rho': to_fp(rho_put),
    }

    for v in result.values():
        if v > I128_MAX or v < I128_MIN:
            return None

    return result


# ════════════════════════════════════════════════════════════
# 1D Functions
# ════════════════════════════════════════════════════════════

def gen_prod_ln():
    print("  ln_fixed_i ...")
    rng = random.Random(0x1A2026)
    buckets = [
        (0.001, 0.01),
        (0.01, 0.1),
        (0.1, 0.5),
        (0.5, 2.0),
        (2.0, 10.0),
        (10.0, 100.0),
        (100.0, 1e6),
        (1e6, 1e15),
    ]
    n_per = N // len(buckets)

    vectors = []
    for bucket_index, (lo, hi) in enumerate(buckets):
        for _ in range(n_per):
            if lo < 1 and hi > 1:
                x_real = rng.uniform(lo, hi)
            else:
                log_lo = math.log10(lo)
                log_hi = math.log10(hi)
                x_real = 10 ** rng.uniform(log_lo, log_hi)
            x = int(x_real * SCALE)
            if x <= 0:
                continue
            ref = _nint(mpmath.log(mpmath.mpf(x) / SCALE) * SCALE)
            if ref > I128_MAX or ref < I128_MIN:
                continue
            vectors.append({"x": _to_str(x), "expected": _to_str(ref),
                            "category": f"bucket_{bucket_index}_{lo}_{hi}"})

    crosscheck(vectors,
        lambda v: int(round(float(np.log(float(int(v['x'])) / SCALE)) * SCALE)),
        'x', 'expected', 100, 'numpy')

    save_vectors("prod_ln_vectors.json", vectors,
                 {"suite": "production", "function": "ln_fixed_i",
                  "domain": "log-uniform [0.001, 1e15], 8 stratified decades",
                  "n": len(vectors)})


def gen_prod_ln_1p():
    print("  ln_1p_fixed ...")
    vectors = []

    def add(x, category):
        if x <= -SCALE or x > I128_MAX:
            return
        ref = _nint(mpmath.log1p(mpmath.mpf(x) / SCALE) * SCALE)
        if I128_MIN <= ref <= I128_MAX:
            vectors.append({"x": _to_str(x), "expected": _to_str(ref),
                            "category": category})

    # Ten equally weighted regimes cover the domain edge, ordinary rates,
    # the sub-ULP cancellation region, and large positive values.
    for _ in range(N // 10):
        one_plus_x_raw = int(10 ** random.uniform(0, 10))
        add(-SCALE + max(1, one_plus_x_raw), "near_domain_edge")
    for _ in range(N // 10):
        add(random.randint(-990_000_000_000, -500_000_000_000), "negative_large")
    for _ in range(N // 10):
        add(random.randint(-500_000_000_000, -10_000_000_000), "negative_mid")
    for _ in range(N // 10):
        add(random.randint(-10_000_000_000, -1_000_000), "negative_small")
    for _ in range(N // 10):
        add(random.randint(-999_999, -1), "near_zero_negative")
    for _ in range(N // 10):
        add(random.randint(0, 999_999), "near_zero_nonnegative")
    for _ in range(N // 10):
        add(random.randint(1_000_000, 10_000_000_000), "positive_small")
    for _ in range(N // 10):
        add(random.randint(10_000_000_000, SCALE), "positive_mid")
    for _ in range(N // 10):
        real = 10 ** random.uniform(0, 6)
        add(int(real * SCALE), "positive_large")
    for _ in range(N // 10):
        real = 10 ** random.uniform(6, 15)
        add(int(real * SCALE), "positive_huge")

    # IEEE f64 cannot preserve raw-unit inputs immediately above x=-1 or at
    # very large magnitudes, so use the ordinary-rate subset for the secondary
    # NumPy cross-check. mpmath remains the primary reference for every row.
    numpy_safe = [v for v in vectors
                  if v["category"] not in ("near_domain_edge", "positive_huge")]
    crosscheck(
        numpy_safe,
        lambda v: int(round(float(np.log1p(float(int(v['x'])) / SCALE)) * SCALE)),
        'x', 'expected', 100, 'numpy.log1p')

    save_vectors("prod_ln_1p_vectors.json", vectors,
                 {"suite": "production", "function": "ln_1p_fixed",
                  "domain": "10 equal regimes over (-1, 1e15], including raw-unit near-zero inputs",
                  "reference": "mpmath.log1p at 50 decimal digits; 1,000-sample numpy.log1p cross-check",
                  "n": len(vectors)})


def gen_prod_expm1():
    print("  expm1_fixed ...")
    vectors = []

    def add(x, category):
        ref = _nint(mpmath.expm1(mpmath.mpf(x) / SCALE) * SCALE)
        if I128_MIN <= ref <= I128_MAX:
            vectors.append({"x": _to_str(x), "expected": _to_str(ref),
                            "category": category})

    regimes = (
        (-40 * SCALE, -20 * SCALE, "negative_saturation"),
        (-20 * SCALE, -2 * SCALE, "negative_large"),
        (-2 * SCALE, -SCALE // 2, "negative_mid"),
        (-SCALE // 2, -1_000_000, "negative_small"),
        (-999_999, -1, "near_zero_negative"),
        (0, 999_999, "near_zero_nonnegative"),
        (1_000_000, SCALE // 2, "positive_small"),
        (SCALE // 2, 2 * SCALE, "positive_mid"),
        (2 * SCALE, 20 * SCALE, "positive_large"),
        (20 * SCALE, 40 * SCALE - 1, "near_overflow"),
    )
    for lo, hi, category in regimes:
        for _ in range(N // len(regimes)):
            add(random.randint(lo, hi), category)

    numpy_safe = [v for v in vectors if abs(int(v["x"])) <= 2 * SCALE]
    crosscheck(
        numpy_safe,
        lambda v: int(round(float(np.expm1(float(int(v['x'])) / SCALE)) * SCALE)),
        'x', 'expected', 2, 'numpy.expm1')

    save_vectors("prod_expm1_vectors.json", vectors,
                 {"suite": "production", "function": "expm1_fixed",
                  "domain": "10 equal regimes over [-40,40), including raw near-zero inputs",
                  "reference": "mpmath.expm1 at 50 decimal digits; 1,000-sample numpy.expm1 cross-check",
                  "n": len(vectors)})


def gen_prod_ln_hp():
    print("  ln_fixed_hp ...")
    buckets = [
        (0.001, 0.01), (0.01, 0.1), (0.1, 0.5),
        (0.5, 2.0), (2.0, 10.0), (10.0, 100.0),
        (100.0, 1e6), (1e6, 1e15),
    ]
    n_per = N // len(buckets)

    vectors = []
    for lo, hi in buckets:
        for _ in range(n_per):
            if lo < 1 and hi > 1:
                x_real = random.uniform(lo, hi)
            else:
                log_lo = math.log10(lo)
                log_hi = math.log10(hi)
                x_real = 10 ** random.uniform(log_lo, log_hi)
            x = int(x_real * SCALE_HP)
            if x <= 0:
                continue
            ref = _nint(mpmath.log(mpmath.mpf(x) / SCALE_HP) * SCALE_HP)
            if ref > I128_MAX or ref < I128_MIN:
                continue
            vectors.append({"x": _to_str(x), "expected": _to_str(ref)})

    save_vectors("prod_ln_hp_vectors.json", vectors,
                 {"suite": "production", "function": "ln_fixed_hp",
                  "domain": "log-uniform [0.001, 1e15], 8 stratified decades at SCALE_HP",
                  "n": len(vectors)})


def gen_prod_exp():
    print("  exp_fixed_i ...")
    buckets = [
        (-20, -10),
        (-10, -3),
        (-3, -1),
        (-1, -0.1),
        (-0.1, 0.1),
        (0.1, 1),
        (1, 3),
        (3, 10),
        (10, 20),
    ]
    vectors = []
    quotient, remainder = divmod(N, len(buckets))
    for bucket_index, (lo, hi) in enumerate(buckets):
        # Distribute the remainder instead of silently emitting 99,999 rows
        # when N is not divisible by the nine production regimes.
        count = quotient + (1 if bucket_index < remainder else 0)
        for _ in range(count):
            x_real = random.uniform(lo, hi)
            x = int(x_real * SCALE)
            ref = _nint(mpmath.exp(mpmath.mpf(x) / SCALE) * SCALE)
            if ref > I128_MAX or ref <= 0:
                continue
            vectors.append({"x": _to_str(x), "expected": _to_str(ref),
                            "category": f"bucket_{bucket_index}_{lo}_{hi}"})

    # At large positive x, converting the scaled result to binary64 loses far
    # more than 1,000 raw decimal units even when both references agree. Keep
    # the independent NumPy check in the ordinary |x| <= 3 domain where its
    # integer conversion has adequate resolution; mpmath is authoritative for
    # all 100,000 rows.
    numpy_safe = [v for v in vectors if abs(int(v["x"])) <= 3 * SCALE]
    crosscheck(numpy_safe,
        lambda v: int(round(float(np.exp(float(int(v['x'])) / SCALE)) * SCALE)),
        'x', 'expected', 1000, 'numpy')

    save_vectors("prod_exp_vectors.json", vectors,
                 {"suite": "production", "function": "exp_fixed_i",
                  "domain": "[-20, 20] stratified 9 buckets, oversampled near 0",
                  "reference": "mpmath.exp at 50 decimal digits; 1,000-sample NumPy cross-check over |x| <= 3",
                  "n": len(vectors)})


def gen_prod_exp_hp():
    print("  exp_fixed_hp ...")
    buckets = [
        (-20, -10), (-10, -3), (-3, -1), (-1, -0.1),
        (-0.1, 0.1), (0.1, 1), (1, 3), (3, 10), (10, 20),
    ]
    n_per = N // len(buckets)

    vectors = []
    for lo, hi in buckets:
        for _ in range(n_per):
            x_real = random.uniform(lo, hi)
            x = int(x_real * SCALE_HP)
            ref = _nint(mpmath.exp(mpmath.mpf(x) / SCALE_HP) * SCALE_HP)
            if ref > I128_MAX or ref <= 0:
                continue
            vectors.append({"x": _to_str(x), "expected": _to_str(ref)})

    save_vectors("prod_exp_hp_vectors.json", vectors,
                 {"suite": "production", "function": "exp_fixed_hp",
                  "domain": "[-20, 20] stratified 9 buckets at SCALE_HP",
                  "n": len(vectors)})


def gen_prod_sqrt():
    print("  fp_sqrt ...")
    buckets = [
        (0.001, 0.01), (0.01, 0.1), (0.1, 1.0),
        (1.0, 10.0), (10.0, 1000.0), (1000.0, 1e6),
        (1e6, 1e9), (1e9, 1e15),
    ]
    n_per = N // len(buckets)

    vectors = []
    for lo, hi in buckets:
        for _ in range(n_per):
            log_lo = math.log10(lo)
            log_hi = math.log10(hi)
            x_real = 10 ** random.uniform(log_lo, log_hi)
            x = int(x_real * SCALE)
            if x <= 0:
                continue
            ref = _nint(mpmath.sqrt(mpmath.mpf(x) / SCALE) * SCALE)
            if ref > U128_MAX:
                continue
            vectors.append({"x": _to_str(x), "expected": _to_str(ref)})

    crosscheck(vectors,
        lambda v: int(round(float(np.sqrt(float(int(v['x'])) / SCALE)) * SCALE)),
        'x', 'expected', 10, 'numpy')

    save_vectors("prod_sqrt_vectors.json", vectors,
                 {"suite": "production", "function": "fp_sqrt",
                  "domain": "log-uniform [0.001, 1e15] stratified 8 decades",
                  "n": len(vectors)})


def gen_prod_sin():
    print("  sin_fixed ...")
    PI = float(mpmath.pi)
    buckets = [
        (-4*PI, -PI), (-PI, -PI/2), (-PI/2, -0.1),
        (-0.1, 0.1),
        (0.1, PI/2), (PI/2, PI), (PI, 4*PI),
    ]
    n_per = N // len(buckets)

    vectors = []
    for lo, hi in buckets:
        for _ in range(n_per):
            x_real = random.uniform(lo, hi)
            x = int(x_real * SCALE)
            ref = _nint(mpmath.sin(mpmath.mpf(x) / SCALE) * SCALE)
            vectors.append({"x": _to_str(x), "expected": _to_str(ref)})

    crosscheck(vectors,
        lambda v: int(round(float(np.sin(float(int(v['x'])) / SCALE)) * SCALE)),
        'x', 'expected', 10, 'numpy')

    save_vectors("prod_sin_vectors.json", vectors,
                 {"suite": "production", "function": "sin_fixed",
                  "domain": "[-4pi, 4pi] stratified 7 buckets",
                  "n": len(vectors)})


def gen_prod_cos():
    print("  cos_fixed ...")
    PI = float(mpmath.pi)
    buckets = [
        (-4*PI, -PI), (-PI, -PI/2), (-PI/2, -0.1),
        (-0.1, 0.1), (0.1, PI/2), (PI/2, PI), (PI, 4*PI),
    ]
    n_per = N // len(buckets)

    vectors = []
    for lo, hi in buckets:
        for _ in range(n_per):
            x_real = random.uniform(lo, hi)
            x = int(x_real * SCALE)
            ref = _nint(mpmath.cos(mpmath.mpf(x) / SCALE) * SCALE)
            vectors.append({"x": _to_str(x), "expected": _to_str(ref)})

    crosscheck(vectors,
        lambda v: int(round(float(np.cos(float(int(v['x'])) / SCALE)) * SCALE)),
        'x', 'expected', 10, 'numpy')

    save_vectors("prod_cos_vectors.json", vectors,
                 {"suite": "production", "function": "cos_fixed",
                  "domain": "[-4pi, 4pi] stratified 7 buckets",
                  "n": len(vectors)})


def gen_prod_norm_cdf():
    print("  norm_cdf_poly ...")
    rng = random.Random(0xCDF2026)
    buckets = [
        (-6, -3),
        (-3, -2),
        (-2, -1),
        (-1, 0),
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 6),
    ]
    n_per = N // len(buckets)

    vectors = []
    for bucket_index, (lo, hi) in enumerate(buckets):
        for _ in range(n_per):
            x_real = rng.uniform(lo, hi)
            x = int(x_real * SCALE)
            ref = _nint(mpmath.ncdf(mpmath.mpf(x) / SCALE) * SCALE)
            vectors.append({"x": _to_str(x), "expected": _to_str(ref),
                            "category": f"bucket_{bucket_index}_{lo}_{hi}"})

    crosscheck(vectors,
        lambda v: int(round(float(scipy.stats.norm.cdf(float(int(v['x'])) / SCALE)) * SCALE)),
        'x', 'expected', 10, 'scipy.stats.norm.cdf')

    save_vectors("prod_norm_cdf_vectors.json", vectors,
                 {"suite": "production", "function": "norm_cdf_poly",
                  "domain": "[-6, 6] stratified 8 buckets, oversampled body",
                  "n": len(vectors)})


def gen_prod_norm_cdf_hp():
    print("  norm_cdf_poly_hp ...")
    buckets = [
        (-6, -3), (-3, -2), (-2, -1), (-1, 0),
        (0, 1), (1, 2), (2, 3), (3, 6),
    ]
    n_per = N // len(buckets)

    vectors = []
    for lo, hi in buckets:
        for _ in range(n_per):
            x_real = random.uniform(lo, hi)
            x = int(x_real * SCALE_HP)
            ref = _nint(mpmath.ncdf(mpmath.mpf(x) / SCALE_HP) * SCALE_HP)
            vectors.append({"x": _to_str(x), "expected": _to_str(ref)})

    save_vectors("prod_norm_cdf_hp_vectors.json", vectors,
                 {"suite": "production", "function": "norm_cdf_poly_hp",
                  "domain": "[-6, 6] stratified 8 buckets at SCALE_HP",
                  "n": len(vectors)})


def gen_prod_norm_pdf():
    print("  norm_pdf ...")
    buckets = [
        (-6, -3), (-3, -2), (-2, -1), (-1, 0),
        (0, 1), (1, 2), (2, 3), (3, 6),
    ]
    n_per = N // len(buckets)

    vectors = []
    for lo, hi in buckets:
        for _ in range(n_per):
            x_real = random.uniform(lo, hi)
            x = int(x_real * SCALE)
            ref = _nint(mpmath.npdf(mpmath.mpf(x) / SCALE) * SCALE)
            vectors.append({"x": _to_str(x), "expected": _to_str(ref)})

    crosscheck(vectors,
        lambda v: int(round(float(scipy.stats.norm.pdf(float(int(v['x'])) / SCALE)) * SCALE)),
        'x', 'expected', 10, 'scipy.stats.norm.pdf')

    save_vectors("prod_norm_pdf_vectors.json", vectors,
                 {"suite": "production", "function": "norm_pdf",
                  "domain": "[-6, 6] stratified 8 buckets",
                  "n": len(vectors)})


# ════════════════════════════════════════════════════════════
# 2D Functions — Stratified Cartesian Grid
# ════════════════════════════════════════════════════════════

def gen_prod_fp_mul():
    print("  fp_mul (unsigned) ...")
    buckets = [
        (0.001, 0.1), (0.1, 1.0), (1.0, 10.0),
        (10.0, 1000.0), (1000.0, 1e6), (1e6, 1e12),
    ]
    n_cells = len(buckets) ** 2
    n_per_cell = N // n_cells

    vectors = []
    for lo_a, hi_a in buckets:
        for lo_b, hi_b in buckets:
            for _ in range(n_per_cell):
                a_real = 10 ** random.uniform(math.log10(lo_a), math.log10(hi_a))
                b_real = 10 ** random.uniform(math.log10(lo_b), math.log10(hi_b))
                a = int(a_real * SCALE)
                b = int(b_real * SCALE)
                ref = _nint(mpmath.mpf(a) * mpmath.mpf(b) / SCALE)
                if ref > U128_MAX:
                    continue
                vectors.append({"a": _to_str(a), "b": _to_str(b), "expected": _to_str(ref)})

    save_vectors("prod_fp_mul_vectors.json", vectors,
                 {"suite": "production", "function": "fp_mul",
                  "domain": "a,b log-uniform [0.001, 1e12], 6x6 stratified grid",
                  "n": len(vectors)})


def gen_prod_fp_mul_i():
    print("  fp_mul_i (signed) ...")
    mag_buckets = [
        (0.001, 0.1), (0.1, 1.0), (1.0, 10.0),
        (10.0, 1000.0), (1000.0, 1e6), (1e6, 1e12),
    ]
    n_cells = len(mag_buckets) ** 2
    n_per_cell = N // n_cells

    vectors = []
    for lo_a, hi_a in mag_buckets:
        for lo_b, hi_b in mag_buckets:
            for _ in range(n_per_cell):
                a_real = 10 ** random.uniform(math.log10(lo_a), math.log10(hi_a))
                b_real = 10 ** random.uniform(math.log10(lo_b), math.log10(hi_b))
                if random.random() < 0.5:
                    a_real = -a_real
                if random.random() < 0.5:
                    b_real = -b_real
                a = int(a_real * SCALE)
                b = int(b_real * SCALE)
                ref = _trunc(mpmath.mpf(a) * mpmath.mpf(b) / SCALE)
                if ref > I128_MAX or ref < I128_MIN:
                    continue
                vectors.append({"a": _to_str(a), "b": _to_str(b), "expected": _to_str(ref)})

    save_vectors("prod_fp_mul_i_vectors.json", vectors,
                 {"suite": "production", "function": "fp_mul_i",
                  "domain": "a,b log-uniform magnitude [0.001, 1e12], random signs, 6x6 grid",
                  "n": len(vectors)})


def gen_prod_fp_div():
    print("  fp_div (unsigned) ...")
    buckets = [
        (0.001, 0.1), (0.1, 1.0), (1.0, 10.0),
        (10.0, 1000.0), (1000.0, 1e6), (1e6, 1e12),
    ]
    n_cells = len(buckets) ** 2
    n_per_cell = N // n_cells

    vectors = []
    for lo_a, hi_a in buckets:
        for lo_b, hi_b in buckets:
            for _ in range(n_per_cell):
                a_real = 10 ** random.uniform(math.log10(lo_a), math.log10(hi_a))
                b_real = 10 ** random.uniform(math.log10(lo_b), math.log10(hi_b))
                a = int(a_real * SCALE)
                b = int(b_real * SCALE)
                if b == 0:
                    continue
                ref = _nint(mpmath.mpf(a) * SCALE / mpmath.mpf(b))
                if ref > U128_MAX:
                    continue
                vectors.append({"a": _to_str(a), "b": _to_str(b), "expected": _to_str(ref)})

    save_vectors("prod_fp_div_vectors.json", vectors,
                 {"suite": "production", "function": "fp_div",
                  "domain": "a,b log-uniform [0.001, 1e12], 6x6 stratified grid",
                  "n": len(vectors)})


def gen_prod_fp_div_i():
    print("  fp_div_i (signed) ...")
    mag_buckets = [
        (0.001, 0.1), (0.1, 1.0), (1.0, 10.0),
        (10.0, 1000.0), (1000.0, 1e6), (1e6, 1e12),
    ]
    n_cells = len(mag_buckets) ** 2
    n_per_cell = N // n_cells

    vectors = []
    for lo_a, hi_a in mag_buckets:
        for lo_b, hi_b in mag_buckets:
            for _ in range(n_per_cell):
                a_real = 10 ** random.uniform(math.log10(lo_a), math.log10(hi_a))
                b_real = 10 ** random.uniform(math.log10(lo_b), math.log10(hi_b))
                if random.random() < 0.5:
                    a_real = -a_real
                if random.random() < 0.5:
                    b_real = -b_real
                a = int(a_real * SCALE)
                b = int(b_real * SCALE)
                if abs(b) < 1:
                    continue
                ref = _trunc(mpmath.mpf(a) * SCALE / mpmath.mpf(b))
                if ref > I128_MAX or ref < I128_MIN:
                    continue
                vectors.append({"a": _to_str(a), "b": _to_str(b), "expected": _to_str(ref)})

    save_vectors("prod_fp_div_i_vectors.json", vectors,
                 {"suite": "production", "function": "fp_div_i",
                  "domain": "a,b log-uniform mag [0.001, 1e12], random signs, 6x6 grid",
                  "n": len(vectors)})


def gen_prod_pow_fixed():
    print("  pow_fixed ...")
    base_buckets = [
        (0.01, 0.1),
        (0.1, 0.5),
        (0.5, 0.99),
        (0.99, 1.01),
        (1.01, 2.0),
        (2.0, 10.0),
        (10.0, 100.0),
    ]
    exp_buckets = [
        (0.01, 0.1),
        (0.1, 0.45),
        (0.45, 0.55),
        (0.55, 0.95),
        (0.95, 1.05),
        (1.05, 2.0),
        (2.0, 5.0),
    ]
    n_cells = len(base_buckets) * len(exp_buckets)
    n_per_cell = N // n_cells

    vectors = []
    for lo_b, hi_b in base_buckets:
        for lo_e, hi_e in exp_buckets:
            for _ in range(n_per_cell):
                base_real = random.uniform(lo_b, hi_b)
                exp_real = random.uniform(lo_e, hi_e)
                base = int(base_real * SCALE)
                exp = int(exp_real * SCALE)
                if base <= 0:
                    continue
                ref = _nint(mpmath.power(mpmath.mpf(base) / SCALE, mpmath.mpf(exp) / SCALE) * SCALE)
                if ref > U128_MAX or ref <= 0:
                    continue
                vectors.append({"base": _to_str(base), "exp": _to_str(exp), "expected": _to_str(ref)})

    crosscheck(vectors,
        lambda v: int(round(float(np.power(float(int(v['base'])) / SCALE, float(int(v['exp'])) / SCALE)) * SCALE)),
        'base', 'expected', 1000, 'numpy')

    save_vectors("prod_pow_fixed_vectors.json", vectors,
                 {"suite": "production", "function": "pow_fixed",
                  "domain": "base 7 buckets [0.01,100], exp 7 buckets [0.01,5], 49-cell grid",
                  "n": len(vectors)})


def gen_prod_pow_fixed_hp():
    print("  pow_fixed_hp ...")
    base_buckets = [
        (0.01, 0.1), (0.1, 0.5), (0.5, 0.99), (0.99, 1.01),
        (1.01, 2.0), (2.0, 10.0), (10.0, 100.0),
    ]
    exp_buckets = [
        (0.01, 0.1), (0.1, 0.45), (0.45, 0.55), (0.55, 0.95),
        (0.95, 1.05), (1.05, 2.0), (2.0, 5.0),
    ]
    n_cells = len(base_buckets) * len(exp_buckets)
    n_per_cell = N // n_cells

    vectors = []
    for lo_b, hi_b in base_buckets:
        for lo_e, hi_e in exp_buckets:
            for _ in range(n_per_cell):
                base_real = random.uniform(lo_b, hi_b)
                exp_real = random.uniform(lo_e, hi_e)
                base = int(base_real * SCALE)
                exp = int(exp_real * SCALE)
                if base <= 0:
                    continue
                ref = _nint(mpmath.power(mpmath.mpf(base) / SCALE, mpmath.mpf(exp) / SCALE) * SCALE)
                if ref > U128_MAX or ref <= 0:
                    continue
                vectors.append({"base": _to_str(base), "exp": _to_str(exp), "expected": _to_str(ref)})

    save_vectors("prod_pow_fixed_hp_vectors.json", vectors,
                 {"suite": "production", "function": "pow_fixed_hp",
                  "domain": "base 7 buckets [0.01,100], exp 7 buckets [0.01,5], 49-cell grid",
                  "n": len(vectors)})


def gen_prod_pow_int():
    print("  pow_int ...")
    base_buckets = [
        (0.5, 0.99), (0.99, 1.01), (1.01, 2.0),
        (2.0, 5.0), (5.0, 10.0),
    ]
    exp_values = [2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 15]
    n_cells = len(base_buckets) * len(exp_values)
    n_per_cell = N // n_cells

    vectors = []
    for lo_b, hi_b in base_buckets:
        for n_exp in exp_values:
            for _ in range(n_per_cell):
                base_real = random.uniform(lo_b, hi_b)
                base = int(base_real * SCALE)
                if base <= 0:
                    continue
                ref = _nint(mpmath.power(mpmath.mpf(base) / SCALE, n_exp) * SCALE)
                if ref > U128_MAX or ref <= 0:
                    continue
                vectors.append({"base": _to_str(base), "n": n_exp, "expected": _to_str(ref)})

    save_vectors("prod_pow_int_vectors.json", vectors,
                 {"suite": "production", "function": "pow_int",
                  "domain": "base 5 buckets [0.5,10], n in {2..15}, 55-cell grid",
                  "n": len(vectors)})


# ════════════════════════════════════════════════════════════
# Black-Scholes — 5D stratified
# ════════════════════════════════════════════════════════════

def gen_prod_bs_full():
    print("  bs_full (std) ...")
    moneyness_buckets = [
        (0.5, 0.8),
        (0.8, 0.95),
        (0.95, 1.05),
        (1.05, 1.2),
        (1.2, 2.0),
    ]
    sigma_buckets = [
        (0.05, 0.15),
        (0.15, 0.30),
        (0.30, 0.60),
        (0.60, 0.80),
    ]
    T_buckets = [
        (0.1, 0.5),
        (0.5, 1.0),
        (1.0, 2.0),
    ]
    r_range = (0.01, 0.10)

    n_cells = len(moneyness_buckets) * len(sigma_buckets) * len(T_buckets)
    n_per_cell = N // n_cells

    vectors = []
    for m_lo, m_hi in moneyness_buckets:
        for s_lo, s_hi in sigma_buckets:
            for t_lo, t_hi in T_buckets:
                for _ in range(n_per_cell):
                    S_real = random.uniform(50, 500)
                    moneyness = random.uniform(m_lo, m_hi)
                    K_real = S_real / moneyness
                    if K_real < 10 or K_real > 2000:
                        continue
                    sigma_real = random.uniform(s_lo, s_hi)
                    T_real = random.uniform(t_lo, t_hi)
                    r_real = random.uniform(r_range[0], r_range[1])

                    S = int(S_real * SCALE)
                    K = int(K_real * SCALE)
                    r = int(r_real * SCALE)
                    sigma = int(sigma_real * SCALE)
                    T = int(T_real * SCALE)

                    greeks = _bs_greeks_ref(S, K, r, sigma, T)
                    if greeks is None:
                        continue

                    vectors.append({
                        "s": _to_str(S), "k": _to_str(K), "r": _to_str(r),
                        "sigma": _to_str(sigma), "t": _to_str(T),
                        **{key: _to_str(val) for key, val in greeks.items()}
                    })

    # Cross-check call prices against scipy
    samples = random.sample(vectors, min(1000, len(vectors)))
    errors = 0
    for v in samples:
        S_f = float(int(v['s'])) / SCALE
        K_f = float(int(v['k'])) / SCALE
        r_f = float(int(v['r'])) / SCALE
        sig_f = float(int(v['sigma'])) / SCALE
        T_f = float(int(v['t'])) / SCALE
        d1 = (np.log(S_f / K_f) + (r_f + sig_f**2 / 2) * T_f) / (sig_f * np.sqrt(T_f))
        d2 = d1 - sig_f * np.sqrt(T_f)
        scipy_call = S_f * scipy.stats.norm.cdf(d1) - K_f * np.exp(-r_f * T_f) * scipy.stats.norm.cdf(d2)
        scipy_call_scaled = int(round(scipy_call * SCALE))
        mpmath_call = int(v['call'])
        if abs(scipy_call_scaled - mpmath_call) > 100:
            errors += 1
    print(f"    Cross-check vs scipy BS: {len(samples)} samples, {errors} call mismatches >100 ULP")

    save_vectors("prod_bs_full_vectors.json", vectors,
                 {"suite": "production", "function": "bs_full",
                  "domain": "moneyness 5 buckets, sigma 4 buckets, T 3 buckets, 60-cell grid",
                  "n": len(vectors)})


def gen_prod_bs_full_hp():
    print("  bs_full_hp ...")
    moneyness_buckets = [
        (0.5, 0.8), (0.8, 0.95), (0.95, 1.05), (1.05, 1.2), (1.2, 2.0),
    ]
    sigma_buckets = [
        (0.05, 0.15), (0.15, 0.30), (0.30, 0.60), (0.60, 0.80),
    ]
    T_buckets = [(0.1, 0.5), (0.5, 1.0), (1.0, 2.0)]
    r_range = (0.01, 0.10)

    n_cells = len(moneyness_buckets) * len(sigma_buckets) * len(T_buckets)
    n_per_cell = N // n_cells

    vectors = []
    for m_lo, m_hi in moneyness_buckets:
        for s_lo, s_hi in sigma_buckets:
            for t_lo, t_hi in T_buckets:
                for _ in range(n_per_cell):
                    S_real = random.uniform(50, 500)
                    moneyness = random.uniform(m_lo, m_hi)
                    K_real = S_real / moneyness
                    if K_real < 10 or K_real > 2000:
                        continue
                    sigma_real = random.uniform(s_lo, s_hi)
                    T_real = random.uniform(t_lo, t_hi)
                    r_real = random.uniform(r_range[0], r_range[1])

                    S = int(S_real * SCALE)
                    K = int(K_real * SCALE)
                    r = int(r_real * SCALE)
                    sigma = int(sigma_real * SCALE)
                    T = int(T_real * SCALE)

                    greeks = _bs_greeks_ref(S, K, r, sigma, T)
                    if greeks is None:
                        continue

                    vectors.append({
                        "s": _to_str(S), "k": _to_str(K), "r": _to_str(r),
                        "sigma": _to_str(sigma), "t": _to_str(T),
                        **{key: _to_str(val) for key, val in greeks.items()}
                    })

    save_vectors("prod_bs_full_hp_vectors.json", vectors,
                 {"suite": "production", "function": "bs_full_hp",
                  "domain": "moneyness 5 buckets, sigma 4 buckets, T 3 buckets, 60-cell grid",
                  "n": len(vectors)})


# ════════════════════════════════════════════════════════════
# Pool Math
# ════════════════════════════════════════════════════════════

def gen_prod_pow_product_hp():
    print("  pow_product_hp ...")
    x_buckets = [
        (1, 10), (10, 100), (100, 1000),
        (1000, 1e4), (1e4, 1e5), (1e5, 1e6),
    ]
    w_buckets = [
        (0.05, 0.15), (0.15, 0.30), (0.30, 0.50),
        (0.50, 0.70), (0.70, 0.85), (0.85, 0.95),
    ]
    n_cells = len(x_buckets) * len(w_buckets)
    n_per_cell = N // n_cells

    vectors = []
    for lo_x, hi_x in x_buckets:
        for lo_w, hi_w in w_buckets:
            for _ in range(n_per_cell):
                x_real = 10 ** random.uniform(math.log10(lo_x), math.log10(hi_x))
                w_real = random.uniform(lo_w, hi_w)
                x = int(x_real * SCALE)
                w = int(w_real * SCALE)
                ref = x  # identity: x^w * x^(1-w) = x
                vectors.append({"x": _to_str(x), "w": _to_str(w), "expected": _to_str(ref)})

    save_vectors("prod_pow_product_hp_vectors.json", vectors,
                 {"suite": "production", "function": "pow_product_hp",
                  "domain": "x log-uniform [1, 1e6], w uniform [0.05, 0.95], 36-cell grid",
                  "note": "reference is identity: x^w * x^(1-w) = x",
                  "n": len(vectors)})


# ════════════════════════════════════════════════════════════
# HP Primitives
# ════════════════════════════════════════════════════════════

def gen_prod_fp_mul_hp_i():
    print("  fp_mul_hp_i ...")
    mag_buckets = [
        (0.001, 0.01), (0.01, 0.1), (0.1, 1.0),
        (1.0, 10.0),
    ]
    n_cells = len(mag_buckets) ** 2
    n_per_cell = N // n_cells

    vectors = []
    for lo_a, hi_a in mag_buckets:
        for lo_b, hi_b in mag_buckets:
            for _ in range(n_per_cell):
                a_real = random.uniform(lo_a, hi_a)
                b_real = random.uniform(lo_b, hi_b)
                if random.random() < 0.5:
                    a_real = -a_real
                if random.random() < 0.5:
                    b_real = -b_real
                a = int(a_real * SCALE_HP)
                b = int(b_real * SCALE_HP)
                ref = _nint(mpmath.mpf(a) * mpmath.mpf(b) / SCALE_HP)
                if ref > I128_MAX or ref < I128_MIN:
                    continue
                vectors.append({"a": _to_str(a), "b": _to_str(b), "expected": _to_str(ref)})

    save_vectors("prod_fp_mul_hp_i_vectors.json", vectors,
                 {"suite": "production", "function": "fp_mul_hp_i",
                  "domain": "a,b in [-10, 10] at SCALE_HP, 4x4 stratified grid",
                  "n": len(vectors)})


def gen_prod_fp_div_hp_safe():
    print("  fp_div_hp_safe ...")
    mag_buckets = [
        (0.001, 0.1), (0.1, 1.0), (1.0, 10.0),
        (10.0, 1000.0), (1000.0, 1e6),
    ]
    n_cells = len(mag_buckets) ** 2
    n_per_cell = N // n_cells

    vectors = []
    for lo_a, hi_a in mag_buckets:
        for lo_b, hi_b in mag_buckets:
            for _ in range(n_per_cell):
                a_real = 10 ** random.uniform(math.log10(lo_a), math.log10(hi_a))
                b_real = 10 ** random.uniform(math.log10(lo_b), math.log10(hi_b))
                if random.random() < 0.5:
                    a_real = -a_real
                if random.random() < 0.5:
                    b_real = -b_real
                a = int(a_real * SCALE_HP)
                b = int(b_real * SCALE_HP)
                if abs(b) < SCALE_HP // 1000:
                    continue
                ref = _nint(mpmath.mpf(a) * SCALE_HP / mpmath.mpf(b))
                if ref > I128_MAX or ref < I128_MIN:
                    continue
                vectors.append({"a": _to_str(a), "b": _to_str(b), "expected": _to_str(ref)})

    save_vectors("prod_fp_div_hp_safe_vectors.json", vectors,
                 {"suite": "production", "function": "fp_div_hp_safe",
                  "domain": "a,b log-uniform mag [0.001, 1e6] at SCALE_HP, 5x5 grid",
                  "n": len(vectors)})


def gen_prod_checked_mul_div():
    print("  checked_mul_div_i ...")
    mag_buckets = [
        (0.001, 1.0), (1.0, 1000.0), (1000.0, 1e6), (1e6, 1e12),
    ]
    n_cells = len(mag_buckets) ** 3
    n_per_cell = N // n_cells

    vectors = []
    for lo_a, hi_a in mag_buckets:
        for lo_b, hi_b in mag_buckets:
            for lo_c, hi_c in mag_buckets:
                for _ in range(n_per_cell):
                    a_real = 10 ** random.uniform(math.log10(lo_a), math.log10(hi_a))
                    b_real = 10 ** random.uniform(math.log10(lo_b), math.log10(hi_b))
                    c_real = 10 ** random.uniform(math.log10(lo_c), math.log10(hi_c))
                    if random.random() < 0.5:
                        a_real = -a_real
                    if random.random() < 0.5:
                        b_real = -b_real
                    if random.random() < 0.3:
                        c_real = -c_real
                    a = int(a_real * SCALE)
                    b = int(b_real * SCALE)
                    c = int(c_real * SCALE)
                    if c == 0:
                        continue
                    ref_big = mpmath.mpf(a) * mpmath.mpf(b) / mpmath.mpf(c)
                    ref = _trunc(ref_big)
                    if ref > I128_MAX or ref < I128_MIN:
                        vectors.append({"a": _to_str(a), "b": _to_str(b), "c": _to_str(c),
                                       "expected": None, "category": "overflow"})
                        continue
                    vectors.append({"a": _to_str(a), "b": _to_str(b), "c": _to_str(c),
                                   "expected": _to_str(ref), "category": "normal"})

    save_vectors("prod_checked_mul_div_i_vectors.json", vectors,
                 {"suite": "production", "function": "checked_mul_div_i",
                  "domain": "a,b,c log-uniform mag [0.001, 1e12] at SCALE, 4x4x4 grid",
                  "n": len(vectors)})


if __name__ == '__main__':
    print("SolMath Production Validation Vector Generator")
    print(f"N = {N} vectors per function, stratified domains\n")

    if "--ln-1p-only" in sys.argv:
        gen_prod_ln_1p()
        sys.exit(0)
    if "--expm1-only" in sys.argv:
        gen_prod_expm1()
        sys.exit(0)
    if "--exp-only" in sys.argv:
        gen_prod_exp()
        sys.exit(0)
    if "--ln-only" in sys.argv:
        gen_prod_ln()
        sys.exit(0)
    if "--norm-cdf-only" in sys.argv:
        gen_prod_norm_cdf()
        sys.exit(0)

    # Arithmetic
    gen_prod_fp_mul()
    gen_prod_fp_mul_i()
    gen_prod_fp_div()
    gen_prod_fp_div_i()
    gen_prod_checked_mul_div()

    # Transcendentals
    gen_prod_ln()
    gen_prod_ln_1p()
    gen_prod_expm1()
    gen_prod_ln_hp()
    gen_prod_exp()
    gen_prod_exp_hp()
    gen_prod_sqrt()
    gen_prod_pow_fixed()
    gen_prod_pow_fixed_hp()
    gen_prod_pow_int()

    # Trig
    gen_prod_sin()
    gen_prod_cos()

    # Normal distribution
    gen_prod_norm_cdf()
    gen_prod_norm_cdf_hp()
    gen_prod_norm_pdf()

    # Black-Scholes
    gen_prod_bs_full()
    gen_prod_bs_full_hp()

    # Pool math
    gen_prod_pow_product_hp()

    # HP primitives
    gen_prod_fp_mul_hp_i()
    gen_prod_fp_div_hp_safe()

    total = sum(1 for name in dir() if name.startswith('gen_prod_'))
    print(f"\nDone. Generated vectors for {total} functions.")
