#!/usr/bin/env python3
"""Shared deterministic QuantLib references for American-option validation.

This module deliberately contains no ROM/POD machinery. It defines the
normalized contract sample and the QuantLib QdFp reference engine used by KBI
accuracy scripts.
"""

from __future__ import annotations

from dataclasses import dataclass

import numpy as np
import QuantLib as ql


EVAL_DATE = ql.Date(15, 6, 2026)
DAY_COUNT = ql.Actual365Fixed()


@dataclass(frozen=True)
class Contract:
    r: float
    q: float
    sigma: float
    days: int


def contracts(seed: int, count: int) -> list[Contract]:
    """Return the release's deterministic Latin-hypercube-style sample."""
    rng = np.random.default_rng(seed)
    columns: list[np.ndarray] = []
    for _ in range(4):
        column = (np.arange(count, dtype=np.float64) + rng.random(count)) / count
        rng.shuffle(column)
        columns.append(column)
    return [
        Contract(
            r=0.12 * columns[0][index],
            q=0.12 * columns[1][index],
            sigma=0.10 + 1.10 * columns[2][index],
            days=int(round(30 + 700 * columns[3][index])),
        )
        for index in range(count)
    ]


class QdFpSurfacePricer:
    """Normalized American price surfaces from QuantLib 1.41."""

    def __init__(self) -> None:
        ql.Settings.instance().evaluationDate = EVAL_DATE
        self.spot = ql.SimpleQuote(1.0)
        self.rate = ql.SimpleQuote(0.05)
        self.yield_ = ql.SimpleQuote(0.02)
        self.vol = ql.SimpleQuote(0.30)
        self.process = ql.BlackScholesMertonProcess(
            ql.QuoteHandle(self.spot),
            ql.YieldTermStructureHandle(
                ql.FlatForward(EVAL_DATE, ql.QuoteHandle(self.yield_), DAY_COUNT)
            ),
            ql.YieldTermStructureHandle(
                ql.FlatForward(EVAL_DATE, ql.QuoteHandle(self.rate), DAY_COUNT)
            ),
            ql.BlackVolTermStructureHandle(
                ql.BlackConstantVol(
                    EVAL_DATE, ql.NullCalendar(), ql.QuoteHandle(self.vol), DAY_COUNT
                )
            ),
        )
        self.qdfp_engine = ql.QdFpAmericanEngine(
            self.process, ql.QdFpAmericanEngine.accurateScheme()
        )

    def surface(
        self,
        grid: np.ndarray,
        contract: Contract,
        remaining_days: int,
        is_call: bool,
    ) -> np.ndarray:
        if remaining_days == 0:
            if is_call:
                return np.maximum(grid - 1.0, 0.0)
            return np.maximum(1.0 - grid, 0.0)
        self.rate.setValue(contract.r)
        self.yield_.setValue(contract.q)
        self.vol.setValue(contract.sigma)
        option_type = ql.Option.Call if is_call else ql.Option.Put
        option = ql.VanillaOption(
            ql.PlainVanillaPayoff(option_type, 1.0),
            ql.AmericanExercise(EVAL_DATE, EVAL_DATE + remaining_days),
        )
        option.setPricingEngine(self.qdfp_engine)
        values = np.empty(grid.size, dtype=np.float64)
        for index, normalized_spot in enumerate(grid):
            self.spot.setValue(float(normalized_spot))
            values[index] = max(option.NPV(), 0.0)
        return values
