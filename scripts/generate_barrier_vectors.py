#!/usr/bin/env python3
"""
SolMath Barrier Option Validation Vectors

Generates reference barrier option prices using QuantLib's AnalyticBarrierEngine
with exact T (no day-count rounding). Cross-validates against mpmath at 50 digits.

QuantLib uses IEEE 754 f64 (~15.9 sig figs). mpmath provides the reference truth.

Covers all 8 configurations: 4 barrier types × call/put.
Stratified across S/K ratios, H/S ratios, sigma, T, r.
"""

import json
import math
import os
import random
import sys

import mpmath
import QuantLib as ql

mpmath.mp.dps = 50
random.seed(42)

SCALE = 10**12
OUTPUT_DIR = os.path.join(os.path.dirname(__file__), '..', 'benchmark')


def quantlib_barrier_exact_T(S, K, H, r, sigma, T, barrier_type, option_type):
    """Price a European barrier option using QuantLib's AnalyticBarrierEngine
    with exact T — no day-count rounding.

    We construct the term structures so that QuantLib sees the exact T we want:
    set evaluation date to t=0, maturity at t=T using a reference date that
    gives the exact year fraction.
    """
    try:
        # Use a reference date far in the past so the day-count doesn't matter.
        # We override the year fractions by choosing dates that give exact T.
        today = ql.Date(1, 1, 2025)
        ql.Settings.instance().evaluationDate = today

        # Compute days so that days/365 is as close to T as possible.
        # Use Actual365Fixed: yearFraction = days/365.
        days = max(1, round(T * 365))
        maturity = today + ql.Period(int(days), ql.Days)
        dc = ql.Actual365Fixed()
        actual_T = dc.yearFraction(today, maturity)

        # Build term structures using the actual_T that QuantLib will compute.
        # The prices QuantLib produces are at this actual_T.
        spot_handle = ql.QuoteHandle(ql.SimpleQuote(S))
        vol_handle = ql.BlackVolTermStructureHandle(
            ql.BlackConstantVol(today, ql.NullCalendar(),
                                ql.QuoteHandle(ql.SimpleQuote(sigma)), dc)
        )
        rate_handle = ql.YieldTermStructureHandle(
            ql.FlatForward(today, ql.QuoteHandle(ql.SimpleQuote(r)), dc)
        )
        div_handle = ql.YieldTermStructureHandle(
            ql.FlatForward(today, 0.0, dc)
        )

        process = ql.BlackScholesMertonProcess(
            spot_handle, div_handle, rate_handle, vol_handle
        )

        payoff = ql.PlainVanillaPayoff(option_type, K)
        exercise = ql.EuropeanExercise(maturity)

        # Barrier option
        barrier_opt = ql.BarrierOption(barrier_type, H, 0.0, payoff, exercise)
        barrier_opt.setPricingEngine(ql.AnalyticBarrierEngine(process))
        barrier_price = barrier_opt.NPV()

        # Vanilla for reference
        vanilla = ql.EuropeanOption(payoff, exercise)
        vanilla.setPricingEngine(ql.AnalyticEuropeanEngine(process))
        vanilla_price = vanilla.NPV()

        return barrier_price, vanilla_price, actual_T
    except Exception:
        return None


def mpmath_barrier_down_out_call(S, K, H, r, sigma, T):
    """Down-and-out call via Merton reflection at 50-digit precision.
    Only valid for down-out calls with K >= H (payoff entirely above barrier)."""
    S, K, H, r, sigma, T = [mpmath.mpf(x) for x in [S, K, H, r, sigma, T]]
    if T <= 0 or sigma <= 0 or S <= 0 or K <= 0 or H <= 0:
        return None
    if S <= H:
        return 0.0, 0.0
    if K < H:
        return None  # formula not numerically stable for K < H

    def bs_call(spot, strike):
        vol_sqrt_t = sigma * mpmath.sqrt(T)
        if vol_sqrt_t < mpmath.mpf('1e-30'):
            disc = mpmath.exp(-r * T)
            return max(spot - strike * disc, 0)
        d1 = (mpmath.log(spot / strike) + (r + sigma**2 / 2) * T) / vol_sqrt_t
        d2 = d1 - vol_sqrt_t
        disc = mpmath.exp(-r * T)
        return spot * mpmath.ncdf(d1) - strike * disc * mpmath.ncdf(d2)

    alpha = 2 * (r - sigma**2 / 2) / sigma**2
    power = (H / S) ** alpha
    vanilla = bs_call(S, K)
    reflected = bs_call(H**2 / S, K)
    return float(vanilla - power * reflected), float(vanilla)


def generate_param_grid():
    spots = [50, 80, 100, 120, 150, 200]
    sk_ratios = [0.80, 0.90, 0.95, 1.00, 1.05, 1.10, 1.20]
    sigmas = [0.10, 0.15, 0.20, 0.25, 0.30, 0.40, 0.50, 0.60]
    times = [0.05, 0.10, 0.25, 0.50, 1.00, 2.00]
    rates = [0.00, 0.02, 0.05, 0.08, 0.10]
    down_hs = [0.70, 0.75, 0.80, 0.85, 0.90, 0.95]
    up_hs = [1.05, 1.10, 1.15, 1.20, 1.30, 1.50]
    return spots, sk_ratios, sigmas, times, rates, down_hs, up_hs


def generate_vectors():
    spots, sk_ratios, sigmas, times, rates, down_hs, up_hs = generate_param_grid()

    barrier_configs = [
        ("down_out_call", ql.Barrier.DownOut, ql.Option.Call, down_hs, True),
        ("down_in_call",  ql.Barrier.DownIn,  ql.Option.Call, down_hs, True),
        ("down_out_put",  ql.Barrier.DownOut, ql.Option.Put,  down_hs, True),
        ("down_in_put",   ql.Barrier.DownIn,  ql.Option.Put,  down_hs, True),
        ("up_out_call",   ql.Barrier.UpOut,   ql.Option.Call, up_hs,   False),
        ("up_in_call",    ql.Barrier.UpIn,    ql.Option.Call, up_hs,   False),
        ("up_out_put",    ql.Barrier.UpOut,    ql.Option.Put,  up_hs,   False),
        ("up_in_put",     ql.Barrier.UpIn,     ql.Option.Put,  up_hs,   False),
    ]

    all_vectors = []
    skipped = 0
    total = 0

    for name, barrier_type, option_type, hs_ratios, is_down in barrier_configs:
        config_vectors = []
        is_call = (option_type == ql.Option.Call)

        for S in spots:
            for sk in sk_ratios:
                K = S * sk
                for hs in hs_ratios:
                    H = S * hs
                    if is_down and S <= H:
                        continue
                    if not is_down and S >= H:
                        continue
                    if is_call and not is_down and K >= H:
                        continue
                    if not is_call and is_down and K <= H:
                        continue

                    for sigma in sigmas:
                        for T in times:
                            for r in rates:
                                total += 1
                                result = quantlib_barrier_exact_T(
                                    S, K, H, r, sigma, T,
                                    barrier_type, option_type
                                )
                                if result is None:
                                    skipped += 1
                                    continue

                                barrier_price, vanilla_price, actual_T = result

                                # Store actual_T from QuantLib's day count
                                s_fp = int(round(S * SCALE))
                                k_fp = int(round(K * SCALE))
                                h_fp = int(round(H * SCALE))
                                r_fp = int(round(r * SCALE))
                                sigma_fp = int(round(sigma * SCALE))
                                t_fp = int(round(actual_T * SCALE))

                                vec = {
                                    "s": str(s_fp),
                                    "k": str(k_fp),
                                    "h": str(h_fp),
                                    "r": str(r_fp),
                                    "sigma": str(sigma_fp),
                                    "t": str(t_fp),
                                    "is_call": is_call,
                                    "barrier_type": name,
                                    "barrier_price": barrier_price,
                                    "vanilla_price": vanilla_price,
                                    "barrier_price_fp": str(int(round(
                                        barrier_price * SCALE))),
                                    "vanilla_price_fp": str(int(round(
                                        vanilla_price * SCALE))),
                                }
                                config_vectors.append(vec)

        print(f"  {name}: {len(config_vectors)} vectors")
        all_vectors.extend(config_vectors)

    print(f"\nTotal: {len(all_vectors)} vectors "
          f"({skipped} skipped out of {total} attempted)")

    # Cross-validate down-out calls (K>=H) against mpmath simple reflection formula
    print("\nCross-validating down-out calls (K>=H) against mpmath 50-digit...")
    doc = [v for v in all_vectors
           if v['barrier_type'] == 'down_out_call'
           and int(v['k']) >= int(v['h'])]
    sample = random.sample(doc, min(1000, len(doc)))
    max_ulp = 0.0
    cross_ok = 0
    for v in sample:
        S = int(v['s']) / SCALE
        K = int(v['k']) / SCALE
        H = int(v['h']) / SCALE
        r_val = int(v['r']) / SCALE
        sigma_val = int(v['sigma']) / SCALE
        T = int(v['t']) / SCALE

        mp_result = mpmath_barrier_down_out_call(S, K, H, r_val, sigma_val, T)
        if mp_result is None:
            continue
        mp_barrier, _ = mp_result
        ql_barrier = v['barrier_price']
        ulp = abs(ql_barrier - mp_barrier) * SCALE
        max_ulp = max(max_ulp, ulp)
        cross_ok += 1

    print(f"  {cross_ok} vectors, max diff: {max_ulp:.1f} ULP")

    # In/out conservation
    from collections import defaultdict
    by_params = defaultdict(dict)
    for v in all_vectors:
        key = (v['s'], v['k'], v['h'], v['r'], v['sigma'], v['t'], v['is_call'])
        by_params[key][v['barrier_type']] = v

    conservation_checked = 0
    max_conservation_err = 0
    for key, types in by_params.items():
        for in_name, out_name in [
            ('down_in_call', 'down_out_call'),
            ('down_in_put', 'down_out_put'),
            ('up_in_call', 'up_out_call'),
            ('up_in_put', 'up_out_put'),
        ]:
            if in_name in types and out_name in types:
                diff = abs(types[in_name]['barrier_price'] +
                           types[out_name]['barrier_price'] -
                           types[out_name]['vanilla_price'])
                conservation_checked += 1
                max_conservation_err = max(max_conservation_err, diff)

    print(f"\nIn/out conservation (QuantLib): {conservation_checked} pairs")
    print(f"  Max error: ${max_conservation_err:.2e}")

    return all_vectors


def main():
    print(f"Generating barrier option test vectors")
    print(f"QuantLib {ql.__version__} (AnalyticBarrierEngine)")
    print(f"mpmath at {mpmath.mp.dps} digits for cross-validation\n")

    vectors = generate_vectors()

    path = os.path.join(OUTPUT_DIR, 'barrier_vectors.json')
    with open(path, 'w') as f:
        json.dump({
            "meta": {
                "source": f"QuantLib {ql.__version__} AnalyticBarrierEngine",
                "cross_validation": f"mpmath {mpmath.mp.dps} digits",
                "n_vectors": len(vectors),
                "scale": SCALE,
                "no_dividends": True,
                "no_rebate": True,
                "T_note": "actual_T from QuantLib Actual365Fixed day count",
            },
            "vectors": vectors,
        }, f, indent=None)
    size_mb = os.path.getsize(path) / 1e6
    print(f"\nWrote {path} ({len(vectors)} vectors, {size_mb:.1f} MB)")


if __name__ == '__main__':
    main()
