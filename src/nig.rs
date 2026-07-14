//! Certified-domain European options in the exponential NIG Levy model.
//!
//! The implementation is entirely on-chain. It evaluates the smaller
//! out-of-the-money leg directly and obtains the other leg from put-call
//! parity. The remaining half-line integral is mapped to `[0, 1]` and
//! evaluated by an embedded 15/7 Gauss-Kronrod rule. A scaled `K1` kernel
//! keeps every density evaluation bounded, and a Chernoff bound handles deep
//! tails without entering the quadrature at all.
//!
//! This is intentionally not the former fixed-term COS implementation. There
//! is no uploaded surface, live oracle, lookup table, or trusted builder.

use crate::arithmetic::{fp_div, fp_div_i, fp_mul, fp_mul_i, fp_sqrt};
use crate::constants::{PI_SCALE, SCALE, SCALE_I};
use crate::error::SolMathError;
use crate::transcendental::{exp_fixed_i, expm1_fixed, ln_fixed_i};

/// Smallest supported NIG tail parameter, `alpha = 2`.
pub const NIG_MIN_ALPHA: u128 = 2 * SCALE;
/// Largest supported NIG tail parameter, `alpha = 100`.
pub const NIG_MAX_ALPHA: u128 = 100 * SCALE;
/// Largest supported elapsed NIG scale, `delta_per_year * time = 15`.
pub const NIG_MAX_DELTA_TIME: u128 = 15 * SCALE;
/// Smallest supported elapsed NIG scale, `1e-3`.
pub const NIG_MIN_DELTA_TIME: u128 = 1_000_000_000;
/// Number of nodes in the production Gauss-Kronrod rule.
pub const NIG_QUADRATURE_NODES: usize = 15;

const NIG_MAX_PRICE: u128 = 100_000 * SCALE;
const NIG_MAX_TIME: u128 = 5 * SCALE;
const NIG_MAX_ABS_RATE: u128 = SCALE / 4;
const NIG_MAX_ABS_LOG_FORWARD: u128 = 2 * SCALE;
// 1e-5 of discounted notional: $0.001 on a $100 quote. This covers the
// fixed-point and embedded-rule residual inside the declared production
// domain; the embedded estimate may increase it quote by quote.
const NIG_ERROR_FLOOR_REL: u128 = 10_000_000;
// Compatibility APIs request at most 5e-5 of notional: $0.005 per $100.
const NIG_DEFAULT_ERROR_REL: u128 = 50_000_000;
const NIG_ROUNDING_REL: u128 = 1_000; // 1e-9 of notional
const NIG_QUADRATURE_SAFETY: u128 = 4;

/// NIG Levy-process parameters, all at [`SCALE`].
///
/// `alpha` and `beta` are inverse log-return units. `delta_per_year` scales
/// linearly with time. The executable production domain additionally requires
/// both `|beta| / alpha <= 0.65` and `|beta + 1| / alpha <= 0.65`; the second
/// condition provides stock-numeraire moment headroom.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NigParams {
    pub alpha: u128,
    pub beta: i128,
    pub delta_per_year: u128,
}

/// A call/put pair together with its quote-local absolute-error allowance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CertifiedNigPrice {
    pub call: u128,
    pub put: u128,
    pub max_abs_error: u128,
    /// `0` is exact expiry, `1` is the Chernoff tail path, and `15` is the
    /// embedded 15/7 Gauss-Kronrod path.
    pub tier: u8,
}

// Kronrod 15 / Gauss 7 geometry after the rational half-line map
// y = scale * t/(1-t). Constants are parameter-independent quadrature
// geometry, not sampled option values.
const GK_U: [u128; NIG_QUADRATURE_NODES] = [
    4_290_645_426,
    26_110_451_522,
    72_464_022_021,
    148_414_691_932,
    260_964_690_514,
    422_631_787_036,
    655_923_922_307,
    1_000_000_000_000,
    1_524_567_051_134,
    2_366_125_858_665,
    3_831_936_029_467,
    6_737_877_409_459,
    13_799_951_646_520,
    38_298_839_801_385,
    233_065_168_689_945,
];

const GK_JAC: [u128; NIG_QUADRATURE_NODES] = [
    1_008_599_700_490,
    1_052_902_658_724,
    1_150_179_078_529,
    1_318_856_304_645,
    1_590_031_950_724,
    2_023_881_201_485,
    2_742_084_036_468,
    4_000_000_000_000,
    6_373_438_795_673,
    11_330_803_296_376,
    23_347_605_792_862,
    59_874_746_803_810,
    219_038_568_739_327,
    1_544_398_809_734_951,
    54_786_503_193_852_536,
];

const GK_WEIGHT: [u128; NIG_QUADRATURE_NODES] = [
    11_467_661_005,
    31_546_046_315,
    52_395_005_161,
    70_326_629_858,
    84_502_363_320,
    95_175_289_032,
    102_216_470_038,
    104_741_070_542,
    102_216_470_038,
    95_175_289_032,
    84_502_363_320,
    70_326_629_858,
    52_395_005_161,
    31_546_046_315,
    11_467_661_005,
];

const G7_WEIGHT: [u128; NIG_QUADRATURE_NODES] = [
    0,
    64_742_483_084,
    0,
    139_852_695_745,
    0,
    190_915_025_253,
    0,
    208_979_591_837,
    0,
    190_915_025_253,
    0,
    139_852_695_745,
    0,
    64_742_483_084,
    0,
];

// Piecewise Chebyshev projections of H(x) = x exp(x) K1(x) on dyadic
// subintervals of [0, 1]. Coefficients are in ascending powers of the local
// coordinate. Dyadic coordinates avoid a fixed-point division at every node.
const K1_SMALL_H: [[i128; 7]; 9] = [
    [
        1_001_941_935_150,
        1_932_638_831,
        -8_385_258,
        563_767,
        -89_961,
        145_057,
        -108_873,
    ],
    [
        1_005_777_191_439,
        1_903_908_444,
        -6_364_790,
        198_229,
        -17_346,
        2_473,
        -424,
    ],
    [
        1_011_435_965_003,
        3_739_596_495,
        -20_613_099,
        757_583,
        -68_299,
        9_810,
        -1_685,
    ],
    [
        1_022_485_681_062,
        7_262_743_550,
        -64_164_215,
        2_816_927,
        -265_263,
        38_607,
        -6_667,
    ],
    [
        1_043_756_979_814,
        13_868_979_760,
        -189_918_073,
        10_056_339,
        -1_005_742,
        149_694,
        -26_096,
    ],
    [
        1_083_867_776_206,
        25_856_671_251,
        -527_679_967,
        33_879_718,
        -3_664_028,
        564_981,
        -100_141,
    ],
    [
        1_157_392_995_973,
        46_688_295_973,
        -1_356_781_762,
        105_515_666,
        -12_545_612,
        2_036_542,
        -370_796,
    ],
    [
        1_287_386_842_132,
        81_038_835_579,
        -3_184_255_272,
        297_187_115,
        -39_292_940,
        6_826_578,
        -1_292_872,
    ],
    [
        1_507_696_399_816,
        134_561_132_879,
        -6_753_634_018,
        742_551_509,
        -109_483_434,
        20_617_628,
        -4_110_566,
    ],
];

// R(v) = sqrt(x) exp(x) K1(x), v = 1/x, x >= 1.
const K1_LARGE_R: [i128; 9] = [
    1_253_314_240_630,
    469_974_369_594,
    -146_312_355_748,
    121_580_534_756,
    -133_560_147_736,
    135_711_020_822,
    -100_936_922_398,
    45_468_297_214,
    -9_085_608_557,
];

#[inline]
fn horner_ascending(x: i128, coefficients: &[i128]) -> Result<i128, SolMathError> {
    let mut coefficients = coefficients.iter().rev();
    let mut value = *coefficients.next().ok_or(SolMathError::DomainError)?;
    for coefficient in coefficients {
        value = fp_mul_i(value, x)?
            .checked_add(*coefficient)
            .ok_or(SolMathError::Overflow)?;
    }
    Ok(value)
}

/// Scaled modified Bessel function `exp(x) * K1(x)` at SCALE.
fn bessel_k1_scaled(x: u128) -> Result<u128, SolMathError> {
    if x == 0 || x > i128::MAX as u128 {
        return Err(SolMathError::DomainError);
    }

    if x <= SCALE {
        let (segment, multiplier, offset) = if x <= SCALE / 256 {
            (0usize, 512u128, SCALE_I)
        } else if x <= SCALE / 128 {
            (1, 512, 3 * SCALE_I)
        } else if x <= SCALE / 64 {
            (2, 256, 3 * SCALE_I)
        } else if x <= SCALE / 32 {
            (3, 128, 3 * SCALE_I)
        } else if x <= SCALE / 16 {
            (4, 64, 3 * SCALE_I)
        } else if x <= SCALE / 8 {
            (5, 32, 3 * SCALE_I)
        } else if x <= SCALE / 4 {
            (6, 16, 3 * SCALE_I)
        } else if x <= SCALE / 2 {
            (7, 8, 3 * SCALE_I)
        } else {
            (8, 4, 3 * SCALE_I)
        };
        let local = x
            .checked_mul(multiplier)
            .and_then(|value| (value as i128).checked_sub(offset))
            .ok_or(SolMathError::Overflow)?;
        let h = horner_ascending(local, &K1_SMALL_H[segment])?;
        if h <= 0 {
            return Err(SolMathError::NoConvergence);
        }
        fp_div(h as u128, x)
    } else {
        let reciprocal = fp_div(SCALE, x)? as i128;
        let ratio = horner_ascending(reciprocal, &K1_LARGE_R)?;
        if ratio <= 0 {
            return Err(SolMathError::NoConvergence);
        }
        fp_div(ratio as u128, fp_sqrt(x)?)
    }
}

#[inline]
fn hypot_fixed(delta: u128, x: i128) -> Result<u128, SolMathError> {
    let x_abs = x.unsigned_abs();
    let squared = fp_mul(delta, delta)?
        .checked_add(fp_mul(x_abs, x_abs)?)
        .ok_or(SolMathError::Overflow)?;
    fp_sqrt(squared)
}

#[inline]
fn rounding_allowance(notional: u128) -> Result<u128, SolMathError> {
    fp_mul(notional, NIG_ROUNDING_REL)?
        .checked_add(4_096)
        .ok_or(SolMathError::Overflow)
}

fn chernoff_lower_tail(
    x: i128,
    beta: i128,
    gamma: i128,
    alpha: i128,
    delta_t: i128,
) -> Result<Option<u128>, SolMathError> {
    let mean = fp_div_i(fp_mul_i(delta_t, beta)?, gamma)?;
    if x >= mean {
        return Ok(None);
    }
    // The optimized Chernoff exponent is
    // d*gamma + beta*x - alpha*hypot(d,x) <= 0.
    let omega = hypot_fixed(delta_t as u128, x)?;
    let alpha_omega = fp_mul_i(alpha, omega as i128)?;
    let exponent = fp_mul_i(delta_t, gamma)?
        .checked_add(fp_mul_i(beta, x)?)
        .and_then(|v| v.checked_sub(alpha_omega))
        .ok_or(SolMathError::Overflow)?;
    // Fixed-point square-root rounding can lift the exact non-positive value
    // by a few raw units at the mean. Clamping it to zero remains an upper
    // bound.
    Ok(Some(exp_fixed_i(exponent.min(0))? as u128))
}

struct IntegralInputs {
    alpha: u128,
    beta: i128,
    delta_t: u128,
    gamma: i128,
    threshold: i128,
    scale: u128,
    call: bool,
}

fn payoff_density_node(inputs: &IntegralInputs, node: usize) -> Result<u128, SolMathError> {
    let y = fp_mul(inputs.scale, GK_U[node])?;
    if y > i128::MAX as u128 {
        return Err(SolMathError::Overflow);
    }
    let y_i = y as i128;
    let x = if inputs.call {
        inputs.threshold.checked_add(y_i)
    } else {
        inputs.threshold.checked_sub(y_i)
    }
    .ok_or(SolMathError::Overflow)?;
    let omega = hypot_fixed(inputs.delta_t, x)?;
    let alpha_omega = fp_mul_i(inputs.alpha as i128, omega as i128)?;
    let exponent = fp_mul_i(inputs.delta_t as i128, inputs.gamma)?
        .checked_add(fp_mul_i(inputs.beta, x)?)
        .and_then(|v| v.checked_sub(alpha_omega))
        .ok_or(SolMathError::Overflow)?;

    // Evaluate the payoff times the density as a difference of bounded
    // exponentials. For small y, expm1 preserves the zero at the exercise
    // boundary instead of subtracting two nearly equal values.
    let exp_e = exp_fixed_i(exponent)?;
    if exp_e < 0 {
        return Err(SolMathError::NoConvergence);
    }
    let payoff_weight = if inputs.call {
        if y_i < 20 * SCALE_I {
            let growth = expm1_fixed(y_i)?;
            if growth < 0 {
                return Err(SolMathError::NoConvergence);
            }
            fp_mul(exp_e as u128, growth as u128)?
        } else {
            let grown = exp_fixed_i(exponent.checked_add(y_i).ok_or(SolMathError::Overflow)?)?;
            grown
                .checked_sub(exp_e)
                .ok_or(SolMathError::NoConvergence)? as u128
        }
    } else {
        let decay = expm1_fixed(y_i.checked_neg().ok_or(SolMathError::Overflow)?)?
            .checked_neg()
            .ok_or(SolMathError::Overflow)?;
        fp_mul(exp_e as u128, decay as u128)?
    };

    if payoff_weight == 0 {
        return Ok(0);
    }
    let z = fp_mul(inputs.alpha, omega)?;
    let scaled_k1 = bessel_k1_scaled(z)?;
    let alpha_delta = fp_mul(inputs.alpha, inputs.delta_t)?;
    let pi_omega = fp_mul(PI_SCALE as u128, omega)?;
    let density_without_exp = fp_mul(fp_div(alpha_delta, pi_omega)?, scaled_k1)?;
    let jacobian = fp_mul(inputs.scale, GK_JAC[node])?;
    fp_mul(fp_mul(density_without_exp, payoff_weight)?, jacobian)
}

fn integrate_otm(inputs: &IntegralInputs) -> Result<(u128, u128), SolMathError> {
    let mut kronrod = 0u128;
    let mut gauss = 0u128;

    let mut node = 0usize;
    while node < NIG_QUADRATURE_NODES {
        let value = payoff_density_node(inputs, node)?;
        kronrod = kronrod
            .checked_add(fp_mul(value, GK_WEIGHT[node])?)
            .ok_or(SolMathError::Overflow)?;
        if G7_WEIGHT[node] != 0 {
            gauss = gauss
                .checked_add(fp_mul(value, G7_WEIGHT[node])?)
                .ok_or(SolMathError::Overflow)?;
        }
        node += 1;
    }
    Ok((kronrod, kronrod.abs_diff(gauss)))
}

fn parity_prices(
    otm: u128,
    call_is_otm: bool,
    discounted_spot: u128,
    discounted_strike: u128,
) -> Result<(u128, u128), SolMathError> {
    if call_is_otm {
        let parity = discounted_strike
            .checked_sub(discounted_spot)
            .ok_or(SolMathError::NoConvergence)?;
        Ok((otm, otm.checked_add(parity).ok_or(SolMathError::Overflow)?))
    } else {
        let parity = discounted_spot
            .checked_sub(discounted_strike)
            .ok_or(SolMathError::NoConvergence)?;
        Ok((otm.checked_add(parity).ok_or(SolMathError::Overflow)?, otm))
    }
}

/// Price a European call and put under an exponential NIG Levy process.
///
/// Rates and dividend yield are continuously compounded signed values. All
/// numeric fields and outputs use [`SCALE`]. The function fails closed when a
/// quote is outside the declared production domain or when its quote-local
/// embedded error allowance exceeds `requested_max_abs_error`.
pub fn nig_price_certified(
    spot: u128,
    strike: u128,
    rate: i128,
    dividend_yield: i128,
    time: u128,
    params: NigParams,
    requested_max_abs_error: u128,
) -> Result<CertifiedNigPrice, SolMathError> {
    if spot == 0 || strike == 0 || spot > NIG_MAX_PRICE || strike > NIG_MAX_PRICE {
        return Err(SolMathError::DomainError);
    }
    if time == 0 {
        return Ok(CertifiedNigPrice {
            call: spot.saturating_sub(strike),
            put: strike.saturating_sub(spot),
            max_abs_error: 0,
            tier: 0,
        });
    }
    if requested_max_abs_error == 0
        || time > NIG_MAX_TIME
        || rate.unsigned_abs() > NIG_MAX_ABS_RATE
        || dividend_yield.unsigned_abs() > NIG_MAX_ABS_RATE
        || params.alpha < NIG_MIN_ALPHA
        || params.alpha > NIG_MAX_ALPHA
        || params.delta_per_year == 0
        || params.delta_per_year > 15 * SCALE
    {
        return Err(SolMathError::DomainError);
    }
    let beta_plus_one = params
        .beta
        .checked_add(SCALE_I)
        .ok_or(SolMathError::Overflow)?;
    let skew_limit = params.alpha.checked_mul(13).ok_or(SolMathError::Overflow)?;
    if params
        .beta
        .unsigned_abs()
        .checked_mul(20)
        .ok_or(SolMathError::Overflow)?
        > skew_limit
        || beta_plus_one
            .unsigned_abs()
            .checked_mul(20)
            .ok_or(SolMathError::Overflow)?
            > skew_limit
    {
        return Err(SolMathError::DomainError);
    }

    let delta_t = fp_mul(params.delta_per_year, time)?;
    if !(NIG_MIN_DELTA_TIME..=NIG_MAX_DELTA_TIME).contains(&delta_t) {
        return Err(SolMathError::DomainError);
    }
    let alpha_i = params.alpha as i128;
    let alpha_sq = fp_mul(params.alpha, params.alpha)?;
    let beta_sq = fp_mul_i(params.beta, params.beta)?;
    let beta_one_sq = fp_mul_i(beta_plus_one, beta_plus_one)?;
    if beta_sq < 0
        || beta_one_sq < 0
        || alpha_sq <= beta_sq as u128
        || alpha_sq <= beta_one_sq as u128
    {
        return Err(SolMathError::DomainError);
    }
    let gamma = fp_sqrt(alpha_sq - beta_sq as u128)? as i128;
    let gamma_one = fp_sqrt(alpha_sq - beta_one_sq as u128)? as i128;

    let log_moneyness = ln_fixed_i(spot)?
        .checked_sub(ln_fixed_i(strike)?)
        .ok_or(SolMathError::Overflow)?;
    let carry = rate
        .checked_sub(dividend_yield)
        .ok_or(SolMathError::Overflow)?;
    let carry_time = fp_mul_i(carry, time as i128)?;
    let log_forward = log_moneyness
        .checked_add(carry_time)
        .ok_or(SolMathError::Overflow)?;
    if log_forward.unsigned_abs() > NIG_MAX_ABS_LOG_FORWARD {
        return Err(SolMathError::DomainError);
    }
    let correction = fp_mul_i(
        delta_t as i128,
        gamma_one.checked_sub(gamma).ok_or(SolMathError::Overflow)?,
    )?;
    let kappa = log_forward
        .checked_add(correction)
        .ok_or(SolMathError::Overflow)?;
    let threshold = kappa.checked_neg().ok_or(SolMathError::Overflow)?;

    let rate_time = fp_mul_i(rate, time as i128)?;
    let dividend_time = fp_mul_i(dividend_yield, time as i128)?;
    let strike_discount = exp_fixed_i(rate_time.checked_neg().ok_or(SolMathError::Overflow)?)?;
    let spot_discount = exp_fixed_i(dividend_time.checked_neg().ok_or(SolMathError::Overflow)?)?;
    if strike_discount < 0 || spot_discount < 0 {
        return Err(SolMathError::NoConvergence);
    }
    let discounted_strike = fp_mul(strike, strike_discount as u128)?;
    let discounted_spot = fp_mul(spot, spot_discount as u128)?;
    let call_is_otm = discounted_spot <= discounted_strike;
    let notional = discounted_spot.max(discounted_strike);
    let rounding = rounding_allowance(notional)?;
    // A loose caller tolerance must not silently downgrade an otherwise
    // computable quote to a zero-OTM tail approximation. The shortcut itself
    // is capped at the crate's standard $0.005-per-$100 quality target.
    let tail_accuracy_cap = fp_mul(notional, NIG_DEFAULT_ERROR_REL)?;

    // A rigorous zero-price shortcut: the OTM payoff is bounded by its asset
    // (call) or cash (put) digital, then by the optimized NIG Chernoff tail.
    let tail_probability = if call_is_otm {
        chernoff_lower_tail(
            kappa,
            beta_plus_one.checked_neg().ok_or(SolMathError::Overflow)?,
            gamma_one,
            alpha_i,
            delta_t as i128,
        )?
    } else {
        chernoff_lower_tail(threshold, params.beta, gamma, alpha_i, delta_t as i128)?
    };
    if let Some(probability) = tail_probability {
        let digital_notional = if call_is_otm {
            discounted_spot
        } else {
            discounted_strike
        };
        let tail_bound = fp_mul(digital_notional, probability)?;
        let certificate = tail_bound
            .checked_add(rounding)
            .ok_or(SolMathError::Overflow)?;
        if certificate <= requested_max_abs_error && certificate <= tail_accuracy_cap {
            let (call, put) = parity_prices(0, call_is_otm, discounted_spot, discounted_strike)?;
            return Ok(CertifiedNigPrice {
                call,
                put,
                max_abs_error: certificate,
                tier: 1,
            });
        }
    }

    let gamma_sq = fp_mul(gamma as u128, gamma as u128)?;
    let gamma_cube = fp_mul(gamma_sq, gamma as u128)?;
    let variance = fp_div(fp_mul(delta_t, alpha_sq)?, gamma_cube)?;
    let base_scale = fp_sqrt(variance)?;
    let tilted_scale = if call_is_otm {
        let gamma_one_sq = fp_mul(gamma_one as u128, gamma_one as u128)?;
        let gamma_one_cube = fp_mul(gamma_one_sq, gamma_one as u128)?;
        fp_sqrt(fp_div(fp_mul(delta_t, alpha_sq)?, gamma_one_cube)?)?
    } else {
        base_scale
    };
    // A factor of four places the final Kronrod node far enough into the NIG
    // tail for the 15/7 embedded pair while retaining dense central coverage.
    let scale = base_scale
        .max(tilted_scale)
        .checked_mul(4)
        .ok_or(SolMathError::Overflow)?;
    if scale == 0 {
        return Err(SolMathError::NoConvergence);
    }
    let integral_inputs = IntegralInputs {
        alpha: params.alpha,
        beta: params.beta,
        delta_t,
        gamma,
        threshold,
        scale,
        call: call_is_otm,
    };
    let (integral, integral_error) = integrate_otm(&integral_inputs)?;
    let otm = fp_mul(discounted_strike, integral)?;
    let (call, put) = parity_prices(otm, call_is_otm, discounted_spot, discounted_strike)?;

    let embedded_price_error = fp_mul(discounted_strike, integral_error)?
        .checked_mul(NIG_QUADRATURE_SAFETY)
        .ok_or(SolMathError::Overflow)?;
    let floor = fp_mul(notional, NIG_ERROR_FLOOR_REL)?;
    let certificate = floor.max(
        embedded_price_error
            .checked_add(rounding)
            .ok_or(SolMathError::Overflow)?,
    );
    if certificate > requested_max_abs_error {
        return Err(SolMathError::NoConvergence);
    }
    if call
        > discounted_spot
            .checked_add(certificate)
            .ok_or(SolMathError::Overflow)?
        || put
            > discounted_strike
                .checked_add(certificate)
                .ok_or(SolMathError::Overflow)?
    {
        return Err(SolMathError::NoConvergence);
    }

    Ok(CertifiedNigPrice {
        call,
        put,
        max_abs_error: certificate,
        tier: NIG_QUADRATURE_NODES as u8,
    })
}

/// Compatibility NIG call API (`q = 0`, default `$0.005 / $100` request).
///
/// New integrations should use [`nig_price_certified`] so signed rates,
/// dividends, and the accepted error allowance are explicit.
pub fn nig_call_price(
    s: u128,
    k: u128,
    r: u128,
    t: u128,
    alpha: u128,
    beta: i128,
    delta: u128,
) -> Result<u128, SolMathError> {
    if s > i128::MAX as u128
        || k > i128::MAX as u128
        || r > i128::MAX as u128
        || t > i128::MAX as u128
        || alpha > i128::MAX as u128
        || delta > i128::MAX as u128
    {
        return Err(SolMathError::Overflow);
    }
    if t == 0 {
        if s == 0 || k == 0 {
            return Err(SolMathError::DomainError);
        }
        return Ok(s.saturating_sub(k));
    }
    let requested = fp_mul(s.max(k), NIG_DEFAULT_ERROR_REL)?.max(1);
    Ok(nig_price_certified(
        s,
        k,
        r as i128,
        0,
        t,
        NigParams {
            alpha,
            beta,
            delta_per_year: delta,
        },
        requested,
    )?
    .call)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expiry_is_intrinsic() {
        let params = NigParams {
            alpha: 10 * SCALE,
            beta: -2 * SCALE_I,
            delta_per_year: SCALE,
        };
        let quote = nig_price_certified(120 * SCALE, 100 * SCALE, 0, 0, 0, params, 0).unwrap();
        assert_eq!(quote.call, 20 * SCALE);
        assert_eq!(quote.put, 0);
        assert_eq!(quote.max_abs_error, 0);
        assert_eq!(quote.tier, 0);
    }

    #[test]
    fn rejects_insufficient_stock_measure_headroom() {
        let params = NigParams {
            alpha: 10 * SCALE,
            beta: 8 * SCALE_I,
            delta_per_year: SCALE,
        };
        assert_eq!(
            nig_price_certified(
                100 * SCALE,
                100 * SCALE,
                0,
                0,
                SCALE,
                params,
                20_000_000_000,
            ),
            Err(SolMathError::DomainError)
        );
    }

    #[test]
    fn scaled_bessel_k1_reference_points() {
        // exp(x) K1(x), references rounded from 100-digit mpmath.
        let cases = [
            (SCALE / 10, 10_890_182_683_050u128),
            (SCALE, 1_636_153_486_263u128),
            (10 * SCALE, 410_766_570_595u128),
        ];
        for (x, expected) in cases {
            let actual = bessel_k1_scaled(x).unwrap();
            assert!(
                actual.abs_diff(expected) <= expected / 10_000_000 + 64,
                "x={x}, actual={actual}"
            );
        }
    }

    #[test]
    fn representative_prices_match_independent_density_integration() {
        let cases = [
            // s, k, r, q, t, alpha, beta, delta, call, put
            (
                100 * SCALE,
                100 * SCALE,
                50_000_000_000,
                20_000_000_000,
                SCALE,
                10 * SCALE,
                -2 * SCALE_I,
                200_000_000_000,
                6_892_015_108_422,
                3_995_090_227_818,
            ),
            (
                80 * SCALE,
                100 * SCALE,
                30_000_000_000,
                10_000_000_000,
                SCALE / 2,
                8 * SCALE,
                -3 * SCALE_I,
                400_000_000_000,
                533_653_027_759,
                19_443_848_652_651,
            ),
            (
                130 * SCALE,
                100 * SCALE,
                -10_000_000_000,
                40_000_000_000,
                2 * SCALE,
                12 * SCALE,
                2 * SCALE_I,
                300_000_000_000,
                21_337_043_419_354,
                3_352_052_391_767,
            ),
        ];
        for (s, k, r, q, t, alpha, beta, delta, expected_call, expected_put) in cases {
            let quote = nig_price_certified(
                s,
                k,
                r,
                q,
                t,
                NigParams {
                    alpha,
                    beta,
                    delta_per_year: delta,
                },
                20_000_000_000,
            )
            .unwrap();
            assert!(
                quote.call.abs_diff(expected_call) <= quote.max_abs_error,
                "quote={quote:?}, expected_call={expected_call}"
            );
            assert!(
                quote.put.abs_diff(expected_put) <= quote.max_abs_error,
                "quote={quote:?}, expected_put={expected_put}"
            );
        }
    }

    #[test]
    fn declared_numerical_boundaries_fail_closed() {
        let valid = NigParams {
            alpha: 10 * SCALE,
            beta: -2 * SCALE_I,
            delta_per_year: SCALE / 5,
        };
        assert_eq!(
            nig_price_certified(
                100 * SCALE,
                100 * SCALE,
                0,
                0,
                SCALE,
                NigParams {
                    delta_per_year: NIG_MIN_DELTA_TIME - 1,
                    ..valid
                },
                SCALE / 100,
            ),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            nig_price_certified(
                100 * SCALE,
                100 * SCALE,
                0,
                0,
                SCALE,
                NigParams {
                    beta: 5 * SCALE_I + SCALE_I / 2 + 1,
                    ..valid
                },
                SCALE / 100,
            ),
            Err(SolMathError::DomainError)
        );
    }

    #[test]
    fn strike_shape_and_homogeneity_are_preserved() {
        let params = NigParams {
            alpha: 10 * SCALE,
            beta: -2 * SCALE_I,
            delta_per_year: SCALE / 5,
        };
        let mut calls = [0u128; 3];
        for (index, strike) in [80 * SCALE, 100 * SCALE, 120 * SCALE]
            .into_iter()
            .enumerate()
        {
            calls[index] = nig_price_certified(
                100 * SCALE,
                strike,
                SCALE_I / 20,
                SCALE_I / 50,
                SCALE,
                params,
                SCALE / 10,
            )
            .unwrap()
            .call;
        }
        assert!(calls[0] >= calls[1] && calls[1] >= calls[2]);
        assert!(calls[0] + calls[2] >= 2 * calls[1]);

        let base = nig_price_certified(
            100 * SCALE,
            100 * SCALE,
            SCALE_I / 20,
            SCALE_I / 50,
            SCALE,
            params,
            SCALE / 10,
        )
        .unwrap();
        let doubled = nig_price_certified(
            200 * SCALE,
            200 * SCALE,
            SCALE_I / 20,
            SCALE_I / 50,
            SCALE,
            params,
            SCALE / 5,
        )
        .unwrap();
        assert!(doubled.call.abs_diff(2 * base.call) <= 2);
        assert!(doubled.put.abs_diff(2 * base.put) <= 2);
    }

    #[test]
    fn symmetric_large_alpha_case_approaches_black_scholes() {
        // alpha=100, beta=-1/2 removes the leading skew under the martingale
        // measure. delta=alpha*sigma^2 gives the sigma=20% Brownian limit.
        let quote = nig_price_certified(
            100 * SCALE,
            100 * SCALE,
            SCALE_I / 20,
            0,
            SCALE,
            NigParams {
                alpha: 100 * SCALE,
                beta: -SCALE_I / 2,
                delta_per_year: 4 * SCALE,
            },
            SCALE,
        )
        .unwrap();
        let black_scholes = 10_450_583_572_186u128;
        assert!(quote.call.abs_diff(black_scholes) <= 5_000_000_000);
    }
}
