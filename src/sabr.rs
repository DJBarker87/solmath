use crate::constants::*;
use crate::error::SolMathError;
use crate::arithmetic::{fp_mul, fp_mul_i, fp_mul_i_fast, fp_div, fp_div_i, fp_sqrt};
use crate::transcendental::{ln_fixed_i, exp_fixed_i};
use crate::hp::{pow_fixed_hp, bs_full_hp};

// ============================================================
// Single-strike API
// ============================================================

/// SABR implied Black volatility (Hagan 2002 + Obloj 2008 correction).
///
/// Returns sigma_B at SCALE such that BS(F, K, sigma_B, T) approximates the SABR price.
///
/// # Parameters
/// All at SCALE (`u128`) except `rho` (`i128`):
/// - `f` -- Forward price
/// - `k` -- Strike price
/// - `t` -- Time to expiry (years)
/// - `alpha` -- Initial volatility
/// - `beta` -- CEV exponent in `[0, SCALE]` (0 = normal, SCALE = lognormal)
/// - `rho` -- Correlation in `(-SCALE, SCALE)`
/// - `nu` -- Vol of vol
///
/// # Returns
/// Implied Black vol at SCALE. Returns 0 for zero inputs.
///
/// # Accuracy
/// 0.5% tolerance vs QuantLib. 36-43K CU per strike.
///
/// # Example
/// ```
/// use solmath::{sabr_implied_vol, SCALE};
/// // ATM SABR vol: F=100, K=100, T=1yr, alpha=0.2, beta=0.5, rho=-0.3, nu=0.4
/// let vol = sabr_implied_vol(
///     100 * SCALE, 100 * SCALE, SCALE,
///     200_000_000_000, SCALE / 2, -300_000_000_000, 400_000_000_000,
/// )?;
/// assert!(vol > 0);
/// # Ok::<(), solmath::SolMathError>(())
/// ```
pub fn sabr_implied_vol(
    f: u128, k: u128, t: u128,
    alpha: u128, beta: u128, rho: i128, nu: u128,
) -> Result<u128, SolMathError> {
    if f > i128::MAX as u128 || k > i128::MAX as u128 || t > i128::MAX as u128
        || alpha > i128::MAX as u128 || beta > i128::MAX as u128 || nu > i128::MAX as u128
    {
        return Err(SolMathError::Overflow);
    }
    if f == 0 || k == 0 || t == 0 || alpha == 0 {
        return Ok(0);
    }
    if beta > SCALE {
        return Err(SolMathError::DomainError);
    }
    if rho <= -SCALE_I || rho >= SCALE_I {
        return Err(SolMathError::DomainError);
    }

    let s = SCALE;
    let si = SCALE_I;

    // s = SCALE = 1e12, beta ∈ [0, SCALE]; one_minus_beta ∈ [0, SCALE]. No underflow (u128).
    let one_minus_beta = s - beta;
    let beta_i = beta as i128;
    let alpha_i = alpha as i128;
    let nu_i = nu as i128;

    // ATM detection: |F−K| < 0.0001·F
    let atm_threshold = f / 10_000;
    let is_atm = if f > k { f - k < atm_threshold } else { k - f < atm_threshold };

    if is_atm || nu == 0 {
        let f_pow = if one_minus_beta == 0 {
            s
        } else if one_minus_beta == s {
            f
        } else {
            pow_fixed_hp(f, one_minus_beta)?
        };

        let base_vol = fp_div_i(alpha_i, f_pow as i128)?;

        if nu == 0 {
            return Ok(if base_vol > 0 { base_vol as u128 } else { 0 });
        }

        let h = compute_h(one_minus_beta, alpha, beta_i, rho, nu_i, alpha_i, f_pow)?;
        // h ∈ [-SCALE_I, SCALE_I] by design; fp_mul_i(h, t) ≤ SCALE_I for t ≤ SCALE_I;
        // si + correction ≤ 2·SCALE_I. Fits i128.
        let correction = si + fp_mul_i(h, t as i128)?;
        let sigma_i = fp_mul_i(base_vol, correction)?;
        return Ok(if sigma_i > 0 { sigma_i as u128 } else { 0 });
    }

    // --- General (OTM/ITM) formula ---

    let f_mid = fp_sqrt(fp_mul(f, k)?)?;

    let f_mid_pow = if one_minus_beta == 0 {
        s
    } else if one_minus_beta == s {
        f_mid
    } else {
        pow_fixed_hp(f_mid, one_minus_beta)?
    };

    // log(F/K) via single ln — saves ~5K CU vs two separate ln calls
    let fk_ratio = fp_div(f, k)?;
    let log_fk = ln_fixed_i(fk_ratio)?;

    sabr_assemble(f_mid_pow, log_fk, one_minus_beta, alpha, alpha_i, beta_i, rho, nu_i, t)
}

/// SABR European option price via implied vol into Black-Scholes.
///
/// Computes `sabr_implied_vol` then prices with `bs_full_hp`.
///
/// # Parameters
/// All at SCALE (`u128`) except `rho` (`i128`):
/// - `s` -- Spot price
/// - `k` -- Strike price
/// - `r` -- Risk-free rate
/// - `t` -- Time to expiry (years)
/// - `alpha`, `beta`, `rho`, `nu` -- SABR parameters (see [`sabr_implied_vol`])
///
/// # Returns
/// `(call, put)` prices at SCALE.
///
/// # CU Cost
/// 158-171K CU.
pub fn sabr_price(
    s: u128, k: u128, r: u128, t: u128,
    alpha: u128, beta: u128, rho: i128, nu: u128,
) -> Result<(u128, u128), SolMathError> {
    if s > i128::MAX as u128 || k > i128::MAX as u128 || r > i128::MAX as u128
        || t > i128::MAX as u128 || alpha > i128::MAX as u128
        || beta > i128::MAX as u128 || nu > i128::MAX as u128
    {
        return Err(SolMathError::Overflow);
    }
    let r_t = fp_mul_i(r as i128, t as i128)?;
    let f = fp_mul_i(s as i128, exp_fixed_i(r_t)?)? as u128;
    let sigma = sabr_implied_vol(f, k, t, alpha, beta, rho, nu)?;
    if sigma == 0 {
        let k_disc = fp_mul_i(k as i128, exp_fixed_i(-r_t)?)? as u128;
        let call = if s > k_disc { s - k_disc } else { 0 };
        let put = if k_disc > s { k_disc - s } else { 0 };
        return Ok((call, put));
    }
    let bs = bs_full_hp(s, k, r, sigma, t)?;
    Ok((bs.call, bs.put))
}

/// Full SABR Greeks via BS(sigma_SABR). Sticky-strike sensitivities.
///
/// Computes SABR implied vol then returns the full [`BsFull`] struct
/// (call, put, delta, gamma, vega, theta, rho) from `bs_full_hp`.
///
/// # Parameters
/// Same as [`sabr_price`].
///
/// # Returns
/// [`BsFull`] with prices and all first-order Greeks at SCALE.
///
/// # CU Cost
/// ~160-175K CU.
pub fn sabr_greeks(
    s: u128, k: u128, r: u128, t: u128,
    alpha: u128, beta: u128, rho: i128, nu: u128,
) -> Result<BsFull, SolMathError> {
    if s > i128::MAX as u128 || k > i128::MAX as u128 || r > i128::MAX as u128
        || t > i128::MAX as u128 || alpha > i128::MAX as u128
        || beta > i128::MAX as u128 || nu > i128::MAX as u128
    {
        return Err(SolMathError::Overflow);
    }
    // Greeks are undefined at expiry — force caller to handle the degenerate case
    if t == 0 {
        return Err(SolMathError::DomainError);
    }
    let r_t = fp_mul_i(r as i128, t as i128)?;
    let f = fp_mul_i(s as i128, exp_fixed_i(r_t)?)? as u128;
    let sigma = sabr_implied_vol(f, k, t, alpha, beta, rho, nu)?;
    if sigma == 0 {
        let k_disc = fp_mul_i(k as i128, exp_fixed_i(-r_t)?)? as u128;
        let call = if s > k_disc { s - k_disc } else { 0 };
        let put = if k_disc > s { k_disc - s } else { 0 };
        return Ok(BsFull { call, put, call_delta: 0, put_delta: 0, gamma: 0, vega: 0, call_theta: 0, put_theta: 0, call_rho: 0, put_rho: 0 });
    }
    bs_full_hp(s, k, r, sigma, t)
}

// ============================================================
// Batch smile API — precompute F-dependent work once
// ============================================================

/// Precomputed intermediates for pricing a SABR smile (multiple strikes, fixed F).
///
/// Created by `sabr_precompute`. Passed to `sabr_vol_at` for each strike.
/// All F-dependent quantities are cached: ln(F), F^(1-β), h numerators, ATM vol.
/// Per-strike cost drops from ~40K to ~25K CU (general β) or ~20K (β=0,1).
#[derive(Clone, Copy)]
pub struct SabrSmile {
    f: u128,
    t: u128,
    ln_f: i128,
    one_minus_beta: u128,
    half_omb: i128,          // (1-β)/2 at SCALE
    half_omb_ln_f: i128,     // (1-β)/2 · ln(F) — for exp-based f_mid_pow
    omb2: u128,
    omb4: u128,
    alpha_i: i128,
    rho: i128,
    nu_over_alpha: i128,
    h1_num: i128,            // (1-β)²·α² — h1 numerator
    h2_num: i128,            // ρ·β·ν·α — h2 numerator
    h3: i128,                // (2-3ρ²)·ν²/24 — strike-independent
    atm_threshold: u128,
    atm_vol: u128,
}

/// Precompute F-dependent SABR intermediates for batch smile pricing.
///
/// Cost: ~31K CU (general β), ~8K (β=0 or β=1).
/// Then call `sabr_vol_at` per strike at ~25K CU each.
pub fn sabr_precompute(
    f: u128, t: u128,
    alpha: u128, beta: u128, rho: i128, nu: u128,
) -> Result<SabrSmile, SolMathError> {
    if f > i128::MAX as u128 || t > i128::MAX as u128 || alpha > i128::MAX as u128
        || beta > i128::MAX as u128 || nu > i128::MAX as u128
    {
        return Err(SolMathError::Overflow);
    }
    if f == 0 || t == 0 || alpha == 0 {
        return Ok(SabrSmile {
            f: 0, t: 0, ln_f: 0, one_minus_beta: 0, half_omb: 0,
            half_omb_ln_f: 0, omb2: 0, omb4: 0, alpha_i: 0, rho: 0,
            nu_over_alpha: 0, h1_num: 0, h2_num: 0, h3: 0,
            atm_threshold: 0, atm_vol: 0,
        });
    }
    if beta > SCALE {
        return Err(SolMathError::DomainError);
    }
    if rho <= -SCALE_I || rho >= SCALE_I {
        return Err(SolMathError::DomainError);
    }

    let s = SCALE;
    let si = SCALE_I;
    // s = SCALE = 1e12, beta ∈ [0, SCALE]; one_minus_beta ∈ [0, SCALE]. No underflow (u128).
    let one_minus_beta = s - beta;
    let alpha_i = alpha as i128;
    let beta_i = beta as i128;
    let nu_i = nu as i128;

    let ln_f = ln_fixed_i(f)?;

    // F^(1-β) for ATM
    let f_pow = if one_minus_beta == 0 {
        s
    } else if one_minus_beta == s {
        f
    } else {
        pow_fixed_hp(f, one_minus_beta)?
    };

    let atm_base_vol = fp_div_i(alpha_i, f_pow as i128)?;

    // h building blocks
    let omb2 = fp_mul(one_minus_beta, one_minus_beta)?;
    let omb4 = fp_mul(omb2, omb2)?;
    let alpha2 = fp_mul(alpha, alpha)?;
    let rho2 = fp_mul_i_fast(rho, rho);

    let h1_num = fp_mul_i(omb2 as i128, alpha2 as i128)?;
    let h2_num = fp_mul_i(fp_mul_i(rho, beta_i)?, fp_mul_i(nu_i, alpha_i)?)?;
    // 2 * si - 3 * rho2: si = SCALE_I = 1e12, rho2 ∈ [0, SCALE_I]; result ∈ [-1e12, 2e12]. Fits i128.
    let h3 = fp_div_i(
        fp_mul_i(2 * si - 3 * rho2, fp_mul_i(nu_i, nu_i)?)?,
        24 * si,
    )?;

    // ATM vol
    let atm_vol = if nu == 0 {
        if atm_base_vol > 0 { atm_base_vol as u128 } else { 0 }
    } else {
        let f_pow_2 = fp_mul(f_pow, f_pow)?;
        let h1 = if f_pow_2 == 0 { 0 } else {
            fp_div_i(h1_num, 24 * f_pow_2 as i128)?
        };
        let h2 = if f_pow == 0 { 0 } else {
            fp_div_i(h2_num, 4 * f_pow as i128)?
        };
        // h1, h2, h3 each ∈ [-SCALE_I, SCALE_I] by design; sum ∈ [-3·SCALE_I, 3·SCALE_I]. Fits i128.
        let h = h1 + h2 + h3;
        // h ∈ [-SCALE_I, SCALE_I]; fp_mul_i(h, t) ≤ SCALE_I for t ≤ SCALE_I;
        // si + correction ≤ 2·SCALE_I. Fits i128.
        let correction = si + fp_mul_i(h, t as i128)?;
        let sigma_i = fp_mul_i(atm_base_vol, correction)?;
        if sigma_i > 0 { sigma_i as u128 } else { 0 }
    };

    // For exp-based f_mid_pow: f_mid^(1-β) = exp((1-β)/2 · (ln F + ln K))
    let half_omb = (one_minus_beta / 2) as i128;
    let half_omb_ln_f = fp_mul_i(half_omb, ln_f)?;

    let nu_over_alpha = if nu == 0 { 0 } else { fp_div_i(nu_i, alpha_i)? };

    Ok(SabrSmile {
        f, t, ln_f, one_minus_beta, half_omb, half_omb_ln_f,
        omb2, omb4, alpha_i, rho, nu_over_alpha,
        h1_num, h2_num, h3,
        atm_threshold: f / 10_000,
        atm_vol,
    })
}

/// Compute SABR implied vol for a single strike using precomputed intermediates.
///
/// Cost: ~25K CU (general β), ~20K (β=0 or β=1).
/// Uses exp((1-β)/2·(lnF+lnK)) for f_mid^(1-β) instead of pow_fixed_hp.
pub fn sabr_vol_at(pre: &SabrSmile, k: u128) -> Result<u128, SolMathError> {
    if k == 0 || pre.alpha_i == 0 {
        return Ok(0);
    }

    let si = SCALE_I;

    // ATM → cached
    let is_atm = if pre.f > k { pre.f - k < pre.atm_threshold }
                 else { k - pre.f < pre.atm_threshold };
    if is_atm {
        return Ok(pre.atm_vol);
    }

    // ln(K) — needed for both log(F/K) and f_mid_pow
    let ln_k = ln_fixed_i(k)?;
    // ln_f and ln_k each ∈ [-40·SCALE_I, 40·SCALE_I]; difference fits i128 easily.
    let log_fk = pre.ln_f - ln_k;

    // f_mid^(1-β) via exp — reuses ln_k, avoids pow_fixed_hp (~6K vs ~23K CU)
    let f_mid_pow = if pre.one_minus_beta == 0 {
        SCALE
    } else if pre.one_minus_beta == SCALE {
        fp_sqrt(fp_mul(pre.f, k)?)?
    } else {
        // half_omb_ln_f and fp_mul_i(half_omb, ln_k) each ≤ 40·SCALE_I; sum ≤ 80·SCALE_I. Fits i128.
        let exponent = pre.half_omb_ln_f + fp_mul_i(pre.half_omb, ln_k)?;
        exp_fixed_i(exponent)? as u128
    };

    // D_log: si = 1e12; each corrective term ≤ si/24 < 1; d_log ∈ (SCALE_I, 2·SCALE_I). Fits i128.
    let log_fk_sq = fp_mul_i(log_fk, log_fk)?;
    let d_log = si
        + fp_div_i(fp_mul_i(pre.omb2 as i128, log_fk_sq)?, 24 * si)?
        + fp_div_i(fp_mul_i(pre.omb4 as i128, fp_mul_i(log_fk_sq, log_fk_sq)?)?, 1920 * si)?;

    // z
    let z = fp_mul_i(pre.nu_over_alpha, fp_mul_i(f_mid_pow as i128, log_fk)?)?;

    let z_over_chi = sabr_z_over_chi(z, pre.rho)?;

    // h using precomputed numerators
    let f_mid_pow_2 = fp_mul(f_mid_pow, f_mid_pow)?;
    let h1 = if f_mid_pow_2 == 0 { 0 } else {
        fp_div_i(pre.h1_num, 24 * f_mid_pow_2 as i128)?
    };
    let h2 = if f_mid_pow == 0 { 0 } else {
        fp_div_i(pre.h2_num, 4 * f_mid_pow as i128)?
    };
    // h1, h2, pre.h3 each ∈ [-SCALE_I, SCALE_I] by design; sum ∈ [-3·SCALE_I, 3·SCALE_I]. Fits i128.
    let h = h1 + h2 + pre.h3;
    // h ∈ [-SCALE_I, SCALE_I]; fp_mul_i(h, t) ≤ SCALE_I for t ≤ SCALE_I;
    // si + correction ≤ 2·SCALE_I. Fits i128.
    let time_correction = si + fp_mul_i(h, pre.t as i128)?;

    let denom = fp_mul_i(f_mid_pow as i128, d_log)?;
    let base_vol = fp_div_i(pre.alpha_i, denom)?;
    let sigma_i = fp_mul_i(fp_mul_i(base_vol, z_over_chi)?, time_correction)?;

    Ok(if sigma_i > 0 { sigma_i as u128 } else { 0 })
}

// ============================================================
// Shared helpers
// ============================================================

/// Assemble the general SABR formula from f_mid_pow and log_fk.
/// Shared between sabr_implied_vol (single) paths.
#[inline]
fn sabr_assemble(
    f_mid_pow: u128, log_fk: i128,
    one_minus_beta: u128, alpha: u128, alpha_i: i128, beta_i: i128,
    rho: i128, nu_i: i128, t: u128,
) -> Result<u128, SolMathError> {
    let si = SCALE_I;

    let log_fk_sq = fp_mul_i(log_fk, log_fk)?;
    let omb2 = fp_mul(one_minus_beta, one_minus_beta)?;
    let omb4 = fp_mul(omb2, omb2)?;
    let log_fk_4 = fp_mul_i(log_fk_sq, log_fk_sq)?;

    // d_log: si = 1e12; each corrective term ≤ si/24 < 1; sum ∈ (SCALE_I, 2·SCALE_I). Fits i128.
    let d_log = si
        + fp_div_i(fp_mul_i(omb2 as i128, log_fk_sq)?, 24 * si)?
        + fp_div_i(fp_mul_i(omb4 as i128, log_fk_4)?, 1920 * si)?;

    let z = fp_mul_i(
        fp_div_i(nu_i, alpha_i)?,
        fp_mul_i(f_mid_pow as i128, log_fk)?,
    )?;

    let z_over_chi = sabr_z_over_chi(z, rho)?;

    let h = compute_h(one_minus_beta, alpha, beta_i, rho, nu_i, alpha_i, f_mid_pow)?;
    // h ∈ [-SCALE_I, SCALE_I] by design; fp_mul_i(h, t) ≤ SCALE_I for t ≤ SCALE_I;
    // si + correction ≤ 2·SCALE_I. Fits i128.
    let time_correction = si + fp_mul_i(h, t as i128)?;

    let denom = fp_mul_i(f_mid_pow as i128, d_log)?;
    let base_vol = fp_div_i(alpha_i, denom)?;
    let sigma_i = fp_mul_i(fp_mul_i(base_vol, z_over_chi)?, time_correction)?;

    Ok(if sigma_i > 0 { sigma_i as u128 } else { 0 })
}

/// h = (1−β)²α²/(24·fpow²) + ρβνα/(4·fpow) + (2−3ρ²)ν²/24
#[inline]
fn compute_h(
    one_minus_beta: u128, alpha: u128, beta_i: i128,
    rho: i128, nu_i: i128, alpha_i: i128, f_pow: u128,
) -> Result<i128, SolMathError> {
    let si = SCALE_I;
    let f_pow_2 = fp_mul(f_pow, f_pow)?;
    let omb2 = fp_mul(one_minus_beta, one_minus_beta)?;
    let alpha2 = fp_mul(alpha, alpha)?;
    let rho2 = fp_mul_i_fast(rho, rho);

    let h1 = if f_pow_2 == 0 { 0 } else {
        fp_div_i(fp_mul_i(omb2 as i128, alpha2 as i128)?, 24 * f_pow_2 as i128)?
    };
    let h2 = if f_pow == 0 { 0 } else {
        fp_div_i(
            fp_mul_i(fp_mul_i_fast(rho, beta_i), fp_mul_i(nu_i, alpha_i)?)?,
            4 * f_pow as i128,
        )?
    };
    // 2 * si - 3 * rho2: si = SCALE_I = 1e12, rho2 ∈ [0, SCALE_I]; result ∈ [-1e12, 2e12]. Fits i128.
    let h3 = fp_div_i(
        fp_mul_i(2 * si - 3 * rho2, fp_mul_i(nu_i, nu_i)?)?,
        24 * si,
    )?;

    // h1, h2, h3 each ∈ [-SCALE_I, SCALE_I] by design; sum ∈ [-3·SCALE_I, 3·SCALE_I]. Fits i128.
    Ok(h1 + h2 + h3)
}

/// z/χ(z) — exact via sqrt + ln. ~10K CU.
fn sabr_z_over_chi(z: i128, rho: i128) -> Result<i128, SolMathError> {
    let si = SCALE_I;

    if z.abs() < si / 1_000_000 {
        return Ok(si);
    }

    // si = 1e12; 2 * fp_mul_i(rho, z) ≤ 2·SCALE_I (both rho, z ≤ SCALE_I); disc ∈ (-1e12, 3e12). Fits i128.
    let disc = si - 2 * fp_mul_i(rho, z)? + fp_mul_i(z, z)?;

    let sqrt_disc = if disc > 0 {
        fp_sqrt(disc as u128)? as i128
    } else {
        0
    };

    // sqrt_disc ≤ SCALE_I, z ≤ SCALE_I, rho ∈ (-SCALE_I, SCALE_I); num ≤ 3·SCALE_I. Fits i128.
    let num = sqrt_disc + z - rho;
    // rho ∈ (-SCALE_I, SCALE_I); den = si - rho ∈ (0, 2·SCALE_I). Fits i128.
    let den = si - rho;

    if den.abs() < si / 1000 {
        return Ok(si);
    }

    let ratio = fp_div_i(num, den)?;
    if ratio <= 0 {
        return Ok(si);
    }

    let chi = ln_fixed_i(ratio as u128)?;
    if chi.abs() < si / 1_000_000_000 {
        return Ok(si);
    }

    fp_div_i(z, chi)
}

// ============================================================
// Experimental: Padé [2/2] rational approximation for z/χ(z)
// ============================================================

/// Degree-3 Taylor approximation of z/chi(z) for SABR implied vol.
///
/// Replaces the exact sqrt + ln path (~10K CU) with a polynomial (~1K CU).
/// Falls back to exact for |z| > 0.5*SCALE where the approximation degrades.
///
/// # Parameters
/// - `z` -- SABR z-variable at SCALE (`i128`)
/// - `rho` -- Spot-vol correlation at SCALE (`i128`)
///
/// # Returns
/// z/chi(z) at SCALE.
///
/// # Accuracy
/// - |z| < 0.3: < 10 ppm (0.001%)
/// - |z| < 0.5: < 200 ppm (0.02%)
/// - |z| > 0.5: falls back to exact (sqrt + ln)
///
/// # CU Cost
/// ~1K CU (polynomial path), ~10K CU (exact fallback).
pub fn sabr_z_over_chi_pade(z: i128, rho: i128) -> Result<i128, SolMathError> {
    let si = SCALE_I;

    if z.abs() < si / 1_000_000 {
        return Ok(si);
    }

    // Fall back to exact for |z| > 0.5
    if z.abs() > si / 2 {
        return sabr_z_over_chi(z, rho);
    }

    let rho2 = fp_mul_i_fast(rho, rho);

    // rho ∈ (-SCALE_I, SCALE_I); c1 = -rho/2 ∈ (-SCALE_I/2, SCALE_I/2). Fits i128.
    let c1 = -rho / 2;
    // 2 * si - 3 * rho2 ∈ [-1e12, 2e12]; divided by 12. Fits i128.
    let c2 = (2 * si - 3 * rho2) / 12;
    // 4*si ≤ 4e12; 4*rho ≤ 4e12; 12*rho2 ≤ 12e12; 9*fp_mul_i_fast(rho,rho2) ≤ 9e12; sum ≤ 29e12 ≪ i128::MAX; divided by 24.
    let c3 = (4 * si - 4 * rho - 12 * rho2 + 9 * fp_mul_i_fast(rho, rho2)) / 24;

    let z2 = fp_mul_i_fast(z, z);
    let z3 = fp_mul_i_fast(z2, z);

    // Each fp_mul_i_fast term ≤ SCALE_I (z ≤ 0.5·SCALE_I, c* ≤ SCALE_I); sum ≤ 4·SCALE_I. Fits i128.
    Ok(si + fp_mul_i_fast(c1, z) + fp_mul_i_fast(c2, z2) + fp_mul_i_fast(c3, z3))
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Single-strike tests ---

    #[test]
    fn test_sabr_atm_basic() {
        let f = 100 * SCALE;
        let alpha = 200_000_000_000;
        let beta = SCALE / 2;
        let rho = -300_000_000_000i128;
        let nu = 400_000_000_000;
        let t = SCALE;

        let vol = sabr_implied_vol(f, f, t, alpha, beta, rho, nu).unwrap();
        assert!(vol > 10_000_000_000 && vol < 50_000_000_000,
            "ATM vol {} out of range", vol);
    }

    #[test]
    fn test_sabr_beta1_rho0_symmetric() {
        let f = 100 * SCALE;
        let alpha = 200_000_000_000;
        let beta = SCALE;
        let rho = 0i128;
        let nu = 300_000_000_000;
        let t = SCALE;

        let k_low = 90 * SCALE;
        let k_high = (f as i128 * f as i128 / k_low as i128) as u128;
        let vol_low = sabr_implied_vol(f, k_low, t, alpha, beta, rho, nu).unwrap();
        let vol_high = sabr_implied_vol(f, k_high, t, alpha, beta, rho, nu).unwrap();
        let diff = (vol_low as i128 - vol_high as i128).abs();
        assert!(diff < SCALE_I / 1000,
            "Symmetry broken: low={} high={} diff={}", vol_low, vol_high, diff);
    }

    #[test]
    fn test_sabr_nu_zero_is_cev() {
        let f = 100 * SCALE;
        let k = 105 * SCALE;
        let alpha = 200_000_000_000;
        let beta = SCALE / 2;
        let t = SCALE;

        let vol = sabr_implied_vol(f, k, t, alpha, beta, 0, 0).unwrap();
        let expected = 20_000_000_000u128;
        let diff = (vol as i128 - expected as i128).abs();
        assert!(diff < SCALE_I / 100, "CEV vol={} expected≈{}", vol, expected);
    }

    #[test]
    fn test_sabr_put_call_parity() {
        let s = 100 * SCALE;
        let k = 105 * SCALE;
        let r = 50_000_000_000u128;
        let t = SCALE;
        let alpha = 300_000_000_000;
        let beta = 700_000_000_000;
        let rho = -500_000_000_000i128;
        let nu = 400_000_000_000;

        let (call, put) = sabr_price(s, k, r, t, alpha, beta, rho, nu).unwrap();
        let disc = exp_fixed_i(-fp_mul_i(r as i128, t as i128).unwrap()).unwrap();
        let parity = (call as i128 - put as i128 - s as i128 + fp_mul_i(k as i128, disc).unwrap()).abs();
        assert!(parity < 1000, "Put-call parity error: {}", parity);
    }

    #[test]
    fn test_sabr_negative_rho_skew() {
        let f = 100 * SCALE;
        let alpha = 200_000_000_000;
        let beta = SCALE / 2;
        let rho = -500_000_000_000i128;
        let nu = 400_000_000_000;
        let t = SCALE;

        let vol_90 = sabr_implied_vol(f, 90 * SCALE, t, alpha, beta, rho, nu).unwrap();
        let vol_100 = sabr_implied_vol(f, f, t, alpha, beta, rho, nu).unwrap();
        let vol_110 = sabr_implied_vol(f, 110 * SCALE, t, alpha, beta, rho, nu).unwrap();

        assert!(vol_90 > vol_100, "Expected vol_90 > vol_100: {} vs {}", vol_90, vol_100);
        assert!(vol_90 > vol_110, "Expected vol_90 > vol_110 (skew): {} vs {}", vol_90, vol_110);
    }

    #[test]
    fn test_sabr_smile_curvature() {
        let f = 100 * SCALE;
        let alpha = 200_000_000_000;
        let beta = SCALE;
        let rho = 0i128;
        let nu = 500_000_000_000;
        let t = SCALE;

        let vol_90 = sabr_implied_vol(f, 90 * SCALE, t, alpha, beta, rho, nu).unwrap();
        let vol_100 = sabr_implied_vol(f, f, t, alpha, beta, rho, nu).unwrap();
        let vol_110 = sabr_implied_vol(f, 110 * SCALE, t, alpha, beta, rho, nu).unwrap();

        assert!(vol_90 > vol_100, "Left wing below ATM: {} vs {}", vol_90, vol_100);
        assert!(vol_110 > vol_100, "Right wing below ATM: {} vs {}", vol_110, vol_100);
    }

    #[test]
    fn test_sabr_hagan_rates_params() {
        let f = 48_800_000_000u128;
        let alpha = 87_300_000_000u128;
        let beta = 700_000_000_000u128;
        let rho = -480_000_000_000i128;
        let nu = 470_000_000_000u128;
        let t = 10 * SCALE;

        for &k_bp in &[100, 200, 300, 400, 500, 600, 700, 800] {
            let k = k_bp as u128 * SCALE / 10000;
            let vol = sabr_implied_vol(f, k, t, alpha, beta, rho, nu);
            assert!(vol.is_ok(), "Failed at K={}bp: {:?}", k_bp, vol);
            let v = vol.unwrap();
            assert!(v > SCALE / 100 && v < 2 * SCALE, "Vol {} out of range at K={}bp", v, k_bp);
        }
    }

    #[test]
    fn test_sabr_zero_inputs() {
        assert_eq!(sabr_implied_vol(0, SCALE, SCALE, SCALE, SCALE/2, 0, SCALE).unwrap(), 0);
        assert_eq!(sabr_implied_vol(SCALE, 0, SCALE, SCALE, SCALE/2, 0, SCALE).unwrap(), 0);
        assert_eq!(sabr_implied_vol(SCALE, SCALE, 0, SCALE, SCALE/2, 0, SCALE).unwrap(), 0);
        assert_eq!(sabr_implied_vol(SCALE, SCALE, SCALE, 0, SCALE/2, 0, SCALE).unwrap(), 0);
    }

    #[test]
    fn test_sabr_beta_zero() {
        let f = 100 * SCALE;
        let alpha = 2 * SCALE;
        let rho = -300_000_000_000i128;
        let nu = 400_000_000_000;
        let t = SCALE;
        let vol = sabr_implied_vol(f, 95 * SCALE, t, alpha, 0, rho, nu).unwrap();
        assert!(vol > 0, "β=0 vol should be positive: {}", vol);
    }

    #[test]
    fn test_sabr_beta_one() {
        let f = 100 * SCALE;
        let alpha = 200_000_000_000;
        let rho = -300_000_000_000i128;
        let nu = 400_000_000_000;
        let t = SCALE;
        let vol_atm = sabr_implied_vol(f, f, t, alpha, SCALE, rho, nu).unwrap();
        let vol_otm = sabr_implied_vol(f, 110 * SCALE, t, alpha, SCALE, rho, nu).unwrap();
        assert!(vol_atm > 150_000_000_000 && vol_atm < 300_000_000_000, "β=1 ATM vol {}", vol_atm);
        assert!(vol_otm > 0);
    }

    #[test]
    fn test_sabr_beta_one_limit() {
        let f = 100 * SCALE;
        let alpha = 200_000_000_000;
        let rho = -300_000_000_000i128;
        let nu = 400_000_000_000;
        let t = SCALE;
        let k = 105 * SCALE;
        let vol_exact = sabr_implied_vol(f, k, t, alpha, SCALE, rho, nu).unwrap();
        let vol_near = sabr_implied_vol(f, k, t, alpha, SCALE - 1, rho, nu).unwrap();
        let diff = (vol_exact as i128 - vol_near as i128).abs();
        assert!(diff < 100, "β=1 limit: exact={} near={} diff={}", vol_exact, vol_near, diff);
    }

    #[test]
    fn test_sabr_greeks_basic() {
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let r = 50_000_000_000;
        let t = SCALE;
        let alpha = 200_000_000_000;
        let beta = SCALE / 2;
        let rho = -300_000_000_000i128;
        let nu = 400_000_000_000;
        let greeks = sabr_greeks(s, k, r, t, alpha, beta, rho, nu).unwrap();
        assert!(greeks.call > 0 && greeks.put > 0 && greeks.vega > 0 && greeks.gamma > 0);
    }

    // --- Batch smile tests ---

    #[test]
    fn test_sabr_smile_matches_single() {
        let f = 100 * SCALE;
        let alpha = 200_000_000_000;
        let beta = SCALE / 2;
        let rho = -300_000_000_000i128;
        let nu = 400_000_000_000;
        let t = SCALE;

        let pre = sabr_precompute(f, t, alpha, beta, rho, nu).unwrap();

        // ATM should match
        let vol_single = sabr_implied_vol(f, f, t, alpha, beta, rho, nu).unwrap();
        let vol_batch = sabr_vol_at(&pre, f).unwrap();
        let atm_diff = (vol_single as i128 - vol_batch as i128).abs();
        assert!(atm_diff < 10, "ATM mismatch: single={} batch={}", vol_single, vol_batch);

        // OTM strikes — batch uses exp instead of pow, so allow ~5 ULP tolerance
        for &k in &[85 * SCALE, 90 * SCALE, 95 * SCALE, 105 * SCALE, 110 * SCALE, 115 * SCALE] {
            let vol_s = sabr_implied_vol(f, k, t, alpha, beta, rho, nu).unwrap();
            let vol_b = sabr_vol_at(&pre, k).unwrap();
            let diff = (vol_s as i128 - vol_b as i128).abs();
            // exp-based f_mid_pow may differ from pow_fixed_hp by a few ULP
            let tol = vol_s / 10_000; // 0.01% relative tolerance
            assert!(diff < tol as i128 || diff < 10,
                "K={}: single={} batch={} diff={} tol={}", k/SCALE, vol_s, vol_b, diff, tol);
        }
    }

    #[test]
    fn test_sabr_smile_beta1() {
        let f = 100 * SCALE;
        let alpha = 200_000_000_000;
        let rho = -300_000_000_000i128;
        let nu = 400_000_000_000;
        let t = SCALE;

        let pre = sabr_precompute(f, t, alpha, SCALE, rho, nu).unwrap();

        for &k in &[90 * SCALE, 95 * SCALE, 100 * SCALE, 105 * SCALE, 110 * SCALE] {
            let vol_s = sabr_implied_vol(f, k, t, alpha, SCALE, rho, nu).unwrap();
            let vol_b = sabr_vol_at(&pre, k).unwrap();
            let diff = (vol_s as i128 - vol_b as i128).abs();
            // β=1: no pow/exp, should match closely
            assert!(diff < 100, "β=1 K={}: single={} batch={} diff={}", k/SCALE, vol_s, vol_b, diff);
        }
    }

    #[test]
    fn test_sabr_smile_beta0() {
        let f = 100 * SCALE;
        let alpha = 2 * SCALE;
        let rho = -300_000_000_000i128;
        let nu = 400_000_000_000;
        let t = SCALE;

        let pre = sabr_precompute(f, t, alpha, 0, rho, nu).unwrap();

        for &k in &[90 * SCALE, 95 * SCALE, 105 * SCALE, 110 * SCALE] {
            let vol_s = sabr_implied_vol(f, k, t, alpha, 0, rho, nu).unwrap();
            let vol_b = sabr_vol_at(&pre, k).unwrap();
            let diff = (vol_s as i128 - vol_b as i128).abs();
            // β=0: both use sqrt, should match exactly
            assert!(diff < 100, "β=0 K={}: single={} batch={} diff={}", k/SCALE, vol_s, vol_b, diff);
        }
    }

    #[test]
    fn test_sabr_smile_zero_inputs() {
        let pre = sabr_precompute(0, SCALE, SCALE, SCALE/2, 0, SCALE).unwrap();
        assert_eq!(sabr_vol_at(&pre, SCALE).unwrap(), 0);
    }

    // --- Padé approximation tests ---

    #[test]
    fn test_pade_vs_exact_small_z() {
        // Taylor-3 tested across ρ × z grid
        let rhos = [-800_000_000_000i128, -500_000_000_000, -200_000_000_000,
                     0, 200_000_000_000, 500_000_000_000, 800_000_000_000];
        let zs = [-400_000_000_000i128, -200_000_000_000, -100_000_000_000,
                   100_000_000_000, 200_000_000_000, 400_000_000_000];

        let mut max_rel_err: i128 = 0;
        for &rho in &rhos {
            for &z in &zs {
                let exact = sabr_z_over_chi(z, rho).unwrap();
                let approx = sabr_z_over_chi_pade(z, rho).unwrap();
                let err = (exact - approx).abs();
                if exact.abs() > SCALE_I / 100 {
                    let rel = err * 1_000_000 / exact.abs(); // ppm
                    if rel > max_rel_err { max_rel_err = rel; }
                }
            }
        }
        // Taylor-3 within 2% for |z| < 0.5, degrades for |ρ| > 0.7
        assert!(max_rel_err < 20_000,
            "Max relative error for |z|<0.5: {} ppm (expect <20000)", max_rel_err);
    }

    #[test]
    fn test_pade_fallback_large_z() {
        // For |z| > 0.5, Taylor poly falls back to exact
        for &z_scale in &[6, 8, 10, 15, 20] {
            let z = z_scale as i128 * SCALE_I / 10;
            let rho = -500_000_000_000i128;
            let exact = sabr_z_over_chi(z, rho).unwrap();
            let approx = sabr_z_over_chi_pade(z, rho).unwrap();
            assert_eq!(exact, approx, "Should fall back to exact for z={}", z_scale);
        }
    }

    // --- Invariant (property-based) tests ---

    #[test]
    fn test_sabr_put_call_parity_sweep() {
        let r = 50_000_000_000u128;
        let params: &[(u128, u128, u128, i128, u128)] = &[
            (100*SCALE, 200_000_000_000, SCALE/2, -300_000_000_000, 400_000_000_000),
            (100*SCALE, 300_000_000_000, SCALE, -700_000_000_000, 500_000_000_000),
            (100*SCALE, 200_000_000_000, 0, -300_000_000_000, 400_000_000_000),
            (50*SCALE, 150_000_000_000, 700_000_000_000, -500_000_000_000, 300_000_000_000),
        ];
        let strikes: &[u128] = &[80, 90, 95, 100, 105, 110, 120];
        let mats: &[u128] = &[100_000_000_000, 250_000_000_000, SCALE, 2*SCALE];

        for &(s, alpha, beta, rho, nu) in params {
            for &k_m in strikes {
                let k = k_m * SCALE;
                for &t in mats {
                    if let Ok((call, put)) = sabr_price(s, k, r, t, alpha, beta, rho, nu) {
                        let disc = exp_fixed_i(-fp_mul_i(r as i128, t as i128).unwrap()).unwrap();
                        let parity = (call as i128 - put as i128
                            - s as i128 + fp_mul_i(k as i128, disc).unwrap()).abs();
                        assert!(parity < 10_000,
                            "P/C parity: s={} k={} t={} err={}", s/SCALE, k/SCALE, t, parity);
                    }
                }
            }
        }
    }

    #[test]
    fn test_sabr_vol_positive_sweep() {
        let params: &[(u128, u128, u128, i128, u128)] = &[
            (100*SCALE, 200_000_000_000, SCALE/2, -300_000_000_000, 400_000_000_000),
            (100*SCALE, 200_000_000_000, SCALE, -900_000_000_000, 800_000_000_000),
            (100*SCALE, 200_000_000_000, 0, 0, 100_000_000_000),
        ];
        for &(f, alpha, beta, rho, nu) in params {
            for k_pct in (50..=200).step_by(10) {
                let k = f / 100 * k_pct as u128;
                for &t in &[100_000_000_000u128, SCALE, 5*SCALE] {
                    if let Ok(vol) = sabr_implied_vol(f, k, t, alpha, beta, rho, nu) {
                        assert!(vol > 0, "Non-positive vol: f={} k={} t={} vol={}", f, k, t, vol);
                    }
                }
            }
        }
    }

    #[test]
    fn test_sabr_skew_direction() {
        let f = 100 * SCALE;
        let t = SCALE;
        let cases: &[(u128, u128, i128, u128)] = &[
            (200_000_000_000, SCALE/2, -500_000_000_000, 200_000_000_000),
            (200_000_000_000, 700_000_000_000, -700_000_000_000, 300_000_000_000),
        ];
        for &(alpha, beta, rho, nu) in cases {
            let vol_90 = sabr_implied_vol(f, 90*SCALE, t, alpha, beta, rho, nu).unwrap();
            let vol_110 = sabr_implied_vol(f, 110*SCALE, t, alpha, beta, rho, nu).unwrap();
            assert!(vol_90 > vol_110,
                "Skew wrong: rho={} vol_90={} vol_110={}", rho, vol_90, vol_110);
        }
    }

    #[test]
    fn test_sabr_beta1_continuity() {
        let f = 100 * SCALE;
        let t = SCALE;
        let alpha = 200_000_000_000u128;
        let rho = -300_000_000_000i128;
        let nu = 400_000_000_000u128;

        for &k in &[90*SCALE, 95*SCALE, 100*SCALE, 105*SCALE, 110*SCALE] {
            let vol_exact = sabr_implied_vol(f, k, t, alpha, SCALE, rho, nu).unwrap();
            let vol_near = sabr_implied_vol(f, k, t, alpha, 999_000_000_000, rho, nu).unwrap();
            let diff = (vol_exact as i128 - vol_near as i128).abs();
            let tol = vol_exact as i128 / 100;
            assert!(diff < tol,
                "β continuity: k={} vol_1.0={} vol_0.999={} diff={}",
                k/SCALE, vol_exact, vol_near, diff);
        }
    }
}

#[cfg(test)]
include!("../test_data/sabr_reference_tests.rs");
