#!/usr/bin/env python3
"""Independent high-precision NIG oracle audit.

Two non-shared representations are evaluated:

1. direct Bessel-density integration of the out-of-the-money payoff with a
   64-point arbitrary-precision Gauss rule; and
2. Lewis Fourier inversion of the martingale-corrected characteristic
   function.

The script also invokes the compiled fixed-point batch harness and records all
three values. It is intentionally a smaller, expensive audit complement to the
100k/10k SciPy campaign in `validate_nig_runtime.py`.
"""

from __future__ import annotations

import argparse
import json
import math
import subprocess
from pathlib import Path

import mpmath as mp


ROOT = Path(__file__).resolve().parents[1]
BINARY = ROOT / "target/release/examples/nig_batch"
SCALE = 10**12

# s, k, r, q, t, alpha, beta, delta/year
CASES = [
    (100, 100, 0.05, 0.02, 1.0, 10, -2, 0.2),
    (80, 100, 0.03, 0.01, 0.5, 8, -3, 0.4),
    (130, 100, -0.01, 0.04, 2.0, 12, 2, 0.3),
    (100, 115, -0.04, 0.08, 1.5, 2.5, -0.8, 0.25),
    (100, 85, 0.12, -0.03, 0.75, 4, 1, 0.5),
    (250, 250, 0.0, 0.0, 0.25, 100, -20, 1.0),
]


def m(value: float | int) -> mp.mpf:
    return mp.mpf(str(value))


def setup(case: tuple[float, ...]) -> dict[str, mp.mpf | bool]:
    spot, strike, rate, dividend, time, alpha, beta, delta_py = map(m, case)
    elapsed = delta_py * time
    gamma = mp.sqrt(alpha * alpha - beta * beta)
    gamma_one = mp.sqrt(alpha * alpha - (beta + 1) ** 2)
    correction = elapsed * (gamma_one - gamma)
    discounted_spot = spot * mp.exp(-dividend * time)
    discounted_strike = strike * mp.exp(-rate * time)
    kappa = mp.log(spot / strike) + (rate - dividend) * time + correction
    return {
        "spot": spot,
        "strike": strike,
        "rate": rate,
        "dividend": dividend,
        "time": time,
        "alpha": alpha,
        "beta": beta,
        "elapsed": elapsed,
        "gamma": gamma,
        "gamma_one": gamma_one,
        "correction": correction,
        "discounted_spot": discounted_spot,
        "discounted_strike": discounted_strike,
        "threshold": -kappa,
        "call_is_otm": discounted_spot <= discounted_strike,
    }


def direct_density_price(data: dict, nodes: mp.matrix, weights: mp.matrix) -> tuple[mp.mpf, mp.mpf]:
    alpha = data["alpha"]
    beta = data["beta"]
    elapsed = data["elapsed"]
    gamma = data["gamma"]
    threshold = data["threshold"]
    call_is_otm = data["call_is_otm"]
    base_scale = mp.sqrt(elapsed * alpha**2 / gamma**3)
    if call_is_otm:
        tilted_scale = mp.sqrt(elapsed * alpha**2 / data["gamma_one"] ** 3)
        base_scale = max(base_scale, tilted_scale)
    scale = 4 * base_scale

    integral = mp.mpf(0)
    for node, weight in zip(nodes, weights):
        t = (node + 1) / 2
        y = scale * t / (1 - t)
        x = threshold + y if call_is_otm else threshold - y
        omega = mp.sqrt(elapsed**2 + x**2)
        density = (
            alpha
            * elapsed
            / (mp.pi * omega)
            * mp.besselk(1, alpha * omega)
            * mp.exp(elapsed * gamma + beta * x)
        )
        payoff = mp.expm1(y) if call_is_otm else -mp.expm1(-y)
        jacobian = scale / (1 - t) ** 2 / 2
        integral += weight * payoff * density * jacobian

    otm = data["discounted_strike"] * integral
    if call_is_otm:
        return (
            otm,
            otm + data["discounted_strike"] - data["discounted_spot"],
        )
    return (
        otm + data["discounted_spot"] - data["discounted_strike"],
        otm,
    )


def lewis_price(data: dict) -> tuple[mp.mpf, mp.mpf]:
    alpha = data["alpha"]
    beta = data["beta"]
    elapsed = data["elapsed"]
    gamma = data["gamma"]
    correction = data["correction"]
    log_discounted_moneyness = mp.log(
        data["discounted_spot"] / data["discounted_strike"]
    )

    def characteristic(z: mp.mpc) -> mp.mpc:
        return mp.exp(
            1j * z * correction
            + elapsed * (gamma - mp.sqrt(alpha**2 - (beta + 1j * z) ** 2))
        )

    def integrand(u: mp.mpf) -> mp.mpf:
        return mp.re(
            mp.exp(1j * u * log_discounted_moneyness)
            * characteristic(u - mp.mpf("0.5") * 1j)
        ) / (u * u + mp.mpf("0.25"))

    cutoff = 50 / elapsed
    points = [mp.mpf(0), mp.mpf(1)]
    while points[-1] < cutoff:
        points.append(min(cutoff, points[-1] * 2))
    integral = mp.quad(integrand, points)
    call = data["discounted_spot"] - mp.sqrt(
        data["discounted_spot"] * data["discounted_strike"]
    ) / mp.pi * integral
    put = call + data["discounted_strike"] - data["discounted_spot"]
    return call, put


def raw(value: float) -> int:
    return round(value * SCALE)


def runtime_prices() -> list[tuple[int, int, int, int]]:
    lines = []
    for spot, strike, rate, dividend, time, alpha, beta, delta in CASES:
        # The audit asks the kernel to return even when its conservative local
        # allowance is wider than the production request; actual error is then
        # measured against both independent oracles below.
        requested = max(1, raw(max(spot, strike) * 1e-2))
        lines.append(
            " ".join(
                str(value)
                for value in (
                    raw(spot),
                    raw(strike),
                    raw(rate),
                    raw(dividend),
                    raw(time),
                    raw(alpha),
                    raw(beta),
                    raw(delta),
                    requested,
                )
            )
        )
    process = subprocess.run(
        [str(BINARY)],
        input="\n".join(lines) + "\n",
        text=True,
        capture_output=True,
        check=True,
    )
    results = []
    for line in process.stdout.splitlines():
        fields = line.split()
        if not fields or fields[0] != "OK":
            raise RuntimeError(f"fixed-point audit quote rejected: {line}")
        results.append(tuple(map(int, fields[1:])))
    return results


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--dps", type=int, default=50)
    parser.add_argument(
        "--output",
        type=Path,
        default=ROOT / "benchmark/nig_independent_oracle_report.json",
    )
    parser.add_argument("--skip-build", action="store_true")
    args = parser.parse_args()
    mp.mp.dps = args.dps

    if not args.skip_build:
        subprocess.run(
            [
                "cargo",
                "build",
                "--release",
                "--no-default-features",
                "--features",
                "nig",
                "--example",
                "nig_batch",
            ],
            cwd=ROOT,
            check=True,
        )

    nodes48, weights48 = mp.gauss_quadrature(48, "legendre")
    nodes64, weights64 = mp.gauss_quadrature(64, "legendre")
    fixed = runtime_prices()
    rows = []
    maxima = {
        "density_48_vs_64": 0.0,
        "density_vs_fourier": 0.0,
        "fixed_vs_density": 0.0,
        "fixed_vs_fourier": 0.0,
    }

    for index, (case, fixed_quote) in enumerate(zip(CASES, fixed)):
        data = setup(case)
        density48 = direct_density_price(data, nodes48, weights48)
        density64 = direct_density_price(data, nodes64, weights64)
        fourier = lewis_price(data)
        fixed_call, fixed_put, allowance, tier = fixed_quote
        fixed_values = (mp.mpf(fixed_call) / SCALE, mp.mpf(fixed_put) / SCALE)

        density_convergence = max(abs(a - b) for a, b in zip(density48, density64))
        oracle_difference = max(abs(a - b) for a, b in zip(density64, fourier))
        fixed_density = max(abs(a - b) for a, b in zip(fixed_values, density64))
        fixed_fourier = max(abs(a - b) for a, b in zip(fixed_values, fourier))
        maxima["density_48_vs_64"] = max(maxima["density_48_vs_64"], float(density_convergence))
        maxima["density_vs_fourier"] = max(maxima["density_vs_fourier"], float(oracle_difference))
        maxima["fixed_vs_density"] = max(maxima["fixed_vs_density"], float(fixed_density))
        maxima["fixed_vs_fourier"] = max(maxima["fixed_vs_fourier"], float(fixed_fourier))
        rows.append(
            {
                "index": index,
                "input": dict(zip(("spot", "strike", "rate", "dividend", "time", "alpha", "beta", "delta_per_year"), case)),
                "density_call": mp.nstr(density64[0], args.dps),
                "density_put": mp.nstr(density64[1], args.dps),
                "fourier_call": mp.nstr(fourier[0], args.dps),
                "fourier_put": mp.nstr(fourier[1], args.dps),
                "fixed_call": fixed_call / SCALE,
                "fixed_put": fixed_put / SCALE,
                "returned_max_abs_error": allowance / SCALE,
                "tier": tier,
                "density_48_vs_64": float(density_convergence),
                "density_vs_fourier": float(oracle_difference),
                "fixed_vs_density": float(fixed_density),
            }
        )

    report = {
        "schema": 1,
        "mpmath_dps": args.dps,
        "methods": [
            "direct NIG Bessel-density OTM integration, arbitrary-precision Gauss-Legendre 64",
            "Lewis characteristic-function inversion",
            "SolMath fixed-point 15/7 runtime",
        ],
        "cases": len(rows),
        "maxima": maxima,
        "rows": rows,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
