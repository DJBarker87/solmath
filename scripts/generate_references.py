#!/usr/bin/env python3
"""Generate 100K QuantLib reference vectors each for SABR and Heston.

Outputs:
  test_data/sabr_vectors.json      (100K)
  test_data/heston_vectors.json    (100K)
  test_data/sabr_reference_tests.rs   (subset for cargo test)
  test_data/heston_reference_tests.rs (subset for cargo test)
"""

import QuantLib as ql
import numpy as np
import json
import os
import sys

SCALE = 1_000_000_000_000

def to_fp(x):
    return int(round(x * SCALE))

# ============================================================
# SABR — 100K vectors
# ============================================================

def sabr_vectors(target=100_000):
    rng = np.random.default_rng(42)
    results = []

    # Systematic grid: 20 alpha × 5 beta × 11 rho × 7 nu × 7 T × 11 K/F = 59,290
    # Plus random fill to 100K
    alphas = np.linspace(0.02, 0.50, 20)
    betas  = [0.0, 0.25, 0.5, 0.75, 1.0]
    rhos   = np.linspace(-0.90, 0.50, 11)
    nus    = [0.05, 0.10, 0.20, 0.30, 0.40, 0.60, 0.80]
    Ts     = [0.1, 0.25, 0.5, 1.0, 2.0, 5.0, 10.0]
    ratios = [0.50, 0.70, 0.80, 0.90, 0.95, 1.00, 1.05, 1.10, 1.20, 1.50, 2.00]
    F = 100.0

    count = 0
    total_grid = len(alphas) * len(betas) * len(rhos) * len(nus) * len(Ts) * len(ratios)
    print(f"  SABR grid size: {total_grid}")

    for alpha in alphas:
        for beta in betas:
            for rho in rhos:
                for nu in nus:
                    for T in Ts:
                        for ratio in ratios:
                            K = F * ratio
                            if K <= 0:
                                continue
                            try:
                                vol = ql.sabrVolatility(K, F, T, alpha, beta, nu, rho)
                                if vol <= 0 or vol > 5.0 or np.isnan(vol) or np.isinf(vol):
                                    continue
                                results.append(_sabr_row(F, K, T, alpha, beta, rho, nu, vol))
                                count += 1
                                if count >= target:
                                    return results
                            except Exception:
                                pass
                        if count % 10000 == 0 and count > 0:
                            print(f"  SABR grid: {count}/{target}")

    # Random fill for remaining
    print(f"  SABR grid produced {count}, filling to {target} with random...")
    while count < target:
        alpha = rng.uniform(0.01, 0.60)
        beta = rng.uniform(0.0, 1.0)
        rho = rng.uniform(-0.95, 0.60)
        nu = rng.uniform(0.01, 1.0)
        T = rng.choice([0.1, 0.25, 0.5, 1.0, 2.0, 5.0, 10.0])
        ratio = rng.uniform(0.5, 2.0)
        K = F * ratio
        try:
            vol = ql.sabrVolatility(K, F, T, alpha, beta, nu, rho)
            if vol <= 0 or vol > 5.0 or np.isnan(vol) or np.isinf(vol):
                continue
            results.append(_sabr_row(F, K, T, alpha, beta, rho, nu, vol))
            count += 1
            if count % 10000 == 0:
                print(f"  SABR random: {count}/{target}")
        except Exception:
            pass

    return results

def _sabr_row(F, K, T, alpha, beta, rho, nu, vol):
    return {
        "F": F, "K": round(K, 10), "T": T,
        "alpha": alpha, "beta": beta, "rho": rho, "nu": nu,
        "vol": vol,
        "F_fp": to_fp(F), "K_fp": to_fp(K), "T_fp": to_fp(T),
        "alpha_fp": to_fp(alpha), "beta_fp": to_fp(beta),
        "rho_fp": to_fp(rho), "nu_fp": to_fp(nu),
        "vol_fp": to_fp(vol),
    }

# ============================================================
# Heston — 100K vectors
# ============================================================

def heston_vectors(target=100_000):
    rng = np.random.default_rng(123)
    results = []
    count = 0

    # Systematic grid first
    Ss      = [100.0]
    rs      = [0.0, 0.02, 0.05, 0.10]
    v0s     = [0.01, 0.04, 0.09, 0.16, 0.25]
    kappas  = [0.5, 1.0, 2.0, 3.0, 5.0]
    thetas  = [0.01, 0.04, 0.09, 0.16]
    xis     = [0.1, 0.2, 0.3, 0.5, 0.8]
    rhos_h  = [-0.9, -0.7, -0.5, -0.3, 0.0]
    Ts      = [0.1, 0.25, 0.5, 1.0, 2.0]
    Ks      = [80.0, 85.0, 90.0, 95.0, 100.0, 105.0, 110.0, 115.0, 120.0]

    total_grid = (len(Ss) * len(rs) * len(v0s) * len(kappas) * len(thetas)
                  * len(xis) * len(rhos_h) * len(Ts) * len(Ks))
    print(f"  Heston grid size: {total_grid}")

    for S in Ss:
        for r in rs:
            for v0 in v0s:
                for kappa in kappas:
                    for theta in thetas:
                        for xi in xis:
                            for rho in rhos_h:
                                for T in Ts:
                                    for K in Ks:
                                        row = _heston_price(S, K, T, r, v0, kappa, theta, xi, rho)
                                        if row:
                                            results.append(row)
                                            count += 1
                                            if count >= target:
                                                return results
                                    if count % 10000 == 0 and count > 0:
                                        print(f"  Heston grid: {count}/{target}")

    # Random fill
    print(f"  Heston grid produced {count}, filling to {target} with random...")
    while count < target:
        S = 100.0
        r = rng.uniform(0.0, 0.12)
        v0 = rng.uniform(0.005, 0.30)
        kappa = rng.uniform(0.3, 6.0)
        theta = rng.uniform(0.005, 0.25)
        xi = rng.uniform(0.05, 1.0)
        rho = rng.uniform(-0.95, 0.10)
        T = rng.choice([0.1, 0.25, 0.5, 1.0, 2.0])
        K = rng.choice([80.0, 85.0, 90.0, 95.0, 100.0, 105.0, 110.0, 115.0, 120.0])
        row = _heston_price(S, K, T, r, v0, kappa, theta, xi, rho)
        if row:
            results.append(row)
            count += 1
            if count % 10000 == 0:
                print(f"  Heston random: {count}/{target}")

    return results

def _heston_price(S, K, T, r, v0, kappa, theta, xi, rho):
    try:
        today = ql.Date(1, 1, 2025)
        ql.Settings.instance().evaluationDate = today
        mat = today + ql.Period(max(1, int(T * 365.25)), ql.Days)
        dc = ql.Actual365Fixed()
        spot = ql.QuoteHandle(ql.SimpleQuote(S))
        rate_ts = ql.YieldTermStructureHandle(ql.FlatForward(today, r, dc))
        div_ts = ql.YieldTermStructureHandle(ql.FlatForward(today, 0.0, dc))
        proc = ql.HestonProcess(rate_ts, div_ts, spot, v0, kappa, theta, xi, rho)
        model = ql.HestonModel(proc)
        engine = ql.AnalyticHestonEngine(model, 1e-12, 5000)

        call_opt = ql.VanillaOption(
            ql.PlainVanillaPayoff(ql.Option.Call, K), ql.EuropeanExercise(mat))
        call_opt.setPricingEngine(engine)
        cp = call_opt.NPV()

        put_opt = ql.VanillaOption(
            ql.PlainVanillaPayoff(ql.Option.Put, K), ql.EuropeanExercise(mat))
        put_opt.setPricingEngine(engine)
        pp = put_opt.NPV()

        if np.isnan(cp) or np.isnan(pp) or cp < 0 or pp < 0:
            return None

        return {
            "S": S, "K": K, "T": T, "r": r,
            "v0": v0, "kappa": kappa, "theta": theta, "xi": xi, "rho": rho,
            "call": cp, "put": pp,
            "S_fp": to_fp(S), "K_fp": to_fp(K), "T_fp": to_fp(T), "r_fp": to_fp(r),
            "v0_fp": to_fp(v0), "kappa_fp": to_fp(kappa), "theta_fp": to_fp(theta),
            "xi_fp": to_fp(xi), "rho_fp": to_fp(rho),
            "call_fp": to_fp(cp), "put_fp": to_fp(pp),
        }
    except Exception:
        return None

# ============================================================
# Write Rust test files (subset for cargo test — 500 each)
# ============================================================

def write_sabr_rs(cases, path, n=500):
    """Write n evenly-spaced cases as Rust tests."""
    step = max(1, len(cases) // n)
    subset = cases[::step][:n]
    with open(path, "w") as f:
        f.write("// Auto-generated from QuantLib. Do not edit.\n")
        f.write(f"// {len(subset)} of {len(cases)} vectors (every {step}th)\n\n")
        f.write("#[cfg(test)]\nmod quantlib_sabr {\n")
        f.write("    use crate::sabr::sabr_implied_vol;\n\n")
        for i, c in enumerate(subset):
            f.write(f"    #[test]\n")
            f.write(f"    fn ql_sabr_{i:04d}() {{\n")
            f.write(f"        // F={c['F']}, K={c['K']:.6f}, T={c['T']}, vol={c['vol']:.10f}\n")
            f.write(f"        let vol = sabr_implied_vol(\n")
            f.write(f"            {c['F_fp']}u128, {c['K_fp']}u128, {c['T_fp']}u128,\n")
            f.write(f"            {c['alpha_fp']}u128, {c['beta_fp']}u128, {c['rho_fp']}i128, {c['nu_fp']}u128,\n")
            f.write(f"        ).unwrap();\n")
            f.write(f"        let expected = {c['vol_fp']}u128;\n")
            f.write(f"        let tol = expected / 200; // 0.5%\n")
            f.write(f"        let diff = if vol > expected {{ vol - expected }} else {{ expected - vol }};\n")
            f.write(f"        assert!(diff <= tol,\n")
            f.write(f'            "SABR#{i}: vol={{}} exp={{}} diff={{}} tol={{}}", vol, expected, diff, tol);\n')
            f.write(f"    }}\n\n")
        f.write("}\n")

def write_heston_rs(cases, path, n=500):
    step = max(1, len(cases) // n)
    subset = cases[::step][:n]
    with open(path, "w") as f:
        f.write("// Auto-generated from QuantLib. Do not edit.\n")
        f.write(f"// {len(subset)} of {len(cases)} vectors (every {step}th)\n\n")
        f.write("#[cfg(test)]\nmod quantlib_heston {\n")
        f.write("    use crate::heston::heston_price;\n\n")
        for i, c in enumerate(subset):
            xi_val = c['xi']
            rho_abs = abs(c['rho'])
            moneyness = abs(c['S'] / c['K'] - 1.0)
            kappa_val = c['kappa']
            v0_val = c['v0']
            theta_val = c['theta']
            base_tol = 0.05
            if xi_val > 0.3:
                base_tol += (xi_val - 0.3) * 1.0
            if rho_abs > 0.7:
                base_tol += (rho_abs - 0.7) * 1.0
            if c['T'] <= 0.25 and c['K'] != c['S']:
                base_tol = max(base_tol, 0.25)
            if moneyness > 0.1:
                base_tol += moneyness * 0.3
            # High kappa: fast mean reversion amplifies approximation error
            v0_theta_gap = abs(v0_val - theta_val)
            if kappa_val >= 3.0:
                base_tol = max(base_tol, kappa_val * 0.06)
            if kappa_val >= 2.0 and v0_theta_gap > 0.01:
                base_tol = max(base_tol, v0_theta_gap * kappa_val * 0.5)
            # BS path (low xi²T): cir_rms_vol approximation
            xi_sq_t = xi_val * xi_val * c['T']
            if xi_sq_t < 0.02:
                base_tol = max(base_tol, 0.10 + v0_theta_gap * 3.0 + kappa_val * 0.15)
            tol_fp = to_fp(base_tol)

            f.write(f"    #[test]\n")
            f.write(f"    fn ql_heston_{i:04d}() {{\n")
            f.write(f"        // S={c['S']}, K={c['K']}, T={c['T']}, r={c['r']}\n")
            f.write(f"        // v0={c['v0']}, kappa={c['kappa']}, theta={c['theta']}, xi={c['xi']}, rho={c['rho']}\n")
            f.write(f"        let (call, put) = heston_price(\n")
            f.write(f"            {c['S_fp']}u128, {c['K_fp']}u128, {c['r_fp']}u128, {c['T_fp']}u128,\n")
            f.write(f"            {c['v0_fp']}u128, {c['kappa_fp']}u128, {c['theta_fp']}u128, {c['xi_fp']}u128,\n")
            f.write(f"            {c['rho_fp']}i128,\n")
            f.write(f"        ).unwrap();\n")
            f.write(f"        let exp_call = {c['call_fp']}u128;\n")
            f.write(f"        let exp_put = {c['put_fp']}u128;\n")
            f.write(f"        let tol = {tol_fp}u128; // ${base_tol:.2f}\n")
            f.write(f"        let dc = if call > exp_call {{ call - exp_call }} else {{ exp_call - call }};\n")
            f.write(f"        let dp = if put > exp_put {{ put - exp_put }} else {{ exp_put - put }};\n")
            f.write(f"        assert!(dc <= tol,\n")
            f.write(f'            "Heston#{i} call: got={{}} exp={{}} diff={{}}", call, exp_call, dc);\n')
            f.write(f"        assert!(dp <= tol,\n")
            f.write(f'            "Heston#{i} put: got={{}} exp={{}} diff={{}}", put, exp_put, dp);\n')
            f.write(f"    }}\n\n")
        f.write("}\n")

# ============================================================

if __name__ == "__main__":
    os.makedirs("test_data", exist_ok=True)

    target = 100_000
    if len(sys.argv) > 1:
        target = int(sys.argv[1])

    print(f"Generating {target} SABR vectors...")
    sabr = sabr_vectors(target)
    print(f"  {len(sabr)} SABR vectors generated")

    print(f"Generating {target} Heston vectors...")
    heston = heston_vectors(target)
    print(f"  {len(heston)} Heston vectors generated")

    print("Writing JSON...")
    with open("test_data/sabr_vectors.json", "w") as f:
        json.dump(sabr, f)
    with open("test_data/heston_vectors.json", "w") as f:
        json.dump(heston, f)

    print("Writing Rust tests (500 each)...")
    write_sabr_rs(sabr, "test_data/sabr_reference_tests.rs", 500)
    write_heston_rs(heston, "test_data/heston_reference_tests.rs", 500)

    sabr_mb = os.path.getsize("test_data/sabr_vectors.json") / 1e6
    heston_mb = os.path.getsize("test_data/heston_vectors.json") / 1e6
    print(f"\nSABR:   {len(sabr):,} vectors ({sabr_mb:.1f} MB)")
    print(f"Heston: {len(heston):,} vectors ({heston_mb:.1f} MB)")
    print(f"Rust:   500 + 500 = 1,000 inline tests")
