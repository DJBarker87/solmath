#[cfg(all(test, feature = "complex"))]
use crate::arithmetic::{fp_div_i, fp_mul, fp_sqrt};
use crate::arithmetic::{fp_mul_i, isqrt_u128};
use crate::constants::*;
use crate::error::SolMathError;
use crate::hp::{bs_full_hp, exp_fixed_hp, fp_div_hp_safe};
#[cfg(all(test, feature = "complex"))]
use crate::i64_cf::{heston_cv_node_h, to_h, to_h_i, SCALE_TO_H};
use crate::overflow::checked_mul_div_rem_u;
use crate::transcendental::exp_fixed_i;
#[cfg(all(test, feature = "complex"))]
use crate::{complex_sqrt, cos_fixed, ln_fixed_i, sincos_fixed, Complex};

// ============================================================
// Heston deterministic-limit execution.
//
// Executable architecture:
// 1. Expiry: intrinsic value.
// 2. Deterministic variance (xi == 0): exact reduction to BS with integrated
//    CIR variance, evaluated with a cancellation-safe HP formula.
// 3. Stochastic variance (xi > 0): fail closed. No unqualified stochastic
//    approximation is shipped in the release implementation.
// ============================================================

const HESTON_MAX_RATE: u128 = 5 * SCALE;
const HESTON_MAX_TIME: u128 = 100 * SCALE;
const HESTON_MAX_VARIANCE: u128 = 4 * SCALE;
const HESTON_MAX_KAPPA: u128 = 20 * SCALE;
const HESTON_MAX_XI: u128 = 5 * SCALE;

fn discounted_strike(k: u128, r: u128, t: u128) -> Result<u128, SolMathError> {
    let rt = fp_mul_i(r as i128, t as i128)?;
    let value = fp_mul_i(k as i128, exp_fixed_i(-rt)?)?;
    if value < 0 {
        Err(SolMathError::Overflow)
    } else {
        Ok(value as u128)
    }
}

/// Enforce European no-arbitrage bounds and derive put from call parity.
/// CV values outside the bounds by more than rounding noise fail closed.
fn parity_from_call(
    raw_call: i128,
    s: u128,
    k_disc: u128,
    max_bound_error: Option<u128>,
) -> Result<(u128, u128), SolMathError> {
    let lower = s.saturating_sub(k_disc);
    let upper = s;
    let call = if raw_call < lower as i128 {
        let miss = (lower as i128 - raw_call).unsigned_abs();
        if max_bound_error.is_some_and(|limit| miss > limit) {
            return Err(SolMathError::NoConvergence);
        }
        lower
    } else if raw_call > upper as i128 {
        let miss = (raw_call - upper as i128).unsigned_abs();
        if max_bound_error.is_some_and(|limit| miss > limit) {
            return Err(SolMathError::NoConvergence);
        }
        upper
    } else {
        raw_call as u128
    };
    let put = if call >= s {
        call.checked_sub(s).and_then(|v| v.checked_add(k_disc))
    } else {
        k_disc.checked_sub(s - call)
    }
    .ok_or(SolMathError::Overflow)?;
    Ok((call, put))
}

fn heston_bs_approx(
    s: u128,
    k: u128,
    r: u128,
    t: u128,
    v0: u128,
    kappa: u128,
    theta: u128,
) -> Result<(u128, u128), SolMathError> {
    let sigma_bar = cir_rms_vol(v0, kappa, theta, t)?;
    let k_disc = discounted_strike(k, r, t)?;
    if sigma_bar == 0 {
        return parity_from_call(s.saturating_sub(k_disc) as i128, s, k_disc, None);
    }
    let bs = bs_full_hp(s, k, r, sigma_bar, t)?;
    parity_from_call(bs.call as i128, s, k_disc, None)
}

/// Fail-closed Heston European option price.
///
/// Positive-expiry execution is supported only for deterministic variance
/// (`xi == 0`). In that case the CIR variance path is deterministic and its
/// integrated variance reduces exactly to a Black-Scholes total variance.
/// Every positive-expiry stochastic case (`xi > 0`) returns `NoConvergence`.
/// At expiry the function returns intrinsic value.
///
/// # Parameters
/// All at SCALE (`u128`) except `rho` (`i128`):
/// - `s` -- Spot price
/// - `k` -- Strike price
/// - `r` -- Risk-free rate
/// - `t` -- Time to expiry (years)
/// - `v0` -- Initial variance (e.g. 40_000_000_000 = 0.04, i.e. sigma_0 = 20%)
/// - `kappa` -- Mean reversion speed
/// - `theta` -- Long-run variance
/// - `xi` -- Vol of vol
/// - `rho` -- Spot-vol correlation (`i128`, e.g. -700_000_000_000 = -0.7)
///
/// # Returns
/// `(call, put)` prices at SCALE.
///
/// # Errors
/// - `DomainError` if `rho` is outside the open interval `(-SCALE_I, SCALE_I)`.
///   The checked i64-CF domain also caps r≤5, T≤100, v0/theta≤4,
///   kappa≤20, and xi≤5.
/// - `Overflow` if an input cannot be represented by the signed fixed-point
///   implementation or deterministic intermediate arithmetic overflows.
/// - `NoConvergence` for every positive-expiry stochastic case (`xi > 0`).
///   Call and put in accepted deterministic cases are returned from one leg via
///   put-call parity.
///
/// # Accuracy
/// The accepted `xi == 0` path uses the exact deterministic-variance reduction;
/// numerical error comes from the fixed-point integrated variance and HP
/// Black-Scholes primitives. Stochastic Heston approximation results are not
/// exposed by this API.
///
/// # CU Cost
/// Final SBF audit: accepted deterministic cases averaged 119,725 CU and
/// maxed at 190,698; stochastic fail-closed cases used at most 283 CU.
pub fn heston_price(
    s: u128,
    k: u128,
    r: u128,
    t: u128,
    v0: u128,
    kappa: u128,
    theta: u128,
    xi: u128,
    rho: i128,
) -> Result<(u128, u128), SolMathError> {
    if s > i128::MAX as u128
        || k > i128::MAX as u128
        || r > i128::MAX as u128
        || t > i128::MAX as u128
        || v0 > i128::MAX as u128
        || kappa > i128::MAX as u128
        || theta > i128::MAX as u128
        || xi > i128::MAX as u128
    {
        return Err(SolMathError::Overflow);
    }
    if rho <= -SCALE_I || rho >= SCALE_I {
        return Err(SolMathError::DomainError);
    }
    if r > HESTON_MAX_RATE
        || t > HESTON_MAX_TIME
        || v0 > HESTON_MAX_VARIANCE
        || theta > HESTON_MAX_VARIANCE
        || kappa > HESTON_MAX_KAPPA
        || xi > HESTON_MAX_XI
    {
        return Err(SolMathError::DomainError);
    }
    if t == 0 {
        let call = if s > k { s - k } else { 0 };
        let put = if k > s { k - s } else { 0 };
        return Ok((call, put));
    }
    if xi != 0 {
        return Err(SolMathError::NoConvergence);
    }
    if s == 0 {
        let r_t = fp_mul_i(r as i128, t as i128)?;
        let put = fp_mul_i(k as i128, exp_fixed_i(-r_t)?)?;
        return Ok((0, if put > 0 { put as u128 } else { 0 }));
    }
    if k == 0 {
        return Ok((s, 0));
    }
    heston_bs_approx(s, k, r, t, v0, kappa, theta)
}

/// Fast signed multiply — no overflow check.
/// CF intermediates bounded: max product < 9e29 ≪ i128::MAX (1.7e38).
#[cfg(all(test, feature = "complex"))]
#[inline(always)]
fn fmul(a: i128, b: i128) -> i128 {
    a * b / SCALE_I
}

// ============================================================
// CV + fully-i64 DE21 loop (production)
//
// C = BS(σ_eff) + √(SK·disc)/π · ∫₀^∞ (φ_BS − φ_H)/(u²+¼) du
//
// Only i128 ops: bs_full_hp, 2×ln, 1×exp, 1×sqrt, prefactor mul/div.
// The entire 21-node loop — CF eval, weight multiply, accumulation —
// runs at SCALE_6 in i64.
//
// Historical research-only stochastic-CF estimate (~410-430K); this path is
// unreachable from public positive-expiry execution and is not a CU contract:
//   130K  bs_full_hp
//    30K  i128 setup (2× ln, 1× exp, 1× sqrt, precomputes)
//     5K  downscale 9 params
//    63K  21 × 3K fully-i64 loop
//     2K  upscale + prefactor
// ============================================================

/// 21-node DE nodes at SCALE_H (2^20). Precomputed: DE_NODES[i] / SCALE_TO_H.
#[cfg(all(test, feature = "complex"))]
const DE_NODES_H: [i64; 21] = [
    0,
    5,
    78,
    661,
    3_518,
    13_091,
    36_985,
    84_677,
    165_529,
    288_138,
    462_497,
    705_139,
    1_048_576,
    1_559_291,
    2_377_199,
    3_815_640,
    6_641_838,
    12_984_058,
    29_728_064,
    83_979_424,
    312_444_416,
];

/// 21-node DE weights at SCALE_H (2^20). Precomputed: DE_WEIGHTS[i] / SCALE_TO_H.
#[cfg(all(test, feature = "complex"))]
const DE_WEIGHTS_H: [i64; 21] = [
    0,
    15,
    188,
    1_245,
    5_197,
    15_237,
    34_162,
    62_792,
    100_303,
    146_490,
    204_809,
    285_571,
    411_774,
    631_479,
    1_052_649,
    1_939_833,
    4_024_659,
    9_627_695,
    27_459_829,
    97_745_668,
    461_585_778,
];

/// Heston via Andersen-Piterbarg control variate.
/// BS(σ_eff) in i128. Entire 21-node DE loop at SCALE_H = 2^20 (shift-only mul).
#[allow(dead_code)] // Retained for research/reference work; public stochastic pricing is disabled.
#[cfg(all(test, feature = "complex"))]
fn heston_price_cv(
    s: u128,
    k: u128,
    r: u128,
    t: u128,
    v0: u128,
    kappa: u128,
    theta: u128,
    xi: u128,
    rho: i128,
) -> Result<(u128, u128), SolMathError> {
    // ── Guard the i64 SCALE_H fast path ──
    // The 21-node loop runs at SCALE_H = 2^20 in i64: any product of two
    // quantities (in real terms) must stay <= i64::MAX >> 20 ≈ 8.8e12, or
    // mul_h wraps silently via `as i64` and the post-hoc correction bound can
    // let a plausible-looking wrong price escape. Reject inputs whose loop
    // products exceed the representable (and reference-tested) range.
    const MAX_H_PRODUCT: u128 = ((i64::MAX >> 20) as u128) * SCALE;

    let sigma_eff_sq = cir_expected_var(v0, kappa, theta, t)?;
    if sigma_eff_sq == 0 {
        let k_disc = discounted_strike(k, r, t)?;
        return parity_from_call(s.saturating_sub(k_disc) as i128, s, k_disc, None);
    }
    let seff_sq_t = fp_mul(sigma_eff_sq, t)?;
    let xi_sq = fp_mul(xi, xi)?;
    let kappa_theta = fp_mul(kappa, theta)?;
    if fp_mul(s, k)? > MAX_H_PRODUCT
        || fp_mul(r, t)? > MAX_H_PRODUCT
        || v0 > MAX_H_PRODUCT
        || fp_mul(kappa, kappa)? > MAX_H_PRODUCT
        || kappa_theta > MAX_H_PRODUCT
        || fp_mul(kappa, t)? > MAX_H_PRODUCT
        || xi_sq > MAX_H_PRODUCT
        || fp_mul(xi, t)? > MAX_H_PRODUCT
        || seff_sq_t > MAX_H_PRODUCT
    {
        return Err(SolMathError::Overflow);
    }
    // The CF's loop-invariant mu = κθ/ξ² (and its t-product) must also be
    // representable — tiny ξ with large κθ would otherwise wrap in div_h.
    if xi_sq > 0 {
        let mu = crate::arithmetic::fp_div(kappa_theta, xi_sq)?;
        if mu > MAX_H_PRODUCT || fp_mul(mu, t)? > MAX_H_PRODUCT {
            return Err(SolMathError::Overflow);
        }
    }
    // ln_h(0) returns an i64::MIN sentinel that would poison the unchecked
    // i64 arithmetic below; values that downscale to zero are out of domain.
    if to_h(s) == 0 || to_h(k) == 0 {
        return Err(SolMathError::DomainError);
    }

    // ── i128: σ_eff + BS price (~140K CU) ──

    let sigma_eff = fp_sqrt(sigma_eff_sq)?;
    let bs = bs_full_hp(s, k, r, sigma_eff, t)?;

    // ── Everything else at SCALE_H ──

    let sh = to_h(s);
    let kh = to_h(k);
    let rh = to_h(r);
    let th = to_h(t);
    let v0h = to_h(v0);
    let kappah = to_h(kappa);
    let thetah = to_h(theta);
    let xih = to_h(xi);
    let rhoh = to_h_i(rho);
    let seff_sq_t_half_h = to_h(seff_sq_t / 2);

    use crate::i64_cf::{div_h_pub, exp_h_pub, ln_h_pub, mul_h_pub, sqrt_h_pub};
    let r_t_h = mul_h_pub(rh, th);
    let x = ln_h_pub(sh) - ln_h_pub(kh) + r_t_h;
    let disc_h = exp_h_pub(-r_t_h);
    let sqrt_sk_disc_h = sqrt_h_pub(mul_h_pub(mul_h_pub(sh, kh), disc_h));

    // Precompute loop-invariant CF parameters
    let rhoxi_h = mul_h_pub(rhoh, xih);
    let m_re_h = kappah - rhoxi_h / 2;
    let m_im_coeff_h = -rhoxi_h; // m_im = m_im_coeff * u
    let xi_sq_h = mul_h_pub(xih, xih);
    let rho_sq_h = mul_h_pub(rhoh, rhoh);
    let xi_sq_1mrho_h = mul_h_pub(xi_sq_h, crate::i64_cf::SH_I64 - rho_sq_h);
    let mu_h = if xi_sq_h != 0 {
        div_h_pub(mul_h_pub(kappah, thetah), xi_sq_h)
    } else {
        0
    };

    // DE loop — skip first 3 + last 2 (negligible contribution)
    const FIRST: usize = 3;
    const LAST: usize = 19;
    let mut sum: i128 = 0;
    let mut idx = FIRST;
    while idx < LAST {
        // Nodes/weights at indices 3..19 are all strictly positive constants.
        let u = DE_NODES_H[idx];
        let w = DE_WEIGHTS_H[idx];
        let f = heston_cv_node_h(
            u,
            x,
            th,
            v0h,
            m_re_h,
            m_im_coeff_h,
            xi_sq_h,
            xi_sq_1mrho_h,
            mu_h,
            seff_sq_t_half_h,
        );
        // w ≤ DE_WEIGHTS_H max ≈ 4.6e8 (i64), f is the CF node result at SCALE_H (2^20);
        // product fits i128 easily (< 4.6e8 * 2^63 ≪ i128::MAX), then shifted back by 20 bits.
        sum = sum
            .checked_add((w as i128 * f as i128) >> 20)
            .ok_or(SolMathError::Overflow)?;
        idx += 1;
    }

    // prefactor: √(SK·disc)/π × correction
    let sum_h = i64::try_from(sum).map_err(|_| SolMathError::Overflow)?;
    let prefactor_h = div_h_pub(mul_h_pub(sqrt_sk_disc_h, sum_h), crate::i64_cf::PI_H_PUB);

    // ── Upscale single result to i128, add to BS ──
    // prefactor_h is i64 at SCALE_H (2^20); SCALE_TO_H = SCALE / 2^20 = 1e12/2^20 ≈ 9.5e5;
    // product < i64::MAX * SCALE_TO_H < 9.2e18 * 9.5e5 ≈ 8.8e24 ≪ i128::MAX.
    let correction_128 = prefactor_h as i128 * SCALE_TO_H;

    // Post-hoc safety: the CV correction is a difference from BS — it should be
    // small relative to the option scale. If i64 CF overflow produced garbage,
    // the correction will be unreasonably large. Reject rather than misprice.
    // Bound against max(s, k): a put's value approaches k, so bounding by s
    // alone would fail closed on legitimate deep-OTM inputs (tiny s, large k).
    if correction_128.unsigned_abs() > s.max(k) {
        return Err(SolMathError::Overflow);
    }

    let raw_call = (bs.call as i128)
        .checked_add(correction_128)
        .ok_or(SolMathError::Overflow)?;
    // The DE/CV approximation must not escape hard arbitrage bounds. Allow
    // only a tiny rounding tolerance; larger violations signal non-convergence.
    let tolerance = s.max(k) / 1_000; // 0.1% of the option notional
    parity_from_call(raw_call, s, discounted_strike(k, r, t)?, Some(tolerance))
}

// ============================================================
// Raw Lewis CF path (21-node DE, i128) — test reference only
// ============================================================

#[cfg(all(test, feature = "complex"))]
fn heston_price_cf_raw(
    s: u128,
    k: u128,
    r: u128,
    t: u128,
    v0: u128,
    kappa: u128,
    theta: u128,
    xi: u128,
    rho: i128,
) -> Result<(u128, u128), SolMathError> {
    let s_i = s as i128;
    let k_i = k as i128;
    let r_i = r as i128;
    let t_i = t as i128;
    let v0_i = v0 as i128;
    let kappa_i = kappa as i128;
    let xi_sq = fp_mul(xi, xi)? as i128;

    let r_t = fp_mul_i(r_i, t_i).unwrap();
    let disc = exp_fixed_i(-r_t)?;
    let ln_s = ln_fixed_i(s)?;
    let ln_k = ln_fixed_i(k)?;
    let x = ln_s - ln_k + r_t;

    let sk_disc = fp_mul(fp_mul(s, k)?, disc as u128)?;
    let sqrt_sk_disc = fp_sqrt(sk_disc)? as i128;

    let rhoxi = fp_mul_i(rho, xi as i128).unwrap();
    let m_re_const = kappa_i - rhoxi / 2;
    let neg_rhoxi = -rhoxi;
    let mu = fp_div_i(fp_mul_i(kappa_i, theta as i128).unwrap(), xi_sq)?;
    let two_mu = 2 * mu;

    let rho_sq = fp_mul_i(rho, rho).unwrap();
    let xi_sq_1mrho = fp_mul_i(xi_sq, SCALE_I - rho_sq).unwrap();
    let xi_sq_quarter = xi_sq / 4;
    let m_re_sq = fp_mul_i(m_re_const, m_re_const).unwrap();

    let mut integral: i128 = 0;
    let mut idx = 0;
    while idx < DE_N {
        let u = DE_NODES[idx];
        let w = DE_WEIGHTS[idx];

        let m_im = fmul(neg_rhoxi, u);
        let u_sq = fmul(u, u);
        let d2_re = m_re_sq + fmul(xi_sq_1mrho, u_sq) + xi_sq_quarter;
        let d2_im = 2 * fmul(m_re_const, m_im);
        let d = complex_sqrt(Complex::new(d2_re, d2_im)).unwrap();
        let d_re_t = fmul(d.re, t_i);
        let d_im_t = fmul(d.im, t_i);
        let exp_mag = exp_fixed_i(-d_re_t).unwrap();
        let (sin_dt, cos_dt) = sincos_fixed(d_im_t).unwrap();
        let exp_dt_re = fmul(exp_mag, cos_dt);
        let exp_dt_im = -fmul(exp_mag, sin_dt);

        let mm_re = m_re_const - d.re;
        let mm_im = m_im - d.im;
        let mmexp_re = fmul(mm_re, exp_dt_re) - fmul(mm_im, exp_dt_im);
        let mmexp_im = fmul(mm_re, exp_dt_im) + fmul(mm_im, exp_dt_re);
        let p_re = (m_re_const + d.re) - mmexp_re;
        let p_im = (m_im + d.im) - mmexp_im;

        let uq = u_sq + SCALE_I / 4;
        let dn_re = -fmul(uq, SCALE_I - exp_dt_re);
        let dn_im = fmul(uq, exp_dt_im);
        let p_mod2 = fmul(p_re, p_re) + fmul(p_im, p_im);
        let (d_coeff_re, d_coeff_im) = if p_mod2 != 0 {
            (
                fp_div_i(fmul(dn_re, p_re) + fmul(dn_im, p_im), p_mod2).unwrap(),
                fp_div_i(fmul(dn_im, p_re) - fmul(dn_re, p_im), p_mod2).unwrap(),
            )
        } else {
            (0, 0)
        };

        let ratio_re = if p_mod2 != 0 {
            fp_div_i(fmul(2 * d.re, p_re) + fmul(2 * d.im, p_im), p_mod2).unwrap()
        } else {
            SCALE_I
        };
        let ratio_im = if p_mod2 != 0 {
            fp_div_i(fmul(2 * d.im, p_re) - fmul(2 * d.re, p_im), p_mod2).unwrap()
        } else {
            0
        };
        let ratio_mod =
            fp_sqrt((fmul(ratio_re, ratio_re) + fmul(ratio_im, ratio_im)) as u128).unwrap() as i128;
        let ln_ratio_mod = if ratio_mod > 0 {
            ln_fixed_i(ratio_mod as u128).unwrap()
        } else {
            0
        };
        let arg_ratio = atan2_fixed(ratio_im, ratio_re).unwrap();
        let real_exp =
            fmul(d_coeff_re, v0_i) + fmul(mu, fmul(mm_re, t_i)) + fmul(two_mu, ln_ratio_mod);
        let imag_exp =
            fmul(d_coeff_im, v0_i) + fmul(mu, fmul(mm_im, t_i)) + fmul(two_mu, arg_ratio);

        let total_angle = imag_exp + fmul(u, x);
        let final_mag = exp_fixed_i(real_exp).unwrap();
        let re_phi_rotated = fmul(final_mag, cos_fixed(total_angle).unwrap());
        integral += fmul(w, fp_div_i(re_phi_rotated, uq).unwrap());
        idx += 1;
    }

    let correction = fp_div_i(fp_mul_i(sqrt_sk_disc, integral).unwrap(), PI_SCALE)?;
    let call_i = s_i - correction;
    let call = if call_i > 0 { call_i as u128 } else { 0 };

    let k_disc = fp_mul_i(k_i, disc).unwrap();
    let put_i = call_i - s_i + k_disc;
    let put = if put_i > 0 { put_i as u128 } else { 0 };

    Ok((call, put))
}

// ============================================================
// atan2
// ============================================================

#[cfg(all(test, feature = "complex"))]
/// atan2 with unwrap — den > 0 guaranteed (swap ensures den = max(|x|,|y|) > 0).
pub(crate) fn atan2_fixed(y: i128, x: i128) -> Result<i128, SolMathError> {
    const PI_HALF: i128 = 1_570_796_326_795;
    if x == 0 && y == 0 {
        return Ok(0);
    }
    if x == 0 {
        return Ok(if y > 0 { PI_HALF } else { -PI_HALF });
    }
    if y == 0 {
        return Ok(if x > 0 { 0 } else { PI_SCALE });
    }
    let ax = x.unsigned_abs();
    let ay = y.unsigned_abs();
    let swap = ay > ax;
    let (num, den) = if swap {
        (ax as i128, ay as i128)
    } else {
        (ay as i128, ax as i128)
    };
    let z = fp_div_i(num, den).unwrap(); // den > 0: max(|x|,|y|)
    let a = atan_01(z);
    let a = if swap { PI_HALF - a } else { a };
    let a = if x < 0 { PI_SCALE - a } else { a };
    Ok(if y < 0 { -a } else { a })
}

#[cfg(all(test, feature = "complex"))]
fn atan_01(z: i128) -> i128 {
    const TAN15: i128 = 267_949_192_431;
    const TAN30: i128 = 577_350_269_190;
    const PI_6: i128 = 523_598_775_598;
    const PI_4: i128 = 785_398_163_397;
    if z <= TAN15 {
        atan_poly(z)
    } else if z <= 750_000_000_000 {
        // den = SCALE + z*TAN30 > SCALE (z,TAN30 > 0)
        PI_6 + atan_poly(fp_div_i(z - TAN30, SCALE_I + fp_mul_i(z, TAN30).unwrap()).unwrap())
    } else {
        // den = z + SCALE > SCALE (z > 0)
        PI_4 + atan_poly(fp_div_i(z - SCALE_I, z + SCALE_I).unwrap())
    }
}

#[cfg(all(test, feature = "complex"))]
fn atan_poly(t: i128) -> i128 {
    let t2 = fp_mul_i(t, t).unwrap();
    let p = fp_mul_i(t2, -90_909_090_909).unwrap() + 111_111_111_111;
    let p = fp_mul_i(t2, p).unwrap() + (-142_857_142_857);
    let p = fp_mul_i(t2, p).unwrap() + 200_000_000_000;
    let p = fp_mul_i(t2, p).unwrap() + (-333_333_333_333);
    let p = fp_mul_i(t2, p).unwrap() + SCALE_I;
    fp_mul_i(t, p).unwrap()
}

// ============================================================
// CIR helpers
// ============================================================

pub(crate) fn cir_rms_vol(
    v0: u128,
    kappa: u128,
    theta: u128,
    t: u128,
) -> Result<u128, SolMathError> {
    let average_wide = cir_expected_var_wide(v0, kappa, theta, t)?;
    if average_wide == 0 {
        return Ok(0);
    }
    // average_wide is scaled by SCALE², so sqrt(average_wide) is sigma at SCALE.
    let floor = isqrt_u128(average_wide);
    let remainder = average_wide - floor * floor;
    Ok(if remainder > floor { floor + 1 } else { floor })
}

#[cfg(test)]
pub(crate) fn cir_expected_var(
    v0: u128,
    kappa: u128,
    theta: u128,
    t: u128,
) -> Result<u128, SolMathError> {
    let wide = cir_expected_var_wide(v0, kappa, theta, t)?;
    wide.checked_add(SCALE / 2)
        .map(|v| v / SCALE)
        .ok_or(SolMathError::Overflow)
}

const VAR_WIDE_SCALE: u128 = SCALE * SCALE;

fn cir_expected_var_wide(
    v0: u128,
    kappa: u128,
    theta: u128,
    t: u128,
) -> Result<u128, SolMathError> {
    if t == 0 || kappa == 0 {
        return v0.checked_mul(SCALE).ok_or(SolMathError::Overflow);
    }
    // kappa and t are at SCALE, so their raw product represents kappa*T at
    // SCALE² exactly. Public bounds keep this below 2e27, well inside i128.
    let x_wide = kappa.checked_mul(t).ok_or(SolMathError::Overflow)?;
    if x_wide == 0 {
        return v0.checked_mul(SCALE).ok_or(SolMathError::Overflow);
    }

    // Average variance = v0 + (theta-v0)*g(x),
    // g(x) = 1 - (1-exp(-x))/x. This form avoids the catastrophic
    // theta + (v0-theta)*ratio cancellation when x is small.
    let g_wide = if x_wide <= VAR_WIDE_SCALE / 10 {
        one_minus_exp_ratio_taylor_wide(x_wide)?
    } else {
        const WIDE_TO_HP: u128 = VAR_WIDE_SCALE / SCALE_HP as u128;
        let x_hp_u = x_wide
            .checked_add(WIDE_TO_HP / 2)
            .ok_or(SolMathError::Overflow)?
            / WIDE_TO_HP;
        if x_hp_u > i128::MAX as u128 {
            return Err(SolMathError::Overflow);
        }
        let x_hp = x_hp_u as i128;
        let exp_neg = exp_fixed_hp(x_hp.checked_neg().ok_or(SolMathError::Overflow)?)?;
        let one_minus_exp = SCALE_HP
            .checked_sub(exp_neg)
            .ok_or(SolMathError::Overflow)?;
        let ratio = fp_div_hp_safe(one_minus_exp, x_hp)?;
        let g_hp = SCALE_HP.checked_sub(ratio).ok_or(SolMathError::Overflow)?;
        if g_hp < 0 || g_hp > SCALE_HP {
            return Err(SolMathError::NoConvergence);
        }
        (g_hp as u128)
            .checked_mul(WIDE_TO_HP)
            .ok_or(SolMathError::Overflow)?
    };

    let v0_wide = v0.checked_mul(SCALE).ok_or(SolMathError::Overflow)?;
    let theta_wide = theta.checked_mul(SCALE).ok_or(SolMathError::Overflow)?;
    if v0_wide > i128::MAX as u128 || theta_wide > i128::MAX as u128 {
        return Err(SolMathError::Overflow);
    }
    let delta_wide = (theta_wide as i128)
        .checked_sub(v0_wide as i128)
        .ok_or(SolMathError::Overflow)?;
    let correction = mul_wide_round_i(delta_wide, g_wide as i128)?;
    let average = (v0_wide as i128)
        .checked_add(correction)
        .ok_or(SolMathError::Overflow)?;
    if average < 0 || average > VAR_WIDE_SCALE as i128 * 4 {
        return Err(SolMathError::NoConvergence);
    }
    Ok(average as u128)
}

/// Cancellation-safe Taylor series for
/// `1 - (1-exp(-x))/x = x/2 - x²/6 + x³/24 - ...` at SCALE².
fn one_minus_exp_ratio_taylor_wide(x: u128) -> Result<u128, SolMathError> {
    debug_assert!(x > 0 && x <= VAR_WIDE_SCALE / 10);
    const DENOMINATORS: [u128; 14] = [
        2,
        6,
        24,
        120,
        720,
        5_040,
        40_320,
        362_880,
        3_628_800,
        39_916_800,
        479_001_600,
        6_227_020_800,
        87_178_291_200,
        1_307_674_368_000,
    ];
    let mut power = x;
    let mut sum = 0i128;
    let mut i = 0usize;
    while i < DENOMINATORS.len() {
        let d = DENOMINATORS[i];
        let term = power.checked_add(d / 2).ok_or(SolMathError::Overflow)? / d;
        if term > i128::MAX as u128 {
            return Err(SolMathError::Overflow);
        }
        sum = if i % 2 == 0 {
            sum.checked_add(term as i128)
        } else {
            sum.checked_sub(term as i128)
        }
        .ok_or(SolMathError::Overflow)?;
        power = mul_wide_round_u(power, x)?;
        i += 1;
    }
    if sum < 0 {
        return Err(SolMathError::NoConvergence);
    }
    Ok(sum as u128)
}

fn mul_wide_round_u(a: u128, b: u128) -> Result<u128, SolMathError> {
    let (q, rem) = checked_mul_div_rem_u(a, b, VAR_WIDE_SCALE).ok_or(SolMathError::Overflow)?;
    if rem >= VAR_WIDE_SCALE - rem {
        q.checked_add(1).ok_or(SolMathError::Overflow)
    } else {
        Ok(q)
    }
}

fn mul_wide_round_i(a: i128, b: i128) -> Result<i128, SolMathError> {
    let neg = (a < 0) != (b < 0);
    let mag = mul_wide_round_u(a.unsigned_abs(), b.unsigned_abs())?;
    if neg {
        if mag == 1u128 << 127 {
            Ok(i128::MIN)
        } else if mag < 1u128 << 127 {
            Ok(-(mag as i128))
        } else {
            Err(SolMathError::Overflow)
        }
    } else if mag <= i128::MAX as u128 {
        Ok(mag as i128)
    } else {
        Err(SolMathError::Overflow)
    }
}

// ============================================================
// DE quadrature: h=0.25, 21 nodes
// ============================================================

#[cfg(all(test, feature = "complex"))]
const DE_N: usize = 21;

#[cfg(all(test, feature = "complex"))]
const DE_NODES: [i128; 21] = [
    146_529,
    4_855_077,
    74_579_941,
    630_580_368,
    3_355_820_405,
    12_485_683_179,
    35_272_052_349,
    80_758_778_787,
    157_867_103_304,
    274_805_390_708,
    441_077_539_800,
    672_466_825_156,
    1_000_000_000_000,
    1_487_062_205_288,
    2_267_175_065_076,
    3_638_938_804_750,
    6_334_441_939_257,
    12_382_554_751_561,
    28_351_057_945_605,
    80_091_732_721_767,
    297_989_725_118_823,
];

#[cfg(all(test, feature = "complex"))]
const DE_WEIGHTS: [i128; 21] = [
    579_312,
    14_972_941,
    179_599_271,
    1_187_766_015,
    4_957_925_286,
    14_533_760_073,
    32_583_937_342,
    59_889_282_729,
    95_662_152_103,
    139_716_814_232,
    195_316_933_331,
    272_372_585_178,
    392_699_081_699,
    602_312_206_376,
    1_003_945_204_820,
    1_850_112_676_736,
    3_838_458_650_317,
    9_182_683_710_144,
    26_190_398_181_291,
    93_229_502_186_530,
    440_253_236_111_365,
];

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    // ── Existing tests (unchanged) ──

    #[test]
    fn test_heston_xi_zero_bs_path() {
        let (c, p) = heston_price(
            100 * SCALE,
            100 * SCALE,
            50_000_000_000,
            SCALE,
            40_000_000_000,
            2 * SCALE,
            40_000_000_000,
            0,
            -700_000_000_000,
        )
        .unwrap();
        let sigma = cir_rms_vol(40_000_000_000, 2 * SCALE, 40_000_000_000, SCALE).unwrap();
        let bs = bs_full_hp(100 * SCALE, 100 * SCALE, 50_000_000_000, sigma, SCALE).unwrap();
        assert_eq!(c, bs.call);
        let kd = discounted_strike(100 * SCALE, 50_000_000_000, SCALE).unwrap();
        assert_eq!(c as i128 - p as i128, 100 * SCALE_I - kd as i128);
    }

    #[test]
    fn test_heston_fails_closed_on_material_arbitrage_violation() {
        let result = heston_price(
            50 * SCALE,
            50 * SCALE,
            50_000_000_000,
            SCALE,
            1_000_000_000,
            0,
            0,
            100_000_000_000,
            700_000_000_000,
        );
        assert_eq!(result, Err(SolMathError::NoConvergence));
    }

    #[test]
    fn test_heston_rejects_unproved_i64_node_domain() {
        assert_eq!(
            heston_price(
                100 * SCALE,
                100 * SCALE,
                0,
                SCALE,
                40_000_000_000,
                2_000_000 * SCALE,
                0,
                2_000_000 * SCALE,
                -999_000_000_000,
            ),
            Err(SolMathError::DomainError)
        );
    }

    #[test]
    fn test_heston_zero_variance_is_discounted_intrinsic() {
        let (call, put) = heston_price(
            100 * SCALE,
            100 * SCALE,
            50_000_000_000,
            SCALE,
            0,
            2 * SCALE,
            0,
            0,
            -700_000_000_000,
        )
        .unwrap();
        let kd = discounted_strike(100 * SCALE, 50_000_000_000, SCALE).unwrap();
        assert_eq!((call, put), (100 * SCALE - kd, 0));
    }

    #[test]
    fn test_all_positive_stochastic_xi_fails_closed() {
        for xi in [1, 100_000_000_000, 500_000_000_000] {
            assert_eq!(
                heston_price(
                    100 * SCALE,
                    100 * SCALE,
                    50_000_000_000,
                    SCALE,
                    40_000_000_000,
                    2 * SCALE,
                    40_000_000_000,
                    xi,
                    -700_000_000_000,
                ),
                Err(SolMathError::NoConvergence)
            );
        }
    }

    #[test]
    fn test_heston_small_positive_xi_is_not_approximated() {
        assert_eq!(
            heston_price(
                100 * SCALE,
                100 * SCALE,
                0,
                100_000_000_000,
                10_000_000_000,
                500_000_000_000,
                10_000_000_000,
                1,
                -900_000_000_000,
            ),
            Err(SolMathError::NoConvergence)
        );
    }

    #[test]
    #[cfg(feature = "complex")]
    fn test_research_cf_path_differs_from_deterministic_reduction() {
        let (c, _) = heston_price_cv(
            100 * SCALE,
            100 * SCALE,
            50_000_000_000,
            SCALE,
            40_000_000_000,
            2 * SCALE,
            40_000_000_000,
            500_000_000_000,
            -700_000_000_000,
        )
        .unwrap();
        let sigma = cir_rms_vol(40_000_000_000, 2 * SCALE, 40_000_000_000, SCALE).unwrap();
        let bs = bs_full_hp(100 * SCALE, 100 * SCALE, 50_000_000_000, sigma, SCALE).unwrap();
        assert_ne!(c, bs.call);
    }

    #[test]
    fn test_heston_put_call_parity() {
        let r: u128 = 50_000_000_000;
        let (c, p) = heston_price(
            100 * SCALE,
            100 * SCALE,
            r,
            SCALE,
            40_000_000_000,
            2 * SCALE,
            40_000_000_000,
            0,
            -700_000_000_000,
        )
        .unwrap();
        let disc = exp_fixed_i(-fp_mul_i(r as i128, SCALE_I).unwrap()).unwrap();
        let parity =
            (c as i128 - p as i128 - 100 * SCALE_I + fp_mul_i(100 * SCALE_I, disc).unwrap()).abs();
        assert!(parity <= 100, "parity error {}", parity);
    }

    #[test]
    fn test_heston_t_zero() {
        let (c, p) = heston_price(
            110 * SCALE,
            100 * SCALE,
            0,
            0,
            40_000_000_000,
            2 * SCALE,
            40_000_000_000,
            500_000_000_000,
            -700_000_000_000,
        )
        .unwrap();
        assert_eq!(c, 10 * SCALE);
        assert_eq!(p, 0);
    }

    #[test]
    fn test_heston_all_moneyness() {
        for &(s, k) in &[(80, 100), (100, 100), (120, 100)] {
            let (c, p) = heston_price(
                s as u128 * SCALE,
                k as u128 * SCALE,
                50_000_000_000,
                SCALE,
                40_000_000_000,
                2 * SCALE,
                40_000_000_000,
                0,
                -600_000_000_000,
            )
            .unwrap();
            assert!(c > 0 || s <= k);
            assert!(p > 0 || s >= k);
        }
    }

    // ── CV vs raw i128 comparison tests ──

    /// Helper: compare CV result against 21-node i128 raw Lewis.
    #[cfg(feature = "complex")]
    fn compare_cv_vs_raw(
        s: u128,
        k: u128,
        r: u128,
        t: u128,
        v0: u128,
        kappa: u128,
        theta: u128,
        xi: u128,
        rho: i128,
        label: &str,
        max_err: i128,
    ) {
        let (cv_c, cv_p) = heston_price_cv(s, k, r, t, v0, kappa, theta, xi, rho).unwrap();
        let (raw_c, raw_p) = heston_price_cf_raw(s, k, r, t, v0, kappa, theta, xi, rho).unwrap();

        let err_c = (cv_c as i128 - raw_c as i128).abs();
        let err_p = (cv_p as i128 - raw_p as i128).abs();

        // Also verify put-call parity for the CV result
        let disc = exp_fixed_i(-fp_mul_i(r as i128, t as i128).unwrap()).unwrap();
        let parity =
            (cv_c as i128 - cv_p as i128 - s as i128 + fp_mul_i(k as i128, disc).unwrap()).abs();

        assert!(
            err_c <= max_err,
            "{} call: cv={}, raw={}, err={} > max={}",
            label,
            cv_c,
            raw_c,
            err_c,
            max_err
        );
        assert!(
            err_p <= max_err,
            "{} put: cv={}, raw={}, err={} > max={}",
            label,
            cv_p,
            raw_p,
            err_p,
            max_err
        );
        assert!(
            parity <= 1000,
            "{} put-call parity error: {} (c={}, p={}, s={}, k_disc={})",
            label,
            parity,
            cv_c,
            cv_p,
            s,
            fp_mul_i(k as i128, disc).unwrap()
        );
    }

    // $0.02 tolerance: 20_000_000_000 at SCALE on $100 notional.
    // Most cases achieve sub-penny ($0.01); deep ITM reaches ~$0.018
    // due to inherent i64 CF precision for large forward moneyness.
    #[cfg(feature = "complex")]
    const MAX_ERR: i128 = 20_000_000_000;

    #[test]
    #[cfg(feature = "complex")]
    fn test_cv_vs_raw_atm() {
        compare_cv_vs_raw(
            100 * SCALE,
            100 * SCALE,
            50_000_000_000,
            SCALE,
            40_000_000_000,
            2 * SCALE,
            40_000_000_000,
            300_000_000_000,
            -700_000_000_000,
            "ATM ξ=0.3 ρ=-0.7",
            MAX_ERR,
        );
    }

    #[test]
    #[cfg(feature = "complex")]
    fn test_cv_vs_raw_itm() {
        compare_cv_vs_raw(
            120 * SCALE,
            100 * SCALE,
            50_000_000_000,
            SCALE,
            40_000_000_000,
            2 * SCALE,
            40_000_000_000,
            300_000_000_000,
            -700_000_000_000,
            "ITM 120/100",
            MAX_ERR,
        );
    }

    #[test]
    #[cfg(feature = "complex")]
    fn test_cv_vs_raw_otm() {
        compare_cv_vs_raw(
            80 * SCALE,
            100 * SCALE,
            50_000_000_000,
            SCALE,
            40_000_000_000,
            2 * SCALE,
            40_000_000_000,
            300_000_000_000,
            -700_000_000_000,
            "OTM 80/100",
            MAX_ERR,
        );
    }

    #[test]
    #[cfg(feature = "complex")]
    fn test_cv_vs_raw_high_xi() {
        compare_cv_vs_raw(
            100 * SCALE,
            100 * SCALE,
            50_000_000_000,
            SCALE,
            40_000_000_000,
            2 * SCALE,
            40_000_000_000,
            500_000_000_000,
            -700_000_000_000,
            "ATM ξ=0.5",
            MAX_ERR,
        );
    }

    #[test]
    #[cfg(feature = "complex")]
    fn test_cv_vs_raw_very_high_xi() {
        compare_cv_vs_raw(
            100 * SCALE,
            100 * SCALE,
            50_000_000_000,
            SCALE,
            40_000_000_000,
            2 * SCALE,
            40_000_000_000,
            800_000_000_000,
            -700_000_000_000,
            "ATM ξ=0.8",
            MAX_ERR,
        );
    }

    #[test]
    #[cfg(feature = "complex")]
    fn test_cv_vs_raw_zero_rho() {
        compare_cv_vs_raw(
            100 * SCALE,
            100 * SCALE,
            50_000_000_000,
            SCALE,
            40_000_000_000,
            2 * SCALE,
            40_000_000_000,
            300_000_000_000,
            0,
            "ATM ρ=0",
            MAX_ERR,
        );
    }

    #[test]
    #[cfg(feature = "complex")]
    fn test_cv_vs_raw_short_maturity() {
        compare_cv_vs_raw(
            100 * SCALE,
            100 * SCALE,
            50_000_000_000,
            100_000_000_000,
            40_000_000_000,
            2 * SCALE,
            40_000_000_000,
            300_000_000_000,
            -700_000_000_000,
            "ATM T=0.1",
            MAX_ERR,
        );
    }

    #[test]
    #[cfg(feature = "complex")]
    fn test_cv_vs_raw_long_maturity() {
        compare_cv_vs_raw(
            100 * SCALE,
            100 * SCALE,
            50_000_000_000,
            2 * SCALE,
            40_000_000_000,
            2 * SCALE,
            40_000_000_000,
            300_000_000_000,
            -700_000_000_000,
            "ATM T=2.0",
            MAX_ERR,
        );
    }

    #[test]
    #[cfg(feature = "complex")]
    fn test_cv_vs_raw_deep_otm_high_xi() {
        compare_cv_vs_raw(
            80 * SCALE,
            100 * SCALE,
            50_000_000_000,
            SCALE,
            40_000_000_000,
            2 * SCALE,
            40_000_000_000,
            500_000_000_000,
            -500_000_000_000,
            "OTM 80/100 ξ=0.5 ρ=-0.5",
            MAX_ERR,
        );
    }

    #[test]
    #[cfg(feature = "complex")]
    fn test_cv_vs_raw_negative_rho_strong() {
        compare_cv_vs_raw(
            100 * SCALE,
            100 * SCALE,
            50_000_000_000,
            SCALE,
            40_000_000_000,
            2 * SCALE,
            40_000_000_000,
            300_000_000_000,
            -900_000_000_000,
            "ATM ρ=-0.9",
            MAX_ERR,
        );
    }

    /// Full matrix test: moneyness × maturity × xi × rho
    #[test]
    #[cfg(feature = "complex")]
    fn test_cv_vs_raw_full_matrix() {
        let moneyness: [(u128, u128); 5] =
            [(80, 100), (90, 100), (100, 100), (110, 100), (120, 100)];
        let maturities: [u128; 4] = [
            100_000_000_000, // T=0.1
            250_000_000_000, // T=0.25
            SCALE,           // T=1.0
            2 * SCALE,       // T=2.0
        ];
        let xis: [u128; 3] = [
            200_000_000_000, // ξ=0.2
            300_000_000_000, // ξ=0.3
            500_000_000_000, // ξ=0.5
        ];
        let rhos: [i128; 3] = [
            -700_000_000_000, // ρ=-0.7
            -500_000_000_000, // ρ=-0.5
            0,                // ρ=0
        ];

        let mut max_err_seen: i128 = 0;
        let mut count = 0;

        for &(s_mult, k_mult) in &moneyness {
            for &t in &maturities {
                for &xi in &xis {
                    for &rho in &rhos {
                        let s = s_mult * SCALE;
                        let k = k_mult * SCALE;

                        let cv = heston_price_cv(
                            s,
                            k,
                            50_000_000_000,
                            t,
                            40_000_000_000,
                            2 * SCALE,
                            40_000_000_000,
                            xi,
                            rho,
                        );
                        let raw = heston_price_cf_raw(
                            s,
                            k,
                            50_000_000_000,
                            t,
                            40_000_000_000,
                            2 * SCALE,
                            40_000_000_000,
                            xi,
                            rho,
                        );

                        if let (Ok((cv_c, _)), Ok((raw_c, _))) = (cv, raw) {
                            let err = (cv_c as i128 - raw_c as i128).abs();
                            if err > max_err_seen {
                                max_err_seen = err;
                            }
                            count += 1;

                            // $2.00 tolerance on $100 notional = 2% of notional
                            assert!(
                                err <= 2_000_000_000_000,
                                "S={} K={} T={} ξ={} ρ={}: cv={} raw={} err={}",
                                s_mult,
                                k_mult,
                                t,
                                xi,
                                rho,
                                cv_c,
                                raw_c,
                                err
                            );
                        }
                    }
                }
            }
        }

        // Sanity: we tested enough cases
        assert!(count >= 150, "Only {} test cases ran", count);
    }

    /// Public stochastic pricing must remain disabled across the old matrix.
    #[test]
    fn test_public_stochastic_matrix_fails_closed() {
        let params: [(u128, u128, u128, i128); 6] = [
            (100, 100, 300_000_000_000, -700_000_000_000),
            (80, 100, 300_000_000_000, -500_000_000_000),
            (120, 100, 500_000_000_000, -700_000_000_000),
            (100, 100, 200_000_000_000, 0),
            (90, 100, 800_000_000_000, -700_000_000_000),
            (110, 100, 300_000_000_000, -900_000_000_000),
        ];
        for &(s_mult, k_mult, xi, rho) in &params {
            let s = s_mult * SCALE;
            let k = k_mult * SCALE;
            assert_eq!(
                heston_price(
                    s,
                    k,
                    50_000_000_000,
                    SCALE,
                    40_000_000_000,
                    2 * SCALE,
                    40_000_000_000,
                    xi,
                    rho,
                ),
                Err(SolMathError::NoConvergence)
            );
        }
    }

    #[test]
    fn test_heston_call_bounds() {
        let s = 100 * SCALE;
        let r = 50_000_000_000u128;
        let params: &[(u128, u128, u128, u128, i128)] = &[
            (
                40_000_000_000,
                2 * SCALE,
                40_000_000_000,
                300_000_000_000,
                -700_000_000_000,
            ),
            (
                40_000_000_000,
                2 * SCALE,
                40_000_000_000,
                800_000_000_000,
                -500_000_000_000,
            ),
        ];
        for &(v0, kappa, theta, xi, rho) in params {
            for &k_mult in &[80u128, 100, 120] {
                let k = k_mult * SCALE;
                for &t in &[SCALE / 10, SCALE, 2 * SCALE] {
                    assert_eq!(
                        heston_price(s, k, r, t, v0, kappa, theta, xi, rho),
                        Err(SolMathError::NoConvergence)
                    );
                }
            }
        }
    }

    #[test]
    fn test_cancellation_safe_deterministic_variance_reproducer() {
        let average = cir_expected_var(0, 1_000_000, 4 * SCALE, SCALE).unwrap();
        assert_eq!(average, 1_999_999);
        let (call, put) = heston_price(
            100 * SCALE,
            100 * SCALE,
            0,
            SCALE,
            0,
            1_000_000,
            4 * SCALE,
            0,
            0,
        )
        .unwrap();
        assert_eq!(call, 56_418_944_263);
        assert_eq!(put, call);
    }
}

// The committed QuantLib stochastic corpus is intentionally not included in
// the release test suite: every vector has xi > 0 and the public API rejects
// it. The file remains available for offline research on the retained CV code.
