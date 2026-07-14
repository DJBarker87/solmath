#!/usr/bin/env python3
"""Compare emitted xi=0 Heston prices with the exact deterministic reduction."""

import math
import sys

import numpy as np
from scipy.special import ndtr


SCALE = 1e12


def main() -> None:
    call_errors: list[int] = []
    put_errors: list[int] = []
    worst_call = None
    worst_put = None
    failures: dict[str, int] = {}

    for line in sys.stdin:
        fields = line.strip().split(",")
        if fields[7].startswith("E"):
            failures[fields[7]] = failures.get(fields[7], 0) + 1
            continue
        raw = list(map(int, fields))
        spot_raw, strike_raw, rate_raw, time_raw, v0_raw, kappa_raw, theta_raw, call_raw, put_raw = raw
        spot, strike, rate, time, v0, kappa, theta = [
            value / SCALE
            for value in (spot_raw, strike_raw, rate_raw, time_raw, v0_raw, kappa_raw, theta_raw)
        ]
        if kappa == 0.0:
            average_variance = v0
        else:
            kappa_time = kappa * time
            average_variance = theta + (v0 - theta) * (-math.expm1(-kappa_time)) / kappa_time
        discounted_strike = strike * math.exp(-rate * time)
        if average_variance <= 0.0 or time <= 0.0:
            call_reference = max(spot - discounted_strike, 0.0)
            put_reference = max(discounted_strike - spot, 0.0)
        else:
            sigma = math.sqrt(average_variance)
            sigma_sqrt_time = sigma * math.sqrt(time)
            d1 = (
                math.log(spot / strike)
                + rate * time
                + 0.5 * average_variance * time
            ) / sigma_sqrt_time
            d2 = d1 - sigma_sqrt_time
            call_reference = spot * ndtr(d1) - discounted_strike * ndtr(d2)
            put_reference = call_reference - spot + discounted_strike
        call_expected = round(call_reference * SCALE)
        put_expected = round(put_reference * SCALE)
        call_error = abs(call_raw - call_expected)
        put_error = abs(put_raw - put_expected)
        call_record = (
            call_error,
            spot,
            strike,
            rate,
            time,
            v0,
            kappa,
            theta,
            call_raw,
            call_expected,
            average_variance,
        )
        put_record = (
            put_error,
            spot,
            strike,
            rate,
            time,
            v0,
            kappa,
            theta,
            put_raw,
            put_expected,
            average_variance,
        )
        call_errors.append(call_error)
        put_errors.append(put_error)
        if worst_call is None or call_error > worst_call[0]:
            worst_call = call_record
        if worst_put is None or put_error > worst_put[0]:
            worst_put = put_record

    for name, values, worst in (
        ("call", call_errors, worst_call),
        ("put", put_errors, worst_put),
    ):
        errors = np.asarray(values, dtype=np.float64)
        print(
            f"{name} n={len(errors)} median_raw={np.median(errors):.0f} "
            f"p95_raw={np.quantile(errors, 0.95, method='lower'):.0f} "
            f"p99_raw={np.quantile(errors, 0.99, method='lower'):.0f} "
            f"max_raw={errors.max():.0f} worst={worst}"
        )
    print(f"failures={failures}")


if __name__ == "__main__":
    main()
