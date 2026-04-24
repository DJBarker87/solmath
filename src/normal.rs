use crate::constants::*;
use crate::error::SolMathError;
use crate::arithmetic::{fp_mul_i, fp_mul_i_round, fp_div_i, fp_div_i_round, fp_sqrt};
use crate::transcendental::{exp_fixed_i, ln_fixed_i};

/// Standard normal PDF: phi(x) = (1/sqrt(2*pi)) * exp(-x^2/2) at SCALE.
///
/// - **x**: signed fixed-point at `SCALE` (1e12).
/// - **Returns**: `i128` at `SCALE`, always >= 0. Returns 0 for extreme |x|.
/// - **Errors**: `Overflow` on internal arithmetic overflow (extremely unlikely).
/// - **Accuracy**: max 2 ULP.
pub fn norm_pdf(x: i128) -> Result<i128, SolMathError> {
    let x_sq = fp_mul_i(x, x)?;
    // x_sq ∈ [0, 64·SCALE_I²/SCALE_I] = [0, ~64·SCALE_I] after fp_mul_i; /2 and negate: result ∈ [-32·SCALE_I, 0], fits i128.
    let neg_half_x_sq = -(x_sq / 2);
    // Guard: for extreme |x|, -x²/2 underflows past exp's range → pdf is 0.
    if neg_half_x_sq < -40 * SCALE_I {
        return Ok(0);
    }
    // After guard: neg_half_x_sq ∈ [-40·SCALE, 0] — exp_fixed_i cannot fail.
    let exp_term = match exp_fixed_i(neg_half_x_sq) {
        Ok(v) => v,
        Err(_) => return Ok(0), // unreachable after guard
    };
    fp_mul_i(INV_SQRT_2PI, exp_term)
}

/// Rounding Horner degree-11. Uses fp_mul_i_round instead of fp_mul_i.
#[inline]
pub(crate) fn horner_11_round(c: &[i128; 12], t: i128) -> Result<i128, SolMathError> {
    let mut r = c[11];
    // Each step: fp_mul_i_round result ∈ [-SCALE_I, SCALE_I] (r and t are both ≤ SCALE_I after initial
    // coefficient), coefficients |c[i]| ≤ SCALE_I; sum ∈ [-2·SCALE_I, 2·SCALE_I], fits i128.
    r = fp_mul_i_round(r, t)? + c[10];
    r = fp_mul_i_round(r, t)? + c[9];
    r = fp_mul_i_round(r, t)? + c[8];
    r = fp_mul_i_round(r, t)? + c[7];
    r = fp_mul_i_round(r, t)? + c[6];
    r = fp_mul_i_round(r, t)? + c[5];
    r = fp_mul_i_round(r, t)? + c[4];
    r = fp_mul_i_round(r, t)? + c[3];
    r = fp_mul_i_round(r, t)? + c[2];
    r = fp_mul_i_round(r, t)? + c[1];
    r = fp_mul_i_round(r, t)? + c[0];
    Ok(r)
}

/// Rounding map: (|x| - mid) × SCALE / hw, rounded to nearest.
#[inline]
pub(crate) fn poly_map_t_round(ax: i128, mid: i128, hw: i128) -> Result<i128, SolMathError> {
    // ax ∈ [0, 8·SCALE_I], mid ∈ [0, 5·SCALE_I]: ax - mid ∈ [-5·SCALE_I, 8·SCALE_I], fits i128.
    // checked_mul guards the ×SCALE_I step. hw/2: hw is a small constant ≤ SCALE_I, fits i128.
    let num = (ax - mid).checked_mul(SCALE_I).ok_or(SolMathError::Overflow)?;
    Ok(if num >= 0 { (num + hw / 2) / hw } else { (num - hw / 2) / hw })
}

/// Mills ratio via 6-level continued fraction. For |x| ≥ 5 at SCALE.
/// At x=5, CF6 vs CF8 difference is < 0.001 ULP. Saves ~2K CU.
#[inline]
pub(crate) fn mills_ratio_cf6(x: i128) -> Result<i128, SolMathError> {
    let mut r = 0i128;
    for k in (1..=6).rev() {
        // k ∈ [1, 6], SCALE_I = 1e12: k * SCALE_I ≤ 6e12, fits i128.
        // r is the previous continued-fraction level (bounded by SCALE_I); x ∈ [5·SCALE_I, 8·SCALE_I];
        // x + r ≤ 9·SCALE_I, fits i128.
        r = fp_div_i_round(k * SCALE_I, x + r)?;
    }
    fp_div_i_round(SCALE_I, x + r)
}

/// Tail CDF for |x| >= 5×SCALE via asymptotic expansion.
/// Φ(x) = SCALE − φ(x) × mills_ratio(x). Naturally monotone.
pub(crate) fn norm_cdf_tail(x_abs: i128) -> Result<i128, SolMathError> {
    // φ(x) = exp(-x²/2) / √(2π)
    // x_abs ∈ [5·SCALE_I, 8·SCALE_I]; fp_mul_i_round result ∈ [0, 64·SCALE_I]; /2: ∈ [0, 32·SCALE_I], fits i128.
    let x_sq_half = fp_mul_i_round(x_abs, x_abs)? / 2;
    if x_sq_half > 40 * SCALE_I {
        return Ok(SCALE_I);
    }
    let exp_val = exp_fixed_i(-x_sq_half)?;
    let pdf = fp_mul_i_round(INV_SQRT_2PI, exp_val)?;

    // tail = φ(x) × R(x)
    let mills = mills_ratio_cf6(x_abs)?;
    let tail = fp_mul_i_round(pdf, mills)?;

    // tail ∈ [0, SCALE_I] (pdf and mills both bounded); SCALE_I - tail ∈ [0, SCALE_I], fits i128.
    Ok((SCALE_I - tail).clamp(0, SCALE_I))
}

/// Standard normal CDF: Phi(x) at SCALE.
///
/// 6-piece minimax polynomial + continued-fraction asymptotic tail.
///
/// - **x**: signed fixed-point at `SCALE` (1e12).
/// - **Returns**: `i128` probability in [0, SCALE_I]. Returns 0 for x < -8*SCALE, SCALE_I for x > 8*SCALE.
/// - **Errors**: `Overflow` on internal arithmetic overflow (extremely unlikely).
/// - **Accuracy**: max 4 ULP, 50% exact. Monotone with zero boundary discontinuity.
pub fn norm_cdf_poly(x: i128) -> Result<i128, SolMathError> {
    if x < -8 * SCALE_I {
        return Ok(0);
    }
    if x > 8 * SCALE_I {
        return Ok(SCALE_I);
    }
    if x == 0 {
        return Ok(SCALE_I / 2);
    }

    let ax = x.abs();

    let cdf_pos = if ax <= POLY_V2_I0_HI {
        horner_11_round(&POLY_V2_I0, poly_map_t_round(ax, POLY_V2_I0_MID, POLY_V2_I0_HW)?)?
    } else if ax <= POLY_V2_I1_HI {
        horner_11_round(&POLY_V2_I1, poly_map_t_round(ax, POLY_V2_I1_MID, POLY_V2_I1_HW)?)?
    } else if ax <= POLY_V2_I2_HI {
        horner_11_round(&POLY_V2_I2, poly_map_t_round(ax, POLY_V2_I2_MID, POLY_V2_I2_HW)?)?
    } else if ax <= POLY_V2_I3_HI {
        horner_11_round(&POLY_V2_I3, poly_map_t_round(ax, POLY_V2_I3_MID, POLY_V2_I3_HW)?)?
    } else if ax <= POLY_V2_I4_HI {
        horner_11_round(&POLY_V2_I4, poly_map_t_round(ax, POLY_V2_I4_MID, POLY_V2_I4_HW)?)?
    } else if ax <= 5 * SCALE_I {
        horner_11_round(&POLY_V2_I5, poly_map_t_round(ax, POLY_V2_I5_MID, POLY_V2_I5_HW)?)?
    } else {
        // Asymptotic tail: Φ(x) = SCALE − φ(x) × mills_ratio(x)
        norm_cdf_tail(ax)?
    };

    let cdf_pos = cdf_pos.clamp(0, SCALE_I);

    // cdf_pos ∈ [0, SCALE_I] after clamp; SCALE_I - cdf_pos ∈ [0, SCALE_I], fits i128.
    Ok(if x >= 0 {
        cdf_pos
    } else {
        SCALE_I - cdf_pos
    })
}

/// Combined normal CDF and PDF: returns `(Phi(x), phi(x))` at SCALE.
///
/// - **x**: signed fixed-point at `SCALE` (1e12).
/// - **Returns**: `(i128, i128)` — `(CDF, PDF)` both at `SCALE`. CDF in [0, SCALE_I].
/// - **Errors**: `Overflow` on internal arithmetic overflow.
/// - **Accuracy**: CDF max 4 ULP, PDF max 2 ULP.
pub fn norm_cdf_and_pdf(x: i128) -> Result<(i128, i128), SolMathError> {
    Ok((norm_cdf_poly(x)?, norm_pdf(x)?))
}

/// BS-guarded CDF+PDF: short-circuits to (0,0)/(SCALE,0) beyond ±8σ. Internal.
#[cfg(feature = "bs")]
pub(crate) fn norm_cdf_and_pdf_bs_guarded(x: i128) -> Result<(i128, i128), SolMathError> {
    if x <= -8 * SCALE_I {
        return Ok((0, 0));
    }
    if x >= 8 * SCALE_I {
        return Ok((SCALE_I, 0));
    }
    Ok((norm_cdf_poly(x)?, norm_pdf(x)?))
}

/// Degree-7 Horner evaluation with rounding.
#[inline]
fn horner_7_round(c: &[i128; 8], r: i128) -> Result<i128, SolMathError> {
    let mut acc = c[7];
    // Each step: fp_mul_i_round result ∈ [-SCALE_I, SCALE_I]; |c[i]| ≤ SCALE_I;
    // sum ∈ [-2·SCALE_I, 2·SCALE_I], fits i128.
    acc = fp_mul_i_round(acc, r)? + c[6];
    acc = fp_mul_i_round(acc, r)? + c[5];
    acc = fp_mul_i_round(acc, r)? + c[4];
    acc = fp_mul_i_round(acc, r)? + c[3];
    acc = fp_mul_i_round(acc, r)? + c[2];
    acc = fp_mul_i_round(acc, r)? + c[1];
    acc = fp_mul_i_round(acc, r)? + c[0];
    Ok(acc)
}

/// Degree-7 Horner evaluation for denominator (constant term = SCALE_I).
#[inline]
fn horner_7_den_round(c: &[i128; 7], r: i128) -> Result<i128, SolMathError> {
    let mut acc = c[6];
    // Each step: fp_mul_i_round result ∈ [-SCALE_I, SCALE_I]; |c[i]| ≤ SCALE_I;
    // sum ∈ [-2·SCALE_I, 2·SCALE_I], fits i128.
    acc = fp_mul_i_round(acc, r)? + c[5];
    acc = fp_mul_i_round(acc, r)? + c[4];
    acc = fp_mul_i_round(acc, r)? + c[3];
    acc = fp_mul_i_round(acc, r)? + c[2];
    acc = fp_mul_i_round(acc, r)? + c[1];
    acc = fp_mul_i_round(acc, r)? + c[0];
    // Final term: fp_mul_i_round ∈ [-SCALE_I, SCALE_I]; + SCALE_I: ∈ [0, 2·SCALE_I], fits i128.
    Ok(fp_mul_i_round(acc, r)? + SCALE_I)
}

/// Inverse normal CDF (quantile function): Phi^-1(p/SCALE) * SCALE.
///
/// AS241 algorithm: 3-branch rational polynomial, ~12 significant digits.
///
/// - **p**: signed fixed-point probability at `SCALE` (1e12), in the open interval `(0, SCALE_I)`.
/// - **Returns**: `i128` z-score at `SCALE` such that Phi(z) ~ p/SCALE.
/// - **Errors**: `DomainError` if `p <= 0` or `p >= SCALE_I`.
/// - **Accuracy**: max 6 ULP, P99 5 ULP, median 1 ULP, 25% exact.
pub fn inverse_norm_cdf(p: i128) -> Result<i128, SolMathError> {
    if p <= 0 || p >= SCALE_I {
        return Err(SolMathError::DomainError);
    }

    // p ∈ (0, SCALE_I); SCALE_I/2 is a compile-time constant ~5e11; q ∈ (-SCALE_I/2, SCALE_I/2), fits i128.
    let q = p - SCALE_I / 2;

    if q.abs() <= AS241_SPLIT1 {
        // Branch 1: near center (|q| ≤ 0.425)
        // AS241_CONST1 ≈ 0.18·SCALE_I; fp_mul_i result ∈ [0, 0.18·SCALE_I]; r ∈ [0, 0.18·SCALE_I], fits i128.
        let r = AS241_CONST1 - fp_mul_i(q, q)?;
        let num = horner_7_round(&AS241_A, r)?;
        let den = horner_7_den_round(&AS241_B, r)?;
        fp_div_i(fp_mul_i_round(q, num)?, den)
    } else {
        // Tail branches: use sqrt(-ln(min(u, 1-u)))
        // p ∈ (0, SCALE_I); SCALE_I - p ∈ (0, SCALE_I), fits i128.
        let tail = if q < 0 { p } else { SCALE_I - p };
        let ln_tail = ln_fixed_i(tail as u128)?;
        // tail ∈ (0, SCALE_I/2]; ln_fixed_i returns a negative value (log < 1); negation gives ∈ (0, ~40·SCALE_I], fits i128.
        let neg_ln = -ln_tail; // positive
        let r = fp_sqrt(neg_ln as u128)? as i128;

        let ret = if r < AS241_SPLIT_2B {
            // Branch 2: near tails (r < 3.0), r_adj ∈ [0, ~1.4]
            // r ∈ [0, 3·SCALE_I), AS241_CONST2 ≈ 1.6·SCALE_I; r_adj ∈ [-1.6·SCALE_I, 1.4·SCALE_I], fits i128.
            let r_adj = r - AS241_CONST2;
            let num = horner_7_round(&AS241_C, r_adj)?;
            let den = horner_7_den_round(&AS241_D, r_adj)?;
            fp_div_i(num, den)?
        } else if r < AS241_SPLIT_2C {
            // Branch 2b: r ∈ [3.0, 4.0), centered at 3.5, r_adj ∈ [-0.5, 0.5]
            // r ∈ [3·SCALE_I, 4·SCALE_I), AS241_CENTER_2B = 3.5·SCALE_I; r_adj ∈ [-0.5·SCALE_I, 0.5·SCALE_I], fits i128.
            let r_adj = r - AS241_CENTER_2B;
            let num = horner_7_round(&AS241_G, r_adj)?;
            let den = horner_7_den_round(&AS241_H, r_adj)?;
            fp_div_i(num, den)?
        } else if r < AS241_SPLIT2 {
            // Branch 2c: r ∈ [4.0, 5.0), centered at 4.5, r_adj ∈ [-0.5, 0.5]
            // r ∈ [4·SCALE_I, 5·SCALE_I), AS241_CENTER_2C = 4.5·SCALE_I; r_adj ∈ [-0.5·SCALE_I, 0.5·SCALE_I], fits i128.
            let r_adj = r - AS241_CENTER_2C;
            let num = horner_7_round(&AS241_I, r_adj)?;
            let den = horner_7_den_round(&AS241_J, r_adj)?;
            fp_div_i(num, den)?
        } else {
            // Branch 3: extreme tails (r ≥ 5.0), r_adj ∈ [0, ~0.26]
            // r ∈ [5·SCALE_I, ~8·SCALE_I), AS241_SPLIT2 = 5·SCALE_I; r_adj ∈ [0, ~3·SCALE_I], fits i128.
            let r_adj = r - AS241_SPLIT2;
            let num = horner_7_round(&AS241_E, r_adj)?;
            let den = horner_7_den_round(&AS241_F, r_adj)?;
            fp_div_i(num, den)?
        };

        // ret is a z-score ∈ [0, ~8·SCALE_I]; negation: ∈ (-8·SCALE_I, 0], fits i128.
        if q < 0 { Ok(-ret) } else { Ok(ret) }
    }
}
