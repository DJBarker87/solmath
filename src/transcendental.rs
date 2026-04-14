use crate::constants::*;
use crate::error::SolMathError;
use crate::arithmetic::{fp_mul, fp_mul_i, fp_mul_i_round, fp_div_i, fp_sqrt};
use crate::overflow::checked_mul_div_u;
use crate::hp::pow_fixed_hp;

/// Natural logarithm: ln(x / SCALE) * SCALE.
///
/// 16-entry split-constant lookup + degree-3 Remez polynomial on narrow subinterval.
/// Combined sub-ULP correction (table + LN2 residuals in one rounding).
///
/// - **x**: unsigned fixed-point at `SCALE` (1e12). Must be > 0.
/// - **Returns**: `i128` at `SCALE`. Negative for x < SCALE, zero for x == SCALE.
/// - **Errors**: `DomainError` if `x == 0`.
/// - **Accuracy**: max 3 ULP, median 1 ULP, 44% exact.
///
/// # Example
/// ```
/// use solmath::{ln_fixed_i, SCALE};
/// // ln(2.0) ≈ 0.693147...
/// let result = ln_fixed_i(2 * SCALE)?;
/// assert!((result - 693_147_180_560i128).abs() <= 3);
/// # Ok::<(), solmath::SolMathError>(())
/// ```
pub fn ln_fixed_i(x: u128) -> Result<i128, SolMathError> {
    if x == 0 {
        return Err(SolMathError::DomainError);
    }

    let mut m = x;
    let mut k: i32 = 0;

    // Primary reduction: m in [SCALE, 2*SCALE)
    while m < SCALE {
        m = m.checked_mul(2).ok_or(SolMathError::Overflow)?;
        k -= 1;
    }
    while m >= 2 * SCALE {
        m /= 2;
        k += 1;
    }

    let k_i = k as i128;
    let m_i = m as i128;

    // Near x = 1: direct computation to avoid cancellation.
    let offset = m - SCALE;
    if offset < LN_TABLE_HALF_STEP {
        let t_num = m_i - SCALE_I;
        let t_den = m_i + SCALE_I;
        // t_num ∈ (-SCALE_I, SCALE_I); t_num * SCALE_I < 1e24 ≪ i128::MAX (1.7e38).
        let t = (t_num * SCALE_I + t_den / 2) / t_den;
        let u = fp_mul_i_round(t, t)?;
        // Horner additions: each fp_mul_i_round result ∈ (-SCALE_I, SCALE_I);
        // LN_REMEZ_W* < SCALE_I, so each partial sum ∈ (-2·SCALE_I, 2·SCALE_I). Fits i128.
        let p = fp_mul_i_round(LN_REMEZ_W3, u)? + LN_REMEZ_W2;
        let p = fp_mul_i_round(p, u)? + LN_REMEZ_W1;
        let p = fp_mul_i_round(p, u)? + LN_REMEZ_W0;
        // 2 * t: t ∈ (-SCALE_I, SCALE_I), so 2*t ∈ (-2e12, 2e12). Fits i128.
        let series_result = fp_mul_i_round(2 * t, p)?;
        // Direct path: only LN2 correction (no table residual).
        // |k_i| ≤ 127, LN2_LO ≈ 5.5e10; product < 7e12 ≪ i128::MAX.
        let ln2_raw = k_i * LN2_LO;
        let ln2_correction = if ln2_raw >= 0 {
            (ln2_raw + SCALE_I / 2) / SCALE_I
        } else {
            (ln2_raw - SCALE_I / 2) / SCALE_I
        };
        // series_result ≤ SCALE_I, k_i * LN2_I ≤ 127 * 6.9e11 ≈ 8.8e13, ln2_correction < 1;
        // total sum ≪ i128::MAX.
        return Ok(series_result + k_i * LN2_I + ln2_correction);
    }

    // Table lookup.
    let j = (offset / LN_TABLE_STEP) as usize;
    let j = j.min(15);

    let m_j = SCALE + (2 * j as u128 + 1) * LN_TABLE_HALF_STEP;
    let ln_m_j = LN_TABLE_16[j];
    let ln_m_j_lo = LN_TABLE_LO_16[j];

    let m_j_i = m_j as i128;
    let t_num = m_i - m_j_i;
    let t_den = m_i + m_j_i;
    // t_num ∈ (-SCALE_I/16, SCALE_I/16) after table reduction; t_num * SCALE_I < 6.25e10 * 1e12 ≪ i128::MAX.
    let p_val = t_num * SCALE_I;
    let t = (p_val + t_den / 2) / t_den;
    // t's sub-ULP residual via multiply-subtract (no second division).
    // t_rem = p_val - t * t_den: t ≤ SCALE_I/16, t_den ≤ 4*SCALE_I; product ≤ 2.5e23 ≪ i128::MAX.
    let t_rem = p_val - t * t_den;  // exact remainder
    // t_rem < t_den ≤ 4*SCALE_I, so t_rem * SCALE_I ≤ 4e24 ≪ i128::MAX.
    let t_lo = t_rem * SCALE_I / t_den;  // scaled to sub-ULP units

    let u = fp_mul_i_round(t, t)?;

    // Horner additions: same bounds as direct path — each partial sum ∈ (-2·SCALE_I, 2·SCALE_I).
    let p = fp_mul_i_round(LN_REMEZ_W3, u)? + LN_REMEZ_W2;
    let p = fp_mul_i_round(p, u)? + LN_REMEZ_W1;
    let p = fp_mul_i_round(p, u)? + LN_REMEZ_W0;

    // 2 * t: t ∈ (-SCALE_I/16, SCALE_I/16) here; 2*t < 1.25e11. Fits i128.
    let series_result = fp_mul_i_round(2 * t, p)?;

    // COMBINED sub-ULP correction: table residual + LN2 residual + t residual.
    // |k_i| ≤ 127, LN2_LO ≈ 5.5e10; k_i * LN2_LO < 7e12. Each of the three terms < 1e12;
    // combined sum < 3e12 ≪ i128::MAX.
    let combined_lo = ln_m_j_lo + k_i * LN2_LO + t_lo;
    let correction = if combined_lo >= 0 {
        (combined_lo + SCALE_I / 2) / SCALE_I
    } else {
        (combined_lo - SCALE_I / 2) / SCALE_I
    };

    // series_result ≤ SCALE_I, ln_m_j ≤ LN_TABLE_16 max ≈ 0.7*SCALE_I, k_i * LN2_I ≤ 8.8e13,
    // correction ≈ 0 to 1; total sum ≪ i128::MAX.
    Ok(series_result + ln_m_j + k_i * LN2_I + correction)
}

/// Exponential: e^(x / SCALE) * SCALE.
///
/// Remez rational approximation with LN2 residual correction.
///
/// - **x**: signed fixed-point at `SCALE` (1e12).
/// - **Returns**: `i128` at `SCALE`. Always positive for valid inputs.
/// - **Errors**: `Overflow` if `x >= 40 * SCALE`. Returns `Ok(0)` for `x <= -40 * SCALE`.
/// - **Accuracy**: max 1 ULP.
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

    if x <= -max_x { return Ok(0); }
    if x >= max_x { return Err(SolMathError::Overflow); }
    if x == 0 { return Ok(SCALE_I); }

    // Range reduction with split LN2 correction.
    // LN2_I overshoots true ln(2)×SCALE (LN2_LO < 0), so k*LN2_I is too large
    // and r = x - k*LN2_I is too small. Subtract the negative correction to add back.
    let mut k = x / LN2_I;
    let ln2_correction = {
        // |k| ≤ 57 (x < 40*SCALE_I, LN2_I ≈ 6.9e11), LN2_LO ≈ 5.5e10; product < 3.1e12 ≪ i128::MAX.
        let raw = k * LN2_LO;
        if raw >= 0 { (raw + SCALE_I / 2) / SCALE_I } else { (raw - SCALE_I / 2) / SCALE_I }
    };
    // x < 40*SCALE_I ≈ 4e13; k * LN2_I ≤ 57 * 6.9e11 ≈ 3.9e13; r ∈ [-LN2/2, LN2/2] ≈ ±3.5e11.
    let mut r = x - k * LN2_I - ln2_correction;

    let half_ln2 = LN2_I / 2;
    if r > half_ln2 {
        k += 1;
        // r ∈ (LN2/2, LN2_I]; r - LN2_I ∈ (-LN2/2, 0]. Stays in [-LN2/2, LN2/2].
        r -= LN2_I;
    } else if r < -half_ln2 {
        k -= 1;
        // r ∈ [-LN2_I, -LN2/2); r + LN2_I ∈ (0, LN2/2]. Stays in [-LN2/2, LN2/2].
        r += LN2_I;
    }

    // Remez rational formula
    let xx = fp_mul_i_round(r, r)?;

    // Horner: poly = P1 + xx*(P2 + xx*(P3 + xx*(P4 + xx*P5)))
    // EXP_REMEZ_P* < SCALE_I; each partial sum ∈ (-2·SCALE_I, 2·SCALE_I). Fits i128.
    let poly = fp_mul_i_round(xx, EXP_REMEZ_P5)? + EXP_REMEZ_P4;
    let poly = fp_mul_i_round(xx, poly)? + EXP_REMEZ_P3;
    let poly = fp_mul_i_round(xx, poly)? + EXP_REMEZ_P2;
    let poly = fp_mul_i_round(xx, poly)? + EXP_REMEZ_P1;

    // c = r - poly * r^2
    // r ∈ [-LN2/2, LN2/2] ≈ ±3.5e11; fp_mul_i_round(poly, xx) ≤ SCALE_I; c ∈ [-2·SCALE_I, 2·SCALE_I].
    let c = r - fp_mul_i_round(poly, xx)?;

    // exp(r) = 1 + r + r*c/(2-c)
    // SCALE_I + r: r ≤ LN2/2 ≈ 3.5e11, SCALE_I = 1e12; sum < 1.35e12.
    // 2 * SCALE_I - c: c ≤ 2·SCALE_I; denominator ∈ (0, 4·SCALE_I). Cannot underflow (c < 2*SCALE_I for valid r).
    let rc = fp_mul_i_round(r, c)?;
    let sum = SCALE_I + r + fp_div_i(rc, 2 * SCALE_I - c)?;

    // Multiply by 2^k
    if k >= 0 {
        sum.checked_shl(k as u32).ok_or(SolMathError::Overflow)
    } else {
        Ok(sum >> (-k) as u32)
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

    // General case: exp(exponent * ln(base))
    let ln_base = ln_fixed_i(base)?; // i128
    let exp_i = exponent as i128;
    let product = fp_mul_i(exp_i, ln_base)?; // exponent * ln(base)
    let result = exp_fixed_i(product)?;
    Ok(if result <= 0 {
        0
    } else {
        result as u128
    })
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
        2 => fp_mul(base, base),
        3 => Ok(fp_mul(fp_mul(base, base)?, base)?),
        4 => { let x2 = fp_mul(base, base)?; fp_mul(x2, x2) },
        _ => {
            // Check if n * ln(base) fits in HP exp's working range
            let ln_base = ln_fixed_i(base)?;
            let total = match (n as i128).checked_mul(ln_base) {
                Some(v) => v,
                None => if ln_base > 0 { return Err(SolMathError::Overflow) } else { return Ok(0) },
            };
            if total.abs() < 39 * SCALE_I {
                pow_fixed_hp(base, n * SCALE)
            } else {
                // Split: base^n = (base^(n/2))² × base^(n%2)
                let half = pow_int(base, n / 2)?;
                let mut result = checked_mul_div_u(half, half, SCALE)
                    .ok_or(SolMathError::Overflow)?;
                if n % 2 == 1 {
                    result = checked_mul_div_u(result, base, SCALE)
                        .ok_or(SolMathError::Overflow)?;
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
        let pos_result = pow_fixed_i(base, -exponent)?;
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
/// Uses degree-11 Taylor on [-0.5, 0.5] to avoid catastrophic cancellation,
/// falls back to `exp_fixed_i(x) - SCALE` outside that range.
///
/// - **x**: signed fixed-point at `SCALE` (1e12).
/// - **Returns**: `i128` at `SCALE`. Near zero for small x.
/// - **Errors**: `Overflow` if `exp_fixed_i` overflows (|x| > 0.5 and x >= 40*SCALE).
/// - **Accuracy**: max 3 ULP.
pub fn expm1_fixed(x: i128) -> Result<i128, SolMathError> {
    let half = SCALE_I / 2;
    if x > half || x < -half {
        // exp_fixed_i(x) ∈ (0, e^40 * SCALE_I); subtracting SCALE_I is safe — no overflow.
        return Ok(exp_fixed_i(x)? - SCALE_I);
    }
    if x == 0 { return Ok(0); }

    const C11: i128 = 25_052;
    const C10: i128 = 275_573;
    const C9: i128 = 2_755_732;
    const C8: i128 = 24_801_587;
    const C7: i128 = 198_412_698;
    const C6: i128 = 1_388_888_889;
    const C5: i128 = 8_333_333_333;
    const C4: i128 = 41_666_666_667;
    const C3: i128 = 166_666_666_667;
    const C2: i128 = 500_000_000_000;
    const C1: i128 = SCALE_I;

    // x ∈ [-0.5·SCALE_I, 0.5·SCALE_I]. Horner accumulation: fp_mul_i_round result ≤ SCALE_I/2;
    // each Cn ≤ SCALE_I, so every partial sum ∈ (-2·SCALE_I, 2·SCALE_I). Fits i128.
    let p = fp_mul_i_round(x, C11)? + C10;
    let p = fp_mul_i_round(x, p)? + C9;
    let p = fp_mul_i_round(x, p)? + C8;
    let p = fp_mul_i_round(x, p)? + C7;
    let p = fp_mul_i_round(x, p)? + C6;
    let p = fp_mul_i_round(x, p)? + C5;
    let p = fp_mul_i_round(x, p)? + C4;
    let p = fp_mul_i_round(x, p)? + C3;
    let p = fp_mul_i_round(x, p)? + C2;
    let p = fp_mul_i_round(x, p)? + C1;
    Ok(fp_mul_i_round(x, p)?)
}
