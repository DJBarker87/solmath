use crate::constants::*;
use crate::error::SolMathError;
use crate::arithmetic::{fp_mul_i, fp_div_i, fp_sqrt};
use crate::transcendental::{ln_fixed_i, exp_fixed_i};
use crate::trig::{cos_fixed, sin_fixed};
use crate::complex::{Complex, complex_mul, complex_sqrt, complex_exp};

/// NIG characteristic function φ(u). Internal — called by nig_call_price COS loop.
pub(crate) fn nig_char_func(
    u: i128,
    drift: i128,
    delta_t: i128,
    gamma: i128,
    alpha_sq: i128,
    beta: i128,
) -> Result<Complex, SolMathError> {
    // α² − (β+iu)² = (α²−β²+u², −2βu)
    let u_sq = fp_mul_i(u, u)?;
    let beta_sq = fp_mul_i(beta, beta)?;
    let inner = complex_sqrt(Complex::new(
        alpha_sq - beta_sq + u_sq,
        -2 * fp_mul_i(beta, u)?,
    ))?;

    // Exponent: iu·drift + δT·(γ − inner)
    let exponent = Complex::new(
        fp_mul_i(delta_t, gamma - inner.re)?,
        fp_mul_i(u, drift)? - fp_mul_i(delta_t, inner.im)?,
    );
    complex_exp(exponent)
}

/// Offline/high-precision NIG call price via COS method (17 terms, i128 arithmetic).
/// ~302K CU native, exceeds on-chain budget with Anchor overhead.
/// For on-chain use, see `nig_call_64`.
///
/// # Errors
/// - `DomainError` if s/k/alpha/delta == 0 or α ≤ |β| or α ≤ |β+1|.
///
/// # Precision
/// 95% within 0.5% of reference prices for α ≥ 10, prices > $1.
///
/// # CU cost
/// ~302,000 CU (native only — exceeds on-chain limits).
pub fn nig_call_price(
    s: u128,
    k: u128,
    r: u128,
    t: u128,
    alpha: u128,
    beta: i128,
    delta: u128,
) -> Result<u128, SolMathError> {
    if s > i128::MAX as u128 || k > i128::MAX as u128 || r > i128::MAX as u128
        || t > i128::MAX as u128 || alpha > i128::MAX as u128 || delta > i128::MAX as u128
    {
        return Err(SolMathError::Overflow);
    }
    if s == 0 || k == 0 || alpha == 0 || delta == 0 {
        return Err(SolMathError::DomainError);
    }
    // Domain: alpha ≤ 10,000. Real NIG calibrations on equity markets have alpha in [1, 100].
    // This guard prevents complex arithmetic overflow in nig_char_func (see pen test audit).
    if alpha > 10_000 * SCALE {
        return Err(SolMathError::DomainError);
    }

    let alpha_i = alpha as i128;
    let delta_i = delta as i128;
    let r_i = r as i128;
    let t_i = t as i128;
    // NIG parameters
    let alpha_sq = fp_mul_i(alpha_i, alpha_i)?;
    let beta_sq = fp_mul_i(beta, beta)?;

    // Domain check: NIG requires α > |β| and α > |β+1|
    if alpha_sq <= beta_sq {
        return Err(SolMathError::DomainError); // invalid: |β| ≥ α
    }
    let gamma = fp_sqrt((alpha_sq - beta_sq) as u128)? as i128;
    let gamma_cu = fp_mul_i(fp_mul_i(gamma, gamma)?, gamma)?; // γ³

    // Convexity correction: ω = δ·(γ − √(α²−(β+1)²))
    let bp1 = beta + SCALE_I;
    let bp1_sq = fp_mul_i(bp1, bp1)?;
    if alpha_sq <= bp1_sq {
        return Err(SolMathError::DomainError); // invalid: |β+1| ≥ α
    }
    let omega = fp_mul_i(
        delta_i,
        gamma - fp_sqrt((alpha_sq - bp1_sq) as u128)? as i128,
    )?;

    // NIG mean and variance of log-price over period T
    // c1 = ln(S) + (r−ω)T + δTβ/γ
    let ln_s = ln_fixed_i(s)?;
    let drift_rate = r_i - omega;
    let c1 =
        ln_s + fp_mul_i(drift_rate, t_i)? + fp_div_i(fp_mul_i(fp_mul_i(delta_i, t_i)?, beta)?, gamma)?;
    // c2 = δTα²/γ³
    let c2 = fp_div_i(fp_mul_i(fp_mul_i(delta_i, t_i)?, alpha_sq)?, gamma_cu)?;
    let nig_std = fp_sqrt(c2 as u128)? as i128;
    // Truncation range [a, b]
    let log_k = ln_fixed_i(k)?;
    let l_std = fp_mul_i(NIG_COS_L, nig_std)?;
    let mut a = c1 - l_std;
    let mut b = c1 + l_std;
    // Extend to cover strike with 1-std margin
    if log_k - nig_std < a {
        a = log_k - nig_std;
    }
    if log_k + nig_std > b {
        b = log_k + nig_std;
    }
    let ba = b - a;

    let discount = exp_fixed_i(-fp_mul_i(r_i, t_i)?)?;
    let exp_b = exp_fixed_i(b)?; // exp(b)
                                // Precompute drift for char func: ln(S) + (r−ω)T
    let cf_drift = ln_s + fp_mul_i(drift_rate, t_i)?;
    let delta_t = fp_mul_i(delta_i, t_i)?;

    // COS expansion: Σ_{k=0}^{N-1} ' Re[φ(kπ/(b-a)) · e^{-ikπa/(b-a)}] · V_k
    // where ' means k=0 term halved
    let mut total: i128 = 0;
    let mut i = 0;
    while i < NIG_COS_N {
        // Frequency: w = i·π / (b-a)  — but i is the loop counter, use as integer
        // w_ba_num = i (integer), w_ba_den = ba/π
        // In SCALE: w = i * PI_SCALE / ba  (but careful about overflow)
        let w = if i == 0 {
            0i128
        } else {
            // i * π / (b-a): compute as fp_div_i(i * PI_SCALE, ba)
            fp_div_i((i as i128) * PI_SCALE, ba)?
        };

        // Characteristic function term
        let char_term = if i == 0 {
            SCALE_I // φ(0) = 1
        } else {
            // Re[φ(w) · exp(-i·w·a)]
            let phi = nig_char_func(w, cf_drift, delta_t, gamma, alpha_sq, beta)?;
            // exp(-i·w·a) = cos(w·a) − i·sin(w·a)
            let wa = fp_mul_i(w, a)?;
            let rot = Complex::new(cos_fixed(wa)?, -sin_fixed(wa)?);
            complex_mul(phi, rot)?.re
        };

        // Payoff coefficients V_k for call: 2/(b-a) × (χ_k − K·ψ_k)
        // where c = ln(K), d = b
        // For k=0: χ = exp(b) − K,  ψ = b − ln(K)
        // For k>0: sin(kπ(d−a)/(b−a)) = sin(kπ) = 0, cos(kπ) = (−1)^k
        //   θ = w·(ln(K)−a)
        //   χ = ((−1)^k·exp(b) − K·(cos(θ)+w·sin(θ))) / (1+w²)
        //   ψ = −sin(θ)/w
        let v_k = if i == 0 {
            // 2/(b-a) × (exp(b) − K − K·(b − ln(K)))
            let chi = exp_b as i128 - (k as i128);
            let psi = b - log_k;
            fp_div_i(2 * (chi - fp_mul_i(k as i128, psi)?), ba)?
        } else {
            let theta = fp_mul_i(w, log_k - a)?;
            let cos_t = cos_fixed(theta)?;
            let sin_t = sin_fixed(theta)?;
            let w_sq = fp_mul_i(w, w)?;
            let sign_k: i128 = if i % 2 == 0 { 1 } else { -1 };
            let chi = fp_div_i(
                sign_k * (exp_b as i128) - fp_mul_i(k as i128, cos_t + fp_mul_i(w, sin_t)?)?,
                SCALE_I + w_sq,
            )?;
            let psi = -fp_div_i(sin_t, w)?;
            fp_div_i(2 * (chi - fp_mul_i(k as i128, psi)?), ba)?
        };

        let weight: i128 = if i == 0 { SCALE_I / 2 } else { SCALE_I };
        total += fp_mul_i(weight, fp_mul_i(char_term, v_k)?)?;
        i += 1;
    }

    let call_i = fp_mul_i(discount, total)?;
    Ok(if call_i > 0 {
        call_i as u128
    } else {
        0
    })
}
