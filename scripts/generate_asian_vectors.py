#!/usr/bin/env python3
"""Generate SolMath Asian/TWAP accuracy corpora.

Exactly 100,000 stratified production vectors and 10,000 adversarial vectors
are evaluated from the fixed-point inputs with mpmath at 60 decimal digits.
These corpora validate the compiled implementation of the stated continuous
GBM moment-match model.  The separate QuantLib generator remains an independent
cross-check for the unseasoned subset supported by QuantLib's Levy engine.
"""

from __future__ import annotations

import json
import random
from itertools import product
from pathlib import Path

import mpmath as mp


SCALE = 10**12
PRODUCTION_COUNT = 100_000
ADVERSARIAL_COUNT = 10_000
SEED = 0xA51A2026
ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "benchmark"


def raw(value: float) -> int:
    return max(1, round(value * SCALE))


def round_raw(value: mp.mpf) -> int:
    return int(mp.floor(value * SCALE + mp.mpf("0.5")))


def reference(values: list[int]) -> list[int]:
    spot, strike, rate, yield_rate, sigma, time, window, fixed_average, weight = [
        mp.mpf(value) / SCALE for value in values
    ]
    discount = mp.exp(-rate * time)
    if weight == 1:
        mean = fixed_average
        call = discount * max(mean - strike, 0)
        put = discount * max(strike - mean, 0)
        return [round_raw(call), round_raw(put), round_raw(mean), 0]

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
    variance = max((1 - weight) ** 2 * (future_second - future_mean**2), 0)
    log_variance = mp.log1p(variance / mean**2)
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


def vector(
    spot: float,
    strike: float,
    rate: float,
    yield_rate: float,
    sigma: float,
    time: float,
    window: float,
    fixed_average: float,
    weight: float,
    category: str,
) -> dict[str, str | int]:
    values = [
        raw(spot),
        raw(strike),
        max(0, round(rate * SCALE)),
        max(0, round(yield_rate * SCALE)),
        raw(sigma),
        raw(time),
        0 if weight == 1 else raw(min(time, window)),
        0 if weight == 0 else raw(fixed_average),
        SCALE if weight == 1 else max(0, min(SCALE - 1, round(weight * SCALE))),
    ]
    return vector_raw(values, category)


def vector_raw(values: list[int], category: str) -> dict[str, str | int]:
    """Build a vector without losing deliberately chosen raw-unit seams."""
    expected = reference(values)
    keys = (
        "s",
        "k",
        "r",
        "q",
        "sigma",
        "t",
        "averaging_time",
        "fixed_average",
        "fixed_weight",
    )
    row: dict[str, str | int] = {key: str(value) for key, value in zip(keys, values)}
    row.update(
        {
            "expected_call": str(expected[0]),
            "expected_put": str(expected[1]),
            "expected_mean": str(expected[2]),
            "expected_log_variance": str(expected[3]),
            "category": category,
        }
    )
    return row


def production_vectors(rng: random.Random) -> list[dict[str, str | int]]:
    money = [(0.50, 0.75), (0.75, 0.95), (0.95, 1.05), (1.05, 1.25), (1.25, 1.50)]
    rates = [(0.0, 0.02), (0.02, 0.08), (0.08, 0.20)]
    yields = [(0.0, 0.02), (0.02, 0.08), (0.08, 0.20)]
    vols = [(0.05, 0.15), (0.15, 0.40), (0.40, 0.80), (0.80, 2.00)]
    maturities = [(1 / 365, 30 / 365), (30 / 365, 0.5), (0.5, 1.0), (1.0, 2.0)]
    window_ratios = [(0.0001, 0.01), (0.01, 0.25), (0.25, 0.75), (0.75, 1.0)]
    fixing_modes = ("unseasoned", "partial_low", "partial_high", "fully_fixed")
    cells = list(product(money, rates, yields, vols, maturities, window_ratios, fixing_modes))
    result = []
    for index in range(PRODUCTION_COUNT):
        money_band, rate_band, yield_band, vol_band, maturity_band, window_band, mode = cells[
            index % len(cells)
        ]
        spot = rng.uniform(20, 500)
        strike = spot / rng.uniform(*money_band)
        rate = rng.uniform(*rate_band)
        yield_rate = rng.uniform(*yield_band)
        sigma = rng.uniform(*vol_band)
        time = rng.uniform(*maturity_band)
        window = time * rng.uniform(*window_band)
        if mode == "unseasoned":
            weight, fixed_average = 0.0, 0.0
        elif mode == "partial_low":
            weight = rng.uniform(0.01, 0.50)
            fixed_average = spot * rng.uniform(0.70, 1.30)
        elif mode == "partial_high":
            weight = rng.uniform(0.50, 0.95)
            fixed_average = spot * rng.uniform(0.70, 1.30)
        else:
            weight = 1.0
            fixed_average = spot * rng.uniform(0.70, 1.30)
        result.append(
            vector(
                spot,
                strike,
                rate,
                yield_rate,
                sigma,
                time,
                window,
                fixed_average,
                weight,
                mode,
            )
        )
    return result


def adversarial_vectors(rng: random.Random) -> list[dict[str, str | int]]:
    result = []
    category_count = 11
    base_count, extra = divmod(ADVERSARIAL_COUNT, category_count)
    for category_index in range(category_count):
        lane_count = base_count + (1 if category_index < extra else 0)
        for lane in range(lane_count):
            spot = rng.uniform(1, 1_000)
            strike = spot * rng.uniform(0.20, 2.00)
            rate = rng.uniform(0, 0.20)
            yield_rate = rng.uniform(0, 0.20)
            sigma = rng.uniform(0.05, 2.0)
            time = rng.uniform(1 / 365, 2.0)
            window = time * rng.uniform(0.001, 1.0)
            weight = rng.uniform(0.01, 0.99)
            fixed_average = spot * rng.uniform(0.50, 1.50)

            if category_index == 0:
                category = "tiny_window"
                window = rng.choice([1 / SCALE, 2 / SCALE, 30 / (365 * 24 * 60)])
            elif category_index == 1:
                category = "future_start"
                time = rng.uniform(2.0, 10.0)
                window = rng.uniform(30 / (365 * 24 * 60), 1 / 365)
                sigma = rng.uniform(0.05, 1.0)
                rate, yield_rate, weight, fixed_average = rng.uniform(0, 0.10), rng.uniform(0, 0.10), 0.0, 0.0
            elif category_index in (2, 3):
                category = "series_below" if category_index == 2 else "series_above"
                sigma = rng.uniform(0.40, 1.50)
                carry = rng.choice([-1, 1]) * rng.uniform(0.01, 0.15)
                rate = 0.16 + min(carry, 0)
                yield_rate = rate - carry
                coefficient = abs(carry + sigma * sigma) + abs(carry)
                seam = 0.249999 if category_index == 2 else 0.250001
                window = seam / coefficient
                time = window + rng.uniform(0, 1.0)
            elif category_index == 4:
                category = "zero_carry"
                rate = yield_rate = rng.uniform(0, 0.20)
            elif category_index == 5:
                category = "carry_raw_seam"
                base_raw = rng.randint(0, 200_000_000_000)
                if lane % 2:
                    rate, yield_rate = (base_raw + 1) / SCALE, base_raw / SCALE
                else:
                    rate, yield_rate = base_raw / SCALE, (base_raw + 1) / SCALE
            elif category_index == 6:
                category = "fixing_weight_seam"
                weight = rng.choice([1 / SCALE, 2 / SCALE, (SCALE - 2) / SCALE, (SCALE - 1) / SCALE])
            elif category_index == 7:
                category = "deep_tail_low_vol"
                strike = spot * rng.choice([rng.uniform(0.05, 0.20), rng.uniform(3.0, 8.0)])
                sigma = rng.uniform(0.001, 0.10)
                time = rng.uniform(1 / 365, 0.25)
                window = time
                weight, fixed_average = 0.0, 0.0
            elif category_index == 8:
                category = "high_variance"
                sigma = rng.uniform(2.0, 6.0)
                time = rng.uniform(0.005, min(0.75, 30 / (sigma * sigma)))
                window = time * rng.uniform(0.25, 1.0)
                rate, yield_rate = rng.uniform(0, 0.10), rng.uniform(0, 0.10)
            elif category_index == 9:
                category = "partial_fixing_cdf_sensitivity"
                if lane == 0:
                    # Retain the production maximizer that exposed this missing
                    # regime. One raw unit of log-variance error is amplified
                    # near the ATM CDF transition when little variance remains.
                    result.append(
                        vector_raw(
                            [
                                498_985_480_275_581,
                                491_004_255_994_972,
                                19_922_217_555,
                                41_549_074_800,
                                95_483_712_338,
                                33_551_608_118,
                                27_155_779_124,
                                490_348_271_125_054,
                                783_463_001_134,
                            ],
                            category,
                        )
                    )
                    continue

                spot = rng.uniform(100, 999)
                rate = rng.uniform(0, 0.08)
                yield_rate = rng.uniform(0, 0.08)
                sigma = rng.uniform(0.005, 0.15)
                time = rng.uniform(1 / 365, 0.10)
                window = time * rng.uniform(0.20, 1.0)
                weight = 1 - 10 ** rng.uniform(-3.0, -0.45)
                fixed_average = spot * rng.uniform(0.90, 1.10)

                # Place the strike at and immediately around the matched
                # distribution's CDF transition. Compute the mean/variance from
                # the exact raw inputs before choosing K; K does not affect them.
                values = [
                    raw(spot),
                    1,
                    round(rate * SCALE),
                    round(yield_rate * SCALE),
                    raw(sigma),
                    raw(time),
                    raw(min(time, window)),
                    raw(fixed_average),
                    max(1, min(SCALE - 1, round(weight * SCALE))),
                ]
                moments = reference(values)
                mean_raw, log_variance_raw = moments[2], moments[3]
                standard_deviation_raw = round(
                    mean_raw * (mp.expm1(mp.mpf(log_variance_raw) / SCALE) ** mp.mpf("0.5"))
                )
                z = (0, 0.05, -0.05, 0.20, -0.20, 0.50, -0.50, 1.0, -1.0)[lane % 9]
                raw_nudge = (-2, -1, 0, 1, 2)[(lane // 9) % 5]
                values[1] = max(1, round(mean_raw + z * standard_deviation_raw) + raw_nudge)
                result.append(vector_raw(values, category))
                continue
            else:
                category = "fully_fixed"
                weight = 1.0
                window = 0.0

            result.append(
                vector(
                    spot,
                    strike,
                    rate,
                    yield_rate,
                    sigma,
                    time,
                    window,
                    fixed_average,
                    weight,
                    category,
                )
            )
    return result


def save(filename: str, kind: str, vectors: list[dict[str, str | int]]) -> None:
    payload = {
        "meta": {
            "reference": "mpmath 1.4.1, 60 decimal digits, independent continuous-GBM moment match",
            "kind": kind,
            "scale": SCALE,
            "seed": SEED,
            "count": len(vectors),
            "generator": "scripts/generate_asian_vectors.py",
        },
        "vectors": vectors,
    }
    path = OUTPUT / filename
    with path.open("w", encoding="utf-8") as handle:
        json.dump(payload, handle, separators=(",", ":"))
        handle.write("\n")
    print(f"{len(vectors):,} -> {path}")


def main() -> None:
    mp.mp.dps = 60
    rng = random.Random(SEED)
    save("prod_asian_vectors.json", "stratified production", production_vectors(rng))
    save("adv_asian_vectors.json", "adversarial seams and tails", adversarial_vectors(rng))


if __name__ == "__main__":
    main()
