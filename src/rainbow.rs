//! Two-asset (rainbow) option pricing via the bivariate normal CDF.
//!
//! Worst-of / best-of options on two assets have analytic Stulz (1982) formulas
//! built from the bivariate normal CDF. SolMath evaluates those formulas with
//! its deterministic `bvn_cdf` kernel, so pricing needs no lattice, Monte Carlo
//! simulation, or off-chain surface.
//!
//! [`worst_of_call`] prices a call on `min(S1, S2)`; [`best_of_call`] a call on
//! `max(S1, S2)`. Each asset carries its own continuous dividend yield; `rho` is
//! the return correlation and may be negative. All values are fixed-point at
//! `SCALE = 1e12`; `rho` is signed at SCALE.
//!
//! Validated against Monte Carlo (6M paths) and the Stulz analytic reference to
//! MC noise (~1e-3) across positive and negative correlation.

use crate::arithmetic::{fp_div_i, fp_mul_i, fp_sqrt};
use crate::bvn_cdf::bvn_cdf;
use crate::constants::{SCALE, SCALE_I};
use crate::error::SolMathError;
use crate::transcendental::{exp_fixed_i, ln_fixed_i};

const MAX_INPUT: u128 = 100_000 * SCALE;

/// Shared, validated intermediates for both min and max payoffs.
struct RainbowTerms {
    s1: i128,
    s2: i128,
    disc_q1: i128,  // e^{-q1 T}
    disc_q2: i128,  // e^{-q2 T}
    disc_r_k: i128, // K e^{-r T}
    y1: i128,
    y2: i128,
    d: i128,
    srt: i128, // σ√T with σ = √(σ1²+σ2²-2ρσ1σ2)
    v1_sqrt_t: i128,
    v2_sqrt_t: i128,
    r1: i128, // corr for the S1 term
    r2: i128, // corr for the S2 term
    rho: i128,
}

#[allow(clippy::too_many_arguments)]
fn prepare(
    s1: u128,
    s2: u128,
    k: u128,
    r: u128,
    q1: u128,
    q2: u128,
    sigma1: u128,
    sigma2: u128,
    rho: i128,
    t: u128,
) -> Result<RainbowTerms, SolMathError> {
    if s1 > MAX_INPUT
        || s2 > MAX_INPUT
        || k > MAX_INPUT
        || r > MAX_INPUT
        || q1 > MAX_INPUT
        || q2 > MAX_INPUT
        || sigma1 > MAX_INPUT
        || sigma2 > MAX_INPUT
        || t > MAX_INPUT
    {
        return Err(SolMathError::Overflow);
    }
    if s1 == 0 || s2 == 0 || k == 0 || sigma1 == 0 || sigma2 == 0 || t == 0 {
        return Err(SolMathError::DomainError);
    }
    if rho.unsigned_abs() >= SCALE {
        return Err(SolMathError::DomainError);
    }
    let (s1, s2, k, r) = (s1 as i128, s2 as i128, k as i128, r as i128);
    let (q1, q2, sigma1, sigma2, t) = (
        q1 as i128,
        q2 as i128,
        sigma1 as i128,
        sigma2 as i128,
        t as i128,
    );
    let b1 = r - q1;
    let b2 = r - q2;

    let sqrt_t = fp_sqrt(t as u128)? as i128;
    let s1_sq = fp_mul_i(sigma1, sigma1)?;
    let s2_sq = fp_mul_i(sigma2, sigma2)?;
    // σ² = σ1² + σ2² - 2ρσ1σ2
    let cross = fp_mul_i(fp_mul_i(rho, sigma1)?, sigma2)?;
    let sig_sq = s1_sq + s2_sq - 2 * cross;
    if sig_sq <= 0 {
        return Err(SolMathError::DomainError);
    }
    let sig = fp_sqrt(sig_sq as u128)? as i128;
    let srt = fp_mul_i(sig, sqrt_t)?;
    let v1_sqrt_t = fp_mul_i(sigma1, sqrt_t)?;
    let v2_sqrt_t = fp_mul_i(sigma2, sqrt_t)?;
    if srt == 0 || v1_sqrt_t == 0 || v2_sqrt_t == 0 {
        return Err(SolMathError::DomainError);
    }

    // d = [ln(S1/S2) + (b1 - b2 + σ²/2)T] / (σ√T)
    let ln_s1s2 = ln_fixed_i(fp_div_i(s1, s2)? as u128)?;
    let d = fp_div_i(ln_s1s2 + fp_mul_i(b1 - b2 + sig_sq / 2, t)?, srt)?;
    // y_i = [ln(S_i/K) + (b_i + σ_i²/2)T] / (σ_i√T)
    let y1 = fp_div_i(
        ln_fixed_i(fp_div_i(s1, k)? as u128)? + fp_mul_i(b1 + s1_sq / 2, t)?,
        v1_sqrt_t,
    )?;
    let y2 = fp_div_i(
        ln_fixed_i(fp_div_i(s2, k)? as u128)? + fp_mul_i(b2 + s2_sq / 2, t)?,
        v2_sqrt_t,
    )?;
    // r1 = (ρσ2 - σ1)/σ, r2 = (ρσ1 - σ2)/σ
    let r1 = fp_div_i(fp_mul_i(rho, sigma2)? - sigma1, sig)?;
    let r2 = fp_div_i(fp_mul_i(rho, sigma1)? - sigma2, sig)?;

    Ok(RainbowTerms {
        s1,
        s2,
        disc_q1: exp_fixed_i(-fp_mul_i(q1, t)?)?,
        disc_q2: exp_fixed_i(-fp_mul_i(q2, t)?)?,
        disc_r_k: fp_mul_i(k, exp_fixed_i(-fp_mul_i(r, t)?)?)?,
        y1,
        y2,
        d,
        srt,
        v1_sqrt_t,
        v2_sqrt_t,
        r1,
        r2,
        rho,
    })
}

/// Call on the **minimum** of two assets: `e^{-rT} E[max(min(S1,S2) - K, 0)]`.
///
/// Continuous dividend yields `q1`, `q2`; `rho` the (signed) return correlation.
/// Exact Stulz closed form via three `bvn_cdf` evaluations.
#[allow(clippy::too_many_arguments)]
pub fn worst_of_call(
    s1: u128,
    s2: u128,
    k: u128,
    r: u128,
    q1: u128,
    q2: u128,
    sigma1: u128,
    sigma2: u128,
    rho: i128,
    t: u128,
) -> Result<u128, SolMathError> {
    let tm = prepare(s1, s2, k, r, q1, q2, sigma1, sigma2, rho, t)?;
    // Cmin = S1 e^{-q1T} M(y1,-d;r1) + S2 e^{-q2T} M(y2,d-σ√T;r2)
    //        - K e^{-rT} M(y1-σ1√T, y2-σ2√T; ρ)
    let a = fp_mul_i(fp_mul_i(tm.s1, tm.disc_q1)?, bvn_cdf(tm.y1, -tm.d, tm.r1)?)?;
    let b = fp_mul_i(
        fp_mul_i(tm.s2, tm.disc_q2)?,
        bvn_cdf(tm.y2, tm.d - tm.srt, tm.r2)?,
    )?;
    let c = fp_mul_i(
        tm.disc_r_k,
        bvn_cdf(tm.y1 - tm.v1_sqrt_t, tm.y2 - tm.v2_sqrt_t, tm.rho)?,
    )?;
    Ok((a + b - c).max(0) as u128)
}

/// Call on the **maximum** of two assets: `e^{-rT} E[max(max(S1,S2) - K, 0)]`.
///
/// Exact Stulz closed form via three `bvn_cdf` evaluations.
#[allow(clippy::too_many_arguments)]
pub fn best_of_call(
    s1: u128,
    s2: u128,
    k: u128,
    r: u128,
    q1: u128,
    q2: u128,
    sigma1: u128,
    sigma2: u128,
    rho: i128,
    t: u128,
) -> Result<u128, SolMathError> {
    let tm = prepare(s1, s2, k, r, q1, q2, sigma1, sigma2, rho, t)?;
    // Cmax = S1 e^{-q1T} M(y1,d;-r1) + S2 e^{-q2T} M(y2,-d+σ√T;-r2)
    //        - K e^{-rT} [1 - M(-y1+σ1√T, -y2+σ2√T; ρ)]
    let a = fp_mul_i(fp_mul_i(tm.s1, tm.disc_q1)?, bvn_cdf(tm.y1, tm.d, -tm.r1)?)?;
    let b = fp_mul_i(
        fp_mul_i(tm.s2, tm.disc_q2)?,
        bvn_cdf(tm.y2, tm.srt - tm.d, -tm.r2)?,
    )?;
    let m = bvn_cdf(-tm.y1 + tm.v1_sqrt_t, -tm.y2 + tm.v2_sqrt_t, tm.rho)?;
    let c = fp_mul_i(tm.disc_r_k, SCALE_I - m)?;
    Ok((a + b - c).max(0) as u128)
}
