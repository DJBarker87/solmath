#!/usr/bin/env python3
"""Generate the repository-only QuantLib SABR corpus and Rust subset."""

from __future__ import annotations

import argparse
import json
import pathlib

import numpy as np
import QuantLib as ql


ROOT = pathlib.Path(__file__).resolve().parents[1]
SCALE = 1_000_000_000_000


def to_fp(value: float) -> int:
    return int(round(value * SCALE))


def row(
    forward: float,
    strike: float,
    maturity: float,
    alpha: float,
    beta: float,
    rho: float,
    nu: float,
    vol: float,
) -> dict[str, float | int]:
    return {
        "F": forward,
        "K": round(strike, 10),
        "T": maturity,
        "alpha": alpha,
        "beta": beta,
        "rho": rho,
        "nu": nu,
        "vol": vol,
        "F_fp": to_fp(forward),
        "K_fp": to_fp(strike),
        "T_fp": to_fp(maturity),
        "alpha_fp": to_fp(alpha),
        "beta_fp": to_fp(beta),
        "rho_fp": to_fp(rho),
        "nu_fp": to_fp(nu),
        "vol_fp": to_fp(vol),
    }


def sabr_vectors(target: int) -> list[dict[str, float | int]]:
    if target <= 0:
        raise ValueError("target must be positive")
    rng = np.random.default_rng(42)
    results: list[dict[str, float | int]] = []
    forward = 100.0
    alphas = np.linspace(0.02, 0.50, 20)
    betas = [0.0, 0.25, 0.5, 0.75, 1.0]
    rhos = np.linspace(-0.90, 0.50, 11)
    nus = [0.05, 0.10, 0.20, 0.30, 0.40, 0.60, 0.80]
    maturities = [0.1, 0.25, 0.5, 1.0, 2.0, 5.0, 10.0]
    ratios = [0.50, 0.70, 0.80, 0.90, 0.95, 1.00, 1.05, 1.10, 1.20, 1.50, 2.00]

    for alpha in alphas:
        for beta in betas:
            for rho in rhos:
                for nu in nus:
                    for maturity in maturities:
                        for ratio in ratios:
                            strike = forward * ratio
                            try:
                                vol = ql.sabrVolatility(
                                    strike, forward, maturity, alpha, beta, nu, rho
                                )
                            except RuntimeError:
                                continue
                            if np.isfinite(vol) and 0.0 < vol <= 5.0:
                                results.append(
                                    row(
                                        forward,
                                        strike,
                                        maturity,
                                        float(alpha),
                                        beta,
                                        float(rho),
                                        nu,
                                        float(vol),
                                    )
                                )
                                if len(results) == target:
                                    return results

    while len(results) < target:
        alpha = rng.uniform(0.01, 0.60)
        beta = rng.uniform(0.0, 1.0)
        rho = rng.uniform(-0.95, 0.60)
        nu = rng.uniform(0.01, 1.0)
        maturity = float(rng.choice(maturities))
        strike = forward * rng.uniform(0.5, 2.0)
        try:
            vol = ql.sabrVolatility(strike, forward, maturity, alpha, beta, nu, rho)
        except RuntimeError:
            continue
        if np.isfinite(vol) and 0.0 < vol <= 5.0:
            results.append(
                row(forward, strike, maturity, alpha, beta, rho, nu, float(vol))
            )
    return results


def render_rust(cases: list[dict[str, float | int]], count: int) -> str:
    step = max(1, len(cases) // count)
    subset = cases[::step][:count]
    output = [
        "// Auto-generated from QuantLib. Do not edit.",
        f"// {len(subset)} of {len(cases)} vectors (every {step}th)",
        "",
        "#[cfg(test)]",
        "mod quantlib_sabr {",
        "    use solmath::sabr_implied_vol;",
        "",
    ]
    for index, case in enumerate(subset):
        expected = int(case["vol_fp"])
        output.extend(
            [
                "    #[test]",
                f"    fn ql_sabr_{index:04d}() {{",
                "        let vol = sabr_implied_vol(",
                f"            {case['F_fp']}u128, {case['K_fp']}u128, {case['T_fp']}u128,",
                f"            {case['alpha_fp']}u128, {case['beta_fp']}u128, {case['rho_fp']}i128, {case['nu_fp']}u128,",
                "        ).unwrap();",
                f"        let expected = {expected}u128;",
                "        let tolerance = expected / 200;",
                "        assert!(vol.abs_diff(expected) <= tolerance);",
                "    }",
                "",
            ]
        )
    output.append("}")
    return "\n".join(output) + "\n"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--target", type=int, default=100_000)
    parser.add_argument("--rust-subset", type=int, default=500)
    args = parser.parse_args()
    cases = sabr_vectors(args.target)
    data_dir = ROOT / "test_data"
    data_dir.mkdir(exist_ok=True)
    (data_dir / "sabr_vectors.json").write_text(json.dumps(cases))
    (data_dir / "sabr_reference_tests.rs").write_text(
        render_rust(cases, args.rust_subset)
    )
    print(f"generated {len(cases):,} SABR vectors")


if __name__ == "__main__":
    main()
