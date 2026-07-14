#!/usr/bin/env python3
"""Cross-check the compiled Asian/TWAP path against high-precision moments."""

from __future__ import annotations

import argparse
import json
import math
import random
import subprocess
from pathlib import Path

import mpmath as mp

SCALE = 10**12
ROOT = Path(__file__).resolve().parents[1]
DEFAULT_BINARY = ROOT / "target/release/examples/asian_batch"


def round_raw(value: mp.mpf) -> int:
    return int(mp.floor(value * SCALE + mp.mpf("0.5")))


def reference(values: list[int]) -> list[int]:
    spot, strike, rate, yield_rate, sigma, time, window, fixed_average, weight = [
        mp.mpf(value) / SCALE for value in values
    ]
    carry = rate - yield_rate
    start = time - window

    def phi1(value: mp.mpf) -> mp.mpf:
        return mp.expm1(value) / value if value else mp.mpf(1)

    b_window = carry * window
    variance_window = sigma * sigma * window
    future_mean = spot * mp.exp(carry * start) * phi1(b_window)
    if b_window:
        second_kernel = 2 / b_window * (
            mp.exp(b_window) * phi1(b_window + variance_window)
            - phi1(2 * b_window + variance_window)
        )
    elif variance_window:
        second_kernel = (
            2 * (mp.expm1(variance_window) - variance_window) / variance_window**2
        )
    else:
        second_kernel = mp.mpf(1)
    future_second = (
        spot**2
        * mp.exp((2 * carry + sigma**2) * start)
        * second_kernel
    )

    mean = weight * fixed_average + (1 - weight) * future_mean
    variance = (1 - weight) ** 2 * (future_second - future_mean**2)
    log_variance = mp.log1p(variance / mean**2)
    discount = mp.exp(-rate * time)
    if log_variance:
        root_variance = mp.sqrt(log_variance)
        d1 = (mp.log(mean / strike) + log_variance / 2) / root_variance
        d2 = d1 - root_variance
        normal = lambda value: mp.erfc(-value / mp.sqrt(2)) / 2
        call = discount * (mean * normal(d1) - strike * normal(d2))
    else:
        call = discount * max(mean - strike, 0)
    put = call - discount * (mean - strike)
    return [round_raw(call), round_raw(put), round_raw(mean), round_raw(log_variance)]


def raw(value: float) -> int:
    return round(float(value) * SCALE)


def vectors(count: int) -> list[list[int]]:
    rng = random.Random(0xA51A2026)
    result = []
    for index in range(count):
        spot = rng.uniform(20, 500)
        strike = rng.uniform(0.5 * spot, 1.5 * spot)
        rate = rng.uniform(0, 0.2)
        yield_rate = rng.uniform(0, 0.2)
        sigma = rng.uniform(0.05, 2)
        time = rng.uniform(1 / 365, 2)
        window = rng.uniform(min(time, 1 / (365 * 24)), time)
        if index % 5 == 0:
            window = min(time, 30 / (365 * 24 * 60))
        weight = 0 if index % 3 == 0 else rng.uniform(0.01, 0.95)
        fixed_average = 0 if weight == 0 else rng.uniform(0.7 * spot, 1.3 * spot)
        result.append(
            [
                raw(spot),
                raw(strike),
                raw(rate),
                raw(yield_rate),
                raw(sigma),
                raw(time),
                raw(window),
                raw(fixed_average),
                raw(weight),
            ]
        )
    return result


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--cases", type=int, default=500)
    parser.add_argument("--binary", type=Path, default=DEFAULT_BINARY)
    parser.add_argument("--report", type=Path)
    args = parser.parse_args()

    mp.mp.dps = 60
    if not args.binary.exists():
        subprocess.run(
            [
                "cargo",
                "build",
                "--release",
                "--no-default-features",
                "--features",
                "asian",
                "--example",
                "asian_batch",
            ],
            cwd=ROOT,
            check=True,
        )

    inputs = vectors(args.cases)
    process = subprocess.run(
        [str(args.binary)],
        input="\n".join(" ".join(map(str, row)) for row in inputs) + "\n",
        text=True,
        capture_output=True,
        check=True,
    )
    lines = process.stdout.splitlines()
    if len(lines) != len(inputs):
        raise RuntimeError(f"runtime returned {len(lines)} rows for {len(inputs)} inputs")

    maxima = [0, 0, 0, 0]
    rejected = 0
    for values, line in zip(inputs, lines):
        if line.startswith("ERR"):
            rejected += 1
            continue
        actual = list(map(int, line.split()))
        expected = reference(values)
        for index, (got, want) in enumerate(zip(actual, expected)):
            maxima[index] = max(maxima[index], abs(got - want))

    report = {
        "seed": "0xA51A2026",
        "mpmath_dps": mp.mp.dps,
        "cases": args.cases,
        "accepted": args.cases - rejected,
        "rejected": rejected,
        "max_abs_raw": dict(zip(("call", "put", "mean", "log_variance"), maxima)),
        "max_abs_real": dict(
            zip(
                ("call", "put", "mean", "log_variance"),
                [value / SCALE for value in maxima],
            )
        ),
    }
    rendered = json.dumps(report, indent=2, sort_keys=True)
    print(rendered)
    if args.report:
        args.report.write_text(rendered + "\n")

    if rejected or maxima[0] > 10_000 or maxima[1] > 10_000 or maxima[2] > 100:
        raise SystemExit("Asian runtime validation exceeded its numerical budget")


if __name__ == "__main__":
    main()
