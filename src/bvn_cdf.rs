use crate::arithmetic::{fp_div_i, fp_mul_i};
use crate::constants::PI_OVER_2_SCALE;
use crate::error::SolMathError;
use crate::normal::norm_cdf_poly;
use crate::transcendental::exp_fixed_i;
use crate::trig::sincos_fixed;
use crate::SCALE_I;

// ── GL6 Gauss-Legendre nodes/weights (6-point, on [-1,1]) ──
// Drezner-Wesolowsky: < 10⁻⁴ accuracy for |ρ| < 0.95.
const GL6_NODES: [i128; 6] = [
    -932_469_514_203, // -0.932469514203152
    -661_209_386_466, // -0.661209386466265
    -238_619_186_083, // -0.238619186083197
    238_619_186_083,
    661_209_386_466,
    932_469_514_203,
];
const GL6_WEIGHTS: [i128; 6] = [
    171_324_492_379, // 0.171324492379170
    360_761_573_048, // 0.360761573048139
    467_913_934_573, // 0.467913934572691
    467_913_934_573,
    360_761_573_048,
    171_324_492_379,
];

// ── GL20 kept for offline table generation ──
const GL20_NODES: [i128; 20] = [
    -993_128_599_185,
    -963_971_927_278,
    -912_234_428_251,
    -839_116_971_822,
    -746_331_906_460,
    -636_053_680_727,
    -510_867_001_951,
    -373_706_088_715,
    -227_785_851_142,
    -76_526_521_133,
    76_526_521_133,
    227_785_851_142,
    373_706_088_715,
    510_867_001_951,
    636_053_680_727,
    746_331_906_460,
    839_116_971_822,
    912_234_428_251,
    963_971_927_278,
    993_128_599_185,
];
const GL20_WEIGHTS: [i128; 20] = [
    17_614_007_139,
    40_601_429_800,
    62_672_048_334,
    83_276_741_577,
    101_930_119_817,
    118_194_531_962,
    131_688_638_449,
    142_096_109_318,
    149_172_986_473,
    152_753_387_131,
    152_753_387_131,
    149_172_986_473,
    142_096_109_318,
    131_688_638_449,
    118_194_531_962,
    101_930_119_817,
    83_276_741_577,
    62_672_048_334,
    40_601_429_800,
    17_614_007_139,
];

const INV_TWO_PI: i128 = 159_154_943_092;
const RHO_NEAR_ONE: i128 = SCALE_I - 1_000_000; // 1 - 1e-6

#[inline]
fn clamp_prob(value: i128) -> i128 {
    value.clamp(0, SCALE_I)
}

fn asin_fixed(x: i128) -> Result<i128, SolMathError> {
    if x.abs() > SCALE_I {
        return Err(SolMathError::DomainError);
    }
    if x == SCALE_I {
        return Ok(PI_OVER_2_SCALE);
    }
    if x == -SCALE_I {
        return Ok(-PI_OVER_2_SCALE);
    }

    let x2 = fp_mul_i(x, x)?;
    let x3 = fp_mul_i(x2, x)?;
    let x5 = fp_mul_i(x3, x2)?;
    let x7 = fp_mul_i(x5, x2)?;

    let mut theta = x
        .checked_add(x3 / 6)
        .ok_or(SolMathError::Overflow)?
        .checked_add((3 * x5) / 40)
        .ok_or(SolMathError::Overflow)?
        .checked_add((5 * x7) / 112)
        .ok_or(SolMathError::Overflow)?;

    for _ in 0..5 {
        let (sin_theta, cos_theta) = sincos_fixed(theta)?;
        if cos_theta == 0 {
            return Ok(theta.clamp(-PI_OVER_2_SCALE, PI_OVER_2_SCALE));
        }
        let error = sin_theta.checked_sub(x).ok_or(SolMathError::Overflow)?;
        if error.abs() <= 4 {
            return Ok(theta.clamp(-PI_OVER_2_SCALE, PI_OVER_2_SCALE));
        }
        let step = fp_div_i(error, cos_theta)?;
        theta = theta
            .checked_sub(step)
            .ok_or(SolMathError::Overflow)?
            .clamp(-PI_OVER_2_SCALE, PI_OVER_2_SCALE);
    }

    Ok(theta.clamp(-PI_OVER_2_SCALE, PI_OVER_2_SCALE))
}

/// Core quadrature for x ≤ 0, y ≤ 0.
#[inline]
fn direct_cdf_negative_gl(
    x: i128,
    y: i128,
    rho: i128,
    nodes: &[i128],
    weights: &[i128],
) -> Result<i128, SolMathError> {
    let phi_x = norm_cdf_poly(x)?;
    let phi_y = norm_cdf_poly(y)?;
    let base = fp_mul_i(phi_x, phi_y)?;
    if rho == 0 {
        return Ok(base);
    }

    let alpha = asin_fixed(rho)?;
    let half = alpha / 2;
    let mid = alpha / 2;
    let x_sq = fp_mul_i(x, x)?;
    let y_sq = fp_mul_i(y, y)?;
    let xy = fp_mul_i(x, y)?;
    let mut weighted_sum = 0i128;

    for idx in 0..nodes.len() {
        let theta = mid
            .checked_add(fp_mul_i(half, nodes[idx])?)
            .ok_or(SolMathError::Overflow)?;
        let (sin_theta, cos_theta) = sincos_fixed(theta)?;
        let cos_sq = fp_mul_i(cos_theta, cos_theta)?;
        if cos_sq <= 0 {
            return Err(SolMathError::DomainError);
        }
        let cross = 2_i128
            .checked_mul(fp_mul_i(xy, sin_theta)?)
            .ok_or(SolMathError::Overflow)?;
        let numerator = x_sq
            .checked_sub(cross)
            .ok_or(SolMathError::Overflow)?
            .checked_add(y_sq)
            .ok_or(SolMathError::Overflow)?;
        let denominator = 2_i128.checked_mul(cos_sq).ok_or(SolMathError::Overflow)?;
        let exponent = -fp_div_i(numerator, denominator)?;
        let exp_term = exp_fixed_i(exponent)?;
        let weighted = fp_mul_i(weights[idx], exp_term)?;
        weighted_sum = weighted_sum
            .checked_add(weighted)
            .ok_or(SolMathError::Overflow)?;
    }

    let integral = fp_mul_i(half, weighted_sum)?;
    let correction = fp_mul_i(INV_TWO_PI, integral)?;
    Ok(clamp_prob(
        base.checked_add(correction).ok_or(SolMathError::Overflow)?,
    ))
}

/// Quadrant-folding dispatch shared by GL6 and GL20 paths.
fn bvn_cdf_with_gl(
    x: i128,
    y: i128,
    rho: i128,
    nodes: &[i128],
    weights: &[i128],
) -> Result<i128, SolMathError> {
    if rho.abs() > SCALE_I {
        return Err(SolMathError::DomainError);
    }
    if rho >= RHO_NEAR_ONE {
        return norm_cdf_poly(x.min(y));
    }
    if rho <= -RHO_NEAR_ONE {
        let value = norm_cdf_poly(x)?
            .checked_add(norm_cdf_poly(y)?)
            .ok_or(SolMathError::Overflow)?
            .checked_sub(SCALE_I)
            .ok_or(SolMathError::Overflow)?;
        return Ok(clamp_prob(value));
    }

    if x > 0 && y > 0 {
        let fx = norm_cdf_poly(x)?;
        let fy = norm_cdf_poly(y)?;
        let tail = bvn_cdf_with_gl(-x, -y, rho, nodes, weights)?;
        let value = fx
            .checked_add(fy)
            .ok_or(SolMathError::Overflow)?
            .checked_sub(SCALE_I)
            .ok_or(SolMathError::Overflow)?
            .checked_add(tail)
            .ok_or(SolMathError::Overflow)?;
        return Ok(clamp_prob(value));
    }
    if x > 0 {
        let fy = norm_cdf_poly(y)?;
        let tail = bvn_cdf_with_gl(-x, y, -rho, nodes, weights)?;
        return Ok(clamp_prob(
            fy.checked_sub(tail).ok_or(SolMathError::Overflow)?,
        ));
    }
    if y > 0 {
        let fx = norm_cdf_poly(x)?;
        let tail = bvn_cdf_with_gl(x, -y, -rho, nodes, weights)?;
        return Ok(clamp_prob(
            fx.checked_sub(tail).ok_or(SolMathError::Overflow)?,
        ));
    }

    direct_cdf_negative_gl(x, y, rho, nodes, weights)
}

// ═══════════════════════════════════════════════════════════════
// Public API
// ═══════════════════════════════════════════════════════════════

/// General bivariate normal CDF. Any `ρ`. ~129K CU median, 153K max.
///
/// Computes `P(X ≤ a, Y ≤ b)` where `(X, Y) ~ N(0, 0, 1, 1, ρ)`.
///
/// All inputs/outputs are signed fixed-point `i128` at `SCALE` (1e12).
/// `rho` must lie in `[-SCALE, SCALE]`. The return value is a probability
/// in `[0, SCALE]`.
///
/// Uses 6-point Gauss-Legendre quadrature (Drezner-Wesolowsky).
/// Accuracy by `|ρ|` range:
/// - `|ρ| ≤ 0.90`: max error < 4×10⁻⁷
/// - `|ρ| ≤ 0.95`: max error < 5×10⁻⁶
/// - `|ρ| ≤ 0.99`: max error < 10⁻⁴
///
/// Near `ρ = ±1` the routine switches to the analytic limit.
///
/// # Errors
///
/// - `DomainError` if `|rho| > SCALE`.
/// - `Overflow` from internal fixed-point operations (extreme inputs).
pub fn bvn_cdf(a: i128, b: i128, rho: i128) -> Result<i128, SolMathError> {
    bvn_cdf_with_gl(a, b, rho, &GL6_NODES, &GL6_WEIGHTS)
}

/// High-precision bivariate normal CDF. Any `ρ`. ~331K CU. Accuracy < 10⁻⁶.
///
/// 20-point Gauss-Legendre. Use offline for table generation and validation.
/// Not recommended on-chain — use [`bvn_cdf`] (GL6) instead.
///
/// All inputs/outputs are signed fixed-point `i128` at `SCALE` (1e12).
///
/// # Errors
///
/// - `DomainError` if `|rho| > SCALE`.
/// - `Overflow` from internal fixed-point operations (extreme inputs).
pub fn bvn_cdf_hp(a: i128, b: i128, rho: i128) -> Result<i128, SolMathError> {
    bvn_cdf_with_gl(a, b, rho, &GL20_NODES, &GL20_WEIGHTS)
}

