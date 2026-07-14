#!/usr/bin/env python3
"""Test a Kim Boundary Integration pricer against QuantLib QdFp.

The experiment deliberately mirrors the constraints of the intended Solana
implementation:

* only the six option inputs are live inputs;
* one normalized American-put kernel prices both puts and calls (by duality);
* the exercise boundary is reconstructed from Kim's nonlinear boundary equation;
* the final price is the European value plus the early-exercise premium.

This first implementation is a floating-point accuracy oracle.  It reports the
number of CDF/kernel evaluations as well as price errors so that numerically
accurate configurations which cannot plausibly meet the CU budget are visible
immediately.  Composite Gauss-Legendre integration is used on a graded
time-to-expiry mesh; boundary values inside each interval are interpolated
linearly, including the as-yet unknown right endpoint during each scalar solve.
"""

from __future__ import annotations

import argparse
import json
import math
import pathlib
from dataclasses import asdict, dataclass

import numpy as np
from scipy.optimize import brentq
from scipy.special import ndtr

import american_quantlib_reference as base


@dataclass
class Work:
    """Operation counts for a boundary construction and a single price."""

    residual_evaluations: int = 0
    boundary_kernel_points: int = 0
    price_kernel_points: int = 0

    @property
    def cdf_evaluations(self) -> int:
        # Each kernel point evaluates both d1 and d2.
        return 2 * (self.boundary_kernel_points + self.price_kernel_points)


@dataclass(frozen=True)
class Boundary:
    times: np.ndarray
    values: np.ndarray
    work: Work


def european_put(spot: np.ndarray | float, maturity: float, r: float, q: float,
                 sigma: float) -> np.ndarray | float:
    """Normalized Black-Scholes-Merton put with strike one."""
    x = np.asarray(spot, dtype=np.float64)
    if maturity <= 0.0:
        result = np.maximum(1.0 - x, 0.0)
    else:
        root_t = math.sqrt(maturity)
        standard_deviation = sigma * root_t
        d1 = (np.log(x) + (r - q + 0.5 * sigma * sigma) * maturity) / standard_deviation
        d2 = d1 - standard_deviation
        result = math.exp(-r * maturity) * ndtr(-d2) - x * math.exp(-q * maturity) * ndtr(-d1)
    if np.ndim(spot) == 0:
        return float(result)
    return result


def expiry_boundary(r: float, q: float) -> float:
    """Right limit B(0+)/K of the put exercise boundary."""
    if q > r and q > 0.0:
        return max(r / q, 0.0)
    return 1.0


def hermite_norm_cdf_pdf(
    value: np.ndarray | float, step: float
) -> tuple[np.ndarray | float, np.ndarray | float]:
    """Cubic-Hermite normal CDF and its analytic derivative on a fixed grid."""
    x = np.asarray(value, dtype=np.float64)
    absolute = np.abs(x)
    clipped = np.minimum(absolute, 8.0)
    intervals = round(8.0 / step)
    index = np.minimum((clipped / step).astype(np.int64), intervals - 1)
    left = index * step
    coordinate = (clipped - left) / step
    y0 = ndtr(left)
    y1 = ndtr(left + step)
    inverse_root_two_pi = 1.0 / math.sqrt(2.0 * math.pi)
    p0 = inverse_root_two_pi * np.exp(-0.5 * left * left)
    p1 = inverse_root_two_pi * np.exp(-0.5 * (left + step) ** 2)
    a = 2.0 * y0 - 2.0 * y1 + step * (p0 + p1)
    b = -3.0 * y0 + 3.0 * y1 - step * (2.0 * p0 + p1)
    c = step * p0
    cdf_positive = ((a * coordinate + b) * coordinate + c) * coordinate + y0
    pdf = ((3.0 * a * coordinate + 2.0 * b) * coordinate + c) / step
    cdf = np.where(x >= 0.0, cdf_positive, 1.0 - cdf_positive)
    cdf = np.where(x > 8.0, 1.0, np.where(x < -8.0, 0.0, cdf))
    pdf = np.where(absolute > 8.0, 0.0, pdf)
    if np.ndim(value) == 0:
        return float(cdf), float(pdf)
    return cdf, pdf


class KimBoundaryIntegration:
    """Product-in-time discretization of Kim's boundary equation."""

    def __init__(
        self,
        nodes: int,
        quadrature_order: int,
        grading: float,
        product_basis: str = "linear-t",
        fh_degree: int = 3,
        root_solver: str = "brent",
        newton_steps: int = 4,
        price_mesh: str = "boundary",
        price_power: float = 2.0,
        late_newton_steps: int | None = None,
        newton_cutover: int = 0,
        derivative_mode: str = "full",
        price_boundary_interp: str = "linear",
        boundary_normal: str = "exact",
        price_normal: str = "exact",
        collocation_stride: int = 1,
        boundary_order: int = 4,
        price_order: int = 12,
        third_predictor_alpha: float = 1.0,
    ) -> None:
        if nodes < 2 or quadrature_order < 1 or grading < 1.0:
            raise ValueError("nodes >= 2, quadrature-order >= 1, grading >= 1 required")
        self.nodes = nodes
        self.quadrature_order = quadrature_order
        self.grading = grading
        self.product_basis = product_basis
        self.fh_degree = fh_degree
        self.root_solver = root_solver
        self.newton_steps = newton_steps
        self.price_mesh = price_mesh
        self.price_power = price_power
        self.late_newton_steps = late_newton_steps
        self.newton_cutover = newton_cutover
        self.derivative_mode = derivative_mode
        self.price_boundary_interp = price_boundary_interp
        self.boundary_normal = boundary_normal
        self.price_normal = price_normal
        self.collocation_stride = collocation_stride
        self.boundary_order = boundary_order
        self.price_order = price_order
        self.third_predictor_alpha = third_predictor_alpha
        self.derivative_samples: list[tuple[float, ...]] = []
        self.gauss_x, self.gauss_w = np.polynomial.legendre.leggauss(quadrature_order)

    @staticmethod
    def _kernel(
        current_boundary: float,
        lag: np.ndarray,
        past_boundary: np.ndarray,
        r: float,
        q: float,
        sigma: float,
    ) -> np.ndarray:
        root_lag = np.sqrt(lag)
        standard_deviation = sigma * root_lag
        d1 = (
            np.log(current_boundary / past_boundary)
            + (r - q + 0.5 * sigma * sigma) * lag
        ) / standard_deviation
        d2 = d1 - standard_deviation
        return (
            r * np.exp(-r * lag) * ndtr(-d2)
            - q * current_boundary * np.exp(-q * lag) * ndtr(-d1)
        )

    def _prefix_quadrature(
        self,
        times: np.ndarray,
        values: np.ndarray,
        right_index: int,
        candidate: float,
    ) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Quadrature lags, weights and B(s) for [0, times[right_index]]."""
        left = times[:right_index]
        right = times[1:right_index + 1]
        half_width = 0.5 * (right - left)
        midpoint = 0.5 * (right + left)
        sample_times = midpoint[:, None] + half_width[:, None] * self.gauss_x[None, :]
        weights = half_width[:, None] * self.gauss_w[None, :]

        right_values = np.concatenate((values[1:right_index], np.asarray([candidate])))
        fraction = (sample_times - left[:, None]) / (right - left)[:, None]
        boundary_samples = (
            values[:right_index, None]
            + fraction * (right_values[:, None] - values[:right_index, None])
        )
        lag = times[right_index] - sample_times
        return lag.ravel(), weights.ravel(), boundary_samples.ravel()

    def boundary(self, maturity: float, r: float, q: float, sigma: float) -> Boundary:
        if maturity <= 0.0:
            work = Work()
            value = expiry_boundary(r, q)
            return Boundary(np.asarray([0.0]), np.asarray([value]), work)
        coordinate = np.linspace(0.0, 1.0, self.nodes + 1)
        times = maturity * coordinate ** self.grading
        values = np.empty(self.nodes + 1, dtype=np.float64)
        values[0] = expiry_boundary(r, q)
        work = Work()

        # With no interest-rate benefit from receiving the strike early, the
        # normalized put has no non-trivial stopping region.  Keep this exact
        # branch out of the ill-conditioned boundary equation.
        if r <= 1.0e-14:
            values.fill(0.0)
            return Boundary(times, values, work)

        for index in range(1, self.nodes + 1):
            t = float(times[index])

            def residual(candidate: float) -> float:
                lag, weights, boundary_samples = self._prefix_quadrature(
                    times, values, index, candidate
                )
                integral = float(np.dot(
                    weights,
                    self._kernel(candidate, lag, boundary_samples, r, q, sigma),
                ))
                work.residual_evaluations += 1
                work.boundary_kernel_points += lag.size
                return 1.0 - candidate - european_put(candidate, t, r, q, sigma) - integral

            upper = min(float(values[index - 1]), expiry_boundary(r, q))
            upper = max(upper, 1.0e-12)
            # The integral equation can be nearly tangent at short maturities.
            # Search from the economically relevant upper branch downwards and
            # take the first genuine sign change.
            probes = np.concatenate((
                upper * (1.0 - np.geomspace(1.0e-11, 0.35, 28)),
                np.geomspace(max(upper * 0.65, 1.0e-10), 1.0e-10, 28),
            ))
            probes = np.unique(np.clip(probes, 1.0e-12, upper))[::-1]
            probe_values = [residual(float(point)) for point in probes]
            root: float | None = None
            for high_index in range(len(probes) - 1):
                high = float(probes[high_index])
                low = float(probes[high_index + 1])
                f_high = probe_values[high_index]
                f_low = probe_values[high_index + 1]
                if f_high == 0.0:
                    root = high
                    break
                if f_high * f_low < 0.0:
                    root = brentq(residual, low, high, xtol=2.0e-13, rtol=2.0e-13)
                    break
            if root is None:
                best = int(np.argmin(np.abs(probe_values)))
                if abs(probe_values[best]) > 2.0e-7:
                    raise RuntimeError(
                        f"failed to bracket boundary at node {index}/{self.nodes}: "
                        f"t={t:.8g}, best B={probes[best]:.8g}, residual={probe_values[best]:.3g}"
                    )
                root = float(probes[best])
            values[index] = min(root, upper)
        return Boundary(times, values, work)

    def _full_quadrature(self, boundary: Boundary) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        index = boundary.times.size - 1
        if self.price_mesh == "boundary":
            return self._prefix_quadrature(
                boundary.times, boundary.values, index, float(boundary.values[-1])
            )
        maturity = float(boundary.times[-1])
        if self.price_mesh == "global":
            gauss_x, gauss_w = np.polynomial.legendre.leggauss(self.price_order)
            y = 0.5 * (gauss_x + 1.0)
            unit_weights = 0.5 * gauss_w
            lag = maturity * y ** self.price_power
            flat_samples = maturity - lag
            weights = (
                maturity
                * self.price_power
                * y ** (self.price_power - 1.0)
                * unit_weights
            )
        else:
            flat_samples = None
        coordinate = np.linspace(0.0, 1.0, index + 1)
        if self.price_mesh == "global":
            pass
        elif self.price_mesh == "uniform":
            normalized = coordinate
        elif self.price_mesh == "valuation":
            normalized = 1.0 - (1.0 - coordinate) ** self.price_power
        elif self.price_mesh == "double":
            normalized = 0.5 * (1.0 - np.cos(math.pi * coordinate))
        else:
            raise ValueError(f"unsupported price mesh: {self.price_mesh}")
        if self.price_mesh != "global":
            price_times = maturity * normalized
            left = price_times[:-1]
            right = price_times[1:]
            half_width = 0.5 * (right - left)
            midpoint = 0.5 * (right + left)
            samples = midpoint[:, None] + half_width[:, None] * self.gauss_x[None, :]
            weights = half_width[:, None] * self.gauss_w[None, :]
            flat_samples = samples.ravel()
        boundary_interval = np.searchsorted(
            boundary.times, flat_samples, side="right"
        ) - 1
        boundary_interval = np.clip(boundary_interval, 0, index - 1)
        interval_left = boundary.times[boundary_interval]
        interval_right = boundary.times[boundary_interval + 1]
        fraction = (flat_samples - interval_left) / (interval_right - interval_left)
        if self.price_boundary_interp == "log":
            log_values = np.log(boundary.values)
            boundary_samples = np.exp(
                log_values[boundary_interval]
                + fraction
                * (log_values[boundary_interval + 1] - log_values[boundary_interval])
            )
        else:
            boundary_samples = (
                boundary.values[boundary_interval]
                + fraction
                * (boundary.values[boundary_interval + 1] - boundary.values[boundary_interval])
            )
        return maturity - flat_samples, weights.ravel(), boundary_samples

    def put_prices(
        self,
        spots: np.ndarray,
        maturity: float,
        r: float,
        q: float,
        sigma: float,
        boundary: Boundary | None = None,
    ) -> tuple[np.ndarray, Boundary, int]:
        spots = np.asarray(spots, dtype=np.float64)
        if maturity <= 0.0:
            b = boundary or self.boundary(maturity, r, q, sigma)
            return np.maximum(1.0 - spots, 0.0), b, 0
        if r <= 1.0e-14:
            b = boundary or self.boundary(maturity, r, q, sigma)
            return np.asarray(european_put(spots, maturity, r, q, sigma)), b, 0
        b = boundary or self.boundary(maturity, r, q, sigma)
        lag, weights, boundary_samples = self._full_quadrature(b)
        root_lag = np.sqrt(lag)[None, :]
        standard_deviation = sigma * root_lag
        spot_matrix = spots[:, None]
        d1 = (
            np.log(spot_matrix / boundary_samples[None, :])
            + (r - q + 0.5 * sigma * sigma) * lag[None, :]
        ) / standard_deviation
        d2 = d1 - standard_deviation
        if self.price_normal == "hermite25":
            cdf_d2, _ = hermite_norm_cdf_pdf(-d2, 0.25)
            cdf_d1, _ = hermite_norm_cdf_pdf(-d1, 0.25)
        else:
            cdf_d2 = ndtr(-d2)
            cdf_d1 = ndtr(-d1)
        rate_premium = r * np.sum(
            weights[None, :] * np.exp(-r * lag)[None, :] * cdf_d2, axis=1
        )
        dividend_premium = q * spots * np.sum(
            weights[None, :] * np.exp(-q * lag)[None, :] * cdf_d1, axis=1
        )
        prices = np.asarray(european_put(spots, maturity, r, q, sigma)) + rate_premium - dividend_premium
        intrinsic = np.maximum(1.0 - spots, 0.0)
        # The premium representation is a continuation value.  Apply the
        # stopping decision at valuation time exactly.
        prices = np.where(spots <= b.values[-1], intrinsic, prices)
        prices = np.maximum(prices, intrinsic)
        price_points = int(lag.size)
        return prices, b, price_points


class SmoothPastingKbi(KimBoundaryIntegration):
    """One-dimensional weakly-singular smooth-pasting reformulation.

    Differentiating the early-exercise-premium representation with respect to
    spot and imposing delta = -1 removes the tangency of Kim's value-matching
    residual.  The remaining 1/sqrt(t-s) singularity is integrated exactly
    against a piecewise-linear kernel.  Thus one boundary residual uses one
    kernel sample per time node rather than an inner normal-CDF quadrature.
    """

    @staticmethod
    def _product_weights(times: np.ndarray, right_index: int) -> np.ndarray:
        """Weights for integral g(s)/sqrt(t_i-s) ds with linear hat functions."""
        t = float(times[right_index])
        weights = np.zeros(right_index + 1, dtype=np.float64)
        for interval in range(right_index):
            a = float(times[interval])
            b = float(times[interval + 1])
            width = b - a
            lag_a = t - a
            lag_b = t - b
            root_a = math.sqrt(lag_a)
            root_b = math.sqrt(max(lag_b, 0.0))
            cubic_difference = lag_a * root_a - lag_b * root_b
            root_difference = root_a - root_b
            left_weight = (
                (2.0 / 3.0) * cubic_difference
                - 2.0 * lag_b * root_difference
            ) / width
            right_weight = (
                2.0 * lag_a * root_difference
                - (2.0 / 3.0) * cubic_difference
            ) / width
            weights[interval] += left_weight
            weights[interval + 1] += right_weight
        return weights

    @staticmethod
    def _trapezoid_weights(times: np.ndarray, right_index: int) -> np.ndarray:
        weights = np.zeros(right_index + 1, dtype=np.float64)
        widths = np.diff(times[:right_index + 1])
        weights[:-1] += 0.5 * widths
        weights[1:] += 0.5 * widths
        return weights

    @staticmethod
    def _floater_hormann_weights(nodes: np.ndarray, degree: int) -> np.ndarray:
        """Barycentric weights for a pole-free Floater-Hormann interpolant."""
        count = nodes.size
        degree = min(max(degree, 0), count - 1)
        weights = np.zeros(count, dtype=np.float64)
        for k in range(count):
            first = max(0, k - degree)
            last = min(k, count - degree - 1)
            total = 0.0
            for start in range(first, last + 1):
                product = 1.0
                for j in range(start, start + degree + 1):
                    if j != k:
                        product /= abs(float(nodes[k] - nodes[j]))
                total += product
            weights[k] = (-1.0 if (k - degree) % 2 else 1.0) * total
        scale = float(np.max(np.abs(weights)))
        return weights / scale

    @classmethod
    def _floater_hormann_basis(
        cls, nodes: np.ndarray, samples: np.ndarray, degree: int
    ) -> np.ndarray:
        weights = cls._floater_hormann_weights(nodes, degree)
        output = np.empty((samples.size, nodes.size), dtype=np.float64)
        for row, sample in enumerate(samples):
            difference = sample - nodes
            exact = np.flatnonzero(np.abs(difference) <= 4.0e-15)
            if exact.size:
                output[row].fill(0.0)
                output[row, exact[0]] = 1.0
            else:
                terms = weights / difference
                output[row] = terms / float(np.sum(terms))
        return output

    def _rational_product_weights(
        self, times: np.ndarray, right_index: int
    ) -> tuple[np.ndarray, np.ndarray]:
        """Precomputable FH weights in time or the uniform graded coordinate."""
        if self.product_basis == "fh-u":
            nodes_coordinate = np.arange(right_index + 1, dtype=np.float64) / self.nodes
            right_coordinate = right_index / self.nodes
        else:
            nodes_coordinate = times[:right_index + 1] / times[-1]
            right_coordinate = float(nodes_coordinate[-1])
        degree = min(self.fh_degree, right_index)
        # These are coefficient-generation quadratures, not live pricing work.
        # The resulting normalized weights are fixed constants for a chosen
        # node count, grading exponent and FH degree.
        x, w = np.polynomial.legendre.leggauss(128)
        unit = 0.5 * (x + 1.0)
        unit_weights = 0.5 * w
        maturity_at_node = float(times[right_index])

        if self.product_basis == "fh-u":
            regular_coordinate = right_coordinate * unit ** (1.0 / self.grading)
            singular_coordinate = (
                right_coordinate * (1.0 - unit * unit) ** (1.0 / self.grading)
            )
        else:
            regular_coordinate = right_coordinate * unit
            singular_coordinate = right_coordinate * (1.0 - unit * unit)
        regular_basis = self._floater_hormann_basis(
            nodes_coordinate, regular_coordinate, degree
        )
        regular_weights = (
            maturity_at_node * unit_weights @ regular_basis
        )

        # s/t = 1-y^2 exactly cancels the endpoint square-root singularity:
        # ds/sqrt(t-s) = 2 sqrt(t) dy.
        singular_basis = self._floater_hormann_basis(
            nodes_coordinate, singular_coordinate, degree
        )
        singular_weights = (
            2.0 * math.sqrt(maturity_at_node) * unit_weights @ singular_basis
        )
        return singular_weights, regular_weights

    def boundary(self, maturity: float, r: float, q: float, sigma: float) -> Boundary:
        if maturity <= 0.0:
            work = Work()
            value = expiry_boundary(r, q)
            return Boundary(np.asarray([0.0]), np.asarray([value]), work)
        coordinate = np.linspace(0.0, 1.0, self.nodes + 1)
        times = maturity * coordinate ** self.grading
        values = np.empty(self.nodes + 1, dtype=np.float64)
        values[0] = expiry_boundary(r, q)
        work = Work()
        if r <= 1.0e-14:
            values.fill(0.0)
            return Boundary(times, values, work)

        inverse_sigma_root_two_pi = 1.0 / (sigma * math.sqrt(2.0 * math.pi))
        for index in range(1, self.nodes + 1):
            t = float(times[index])
            if self.product_basis == "gauss-y":
                # lag=t*y^2 removes the 1/sqrt(lag) endpoint singularity.
                # A single global Gaussian rule then serves both the regular
                # and singular integrals with parameter-independent geometry.
                gauss_x, gauss_w = np.polynomial.legendre.leggauss(
                    self.boundary_order
                )
                y = 0.5 * (gauss_x + 1.0)
                unit_weights = 0.5 * gauss_w
                lag_samples = t * y * y
                sample_times = t - lag_samples
                sample_left = np.searchsorted(
                    times[:index + 1], sample_times, side="right"
                ) - 1
                sample_left = np.clip(sample_left, 0, index - 1)
                sample_right = sample_left + 1
                interpolation_fraction = (
                    (sample_times - times[sample_left])
                    / (times[sample_right] - times[sample_left])
                )
                singular_weights = 2.0 * math.sqrt(t) * unit_weights
                regular_weights = 2.0 * t * y * unit_weights
            elif self.product_basis in ("fh-u", "fh-t"):
                singular_weights, regular_weights = self._rational_product_weights(
                    times, index
                )
            else:
                singular_weights = self._product_weights(times, index)
                regular_weights = self._trapezoid_weights(times, index)

            def residual_and_derivative(candidate: float) -> tuple[float, float]:
                coefficient_override = None
                coefficient_derivative_override = None
                if self.product_basis == "gauss-y":
                    augmented = np.concatenate(
                        (values[:index], np.asarray([candidate]))
                    )
                    left_values = augmented[sample_left]
                    right_values = augmented[sample_right]
                    fraction = interpolation_fraction
                    if self.price_boundary_interp == "log":
                        log_past = (
                            np.log(left_values)
                            + fraction
                            * (np.log(right_values) - np.log(left_values))
                        )
                        past = np.exp(log_past)
                        past_derivative = np.where(
                            sample_right == index,
                            fraction * past / candidate,
                            0.0,
                        )
                        node_coefficients = q - r / augmented
                        coefficient_override = (
                            node_coefficients[sample_left]
                            + fraction
                            * (
                                node_coefficients[sample_right]
                                - node_coefficients[sample_left]
                            )
                        )
                        coefficient_derivative_override = np.where(
                            sample_right == index,
                            fraction * r / (candidate * candidate),
                            0.0,
                        )
                    else:
                        past = left_values + fraction * (
                            right_values - left_values
                        )
                        past_derivative = np.where(
                            sample_right == index, fraction, 0.0
                        )
                    lag = lag_samples
                    inverse_standard_deviation = 1.0 / (
                        sigma * np.sqrt(lag)
                    )
                    d1 = (
                        np.log(candidate / past)
                        + (r - q + 0.5 * sigma * sigma) * lag
                    ) * inverse_standard_deviation
                    d_candidate = (
                        1.0 / candidate - past_derivative / past
                    ) * inverse_standard_deviation
                else:
                    lag = t - times[:index + 1]
                    past = np.concatenate((values[:index], np.asarray([candidate])))
                    d1 = np.zeros(index + 1, dtype=np.float64)
                    d_candidate = np.zeros(index + 1, dtype=np.float64)
                    positive = lag > 0.0
                    d1[positive] = (
                        np.log(candidate / past[positive])
                        + (r - q + 0.5 * sigma * sigma) * lag[positive]
                    ) / (sigma * np.sqrt(lag[positive]))
                    d_candidate[positive] = 1.0 / (
                        candidate * sigma * np.sqrt(lag[positive])
                    )
                    past_derivative = np.zeros(index + 1, dtype=np.float64)
                    past_derivative[-1] = 1.0
                inverse_root_two_pi = 1.0 / math.sqrt(2.0 * math.pi)
                discount_q = np.exp(-q * lag)
                if self.boundary_normal == "exact":
                    cdf_minus_d1 = ndtr(-d1)
                    density_d1 = inverse_root_two_pi * np.exp(-0.5 * d1 * d1)
                else:
                    step = 0.25 if self.boundary_normal == "hermite25" else 0.5
                    cdf_minus_d1, density_d1 = hermite_norm_cdf_pdf(-d1, step)
                regular = discount_q * cdf_minus_d1
                # S e^-q tau phi(d1) = K e^-r tau phi(d2).
                # Here S=candidate and K=past, so the two density kernels
                # collapse to one shared phi(d1) evaluation.
                coefficient = (
                    coefficient_override
                    if coefficient_override is not None
                    else q - r / past
                )
                singular = discount_q * density_d1 * coefficient
                regular_derivative = (
                    -discount_q * density_d1 * d_candidate
                )
                singular_derivative = (
                    discount_q
                    * density_d1
                    * (
                        -coefficient * d1 * d_candidate
                        + (
                            coefficient_derivative_override
                            if coefficient_derivative_override is not None
                            else (r / (past * past)) * past_derivative
                        )
                    )
                )
                if self.derivative_mode == "diagonal" or self.derivative_mode.startswith("last"):
                    retained = 0 if self.derivative_mode == "diagonal" else int(
                        self.derivative_mode.removeprefix("last")
                    )
                    cutoff = max(0, index - retained)
                    regular_derivative[:cutoff] = 0.0
                    singular_derivative[:cutoff] = 0.0
                d1_european = (
                    math.log(candidate)
                    + (r - q + 0.5 * sigma * sigma) * t
                ) / (sigma * math.sqrt(t))
                work.residual_evaluations += 1
                work.boundary_kernel_points += lag.size
                if self.boundary_normal == "exact":
                    european_cdf = float(ndtr(-d1_european))
                    european_pdf = math.exp(-0.5 * d1_european * d1_european) * inverse_root_two_pi
                else:
                    step = 0.25 if self.boundary_normal == "hermite25" else 0.5
                    european_cdf, european_pdf = hermite_norm_cdf_pdf(-d1_european, step)
                value = (
                    1.0
                    - math.exp(-q * t) * european_cdf
                    - q * float(np.dot(regular_weights, regular))
                    + (1.0 / sigma) * float(np.dot(singular_weights, singular))
                )
                european_derivative = (
                    math.exp(-q * t)
                    * european_pdf
                    / (candidate * sigma * math.sqrt(t))
                )
                derivative = (
                    european_derivative
                    - q * float(np.dot(regular_weights, regular_derivative))
                    + (1.0 / sigma)
                    * float(np.dot(singular_weights, singular_derivative))
                )
                self.derivative_samples.append(
                    (
                        float(index),
                        2.0 * r / (sigma * sigma),
                        2.0 * q / (sigma * sigma),
                        sigma * sigma * float(times[-1]),
                        candidate,
                        value,
                        derivative,
                        european_derivative,
                    )
                )
                if self.derivative_mode.startswith("euro"):
                    derivative = european_derivative * float(
                        self.derivative_mode.removeprefix("euro")
                    )
                return value, derivative

            def residual(candidate: float) -> float:
                return residual_and_derivative(candidate)[0]

            upper = max(min(float(values[index - 1]), expiry_boundary(r, q)), 1.0e-12)
            floor = min(1.0e-10, 0.5 * upper)
            if self.root_solver == "newton":
                # The graded mesh is uniform in sqrt(time), where log B is
                # nearly affine.  Sparse defect correction can therefore
                # retain the full interpolation mesh while avoiding a dense
                # nonlinear residual at every node.
                if (
                    self.collocation_stride > 1
                    and index > 3
                    and index < self.nodes
                    and index % self.collocation_stride != 0
                ):
                    values[index] = min(
                        values[index - 1] * values[index - 1] / values[index - 2],
                        upper,
                    )
                    continue
                if index == 1:
                    asymptotic_scale = 2.0 if values[0] > 0.95 else 0.75
                    candidate = upper * math.exp(
                        -asymptotic_scale * sigma * math.sqrt(t)
                    )
                else:
                    extrapolated = (
                        values[index - 1] * values[index - 1] / values[index - 2]
                    )
                    if index == 3:
                        candidate = values[index - 1] + self.third_predictor_alpha * (
                            extrapolated - values[index - 1]
                        )
                    else:
                        candidate = extrapolated
                candidate = min(max(candidate, floor), upper * (1.0 - 1.0e-12))
                steps_here = self.newton_steps
                if (
                    self.late_newton_steps is not None
                    and index > self.newton_cutover
                ):
                    steps_here = self.late_newton_steps
                for _ in range(steps_here):
                    value, derivative = residual_and_derivative(candidate)
                    if abs(value) <= 2.0e-12:
                        break
                    if not math.isfinite(derivative) or abs(derivative) < 1.0e-12:
                        proposal = 0.5 * candidate
                    else:
                        proposal = candidate - value / derivative
                    # Safeguard against a bad short-time asymptotic predictor
                    # while retaining quadratic convergence near the root.
                    proposal = min(proposal, upper * (1.0 - 1.0e-12))
                    proposal = max(proposal, floor)
                    if proposal < 0.35 * candidate:
                        proposal = 0.35 * candidate
                    elif proposal > 1.65 * candidate:
                        proposal = 1.65 * candidate
                    candidate = proposal
                values[index] = min(candidate, upper)
                continue
            f_upper = residual(upper)
            f_floor = residual(floor)
            if f_upper == 0.0:
                values[index] = upper
                continue
            if f_upper * f_floor < 0.0:
                values[index] = brentq(
                    residual, floor, upper, xtol=2.0e-13, rtol=2.0e-13
                )
                continue
            probes = np.concatenate((
                upper * (1.0 - np.geomspace(1.0e-11, 0.45, 32)),
                np.geomspace(max(upper * 0.55, 1.0e-10), 1.0e-10, 32),
            ))
            probes = np.unique(np.clip(probes, 1.0e-12, upper))[::-1]
            probe_values = [residual(float(point)) for point in probes]
            root: float | None = None
            for high_index in range(len(probes) - 1):
                high = float(probes[high_index])
                low = float(probes[high_index + 1])
                f_high = probe_values[high_index]
                f_low = probe_values[high_index + 1]
                if f_high == 0.0:
                    root = high
                    break
                if f_high * f_low < 0.0:
                    root = brentq(residual, low, high, xtol=2.0e-13, rtol=2.0e-13)
                    break
            if root is None:
                best = int(np.argmin(np.abs(probe_values)))
                raise RuntimeError(
                    f"smooth-pasting boundary root missing at node {index}/{self.nodes}: "
                    f"t={t:.8g}, best B={probes[best]:.8g}, residual={probe_values[best]:.3g}"
                )
            values[index] = min(root, upper)
        return Boundary(times, values, work)


def metric(values: list[float]) -> dict[str, float | int]:
    array = np.asarray(values, dtype=np.float64)
    return {
        "count": int(array.size),
        "median": float(np.median(array)),
        "p95": float(np.quantile(array, 0.95)),
        "p99": float(np.quantile(array, 0.99)),
        "max": float(np.max(array)),
        "mean": float(np.mean(array)),
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--nodes", type=int, default=16)
    parser.add_argument("--quadrature-order", type=int, default=2)
    parser.add_argument("--grading", type=float, default=2.0)
    parser.add_argument(
        "--boundary-equation",
        choices=("value", "smooth"),
        default="smooth",
    )
    parser.add_argument(
        "--product-basis",
        choices=("linear-t", "fh-u", "fh-t", "gauss-y"),
        default="linear-t",
    )
    parser.add_argument("--fh-degree", type=int, default=3)
    parser.add_argument(
        "--root-solver", choices=("brent", "newton"), default="brent"
    )
    parser.add_argument("--newton-steps", type=int, default=4)
    parser.add_argument(
        "--price-mesh",
        choices=("boundary", "uniform", "valuation", "double", "global"),
        default="boundary",
    )
    parser.add_argument("--price-power", type=float, default=2.0)
    parser.add_argument("--late-newton-steps", type=int)
    parser.add_argument("--newton-cutover", type=int, default=0)
    parser.add_argument(
        "--derivative-mode",
        default="full",
    )
    parser.add_argument(
        "--price-boundary-interp",
        choices=("linear", "log"),
        default="linear",
    )
    parser.add_argument(
        "--boundary-normal",
        choices=("exact", "hermite25", "hermite50"),
        default="exact",
    )
    parser.add_argument(
        "--price-normal",
        choices=("exact", "hermite25"),
        default="exact",
    )
    parser.add_argument("--spot-count", type=int, default=33)
    parser.add_argument("--collocation-stride", type=int, default=1)
    parser.add_argument("--boundary-order", type=int, default=4)
    parser.add_argument("--price-order", type=int, default=12)
    parser.add_argument("--third-predictor-alpha", type=float, default=1.0)
    parser.add_argument("--contract-limit", type=int)
    parser.add_argument("--contract-seed", type=lambda value: int(value, 0))
    parser.add_argument("--contract-count", type=int)
    parser.add_argument("--report", type=pathlib.Path)
    args = parser.parse_args()

    contracts = base.contracts(0x51D3, 24)
    if args.contract_seed is not None:
        if args.contract_count is None:
            raise SystemExit("--contract-seed requires --contract-count")
        contracts = base.contracts(args.contract_seed, args.contract_count)
    if args.contract_limit is not None:
        contracts = contracts[:args.contract_limit]

    log_spots = np.linspace(-0.75, 0.75, args.spot_count)
    spots = np.exp(log_spots)
    qdfp = base.QdFpSurfacePricer()
    method_class = (
        SmoothPastingKbi
        if args.boundary_equation == "smooth"
        else KimBoundaryIntegration
    )
    method = method_class(
        args.nodes,
        args.quadrature_order,
        args.grading,
        args.product_basis,
        args.fh_degree,
        args.root_solver,
        args.newton_steps,
        args.price_mesh,
        args.price_power,
        args.late_newton_steps,
        args.newton_cutover,
        args.derivative_mode,
        args.price_boundary_interp,
        args.boundary_normal,
        args.price_normal,
        args.collocation_stride,
        args.boundary_order,
        args.price_order,
        args.third_predictor_alpha,
    )
    errors: dict[str, list[float]] = {"call": [], "put": []}
    signed_errors: dict[str, list[float]] = {"call": [], "put": []}
    work_rows: list[dict[str, object]] = []
    contract_rows: list[dict[str, object]] = []

    for contract in contracts:
        maturity = contract.days / 365.0
        put_boundary = method.boundary(maturity, contract.r, contract.q, contract.sigma)
        put_values, _, put_price_points = method.put_prices(
            spots, maturity, contract.r, contract.q, contract.sigma, put_boundary
        )
        # Exact American duality C(S,K;r,q) = P(K,S;q,r).  With K=1,
        # normalize the dual put by its strike S and use moneyness 1/S.
        call_boundary = method.boundary(maturity, contract.q, contract.r, contract.sigma)
        dual_put, _, call_price_points = method.put_prices(
            1.0 / spots,
            maturity,
            contract.q,
            contract.r,
            contract.sigma,
            call_boundary,
        )
        call_values = spots * dual_put
        truths = {
            "call": qdfp.surface(spots, contract, contract.days, True),
            "put": qdfp.surface(spots, contract, contract.days, False),
        }
        approximations = {"call": call_values, "put": put_values}
        leg_rows: dict[str, object] = {}
        for name in ("call", "put"):
            signed = 100.0 * (approximations[name] - truths[name])
            absolute = np.abs(signed)
            signed_errors[name].extend(signed.tolist())
            errors[name].extend(absolute.tolist())
            worst = int(np.argmax(absolute))
            leg_rows[name] = {
                "max_abs_error_dollars": float(absolute[worst]),
                "worst_spot": float(100.0 * spots[worst]),
                "signed_error_dollars": float(signed[worst]),
                "boundary_at_valuation": float(
                    call_boundary.values[-1] if name == "call" else put_boundary.values[-1]
                ),
            }
        contract_rows.append({"contract": asdict(contract), **leg_rows})
        for name, boundary, price_points in (
            ("put", put_boundary, put_price_points),
            ("call", call_boundary, call_price_points),
        ):
            work_rows.append({
                "leg": name,
                "residual_evaluations": boundary.work.residual_evaluations,
                "boundary_kernel_points": boundary.work.boundary_kernel_points,
                "price_kernel_points_per_spot": price_points,
                "boundary_cdf_evaluations": 2 * boundary.work.boundary_kernel_points,
                "price_cdf_evaluations_per_spot": 2 * price_points,
            })

    report = {
        "method": "Kim boundary reconstruction plus early-exercise-premium integration",
        "runtime_model": "all boundary construction and pricing performed from six live inputs",
        "configuration": {
            "nodes": args.nodes,
            "quadrature_order": args.quadrature_order,
            "grading": args.grading,
            "boundary_equation": args.boundary_equation,
            "product_basis": args.product_basis,
            "fh_degree": args.fh_degree,
            "root_solver": args.root_solver,
            "newton_steps": args.newton_steps,
            "price_mesh": args.price_mesh,
            "price_power": args.price_power,
            "late_newton_steps": args.late_newton_steps,
            "newton_cutover": args.newton_cutover,
            "derivative_mode": args.derivative_mode,
            "price_boundary_interp": args.price_boundary_interp,
            "boundary_normal": args.boundary_normal,
            "price_normal": args.price_normal,
            "spot_count": args.spot_count,
            "collocation_stride": args.collocation_stride,
            "boundary_order": args.boundary_order,
            "price_order": args.price_order,
            "third_predictor_alpha": args.third_predictor_alpha,
            "contracts": len(contracts),
        },
        "errors_dollars_at_100_strike": {
            name: metric(values) for name, values in errors.items()
        },
        "signed_error_dollars_at_100_strike": {
            name: metric(values) for name, values in signed_errors.items()
        },
        "work": {
            "per_boundary": {
                key: metric([float(row[key]) for row in work_rows])
                for key in (
                    "residual_evaluations",
                    "boundary_kernel_points",
                    "boundary_cdf_evaluations",
                    "price_kernel_points_per_spot",
                    "price_cdf_evaluations_per_spot",
                )
            },
            "note": "price work is per one requested spot; a transaction prices one leg",
        },
        "held_out_contracts": contract_rows,
    }
    if args.report:
        args.report.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({
        "configuration": report["configuration"],
        "errors_dollars_at_100_strike": report["errors_dollars_at_100_strike"],
        "work": report["work"]["per_boundary"],
    }, indent=2))


if __name__ == "__main__":
    main()
