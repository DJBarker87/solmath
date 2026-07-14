#!/usr/bin/env python3
"""Regenerate the scaled-K1 Chebyshev coefficients embedded in src/nig.rs."""

from __future__ import annotations

import hashlib
import json

import numpy as np
from numpy.polynomial import Chebyshev, Polynomial
from scipy.special import k1e


SCALE = 10**12
SAMPLES = 200_000


def fit_power(x: np.ndarray, y: np.ndarray, degree: int) -> list[int]:
    polynomial = Chebyshev.fit(x, y, degree, domain=[-1, 1]).convert(kind=Polynomial)
    return [round(float(value) * SCALE) for value in polynomial.coef]


def fit_unit_interval(x: np.ndarray, y: np.ndarray, degree: int) -> list[int]:
    polynomial = Chebyshev.fit(x, y, degree, domain=[0, 1]).convert(kind=Polynomial)
    return [round(float(value) * SCALE) for value in polynomial.coef]


def main() -> None:
    chebyshev_nodes = np.cos((np.arange(SAMPLES) + 0.5) / SAMPLES * np.pi)
    edges = [0.0, 2**-8]
    while edges[-1] < 1.0:
        edges.append(edges[-1] * 2)

    small = []
    for low, high in zip(edges[:-1], edges[1:]):
        z = (chebyshev_nodes + 1) / 2 * (high - low) + low
        target = z * k1e(z)
        target[z < 1e-15] = 1.0
        small.append(fit_power(chebyshev_nodes, target, 6))

    reciprocal = (chebyshev_nodes + 1) / 2
    z = np.divide(1.0, reciprocal, out=np.full_like(reciprocal, np.inf), where=reciprocal != 0)
    target = np.sqrt(z) * k1e(z)
    large = fit_unit_interval(reciprocal, target, 8)
    payload = {"scale": SCALE, "samples": SAMPLES, "small": small, "large": large}
    canonical = json.dumps(payload, sort_keys=True, separators=(",", ":")).encode()
    payload["sha256"] = hashlib.sha256(canonical).hexdigest()
    print(json.dumps(payload, indent=2))


if __name__ == "__main__":
    main()
