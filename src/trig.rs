use crate::arithmetic::fp_mul_i_fast;
use crate::constants::*;
use crate::error::SolMathError;

/// Largest angle covered by the two-word period reduction error bound:
/// 1e25 raw = 1e13 radians at SCALE.
const MAX_TRIG_ANGLE: i128 = 10_000_000_000_000_000_000_000_000;

#[inline]
fn validate_angle(x: i128) -> Result<(), SolMathError> {
    if x.unsigned_abs() > MAX_TRIG_ANGLE as u128 {
        Err(SolMathError::DomainError)
    } else {
        Ok(())
    }
}

/// Core sin on [−π/4, π/4]: sin(x) = x × P(x²). Internal — called by sin_fixed.
pub(crate) fn sin_core(x: i128) -> Result<i128, SolMathError> {
    let t = fp_mul_i_fast(x, x);
    let mut r = SIN_C11;
    // Horner accumulator: output ∈ [-SCALE_I, SCALE_I]; coefficients ∈ [-SCALE_I, SCALE_I]; sum ≤ 2·SCALE_I ≈ 2e12, fits i128
    r = fp_mul_i_fast(r, t) + SIN_C9;
    r = fp_mul_i_fast(r, t) + SIN_C7;
    r = fp_mul_i_fast(r, t) + SIN_C5;
    r = fp_mul_i_fast(r, t) + SIN_C3;
    r = fp_mul_i_fast(r, t) + SIN_C1;
    Ok(fp_mul_i_fast(r, x))
}

/// Core cos on [−π/4, π/4]: cos(x) = Q(x²). Internal — called by cos_fixed.
pub(crate) fn cos_core(x: i128) -> Result<i128, SolMathError> {
    let t = fp_mul_i_fast(x, x);
    let mut r = COS_C10;
    // Horner accumulator: output ∈ [-SCALE_I, SCALE_I]; coefficients ∈ [-SCALE_I, SCALE_I]; sum ≤ 2·SCALE_I ≈ 2e12, fits i128
    r = fp_mul_i_fast(r, t) + COS_C8;
    r = fp_mul_i_fast(r, t) + COS_C6;
    r = fp_mul_i_fast(r, t) + COS_C4;
    r = fp_mul_i_fast(r, t) + COS_C2;
    r = fp_mul_i_fast(r, t) + COS_C0;
    Ok(r)
}

/// Reduce angle to (−π, π] with Cody-Waite compensation. Internal.
pub(crate) fn reduce_mod_2pi(x: i128) -> i128 {
    // Two-stage reduction: Euclidean reduction against the rounded period
    // (handles every i128 input, including i128::MIN), then fold back q times
    // the sub-ULP residual — TWO_PI_SCALE overshoots 2π by ~0.41 ULP, so the
    // plain remainder drifts ~0.41 ULP per elapsed period (61K ULP of phase
    // error at |x| ≈ 1e6 rad without this). Each pass shrinks |q| by ~13
    // orders of magnitude, so at most 3 passes run even from i128::MAX.
    let mut r = x.rem_euclid(TWO_PI_SCALE);
    let mut q = x.div_euclid(TWO_PI_SCALE);
    while q != 0 {
        // |q| ≤ i128::MAX/TWO_PI_SCALE ≈ 2.7e25; |q·TWO_PI_LO| ≤ 1.2e37 ≪ i128::MAX.
        let corr = q * (-TWO_PI_LO); // sub-ULP units
        let corr_ulp = if corr >= 0 {
            (corr + SCALE_I / 2) / SCALE_I
        } else {
            (corr - SCALE_I / 2) / SCALE_I
        };
        if corr_ulp == 0 {
            break;
        }
        // r < TWO_PI_SCALE ≈ 6.3e12, |corr_ulp| ≤ 1.2e25: sum fits i128.
        let t = r + corr_ulp;
        r = t.rem_euclid(TWO_PI_SCALE);
        q = t.div_euclid(TWO_PI_SCALE);
    }
    if r > PI_SCALE {
        r -= TWO_PI_SCALE;
    }
    r
}

/// Sine of angle x in radians at SCALE.
///
/// - **x**: signed fixed-point radians at `SCALE` (1e12), with
///   `|x| <= 1e25` raw (1e13 radians).
/// - **Returns**: `Result<i128, SolMathError>` — value at `SCALE`, in [-SCALE, SCALE].
/// - **Accuracy**: max ~3 ULP over the supported range.
///
/// # Example
/// ```
/// use solmath::{sin_fixed, SCALE_I};
/// // sin(π/2) ≈ 1.0
/// let pi_over_2 = 1_570_796_326_795i128; // π/2 at SCALE
/// let result = sin_fixed(pi_over_2).unwrap();
/// assert!((result - SCALE_I).abs() <= 2);
/// ```
pub fn sin_fixed(x: i128) -> Result<i128, SolMathError> {
    validate_angle(x)?;
    let mut xx = reduce_mod_2pi(x);
    let sign = if xx < 0 {
        xx = -xx;
        -1i128
    } else {
        1i128
    };
    if xx > PI_OVER_2_SCALE {
        // xx ∈ [0, π·SCALE] after abs; PI_SCALE ≈ 3.14e12 ≥ xx; no underflow; result ∈ [0, π·SCALE], fits i128
        xx = PI_SCALE - xx;
    }
    if xx > PI_OVER_4_SCALE {
        // xx ∈ [0, π/2·SCALE]; PI_OVER_2_SCALE ≥ xx; no underflow; result ∈ [0, π/2·SCALE], fits i128
        // cos_core output ∈ [-SCALE_I, SCALE_I]; sign ∈ {-1, +1}; product ≤ SCALE_I, fits i128
        Ok(cos_core(PI_OVER_2_SCALE - xx)? * sign)
    } else {
        // sin_core output ∈ [-SCALE_I, SCALE_I]; sign ∈ {-1, +1}; product ≤ SCALE_I, fits i128
        Ok(sin_core(xx)? * sign)
    }
}

/// Cosine of angle x in radians at SCALE.
///
/// - **x**: signed fixed-point radians at `SCALE` (1e12), with
///   `|x| <= 1e25` raw (1e13 radians).
/// - **Returns**: `Result<i128, SolMathError>` — value at `SCALE`, in [-SCALE, SCALE].
/// - **Accuracy**: max ~3 ULP over the supported range.
pub fn cos_fixed(x: i128) -> Result<i128, SolMathError> {
    validate_angle(x)?;
    let mut xx = reduce_mod_2pi(x);
    xx = if xx < 0 { -xx } else { xx };
    let cos_sign = if xx > PI_OVER_2_SCALE {
        // xx ∈ (π/2, π]·SCALE; PI_SCALE ≈ 3.14e12 ≥ xx; no underflow; result ∈ [0, π/2·SCALE], fits i128
        xx = PI_SCALE - xx;
        -1i128
    } else {
        1i128
    };
    if xx > PI_OVER_4_SCALE {
        // xx ∈ (π/4, π/2]·SCALE; PI_OVER_2_SCALE ≥ xx; no underflow; result ∈ [0, π/4·SCALE], fits i128
        // sin_core output ∈ [-SCALE_I, SCALE_I]; cos_sign ∈ {-1, +1}; product ≤ SCALE_I, fits i128
        Ok(sin_core(PI_OVER_2_SCALE - xx)? * cos_sign)
    } else {
        // cos_core output ∈ [-SCALE_I, SCALE_I]; cos_sign ∈ {-1, +1}; product ≤ SCALE_I, fits i128
        Ok(cos_core(xx)? * cos_sign)
    }
}

/// Fused sine and cosine: returns `(sin(x), cos(x))` sharing one angle reduction.
///
/// - **x**: signed fixed-point radians at `SCALE` (1e12), with
///   `|x| <= 1e25` raw (1e13 radians).
/// - **Returns**: `Result<(i128, i128), SolMathError>` — `(sin, cos)` each at `SCALE`.
/// - **Accuracy**: max ~3 ULP each over the supported range.
pub fn sincos_fixed(x: i128) -> Result<(i128, i128), SolMathError> {
    validate_angle(x)?;
    let mut xx = reduce_mod_2pi(x);
    let sin_sign = if xx < 0 {
        xx = -xx;
        -1i128
    } else {
        1i128
    };
    let cos_sign = if xx > PI_OVER_2_SCALE {
        // xx ∈ (π/2, π]·SCALE; PI_SCALE ≥ xx; no underflow; result ∈ [0, π/2·SCALE], fits i128
        xx = PI_SCALE - xx;
        -1i128
    } else {
        1i128
    };
    if xx > PI_OVER_4_SCALE {
        // xx ∈ (π/4, π/2]·SCALE; PI_OVER_2_SCALE ≥ xx; no underflow; result ∈ [0, π/4·SCALE], fits i128
        let y = PI_OVER_2_SCALE - xx;
        // cos_core/sin_core output ∈ [-SCALE_I, SCALE_I]; sin_sign/cos_sign ∈ {-1, +1}; products ≤ SCALE_I, fit i128
        Ok((cos_core(y)? * sin_sign, sin_core(y)? * cos_sign))
    } else {
        // sin_core/cos_core output ∈ [-SCALE_I, SCALE_I]; sin_sign/cos_sign ∈ {-1, +1}; products ≤ SCALE_I, fit i128
        Ok((sin_core(xx)? * sin_sign, cos_core(xx)? * cos_sign))
    }
}
