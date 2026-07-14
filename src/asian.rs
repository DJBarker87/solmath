//! Continuous arithmetic-Asian / TWAP-settled option pricing.
//!
//! The final settlement value is modelled as
//!
//! ```text
//! A = w A_fixed + (1 - w) / tau * integral_[T-tau,T] S(u) du
//! ```
//!
//! where `T` is time to payment, `tau` is the remaining averaging-window
//! length, and `w` is the fraction of the final average already observed.
//! This covers both common TWAP states:
//!
//! - before averaging starts: `w = 0`, `tau < T`;
//! - while averaging is running: `w > 0`, normally `tau = T`.
//!
//! Arithmetic averages of lognormal prices are not themselves lognormal. The
//! pricer therefore computes the first two risk-neutral moments of the
//! remaining **continuous arithmetic average exactly**, then prices a
//! moment-matched lognormal distribution (the Levy/Turnbull-Wakeman family of
//! approximations). The approximation is constant-time, scalar-only, `no_std`,
//! and suitable for an on-chain mark; it is not an exact arithmetic-Asian
//! distribution and does not model discrete oracle sampling error.
//!
//! All public values use [`crate::SCALE`] (`1e12`). Internal moment and option
//! calculations use `1e15` fixed point to retain the small variance of short
//! crypto settlement windows.

use crate::arithmetic::{european_prices_from_call, isqrt_u128};
use crate::constants::{SCALE, SCALE_HP, SCALE_HP_U};
use crate::error::SolMathError;
use crate::hp::{
    downscale_hp_to_std, downscale_hp_to_std_i, exp_fixed_hp, fp_div_hp_safe, fp_mul_hp_i,
    upscale_std_to_hp,
};
use crate::normal::norm_cdf_poly;
use crate::transcendental::{ln_1p_fixed, ln_fixed_i};

const MAX_PRICE: u128 = 100_000 * SCALE;
const MAX_RATE: u128 = 10 * SCALE;
const MAX_VOL: u128 = 100 * SCALE;
const MAX_TIME: u128 = 100 * SCALE;
const HP_TO_STD: i128 = 1_000;
// Below 1e-4 dimensionless carry over the averaging window, the closed form's
// O(B) numerator loses material digits before division by B. A second-order
// expansion has O(B^3) truncation instead and is stable at HP precision.
const SMALL_WINDOW_CARRY_HP: u128 = 100_000_000_000;

/// Use the certified SCALE logarithm for final option transforms. Moment
/// construction remains at HP precision; a SCALE log contributes at most a
/// few 1e-12 units, below the public price scale, while avoiding the
/// compensated HP polynomial's on-chain cost.
#[inline]
fn ln_hp_via_std(x_hp: i128) -> Result<i128, SolMathError> {
    let x_std = downscale_hp_to_std_i(x_hp);
    if x_std <= 0 {
        return Err(SolMathError::DomainError);
    }
    ln_fixed_i(x_std as u128)?
        .checked_mul(HP_TO_STD)
        .ok_or(SolMathError::Overflow)
}

/// `ln(1+x)` at HP interface precision via the cancellation-safe standard
/// kernel. `x` is first rounded once to the public 1e12 scale.
#[inline]
fn ln_1p_hp_via_std(x_hp: i128) -> Result<i128, SolMathError> {
    ln_1p_fixed(downscale_hp_to_std_i(x_hp))?
        .checked_mul(HP_TO_STD)
        .ok_or(SolMathError::Overflow)
}

/// Standard CDF lifted to HP scale for final price multiplication. The CDF
/// kernel is certified within two SCALE ULP and is roughly 20-35x cheaper on
/// SBF than the research HP polynomial.
#[inline]
fn cdf_hp_via_std(x_hp: i128) -> Result<i128, SolMathError> {
    norm_cdf_poly(downscale_hp_to_std_i(x_hp))?
        .checked_mul(HP_TO_STD)
        .ok_or(SolMathError::Overflow)
}

/// Prices and diagnostics for a moment-matched arithmetic-Asian option.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsianOptionResult {
    /// Discounted call price at `SCALE`.
    pub call: u128,
    /// Discounted put price at `SCALE`.
    pub put: u128,
    /// Risk-neutral expected final arithmetic average at `SCALE`.
    pub expected_average: u128,
    /// Log-variance of the matched lognormal distribution at `SCALE`.
    /// Zero denotes a deterministic settlement value at fixed-point precision.
    pub log_variance: u128,
}

#[derive(Debug, Clone, Copy)]
struct FutureAverageMoments {
    mean_hp: i128,
    variance_hp: i128,
}

/// `exp(x) - 1` at HP precision, with a series that preserves very small TWAP
/// exponents instead of subtracting two nearly equal `1e15` values.
#[inline]
fn expm1_hp(x: i128) -> Result<i128, SolMathError> {
    if x.unsigned_abs() < 10_000_000_000_000 {
        let x2 = fp_mul_hp_i(x, x)?;
        let x3 = fp_mul_hp_i(x2, x)?;
        let x4 = fp_mul_hp_i(x3, x)?;
        let x5 = fp_mul_hp_i(x4, x)?;
        let x6 = fp_mul_hp_i(x5, x)?;
        return x
            .checked_add(x2 / 2)
            .and_then(|v| v.checked_add(x3 / 6))
            .and_then(|v| v.checked_add(x4 / 24))
            .and_then(|v| v.checked_add(x5 / 120))
            .and_then(|v| v.checked_add(x6 / 720))
            .ok_or(SolMathError::Overflow);
    }
    exp_fixed_hp(x)?
        .checked_sub(SCALE_HP)
        .ok_or(SolMathError::Overflow)
}

/// `expm1(x) / x`, continuously extended to one at `x = 0`.
#[inline]
fn phi1_hp(x: i128) -> Result<i128, SolMathError> {
    if x == 0 {
        return Ok(SCALE_HP);
    }
    // Dividing expm1's tiny x^2 correction by x magnifies its fixed-point
    // rounding. Evaluate phi1's own series directly in short-window regimes.
    if x.unsigned_abs() < 10_000_000_000_000 {
        let x2 = fp_mul_hp_i(x, x)?;
        let x3 = fp_mul_hp_i(x2, x)?;
        let x4 = fp_mul_hp_i(x3, x)?;
        let x5 = fp_mul_hp_i(x4, x)?;
        return SCALE_HP
            .checked_add(x / 2)
            .and_then(|v| v.checked_add(x2 / 6))
            .and_then(|v| v.checked_add(x3 / 24))
            .and_then(|v| v.checked_add(x4 / 120))
            .and_then(|v| v.checked_add(x5 / 720))
            .ok_or(SolMathError::Overflow);
    }
    fp_div_hp_safe(expm1_hp(x)?, x)
}

/// Dimensionless second moment of an arithmetic average over a unit interval.
///
/// With `B = carry * tau` and `V = sigma^2 * tau`, this evaluates
///
/// ```text
/// J(B,V) = 2 int_0^1 exp((B+V)u) int_u^1 exp(Bv) dv du.
/// ```
///
/// A short bivariate series avoids cancellation in the settlement-window
/// regime where both `B` and `V` are tiny. Outside that regime, the closed form
/// uses three exponentials regardless of maturity.
fn average_second_factor_hp(b_window: i128, variance_window: i128) -> Result<i128, SolMathError> {
    let a = b_window
        .checked_add(variance_window)
        .ok_or(SolMathError::Overflow)?;
    let series_size = a
        .unsigned_abs()
        .checked_add(b_window.unsigned_abs())
        .ok_or(SolMathError::Overflow)?;

    // Exact series:
    // 2 sum_{m,n>=0} A^m B^n / [m! n! (m+1)(m+n+2)].
    // Total degree 8 has absolute remainder below 1.4e-11 when
    // |A|+|B| <= 0.25; ordinary short TWAP windows are many orders smaller.
    if series_size <= (SCALE_HP_U / 4) {
        const DEGREE: usize = 8;
        let mut sum = 0i128;
        let mut a_term = SCALE_HP;
        for m in 0..=DEGREE {
            let mut b_term = SCALE_HP;
            for n in 0..=(DEGREE - m) {
                let product = fp_mul_hp_i(a_term, b_term)?;
                let denominator = ((m + 1) * (m + n + 2)) as i128;
                let term = product.checked_mul(2).ok_or(SolMathError::Overflow)? / denominator;
                sum = sum.checked_add(term).ok_or(SolMathError::Overflow)?;

                if n < DEGREE - m {
                    b_term = fp_mul_hp_i(b_term, b_window)? / (n as i128 + 1);
                }
            }
            if m < DEGREE {
                a_term = fp_mul_hp_i(a_term, a)? / (m as i128 + 1);
            }
        }
        return Ok(sum);
    }

    if b_window.unsigned_abs() <= SMALL_WINDOW_CARRY_HP {
        if variance_window == 0 {
            return Ok(SCALE_HP);
        }
        let expm1_v = expm1_hp(variance_window)?;
        // J(0,V) = 2 * (expm1(V) - V) / V^2.
        let numerator = expm1_v
            .checked_sub(variance_window)
            .and_then(|v| v.checked_mul(2))
            .ok_or(SolMathError::Overflow)?;
        let v2 = fp_mul_hp_i(variance_window, variance_window)?;
        let j0 = fp_div_hp_safe(numerator, v2)?;
        if b_window == 0 {
            return Ok(j0);
        }

        // J(B,V) = J(0,V) + B J1(V) + B^2 J2(V) + O(B^3), where
        // J_B(0,V) = integral_0^1 exp(Vu) * (1 + 2u - 3u^2) du.
        // This branch is reached only for V > 0.25 (smaller V used the
        // bivariate series above), so the following closed moments do not
        // suffer origin cancellation.
        let exp_v = SCALE_HP
            .checked_add(expm1_v)
            .ok_or(SolMathError::Overflow)?;
        let i0 = fp_div_hp_safe(expm1_v, variance_window)?;
        let i1_numerator = fp_mul_hp_i(variance_window - SCALE_HP, exp_v)?
            .checked_add(SCALE_HP)
            .ok_or(SolMathError::Overflow)?;
        let i1 = fp_div_hp_safe(i1_numerator, v2)?;
        let v3 = fp_mul_hp_i(v2, variance_window)?;
        let i2_polynomial = v2
            .checked_sub(2 * variance_window)
            .and_then(|value| value.checked_add(2 * SCALE_HP))
            .ok_or(SolMathError::Overflow)?;
        let i2_numerator = fp_mul_hp_i(exp_v, i2_polynomial)?
            .checked_sub(2 * SCALE_HP)
            .ok_or(SolMathError::Overflow)?;
        let i2 = fp_div_hp_safe(i2_numerator, v3)?;
        let j1 = i0
            .checked_add(2 * i1)
            .and_then(|value| value.checked_sub(3 * i2))
            .ok_or(SolMathError::Overflow)?;
        let v4 = fp_mul_hp_i(v3, variance_window)?;
        let i3_polynomial = v3
            .checked_sub(3 * v2)
            .and_then(|value| value.checked_add(6 * variance_window))
            .and_then(|value| value.checked_sub(6 * SCALE_HP))
            .ok_or(SolMathError::Overflow)?;
        let i3_numerator = fp_mul_hp_i(exp_v, i3_polynomial)?
            .checked_add(6 * SCALE_HP)
            .ok_or(SolMathError::Overflow)?;
        let i3 = fp_div_hp_safe(i3_numerator, v4)?;
        // J2(V) = integral exp(Vu) * (1 + 3u + 3u^2 - 7u^3) / 3 du.
        let j2 = i0
            .checked_add(3 * i1)
            .and_then(|value| value.checked_add(3 * i2))
            .and_then(|value| value.checked_sub(7 * i3))
            .ok_or(SolMathError::Overflow)?
            / 3;
        let b2 = fp_mul_hp_i(b_window, b_window)?;
        let correction = fp_mul_hp_i(b_window, j1)?
            .checked_add(fp_mul_hp_i(b2, j2)?)
            .ok_or(SolMathError::Overflow)?;
        return j0
            .checked_add(correction)
            .filter(|value| *value > 0)
            .ok_or(SolMathError::DomainError);
    }

    // 2/B * [exp(B) phi1(B+V) - phi1(2B+V)].
    let first = fp_mul_hp_i(exp_fixed_hp(b_window)?, phi1_hp(a)?)?;
    let two_b_plus_v = b_window
        .checked_mul(2)
        .and_then(|v| v.checked_add(variance_window))
        .ok_or(SolMathError::Overflow)?;
    let bracket = first
        .checked_sub(phi1_hp(two_b_plus_v)?)
        .ok_or(SolMathError::Overflow)?;
    fp_div_hp_safe(
        bracket.checked_mul(2).ok_or(SolMathError::Overflow)?,
        b_window,
    )
}

fn future_average_moments_hp(
    spot_hp: i128,
    carry_hp: i128,
    sigma_hp: i128,
    time_hp: i128,
    averaging_time_hp: i128,
) -> Result<FutureAverageMoments, SolMathError> {
    let start_hp = time_hp
        .checked_sub(averaging_time_hp)
        .ok_or(SolMathError::DomainError)?;
    let sigma_sq_hp = fp_mul_hp_i(sigma_hp, sigma_hp)?;
    let b_window = fp_mul_hp_i(carry_hp, averaging_time_hp)?;
    let variance_window = fp_mul_hp_i(sigma_sq_hp, averaging_time_hp)?;

    // E[B] / S0 = exp(b * start) * phi1(b * tau).
    let mean_start = exp_fixed_hp(fp_mul_hp_i(carry_hp, start_hp)?)?;
    let mean_factor = fp_mul_hp_i(mean_start, phi1_hp(b_window)?)?;

    // E[B^2] / S0^2 = exp((2b + sigma^2) * start) * J(B,V).
    let second_rate = carry_hp
        .checked_mul(2)
        .and_then(|v| v.checked_add(sigma_sq_hp))
        .ok_or(SolMathError::Overflow)?;
    let second_start = exp_fixed_hp(fp_mul_hp_i(second_rate, start_hp)?)?;
    let second_factor = fp_mul_hp_i(
        second_start,
        average_second_factor_hp(b_window, variance_window)?,
    )?;
    let mean_factor_sq = fp_mul_hp_i(mean_factor, mean_factor)?;

    let variance_factor = if second_factor >= mean_factor_sq {
        second_factor - mean_factor_sq
    } else {
        // Rounding may make a genuinely deterministic/tiny-variance result a
        // few HP units negative. Material violations fail closed.
        let deficit = mean_factor_sq - second_factor;
        let tolerance = (mean_factor_sq / SCALE_HP).max(16);
        if deficit > tolerance {
            return Err(SolMathError::DomainError);
        }
        0
    };

    let mean_hp = fp_mul_hp_i(spot_hp, mean_factor)?;
    let spot_sq_hp = fp_mul_hp_i(spot_hp, spot_hp)?;
    let variance_hp = fp_mul_hp_i(spot_sq_hp, variance_factor)?;
    Ok(FutureAverageMoments {
        mean_hp,
        variance_hp,
    })
}

fn price_matched_lognormal_hp(
    mean_hp: i128,
    variance_hp: i128,
    strike_hp: i128,
    rate_hp: i128,
    time_hp: i128,
) -> Result<AsianOptionResult, SolMathError> {
    if mean_hp <= 0 || variance_hp < 0 || strike_hp <= 0 {
        return Err(SolMathError::DomainError);
    }

    let discount_hp = exp_fixed_hp(-fp_mul_hp_i(rate_hp, time_hp)?)?;
    let discounted_mean_hp = fp_mul_hp_i(mean_hp, discount_hp)?;
    let discounted_strike_hp = fp_mul_hp_i(strike_hp, discount_hp)?;

    let mean_sq_hp = fp_mul_hp_i(mean_hp, mean_hp)?;
    let cv_sq_hp = if variance_hp == 0 {
        0
    } else {
        fp_div_hp_safe(variance_hp, mean_sq_hp)?
    };
    let log_variance_hp = if cv_sq_hp == 0 {
        0
    } else {
        ln_1p_hp_via_std(cv_sq_hp)?
    };

    let call_hp = if log_variance_hp <= 0 {
        discounted_mean_hp
            .checked_sub(discounted_strike_hp)
            .unwrap_or(i128::MIN)
            .max(0)
    } else {
        let radicand = (log_variance_hp as u128)
            .checked_mul(SCALE_HP_U)
            .ok_or(SolMathError::Overflow)?;
        let sqrt_log_variance_hp = isqrt_u128(radicand) as i128;
        if sqrt_log_variance_hp == 0 {
            discounted_mean_hp
                .checked_sub(discounted_strike_hp)
                .unwrap_or(i128::MIN)
                .max(0)
        } else {
            let mean_strike_ratio = fp_div_hp_safe(mean_hp, strike_hp)?;
            let d1 = fp_div_hp_safe(
                ln_hp_via_std(mean_strike_ratio)? + log_variance_hp / 2,
                sqrt_log_variance_hp,
            )?;
            let d2 = d1 - sqrt_log_variance_hp;
            let undiscounted_call = fp_mul_hp_i(mean_hp, cdf_hp_via_std(d1)?)?
                .checked_sub(fp_mul_hp_i(strike_hp, cdf_hp_via_std(d2)?)?)
                .ok_or(SolMathError::Overflow)?;
            fp_mul_hp_i(discount_hp, undiscounted_call.max(0))?
        }
    };

    let discounted_mean = downscale_hp_to_std(discounted_mean_hp);
    let discounted_strike = downscale_hp_to_std_i(discounted_strike_hp);
    let call_std = downscale_hp_to_std_i(call_hp);
    let (call, put) = european_prices_from_call(call_std, discounted_mean, discounted_strike)?;

    Ok(AsianOptionResult {
        call,
        put,
        expected_average: downscale_hp_to_std(mean_hp),
        log_variance: downscale_hp_to_std(log_variance_hp),
    })
}

fn validate_inputs(
    s: u128,
    k: u128,
    r: u128,
    q: u128,
    sigma: u128,
    t: u128,
    averaging_time: u128,
    fixed_average: u128,
    fixed_weight: u128,
) -> Result<(), SolMathError> {
    if s > MAX_PRICE || k > MAX_PRICE || fixed_average > MAX_PRICE {
        return Err(SolMathError::Overflow);
    }
    if r > MAX_RATE || q > MAX_RATE || sigma > MAX_VOL || t > MAX_TIME {
        return Err(SolMathError::Overflow);
    }
    if s == 0 || k == 0 || sigma == 0 || fixed_weight > SCALE {
        return Err(SolMathError::DomainError);
    }
    if fixed_weight == 0 {
        if fixed_average != 0 {
            return Err(SolMathError::DomainError);
        }
    } else if fixed_average == 0 {
        return Err(SolMathError::DomainError);
    }
    if fixed_weight < SCALE {
        if t == 0 || averaging_time == 0 || averaging_time > t {
            return Err(SolMathError::DomainError);
        }
    } else if averaging_time != 0 {
        return Err(SolMathError::DomainError);
    }
    Ok(())
}

/// Price a continuously sampled, partially fixed arithmetic-Asian option.
///
/// The remaining arithmetic average is sampled over the interval
/// `[t - averaging_time, t]`. `fixed_weight` is in `[0, SCALE]`; the future
/// average receives weight `SCALE - fixed_weight`.
///
/// # Typical TWAP states
///
/// - **Before a 30-minute window:** set `averaging_time = 30 minutes`,
///   `fixed_weight = 0`, and `fixed_average = 0`.
/// - **12 minutes into that window:** set `t = averaging_time = 18 minutes`,
///   `fixed_weight = 0.4`, and `fixed_average` to the observed 12-minute TWAP.
/// - **Fully fixed:** set `fixed_weight = 1` and `averaging_time = 0`; the
///   function returns discounted intrinsic value.
///
/// Rates, times, prices, weights, and results are fixed point at `SCALE`.
/// `q` is the continuous yield, so risk-neutral carry is `r - q`.
///
/// # Approximation
///
/// The first two continuous-average moments under GBM are exact. Option prices
/// use a two-moment lognormal approximation, not an exact arithmetic-Asian law.
#[allow(clippy::too_many_arguments)]
pub fn arithmetic_asian_price(
    s: u128,
    k: u128,
    r: u128,
    q: u128,
    sigma: u128,
    t: u128,
    averaging_time: u128,
    fixed_average: u128,
    fixed_weight: u128,
) -> Result<AsianOptionResult, SolMathError> {
    validate_inputs(
        s,
        k,
        r,
        q,
        sigma,
        t,
        averaging_time,
        fixed_average,
        fixed_weight,
    )?;

    let strike_hp = upscale_std_to_hp(k)?;
    let rate_hp = upscale_std_to_hp(r)?;
    let time_hp = upscale_std_to_hp(t)?;

    if fixed_weight == SCALE {
        return price_matched_lognormal_hp(
            upscale_std_to_hp(fixed_average)?,
            0,
            strike_hp,
            rate_hp,
            time_hp,
        );
    }

    let spot_hp = upscale_std_to_hp(s)?;
    let yield_hp = upscale_std_to_hp(q)?;
    let sigma_hp = upscale_std_to_hp(sigma)?;
    let averaging_time_hp = upscale_std_to_hp(averaging_time)?;
    let carry_hp = rate_hp
        .checked_sub(yield_hp)
        .ok_or(SolMathError::Overflow)?;
    let future =
        future_average_moments_hp(spot_hp, carry_hp, sigma_hp, time_hp, averaging_time_hp)?;

    let fixed_weight_hp = upscale_std_to_hp(fixed_weight)?;
    let future_weight_hp = SCALE_HP
        .checked_sub(fixed_weight_hp)
        .ok_or(SolMathError::DomainError)?;
    let fixed_average_hp = upscale_std_to_hp(fixed_average)?;
    let mean_hp = fp_mul_hp_i(fixed_weight_hp, fixed_average_hp)?
        .checked_add(fp_mul_hp_i(future_weight_hp, future.mean_hp)?)
        .ok_or(SolMathError::Overflow)?;
    let future_weight_sq_hp = fp_mul_hp_i(future_weight_hp, future_weight_hp)?;
    let variance_hp = fp_mul_hp_i(future_weight_sq_hp, future.variance_hp)?;

    price_matched_lognormal_hp(mean_hp, variance_hp, strike_hp, rate_hp, time_hp)
}

/// TWAP-named alias of [`arithmetic_asian_price`].
///
/// This spelling is intended for protocols whose contract specification calls
/// the settlement average a TWAP rather than an arithmetic-Asian underlying.
#[allow(clippy::too_many_arguments)]
#[inline]
pub fn twap_option_price(
    s: u128,
    k: u128,
    r: u128,
    q: u128,
    sigma: u128,
    t: u128,
    averaging_time: u128,
    fixed_average: u128,
    fixed_weight: u128,
) -> Result<AsianOptionResult, SolMathError> {
    arithmetic_asian_price(
        s,
        k,
        r,
        q,
        sigma,
        t,
        averaging_time,
        fixed_average,
        fixed_weight,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const S: u128 = 100 * SCALE;
    const K: u128 = 100 * SCALE;
    const R: u128 = 5 * SCALE / 100;
    const Q: u128 = 2 * SCALE / 100;
    const SIGMA: u128 = 40 * SCALE / 100;

    #[test]
    fn twap_alias_is_exact() {
        let a = arithmetic_asian_price(S, K, R, Q, SIGMA, SCALE, SCALE / 2, 0, 0);
        let b = twap_option_price(S, K, R, Q, SIGMA, SCALE, SCALE / 2, 0, 0);
        assert_eq!(a, b);
    }

    #[test]
    fn fully_fixed_is_discounted_intrinsic() {
        let result =
            arithmetic_asian_price(S, K, R, Q, SIGMA, SCALE / 2, 0, 110 * SCALE, SCALE).unwrap();
        let discount = exp_fixed_hp(
            -fp_mul_hp_i(
                upscale_std_to_hp(R).unwrap(),
                upscale_std_to_hp(SCALE / 2).unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
        let discounted_average =
            downscale_hp_to_std(fp_mul_hp_i(110 * SCALE_HP, discount).unwrap());
        let discounted_strike = downscale_hp_to_std(fp_mul_hp_i(100 * SCALE_HP, discount).unwrap());
        let expected = discounted_average - discounted_strike;
        assert_eq!(result.call, expected);
        assert_eq!(result.put, 0);
        assert_eq!(result.expected_average, 110 * SCALE);
        assert_eq!(result.log_variance, 0);
    }

    #[test]
    fn output_satisfies_exact_discounted_average_parity() {
        let result = arithmetic_asian_price(
            S,
            105 * SCALE,
            R,
            Q,
            SIGMA,
            SCALE,
            SCALE / 12,
            98 * SCALE,
            SCALE / 3,
        )
        .unwrap();
        let discount = exp_fixed_hp(
            -fp_mul_hp_i(
                upscale_std_to_hp(R).unwrap(),
                upscale_std_to_hp(SCALE).unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
        let mean_disc = downscale_hp_to_std(
            fp_mul_hp_i(
                upscale_std_to_hp(result.expected_average).unwrap(),
                discount,
            )
            .unwrap(),
        );
        let strike_disc = downscale_hp_to_std(
            fp_mul_hp_i(upscale_std_to_hp(105 * SCALE).unwrap(), discount).unwrap(),
        );
        assert_eq!(
            result.call as i128 - result.put as i128,
            mean_disc as i128 - strike_disc as i128
        );
    }

    #[test]
    fn fixing_more_of_the_average_reduces_variance() {
        let unseasoned =
            arithmetic_asian_price(S, K, R, Q, SIGMA, SCALE / 365, SCALE / 365, 0, 0).unwrap();
        let seasoned =
            arithmetic_asian_price(S, K, R, Q, SIGMA, SCALE / 730, SCALE / 730, S, SCALE / 2)
                .unwrap();
        assert!(seasoned.log_variance < unseasoned.log_variance);
    }

    #[test]
    fn one_raw_carry_above_series_seam_is_stable() {
        // Retained adversarial vector: B is only 7.8e-14 while V is above the
        // bivariate-series seam. Direct division by B previously moved the
        // price by about $9 because the closed-form numerator had lost its
        // significant digits.
        let result = arithmetic_asian_price(
            931_700_559_528_446,
            1_717_008_561_609_268,
            120_633_804_209,
            120_633_804_208,
            1_871_600_373_667,
            1_795_710_902_611,
            78_071_066_562,
            1_286_913_254_987_408,
            914_756_059_943,
        )
        .unwrap();
        assert!(result.call.abs_diff(298_725_289_603_127) <= 5_000);
        assert!(result.put.abs_diff(669_434_522_411_088) <= 5_000);
        assert_eq!(result.expected_average, 1_256_633_525_268_358);
        assert!(result.log_variance.abs_diff(1_027_764_862_497) <= 2);
    }

    #[test]
    fn invalid_average_state_fails_closed() {
        assert_eq!(
            arithmetic_asian_price(S, K, R, Q, SIGMA, SCALE, SCALE + 1, 0, 0),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            arithmetic_asian_price(S, K, R, Q, SIGMA, SCALE, SCALE, S, 0),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            arithmetic_asian_price(S, K, R, Q, SIGMA, SCALE, 0, S, SCALE / 2),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            arithmetic_asian_price(S, K, R, Q, SIGMA, SCALE, 1, S, SCALE + 1),
            Err(SolMathError::DomainError)
        );
    }
}
