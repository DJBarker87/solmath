#!/usr/bin/env python3
"""Regenerate the independent BVN/Phi2 audit corpora.

The 22,500-case BVN corpus exactly preserves the 2026-07-11 audit seed,
stratification, adaptive-quadrature tolerances, and row order.  The separate
10,000-case Phi2 corpus uses five fixed correlations with off-grid thresholds.
"""

import argparse
import json
import math
import random
from concurrent.futures import ProcessPoolExecutor

from scipy.integrate import quad
from scipy.special import ndtr


SCALE = 10**12


def reference(row: tuple[float, float, float]) -> dict[str, int]:
    a, b, rho = row
    if rho >= 1.0:
        probability = float(ndtr(min(a, b)))
    elif rho <= -1.0:
        probability = max(0.0, float(ndtr(a) + ndtr(b) - 1.0))
    elif rho == 0.0:
        probability = float(ndtr(a) * ndtr(b))
    else:
        alpha = math.asin(rho)

        def integrand(theta: float) -> float:
            sin_theta = math.sin(theta)
            cos_theta = math.cos(theta)
            exponent = -(
                a * a - 2.0 * a * b * sin_theta + b * b
            ) / (2.0 * cos_theta * cos_theta)
            return 0.0 if exponent < -745.0 else math.exp(exponent)

        integral = quad(
            integrand,
            0.0,
            alpha,
            epsabs=2e-15,
            epsrel=2e-14,
            limit=250,
        )[0]
        probability = float(ndtr(a) * ndtr(b) + integral / (2.0 * math.pi))
    probability = min(1.0, max(0.0, probability))
    return {
        "a": round(a * SCALE),
        "b": round(b * SCALE),
        "rho": round(rho * SCALE),
        "expected": round(probability * SCALE),
    }


def build_inputs() -> tuple[list[tuple[float, float, float]], list[tuple[float, float, float]]]:
    rng = random.Random(20260710)
    bvn: list[tuple[float, float, float]] = []
    rho_buckets = [
        (-0.99, -0.95),
        (-0.95, -0.8),
        (-0.8, -0.2),
        (-0.2, 0.2),
        (0.2, 0.8),
        (0.8, 0.95),
        (0.95, 0.99),
    ]
    for low, high in rho_buckets:
        for _ in range(2_500):
            bvn.append(
                (rng.uniform(-4.0, 4.0), rng.uniform(-4.0, 4.0), rng.uniform(low, high))
            )

    # Near-singular unequal and equal-threshold boundary layers.
    for sign in (-1, 1):
        for exponent in (6, 8, 10, 12):
            rho = sign * (1.0 - 10.0 ** (-exponent))
            for _ in range(500):
                a = rng.uniform(-3.5, 3.5)
                b = (-a if sign < 0 else a) + rng.uniform(-2e-3, 2e-3)
                bvn.append((a, b, rho))

    # Exact endpoints and their analytic identities.
    for sign in (-1, 1):
        for _ in range(500):
            bvn.append((rng.uniform(-4.0, 4.0), rng.uniform(-4.0, 4.0), float(sign)))

    phi2: list[tuple[float, float, float]] = []
    for rho in (-0.9, -0.5, 0.0, 0.5, 0.9):
        for _ in range(2_000):
            phi2.append((rng.uniform(-4.0, 4.0), rng.uniform(-4.0, 4.0), rho))
    return bvn, phi2


def write_corpus(path: str, rows: list[dict[str, int]], kind: str) -> None:
    with open(path, "w", encoding="utf-8") as output:
        json.dump(
            {
                "meta": {
                    "kind": kind,
                    "vectors": len(rows),
                    "seed": 20260710,
                    "scale": SCALE,
                    "reference": "SciPy ndtr plus adaptive angular quadrature",
                    "scipy_quad_epsabs": 2e-15,
                    "scipy_quad_epsrel": 2e-14,
                },
                "vectors": rows,
            },
            output,
            separators=(",", ":"),
        )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--bvn-output", default="/tmp/bvn-audit-vectors.json")
    parser.add_argument("--phi2-output", default="/tmp/phi2-audit-vectors.json")
    parser.add_argument("--workers", type=int, default=None)
    args = parser.parse_args()

    bvn_inputs, phi2_inputs = build_inputs()
    with ProcessPoolExecutor(max_workers=args.workers) as executor:
        bvn_rows = list(executor.map(reference, bvn_inputs, chunksize=64))
    with ProcessPoolExecutor(max_workers=args.workers) as executor:
        phi2_rows = list(executor.map(reference, phi2_inputs, chunksize=64))

    write_corpus(args.bvn_output, bvn_rows, "bvn")
    write_corpus(args.phi2_output, phi2_rows, "phi2_off_grid")
    print(f"bvn={len(bvn_rows)} phi2={len(phi2_rows)}")


if __name__ == "__main__":
    main()
