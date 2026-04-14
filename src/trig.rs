use crate::constants::*;
use crate::arithmetic::fp_mul_i_fast;
use crate::error::SolMathError;

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
    // Use Euclidean reduction against the full fixed-point period so every i128 input,
    // including i128::MIN, lands in a small bounded range before polynomial evaluation.
    let mut r = x.rem_euclid(TWO_PI_SCALE);
    if r > PI_SCALE {
        r -= TWO_PI_SCALE;
    }
    r
}

/// Sine of angle x in radians at SCALE.
///
/// - **x**: signed fixed-point radians at `SCALE` (1e12). Any value accepted.
/// - **Returns**: `Result<i128, SolMathError>` — value at `SCALE`, in [-SCALE, SCALE].
/// - **Accuracy**: max 2 ULP, ~47% exact.
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
/// - **x**: signed fixed-point radians at `SCALE` (1e12). Any value accepted.
/// - **Returns**: `Result<i128, SolMathError>` — value at `SCALE`, in [-SCALE, SCALE].
/// - **Accuracy**: max 2 ULP, ~47% exact.
pub fn cos_fixed(x: i128) -> Result<i128, SolMathError> {
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
/// - **x**: signed fixed-point radians at `SCALE` (1e12). Any value accepted.
/// - **Returns**: `Result<(i128, i128), SolMathError>` — `(sin, cos)` each at `SCALE`.
/// - **Accuracy**: max 2 ULP each, ~47% exact.
pub fn sincos_fixed(x: i128) -> Result<(i128, i128), SolMathError> {
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
