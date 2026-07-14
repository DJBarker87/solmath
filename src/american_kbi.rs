//! Fully on-chain American pricing by Kim Boundary Integration (KBI).
//!
//! The normalized American-put exercise boundary is reconstructed from
//! `(r, q, sigma, T)` with a fixed, singularity-cancelled Gaussian history
//! rule and a compact QdFp-regularized premium cubature.
//! Calls use exact American put-call duality. There is no live surface, matrix,
//! account, operator upload, trusted builder, or other off-chain input.

use crate::american_kbi_data::*;
use crate::arithmetic::fp_sqrt;
use crate::constants::{SCALE, SCALE_I};
use crate::error::SolMathError;
use crate::transcendental::{exp_fixed_i, ln_fixed_i};

/// Number of graded time nodes used to reconstruct the exercise boundary.
pub const AMERICAN_KBI_NODES: usize = KBI_NODES;
/// Number of quadrature points used for the final early-exercise premium.
pub const AMERICAN_KBI_PRICE_POINTS: usize = KBI_PRICE_POINTS;
/// SHA-256 of the generated, parameter-independent KBI geometry artifact.
pub const AMERICAN_KBI_ARTIFACT_SHA256: [u8; 32] = KBI_DATA_SHA256;

const MAX_RATE: u128 = 120_000_000_000;
const MIN_SIGMA: u128 = 100_000_000_000;
const MAX_SIGMA: u128 = 1_200_000_000_000;
const MIN_MATURITY: u128 = 30 * SCALE / 365;
const MAX_MATURITY: u128 = 2 * SCALE;
// Certified quote domain: |ln(S/K)| <= 0.75. Calls reach this same guard
// through exact put-call duality, where the normalized ratio is K/S.
const MIN_NORMALIZED_SPOT_Q: i64 = 519_372_517_311;
const MAX_NORMALIZED_SPOT_Q: i64 = 2_327_666_134_268;
const MAX_SAFE_QUOTE: u128 = u128::MAX / Q_ONE as u128;

// Internal arithmetic is Q40. Unlike the crate's public decimal SCALE, every
// normalized multiply is a single wide multiply followed by a constant shift.
const Q_BITS: u32 = 40;
const Q_ONE: i64 = 1i64 << Q_BITS;
const SCALE_TO_Q_RECIP_Q48: i128 = 309_485_009_821_345;
const PDF_ZERO_Q: i64 = 438_641_676_113;

const BOUNDARY_FLOOR_Q: i64 = 1_099_512; // 1e-6
const NEGLIGIBLE_PREMIUM_Q: i64 = 1_099_512; // rT <= 1e-6
const FIRST_NODE_HIGH_SCALE_Q: i64 = 2 * Q_ONE;
const FIRST_NODE_LOW_SCALE_Q: i64 = 3 * Q_ONE / 4;
const THIRD_NODE_EXTRAPOLATION_Q: i64 = 37 * Q_ONE / 40;
const BOUNDARY_HIGH_SWITCH_Q: i64 = 95 * Q_ONE / 100;
const STEP_LOW_Q: i64 = 35 * Q_ONE / 100;
const STEP_HIGH_Q: i64 = 165 * Q_ONE / 100;

const EXP_C0_Q: i64 = Q_ONE;
const EXP_C1_Q: i64 = Q_ONE;
const EXP_C2_Q: i64 = 549_755_813_888;
const EXP_C3_Q: i64 = 183_251_937_963;
const EXP_C4_Q: i64 = 45_812_984_491;
const EXP_C5_Q: i64 = 9_162_596_898;
const EXP_C6_Q: i64 = 1_527_099_483;

/// American option leg priced by [`american_kbi_price`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmericanKbiKind {
    /// American call, evaluated through exact call-put duality.
    Call,
    /// American put.
    Put,
}

#[derive(Clone, Copy)]
struct Parameters {
    rate: i64,
    yield_rate: i64,
    sigma_sqrt_maturity: i64,
    inverse_sigma_sqrt_maturity: i64,
    sqrt_maturity_over_sigma: i64,
    rate_maturity: i64,
    yield_maturity: i64,
    drift_maturity: i64,
}

struct ExerciseBoundary {
    values: [i64; KBI_NODES + 1],
    log_values: [i64; KBI_NODES + 1],
}

#[inline(always)]
fn round_shift(value: i128, bits: u32) -> i128 {
    let half = 1i128 << (bits - 1);
    if value >= 0 {
        (value + half) >> bits
    } else {
        -((-value + half) >> bits)
    }
}

#[inline(always)]
fn qmul(a: i64, b: i64) -> i64 {
    let value = (a as i128 * b as i128) >> Q_BITS;
    debug_assert!(i64::try_from(value).is_ok());
    value as i64
}

#[inline(always)]
fn qdiv(a: i64, b: i64) -> Result<i64, SolMathError> {
    if b == 0 {
        return Err(SolMathError::DivisionByZero);
    }
    let value = ((a as i128) << Q_BITS) / b as i128;
    i64::try_from(value).map_err(|_| SolMathError::Overflow)
}

#[inline(always)]
fn scale_to_q(value: i128) -> Result<i64, SolMathError> {
    let converted = round_shift(value * SCALE_TO_Q_RECIP_Q48, 48);
    i64::try_from(converted).map_err(|_| SolMathError::Overflow)
}

#[inline(always)]
fn q_to_scale(value: i64) -> i128 {
    round_shift(value as i128 * SCALE_I, Q_BITS)
}

#[inline(always)]
fn qln(value: i64) -> Result<i64, SolMathError> {
    if value <= 0 {
        return Err(SolMathError::DomainError);
    }
    scale_to_q(ln_fixed_i(q_to_scale(value) as u128)?)
}

#[inline(never)]
fn qsqrt(value: i64) -> Result<i64, SolMathError> {
    if value <= 0 {
        return Err(SolMathError::DomainError);
    }
    scale_to_q(fp_sqrt(q_to_scale(value) as u128)? as i128)
}

#[inline(never)]
fn qexp(value: i64) -> Result<i64, SolMathError> {
    scale_to_q(exp_fixed_i(q_to_scale(value))?)
}

#[inline(always)]
fn boundary_cdf_and_pdf(value: i64) -> (i64, i64) {
    const STEP_BITS: u32 = Q_BITS - 3;
    // At six standard deviations the omitted probability is below 1e-9,
    // far under the boundary kernel's 5.5e-6 certified interpolation error.
    // Saturating there also removes sub-unit coefficient wobble in the tail.
    const LIMIT: i64 = 6 * Q_ONE;
    if value <= -LIMIT {
        return (0, 0);
    }
    if value >= LIMIT {
        return (Q_ONE, 0);
    }

    let absolute = value.abs();
    let index = (absolute >> STEP_BITS) as usize;
    let interval_start = (index as i64) << STEP_BITS;
    let coordinate = (absolute - interval_start) << 3;
    let a = KBI_NORMAL_HERMITE_A[index];
    let b = KBI_NORMAL_HERMITE_B[index];
    let c = KBI_NORMAL_HERMITE_C[index];
    let d = KBI_NORMAL_HERMITE_D[index];
    let cdf_positive = (qmul(qmul(qmul(a, coordinate) + b, coordinate) + c, coordinate) + d)
        .clamp(Q_ONE / 2, Q_ONE);
    let coordinate_derivative = qmul(qmul(3 * a, coordinate) + 2 * b, coordinate) + c;
    let pdf = (8 * coordinate_derivative).clamp(0, PDF_ZERO_Q);
    (
        if value >= 0 {
            cdf_positive
        } else {
            Q_ONE - cdf_positive
        },
        pdf,
    )
}

#[inline(always)]
fn premium_cdf(value: i64) -> i64 {
    const STEP_BITS: u32 = Q_BITS - 3;
    const LIMIT: i64 = 6 * Q_ONE;
    if value <= -LIMIT {
        return 0;
    }
    if value >= LIMIT {
        return Q_ONE;
    }

    let absolute = value.abs();
    let index = (absolute >> STEP_BITS) as usize;
    let interval_start = (index as i64) << STEP_BITS;
    let coordinate = (absolute - interval_start) << 3;
    let a = KBI_NORMAL_HERMITE_A[index];
    let b = KBI_NORMAL_HERMITE_B[index];
    let c = KBI_NORMAL_HERMITE_C[index];
    let d = KBI_NORMAL_HERMITE_D[index];
    let cdf_positive = (qmul(qmul(qmul(a, coordinate) + b, coordinate) + c, coordinate) + d)
        .clamp(Q_ONE / 2, Q_ONE);
    if value >= 0 {
        cdf_positive
    } else {
        Q_ONE - cdf_positive
    }
}

/// Sixth-order exponential on the live `[-0.24, 0]` discount domain.
#[inline(never)]
fn exp_small(value: i64) -> Result<i64, SolMathError> {
    if !(-3 * Q_ONE / 10..=0).contains(&value) {
        return Err(SolMathError::DomainError);
    }
    let polynomial = EXP_C5_Q + qmul(value, EXP_C6_Q);
    let polynomial = EXP_C4_Q + qmul(value, polynomial);
    let polynomial = EXP_C3_Q + qmul(value, polynomial);
    let polynomial = EXP_C2_Q + qmul(value, polynomial);
    let polynomial = EXP_C1_Q + qmul(value, polynomial);
    Ok((EXP_C0_Q + qmul(value, polynomial)).max(0))
}

/// Degree-five discount kernel. On `[-0.24, 0]` the Taylor remainder is
/// bounded by `0.24^6 / 6! < 2.66e-7`; the European control value retains the
/// degree-six path above.
#[inline(always)]
fn exp_kernel(value: i64) -> Result<i64, SolMathError> {
    if !(-3 * Q_ONE / 10..=0).contains(&value) {
        return Err(SolMathError::DomainError);
    }
    let polynomial = EXP_C4_Q + qmul(value, EXP_C5_Q);
    let polynomial = EXP_C3_Q + qmul(value, polynomial);
    let polynomial = EXP_C2_Q + qmul(value, polynomial);
    let polynomial = EXP_C1_Q + qmul(value, polynomial);
    Ok((EXP_C0_Q + qmul(value, polynomial)).max(0))
}

fn parameters(
    rate: u128,
    dividend_yield: u128,
    sigma: u128,
    maturity: u128,
) -> Result<Parameters, SolMathError> {
    if rate > MAX_RATE
        || dividend_yield > MAX_RATE
        || !(MIN_SIGMA..=MAX_SIGMA).contains(&sigma)
        || !(MIN_MATURITY..=MAX_MATURITY).contains(&maturity)
        || rate > i128::MAX as u128
        || dividend_yield > i128::MAX as u128
        || sigma > i128::MAX as u128
        || maturity > i128::MAX as u128
    {
        return Err(SolMathError::DomainError);
    }

    let rate = scale_to_q(rate as i128)?;
    let yield_rate = scale_to_q(dividend_yield as i128)?;
    let sigma = scale_to_q(sigma as i128)?;
    let maturity = scale_to_q(maturity as i128)?;
    let sqrt_maturity = qsqrt(maturity)?;
    let sigma_sqrt_maturity = qmul(sigma, sqrt_maturity);
    let sigma_squared = qmul(sigma, sigma);
    let drift = rate
        .checked_sub(yield_rate)
        .and_then(|value| value.checked_add(sigma_squared / 2))
        .ok_or(SolMathError::Overflow)?;

    Ok(Parameters {
        rate,
        yield_rate,
        sigma_sqrt_maturity,
        inverse_sigma_sqrt_maturity: qdiv(Q_ONE, sigma_sqrt_maturity)?,
        sqrt_maturity_over_sigma: qdiv(sqrt_maturity, sigma)?,
        rate_maturity: qmul(rate, maturity),
        yield_maturity: qmul(yield_rate, maturity),
        drift_maturity: qmul(drift, maturity),
    })
}

#[inline(never)]
fn european_put_normalized(
    normalized_spot: i64,
    parameters: Parameters,
) -> Result<i64, SolMathError> {
    let log_spot = qln(normalized_spot)?;
    let d1 = qmul(
        log_spot
            .checked_add(parameters.drift_maturity)
            .ok_or(SolMathError::Overflow)?,
        parameters.inverse_sigma_sqrt_maturity,
    );
    let d2 = d1
        .checked_sub(parameters.sigma_sqrt_maturity)
        .ok_or(SolMathError::Overflow)?;
    let n_d1 = premium_cdf(-d1);
    let n_d2 = premium_cdf(-d2);
    let discounted_strike = qmul(exp_small(-parameters.rate_maturity)?, n_d2);
    let discounted_spot = qmul(
        normalized_spot,
        qmul(exp_small(-parameters.yield_maturity)?, n_d1),
    );
    Ok(discounted_strike
        .checked_sub(discounted_spot)
        .ok_or(SolMathError::Overflow)?
        .clamp(0, Q_ONE))
}

#[inline(always)]
fn expiry_boundary(parameters: Parameters) -> Result<i64, SolMathError> {
    if parameters.yield_rate > parameters.rate {
        Ok(qdiv(parameters.rate, parameters.yield_rate)?.max(BOUNDARY_FLOOR_Q))
    } else {
        Ok(Q_ONE)
    }
}

#[inline(never)]
fn boundary_residual(
    index: usize,
    candidate: i64,
    log_boundary: &[i64; KBI_NODES + 1],
    coefficient: &[i64; KBI_NODES + 1],
    parameters: Parameters,
    node_discount: i64,
    boundary_discount: &[i64; KBI_BOUNDARY_ORDER],
) -> Result<(i64, i64), SolMathError> {
    let time_fraction = KBI_TIME_FRACTION[index];
    let inverse_volatility_at_node = qmul(
        parameters.inverse_sigma_sqrt_maturity,
        KBI_NODES as i64 * Q_ONE / index as i64,
    );
    let inverse_candidate = qdiv(Q_ONE, candidate)?;
    let log_candidate = qln(candidate)?;
    let drift_at_node = qmul(parameters.drift_maturity, time_fraction);
    let european_d1 = qmul(
        log_candidate
            .checked_add(drift_at_node)
            .ok_or(SolMathError::Overflow)?,
        inverse_volatility_at_node,
    );
    let (european_cdf, european_pdf) = boundary_cdf_and_pdf(-european_d1);
    let mut residual = Q_ONE
        .checked_sub(qmul(node_discount, european_cdf))
        .ok_or(SolMathError::Overflow)?;
    let mut derivative = qmul(
        qmul(node_discount, european_pdf),
        qmul(inverse_candidate, inverse_volatility_at_node),
    );

    let mut regular_sum = 0i64;
    let mut regular_derivative_sum = 0i64;
    let mut singular_sum = 0i64;
    let mut singular_derivative_sum = 0i64;
    let rate_over_boundary = qmul(parameters.rate, inverse_candidate);
    let rate_over_boundary_squared = qmul(rate_over_boundary, inverse_candidate);
    let candidate_coefficient = parameters
        .yield_rate
        .checked_sub(rate_over_boundary)
        .ok_or(SolMathError::Overflow)?;
    for point in 0..KBI_BOUNDARY_ORDER {
        let flat_index = (index - 1) * KBI_BOUNDARY_ORDER + point;
        let lag_fraction = KBI_BOUNDARY_LAG_FRACTION[flat_index];
        let discount = boundary_discount[point];
        let left = KBI_BOUNDARY_LEFT[flat_index] as usize;
        let fraction = KBI_BOUNDARY_FRACTION[flat_index];
        let right_log_boundary = if left + 1 == index {
            log_candidate
        } else {
            log_boundary[left + 1]
        };
        let log_boundary_sample = log_boundary[left]
            .checked_add(qmul(right_log_boundary - log_boundary[left], fraction))
            .ok_or(SolMathError::Overflow)?;
        let right_coefficient = if left + 1 == index {
            candidate_coefficient
        } else {
            coefficient[left + 1]
        };
        let coefficient_sample = coefficient[left]
            .checked_add(qmul(right_coefficient - coefficient[left], fraction))
            .ok_or(SolMathError::Overflow)?;
        let drift_lag = qmul(parameters.drift_maturity, lag_fraction);
        let numerator = log_candidate
            .checked_sub(log_boundary_sample)
            .and_then(|value| value.checked_add(drift_lag))
            .ok_or(SolMathError::Overflow)?;
        let inverse_volatility_lag = qmul(
            parameters.inverse_sigma_sqrt_maturity,
            KBI_BOUNDARY_INV_SQRT_LAG_FRACTION[flat_index],
        );
        let d1 = qmul(numerator, inverse_volatility_lag);
        let (cdf, pdf) = boundary_cdf_and_pdf(-d1);
        let d_candidate = qmul(
            qmul(inverse_candidate, inverse_volatility_lag),
            KBI_BOUNDARY_CANDIDATE_LOG_FACTOR[flat_index],
        );
        let discounted_pdf = qmul(discount, pdf);
        let regular = qmul(discount, cdf);
        let integration_weight = KBI_BOUNDARY_REGULAR_WEIGHT[flat_index];
        regular_sum = regular_sum
            .checked_add(qmul(integration_weight, regular))
            .ok_or(SolMathError::Overflow)?;
        regular_derivative_sum = regular_derivative_sum
            .checked_add(qmul(integration_weight, -qmul(discounted_pdf, d_candidate)))
            .ok_or(SolMathError::Overflow)?;
        let coefficient_derivative = qmul(
            rate_over_boundary_squared,
            KBI_BOUNDARY_CANDIDATE_COEFFICIENT_FACTOR[flat_index],
        );
        singular_sum = singular_sum
            .checked_add(qmul(
                KBI_BOUNDARY_SINGULAR_WEIGHT[flat_index],
                qmul(discounted_pdf, coefficient_sample),
            ))
            .ok_or(SolMathError::Overflow)?;
        let density_derivative = coefficient_derivative
            .checked_sub(qmul(qmul(coefficient_sample, d1), d_candidate))
            .ok_or(SolMathError::Overflow)?;
        singular_derivative_sum = singular_derivative_sum
            .checked_add(qmul(
                KBI_BOUNDARY_SINGULAR_WEIGHT[flat_index],
                qmul(discounted_pdf, density_derivative),
            ))
            .ok_or(SolMathError::Overflow)?;
    }

    residual = residual
        .checked_sub(qmul(parameters.yield_maturity, regular_sum))
        .and_then(|value| {
            value.checked_add(qmul(parameters.sqrt_maturity_over_sigma, singular_sum))
        })
        .ok_or(SolMathError::Overflow)?;
    derivative = derivative
        .checked_sub(qmul(parameters.yield_maturity, regular_derivative_sum))
        .and_then(|value| {
            value.checked_add(qmul(
                parameters.sqrt_maturity_over_sigma,
                singular_derivative_sum,
            ))
        })
        .ok_or(SolMathError::Overflow)?;
    Ok((residual, derivative))
}

#[inline(never)]
fn exercise_boundary(parameters: Parameters) -> Result<ExerciseBoundary, SolMathError> {
    let mut boundary = [0i64; KBI_NODES + 1];
    let mut log_boundary = [0i64; KBI_NODES + 1];
    let mut coefficient = [0i64; KBI_NODES + 1];
    let mut inverse_boundary = [0i64; KBI_NODES + 1];
    boundary[0] = expiry_boundary(parameters)?;
    inverse_boundary[0] = qdiv(Q_ONE, boundary[0])?;
    log_boundary[0] = qln(boundary[0])?;
    coefficient[0] = parameters
        .yield_rate
        .checked_sub(qmul(parameters.rate, inverse_boundary[0]))
        .ok_or(SolMathError::Overflow)?;

    // For lag=t_i*y_k^2 on the quadratic grid, each discount is
    // exp(-qT*y_k^2*i^2/N^2).  Advance it with two multiplications per node
    // after evaluating only one exponential per Gaussian ordinate.
    let mut boundary_discount = [Q_ONE; KBI_BOUNDARY_ORDER];
    let mut boundary_discount_ratio = [Q_ONE; KBI_BOUNDARY_ORDER];
    let mut boundary_discount_ratio_growth = [Q_ONE; KBI_BOUNDARY_ORDER];
    for point in 0..KBI_BOUNDARY_ORDER {
        let base = exp_kernel(-qmul(
            parameters.yield_maturity,
            KBI_BOUNDARY_Y_SQUARED_OVER_NODES_SQUARED[point],
        ))?;
        boundary_discount_ratio[point] = base;
        boundary_discount_ratio_growth[point] = qmul(base, base);
    }
    let nodes_squared = (KBI_NODES * KBI_NODES) as i64;
    let node_discount_base = exp_kernel(-parameters.yield_maturity / nodes_squared)?;
    let mut node_discount = Q_ONE;
    let mut node_discount_ratio = node_discount_base;
    let node_discount_ratio_growth = qmul(node_discount_base, node_discount_base);

    for index in 1..=KBI_NODES {
        for point in 0..KBI_BOUNDARY_ORDER {
            boundary_discount[point] =
                qmul(boundary_discount[point], boundary_discount_ratio[point]);
            boundary_discount_ratio[point] = qmul(
                boundary_discount_ratio[point],
                boundary_discount_ratio_growth[point],
            );
        }
        node_discount = qmul(node_discount, node_discount_ratio);
        node_discount_ratio = qmul(node_discount_ratio, node_discount_ratio_growth);
        let upper = boundary[index - 1].max(BOUNDARY_FLOOR_Q);
        let mut candidate = if index == 1 {
            let predictor_scale = if boundary[0] > BOUNDARY_HIGH_SWITCH_Q {
                FIRST_NODE_HIGH_SCALE_Q
            } else {
                FIRST_NODE_LOW_SCALE_Q
            };
            let sqrt_fraction = index as i64 * Q_ONE / KBI_NODES as i64;
            let exponent = -qmul(
                predictor_scale,
                qmul(parameters.sigma_sqrt_maturity, sqrt_fraction),
            );
            qmul(upper, qexp(exponent)?)
        } else {
            let extrapolated = qmul(
                qmul(boundary[index - 1], boundary[index - 1]),
                inverse_boundary[index - 2],
            );
            if index == 3 {
                boundary[index - 1]
                    + qmul(
                        extrapolated - boundary[index - 1],
                        THIRD_NODE_EXTRAPOLATION_Q,
                    )
            } else {
                extrapolated
            }
        };
        candidate = candidate.clamp(BOUNDARY_FLOOR_Q, (upper - 1).max(BOUNDARY_FLOOR_Q));
        let steps = if index <= 2 { 2 } else { 1 };
        for _ in 0..steps {
            let (residual, derivative) = boundary_residual(
                index,
                candidate,
                &log_boundary,
                &coefficient,
                parameters,
                node_discount,
                &boundary_discount,
            )?;
            let mut proposal = if derivative.abs() < 1 {
                candidate / 2
            } else {
                candidate
                    .checked_sub(qdiv(residual, derivative)?)
                    .ok_or(SolMathError::Overflow)?
            };
            proposal = proposal.clamp(BOUNDARY_FLOOR_Q, (upper - 1).max(BOUNDARY_FLOOR_Q));
            let low = qmul(candidate, STEP_LOW_Q);
            let high = qmul(candidate, STEP_HIGH_Q);
            candidate = proposal.clamp(low.max(BOUNDARY_FLOOR_Q), high.min(upper));
        }
        boundary[index] = candidate.min(upper);
        inverse_boundary[index] = qdiv(Q_ONE, boundary[index])?;
        log_boundary[index] = qln(boundary[index])?;
        coefficient[index] = parameters
            .yield_rate
            .checked_sub(qmul(parameters.rate, inverse_boundary[index]))
            .ok_or(SolMathError::Overflow)?;
    }
    Ok(ExerciseBoundary {
        values: boundary,
        log_values: log_boundary,
    })
}

#[inline(never)]
fn american_put_normalized(
    normalized_spot: i64,
    parameters: Parameters,
) -> Result<i64, SolMathError> {
    let intrinsic = (Q_ONE - normalized_spot).max(0);
    let european = european_put_normalized(normalized_spot, parameters)?;
    // The positive early-exercise premium is bounded above by rT in these
    // normalized units. This analytic gate avoids an ill-conditioned boundary
    // only when its maximum dollar contribution is already negligible.
    if parameters.rate_maturity <= NEGLIGIBLE_PREMIUM_Q {
        return Ok(european.max(intrinsic));
    }

    let boundary = exercise_boundary(parameters)?;
    if normalized_spot <= boundary.values[KBI_NODES] {
        return Ok(intrinsic);
    }

    let log_spot = qln(normalized_spot)?;
    let mut rate_sum = 0i64;
    let mut yield_sum = 0i64;
    for point in 0..KBI_PRICE_POINTS {
        let left = KBI_PRICE_BOUNDARY_LEFT[point] as usize;
        let fraction = KBI_PRICE_BOUNDARY_FRACTION[point];
        let log_boundary_sample = boundary.log_values[left]
            .checked_add(qmul(
                boundary.log_values[left + 1] - boundary.log_values[left],
                fraction,
            ))
            .ok_or(SolMathError::Overflow)?;
        let lag_fraction = KBI_PRICE_LAG_FRACTION[point];
        let inverse_volatility_lag = qmul(
            parameters.inverse_sigma_sqrt_maturity,
            KBI_PRICE_INV_SQRT_LAG_FRACTION[point],
        );
        let drift_lag = qmul(parameters.drift_maturity, lag_fraction);
        let numerator = log_spot
            .checked_sub(log_boundary_sample)
            .and_then(|value| value.checked_add(drift_lag))
            .ok_or(SolMathError::Overflow)?;
        let d1 = qmul(numerator, inverse_volatility_lag);
        let volatility_lag = qmul(
            parameters.sigma_sqrt_maturity,
            KBI_PRICE_SQRT_LAG_FRACTION[point],
        );
        let d2 = d1
            .checked_sub(volatility_lag)
            .ok_or(SolMathError::Overflow)?;
        let n_d1 = premium_cdf(-d1);
        let n_d2 = premium_cdf(-d2);
        let rate_discount = exp_kernel(-qmul(parameters.rate_maturity, lag_fraction))?;
        let yield_discount = exp_kernel(-qmul(parameters.yield_maturity, lag_fraction))?;
        let weight = KBI_PRICE_WEIGHT[point];
        rate_sum = rate_sum
            .checked_add(qmul(weight, qmul(rate_discount, n_d2)))
            .ok_or(SolMathError::Overflow)?;
        yield_sum = yield_sum
            .checked_add(qmul(weight, qmul(yield_discount, n_d1)))
            .ok_or(SolMathError::Overflow)?;
    }
    let rate_premium = qmul(parameters.rate_maturity, rate_sum);
    let yield_premium = qmul(qmul(parameters.yield_maturity, normalized_spot), yield_sum);
    Ok(european
        .checked_add(rate_premium)
        .and_then(|value| value.checked_sub(yield_premium))
        .ok_or(SolMathError::Overflow)?
        .clamp(intrinsic, Q_ONE))
}

#[inline(never)]
fn price_put_leg(
    spot: u128,
    strike: u128,
    rate: u128,
    dividend_yield: u128,
    sigma: u128,
    maturity: u128,
) -> Result<u128, SolMathError> {
    if spot == 0 || strike == 0 || spot > MAX_SAFE_QUOTE || strike > MAX_SAFE_QUOTE {
        return Err(SolMathError::DomainError);
    }
    let normalized_spot = spot
        .checked_mul(Q_ONE as u128)
        .ok_or(SolMathError::Overflow)?
        / strike;
    if normalized_spot < MIN_NORMALIZED_SPOT_Q as u128
        || normalized_spot > MAX_NORMALIZED_SPOT_Q as u128
    {
        return Err(SolMathError::DomainError);
    }
    let normalized_price = american_put_normalized(
        normalized_spot as i64,
        parameters(rate, dividend_yield, sigma, maturity)?,
    )?;
    strike
        .checked_mul(normalized_price as u128)
        .ok_or(SolMathError::Overflow)
        .map(|value| value / Q_ONE as u128)
}

/// Price an American call or put from six scalar inputs, entirely on-chain.
#[allow(clippy::too_many_arguments)]
pub fn american_kbi_price(
    spot: u128,
    strike: u128,
    rate: u128,
    dividend_yield: u128,
    sigma: u128,
    maturity: u128,
    kind: AmericanKbiKind,
) -> Result<u128, SolMathError> {
    match kind {
        AmericanKbiKind::Put => price_put_leg(spot, strike, rate, dividend_yield, sigma, maturity),
        AmericanKbiKind::Call => price_put_leg(strike, spot, dividend_yield, rate, sigma, maturity),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_geometry_is_well_formed() {
        assert_eq!(KBI_DATA_BITS, Q_BITS);
        assert_eq!(KBI_NODES, 18);
        assert_eq!(KBI_PRICE_ORDER, 9);
        assert_eq!(KBI_PRICE_POINTS, 9);
        assert_eq!(KBI_BOUNDARY_ORDER, 6);
        assert_eq!(KBI_BOUNDARY_POINTS, 108);
        assert_eq!(KBI_GRADING, 2 * Q_ONE);
        assert_eq!(KBI_PRICE_POWER, 9 * Q_ONE / 4);
        assert_eq!(KBI_TIME_FRACTION[0], 0);
        assert_eq!(KBI_TIME_FRACTION[KBI_NODES], Q_ONE);
    }

    #[test]
    fn scale_bridge_is_round_trip_stable() {
        for value in [-8 * SCALE_I, -SCALE_I, -1, 0, 1, SCALE_I, 8 * SCALE_I] {
            assert!(q_to_scale(scale_to_q(value).unwrap()).abs_diff(value) <= 1);
        }
    }

    #[test]
    fn boundary_normal_kernel_is_shape_preserving() {
        let mut previous = 0i64;
        for index in -512..=512i64 {
            let x = index * Q_ONE / 64;
            let (cdf, pdf) = boundary_cdf_and_pdf(x);
            assert!(
                cdf >= previous,
                "index={index} x={x} previous={previous} cdf={cdf}"
            );
            assert!((0..=Q_ONE).contains(&cdf));
            assert!((0..=PDF_ZERO_Q).contains(&pdf));
            let exact = crate::normal::norm_cdf_poly(q_to_scale(x)).unwrap();
            assert!(q_to_scale(cdf).abs_diff(exact) <= 5_500_000);
            previous = cdf;
        }
    }

    #[test]
    fn kernel_discount_error_is_bounded_on_the_option_domain() {
        for index in 0..=240i64 {
            let x = -index * Q_ONE / 1_000;
            let exact =
                scale_to_q(crate::transcendental::exp_fixed_i(q_to_scale(x)).unwrap()).unwrap();
            assert!(exp_kernel(x).unwrap().abs_diff(exact) <= 300_000);
        }
    }

    #[test]
    fn reference_quote_tracks_qdfp() {
        let call = american_kbi_price(
            100 * SCALE,
            100 * SCALE,
            50_000_000_000,
            30_000_000_000,
            300_000_000_000,
            SCALE,
            AmericanKbiKind::Call,
        )
        .unwrap();
        let put = american_kbi_price(
            100 * SCALE,
            100 * SCALE,
            50_000_000_000,
            30_000_000_000,
            300_000_000_000,
            SCALE,
            AmericanKbiKind::Put,
        )
        .unwrap();
        assert!(call.abs_diff(12_447_377_336_083) <= 20_000_000_000);
        assert!(put.abs_diff(10_790_235_865_957) <= 20_000_000_000);
    }

    #[test]
    fn input_domain_fails_closed() {
        assert_eq!(
            american_kbi_price(
                100 * SCALE,
                100 * SCALE,
                0,
                0,
                MIN_SIGMA - 1,
                SCALE,
                AmericanKbiKind::Put,
            ),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            american_kbi_price(
                100 * SCALE,
                100 * SCALE,
                0,
                MAX_RATE + 1,
                MIN_SIGMA,
                SCALE,
                AmericanKbiKind::Put,
            ),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            american_kbi_price(
                3 * 100 * SCALE,
                100 * SCALE,
                50_000_000_000,
                30_000_000_000,
                300_000_000_000,
                SCALE,
                AmericanKbiKind::Put,
            ),
            Err(SolMathError::DomainError)
        );
    }

    #[test]
    fn boundary_regression_contract_zero() {
        let parameters = parameters(
            92_840_699_462,
            47_682_052_097,
            363_772_111_820,
            1_942_465_753_425,
        )
        .unwrap();
        let boundary = exercise_boundary(parameters).unwrap();
        let terminal = q_to_scale(boundary.values[KBI_NODES]);
        assert!(
            terminal.abs_diff(593_462_260_054) < 3_000_000,
            "terminal={terminal}"
        );
        let price = q_to_scale(
            american_put_normalized(scale_to_q(597_127_273_422).unwrap(), parameters).unwrap(),
        );
        assert!(price.abs_diff(402_893_802_428) < 3_000_000, "price={price}");
    }
}
