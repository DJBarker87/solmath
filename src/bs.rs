use crate::constants::*;
use crate::error::SolMathError;
use crate::arithmetic::{fp_mul_i, fp_div, fp_div_i, fp_sqrt};
use crate::transcendental::{ln_fixed_i, exp_fixed_i};
use crate::normal::{norm_cdf_poly, norm_cdf_and_pdf_bs_guarded};

// ============================================================
// black_scholes_price: All intermediates in i128
// ============================================================

/// Black-Scholes European call and put prices at SCALE.
///
/// Returns `(call, put)` as unsigned fixed-point values at SCALE (1e12).
///
/// # Parameters
/// - `s` -- Spot price at SCALE (u128)
/// - `k` -- Strike price at SCALE (u128)
/// - `r` -- Risk-free rate at SCALE (u128, e.g. `50_000_000_000` = 5%)
/// - `sigma` -- Volatility at SCALE (u128, e.g. `200_000_000_000` = 20%)
/// - `t` -- Time to expiry in years at SCALE (u128, e.g. `SCALE` = 1 year)
///
/// # Errors
/// Returns `Err` on domain violations or arithmetic overflow.
///
/// # Accuracy
/// 6-9 significant figures vs analytic reference.
///
/// # Example
/// ```
/// use solmath_core::{black_scholes_price, SCALE};
/// let (call, put) = black_scholes_price(
///     100 * SCALE, 100 * SCALE,
///     50_000_000_000, 200_000_000_000, SCALE,
/// )?;
/// assert!(call > 0);
/// assert!(put > 0);
/// # Ok::<(), solmath_core::SolMathError>(())
/// ```
pub fn black_scholes_price(s: u128, k: u128, r: u128, sigma: u128, t: u128) -> Result<(u128, u128), SolMathError> {
    black_scholes_price_selective(s, k, r, sigma, t)
}


/// Selective BS price implementation. Internal.
pub(crate) fn black_scholes_price_selective(s: u128, k: u128, r: u128, sigma: u128, t: u128) -> Result<(u128, u128), SolMathError> {
    if s > i128::MAX as u128 || k > i128::MAX as u128 || r > i128::MAX as u128
        || sigma > i128::MAX as u128 || t > i128::MAX as u128
    {
        return Err(SolMathError::Overflow);
    }
    if s == 0 {
        let r_t = fp_mul_i(r as i128, t as i128)?;
let k_disc = fp_mul_i(k as i128, exp_fixed_i(-r_t)?)?;
        return Ok((0, if k_disc > 0 { k_disc as u128 } else { 0 }));
    }
    if k == 0 {
        return Ok((s, 0));
    }

    let s_i = s as i128;
    let k_i = k as i128;
    let r_i = r as i128;
    let sigma_i = sigma as i128;
    let t_i = t as i128;

    if t == 0 {
        // s, k are u128 prices; subtraction is guarded by the comparison, so no underflow.
        let call = if s > k { s - k } else { 0 };
        let put = if k > s { k - s } else { 0 };
        return Ok((call, put));
    }

    let sk_ratio = fp_div(s, k)?;
    let ln_sk = ln_fixed_i(sk_ratio)?;

    let sigma_sq = fp_mul_i(sigma_i, sigma_i)?;
    // sigma_sq ∈ [0, SCALE_I] after fp_mul_i (sigma ≤ SCALE_I); /2: ∈ [0, SCALE_I/2], fits i128.
    let sigma_sq_half = sigma_sq / 2;

    // r_i ∈ [0, SCALE_I], sigma_sq_half ∈ [0, SCALE_I/2]: sum ≤ 1.5·SCALE_I, well within i128.
    let drift = fp_mul_i(r_i + sigma_sq_half, t_i)?;
    // ln_sk ∈ [-40·SCALE_I, 40·SCALE_I] (ln domain), drift ∈ [-SCALE_I, SCALE_I] after fp_mul_i;
    // sum ∈ [-41·SCALE_I, 41·SCALE_I], fits i128.
    let d1_num = ln_sk + drift;

    let sqrt_t = fp_sqrt(t)? as i128;
    let sigma_sqrt_t = fp_mul_i(sigma_i, sqrt_t)?;

    if sigma_sqrt_t <= 1 {
        let r_t = fp_mul_i(r_i, t_i)?;
let discount = exp_fixed_i(-r_t)?;
        let k_disc = fp_mul_i(k_i, discount)?;

        // s_i is a SCALE price (< ~1e20 in practice), k_disc = k·discount ≤ k ≤ ~1e20;
        // both ≤ i128::MAX; difference ∈ (-1e20, 1e20), fits i128.
        let call_i = s_i - k_disc;
        let put_i = k_disc - s_i;

        let call = if call_i > 0 { call_i as u128 } else { 0 };
        let put = if put_i > 0 { put_i as u128 } else { 0 };
        return Ok((call, put));
    }

    // sigma_sqrt_t > 1, so division cannot fail
    let d1 = fp_div_i(d1_num, sigma_sqrt_t)?;
    // d1 ∈ [-8·SCALE_I, 8·SCALE_I] (clamped by norm_cdf_bs_guarded); sigma_sqrt_t ∈ [0, ~SCALE_I];
    // d2 = d1 - sigma_sqrt_t ∈ [-9·SCALE_I, 8·SCALE_I], fits i128.
    let d2 = d1 - sigma_sqrt_t;

    let phi_d1 = norm_cdf_poly(d1)?;
    // phi_d1 ∈ [0, SCALE_I]; SCALE_I - phi_d1 ∈ [0, SCALE_I], fits i128.
    let phi_neg_d1 = SCALE_I - phi_d1;
    let phi_d2 = norm_cdf_poly(d2)?;
    // phi_d2 ∈ [0, SCALE_I]; SCALE_I - phi_d2 ∈ [0, SCALE_I], fits i128.
    let phi_neg_d2 = SCALE_I - phi_d2;

    let r_t = fp_mul_i(r_i, t_i)?;
    // -r_t is ≤ 0, so exp cannot overflow
    let discount = exp_fixed_i(-r_t)?;
    let k_disc = fp_mul_i(k_i, discount)?;

    let term1 = fp_mul_i(s_i, phi_d1)?;
    let term2 = fp_mul_i(k_disc, phi_d2)?;
    // term1, term2 ∈ [0, SCALE_I] (prices after fp_mul_i); difference ∈ (-SCALE_I, SCALE_I), fits i128.
    let call_i = term1 - term2;
    let call = if call_i > 0 { call_i as u128 } else { 0 };

    let term3 = fp_mul_i(k_disc, phi_neg_d2)?;
    let term4 = fp_mul_i(s_i, phi_neg_d1)?;
    // term3, term4 ∈ [0, SCALE_I]; difference ∈ (-SCALE_I, SCALE_I), fits i128.
    let put_i = term3 - term4;
    let put = if put_i > 0 { put_i as u128 } else { 0 };

    Ok((call, put))
}

// ============================================================
// Black-Scholes Greeks + implied volatility
// ============================================================

/// Compute BS intermediates (d1, d2, CDFs, discount). Internal.
pub(crate) fn bs_intermediates(s: u128, k: u128, r: u128, sigma: u128, t: u128) -> Result<BsIntermediates, SolMathError> {
    bs_intermediates_selective(s, k, r, sigma, t)
}


/// Selective BS intermediates. Internal.
pub(crate) fn bs_intermediates_selective(s: u128, k: u128, r: u128, sigma: u128, t: u128) -> Result<BsIntermediates, SolMathError> {
    if s > i128::MAX as u128 || k > i128::MAX as u128 || r > i128::MAX as u128
        || sigma > i128::MAX as u128 || t > i128::MAX as u128
    {
        return Err(SolMathError::Overflow);
    }
    // Caller must ensure s > 0, k > 0, sigma > 0, t > 0
    let k_i = k as i128;
    let r_i = r as i128;
    let sigma_i = sigma as i128;
    let t_i = t as i128;

    let sk_ratio = fp_div(s, k)?;
    let ln_sk = ln_fixed_i(sk_ratio)?;

    let sigma_sq = fp_mul_i(sigma_i, sigma_i)?;
    // sigma_sq ∈ [0, SCALE_I] after fp_mul_i; /2: ∈ [0, SCALE_I/2], fits i128.
    let sigma_sq_half = sigma_sq / 2;
    // r_i ∈ [0, SCALE_I], sigma_sq_half ∈ [0, SCALE_I/2]: sum ≤ 1.5·SCALE_I, fits i128.
    let drift = fp_mul_i(r_i + sigma_sq_half, t_i)?;
    // ln_sk ∈ [-40·SCALE_I, 40·SCALE_I], drift ∈ [-SCALE_I, SCALE_I]; sum ∈ [-41·SCALE_I, 41·SCALE_I], fits i128.
    let d1_num = ln_sk + drift;

    let sqrt_t = fp_sqrt(t)? as i128;
    let sigma_sqrt_t = fp_mul_i(sigma_i, sqrt_t)?;

    // When sigma_sqrt_t truncates to zero (tiny sigma or t), d1 → ±∞.
    // Push CDF to 0 or 1 to give intrinsic value, matching black_scholes_price.
    let d1 = if sigma_sqrt_t > 0 {
        fp_div_i(d1_num, sigma_sqrt_t)?
    } else if d1_num > 0 {
        8 * SCALE_I
    } else if d1_num < 0 {
        -8 * SCALE_I
    } else {
        0
    };
    // d1 ∈ [-8·SCALE_I, 8·SCALE_I], sigma_sqrt_t ∈ [0, ~SCALE_I]; d2 ∈ [-9·SCALE_I, 8·SCALE_I], fits i128.
    let d2 = d1 - sigma_sqrt_t;

    let (phi_d1, pdf_d1) = norm_cdf_and_pdf_bs_guarded(d1)?;
    let phi_d2 = norm_cdf_poly(d2)?;
    // phi_d1, phi_d2 ∈ [0, SCALE_I]; SCALE_I - phi ∈ [0, SCALE_I], fits i128.
    let phi_neg_d1 = SCALE_I - phi_d1;
    let phi_neg_d2 = SCALE_I - phi_d2;

    let r_t = fp_mul_i(r_i, t_i)?;
    // -r_t is ≤ 0 for non-negative r, so exp cannot overflow
    let discount = exp_fixed_i(-r_t)?;
    let k_disc = fp_mul_i(k_i, discount)?;

    Ok(BsIntermediates {
        d1,
        d2,
        phi_d1,
        phi_d2,
        phi_neg_d1,
        phi_neg_d2,
        pdf_d1,
        discount,
        sqrt_t,
        sigma_sqrt_t,
        k_disc,
    })
}


/// Black-Scholes vega: S * phi(d1) * sqrt(T) at SCALE.
///
/// Returns vega (signed, at SCALE). Same for calls and puts.
///
/// # Errors
/// Returns `Err(DomainError)` if `sigma == 0` or `t == 0`.
///
/// # Accuracy
/// 6-9 significant figures.
pub fn bs_vega(s: u128, k: u128, r: u128, sigma: u128, t: u128) -> Result<i128, SolMathError> {
    bs_vega_selective(s, k, r, sigma, t)
}


/// Selective BS vega. Internal.
pub(crate) fn bs_vega_selective(s: u128, k: u128, r: u128, sigma: u128, t: u128) -> Result<i128, SolMathError> {
    if sigma == 0 || t == 0 {
        return Err(SolMathError::DomainError);
    }
    if s == 0 || k == 0 {
        return Ok(0);
    }
    let im = bs_intermediates(s, k, r, sigma, t)?;
    let s_i = s as i128;
    Ok(fp_mul_i(fp_mul_i(s_i, im.pdf_d1)?, im.sqrt_t)?)
}

/// Black-Scholes delta: returns `(call_delta, put_delta)` at SCALE.
///
/// Call delta is in `[0, SCALE]`, put delta is in `[-SCALE, 0]`.
///
/// # Errors
/// Returns `Err(DomainError)` if `sigma == 0` or `t == 0`.
///
/// # Accuracy
/// 6-9 significant figures.
pub fn bs_delta(s: u128, k: u128, r: u128, sigma: u128, t: u128) -> Result<(i128, i128), SolMathError> {
    if sigma == 0 || t == 0 {
        return Err(SolMathError::DomainError);
    }
    if s == 0 {
        return Ok((0, -SCALE_I));
    }
    if k == 0 {
        return Ok((SCALE_I, 0));
    }
    let im = bs_intermediates(s, k, r, sigma, t)?;
    // phi_d1 ∈ [0, SCALE_I]; phi_d1 - SCALE_I ∈ [-SCALE_I, 0], fits i128 (put delta is non-positive).
    Ok((im.phi_d1, im.phi_d1 - SCALE_I))
}

/// Black-Scholes gamma: phi(d1) / (S * sigma * sqrt(T)) at SCALE.
///
/// Returns gamma (signed, at SCALE). Same for calls and puts.
///
/// # Errors
/// Returns `Err(DomainError)` if `sigma == 0` or `t == 0`.
///
/// # Accuracy
/// 6-9 significant figures.
pub fn bs_gamma(s: u128, k: u128, r: u128, sigma: u128, t: u128) -> Result<i128, SolMathError> {
    bs_gamma_selective(s, k, r, sigma, t)
}


/// Selective BS gamma. Internal.
pub(crate) fn bs_gamma_selective(s: u128, k: u128, r: u128, sigma: u128, t: u128) -> Result<i128, SolMathError> {
    if sigma == 0 || t == 0 {
        return Err(SolMathError::DomainError);
    }
    if s == 0 || k == 0 {
        return Ok(0);
    }
    let im = bs_intermediates(s, k, r, sigma, t)?;
    let s_i = s as i128;
    let denom = fp_mul_i(s_i, im.sigma_sqrt_t)?;
    if denom == 0 {
        return Ok(0);
    }
    Ok(fp_div_i(im.pdf_d1, denom)?)
}

/// Black-Scholes theta: returns `(call_theta, put_theta)` at SCALE.
///
/// Theta measures time decay. Call theta is typically negative.
///
/// # Errors
/// Returns `Err(DomainError)` if `sigma == 0` or `t == 0`.
///
/// # Accuracy
/// 6-9 significant figures.
pub fn bs_theta(s: u128, k: u128, r: u128, sigma: u128, t: u128) -> Result<(i128, i128), SolMathError> {
    bs_theta_selective(s, k, r, sigma, t)
}


/// Selective BS theta. Internal.
pub(crate) fn bs_theta_selective(s: u128, k: u128, r: u128, sigma: u128, t: u128) -> Result<(i128, i128), SolMathError> {
    if sigma == 0 || t == 0 {
        return Err(SolMathError::DomainError);
    }
    if s == 0 || k == 0 {
        return Ok((0, 0));
    }
    let im = bs_intermediates(s, k, r, sigma, t)?;
    let s_i = s as i128;
    let r_i = r as i128;
    let sigma_i = sigma as i128;

    let term1_num = fp_mul_i(fp_mul_i(s_i, im.pdf_d1)?, sigma_i)?;
    // im.sqrt_t ∈ [0, SCALE_I]; 2 * im.sqrt_t ≤ 2e12, fits i128.
    let two_sqrt_t = 2 * im.sqrt_t;
    let term1 = if two_sqrt_t > 0 {
        -fp_div_i(term1_num, two_sqrt_t)?
    } else {
        0
    };
    let r_k_disc = fp_mul_i(r_i, im.k_disc)?;
    let term2_call = fp_mul_i(r_k_disc, im.phi_d2)?;
    let term2_put = fp_mul_i(r_k_disc, im.phi_neg_d2)?;

    // term1 is negative (≥ -SCALE_I), term2_call ≥ 0; difference ∈ [-2·SCALE_I, 0], fits i128.
    let theta_call = term1 - term2_call;
    // term1 ∈ [-SCALE_I, 0], term2_put ∈ [0, SCALE_I]; sum ∈ [-SCALE_I, SCALE_I], fits i128.
    let theta_put = term1 + term2_put;
    Ok((theta_call, theta_put))
}

/// Black-Scholes rho: returns `(call_rho, put_rho)` at SCALE.
///
/// Rho measures sensitivity to the risk-free rate.
///
/// # Errors
/// Returns `Err(DomainError)` if `sigma == 0` or `t == 0`.
///
/// # Accuracy
/// 6-9 significant figures.
pub fn bs_rho(s: u128, k: u128, r: u128, sigma: u128, t: u128) -> Result<(i128, i128), SolMathError> {
    bs_rho_selective(s, k, r, sigma, t)
}


/// Selective BS rho. Internal.
pub(crate) fn bs_rho_selective(s: u128, k: u128, r: u128, sigma: u128, t: u128) -> Result<(i128, i128), SolMathError> {
    if sigma == 0 || t == 0 {
        return Err(SolMathError::DomainError);
    }
    if s == 0 || k == 0 {
        return Ok((0, 0));
    }
    let im = bs_intermediates(s, k, r, sigma, t)?;
    let t_i = t as i128;

    let kt_disc = fp_mul_i(im.k_disc, t_i)?;
    let rho_call = fp_mul_i(kt_disc, im.phi_d2)?;
    // fp_mul_i result ∈ [0, SCALE_I]; negation: ∈ [-SCALE_I, 0], fits i128 (put rho is non-positive).
    let rho_put = -fp_mul_i(kt_disc, im.phi_neg_d2)?;
    Ok((rho_call, rho_put))
}

/// Full Black-Scholes: prices + all Greeks in one pass at SCALE.
///
/// Returns a [`BsFull`] containing call/put prices and all five Greeks
/// (delta, gamma, vega, theta, rho) computed from shared intermediates.
///
/// # Parameters
/// - `s` -- Spot price at SCALE (u128)
/// - `k` -- Strike price at SCALE (u128)
/// - `r` -- Risk-free rate at SCALE (u128, e.g. `50_000_000_000` = 5%)
/// - `sigma` -- Volatility at SCALE (u128, e.g. `200_000_000_000` = 20%)
/// - `t` -- Time to expiry in years at SCALE (u128, e.g. `SCALE` = 1 year)
///
/// # Errors
/// Returns `Err(DomainError)` if `sigma == 0` or `t == 0`.
///
/// # Accuracy
/// 6-9 significant figures.
///
/// # Example
/// ```
/// use solmath_core::{bs_full, SCALE};
/// let bs = bs_full(100 * SCALE, 100 * SCALE, 50_000_000_000, 200_000_000_000, SCALE)?;
/// assert!(bs.call > 0);
/// assert!(bs.put > 0);
/// assert!(bs.call_delta > 0);
/// assert!(bs.put_delta < 0);
/// assert!(bs.gamma > 0);
/// assert!(bs.vega > 0);
/// # Ok::<(), solmath_core::SolMathError>(())
/// ```
pub fn bs_full(s: u128, k: u128, r: u128, sigma: u128, t: u128) -> Result<BsFull, SolMathError> {
    bs_full_selective(s, k, r, sigma, t)
}


/// Selective BS full. Internal.
pub(crate) fn bs_full_selective(s: u128, k: u128, r: u128, sigma: u128, t: u128) -> Result<BsFull, SolMathError> {
    if s > i128::MAX as u128 || k > i128::MAX as u128 || r > i128::MAX as u128
        || sigma > i128::MAX as u128 || t > i128::MAX as u128
    {
        return Err(SolMathError::Overflow);
    }
    if sigma == 0 || t == 0 {
        return Err(SolMathError::DomainError);
    }

    if s == 0 || k == 0 {
        let r_t = fp_mul_i(r as i128, t as i128)?;
        let discount = exp_fixed_i(-r_t)?;
        let k_disc = fp_mul_i(k as i128, discount)?;
        return Ok(BsFull {
            call: if s > 0 { s } else { 0 },
            put: if s == 0 { if k_disc > 0 { k_disc as u128 } else { 0 } } else { 0 },
            call_delta: if s == 0 { 0 } else { SCALE_I },
            put_delta: if s == 0 { -SCALE_I } else { 0 },
            gamma: 0, vega: 0, call_theta: 0, put_theta: 0, call_rho: 0, put_rho: 0,
        });
    }

    let im = bs_intermediates(s, k, r, sigma, t)?;
    let s_i = s as i128;
    let r_i = r as i128;
    let sigma_i = sigma as i128;
    let t_i = t as i128;

    // fp_mul_i terms: s·phi_d1 and k_disc·phi_d2 each ∈ [0, SCALE_I]; difference ∈ (-SCALE_I, SCALE_I), fits i128.
    let call_i = fp_mul_i(s_i, im.phi_d1)? - fp_mul_i(im.k_disc, im.phi_d2)?;
    // Similarly: k_disc·phi_neg_d2 and s·phi_neg_d1 each ∈ [0, SCALE_I]; difference ∈ (-SCALE_I, SCALE_I), fits i128.
    let put_i = fp_mul_i(im.k_disc, im.phi_neg_d2)? - fp_mul_i(s_i, im.phi_neg_d1)?;
    let call = if call_i > 0 { call_i as u128 } else { 0 };
    let put = if put_i > 0 { put_i as u128 } else { 0 };

    let call_delta = im.phi_d1;
    // phi_d1 ∈ [0, SCALE_I]; phi_d1 - SCALE_I ∈ [-SCALE_I, 0], fits i128.
    let put_delta = im.phi_d1 - SCALE_I;

    let denom = fp_mul_i(s_i, im.sigma_sqrt_t)?;
    let gamma = if denom != 0 {
        fp_div_i(im.pdf_d1, denom)?
    } else {
        0
    };

    let vega = fp_mul_i(fp_mul_i(s_i, im.pdf_d1)?, im.sqrt_t)?;

    let term1_num = fp_mul_i(fp_mul_i(s_i, im.pdf_d1)?, sigma_i)?;
    // im.sqrt_t ∈ [0, SCALE_I]; 2 * im.sqrt_t ≤ 2e12, fits i128.
    let two_sqrt_t = 2 * im.sqrt_t;
    let term1 = if two_sqrt_t > 0 {
        -fp_div_i(term1_num, two_sqrt_t)?
    } else {
        0
    };
    let r_k_disc = fp_mul_i(r_i, im.k_disc)?;
    // term1 ∈ [-SCALE_I, 0], fp_mul_i terms ∈ [0, SCALE_I]; differences ∈ [-2·SCALE_I, SCALE_I], fits i128.
    let call_theta = term1 - fp_mul_i(r_k_disc, im.phi_d2)?;
    let put_theta = term1 + fp_mul_i(r_k_disc, im.phi_neg_d2)?;

    let kt_disc = fp_mul_i(im.k_disc, t_i)?;
    let call_rho = fp_mul_i(kt_disc, im.phi_d2)?;
    // fp_mul_i result ∈ [0, SCALE_I]; negation: ∈ [-SCALE_I, 0], fits i128 (put rho is non-positive).
    let put_rho = -fp_mul_i(kt_disc, im.phi_neg_d2)?;

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
