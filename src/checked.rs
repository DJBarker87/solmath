//! Safe-by-construction pricing inputs.
//!
//! The raw pricing functions accept any `u128` and fail closed via `Result`
//! on out-of-range values. This module goes one step further: it makes the
//! *valid financial domain a type*. A [`Price`], [`Rate`], [`Vol`], or [`Time`]
//! can only be constructed inside the certified domain, and the input bundles
//! ([`EuropeanInputs`], [`ImpliedVolInputs`], [`TwapInputs`]) can only be built
//! from those.
//!
//! Once you hold a bundle, the pricing methods **cannot panic and cannot
//! silently wrap**: every internal bound the raw kernels assume is already
//! established by construction. Degenerate-but-in-range corners still fail
//! closed as `Err` — that is the errors-as-values contract, not a fault — but
//! nothing in this layer aborts your instruction. This is the recommended entry
//! point for on-chain programs: validate untrusted instruction data **once**
//! into these types at your program boundary, then thread them through your
//! logic.
//!
//! ```
//! use solmath::checked::{EuropeanInputs, ImpliedVolInputs};
//!
//! # fn demo() -> Result<(), solmath::SolMathError> {
//! // Validate raw instruction data once, at the boundary.
//! let inputs = EuropeanInputs::from_raw(
//!     100_000_000_000_000, // s = 100
//!     105_000_000_000_000, // k = 105
//!     50_000_000_000,      // r = 5%
//!     200_000_000_000,     // sigma = 20%
//!     1_000_000_000_000,   // t = 1 year
//! )?;
//! let greeks = inputs.full()?; // cannot panic or wrap; Err only on degenerate corners
//! let _ = greeks.call;
//! # Ok(())
//! # }
//! ```
//!
//! # Domain
//!
//! The bounds below are deliberately generous — far past any realistic economic
//! value — while remaining inside the region the kernels are proven/measured to
//! handle without overflow. Values larger than these should be rescaled
//! homogeneously (divide `s`, `k`, and the resulting price by a common factor).
//! The bounds are enforced by [`Price::MAX`] etc. and continuously verified by
//! the `checked_inputs_never_panic_over_domain` fuzz test.

use crate::error::SolMathError;
use crate::SCALE;

#[cfg(feature = "bs")]
use crate::bs::{black_scholes_price, bs_delta, bs_full, bs_gamma, bs_rho, bs_theta, bs_vega};
#[cfg(feature = "bs")]
use crate::constants::BsFull;

/// A non-negative price in fixed-point (`SCALE = 1e12`), bounded to the
/// certified pricing domain `[0, 100_000]` real units.
///
/// Spot, strike, barrier, and observed option prices are all `Price`s.
#[cfg(any(feature = "bs", feature = "barrier", feature = "asian"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price(u128);

/// A non-negative interest rate in fixed-point, bounded to `[0, 1000%]`.
#[cfg(any(feature = "bs", feature = "barrier", feature = "asian"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rate(u128);

/// A strictly positive volatility in fixed-point, bounded to `(0, 10000%]`.
///
/// Black-Scholes requires `sigma > 0`; the type enforces it, so the raw
/// `DomainError` path for zero volatility is unreachable from this layer.
#[cfg(any(feature = "bs", feature = "barrier", feature = "asian"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vol(u128);

/// A strictly positive time-to-expiry in years (fixed-point), bounded to
/// `(0, 100]` years. Black-Scholes requires `t > 0`.
#[cfg(any(feature = "bs", feature = "barrier", feature = "asian"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time(u128);

#[cfg(any(feature = "bs", feature = "barrier", feature = "asian"))]
impl Price {
    /// Upper bound: `100_000` real units (`1e17` raw). This is the price bound
    /// the implied-volatility kernel is proven safe against, and it also keeps
    /// the Black-Scholes Greek combinations well within `i128`.
    pub const MAX: u128 = 100_000 * SCALE;

    /// Construct a validated price. Rejects values above [`Price::MAX`].
    #[inline]
    pub const fn new(raw: u128) -> Result<Self, SolMathError> {
        if raw > Self::MAX {
            return Err(SolMathError::DomainError);
        }
        Ok(Self(raw))
    }

    /// The underlying fixed-point value.
    #[inline]
    pub const fn get(self) -> u128 {
        self.0
    }
}

#[cfg(any(feature = "bs", feature = "barrier", feature = "asian"))]
impl Rate {
    /// Upper bound: `1000%` (`10 * SCALE`).
    pub const MAX: u128 = 10 * SCALE;

    /// Construct a validated rate. Rejects values above [`Rate::MAX`].
    #[inline]
    pub const fn new(raw: u128) -> Result<Self, SolMathError> {
        if raw > Self::MAX {
            return Err(SolMathError::DomainError);
        }
        Ok(Self(raw))
    }

    /// The underlying fixed-point value.
    #[inline]
    pub const fn get(self) -> u128 {
        self.0
    }
}

#[cfg(any(feature = "bs", feature = "barrier", feature = "asian"))]
impl Vol {
    /// Upper bound: `10000%` (`100 * SCALE`).
    pub const MAX: u128 = 100 * SCALE;

    /// Construct a validated volatility. Requires `0 < raw <= Vol::MAX`.
    #[inline]
    pub const fn new(raw: u128) -> Result<Self, SolMathError> {
        if raw == 0 || raw > Self::MAX {
            return Err(SolMathError::DomainError);
        }
        Ok(Self(raw))
    }

    /// The underlying fixed-point value.
    #[inline]
    pub const fn get(self) -> u128 {
        self.0
    }
}

#[cfg(any(feature = "bs", feature = "barrier", feature = "asian"))]
impl Time {
    /// Upper bound: `100` years (`100 * SCALE`).
    pub const MAX: u128 = 100 * SCALE;

    /// Construct a validated time-to-expiry. Requires `0 < raw <= Time::MAX`.
    #[inline]
    pub const fn new(raw: u128) -> Result<Self, SolMathError> {
        if raw == 0 || raw > Self::MAX {
            return Err(SolMathError::DomainError);
        }
        Ok(Self(raw))
    }

    /// The underlying fixed-point value.
    #[inline]
    pub const fn get(self) -> u128 {
        self.0
    }
}

/// A validated European-option parameter set: spot, strike, rate, volatility,
/// and time. Every pricing method on this type is guaranteed not to panic or
/// silently wrap for any in-domain input; degenerate corners (e.g. a
/// near-zero spot against a huge strike) fail closed with `Err`.
#[cfg(feature = "bs")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EuropeanInputs {
    s: Price,
    k: Price,
    r: Rate,
    sigma: Vol,
    t: Time,
}

#[cfg(feature = "bs")]
impl EuropeanInputs {
    /// Bundle already-validated components.
    #[inline]
    pub const fn new(s: Price, k: Price, r: Rate, sigma: Vol, t: Time) -> Self {
        Self { s, k, r, sigma, t }
    }

    /// Validate raw fixed-point instruction data in one shot. Returns
    /// `Err(DomainError)` if any field is outside its certified range.
    #[inline]
    pub const fn from_raw(
        s: u128,
        k: u128,
        r: u128,
        sigma: u128,
        t: u128,
    ) -> Result<Self, SolMathError> {
        // `?` is not usable in const fn on the MSRV, so match explicitly.
        let s = match Price::new(s) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let k = match Price::new(k) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let r = match Rate::new(r) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let sigma = match Vol::new(sigma) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let t = match Time::new(t) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok(Self { s, k, r, sigma, t })
    }

    /// `(call, put)` European prices at SCALE.
    #[inline]
    pub fn price(&self) -> Result<(u128, u128), SolMathError> {
        black_scholes_price(self.s.0, self.k.0, self.r.0, self.sigma.0, self.t.0)
    }

    /// Price plus all five Greeks in one call.
    #[inline]
    pub fn full(&self) -> Result<BsFull, SolMathError> {
        bs_full(self.s.0, self.k.0, self.r.0, self.sigma.0, self.t.0)
    }

    /// High-precision (1e15 internal) price plus Greeks.
    #[cfg(feature = "transcendental")]
    #[inline]
    pub fn full_hp(&self) -> Result<BsFull, SolMathError> {
        crate::hp::bs_full_hp(self.s.0, self.k.0, self.r.0, self.sigma.0, self.t.0)
    }

    /// `(call_delta, put_delta)`.
    #[inline]
    pub fn delta(&self) -> Result<(i128, i128), SolMathError> {
        bs_delta(self.s.0, self.k.0, self.r.0, self.sigma.0, self.t.0)
    }

    /// Gamma.
    #[inline]
    pub fn gamma(&self) -> Result<i128, SolMathError> {
        bs_gamma(self.s.0, self.k.0, self.r.0, self.sigma.0, self.t.0)
    }

    /// Vega.
    #[inline]
    pub fn vega(&self) -> Result<i128, SolMathError> {
        bs_vega(self.s.0, self.k.0, self.r.0, self.sigma.0, self.t.0)
    }

    /// `(call_theta, put_theta)`.
    #[inline]
    pub fn theta(&self) -> Result<(i128, i128), SolMathError> {
        bs_theta(self.s.0, self.k.0, self.r.0, self.sigma.0, self.t.0)
    }

    /// `(call_rho, put_rho)`.
    #[inline]
    pub fn rho(&self) -> Result<(i128, i128), SolMathError> {
        bs_rho(self.s.0, self.k.0, self.r.0, self.sigma.0, self.t.0)
    }

    /// The validated spot.
    #[inline]
    pub const fn spot(&self) -> Price {
        self.s
    }
    /// The validated strike.
    #[inline]
    pub const fn strike(&self) -> Price {
        self.k
    }
}

/// A validated implied-volatility problem: an observed market price plus the
/// contract parameters. [`ImpliedVolInputs::solve`] cannot panic; it returns
/// `Ok(sigma)` or `Err(NoConvergence)` for prices with no invertible volatility
/// (a mathematical outcome, not a failure).
#[cfg(feature = "iv")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImpliedVolInputs {
    market_price: Price,
    s: Price,
    k: Price,
    r: Rate,
    t: Time,
}

#[cfg(feature = "iv")]
impl ImpliedVolInputs {
    /// Bundle already-validated components.
    #[inline]
    pub const fn new(market_price: Price, s: Price, k: Price, r: Rate, t: Time) -> Self {
        Self {
            market_price,
            s,
            k,
            r,
            t,
        }
    }

    /// Validate raw fixed-point instruction data in one shot.
    #[inline]
    pub const fn from_raw(
        market_price: u128,
        s: u128,
        k: u128,
        r: u128,
        t: u128,
    ) -> Result<Self, SolMathError> {
        let market_price = match Price::new(market_price) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let s = match Price::new(s) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let k = match Price::new(k) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let r = match Rate::new(r) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let t = match Time::new(t) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok(Self {
            market_price,
            s,
            k,
            r,
            t,
        })
    }

    /// Solve for the implied volatility at SCALE.
    #[inline]
    pub fn solve(&self) -> Result<u128, SolMathError> {
        crate::iv::implied_vol(self.market_price.0, self.s.0, self.k.0, self.r.0, self.t.0)
    }
}

/// A validated European **barrier**-option parameter set: spot, strike, barrier
/// level, rate, volatility, and time. The barrier `h` is a [`Price`] like spot
/// and strike. Every method is guaranteed not to panic or silently wrap for any
/// in-domain input; degenerate corners fail closed with `Err`.
#[cfg(feature = "barrier")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BarrierInputs {
    s: Price,
    k: Price,
    h: Price,
    r: Rate,
    sigma: Vol,
    t: Time,
}

#[cfg(feature = "barrier")]
impl BarrierInputs {
    /// Bundle already-validated components.
    #[inline]
    pub const fn new(s: Price, k: Price, h: Price, r: Rate, sigma: Vol, t: Time) -> Self {
        Self {
            s,
            k,
            h,
            r,
            sigma,
            t,
        }
    }

    /// Validate raw fixed-point instruction data in one shot. Returns
    /// `Err(DomainError)` if any field is outside its certified range.
    #[inline]
    pub const fn from_raw(
        s: u128,
        k: u128,
        h: u128,
        r: u128,
        sigma: u128,
        t: u128,
    ) -> Result<Self, SolMathError> {
        let s = match Price::new(s) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let k = match Price::new(k) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let h = match Price::new(h) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let r = match Rate::new(r) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let sigma = match Vol::new(sigma) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let t = match Time::new(t) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok(Self {
            s,
            k,
            h,
            r,
            sigma,
            t,
        })
    }

    /// Continuously-monitored barrier option price (fresh contract, barrier not
    /// yet breached). `barrier_type` selects knock-in/out and up/down.
    #[inline]
    pub fn price(
        &self,
        is_call: bool,
        barrier_type: crate::barrier::BarrierType,
    ) -> Result<crate::barrier::BarrierResult, SolMathError> {
        crate::barrier::barrier_option(
            self.s.0,
            self.k.0,
            self.h.0,
            self.r.0,
            self.sigma.0,
            self.t.0,
            is_call,
            barrier_type,
        )
    }

    /// Barrier option price given persisted breach state. Persist
    /// `barrier_was_breached` on-chain across observations.
    #[inline]
    pub fn price_with_state(
        &self,
        is_call: bool,
        barrier_type: crate::barrier::BarrierType,
        barrier_was_breached: bool,
    ) -> Result<crate::barrier::BarrierResult, SolMathError> {
        crate::barrier::barrier_option_with_state(
            self.s.0,
            self.k.0,
            self.h.0,
            self.r.0,
            self.sigma.0,
            self.t.0,
            is_call,
            barrier_type,
            barrier_was_breached,
        )
    }
}

/// A validated continuous arithmetic-Asian / partially fixed TWAP quote.
///
/// Unlike a vanilla European quote, TWAP validity is relational: the remaining
/// averaging window cannot exceed time to expiry, and the fixed average must be
/// present exactly when the fixed weight is non-zero. Construction checks those
/// relations once; [`Self::price`] then calls the raw kernel with the same
/// validated state.
#[cfg(feature = "asian")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TwapInputs {
    s: Price,
    k: Price,
    r: Rate,
    q: Rate,
    sigma: Vol,
    t: Time,
    averaging_time: u128,
    fixed_average: Price,
    fixed_weight: u128,
}

#[cfg(feature = "asian")]
impl TwapInputs {
    /// Validate raw fixed-point instruction data in one shot.
    #[allow(clippy::too_many_arguments)]
    pub const fn from_raw(
        s: u128,
        k: u128,
        r: u128,
        q: u128,
        sigma: u128,
        t: u128,
        averaging_time: u128,
        fixed_average: u128,
        fixed_weight: u128,
    ) -> Result<Self, SolMathError> {
        let s = match Price::new(s) {
            Ok(v) if v.get() > 0 => v,
            Ok(_) => return Err(SolMathError::DomainError),
            Err(e) => return Err(e),
        };
        let k = match Price::new(k) {
            Ok(v) if v.get() > 0 => v,
            Ok(_) => return Err(SolMathError::DomainError),
            Err(e) => return Err(e),
        };
        let r = match Rate::new(r) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let q = match Rate::new(q) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let sigma = match Vol::new(sigma) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let t = match Time::new(t) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let fixed_average = match Price::new(fixed_average) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        if averaging_time > t.get() || fixed_weight > SCALE {
            return Err(SolMathError::DomainError);
        }
        if fixed_weight < SCALE && averaging_time == 0 {
            return Err(SolMathError::DomainError);
        }
        if fixed_weight == SCALE && averaging_time != 0 {
            return Err(SolMathError::DomainError);
        }
        if fixed_weight == 0 {
            if fixed_average.get() != 0 {
                return Err(SolMathError::DomainError);
            }
        } else if fixed_average.get() == 0 {
            return Err(SolMathError::DomainError);
        }

        Ok(Self {
            s,
            k,
            r,
            q,
            sigma,
            t,
            averaging_time,
            fixed_average,
            fixed_weight,
        })
    }

    /// Price the validated partially fixed TWAP state.
    #[inline]
    pub fn price(&self) -> Result<crate::asian::AsianOptionResult, SolMathError> {
        crate::asian::twap_option_price(
            self.s.0,
            self.k.0,
            self.r.0,
            self.q.0,
            self.sigma.0,
            self.t.0,
            self.averaging_time,
            self.fixed_average.0,
            self.fixed_weight,
        )
    }

    /// Remaining averaging-window length in years at `SCALE`.
    #[inline]
    pub const fn averaging_time(&self) -> u128 {
        self.averaging_time
    }

    /// Fraction of the final average already fixed, in `[0, SCALE]`.
    #[inline]
    pub const fn fixed_weight(&self) -> u128 {
        self.fixed_weight
    }

    /// Average of the already-fixed observations.
    #[inline]
    pub const fn fixed_average(&self) -> Price {
        self.fixed_average
    }
}

/// A validated weighted-pool swap request.
///
/// Unlike the option types, the pool's validity is *relational* — it depends on
/// the balance ratio `balance_in / (balance_in + amount_in) >= 0.01` and the
/// weight ratio `weight_in / weight_out <= 20`, not on independent per-field
/// bounds. [`PoolSwapInputs::from_raw`] checks the whole certified domain up
/// front, so holding a value proves the swap is quotable; [`Self::quote`] then
/// cannot fail for a domain reason and cannot panic. Rounding stays
/// protocol-favouring exactly as in [`crate::weighted_pool_swap`].
#[cfg(feature = "pool")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PoolSwapInputs {
    balance_in: u128,
    balance_out: u128,
    weight_in: u128,
    weight_out: u128,
    amount_in: u128,
    fee_rate: u128,
}

#[cfg(feature = "pool")]
impl PoolSwapInputs {
    /// Validate a raw swap against the certified pool domain. Mirrors the guard
    /// chain in [`crate::weighted_pool_swap`]: non-zero balances/weights, a fee
    /// in `[0, 100%]`, a weight ratio `w_in / w_out <= 20`, and a post-trade
    /// balance ratio `>= 0.01`. `amount_in == 0` is accepted (a no-op quote).
    pub fn from_raw(
        balance_in: u128,
        balance_out: u128,
        weight_in: u128,
        weight_out: u128,
        amount_in: u128,
        fee_rate: u128,
    ) -> Result<Self, SolMathError> {
        if weight_out == 0 || weight_in == 0 || balance_in == 0 || balance_out == 0 {
            return Err(SolMathError::DomainError);
        }
        if fee_rate > SCALE {
            return Err(SolMathError::DomainError);
        }
        // Weight ratio must not exceed 20 (exact-integer test, matching kernel).
        let weight_q = weight_in / weight_out;
        if weight_q > 20 || (weight_q == 20 && weight_in % weight_out != 0) {
            return Err(SolMathError::DomainError);
        }
        // Post-trade balance ratio must be >= 0.01, i.e. balance_in >=
        // ceil((balance_in + amount_in) / 100). amount_in == 0 trivially passes.
        if amount_in != 0 {
            let denominator = balance_in
                .checked_add(amount_in)
                .ok_or(SolMathError::Overflow)?;
            let min_balance = denominator / 100 + u128::from(denominator % 100 != 0);
            if balance_in < min_balance {
                return Err(SolMathError::DomainError);
            }
        }
        Ok(Self {
            balance_in,
            balance_out,
            weight_in,
            weight_out,
            amount_in,
            fee_rate,
        })
    }

    /// Quote the swap: returns `(net_output, fee)` at SCALE. Output rounds down
    /// and fee rounds up (protocol-favouring), matching the raw kernel exactly.
    #[inline]
    pub fn quote(&self) -> Result<(u128, u128), SolMathError> {
        crate::pool::weighted_pool_swap(
            self.balance_in,
            self.balance_out,
            self.weight_in,
            self.weight_out,
            self.amount_in,
            self.fee_rate,
        )
    }
}
