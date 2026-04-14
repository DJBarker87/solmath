// European barrier option pricing via Rubinstein-Reiner building blocks.
//
// Uses Haug building blocks A, B, C, D with eta = phi (not barrier direction).
// Verified against QuantLib AnalyticBarrierEngine on 443K vectors.
//
// All arithmetic at HP precision (1e15). Single barriers: ~160K CU.

use crate::constants::*;
use crate::error::SolMathError;
use crate::arithmetic::{fp_mul_i, fp_div_i, isqrt_u128};
use crate::hp::{black_scholes_price_hp, fp_mul_hp_i, fp_div_hp_safe, upscale_std_to_hp, downscale_hp_to_std, ln_fixed_hp, exp_fixed_hp, norm_cdf_poly_hp};

/// Barrier option type (single barrier, European exercise).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarrierType {
    /// Knocked out if spot falls to or below the barrier.
    DownAndOut,
    /// Knocked in (activated) if spot falls to or below the barrier.
    DownAndIn,
    /// Knocked out if spot rises to or above the barrier.
    UpAndOut,
    /// Knocked in (activated) if spot rises to or above the barrier.
    UpAndIn,
}

/// Result of a barrier option pricing computation.
///
/// The returned `price` and `vanilla` are rounded so that paired knock-in/knock-out
/// calls satisfy the exact public identity `in_price + out_price == vanilla`.
#[derive(Debug, Clone, Copy)]
pub struct BarrierResult {
    /// Barrier option price at SCALE.
    pub price: u128,
    /// Vanilla BS price for reference (in + out = vanilla).
    pub vanilla: u128,
}

/// All HP intermediates for barrier pricing.
struct HaugIntermediates {
    s_hp: i128,
    k_disc_hp: i128,
    x1_hp: i128,
    y1_hp: i128,
    d1_hp: i128,
    y_hp: i128,
    sigma_sqrt_t_hp: i128,
    discount_hp: i128,
    pow_2l_hp: i128,
    pow_2lm2_hp: i128,
    phi: i128,
}

/// Compute all HP intermediates for barrier pricing.
/// eta = phi (call/put sign), NOT barrier direction.
#[inline(never)]
fn compute_intermediates(
    s: u128, k: u128, h: u128,
    r: u128, sigma: u128, t: u128,
    is_call: bool,
) -> Result<HaugIntermediates, SolMathError> {
    let s_hp = upscale_std_to_hp(s)?;
    let k_hp = upscale_std_to_hp(k)?;
    let h_hp = upscale_std_to_hp(h)?;
    let r_hp = upscale_std_to_hp(r)?;
    let sigma_hp = upscale_std_to_hp(sigma)?;
    let t_hp = upscale_std_to_hp(t)?;

    let sqrt_t_hp = isqrt_u128((t_hp as u128).checked_mul(SCALE_HP_U).ok_or(SolMathError::Overflow)?) as i128;
    let sigma_sqrt_t_hp = fp_mul_hp_i(sigma_hp, sqrt_t_hp)?;

    let r_t_hp = fp_mul_hp_i(r_hp, t_hp)?;
    let discount_hp = exp_fixed_hp(-r_t_hp)?;
    let k_disc_hp = fp_mul_hp_i(k_hp, discount_hp)?;

    let sigma_sq_hp = fp_mul_hp_i(sigma_hp, sigma_hp)?;
    let drift_hp = fp_mul_hp_i(r_hp + sigma_sq_hp / 2, t_hp)?;
    let lambda_sst = if sigma_sqrt_t_hp > 0 {
        fp_div_hp_safe(drift_hp, sigma_sqrt_t_hp)?
    } else {
        0
    };

    let ln_sk = ln_fixed_hp(fp_div_hp_safe(s_hp, k_hp)?)?;
    let ln_sh = ln_fixed_hp(fp_div_hp_safe(s_hp, h_hp)?)?;
    let ln_hk = ln_fixed_hp(fp_div_hp_safe(h_hp, k_hp)?)?;

    let mk = |log_val: i128| -> Result<i128, SolMathError> {
        if sigma_sqrt_t_hp > 0 {
            // fp_div_hp_safe result ∈ [-~1e15, ~1e15]; lambda_sst ∈ [-~1e15, ~1e15] (finite-rate drift); sum ≤ ~2e15, fits i128
            Ok(fp_div_hp_safe(log_val, sigma_sqrt_t_hp)? + lambda_sst)
        } else {
            Ok(0)
        }
    };

    let d1_hp = mk(ln_sk)?;
    let x1_hp = mk(ln_sh)?;
    let y1_hp = mk(-ln_sh)?;
    // -ln_sh ∈ [-~1e15, ~1e15], ln_hk ∈ [-~1e15, ~1e15]; sum ≤ ~2e15, fits i128
    let y_hp = mk(-ln_sh + ln_hk)?;

    // Power terms at HP via exp(2λ·ln(H/S))
    let sigma_sq_std = fp_mul_i(sigma as i128, sigma as i128)?;
    // r as i128 ≤ ~1e12 (rate at SCALE), sigma_sq_std ≤ SCALE (volatility² ≤ 1.0 at SCALE); sum ≤ ~2e12, fits i128
    let two_lambda_std = fp_div_i(2 * (r as i128 + sigma_sq_std / 2), sigma_sq_std)?;
    let two_lambda_hp = upscale_std_to_hp(two_lambda_std as u128)?;
    // two_lambda_hp ≤ ~100·SCALE_HP (lambda is a dimensionless financial ratio, typically ≤ 100); 2·SCALE_HP ≈ 2e15; no underflow for lambda > 1
    let two_lambda_m2_hp = two_lambda_hp - 2 * SCALE_HP;
    let ln_h_over_s_hp = -ln_sh;

    let pow_2l_hp = if ln_h_over_s_hp == 0 {
        SCALE_HP
    } else {
        exp_fixed_hp(fp_mul_hp_i(two_lambda_hp, ln_h_over_s_hp)?)?
    };
    let pow_2lm2_hp = if ln_h_over_s_hp == 0 {
        SCALE_HP
    } else {
        exp_fixed_hp(fp_mul_hp_i(two_lambda_m2_hp, ln_h_over_s_hp)?)?
    };

    Ok(HaugIntermediates {
        s_hp, k_disc_hp, x1_hp, y1_hp, d1_hp, y_hp,
        sigma_sqrt_t_hp, discount_hp, pow_2l_hp, pow_2lm2_hp,
        phi: if is_call { 1 } else { -1 },
    })
}

/// Compute a single Haug building block at HP.
/// block(z) = φ·[s_eff·N(eta·z) - k_eff·N(eta·(z - σ√T))]
/// where eta = phi.
#[inline(never)]
fn block_hp(phi: i128, z: i128, s_eff: i128, k_eff: i128, sst: i128) -> Result<i128, SolMathError> {
    // eta = phi for all blocks
    // phi ∈ {-1, +1} (literal scalar, not SCALE-valued); z and sst are HP-scale ∈ [-~1e15, ~1e15]
    // z - sst: both ≤ ~1e15; difference ≤ ~2e15, fits i128
    // phi * z, phi * (z - sst): sign flips only, magnitude unchanged, fits i128
    // fp_mul_hp_i outputs ∈ [-~1e15, ~1e15] (price × N(·) where N ∈ [0,1]); difference ≤ ~2e15, fits i128
    // outer phi * (...): sign flip, magnitude unchanged; fits i128
    Ok(phi * (
        fp_mul_hp_i(s_eff, norm_cdf_poly_hp(phi * z)?)?
        - fp_mul_hp_i(k_eff, norm_cdf_poly_hp(phi * (z - sst))?)?
    ))
}

/// Compute all 4 building blocks: (A, B, C, D).
#[inline(never)]
fn all_blocks(im: &HaugIntermediates) -> Result<(i128, i128, i128, i128), SolMathError> {
    let s_pow = fp_mul_hp_i(im.s_hp, im.pow_2l_hp)?;
    let k_pow = fp_mul_hp_i(im.k_disc_hp, im.pow_2lm2_hp)?;

    let a = block_hp(im.phi, im.x1_hp, im.s_hp, im.k_disc_hp, im.sigma_sqrt_t_hp)?;
    let b = block_hp(im.phi, im.d1_hp, im.s_hp, im.k_disc_hp, im.sigma_sqrt_t_hp)?;
    let c = block_hp(im.phi, im.y1_hp, s_pow, k_pow, im.sigma_sqrt_t_hp)?;
    let d = block_hp(im.phi, im.y_hp, s_pow, k_pow, im.sigma_sqrt_t_hp)?;

    Ok((a, b, c, d))
}

/// Single barrier European option price via Rubinstein-Reiner building blocks.
///
/// Prices a European option with a single knock-in or knock-out barrier
/// using Haug's ABCD decomposition, verified against QuantLib on 443K vectors.
///
/// # Parameters
/// - `s` -- Spot price at SCALE (u128)
/// - `k` -- Strike price at SCALE (u128)
/// - `h` -- Barrier level at SCALE (u128)
/// - `r` -- Risk-free rate at SCALE (u128, e.g. `50_000_000_000` = 5%)
/// - `sigma` -- Volatility at SCALE (u128, e.g. `250_000_000_000` = 25%)
/// - `t` -- Time to expiry in years at SCALE (u128)
/// - `is_call` -- `true` for call, `false` for put
/// - `barrier_type` -- [`BarrierType`] variant
///
/// # Errors
/// Returns `Err(DomainError)` if `s`, `k`, `h`, `sigma`, or `t` are zero.
///
/// # Accuracy
/// Max 1.7K ULP, P99 33, median 1. CU: ~160K average.
///
/// Public return values preserve exact in/out conservation after rounding.
///
/// # Example
/// ```
/// use solmath::{barrier_option, BarrierType, SCALE};
/// let result = barrier_option(
///     100 * SCALE, 100 * SCALE, 90 * SCALE,
///     50_000_000_000, 250_000_000_000, SCALE,
///     true, BarrierType::DownAndOut,
/// )?;
/// assert!(result.price > 0);
/// assert!(result.price <= result.vanilla);
/// # Ok::<(), solmath::SolMathError>(())
/// ```
pub fn barrier_option(
    s: u128, k: u128, h: u128,
    r: u128, sigma: u128, t: u128,
    is_call: bool,
    barrier_type: BarrierType,
) -> Result<BarrierResult, SolMathError> {
    if s == 0 || k == 0 || sigma == 0 || t == 0 || h == 0 {
        return Err(SolMathError::DomainError);
    }

    let is_down = matches!(barrier_type, BarrierType::DownAndOut | BarrierType::DownAndIn);
    let is_out = matches!(barrier_type, BarrierType::DownAndOut | BarrierType::UpAndOut);

    // Already at or past barrier
    if (is_down && s <= h) || (!is_down && s >= h) {
        let (call, put) = black_scholes_price_hp(s, k, r, sigma, t)?;
        let vanilla = if is_call { call } else { put };
        return Ok(BarrierResult { price: if is_out { 0 } else { vanilla }, vanilla });
    }

    // Impossible payoff: up call K≥H, down put K≤H
    if is_call && !is_down && k >= h {
        let (call, _) = black_scholes_price_hp(s, k, r, sigma, t)?;
        return Ok(BarrierResult { price: if is_out { 0 } else { call }, vanilla: call });
    }
    if !is_call && is_down && k <= h {
        let (_, put) = black_scholes_price_hp(s, k, r, sigma, t)?;
        return Ok(BarrierResult { price: if is_out { 0 } else { put }, vanilla: put });
    }

    let im = compute_intermediates(s, k, h, r, sigma, t, is_call)?;
    let (a, b, c, d) = all_blocks(&im)?;
    let vanilla_hp = b;

    // Select formula based on verified QuantLib match:
    //   Down call K≥H: out = B-D        Down call K<H: out = A-C
    //   Down put  K>H: out = B-A+C-D    Up call   K<H: out = B-A+C-D
    //   Up put   K≤H: out = B-D         Up put    K>H: digital decomposition
    let out_hp = if !is_down && !is_call && k > h {
        // Up put K > H: digital decomposition
        let im_h = compute_intermediates(s, h, h, r, sigma, t, false)?;
        let (_, b_h, _, d_h) = all_blocks(&im_h)?;
        // b_h, d_h are Haug blocks at HP ∈ [-~1e20, ~1e20] (price × N(·)); d_h ≤ b_h by construction; no underflow
        let p_uo_h_hp = b_h - d_h;

        // Digital: disc · [N(σ√T - x₁) - (H/S)^α · N(σ√T - y₁)]
        // sigma_sqrt_t_hp - x1_hp: both ∈ [-~1e15, ~1e15]; difference ≤ ~2e15, fits i128
        // sigma_sqrt_t_hp - y1_hp: same reasoning
        // N(·) ∈ [0, SCALE_HP]; fp_mul_hp_i result ∈ [0, SCALE_HP]; difference ≤ SCALE_HP, fits i128
        let digital_hp = fp_mul_hp_i(
            im.discount_hp,
            norm_cdf_poly_hp(im.sigma_sqrt_t_hp - im.x1_hp)?
                - fp_mul_hp_i(im.pow_2lm2_hp, norm_cdf_poly_hp(im.sigma_sqrt_t_hp - im.y1_hp)?)?,
        )?;

        // p_uo_h_hp ∈ [-~1e20, ~1e20]; fp_mul_hp_i of (k-h) upscaled × digital ∈ [-~1e20, ~1e20]; sum ≤ ~2e20, fits i128
        p_uo_h_hp + fp_mul_hp_i(upscale_std_to_hp(k - h)?, digital_hp)?
    } else if is_down && is_call && k < h {
        // Down call K < H: out = A - C
        // a, c: Haug blocks at HP ∈ [-~1e20, ~1e20]; a ≥ c by formula construction; difference ∈ [-~1e20, ~1e20], fits i128
        a - c
    } else if (is_down && !is_call && k > h) || (!is_down && is_call) {
        // Straddling: down put K>H or up call K<H: out = B - A + C - D
        // a, b, c, d all Haug blocks at HP ∈ [-~1e20, ~1e20]; cumulative sum of four terms ≤ ~4e20, fits i128
        b - a + c - d
    } else {
        // Non-straddling: down call K≥H or up put K≤H: out = B - D
        // b, d: Haug blocks at HP ∈ [-~1e20, ~1e20]; d ≤ b by formula construction; no underflow; fits i128
        b - d
    };

    let vanilla = downscale_hp_to_std(vanilla_hp);
    let out_price = core::cmp::min(downscale_hp_to_std(out_hp), vanilla);
    let price = if is_out { out_price } else { vanilla - out_price };

    Ok(BarrierResult { price, vanilla })
}
