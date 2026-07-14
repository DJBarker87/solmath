#!/usr/bin/env python3
"""Reproducible accuracy campaign for the actual fixed-point NIG runtime.

The primary reference uses the exact Esscher-shift identity with SciPy's NIG
CDF.  Upper tails are evaluated by reflection rather than `1 - cdf`, avoiding
catastrophic cancellation.  A smaller audit set can additionally be checked
against arbitrary-precision direct density integration by
`scripts/nig_reference.py`.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import subprocess
import sys
from collections import Counter
from dataclasses import asdict, dataclass
from pathlib import Path

import numpy as np
import scipy
from scipy.integrate import quad
from scipy.special import k1e
from scipy.stats import norminvgauss


SCALE = 10**12
ROOT = Path(__file__).resolve().parents[1]
BINARY = ROOT / "target/release/examples/nig_batch"


@dataclass(frozen=True)
class Quote:
    spot: int
    strike: int
    rate: int
    dividend: int
    time: int
    alpha: int
    beta: int
    delta: int
    requested: int

    def line(self) -> str:
        return (
            f"{self.spot} {self.strike} {self.rate} {self.dividend} "
            f"{self.time} {self.alpha} {self.beta} {self.delta} {self.requested}"
        )


def raw(value: float) -> int:
    return int(round(value * SCALE))


def clamp(value: float, low: float, high: float) -> float:
    return min(max(value, low), high)


def make_quote(
    spot: float,
    strike: float,
    rate: float,
    dividend: float,
    time: float,
    alpha: float,
    beta: float,
    delta: float,
    request_relative: float,
) -> Quote:
    values = [spot, strike, time, alpha, delta]
    if not all(math.isfinite(value) and value > 0 for value in values):
        raise ValueError("invalid generated NIG quote")
    q = Quote(
        raw(spot),
        raw(strike),
        raw(rate),
        raw(dividend),
        raw(time),
        raw(alpha),
        raw(beta),
        raw(delta),
        max(1, raw(max(spot, strike) * request_relative)),
    )
    return q


def production_quotes(count: int, seed: int) -> list[Quote]:
    rng = np.random.default_rng(seed)
    quotes: list[Quote] = []
    while len(quotes) < count:
        time = math.exp(rng.uniform(math.log(1 / 3650), math.log(5.0)))
        alpha = math.exp(rng.uniform(math.log(2.05), math.log(100.0)))
        # Sample beta from the intersection of the two declared 0.65-alpha
        # headroom constraints. Ten percent of quotes concentrate near a gate.
        lo = max(-0.65 * alpha, -1.0 - 0.65 * alpha)
        hi = min(0.65 * alpha, -1.0 + 0.65 * alpha)
        if rng.random() < 0.10:
            edge = lo if rng.random() < 0.5 else hi
            beta = edge + (hi - lo) * rng.uniform(1e-6, 2e-3) * (1 if edge == lo else -1)
        else:
            beta = rng.uniform(lo, hi)

        gamma = math.sqrt(alpha * alpha - beta * beta)
        annual_sigma = math.exp(rng.uniform(math.log(0.03), math.log(1.5)))
        delta = annual_sigma * annual_sigma * gamma**3 / (alpha * alpha)
        # Keep both delta/year and elapsed delta inside the executable domain.
        delta = clamp(delta, 1e-3 / time * (1 + 1e-5), 15.0 * (1 - 1e-8))
        if delta * time > 15.0:
            delta = 15.0 / time * (1 - 1e-8)
        if not (0 < delta <= 15 and 1e-3 <= delta * time <= 15):
            continue

        rate = rng.uniform(-0.25, 0.25)
        dividend = rng.uniform(-0.25, 0.25)
        if rng.random() < 0.10:
            magnitude = rng.uniform(1.8, 1.995)
            log_forward = magnitude if rng.random() < 0.5 else -magnitude
        else:
            log_forward = rng.triangular(-1.8, 0.0, 1.8)
        spot = math.exp(rng.uniform(math.log(1.0), math.log(1_000.0)))
        log_moneyness = log_forward - (rate - dividend) * time
        strike = spot / math.exp(log_moneyness)
        if not (1e-6 < strike <= 100_000 and spot <= 100_000):
            continue
        quotes.append(
            make_quote(
                spot,
                strike,
                rate,
                dividend,
                time,
                alpha,
                beta,
                delta,
                5e-5,
            )
        )
    return quotes


def adversarial_quotes(count: int, seed: int) -> list[Quote]:
    rng = np.random.default_rng(seed)
    quotes: list[Quote] = []
    times = np.array([1e-5, 1e-4, 1e-3, 1e-2, 0.1, 1.0, 4.999999])
    alphas = np.array([2.000001, 2.01, 3.0, 10.0, 99.99, 99.999999])
    log_forwards = np.array([-1.999999, -1.99, -1.0, -1e-8, 0.0, 1e-8, 1.0, 1.99, 1.999999])
    boundary_rates = np.array([-0.249999, -0.20, 0.0, 0.20, 0.249999])

    while len(quotes) < count:
        time = float(rng.choice(times))
        alpha = float(rng.choice(alphas))
        lo = max(-0.65 * alpha, -1.0 - 0.65 * alpha)
        hi = min(0.65 * alpha, -1.0 + 0.65 * alpha)
        if rng.random() < 0.75:
            epsilon = (hi - lo) * 10 ** rng.uniform(-9, -4)
            beta = lo + epsilon if rng.random() < 0.5 else hi - epsilon
        else:
            beta = rng.uniform(lo, hi)

        max_elapsed = min(15.0, 15.0 * time)
        if max_elapsed < 1e-3:
            continue
        elapsed_choices = [1.000001e-3, max_elapsed * (1 - 1e-8)]
        if max_elapsed > 1.01e-3:
            elapsed_choices.append(
                math.exp(rng.uniform(math.log(1.000001e-3), math.log(max_elapsed)))
            )
        elapsed = float(rng.choice(elapsed_choices))
        delta = elapsed / time
        if not (0 < delta <= 15):
            continue

        rate = float(rng.choice(boundary_rates))
        dividend = float(rng.choice(boundary_rates))
        log_forward = float(rng.choice(log_forwards))
        spot = float(10 ** rng.uniform(-2, 4))
        strike = spot / math.exp(log_forward - (rate - dividend) * time)
        if not (1e-9 < strike <= 100_000 and spot <= 100_000):
            continue
        quotes.append(
            make_quote(
                spot,
                strike,
                rate,
                dividend,
                time,
                alpha,
                beta,
                delta,
                1e-4,
            )
        )
    return quotes


def fixed_runtime(quotes: list[Quote]) -> list[tuple[str, ...]]:
    payload = "\n".join(quote.line() for quote in quotes) + "\n"
    process = subprocess.run(
        [str(BINARY)],
        input=payload,
        text=True,
        capture_output=True,
        check=True,
    )
    lines = process.stdout.splitlines()
    if len(lines) != len(quotes):
        raise RuntimeError(f"runtime returned {len(lines)} lines for {len(quotes)} quotes")
    return [tuple(line.split()) for line in lines]


def direct_density_reference(quote: Quote) -> tuple[float, float]:
    """Adaptive direct-OTM fallback for CDF cancellation/slow convergence."""
    spot, strike, rate, dividend, time, alpha, beta, delta_py = (
        value / SCALE
        for value in (
            quote.spot,
            quote.strike,
            quote.rate,
            quote.dividend,
            quote.time,
            quote.alpha,
            quote.beta,
            quote.delta,
        )
    )
    elapsed = delta_py * time
    gamma = math.sqrt(alpha * alpha - beta * beta)
    gamma_one = math.sqrt(alpha * alpha - (beta + 1) ** 2)
    kappa = math.log(spot / strike) + (rate - dividend) * time + elapsed * (
        gamma_one - gamma
    )
    threshold = -kappa
    discounted_spot = spot * math.exp(-dividend * time)
    discounted_strike = strike * math.exp(-rate * time)
    call_is_otm = discounted_spot <= discounted_strike

    def integrand(y: float) -> float:
        if y <= 0:
            return 0.0
        x = threshold + y if call_is_otm else threshold - y
        omega = math.hypot(elapsed, x)
        z = alpha * omega
        log_value = (
            math.log(alpha * elapsed / (math.pi * omega))
            + math.log(float(k1e(z)))
            + elapsed * gamma
            + beta * x
            - z
        )
        if call_is_otm:
            log_value += (
                math.log(math.expm1(y))
                if y < 50
                else y + math.log1p(-math.exp(-y))
            )
        else:
            log_value += math.log(-math.expm1(-y))
        if log_value < -745:
            return 0.0
        return math.exp(log_value)

    integral, _ = quad(integrand, 0.0, np.inf, epsabs=1e-13, epsrel=2e-12, limit=300)
    otm = discounted_strike * integral
    if call_is_otm:
        return otm, otm + discounted_strike - discounted_spot
    return otm + discounted_spot - discounted_strike, otm


def reference_prices(quotes: list[Quote]) -> tuple[np.ndarray, np.ndarray, int]:
    matrix = np.array(
        [
            [
                q.spot,
                q.strike,
                q.rate,
                q.dividend,
                q.time,
                q.alpha,
                q.beta,
                q.delta,
            ]
            for q in quotes
        ],
        dtype=np.float64,
    ) / SCALE
    spot, strike, rate, dividend, time, alpha, beta, delta_py = matrix.T
    elapsed = delta_py * time
    gamma = np.sqrt(alpha * alpha - beta * beta)
    gamma_one = np.sqrt(alpha * alpha - (beta + 1.0) ** 2)
    kappa = np.log(spot / strike) + (rate - dividend) * time + elapsed * (gamma_one - gamma)
    threshold = -kappa
    discounted_spot = spot * np.exp(-dividend * time)
    discounted_strike = strike * np.exp(-rate * time)
    scipy_a = alpha * elapsed
    scipy_b = beta * elapsed
    scipy_b_one = (beta + 1.0) * elapsed

    call_is_otm = discounted_spot <= discounted_strike
    call = np.empty(len(quotes), dtype=np.float64)
    put = np.empty(len(quotes), dtype=np.float64)

    call_indices = np.flatnonzero(call_is_otm)
    if len(call_indices):
        i = call_indices
        # NIG symmetry: P_beta[X > h] = P_-beta[X < -h]. This avoids 1-CDF.
        tail = norminvgauss.cdf(
            -threshold[i], scipy_a[i], -scipy_b[i], scale=elapsed[i]
        )
        tail_one = norminvgauss.cdf(
            -threshold[i], scipy_a[i], -scipy_b_one[i], scale=elapsed[i]
        )
        call[i] = discounted_spot[i] * tail_one - discounted_strike[i] * tail
        put[i] = call[i] + discounted_strike[i] - discounted_spot[i]

    put_indices = np.flatnonzero(~call_is_otm)
    if len(put_indices):
        i = put_indices
        lower = norminvgauss.cdf(
            threshold[i], scipy_a[i], scipy_b[i], scale=elapsed[i]
        )
        lower_one = norminvgauss.cdf(
            threshold[i], scipy_a[i], scipy_b_one[i], scale=elapsed[i]
        )
        put[i] = discounted_strike[i] * lower - discounted_spot[i] * lower_one
        call[i] = put[i] + discounted_spot[i] - discounted_strike[i]

    # Roundoff in a difference of two positive digital legs may produce a
    # sub-nanodollar negative number. Anything material is retained as a
    # reference failure rather than hidden.
    call[(call < 0) & (call > -1e-9)] = 0.0
    put[(put < 0) & (put > -1e-9)] = 0.0
    failed = ~np.isfinite(call) | ~np.isfinite(put) | (call < -1e-9) | (put < -1e-9)
    fallback_count = int(np.sum(failed))
    for index in np.flatnonzero(failed):
        call[index], put[index] = direct_density_reference(quotes[int(index)])
    return call, put, fallback_count


def percentile(values: list[float], quantile: float) -> float | None:
    if not values:
        return None
    return float(np.quantile(np.asarray(values), quantile))


def evaluate(name: str, quotes: list[Quote]) -> dict:
    digest = hashlib.sha256(("\n".join(q.line() for q in quotes) + "\n").encode()).hexdigest()
    runtime = fixed_runtime(quotes)
    accepted_indices = [
        index for index, result in enumerate(runtime) if result and result[0] == "OK"
    ]
    accepted_quotes = [quotes[index] for index in accepted_indices]
    reference_call, reference_put, reference_fallbacks = reference_prices(accepted_quotes)
    errors: list[float] = []
    normalized_errors: list[float] = []
    certificate_ratios: list[float] = []
    reject_reasons: Counter[str] = Counter()
    tiers: Counter[int] = Counter()
    certificate_violations = 0
    request_violations = 0
    reference_failures = 0
    worst: dict | None = None

    reference_index = 0
    for index, (quote, result) in enumerate(zip(quotes, runtime)):
        if not result or result[0] != "OK":
            reject_reasons[" ".join(result) if result else "ERR:empty"] += 1
            continue
        call_ref = reference_call[reference_index]
        put_ref = reference_put[reference_index]
        reference_index += 1
        if (
            not np.isfinite(call_ref)
            or not np.isfinite(put_ref)
            or call_ref < -1e-9
            or put_ref < -1e-9
        ):
            reference_failures += 1
            continue
        call = int(result[1]) / SCALE
        put = int(result[2]) / SCALE
        certificate = int(result[3]) / SCALE
        tier = int(result[4])
        tiers[tier] += 1
        error = max(abs(call - call_ref), abs(put - put_ref))
        errors.append(error)
        normalized_errors.append(error / (max(quote.spot, quote.strike) / SCALE) * 100)
        certificate_ratios.append(error / certificate if certificate else math.inf)
        # Allow five raw output units for conversion/reference rounding.
        if error > certificate + 5 / SCALE:
            certificate_violations += 1
        if error > quote.requested / SCALE + 5 / SCALE:
            request_violations += 1
        if worst is None or error > worst["max_call_put_abs_error"]:
            worst = {
                "index": index,
                "input": asdict(quote),
                "runtime_call": call,
                "runtime_put": put,
                "reference_call": float(call_ref),
                "reference_put": float(put_ref),
                "returned_max_abs_error": certificate,
                "max_call_put_abs_error": error,
                "tier": tier,
            }

    accepted = len(errors)
    return {
        "name": name,
        "seeded_input_sha256": digest,
        "quotes": len(quotes),
        "accepted": accepted,
        "acceptance_rate": accepted / len(quotes),
        "rejected": sum(reject_reasons.values()),
        "reference_failures": reference_failures,
        "reference_fallbacks": reference_fallbacks,
        "reject_reasons": dict(reject_reasons),
        "tiers": {str(key): value for key, value in sorted(tiers.items())},
        "certificate_violations": certificate_violations,
        "request_violations": request_violations,
        "absolute_error": {
            "median": percentile(errors, 0.5),
            "p90": percentile(errors, 0.9),
            "p99": percentile(errors, 0.99),
            "p999": percentile(errors, 0.999),
            "max": max(errors) if errors else None,
        },
        "absolute_error_per_100_notional": {
            "median": percentile(normalized_errors, 0.5),
            "p90": percentile(normalized_errors, 0.9),
            "p99": percentile(normalized_errors, 0.99),
            "p999": percentile(normalized_errors, 0.999),
            "max": max(normalized_errors) if normalized_errors else None,
        },
        "error_over_returned_allowance": {
            "p99": percentile(certificate_ratios, 0.99),
            "max": max(certificate_ratios) if certificate_ratios else None,
        },
        "worst_accepted_quote": worst,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--production", type=int, default=100_000)
    parser.add_argument("--adversarial", type=int, default=10_000)
    parser.add_argument("--seed", type=int, default=0x4E49475F2026)
    parser.add_argument(
        "--output", type=Path, default=ROOT / "benchmark/nig_release_report.json"
    )
    parser.add_argument("--skip-build", action="store_true")
    args = parser.parse_args()

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

    production = production_quotes(args.production, args.seed)
    adversarial = adversarial_quotes(args.adversarial, args.seed ^ 0xA5A5A5A5)
    report = {
        "schema": 1,
        "model": "exponential NIG with martingale correction and beta+1 Esscher shift",
        "runtime": "SolMath fixed-point direct OTM Gauss-Kronrod 15/7",
        "reference": "SciPy norminvgauss CDF Esscher identity; upper tails by NIG reflection",
        "versions": {
            "python": sys.version.split()[0],
            "numpy": np.__version__,
            "scipy": scipy.__version__,
        },
        "scale": SCALE,
        "seed": args.seed,
        "production": evaluate("production", production),
        "adversarial": evaluate("adversarial", adversarial),
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
