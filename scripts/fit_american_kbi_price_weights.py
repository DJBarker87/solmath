#!/usr/bin/env python3
"""Reproduce KBI's fixed nine-node QdFp empirical cubature weights.

The fit changes only global integration weights.  Runtime nodes, boundary
reconstruction, normal kernels, and all six quote inputs remain live on-chain.
No option price or contract-specific coefficient is embedded in the program.
"""

from __future__ import annotations

import argparse
import json
import math

import numpy as np
from scipy.optimize import lsq_linear
from scipy.special import ndtr

import american_quantlib_reference as base
from generate_american_kbi_data import KBI_QDFP_PRICE_WEIGHTS
from american_kbi_reference import SmoothPastingKbi, european_put


POWER = 2.25
ORDER = 9
RIDGE = 1.0e-3


def boundary_at(samples: np.ndarray, boundary) -> np.ndarray:
    left = np.searchsorted(boundary.times, samples, side="right") - 1
    left = np.clip(left, 0, boundary.times.size - 2)
    fraction = (
        (samples - boundary.times[left])
        / (boundary.times[left + 1] - boundary.times[left])
    )
    return np.exp(
        np.log(boundary.values[left])
        + fraction
        * (np.log(boundary.values[left + 1]) - np.log(boundary.values[left]))
    )


def premium_row(
    spot: float,
    boundary,
    rate: float,
    yield_rate: float,
    sigma: float,
    y: np.ndarray,
) -> np.ndarray:
    maturity = float(boundary.times[-1])
    lag = maturity * y**POWER
    boundary_samples = boundary_at(maturity - lag, boundary)
    standard_deviation = sigma * np.sqrt(lag)
    d1 = (
        np.log(spot / boundary_samples)
        + (rate - yield_rate + 0.5 * sigma * sigma) * lag
    ) / standard_deviation
    d2 = d1 - standard_deviation
    return (
        rate * maturity * np.exp(-rate * lag) * ndtr(-d2)
        - yield_rate
        * maturity
        * spot
        * np.exp(-yield_rate * lag)
        * ndtr(-d1)
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()

    contracts = base.contracts(0xA3E1, 48)
    if len(contracts) != 48:
        raise RuntimeError(f"expected 48 training contracts, found {len(contracts)}")

    method = SmoothPastingKbi(
        nodes=18,
        quadrature_order=2,
        grading=2.0,
        product_basis="gauss-y",
        root_solver="newton",
        newton_steps=2,
        price_mesh="global",
        price_power=POWER,
        late_newton_steps=1,
        newton_cutover=2,
        derivative_mode="full",
        price_boundary_interp="log",
        boundary_normal="exact",
        price_normal="exact",
        boundary_order=6,
        price_order=ORDER,
        third_predictor_alpha=0.925,
    )
    gauss_x, gauss_weight = np.polynomial.legendre.leggauss(ORDER)
    y = 0.5 * (gauss_x + 1.0)
    gaussian_weights = (
        POWER * y ** (POWER - 1.0) * 0.5 * gauss_weight
    )
    spots = np.exp(np.linspace(-0.75, 0.75, 33))
    qdfp = base.QdFpSurfacePricer()
    rows: list[np.ndarray] = []
    targets: list[float] = []

    for contract in contracts:
        maturity = contract.days / 365.0
        put_boundary = method.boundary(
            maturity, contract.r, contract.q, contract.sigma
        )
        call_boundary = method.boundary(
            maturity, contract.q, contract.r, contract.sigma
        )
        put_truth = qdfp.surface(spots, contract, contract.days, False)
        call_truth = qdfp.surface(spots, contract, contract.days, True)

        # Exact call-put duality maps the call to a put with spot 1/S.  Scale
        # each dual residual by S so least squares minimizes original call
        # dollars rather than dual normalized dollars.
        legs = (
            (
                put_boundary,
                contract.r,
                contract.q,
                spots,
                put_truth,
                np.ones_like(spots),
            ),
            (
                call_boundary,
                contract.q,
                contract.r,
                1.0 / spots,
                call_truth / spots,
                spots,
            ),
        )
        for boundary, rate, yield_rate, leg_spots, truth, scale in legs:
            for spot, exact, row_scale in zip(leg_spots, truth, scale):
                if spot <= boundary.values[-1]:
                    continue
                european = european_put(
                    float(spot), maturity, rate, yield_rate, contract.sigma
                )
                rows.append(
                    float(row_scale)
                    * premium_row(
                        float(spot),
                        boundary,
                        rate,
                        yield_rate,
                        contract.sigma,
                        y,
                    )
                )
                targets.append(float(row_scale) * (float(exact) - european))

    matrix = np.asarray(rows, dtype=np.float64)
    target = np.asarray(targets, dtype=np.float64)
    root_ridge = math.sqrt(RIDGE)
    augmented_matrix = np.vstack(
        (
            matrix,
            root_ridge * np.eye(ORDER),
            100.0 * np.ones((1, ORDER)),
        )
    )
    augmented_target = np.concatenate(
        (target, root_ridge * gaussian_weights, np.asarray([100.0]))
    )
    result = lsq_linear(
        augmented_matrix,
        augmented_target,
        bounds=(0.0, np.inf),
        tol=1.0e-14,
        lsmr_tol=1.0e-14,
        max_iter=2_000,
    )
    weights = result.x / np.sum(result.x)
    maximum_difference = float(np.max(np.abs(weights - KBI_QDFP_PRICE_WEIGHTS)))
    print(json.dumps({
        "training_contracts": len(contracts),
        "training_rows": int(matrix.shape[0]),
        "ridge": RIDGE,
        "weights": weights.tolist(),
        "maximum_committed_difference": maximum_difference,
    }, indent=2))
    if args.check and maximum_difference > 5.0e-15:
        raise SystemExit("committed KBI empirical weights do not reproduce")


if __name__ == "__main__":
    main()
