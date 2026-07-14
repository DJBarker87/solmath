use crate::arithmetic::{fp_div_i, fp_mul, fp_mul_i, fp_sqrt};
use crate::constants::*;
use crate::error::SolMathError;
use crate::exp_coeffs::{
    EXP2_PHASE_Q62, EXP_LN2_RESIDUAL_Q96, EXP_PHASES, EXP_PHASE_BITS, EXP_POLY_GUARD,
    EXP_RAW_TO_Q63_FRAC_Q28, EXP_RAW_TO_Q63_HI, EXP_REMEZ_Q22, EXP_STEP_Q63,
};
use crate::expm1_lut::{
    EXPM1_INV_LN2_Q56, EXPM1_LUT_SEGMENTS, EXPM1_LUT_STEP, EXPM1_LUT_STEP_SHIFT,
    EXPM1_MID_EXP_RAW_Q22, EXPM1_RAW_TO_Q43_G31, EXPM1_R_MIN,
};
use crate::hp::pow_fixed_hp;
use crate::ln2_lut::{K_LN2_MAX, K_LN2_MIN, K_LN2_RAW};
use crate::ln_lut::{
    LN_LUT_HALF_STEP, LN_LUT_MID_LOG, LN_LUT_SEGMENTS, LN_LUT_STEP, LN_Q42_RECIP_G32,
};
use crate::overflow::checked_mul_div_u;

#[inline(always)]
fn round_shift_signed(value: i128, shift: u32) -> i128 {
    let half = 1i128 << (shift - 1);
    if value >= 0 {
        (value + half) >> shift
    } else {
        -((-value + half) >> shift)
    }
}

#[inline(always)]
fn round_shift_i64(value: i64, shift: u32) -> i64 {
    let half = 1i64 << (shift - 1);
    if value >= 0 {
        (value + half) >> shift
    } else {
        -((-value + half) >> shift)
    }
}

#[inline(always)]
fn mul_q42(a: i64, b: i64) -> i64 {
    round_shift_i64(a * b, 42)
}

#[inline(always)]
fn mul_q43(a: i64, b: i64) -> i64 {
    round_shift_i64(a * b, 43)
}

#[inline(always)]
fn mul_q63_i64(a: i64, b: i64) -> i64 {
    round_shift_signed(a as i128 * b as i128, 63) as i64
}

/// Wide-division-free local log kernel for a mantissa in `[SCALE, 2*SCALE)`.
#[inline]
fn ln_mantissa_lut(m: u128, k: i32) -> i128 {
    debug_assert!(m >= SCALE && m < 2 * SCALE);
    debug_assert!((K_LN2_MIN..=K_LN2_MAX).contains(&k));

    // Preserve exact rounded logarithms for powers of two without entering
    // the local polynomial around the first midpoint.
    if m == SCALE {
        return K_LN2_RAW[(k - K_LN2_MIN) as usize] as i128;
    }

    let offset = m - SCALE;
    // `offset < 1e12` and the step is below 1e9, so this exact index needs
    // only a 64-bit division. Keeping it as u128 links the much costlier SBF
    // wide-division path even though neither operand requires it.
    let j = ((offset as u64) / (LN_LUT_STEP as u64)) as usize;
    debug_assert!(j < LN_LUT_SEGMENTS);
    let midpoint = SCALE + j as u128 * LN_LUT_STEP + LN_LUT_HALF_STEP;
    let d = m as i64 - midpoint as i64;

    // q=(m-midpoint)/midpoint at Q42. The reciprocal carries 32 guard bits,
    // and this conversion plus every polynomial product fits i64.
    let q = round_shift_i64(d * LN_Q42_RECIP_G32[j], 32);
    let q2 = mul_q42(q, q);
    let q3 = mul_q42(q2, q);
    let local_q42 = q - q2 / 2 + q3 / 3;
    // One final wide conversion preserves a single rounding point. This is
    // the only i128 multiplication in the local kernel.
    let local_raw = round_shift_signed(local_q42 as i128 * SCALE_I, 42);

    let k_log = K_LN2_RAW[(k - K_LN2_MIN) as usize];
    LN_LUT_MID_LOG[j] as i128 + local_raw + k_log as i128
}

#[cold]
#[inline(never)]
fn normalize_ln_fallback(value: u128) -> (u128, i32) {
    // SCALE has bit length 40. A bit-length estimate gets the mantissa into
    // [2^39, 2^40) in one shift; because SCALE lies inside that interval, at
    // most one corrected shift is needed. Re-shifting the original value is
    // important: adjusting an already truncated mantissa would lose one bit.
    let bit_length = 128 - value.leading_zeros() as i32;
    let mut k = bit_length - 40;
    let shift = |exponent: i32| {
        if exponent >= 0 {
            value >> exponent as u32
        } else {
            value << (-exponent) as u32
        }
    };
    let mut m = shift(k);
    if m < SCALE {
        k -= 1;
        m = shift(k);
    } else if m >= 2 * SCALE {
        k += 1;
        m = shift(k);
    }
    (m, k)
}

#[inline]
fn normalize_ln(value: u128) -> (u128, i32) {
    if value >= SCALE && value < 2 * SCALE {
        (value, 0)
    } else if value >= SCALE / 2 && value < SCALE {
        (value * 2, -1)
    } else {
        normalize_ln_fallback(value)
    }
}

/// Natural logarithm: ln(x / SCALE) * SCALE.
///
/// Wide-division-free 1,024-segment Q42 midpoint kernel with a cubic residual.
///
/// - **x**: unsigned fixed-point at `SCALE` (1e12). Must be > 0.
/// - **Returns**: `i128` at `SCALE`. Negative for x < SCALE, zero for x == SCALE.
/// - **Errors**: `DomainError` if `x == 0`.
/// - **Accuracy**: max 2 ULP over the retained production and adversarial corpora.
///
/// # Example
/// ```
/// use solmath::{ln_fixed_i, SCALE};
/// // ln(2.0) ≈ 0.693147...
/// let result = ln_fixed_i(2 * SCALE)?;
/// assert!((result - 693_147_180_560i128).abs() <= 2);
/// # Ok::<(), solmath::SolMathError>(())
/// ```
pub fn ln_fixed_i(x: u128) -> Result<i128, SolMathError> {
    if x == 0 {
        return Err(SolMathError::DomainError);
    }

    // For |x-1| < 1e-6, the quadratic remainder is below half a raw
    // output unit. This preserves raw increments that would be lost while
    // converting the first midpoint residual to Q42.
    if x.abs_diff(SCALE) < 1_000_000 {
        return Ok(x as i128 - SCALE_I);
    }

    let (m, k) = normalize_ln(x);
    Ok(ln_mantissa_lut(m, k))
}

/// Natural logarithm of one plus a signed fixed-point value.
///
/// Computes `ln(1 + x / SCALE) * SCALE`. This is the fixed-point counterpart
/// of `log1p`/`ln_1p` and preserves small increments because `1 + x` is formed
/// exactly in the integer representation before the compensated near-one
/// logarithm kernel runs.
///
/// - **x**: signed fixed-point at [`SCALE`] (1e12).
/// - **Returns**: `ln(1 + x)` at `SCALE`.
/// - **Errors**: [`SolMathError::DomainError`] when `x <= -SCALE`.
/// - **Accuracy**: max 2 ULP, P99 1 ULP, median 0 over 110,000 retained vectors.
///
/// # Example
/// ```
/// use solmath::{ln_1p_fixed, SCALE_I};
///
/// // ln(1.05) ≈ 0.048790164169
/// let result = ln_1p_fixed(SCALE_I / 20)?;
/// assert!((result - 48_790_164_169).abs() <= 2);
/// # Ok::<(), solmath::SolMathError>(())
/// ```
pub fn ln_1p_fixed(x: i128) -> Result<i128, SolMathError> {
    if x <= -SCALE_I {
        return Err(SolMathError::DomainError);
    }
    if x == SCALE_I {
        return Ok(LN2_I);
    }

    // For |x| < 1e-6, |ln(1+x) - x| = x²/2 + O(x³), which is strictly
    // below half a raw SCALE unit. Returning x is therefore correctly rounded
    // in this interval and, unlike forming the atanh quotient at SCALE,
    // preserves one-raw-unit increments.
    if x.unsigned_abs() < 1_000_000 {
        return Ok(x);
    }

    let one_plus_x = if x < 0 {
        (SCALE_I + x) as u128
    } else {
        SCALE.checked_add(x as u128).ok_or(SolMathError::Overflow)?
    };

    // Normalize once, then use a dedicated 1,024-segment Q42 kernel. Common
    // rate inputs stay in the first two branches and execute no loop.
    let (m, k) = normalize_ln(one_plus_x);

    Ok(ln_mantissa_lut(m, k))
}

/// Exponential: e^(x / SCALE) * SCALE.
///
/// Division-free degree-5 near-minimax approximation after ln(2)/32 reduction.
///
/// - **x**: signed fixed-point at `SCALE` (1e12).
/// - **Returns**: non-negative `i128` at `SCALE`; very negative valid inputs
///   can round to zero.
/// - **Errors**: `Overflow` if `x >= 40 * SCALE`. Returns `Ok(0)` for `x <= -40 * SCALE`.
/// - **Accuracy**: the reduced kernel has a conservative relative bound below
///   `4.833e-15`. Absolute raw error grows with the reconstructed power of two;
///   see the retained production/adversarial measurements in `VALIDATION.md`.
///
/// # Example
/// ```
/// use solmath::{exp_fixed_i, SCALE, SCALE_I};
/// // e^1 ≈ 2.718281828...
/// let result = exp_fixed_i(SCALE_I)?;
/// assert!((result - 2_718_281_828_459i128).abs() <= 1);
/// # Ok::<(), solmath::SolMathError>(())
/// ```
pub fn exp_fixed_i(x: i128) -> Result<i128, SolMathError> {
    let max_x = 40 * SCALE_I;

    if x <= -max_x {
        return Ok(0);
    }
    if x >= max_x {
        return Err(SolMathError::Overflow);
    }
    // For |x| < 1e-6, the exp Taylor remainder is strictly below half a
    // raw unit. This is correctly rounded and preserves tiny rate inputs.
    if (-1_000_000..1_000_000).contains(&x) {
        return Ok(SCALE_I + x);
    }

    // First reduce to the nearest full ln(2) octave using only i64. A split
    // reciprocal converts the small raw residual to Q63 without a wide
    // range-reduction multiply, then restores LN2_I's sub-raw residual.
    let x64 = x as i64;
    let octave_estimate = round_shift_i64(x64 * EXPM1_INV_LN2_Q56, 56) as i32;
    let raw_residual = x64 - octave_estimate as i64 * LN2_I as i64;
    let scaled_residual = raw_residual * EXP_RAW_TO_Q63_HI
        + round_shift_i64(raw_residual * EXP_RAW_TO_Q63_FRAC_Q28, 28);
    let octave_residual_q63 = round_shift_i64(scaled_residual, 1)
        - round_shift_i64(octave_estimate as i64 * EXP_LN2_RESIDUAL_Q96, 33);

    // Split the octave into 32 cells. The proposal uses raw i64 arithmetic;
    // the Q63 check makes the final cell exact at every reduction seam.
    let mut subcell = round_shift_i64(
        raw_residual * EXPM1_INV_LN2_Q56,
        (56 - EXP_PHASE_BITS) as u32,
    ) as i32;
    let mut r_q63 = octave_residual_q63 - subcell as i64 * EXP_STEP_Q63;
    let half_step_q63 = (EXP_STEP_Q63 + 1) / 2;
    if r_q63 > half_step_q63 {
        subcell += 1;
        r_q63 -= EXP_STEP_Q63;
    } else if r_q63 < -half_step_q63 {
        subcell -= 1;
        r_q63 += EXP_STEP_Q63;
    }
    debug_assert!(r_q63.abs() <= half_step_q63 + 1);

    let poly = mul_q63_i64(EXP_REMEZ_Q22[0], r_q63) + EXP_REMEZ_Q22[1];
    let poly = mul_q63_i64(poly, r_q63) + EXP_REMEZ_Q22[2];
    let poly = mul_q63_i64(poly, r_q63) + EXP_REMEZ_Q22[3];
    let poly = mul_q63_i64(poly, r_q63) + EXP_REMEZ_Q22[4];
    let poly = mul_q63_i64(poly, r_q63) + EXP_REMEZ_Q22[5];

    // cell = 32*octave + phase. These constants reconstruct only the
    // fractional power of two; they are not a sampled answer table.
    let cell = octave_estimate * EXP_PHASES as i32 + subcell;
    let octave = cell >> EXP_PHASE_BITS;
    let phase = (cell & (EXP_PHASES as i32 - 1)) as usize;
    let (guarded, guard) = if phase == 0 {
        (poly as i128, EXP_POLY_GUARD)
    } else {
        (
            poly as i128 * EXP2_PHASE_Q62[phase] as i128,
            EXP_POLY_GUARD + 62,
        )
    };

    // Fold phase and octave reconstruction into one final rounding point.
    let shift = guard - octave;
    if shift >= 128 {
        Ok(0)
    } else if shift > 0 {
        Ok(round_shift_signed(guarded, shift as u32))
    } else if shift == 0 {
        Ok(guarded)
    } else {
        guarded
            .checked_shl((-shift) as u32)
            .ok_or(SolMathError::Overflow)
    }
}

/// Unsigned fixed-point power: base^exponent via exp(exponent * ln(base)).
///
/// - **base**: unsigned fixed-point at `SCALE` (1e12).
/// - **exponent**: unsigned fixed-point at `SCALE` (1e12).
/// - **Returns**: `u128` at `SCALE`.
/// - **Errors**: `DomainError` for `0^0`, `Overflow` if result too large.
///
/// Uses sqrt for x^0.5, direct multiply for x^2, HP path for small integer
/// exponents, and general exp/ln composition otherwise.
///
/// # Example
/// ```
/// use solmath::{pow_fixed, SCALE};
/// // 2.0 ^ 1.5 = 2*sqrt(2) ≈ 2.828427...
/// let result = pow_fixed(2 * SCALE, 1_500_000_000_000)?;
/// assert!((result as i128 - 2_828_427_124_746i128).abs() <= 5);
/// # Ok::<(), solmath::SolMathError>(())
/// ```
pub fn pow_fixed(base: u128, exponent: u128) -> Result<u128, SolMathError> {
    // Special cases
    if base == 0 && exponent == 0 {
        return Err(SolMathError::DomainError); // 0^0 is undefined
    }
    if exponent == 0 {
        return Ok(SCALE); // x^0 = 1
    }
    if base == 0 {
        return Ok(0); // 0^y = 0 for y > 0
    }
    if exponent == SCALE {
        return Ok(base); // x^1 = x
    }
    if base == SCALE {
        return Ok(SCALE); // 1^y = 1
    }

    // x^2 = x * x (cheaper than exp∘ln)
    if exponent == 2 * SCALE {
        return fp_mul(base, base);
    }

    // x^0.5 — use sqrt (cheaper and more accurate)
    if exponent == SCALE / 2 {
        return fp_sqrt(base);
    }

    // Integer exponent: use HP path for accuracy (≤1 ULP)
    if exponent % SCALE == 0 {
        let n = exponent / SCALE;
        if n >= 1 && n <= 20 {
            return pow_fixed_hp(base, exponent);
        }
    }

    // The standard log now preserves raw near-one deltas, but its sub-ULP
    // log error can still be amplified by a large real exponent. Keep that
    // narrow composition on the HP path; ordinary exponents and the special
    // square/square-root branches above retain the cheaper implementation.
    if base.abs_diff(SCALE) < 1_000_000 && exponent > SCALE {
        return pow_fixed_hp(base, exponent);
    }

    // General case: exp(exponent * ln(base))
    let ln_base = ln_fixed_i(base)?; // i128
    if ln_base == 0 && base != SCALE {
        // Standard-scale ln cannot resolve bases one or a few raw units from
        // one. A huge exponent would otherwise turn that lost bit into a
        // catastrophic result of exactly one.
        return pow_fixed_hp(base, exponent);
    }
    let exp_i = match i128::try_from(exponent) {
        Ok(v) => v,
        Err(_) if base < SCALE => return Ok(0),
        Err(_) => return Err(SolMathError::Overflow),
    };
    let product = match fp_mul_i(exp_i, ln_base) {
        Ok(v) => v,
        Err(SolMathError::Overflow) if base < SCALE => return Ok(0),
        Err(e) => return Err(e),
    }; // exponent * ln(base)
    let result = exp_fixed_i(product)?;
    Ok(if result <= 0 { 0 } else { result as u128 })
}

#[cfg(test)]
mod adversarial_power_tests {
    use super::*;

    #[test]
    fn huge_positive_exponents_preserve_direction() {
        assert_eq!(pow_fixed(SCALE / 2, 1u128 << 127), Ok(0));
        assert_eq!(
            pow_fixed(2 * SCALE, 1u128 << 127),
            Err(SolMathError::Overflow)
        );
        assert_eq!(pow_fixed(SCALE / 10, i128::MAX as u128), Ok(0));
    }

    #[test]
    fn near_one_large_exponent_uses_high_precision_log() {
        let exponent = 1u128 << 80;
        assert_eq!(
            pow_fixed(SCALE + 1, exponent),
            pow_fixed_hp(SCALE + 1, exponent)
        );
        assert_eq!(
            pow_fixed(SCALE + 1, 1u128 << 86),
            Err(SolMathError::Overflow)
        );
    }
}

/// Integer power: base^n via repeated squaring or HP path.
///
/// - **base**: unsigned fixed-point at `SCALE` (1e12).
/// - **n**: raw integer (NOT scaled by SCALE). e.g. pass `3` for cubing.
/// - **Returns**: `u128` at `SCALE`.
/// - **Errors**: `Overflow` if result too large, `DomainError` via `ln_fixed_i` if base == 0 and n >= 5.
///
/// Near-exact for n <= 4 (direct multiply). Uses HP exp/ln for n >= 5.
pub fn pow_int(base: u128, n: u128) -> Result<u128, SolMathError> {
    match n {
        0 => Ok(SCALE),
        1 => Ok(base),
        _ if base == 0 => Ok(0),
        _ if base == SCALE => Ok(SCALE),
        2 => fp_mul(base, base),
        3 => Ok(fp_mul(fp_mul(base, base)?, base)?),
        4 => {
            let x2 = fp_mul(base, base)?;
            fp_mul(x2, x2)
        }
        _ => {
            // Check if n * ln(base) fits in HP exp's working range
            let ln_base = ln_fixed_i(base)?;
            let n_i = match i128::try_from(n) {
                Ok(value) => value,
                Err(_) if base < SCALE => return Ok(0),
                Err(_) => return Err(SolMathError::Overflow),
            };
            let total = match n_i.checked_mul(ln_base) {
                Some(v) => v,
                None => {
                    if ln_base > 0 {
                        return Err(SolMathError::Overflow);
                    } else {
                        return Ok(0);
                    }
                }
            };
            if total.unsigned_abs() < (39 * SCALE_I) as u128 {
                let exponent = n.checked_mul(SCALE).ok_or(SolMathError::Overflow)?;
                pow_fixed_hp(base, exponent)
            } else {
                // Split: base^n = (base^(n/2))² × base^(n%2)
                let half = pow_int(base, n / 2)?;
                let mut result =
                    checked_mul_div_u(half, half, SCALE).ok_or(SolMathError::Overflow)?;
                if n % 2 == 1 {
                    result =
                        checked_mul_div_u(result, base, SCALE).ok_or(SolMathError::Overflow)?;
                }
                Ok(result)
            }
        }
    }
}

/// Signed fixed-point power: base^exponent for signed values.
///
/// - **base**: signed fixed-point at `SCALE` (1e12). Negative bases allowed for integer exponents.
/// - **exponent**: signed fixed-point at `SCALE` (1e12). Negative exponents compute 1/base^|exp|.
/// - **Returns**: `i128` at `SCALE`.
/// - **Errors**: `DomainError` for `0^0` or fractional exponent with negative base.
///   `Overflow` if result too large or 1/0 from negative exponent.
pub fn pow_fixed_i(base: i128, exponent: i128) -> Result<i128, SolMathError> {
    // Special cases
    if base == 0 && exponent == 0 {
        return Err(SolMathError::DomainError); // 0^0 is undefined
    }
    if exponent == 0 {
        return Ok(SCALE_I); // x^0 = 1
    }
    if base == 0 {
        if exponent < 0 {
            return Err(SolMathError::Overflow); // 0^neg → 1/0 → overflow
        }
        return Ok(0); // 0^y = 0 for y > 0
    }
    if exponent == SCALE_I {
        return Ok(base); // x^1 = x
    }
    if base == SCALE_I {
        return Ok(SCALE_I); // 1^y = 1
    }

    // Negative exponent: 1 / pow(base, |exponent|)
    if exponent < 0 {
        let positive_exponent = exponent.checked_neg().ok_or(SolMathError::Overflow)?;
        let pos_result = pow_fixed_i(base, positive_exponent)?;
        if pos_result == 0 {
            return Err(SolMathError::Overflow); // 1/0 → overflow
        }
        return fp_div_i(SCALE_I, pos_result);
    }

    // Negative base: only valid for integer exponents
    if base < 0 {
        if exponent % SCALE_I != 0 {
            return Err(SolMathError::DomainError); // fractional power of negative is undefined
        }
        let n = exponent / SCALE_I;
        let abs_base = base.unsigned_abs();
        let abs_result = pow_fixed(abs_base, exponent as u128)?;
        if abs_result > i128::MAX as u128 {
            return Err(SolMathError::Overflow);
        }
        Ok(if n % 2 == 0 {
            abs_result as i128
        } else {
            -(abs_result as i128)
        })
    } else {
        // Positive base, positive exponent — delegate to unsigned
        let result = pow_fixed(base as u128, exponent as u128)?;
        if result > i128::MAX as u128 {
            return Err(SolMathError::Overflow);
        }
        Ok(result as i128)
    }
}

/// exp(x) - 1 with better precision near zero: (e^(x/SCALE) - 1) * SCALE.
///
/// Uses a 1,292-segment midpoint table and Q43 cubic residual with one wide
/// multiply. A correctly-rounded direct path preserves raw increments near zero.
///
/// - **x**: signed fixed-point at `SCALE` (1e12).
/// - **Returns**: `i128` at `SCALE`. Near zero for small x.
/// - **Errors**: `Overflow` when `x >= 40*SCALE`; saturates to `-SCALE` when
///   `x <= -40*SCALE`.
/// - **Accuracy**: for `|x| <= 2`, max 3 ULP, P99 2 ULP, median 0 over
///   60,000 retained production vectors. Measured full-domain relative error
///   is below one part per trillion for outputs with magnitude at least one.
pub fn expm1_fixed(x: i128) -> Result<i128, SolMathError> {
    let limit = 40 * SCALE_I;
    if x <= -limit {
        return Ok(-SCALE_I);
    }
    if x >= limit {
        return Err(SolMathError::Overflow);
    }

    // On |x| < 1e-6, expm1(x)-x = x²/2+O(x³) is below half a raw unit.
    if x.unsigned_abs() < 1_000_000 {
        return Ok(x);
    }

    let x64 = x as i64; // |x| < 40*SCALE_I < i64::MAX.
    let mut k = round_shift_i64(x64 * EXPM1_INV_LN2_Q56, 56) as i32;
    debug_assert!((K_LN2_MIN..=64).contains(&k));
    let mut r = x64 - K_LN2_RAW[(k - K_LN2_MIN) as usize];
    const HALF_LN2_RAW: i64 = 346_573_590_280;
    if r > HALF_LN2_RAW {
        k += 1;
        r = x64 - K_LN2_RAW[(k - K_LN2_MIN) as usize];
    } else if r < -HALF_LN2_RAW {
        k -= 1;
        r = x64 - K_LN2_RAW[(k - K_LN2_MIN) as usize];
    }

    let offset = (r - EXPM1_R_MIN) as u64;
    debug_assert!(r >= EXPM1_R_MIN);
    let j = ((offset >> EXPM1_LUT_STEP_SHIFT) as usize).min(EXPM1_LUT_SEGMENTS - 1);
    let midpoint = EXPM1_R_MIN + j as i64 * EXPM1_LUT_STEP + EXPM1_LUT_STEP / 2;
    let delta = r - midpoint;

    // exp(delta) at Q43. |delta| <= 2^28 raw, so all products fit i64;
    // the omitted quartic contributes less than 0.001 raw unit before scaling.
    let q = round_shift_i64(delta * EXPM1_RAW_TO_Q43_G31, 31);
    let q2 = mul_q43(q, q);
    let q3 = mul_q43(q2, q);
    let local_q43 = (1i64 << 43) + q + q2 / 2 + q3 / 6;

    // The midpoint already contains decimal SCALE with 22 binary guard bits,
    // so one wide product produces the final-scale value without a second
    // wide multiplication by SCALE.
    let exp_r_raw_q22 =
        round_shift_signed(EXPM1_MID_EXP_RAW_Q22[j] as i128 * local_q43 as i128, 43);
    let shift = 22 - k;
    let exp_x = if shift > 0 {
        round_shift_signed(exp_r_raw_q22, shift as u32)
    } else if shift == 0 {
        exp_r_raw_q22
    } else {
        exp_r_raw_q22
            .checked_shl((-shift) as u32)
            .ok_or(SolMathError::Overflow)?
    };
    Ok(exp_x - SCALE_I)
}

#[cfg(test)]
mod boundary_tests {
    use super::*;

    #[test]
    fn pow_fixed_rejects_unsigned_exponent_that_cannot_be_signed() {
        assert_eq!(
            pow_fixed(2 * SCALE, i128::MAX as u128 + 1),
            Err(SolMathError::Overflow)
        );
    }

    #[test]
    fn pow_int_handles_zero_and_extreme_exponents_consistently() {
        assert_eq!(pow_int(0, 5), Ok(0));
        assert_eq!(pow_int(SCALE, u128::MAX), Ok(SCALE));
        assert_eq!(pow_int(SCALE / 2, i128::MAX as u128 + 1), Ok(0));
        assert_eq!(
            pow_int(2 * SCALE, i128::MAX as u128 + 1),
            Err(SolMathError::Overflow)
        );
    }

    #[test]
    fn pow_fixed_i_rejects_unnegatable_min_exponent() {
        assert_eq!(
            pow_fixed_i(2 * SCALE_I, i128::MIN),
            Err(SolMathError::Overflow)
        );
    }

    #[test]
    fn ln_1p_has_explicit_domain_and_exact_special_values() {
        assert_eq!(ln_1p_fixed(-SCALE_I), Err(SolMathError::DomainError));
        assert_eq!(ln_1p_fixed(i128::MIN), Err(SolMathError::DomainError));
        assert_eq!(ln_1p_fixed(0), Ok(0));
        assert_eq!(ln_1p_fixed(SCALE_I), ln_fixed_i(2 * SCALE));
        assert!(ln_1p_fixed(i128::MAX).is_ok());
    }

    #[test]
    fn ln_1p_preserves_raw_increments_near_zero() {
        assert_eq!(ln_1p_fixed(1), Ok(1));
        assert_eq!(ln_1p_fixed(-1), Ok(-1));
        assert_eq!(ln_1p_fixed(2), Ok(2));
        assert_eq!(ln_1p_fixed(-2), Ok(-2));
    }

    #[test]
    fn ln_1p_and_expm1_round_trip_financial_rates() {
        for x in [
            -900_000_000_000,
            -500_000_000_000,
            -10_000_000_000,
            -1_000_000,
            1_000_000,
            10_000_000_000,
            500_000_000_000,
            5 * SCALE_I,
        ] {
            let recovered = expm1_fixed(ln_1p_fixed(x).unwrap()).unwrap();
            assert!((recovered - x).abs() <= 12, "x={x}, recovered={recovered}");
        }
    }

    #[test]
    fn expm1_has_explicit_limits_and_preserves_raw_increments() {
        let limit = 40 * SCALE_I;
        assert_eq!(expm1_fixed(i128::MIN), Ok(-SCALE_I));
        assert_eq!(expm1_fixed(-limit), Ok(-SCALE_I));
        assert_eq!(expm1_fixed(limit), Err(SolMathError::Overflow));
        assert_eq!(expm1_fixed(i128::MAX), Err(SolMathError::Overflow));
        assert_eq!(expm1_fixed(-1), Ok(-1));
        assert_eq!(expm1_fixed(0), Ok(0));
        assert_eq!(expm1_fixed(1), Ok(1));
    }

    #[test]
    fn expm1_matches_known_ordinary_values() {
        assert!(expm1_fixed(SCALE_I).unwrap().abs_diff(1_718_281_828_459) <= 3);
        assert!(expm1_fixed(-SCALE_I).unwrap().abs_diff(-632_120_558_829) <= 1);
    }

    #[test]
    fn expm1_is_monotone_across_every_lut_boundary_and_exponent() {
        for k in -58..=58 {
            let k_ln2 = K_LN2_RAW[(k - K_LN2_MIN) as usize] as i128;
            for j in 1..EXPM1_LUT_SEGMENTS {
                let boundary = EXPM1_R_MIN as i128 + j as i128 * EXPM1_LUT_STEP as i128;
                let x_left = k_ln2 + boundary - 1;
                let x_right = k_ln2 + boundary;
                if x_left <= -40 * SCALE_I || x_right >= 40 * SCALE_I {
                    continue;
                }
                let left = expm1_fixed(x_left).unwrap();
                let right = expm1_fixed(x_right).unwrap();
                assert!(left <= right, "k={k}, j={j}: {left} > {right}");
            }
        }
    }
}
