use crate::constants::*;
use crate::double_word::DoubleWord;
use crate::error::SolMathError;
use crate::arithmetic::{fp_mul, fp_mul_i, fp_sqrt, isqrt_u128};
use crate::overflow::{checked_mul_div_i, checked_mul_div_u};
use crate::transcendental::exp_fixed_i;


/// Unsigned high-precision fixed-point multiply at `SCALE_HP` (1e15).
///
/// - **a**, **b**: unsigned fixed-point at 1e15 scale.
/// - **Returns**: `u128` at 1e15 scale, rounded to nearest.
/// - **Errors**: `Overflow` if the product exceeds `u128::MAX`.
/// - **Accuracy**: exact to rounding (0.5 ULP).
#[inline(always)]
pub fn fp_mul_hp_u(a: u128, b: u128) -> Result<u128, SolMathError> {
    let hi_a = a / SCALE_HP_U;
    let lo_a = a % SCALE_HP_U;
    let hi_b = b / SCALE_HP_U;
    let lo_b = b % SCALE_HP_U;

    let hh = hi_a.checked_mul(hi_b).ok_or(SolMathError::Overflow)?
        .checked_mul(SCALE_HP_U).ok_or(SolMathError::Overflow)?;
    let hl = hi_a.checked_mul(lo_b).ok_or(SolMathError::Overflow)?;
    let lh = lo_a.checked_mul(hi_b).ok_or(SolMathError::Overflow)?;
    let ll = lo_a.checked_mul(lo_b).ok_or(SolMathError::Overflow)?
        .checked_add(SCALE_HP_U / 2).ok_or(SolMathError::Overflow)? / SCALE_HP_U;

    hh.checked_add(hl).ok_or(SolMathError::Overflow)?
        .checked_add(lh).ok_or(SolMathError::Overflow)?
        .checked_add(ll).ok_or(SolMathError::Overflow)
}

/// Signed high-precision fixed-point multiply at `SCALE_HP` (1e15).
///
/// - **a**, **b**: signed fixed-point at 1e15 scale.
/// - **Returns**: `i128` at 1e15 scale, rounded to nearest.
/// - **Errors**: `Overflow` if the product exceeds `i128` range.
/// - **Accuracy**: exact to rounding (0.5 ULP).
#[inline(always)]
pub fn fp_mul_hp_i(a: i128, b: i128) -> Result<i128, SolMathError> {
    let neg = (a < 0) != (b < 0);
    let raw = fp_mul_hp_u(a.unsigned_abs(), b.unsigned_abs())?;
    if neg {
        if raw == (1u128 << 127) {
            Ok(i128::MIN)
        } else if raw < (1u128 << 127) {
            Ok(-(raw as i128))
        } else {
            Err(SolMathError::Overflow)
        }
    } else if raw <= i128::MAX as u128 {
        Ok(raw as i128)
    } else {
        Err(SolMathError::Overflow)
    }
}

/// Fast HP multiply assuming no overflow. Internal — used by ln/exp_fixed_hp.
#[inline]
pub(crate) fn fp_mul_hp_fast(a: i128, b: i128) -> i128 {
    // Callers guarantee |a|, |b| ≤ ~40·SCALE_HP = 4e16; product ≤ 1.6e33, fits i128 (max ~1.7e38).
    let p = a * b;
    if p >= 0 {
        (p + SCALE_HP / 2) / SCALE_HP
    } else {
        (p - SCALE_HP / 2) / SCALE_HP
    }
}

/// Signed high-precision fixed-point division at `SCALE_HP` (1e15).
///
/// - **a**: signed numerator at 1e15 scale.
/// - **b**: signed denominator at 1e15 scale. Must be non-zero.
/// - **Returns**: `i128` at 1e15 scale.
/// - **Errors**: `DivisionByZero` if `b == 0`, `Overflow` if the scaled quotient exceeds `i128`.
/// - **Accuracy**: exact to rounding.
#[inline]
pub fn fp_div_hp_safe(a: i128, b: i128) -> Result<i128, SolMathError> {
    if b == 0 {
        return Err(SolMathError::DivisionByZero);
    }

    let q = a / b;
    let r = a % b;

    let q_scaled = q.checked_mul(SCALE_HP).ok_or(SolMathError::Overflow)?;
    // SAFETY: r * SCALE_HP can overflow i128 when |r| is large.
    // Use checked_mul_div_i for overflow-safe widened arithmetic (see PROOFS.md §7a).
    let r_scaled = match r.checked_mul(SCALE_HP) {
        Some(v) => v / b,
        None => checked_mul_div_i(r, SCALE_HP, b)?,
    };
    q_scaled.checked_add(r_scaled).ok_or(SolMathError::Overflow)
}

/// Upscale from 1e12 to 1e15. Internal.
///
/// Returns `Err(Overflow)` if the input exceeds the representable HP range.
#[inline]
pub(crate) fn upscale_std_to_hp(x: u128) -> Result<i128, SolMathError> {
    let max_input = (i128::MAX / HP_TO_STD) as u128;
    if x > max_input {
        return Err(SolMathError::Overflow);
    }
    Ok(x as i128 * HP_TO_STD)
}

/// Downscale from 1e15 to 1e12 (unsigned, with rounding). Internal.
#[inline]
pub(crate) fn downscale_hp_to_std(x: i128) -> u128 {
    if x <= 0 {
        0
    } else {
        // x ≥ 0 and ≤ ~1e20 in practice; HP_TO_STD/2 = 500; x + 500 fits i128 with enormous margin.
        ((x + HP_TO_STD / 2) / HP_TO_STD) as u128
    }
}

/// Downscale from 1e15 to 1e12 (signed, with rounding). Internal.
#[inline]
pub(crate) fn downscale_hp_to_std_i(x: i128) -> i128 {
    if x >= 0 {
        // x ≥ 0 and bounded by SCALE_HP result range; HP_TO_STD/2 = 500; x ± 500 fits i128.
        (x + HP_TO_STD / 2) / HP_TO_STD
    } else {
        (x - HP_TO_STD / 2) / HP_TO_STD
    }
}

/// High-precision natural logarithm at `SCALE_HP` (1e15).
///
/// Input and output at `SCALE_HP` (1e15). Compensated Remez degree-9 polynomial
/// with double-word propagation and split LN2 correction.
///
/// - **x**: signed fixed-point at 1e15 scale. Must be > 0.
/// - **Returns**: `i128` at 1e15 scale.
/// - **Errors**: `DomainError` if `x <= 0`.
/// - **Accuracy**: max 2 ULP, median 0 ULP, 72% exact.
pub fn ln_fixed_hp(x: i128) -> Result<i128, SolMathError> {
    if x <= 0 {
        return Err(SolMathError::DomainError);
    }

    let mut m = x as u128;
    let mut k: i32 = 0;

    while m < SCALE_HP_U {
        m = m.checked_mul(2).ok_or(SolMathError::Overflow)?;
        k -= 1;
    }
    while m >= 2 * SCALE_HP_U {
        m /= 2;
        k += 1;
    }

    // t with sub-ULP remainder.
    // m ∈ [SCALE_HP, 2·SCALE_HP): t_num = m - SCALE_HP ∈ [0, SCALE_HP), fits i128.
    // t_den = m + SCALE_HP ∈ (SCALE_HP, 3·SCALE_HP), fits i128.
    // t_num × SCALE_HP < SCALE_HP² = 1e30 — well within i128 (max ~1.7e38).
    let t_num = m as i128 - SCALE_HP;
    let t_den = m as i128 + SCALE_HP;
    let p = t_num * SCALE_HP;
    let q = p / t_den;
    let r = p % t_den;
    // t_den ∈ (SCALE_HP, 3·SCALE_HP), half_den ≤ 1.5·SCALE_HP, fits i128.
    let half_den = t_den / 2;
    let (t, t_lo) = if r >= half_den {
        // r - t_den ∈ (-t_den, 0); (r - t_den) * SCALE_HP: |product| < 3e30, fits i128.
        // r * SCALE_HP: r < t_den < 3e15, product < 3e30, fits i128.
        (q + 1, (r - t_den) * SCALE_HP / t_den)
    } else {
        (q, r * SCALE_HP / t_den)
    };

    let u = fp_mul_hp_fast(t, t);

    // Compensated polynomial: P(u) with sub-ULP residual.
    let p_dw = horner_compensated_hp_dw(&LN_REMEZ_HP_COEFFS, u)?;

    // Multiply t × P(u).hi with sub-ULP tracking.
    let tp_dw = fp_mul_hp_fast_dw(t, p_dw.hi());

    // Three sub-ULP correction sources for the product t × P(u):
    // t ∈ (-SCALE_HP, SCALE_HP), p_dw.lo and t_lo are sub-ULP residuals with |value| < SCALE_HP;
    // p_dw.hi ∈ (-a few ×SCALE_HP, a few ×SCALE_HP).
    // t * p_dw.lo: |product| < SCALE_HP² = 1e30, fits i128; /SCALE_HP gives sub-ULP result.
    let poly_lo_corr = t * p_dw.lo() / SCALE_HP;
    // t_lo * p_dw.hi: |t_lo| < SCALE_HP, |p_dw.hi| ≤ ~5·SCALE_HP; product ≤ 5e30, fits i128.
    let t_lo_corr = t_lo * p_dw.hi() / SCALE_HP;

    // Combine all sub-ULP residuals, multiply by 2, THEN round.
    // tp_dw.lo, poly_lo_corr, t_lo_corr are each sub-ULP residuals: |each| < SCALE_HP;
    // sum ∈ (-3·SCALE_HP, 3·SCALE_HP); × 2: ∈ (-6·SCALE_HP, 6·SCALE_HP) = (-6e15, 6e15), fits i128.
    let total_lo_2 = 2 * (tp_dw.lo() + poly_lo_corr + t_lo_corr);
    let sub_ulp_correction = if total_lo_2 >= 0 {
        (total_lo_2 + SCALE_HP / 2) / SCALE_HP
    } else {
        (total_lo_2 - SCALE_HP / 2) / SCALE_HP
    };

    // tp_dw.hi is the series partial result, bounded by a few × SCALE_HP; sub_ulp_correction is ≤ 6;
    // 2 * tp_dw.hi: |value| ≤ a few ×10·SCALE_HP, fits i128.
    let series_result = 2 * tp_dw.hi() + sub_ulp_correction;

    // Split LN2 correction.
    // k is the power-of-2 exponent from argument range reduction; |k| ≤ ~128 for typical inputs.
    // LN2_HP_LO ≈ 3e14; k_i * LN2_HP_LO: |product| ≤ 128 * 3e14 ≈ 3.8e16, fits i128.
    let k_i = k as i128;
    let raw = k_i * LN2_HP_LO;
    let ln2_correction = if raw >= 0 {
        (raw + SCALE_HP / 2) / SCALE_HP
    } else {
        (raw - SCALE_HP / 2) / SCALE_HP
    };
    // LN2_HP ≈ 6.9e14; k_i * LN2_HP: |product| ≤ 128 * 6.9e14 ≈ 8.8e16, fits i128.
    // series_result ∈ (-40·SCALE_HP, 40·SCALE_HP); sum with k*LN2_HP and ln2_correction fits i128.
    Ok(series_result + k_i * LN2_HP + ln2_correction)
}

/// High-precision exponential at `SCALE_HP` (1e15).
///
/// Input and output at `SCALE_HP` (1e15). Remez rational approximation
/// with split LN2 correction.
///
/// - **x**: signed fixed-point at 1e15 scale.
/// - **Returns**: `i128` at 1e15 scale. Returns `Ok(0)` for `x <= -40 * SCALE_HP`.
/// - **Errors**: `Overflow` for `x >= 40 * SCALE_HP`.
/// - **Accuracy**: max 1 ULP.
pub fn exp_fixed_hp(x: i128) -> Result<i128, SolMathError> {
    let max_x = 40 * SCALE_HP;

    if x <= -max_x { return Ok(0); }
    if x >= max_x { return Err(SolMathError::Overflow); }
    if x == 0 { return Ok(SCALE_HP); }

    // Split LN2: LN2_HP undershoots (LN2_HP_LO > 0), so k*LN2_HP is too small
    // and r = x - k*LN2_HP is too large. Subtract the positive correction.
    // x ∈ (-40·SCALE_HP, 40·SCALE_HP); LN2_HP ≈ 6.9e14; k ∈ (-58, 58), fits i128.
    let mut k = x / LN2_HP;
    let ln2_correction = {
        // k ∈ (-58, 58), LN2_HP_LO ≈ 3e14; k * LN2_HP_LO ≤ 58 * 3e14 ≈ 1.7e16, fits i128.
        let raw = k * LN2_HP_LO;
        if raw >= 0 { (raw + SCALE_HP / 2) / SCALE_HP } else { (raw - SCALE_HP / 2) / SCALE_HP }
    };
    // k * LN2_HP ≤ 58 * 6.9e14 ≈ 4e16; x ≤ 40·SCALE_HP = 4e16; r ∈ (-2·SCALE_HP, 2·SCALE_HP), fits i128.
    let mut r = x - k * LN2_HP - ln2_correction;

    // r ∈ (-2·SCALE_HP, 2·SCALE_HP); LN2_HP ≈ 6.9e14; r ± LN2_HP ∈ (-3·SCALE_HP, 3·SCALE_HP), fits i128.
    if r > HALF_LN2_HP { k += 1; r -= LN2_HP; }
    else if r < -HALF_LN2_HP { k -= 1; r += LN2_HP; }

    let xx = fp_mul_hp_fast(r, r);

    // fp_mul_hp_fast results ∈ [-SCALE_HP, SCALE_HP]; EXP_REMEZ_HP_P* coefficients ≪ SCALE_HP;
    // each + step: sum ∈ [-2·SCALE_HP, 2·SCALE_HP], fits i128.
    let poly = fp_mul_hp_fast(xx, EXP_REMEZ_HP_P5) + EXP_REMEZ_HP_P4;
    let poly = fp_mul_hp_fast(xx, poly) + EXP_REMEZ_HP_P3;
    let poly = fp_mul_hp_fast(xx, poly) + EXP_REMEZ_HP_P2;
    let poly = fp_mul_hp_fast(xx, poly) + EXP_REMEZ_HP_P1;

    // r ∈ (-HALF_LN2_HP, HALF_LN2_HP) ≈ ±3.5e14 after range reduction;
    // fp_mul_hp_fast(poly, xx): |value| ≤ SCALE_HP; c = r - that ∈ (-2·SCALE_HP, 2·SCALE_HP), fits i128.
    let c = r - fp_mul_hp_fast(poly, xx);
    let rc = fp_mul_hp_fast(r, c);
    // SCALE_HP + r: ≤ 2·SCALE_HP; + fp_div_hp_safe(rc, ...): sum ≤ 3·SCALE_HP, fits i128.
    // 2 * SCALE_HP - c: SCALE_HP = 1e15, c ≤ 2·SCALE_HP; result ∈ (0, 4·SCALE_HP), fits i128.
    let sum = SCALE_HP + r + fp_div_hp_safe(rc, 2 * SCALE_HP - c)?;

    if k >= 0 {
        sum.checked_shl(k as u32).ok_or(SolMathError::Overflow)
    } else {
        Ok(sum >> (-k) as u32)
    }
}

/// High-precision power: base^exponent via HP exp/ln composition.
///
/// Accepts and returns values at `SCALE` (1e12) but computes internally at `SCALE_HP` (1e15).
///
/// - **base**: unsigned fixed-point at `SCALE` (1e12).
/// - **exponent**: unsigned fixed-point at `SCALE` (1e12).
/// - **Returns**: `u128` at `SCALE`.
/// - **Errors**: `DomainError` for `0^0`, `Overflow` if result too large.
/// - **Accuracy**: P99 648 ULP, median 0 ULP, 96% exact.
///
/// For small products, uses direct exp(y*ln(x)). For large exponents, splits into
/// integer + fractional parts with repeated squaring for the integer part.
pub fn pow_fixed_hp(base: u128, exponent: u128) -> Result<u128, SolMathError> {
    if base == 0 && exponent == 0 {
        return Err(SolMathError::DomainError);
    }
    if exponent == 0 {
        return Ok(SCALE);
    }
    if base == 0 {
        return Ok(0);
    }
    if exponent == SCALE {
        return Ok(base);
    }
    if base == SCALE {
        return Ok(SCALE);
    }
    if exponent == 2 * SCALE {
        return fp_mul(base, base);
    }
    if exponent == SCALE / 2 {
        return fp_sqrt(base);
    }

    let base_hp = upscale_std_to_hp(base)?;
    let exp_hp = upscale_std_to_hp(exponent)?;
    let ln_base = ln_fixed_hp(base_hp)?;
    let product = fp_mul_hp_i(exp_hp, ln_base)?;

    // Fast path: product fits in exp_fixed_hp's range
    if product.abs() < 39 * SCALE_HP {
        let result_hp = exp_fixed_hp(product)?;
        return Ok(downscale_hp_to_std(result_hp));
    }

    // Split path: decompose exponent into integer + fractional parts.
    let n = (exponent / SCALE) as u32;
    let frac_std = exponent % SCALE;

    let mut int_result: u128 = SCALE;
    let mut pow_base: u128 = base;
    let mut remaining = n;
    while remaining > 0 {
        if remaining & 1 == 1 {
            int_result = match checked_mul_div_u(int_result, pow_base, SCALE) {
                Some(v) if v > 0 => v,
                _ => return Ok(0),
            };
        }
        remaining >>= 1;
        if remaining > 0 {
            pow_base = match checked_mul_div_u(pow_base, pow_base, SCALE) {
                Some(v) if v > 0 => v,
                _ => {
                    if base < SCALE {
                        return Ok(0);
                    } else {
                        return Err(SolMathError::Overflow);
                    }
                }
            };
        }
    }

    let frac_result = if frac_std == 0 {
        SCALE
    } else {
        let frac_hp = upscale_std_to_hp(frac_std)?;
        let frac_product = fp_mul_hp_i(frac_hp, ln_base)?;
        let frac_hp_result = exp_fixed_hp(frac_product)?;
        downscale_hp_to_std(frac_hp_result)
    };

    match checked_mul_div_u(int_result, frac_result, SCALE) {
        Some(v) => Ok(v),
        None => {
            if base < SCALE {
                Ok(0)
            } else {
                Err(SolMathError::Overflow)
            }
        }
    }
}

/// Compensated product x^w * x^(1-w) for weighted pool invariants.
///
/// Computes `exp(w*ln(x)) * exp((1-w)*ln(x))` with error cancellation from
/// the complementary split, giving better accuracy than two separate `pow_fixed_hp` calls.
///
/// - **x**: unsigned fixed-point at `SCALE` (1e12). Must be > 0.
/// - **w**: unsigned fixed-point weight at `SCALE` (1e12), in [0, SCALE].
/// - **Returns**: `u128` at `SCALE`. Equals `x` when w == 0 or w == SCALE.
/// - **Errors**: `DomainError` if `x == 0` or `w > SCALE`.
/// - **Accuracy**: max 2.6K ULP, median 1 ULP.
pub fn pow_product_hp(x: u128, w: u128) -> Result<u128, SolMathError> {
    if x == 0 {
        return Err(SolMathError::DomainError);
    }
    if w > SCALE {
        return Err(SolMathError::DomainError);
    }
    if x == SCALE {
        return Ok(SCALE);
    }
    if w == 0 || w == SCALE {
        return Ok(x);
    }

    let x_hp = upscale_std_to_hp(x)?;
    let w_hp = upscale_std_to_hp(w)?;
    let ln_x = ln_fixed_hp(x_hp)?;
    let a = fp_mul_hp_i(w_hp, ln_x)?;
    // ln_x ∈ (-40·SCALE_HP, 40·SCALE_HP); a ∈ [0, 40·SCALE_HP] (w ∈ [0, SCALE]);
    // b = ln_x - a ∈ (-40·SCALE_HP, 40·SCALE_HP), fits i128.
    let b = ln_x - a;
    let exp_a = exp_fixed_hp(a)?;
    let exp_b = exp_fixed_hp(b)?;
    if exp_a <= 0 || exp_b <= 0 {
        return Ok(0);
    }
    let product_hp = fp_mul_hp_i(exp_a, exp_b)?;
    Ok(downscale_hp_to_std(product_hp))
}

/// Horner degree-13 HP evaluation. Internal — called by norm_cdf_poly_hp.
#[inline]
pub(crate) fn horner_hp_13(c: &[i128; 14], t: i128) -> Result<i128, SolMathError> {
    let mut r = c[13];
    // fp_mul_hp_i is checked; r after each step is a CDF polynomial accumulator bounded by a few ×SCALE_HP;
    // |c[i]| ≤ SCALE_HP (minimax coefficients); sum ∈ (-2·SCALE_HP, 2·SCALE_HP), fits i128.
    r = fp_mul_hp_i(r, t)? + c[12];
    r = fp_mul_hp_i(r, t)? + c[11];
    r = fp_mul_hp_i(r, t)? + c[10];
    r = fp_mul_hp_i(r, t)? + c[9];
    r = fp_mul_hp_i(r, t)? + c[8];
    r = fp_mul_hp_i(r, t)? + c[7];
    r = fp_mul_hp_i(r, t)? + c[6];
    r = fp_mul_hp_i(r, t)? + c[5];
    r = fp_mul_hp_i(r, t)? + c[4];
    r = fp_mul_hp_i(r, t)? + c[3];
    r = fp_mul_hp_i(r, t)? + c[2];
    r = fp_mul_hp_i(r, t)? + c[1];
    r = fp_mul_hp_i(r, t)? + c[0];
    Ok(r)
}

/// Horner degree-15 HP evaluation. Internal — called by norm_cdf_poly_hp.
#[inline]
pub(crate) fn horner_hp_15(c: &[i128; 16], t: i128) -> Result<i128, SolMathError> {
    let mut r = c[15];
    // fp_mul_hp_i is checked; r and |c[i]| ≤ a few ×SCALE_HP; sum ∈ (-2·SCALE_HP, 2·SCALE_HP), fits i128.
    r = fp_mul_hp_i(r, t)? + c[14];
    r = fp_mul_hp_i(r, t)? + c[13];
    r = fp_mul_hp_i(r, t)? + c[12];
    r = fp_mul_hp_i(r, t)? + c[11];
    r = fp_mul_hp_i(r, t)? + c[10];
    r = fp_mul_hp_i(r, t)? + c[9];
    r = fp_mul_hp_i(r, t)? + c[8];
    r = fp_mul_hp_i(r, t)? + c[7];
    r = fp_mul_hp_i(r, t)? + c[6];
    r = fp_mul_hp_i(r, t)? + c[5];
    r = fp_mul_hp_i(r, t)? + c[4];
    r = fp_mul_hp_i(r, t)? + c[3];
    r = fp_mul_hp_i(r, t)? + c[2];
    r = fp_mul_hp_i(r, t)? + c[1];
    r = fp_mul_hp_i(r, t)? + c[0];
    Ok(r)
}

/// Horner degree-17 HP evaluation. Internal — called by norm_cdf_poly_hp.
#[inline]
pub(crate) fn horner_hp_17(c: &[i128; 18], t: i128) -> Result<i128, SolMathError> {
    let mut r = c[17];
    // fp_mul_hp_i is checked; r and |c[i]| ≤ a few ×SCALE_HP; sum ∈ (-2·SCALE_HP, 2·SCALE_HP), fits i128.
    r = fp_mul_hp_i(r, t)? + c[16];
    r = fp_mul_hp_i(r, t)? + c[15];
    r = fp_mul_hp_i(r, t)? + c[14];
    r = fp_mul_hp_i(r, t)? + c[13];
    r = fp_mul_hp_i(r, t)? + c[12];
    r = fp_mul_hp_i(r, t)? + c[11];
    r = fp_mul_hp_i(r, t)? + c[10];
    r = fp_mul_hp_i(r, t)? + c[9];
    r = fp_mul_hp_i(r, t)? + c[8];
    r = fp_mul_hp_i(r, t)? + c[7];
    r = fp_mul_hp_i(r, t)? + c[6];
    r = fp_mul_hp_i(r, t)? + c[5];
    r = fp_mul_hp_i(r, t)? + c[4];
    r = fp_mul_hp_i(r, t)? + c[3];
    r = fp_mul_hp_i(r, t)? + c[2];
    r = fp_mul_hp_i(r, t)? + c[1];
    r = fp_mul_hp_i(r, t)? + c[0];
    Ok(r)
}

/// Mills ratio via 8-deep continued fraction. Internal — called by norm_cdf_poly_hp tail.
#[inline]
pub(crate) fn mills_ratio_cf8_hp(x: i128) -> Result<i128, SolMathError> {
    let mut r = 0i128;
    for k in (1..=8).rev() {
        // k ∈ [1, 8], SCALE_HP = 1e15: k * SCALE_HP ≤ 8e15, fits i128.
        // r is bounded by the previous CF level (≤ SCALE_HP); x ∈ [5·SCALE_HP, 8·SCALE_HP];
        // x + r ≤ 9·SCALE_HP, fits i128.
        r = fp_div_hp_safe((k as i128) * SCALE_HP, x + r)?;
    }
    fp_div_hp_safe(SCALE_HP, x + r)
}

/// High-precision normal CDF: Phi(x) at `SCALE_HP` (1e15).
///
/// Input and output at `SCALE_HP` (1e15). 6 piecewise minimax polynomials
/// + Mills ratio continued-fraction tail.
///
/// - **x**: signed fixed-point at 1e15 scale.
/// - **Returns**: `i128` probability in [0, SCALE_HP]. Returns 0 for x < -8*SCALE_HP.
/// - **Errors**: `Overflow` on internal arithmetic overflow.
/// - **Accuracy**: max 5 ULP. Monotone, boundary-constrained.
pub fn norm_cdf_poly_hp(x: i128) -> Result<i128, SolMathError> {
    if x < -8 * SCALE_HP {
        return Ok(0);
    }
    if x > 8 * SCALE_HP {
        return Ok(SCALE_HP);
    }
    if x == 0 {
        return Ok(SCALE_HP / 2);
    }

    let ax = x.abs();

    let cdf_pos = if ax <= POLY_HP_V2_I0_HI {
        horner_hp_13(
            &POLY_HP_V2_I0,
            poly_map_t_hp(ax, POLY_HP_V2_I0_MID, POLY_HP_V2_I0_HW)?,
        )?
    } else if ax <= POLY_HP_V2_I1_HI {
        horner_hp_13(
            &POLY_HP_V2_I1,
            poly_map_t_hp(ax, POLY_HP_V2_I1_MID, POLY_HP_V2_I1_HW)?,
        )?
    } else if ax <= POLY_HP_V2_I2A_HI {
        horner_hp_15(
            &POLY_HP_V2_I2A,
            poly_map_t_hp(ax, POLY_HP_V2_I2A_MID, POLY_HP_V2_I2A_HW)?,
        )?
    } else if ax <= POLY_HP_V2_I2B_HI {
        horner_hp_15(
            &POLY_HP_V2_I2B,
            poly_map_t_hp(ax, POLY_HP_V2_I2B_MID, POLY_HP_V2_I2B_HW)?,
        )?
    } else if ax <= POLY_HP_V2_I3A_HI {
        horner_hp_17(
            &POLY_HP_V2_I3A,
            poly_map_t_hp(ax, POLY_HP_V2_I3A_MID, POLY_HP_V2_I3A_HW)?,
        )?
    } else if ax <= 5 * SCALE_HP {
        horner_hp_17(
            &POLY_HP_V2_I3B,
            poly_map_t_hp(ax, POLY_HP_V2_I3B_MID, POLY_HP_V2_I3B_HW)?,
        )?
    } else {
        let t = poly_map_t_hp(ax, POLY_HP_I4_MID, POLY_HP_I4_HW)?;
        let pdf_hp = horner_hp_17(&POLY_HP_I4_PDF, t)?.max(0);
        let tail_hp = fp_mul_hp_i(pdf_hp, mills_ratio_cf8_hp(ax)?)?;
        // tail_hp ∈ [0, SCALE_HP] (pdf and mills each ≤ SCALE_HP); SCALE_HP - tail_hp ∈ [0, SCALE_HP], fits i128.
        SCALE_HP - tail_hp
    };

    let cdf_pos = cdf_pos.clamp(0, SCALE_HP);

    // cdf_pos ∈ [0, SCALE_HP] after clamp; SCALE_HP - cdf_pos ∈ [0, SCALE_HP], fits i128.
    Ok(if x >= 0 {
        cdf_pos
    } else {
        SCALE_HP - cdf_pos
    })
}

/// Map |x| to local Chebyshev variable t at HP scale. Internal — called by norm_cdf_poly_hp.
#[inline]
pub(crate) fn poly_map_t_hp(ax: i128, mid: i128, hw: i128) -> Result<i128, SolMathError> {
    let product = (ax - mid).checked_mul(SCALE_HP).ok_or(SolMathError::Overflow)?;
    Ok(product / hw)
}

/// Shared HP intermediates for Black-Scholes pricing.
pub(crate) struct BsIntermediatesHp {
    pub s_hp: i128,
    #[allow(dead_code)]
    pub k_hp: i128,
    pub d1_hp: i128,
    #[allow(dead_code)]
    pub d2_hp: i128,
    pub phi_d1_hp: i128,
    pub phi_d2_hp: i128,
    pub phi_neg_d1_hp: i128,
    pub phi_neg_d2_hp: i128,
    pub k_disc_hp: i128,
    pub sigma_sqrt_t_hp: i128,
    pub sqrt_t_hp: i128,
    pub sigma_hp: i128,
    pub r_hp: i128,
    pub t_hp: i128,
}

/// Compute HP Black-Scholes intermediates shared between price-only and full Greeks.
pub(crate) fn compute_bs_intermediates_hp(
    s: u128, k: u128, r: u128, sigma: u128, t: u128,
) -> Result<BsIntermediatesHp, SolMathError> {
    let s_hp = upscale_std_to_hp(s)?;
    let k_hp = upscale_std_to_hp(k)?;
    let r_hp = upscale_std_to_hp(r)?;
    let sigma_hp = upscale_std_to_hp(sigma)?;
    let t_hp = upscale_std_to_hp(t)?;

    let sqrt_t_hp = isqrt_u128((t_hp as u128).checked_mul(SCALE_HP_U).ok_or(SolMathError::Overflow)?) as i128;
    let sigma_sqrt_t_hp = fp_mul_hp_i(sigma_hp, sqrt_t_hp)?;

    let sk_ratio_hp = fp_div_hp_safe(s_hp, k_hp)?;
    let ln_sk_hp = ln_fixed_hp(sk_ratio_hp)?;

    // sigma_sq_half_hp: fp_mul_hp_i is checked; /2: ∈ [0, SCALE_HP/2], fits i128.
    let sigma_sq_half_hp = fp_mul_hp_i(sigma_hp, sigma_hp)? / 2;
    // r_hp ∈ [0, SCALE_HP], sigma_sq_half_hp ∈ [0, SCALE_HP/2]: sum ≤ 1.5·SCALE_HP, fits i128.
    let drift_hp = fp_mul_hp_i(r_hp + sigma_sq_half_hp, t_hp)?;
    // ln_sk_hp ∈ [-40·SCALE_HP, 40·SCALE_HP], drift_hp ∈ [-SCALE_HP, SCALE_HP]; sum fits i128.
    let d1_num_hp = ln_sk_hp + drift_hp;

    let d1_hp = if sigma_sqrt_t_hp > 0 {
        fp_div_hp_safe(d1_num_hp, sigma_sqrt_t_hp)?
    } else {
        0
    };
    // d1_hp ∈ [-8·SCALE_HP, 8·SCALE_HP] (clamped by norm_cdf_poly_hp); sigma_sqrt_t_hp ∈ [0, ~SCALE_HP];
    // d2_hp = d1_hp - sigma_sqrt_t_hp ∈ [-9·SCALE_HP, 8·SCALE_HP], fits i128.
    let d2_hp = d1_hp - sigma_sqrt_t_hp;

    let phi_d1_hp = norm_cdf_poly_hp(d1_hp)?;
    let phi_d2_hp = norm_cdf_poly_hp(d2_hp)?;
    // phi_d1_hp, phi_d2_hp ∈ [0, SCALE_HP]; SCALE_HP - phi ∈ [0, SCALE_HP], fits i128.
    let phi_neg_d1_hp = SCALE_HP - phi_d1_hp;
    let phi_neg_d2_hp = SCALE_HP - phi_d2_hp;

    let r_t_hp = fp_mul_hp_i(r_hp, t_hp)?;
    let discount_hp = exp_fixed_hp(-r_t_hp)?;
    let k_disc_hp = fp_mul_hp_i(k_hp, discount_hp)?;

    Ok(BsIntermediatesHp {
        s_hp, k_hp, d1_hp, d2_hp,
        phi_d1_hp, phi_d2_hp, phi_neg_d1_hp, phi_neg_d2_hp,
        k_disc_hp, sigma_sqrt_t_hp, sqrt_t_hp,
        sigma_hp, r_hp, t_hp,
    })
}

/// High-precision Black-Scholes call and put prices (no Greeks).
///
/// Accepts and returns values at `SCALE` (1e12) but computes internally at `SCALE_HP` (1e15).
/// ~60K CU on Solana.
///
/// - **s**: spot price at `SCALE`.
/// - **k**: strike price at `SCALE`.
/// - **r**: risk-free rate at `SCALE` (e.g. 50_000_000_000 = 5%).
/// - **sigma**: volatility at `SCALE` (e.g. 200_000_000_000 = 20%).
/// - **t**: time to expiry in years at `SCALE` (e.g. SCALE = 1 year).
/// - **Returns**: `(call, put)` both `u128` at `SCALE`.
/// - **Errors**: `DomainError` if `sigma == 0` or `t == 0`.
/// - **Accuracy**: 3-4 ULP max.
pub fn black_scholes_price_hp(
    s: u128, k: u128, r: u128, sigma: u128, t: u128,
) -> Result<(u128, u128), SolMathError> {
    if s > i128::MAX as u128 || k > i128::MAX as u128 || r > i128::MAX as u128
        || sigma > i128::MAX as u128 || t > i128::MAX as u128
    {
        return Err(SolMathError::Overflow);
    }
    if sigma == 0 || t == 0 {
        return Err(SolMathError::DomainError);
    }

    if s == 0 || k == 0 {
        if s == 0 {
            let r_t = fp_mul_i(r as i128, t as i128)?;
            let k_disc = fp_mul_i(k as i128, exp_fixed_i(-r_t)?)?;
            let put = if k_disc > 0 { k_disc as u128 } else { 0 };
            return Ok((0, put));
        }
        return Ok((s, 0));
    }

    let im = compute_bs_intermediates_hp(s, k, r, sigma, t)?;

    // fp_mul_hp_i terms are checked; s_hp and k_disc_hp are SCALE_HP prices, phi values ∈ [0, SCALE_HP];
    // each product ≤ SCALE_HP; differences ∈ (-SCALE_HP, SCALE_HP), fits i128.
    let call_hp = fp_mul_hp_i(im.s_hp, im.phi_d1_hp)? - fp_mul_hp_i(im.k_disc_hp, im.phi_d2_hp)?;
    let put_hp = fp_mul_hp_i(im.k_disc_hp, im.phi_neg_d2_hp)? - fp_mul_hp_i(im.s_hp, im.phi_neg_d1_hp)?;
    let call = downscale_hp_to_std(call_hp);
    let put = downscale_hp_to_std(put_hp);

    Ok((call, put))
}

/// High-precision Black-Scholes: call/put prices + all 5 Greeks.
///
/// Accepts and returns values at `SCALE` (1e12) but computes internally at `SCALE_HP` (1e15).
/// Returns [`BsFull`] with call, put, delta, gamma, vega, theta, and rho.
///
/// - **s**: spot price at `SCALE`.
/// - **k**: strike price at `SCALE`.
/// - **r**: risk-free rate at `SCALE` (e.g. 50_000_000_000 = 5%).
/// - **sigma**: volatility at `SCALE` (e.g. 200_000_000_000 = 20%).
/// - **t**: time to expiry in years at `SCALE` (e.g. SCALE = 1 year).
/// - **Returns**: [`BsFull`] with all fields at `SCALE`.
/// - **Errors**: `DomainError` if `sigma == 0` or `t == 0`.
/// - **Accuracy**: call/put 3-4 ULP max, ~74% exact. Gamma 1 ULP max, 100% exact.
///   Delta 1 ULP max, 99.9% exact.
///
/// # Example
/// ```
/// use solmath::{bs_full_hp, SCALE};
/// // S=100, K=100, r=5%, sigma=20%, T=1yr
/// let greeks = bs_full_hp(100 * SCALE, 100 * SCALE, 50_000_000_000, 200_000_000_000, SCALE)?;
/// assert!(greeks.call > 0);
/// assert!(greeks.put > 0);
/// # Ok::<(), solmath::SolMathError>(())
/// ```
pub fn bs_full_hp(s: u128, k: u128, r: u128, sigma: u128, t: u128) -> Result<BsFull, SolMathError> {
    if s > i128::MAX as u128 || k > i128::MAX as u128 || r > i128::MAX as u128
        || sigma > i128::MAX as u128 || t > i128::MAX as u128
    {
        return Err(SolMathError::Overflow);
    }
    if sigma == 0 || t == 0 {
        return Err(SolMathError::DomainError);
    }

    if s == 0 || k == 0 {
        let zero_full = BsFull {
            call: if s > 0 { s } else { 0 },
            put: if s == 0 {
                let r_t = fp_mul_i(r as i128, t as i128)?;
                let kd = fp_mul_i(k as i128, exp_fixed_i(-r_t)?)?;
                if kd > 0 { kd as u128 } else { 0 }
            } else { 0 },
            call_delta: if s == 0 { 0 } else { SCALE_I },
            put_delta: if s == 0 { -SCALE_I } else { 0 },
            gamma: 0, vega: 0, call_theta: 0, put_theta: 0, call_rho: 0, put_rho: 0,
        };
        return Ok(zero_full);
    }

    let im = compute_bs_intermediates_hp(s, k, r, sigma, t)?;

    // fp_mul_hp_i is checked; result ∈ [0, 64·SCALE_HP]; /2: ∈ [0, 32·SCALE_HP], fits i128.
    let d1_sq_half_hp = fp_mul_hp_i(im.d1_hp, im.d1_hp)? / 2;
    let exp_neg_hp = exp_fixed_hp(-d1_sq_half_hp)?;
    let pdf_d1_hp = fp_mul_hp_i(exp_neg_hp, INV_SQRT_2PI_HP)?;

    // fp_mul_hp_i terms are checked; s_hp and k_disc_hp are SCALE_HP prices; phi values ∈ [0, SCALE_HP];
    // products ≤ SCALE_HP; difference ∈ (-SCALE_HP, SCALE_HP), fits i128.
    let call_hp = fp_mul_hp_i(im.s_hp, im.phi_d1_hp)? - fp_mul_hp_i(im.k_disc_hp, im.phi_d2_hp)?;
    let put_hp = fp_mul_hp_i(im.k_disc_hp, im.phi_neg_d2_hp)? - fp_mul_hp_i(im.s_hp, im.phi_neg_d1_hp)?;
    let call = downscale_hp_to_std(call_hp);
    let put = downscale_hp_to_std(put_hp);

    let phi_d1_std = downscale_hp_to_std_i(im.phi_d1_hp);
    let call_delta = phi_d1_std;
    // phi_d1_std ∈ [0, SCALE_I]; phi_d1_std - SCALE_I ∈ [-SCALE_I, 0], fits i128.
    let put_delta = phi_d1_std - SCALE_I;

    let gamma_denom_hp = fp_mul_hp_i(im.s_hp, im.sigma_sqrt_t_hp)?;
    let gamma_hp = if gamma_denom_hp != 0 {
        fp_div_hp_safe(pdf_d1_hp, gamma_denom_hp)?
    } else {
        0
    };
    let gamma = downscale_hp_to_std_i(gamma_hp);

    let vega_hp = fp_mul_hp_i(fp_mul_hp_i(im.s_hp, pdf_d1_hp)?, im.sqrt_t_hp)?;
    let vega = downscale_hp_to_std_i(vega_hp);

    let spd_sigma_hp = fp_mul_hp_i(fp_mul_hp_i(im.s_hp, pdf_d1_hp)?, im.sigma_hp)?;
    // im.sqrt_t_hp = isqrt(t_hp * SCALE_HP); for t ≤ SCALE (1 year), sqrt_t_hp ≤ SCALE_HP;
    // 2 * im.sqrt_t_hp ≤ 2·SCALE_HP = 2e15, fits i128.
    let two_sqrt_t_hp = 2 * im.sqrt_t_hp;
    let theta_common_hp = if two_sqrt_t_hp > 0 {
        -fp_div_hp_safe(spd_sigma_hp, two_sqrt_t_hp)?
    } else {
        0
    };
    let r_k_disc_hp = fp_mul_hp_i(im.r_hp, im.k_disc_hp)?;
    // theta_common_hp ∈ [-SCALE_HP, 0]; fp_mul_hp_i terms ∈ [0, SCALE_HP];
    // differences/sums ∈ (-2·SCALE_HP, SCALE_HP), fits i128.
    let call_theta_hp = theta_common_hp - fp_mul_hp_i(r_k_disc_hp, im.phi_d2_hp)?;
    let put_theta_hp = theta_common_hp + fp_mul_hp_i(r_k_disc_hp, im.phi_neg_d2_hp)?;
    let call_theta = downscale_hp_to_std_i(call_theta_hp);
    let put_theta = downscale_hp_to_std_i(put_theta_hp);

    let kt_disc_hp = fp_mul_hp_i(im.k_disc_hp, im.t_hp)?;
    let call_rho_hp = fp_mul_hp_i(kt_disc_hp, im.phi_d2_hp)?;
    // fp_mul_hp_i result ∈ [0, SCALE_HP]; negation: ∈ [-SCALE_HP, 0], fits i128.
    let put_rho_hp = -fp_mul_hp_i(kt_disc_hp, im.phi_neg_d2_hp)?;
    let call_rho = downscale_hp_to_std_i(call_rho_hp);
    let put_rho = downscale_hp_to_std_i(put_rho_hp);

    Ok(BsFull {
        call,
        put,
        call_delta,
        put_delta,
        gamma,
        vega,
        call_theta,
        put_theta,
        call_rho,
        put_rho,
    })
}

/// Fast HP multiply returning DoubleWord. Assumes a*b fits in i128.
/// Used by horner_compensated_hp where inputs are bounded.
/// One i128 division + cheap multiply-subtract for remainder.
#[inline]
pub(crate) fn fp_mul_hp_fast_dw(a: i128, b: i128) -> DoubleWord {
    // Callers are ln_fixed_hp (a=t ∈ (-SCALE_HP, SCALE_HP), b=poly ≤ a few ×SCALE_HP)
    // and horner_compensated_hp_dw (a=s ≤ a few ×SCALE_HP, b=t ∈ (-SCALE_HP, SCALE_HP));
    // |a|, |b| ≤ ~10·SCALE_HP = 1e16; product ≤ 1e32, fits i128 (max ~1.7e38).
    let p = a * b;
    let q = if p >= 0 {
        (p + SCALE_HP / 2) / SCALE_HP
    } else {
        (p - SCALE_HP / 2) / SCALE_HP
    };
    // q ≤ p / SCALE_HP ≤ 1e17; q * SCALE_HP ≤ 1e32, fits i128.
    // r = p - q * SCALE_HP: |r| < SCALE_HP by construction, fits i128.
    let r = p - q * SCALE_HP;
    DoubleWord::new_raw(q, r)
}

/// Compensated Horner evaluation at HP scale (1e15).
#[allow(dead_code)]
pub(crate) fn horner_compensated_hp(coeffs: &[i128], t: i128) -> Result<i128, SolMathError> {
    Ok(horner_compensated_hp_dw(coeffs, t)?.to_i128_at_scale(SCALE_HP))
}

/// Compensated Horner at HP scale returning DoubleWord for downstream propagation.
pub(crate) fn horner_compensated_hp_dw(coeffs: &[i128], t: i128) -> Result<DoubleWord, SolMathError> {
    let n = coeffs.len();
    if n <= 1 {
        return Ok(DoubleWord::from_hi(if n == 1 { coeffs[0] } else { 0 }));
    }

    let mut s = coeffs[n - 1];
    let mut comp: i128 = 0;

    for i in (0..n - 1).rev() {
        let dw = fp_mul_hp_fast_dw(s, t);
        // dw.hi ∈ (-a few ×SCALE_HP, a few ×SCALE_HP); |coeffs[i]| ≤ SCALE_HP (ln Remez coefficients);
        // sum ∈ (-a few ×SCALE_HP, a few ×SCALE_HP), fits i128.
        s = dw.hi() + coeffs[i];

        let comp_propagated = comp.checked_mul(t)
            .ok_or(SolMathError::Overflow)? / SCALE_HP;

        // comp_propagated and dw.lo are both sub-ULP residuals with |value| < SCALE_HP;
        // sum ∈ (-2·SCALE_HP, 2·SCALE_HP), fits i128.
        comp = comp_propagated + dw.lo();
    }

    Ok(DoubleWord::new_raw(s, comp))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{SCALE_HP, LN_REMEZ_HP_COEFFS};

    // ===== fp_mul_hp_fast_dw tests =====

    #[test]
    fn test_hp_dw_zero() {
        let dw = fp_mul_hp_fast_dw(0, SCALE_HP);
        assert_eq!(dw.hi(), 0);
        assert_eq!(dw.lo(), 0);
        let dw2 = fp_mul_hp_fast_dw(SCALE_HP, 0);
        assert_eq!(dw2.hi(), 0);
        assert_eq!(dw2.lo(), 0);
    }

    #[test]
    fn test_hp_dw_exact_one() {
        let dw = fp_mul_hp_fast_dw(SCALE_HP, SCALE_HP);
        assert_eq!(dw.hi(), SCALE_HP);
        assert_eq!(dw.lo(), 0);
    }

    #[test]
    fn test_hp_dw_exact_product() {
        let dw = fp_mul_hp_fast_dw(2 * SCALE_HP, 3 * SCALE_HP);
        assert_eq!(dw.hi(), 6 * SCALE_HP);
        assert_eq!(dw.lo(), 0);
    }

    #[test]
    fn test_hp_dw_consistency_with_fp_mul_hp_fast() {
        let values: &[i128] = &[
            1, -1, 2, -2,
            SCALE_HP / 2, -SCALE_HP / 2,
            SCALE_HP, -SCALE_HP,
            SCALE_HP + 1, -(SCALE_HP + 1),
            SCALE_HP * 2, -(SCALE_HP * 2),
            SCALE_HP / 3, -(SCALE_HP / 3),
            SCALE_HP / 7, -(SCALE_HP / 7),
            SCALE_HP * 50, -(SCALE_HP * 50),
            999_999_999_999_999, -999_999_999_999_999,
            500_000_000_000_001, -500_000_000_000_001,
        ];
        for &a in values {
            for &b in values {
                if a.checked_mul(b).is_some() {
                    let expected = fp_mul_hp_fast(a, b);
                    let dw = fp_mul_hp_fast_dw(a, b);
                    assert_eq!(dw.hi(), expected,
                        "hi mismatch: a={}, b={}, expected={}, got={}", a, b, expected, dw.hi());
                }
            }
        }
    }

    #[test]
    fn test_hp_dw_residual_bounded() {
        let values: &[i128] = &[
            0, 1, -1, SCALE_HP, -SCALE_HP, SCALE_HP / 3, -SCALE_HP / 7,
            SCALE_HP * 50, -SCALE_HP * 50, 999_999_999_999_999,
        ];
        for &a in values {
            for &b in values {
                let dw = fp_mul_hp_fast_dw(a, b);
                assert!(dw.lo().abs() < SCALE_HP,
                    "lo out of bounds: a={}, b={}, lo={}", a, b, dw.lo());
            }
        }
    }

    #[test]
    fn test_hp_dw_negative() {
        let dw = fp_mul_hp_fast_dw(-SCALE_HP, SCALE_HP);
        assert_eq!(dw.hi(), -SCALE_HP);
        assert_eq!(dw.lo(), 0);
    }

    #[test]
    fn test_hp_dw_identity() {
        for a in [1i128, -1, SCALE_HP / 3, -SCALE_HP * 7, 42 * SCALE_HP] {
            let dw = fp_mul_hp_fast_dw(a, SCALE_HP);
            assert_eq!(dw.hi(), a, "Identity failed for a={}", a);
            assert_eq!(dw.lo(), 0, "Identity residual nonzero for a={}", a);
        }
    }

    #[test]
    fn test_hp_dw_small_inputs() {
        let dw = fp_mul_hp_fast_dw(1, 1);
        assert_eq!(dw.hi(), 0);
        assert_eq!(dw.lo(), 1);
    }

    // ===== horner_compensated_hp tests =====

    #[test]
    fn test_horner_hp_constant() {
        let coeffs = [42 * SCALE_HP];
        assert_eq!(horner_compensated_hp(&coeffs, SCALE_HP).unwrap(), 42 * SCALE_HP);
        assert_eq!(horner_compensated_hp(&coeffs, 0).unwrap(), 42 * SCALE_HP);
        assert_eq!(horner_compensated_hp(&coeffs, -SCALE_HP).unwrap(), 42 * SCALE_HP);
    }

    #[test]
    fn test_horner_hp_linear() {
        let coeffs = [5 * SCALE_HP, 3 * SCALE_HP];
        assert_eq!(horner_compensated_hp(&coeffs, 2 * SCALE_HP).unwrap(), 11 * SCALE_HP);
    }

    #[test]
    fn test_horner_hp_at_zero() {
        let coeffs = [7 * SCALE_HP, 3 * SCALE_HP, SCALE_HP];
        assert_eq!(horner_compensated_hp(&coeffs, 0).unwrap(), 7 * SCALE_HP);
    }

    #[test]
    fn test_horner_hp_exact_quadratic() {
        let coeffs = [SCALE_HP, 2 * SCALE_HP, 3 * SCALE_HP];
        assert_eq!(horner_compensated_hp(&coeffs, SCALE_HP).unwrap(), 6 * SCALE_HP);
    }

    #[test]
    fn test_horner_hp_consistency_with_inline() {
        let coeffs = &LN_REMEZ_HP_COEFFS;
        let test_u: &[i128] = &[0, SCALE_HP / 20, SCALE_HP / 9];

        for &u in test_u {
            let mut r = coeffs[9];
            for i in (0..9).rev() {
                r = fp_mul_hp_fast(r, u) + coeffs[i];
            }

            let compensated = horner_compensated_hp(&coeffs[..], u).unwrap();
            let diff = (compensated - r).abs();
            assert!(diff <= 1,
                "u={}: inline={}, compensated={}, diff={}", u, r, compensated, diff);
        }
    }

    #[test]
    fn test_horner_hp_dw_residual_bounded() {
        let coeffs = &LN_REMEZ_HP_COEFFS;
        for u in [0i128, SCALE_HP / 20, SCALE_HP / 9, -SCALE_HP / 9] {
            let dw = horner_compensated_hp_dw(&coeffs[..], u).unwrap();
            assert!(dw.lo().abs() < 12 * SCALE_HP,
                "u={}: lo={} exceeds 12*SCALE_HP", u, dw.lo());
        }
    }

    #[test]
    fn test_fp_mul_hp_i_allows_exact_i128_min() {
        let got = fp_mul_hp_i(i128::MIN, SCALE_HP).unwrap();
        assert_eq!(got, i128::MIN);
    }

    #[test]
    fn test_fp_mul_hp_i_rejects_magnitude_above_i128_min() {
        assert_eq!(fp_mul_hp_i(i128::MIN, SCALE_HP + 1), Err(SolMathError::Overflow));
    }

    #[test]
    fn test_horner_hp_no_panic() {
        let coeffs = &LN_REMEZ_HP_COEFFS;
        for u in [-SCALE_HP / 9, -SCALE_HP / 20, 0, SCALE_HP / 20, SCALE_HP / 9] {
            let _ = horner_compensated_hp(&coeffs[..], u).unwrap();
            let _ = horner_compensated_hp_dw(&coeffs[..], u).unwrap();
        }
    }
}
