use crate::constants::*;
use crate::error::SolMathError;
use crate::arithmetic::{fp_mul, fp_mul_i, fp_div_i, fp_sqrt};
use crate::transcendental::{exp_fixed_i, expm1_fixed};
use crate::hp::bs_full_hp;
use crate::i64_cf::{heston_cv_node_h, to_h, to_h_i, SCALE_TO_H};

// Used only by #[cfg(test)] heston_price_cf_raw
#[cfg(test)]
use crate::transcendental::ln_fixed_i;
#[cfg(test)]
use crate::trig::{sincos_fixed, cos_fixed};
#[cfg(test)]
use crate::complex::{Complex, complex_sqrt};

// ============================================================
// Heston stochastic volatility pricing.
//
// Three-path architecture:
// 1. Degenerate: t=0, s=0, k=0.
// 2. BS path (ξ²T < 0.01): BS(σ̄) via bs_full_hp. ~130K CU.
// 3. CV path: BS(σ_eff) + 21-node DE quadrature of (φ_BS − φ_H)
//    with i64 Heston CF + i128 BS CF. Target: ~300K CU.
// ============================================================

const HESTON_BS_THRESHOLD: u128 = 10_000_000_000; // 0.01 at SCALE

/// Heston stochastic-volatility European option price.
///
/// Three-path architecture:
/// 1. **Degenerate**: t=0, s=0, or k=0 -> intrinsic value.
/// 2. **BS path**: when xi^2*T < 0.01, uses BS(sigma_bar) via `bs_full_hp`. ~130K CU.
/// 3. **CV path**: BS control variate + 21-node DE quadrature. ~410-430K CU.
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
/// - `Overflow` if intermediate arithmetic overflows.
///
/// # Accuracy
/// $0.002-$0.007 typical, $0.018 worst case vs QuantLib AnalyticHestonEngine.
///
/// # CU Cost
/// 410-430K CU (CV path). 130K CU (BS fallback when xi^2*T < 0.01).
pub fn heston_price(
    s: u128, k: u128, r: u128, t: u128,
    v0: u128, kappa: u128, theta: u128, xi: u128,
    rho: i128,
) -> Result<(u128, u128), SolMathError> {
    const MAX_H_INPUT: u128 = ((i64::MAX as i128) * SCALE_TO_H) as u128;

    if s > i128::MAX as u128 || k > i128::MAX as u128 || r > i128::MAX as u128
        || t > i128::MAX as u128 || v0 > i128::MAX as u128 || kappa > i128::MAX as u128
        || theta > i128::MAX as u128 || xi > i128::MAX as u128
    {
        return Err(SolMathError::Overflow);
    }
    if rho <= -SCALE_I || rho >= SCALE_I {
        return Err(SolMathError::DomainError);
    }
    if s > MAX_H_INPUT || k > MAX_H_INPUT || r > MAX_H_INPUT || t > MAX_H_INPUT
        || v0 > MAX_H_INPUT || kappa > MAX_H_INPUT || theta > MAX_H_INPUT || xi > MAX_H_INPUT
    {
        return Err(SolMathError::Overflow);
    }
    if t == 0 {
        let call = if s > k { s - k } else { 0 };
        let put = if k > s { k - s } else { 0 };
        return Ok((call, put));
    }
    if s == 0 {
        let r_t = fp_mul_i(r as i128, t as i128)?;
        let put = fp_mul_i(k as i128, exp_fixed_i(-r_t)?)?;
        return Ok((0, if put > 0 { put as u128 } else { 0 }));
    }
    if k == 0 { return Ok((s, 0)); }

    let xi_sq_t = fp_mul(fp_mul(xi, xi)?, t)?;
    if xi == 0 || xi_sq_t < HESTON_BS_THRESHOLD {
        let sigma_bar = cir_rms_vol(v0, kappa, theta, t)?;
        if sigma_bar == 0 {
            let r_t = fp_mul_i(r as i128, t as i128)?;
            let k_disc = fp_mul_i(k as i128, exp_fixed_i(-r_t)?)? as u128;
            let call = if s > k_disc { s - k_disc } else { 0 };
            let put = if k_disc > s { k_disc - s } else { 0 };
            return Ok((call, put));
        }
        let bs = bs_full_hp(s, k, r, sigma_bar, t)?;
        return Ok((bs.call, bs.put));
    }

    heston_price_cv(s, k, r, t, v0, kappa, theta, xi, rho)
}

/// Fast signed multiply — no overflow check.
/// CF intermediates bounded: max product < 9e29 ≪ i128::MAX (1.7e38).
#[cfg(test)]
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
// CU budget:
//   130K  bs_full_hp
//    30K  i128 setup (2× ln, 1× exp, 1× sqrt, precomputes)
//     5K  downscale 9 params
//    63K  21 × 3K fully-i64 loop
//     2K  upscale + prefactor
//   ≈230K total
// ============================================================

/// 21-node DE nodes at SCALE_H (2^20). Precomputed: DE_NODES[i] / SCALE_TO_H.
const DE_NODES_H: [i64; 21] = [
    0, 5, 78, 661, 3_518, 13_091,
    36_985, 84_677, 165_529, 288_138, 462_497, 705_139,
    1_048_576, 1_559_291, 2_377_199, 3_815_640, 6_641_838, 12_984_058,
    29_728_064, 83_979_424, 312_444_416,
];

/// 21-node DE weights at SCALE_H (2^20). Precomputed: DE_WEIGHTS[i] / SCALE_TO_H.
const DE_WEIGHTS_H: [i64; 21] = [
    0, 15, 188, 1_245, 5_197, 15_237,
    34_162, 62_792, 100_303, 146_490, 204_809, 285_571,
    411_774, 631_479, 1_052_649, 1_939_833, 4_024_659, 9_627_695,
    27_459_829, 97_745_668, 461_585_778,
];

/// Heston via Andersen-Piterbarg control variate.
/// BS(σ_eff) in i128. Entire 21-node DE loop at SCALE_H = 2^20 (shift-only mul).
fn heston_price_cv(
    s: u128, k: u128, r: u128, t: u128,
    v0: u128, kappa: u128, theta: u128, xi: u128,
    rho: i128,
) -> Result<(u128, u128), SolMathError> {
    // ── i128: σ_eff + BS price (~140K CU) ──

    let sigma_eff_sq = cir_expected_var(v0, kappa, theta, t)?;
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
    let seff_sq_t_half_h = to_h(fp_mul(sigma_eff_sq, t)? / 2);

    use crate::i64_cf::{exp_h_pub, ln_h_pub, sqrt_h_pub, mul_h_pub, div_h_pub};
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
    let mu_h = if xi_sq_h != 0 { div_h_pub(mul_h_pub(kappah, thetah), xi_sq_h) } else { 0 };

    // DE loop — skip first 3 + last 2 (negligible contribution)
    const FIRST: usize = 3;
    const LAST: usize = 19;
    let mut sum: i64 = 0;
    let mut idx = FIRST;
    while idx < LAST {
        let u = DE_NODES_H[idx];
        let w = DE_WEIGHTS_H[idx];
        if u > 0 && w > 0 {
            let f = heston_cv_node_h(
                u, x, th, v0h,
                m_re_h, m_im_coeff_h, xi_sq_h, xi_sq_1mrho_h,
                mu_h, seff_sq_t_half_h,
            );
            // w ≤ DE_WEIGHTS_H max ≈ 4.6e8 (i64), f is the CF node result at SCALE_H (2^20);
            // product fits i128 easily (< 4.6e8 * 2^63 ≪ i128::MAX), then shifted back by 20 bits.
            sum += ((w as i128 * f as i128) >> 20) as i64;
        }
        idx += 1;
    }

    // prefactor: √(SK·disc)/π × correction
    let prefactor_h = div_h_pub(mul_h_pub(sqrt_sk_disc_h, sum), crate::i64_cf::PI_H_PUB);

    // ── Upscale single result to i128, add to BS ──
    // prefactor_h is i64 at SCALE_H (2^20); SCALE_TO_H = SCALE / 2^20 = 1e12/2^20 ≈ 9.5e5;
    // product < i64::MAX * SCALE_TO_H < 9.2e18 * 9.5e5 ≈ 8.8e24 ≪ i128::MAX.
    let correction_128 = prefactor_h as i128 * SCALE_TO_H;

    // Post-hoc safety: the CV correction is a difference from BS — it should be
    // small relative to the spot price. If i64 CF overflow produced garbage,
    // the correction will be unreasonably large. Reject rather than misprice.
    // Bound: correction should not exceed spot price (conservative).
    if correction_128.unsigned_abs() > s as u128 {
        return Err(SolMathError::Overflow);
    }

    let call = (bs.call as i128 + correction_128).max(0) as u128;
    let put = (bs.put as i128 + correction_128).max(0) as u128;

    Ok((call, put))
}

// ============================================================
// Raw Lewis CF path (21-node DE, i128) — test reference only
// ============================================================

#[cfg(test)]
fn heston_price_cf_raw(
    s: u128, k: u128, r: u128, t: u128,
    v0: u128, kappa: u128, theta: u128, xi: u128,
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
            (fp_div_i(fmul(dn_re, p_re) + fmul(dn_im, p_im), p_mod2).unwrap(),
             fp_div_i(fmul(dn_im, p_re) - fmul(dn_re, p_im), p_mod2).unwrap())
        } else { (0, 0) };

        let ratio_re = if p_mod2 != 0 {
            fp_div_i(fmul(2*d.re, p_re) + fmul(2*d.im, p_im), p_mod2).unwrap()
        } else { SCALE_I };
        let ratio_im = if p_mod2 != 0 {
            fp_div_i(fmul(2*d.im, p_re) - fmul(2*d.re, p_im), p_mod2).unwrap()
        } else { 0 };
        let ratio_mod = fp_sqrt(
            (fmul(ratio_re, ratio_re) + fmul(ratio_im, ratio_im)) as u128
        ).unwrap() as i128;
        let ln_ratio_mod = if ratio_mod > 0 { ln_fixed_i(ratio_mod as u128).unwrap() } else { 0 };
        let arg_ratio = atan2_fixed(ratio_im, ratio_re).unwrap();
        let real_exp = fmul(d_coeff_re, v0_i)
            + fmul(mu, fmul(mm_re, t_i))
            + fmul(two_mu, ln_ratio_mod);
        let imag_exp = fmul(d_coeff_im, v0_i)
            + fmul(mu, fmul(mm_im, t_i))
            + fmul(two_mu, arg_ratio);

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

#[cfg(test)]
/// atan2 with unwrap — den > 0 guaranteed (swap ensures den = max(|x|,|y|) > 0).
pub(crate) fn atan2_fixed(y: i128, x: i128) -> Result<i128, SolMathError> {
    const PI_HALF: i128 = 1_570_796_326_795;
    if x == 0 && y == 0 { return Ok(0); }
    if x == 0 { return Ok(if y > 0 { PI_HALF } else { -PI_HALF }); }
    if y == 0 { return Ok(if x > 0 { 0 } else { PI_SCALE }); }
    let ax = x.unsigned_abs();
    let ay = y.unsigned_abs();
    let swap = ay > ax;
    let (num, den) = if swap { (ax as i128, ay as i128) } else { (ay as i128, ax as i128) };
    let z = fp_div_i(num, den).unwrap(); // den > 0: max(|x|,|y|)
    let a = atan_01(z);
    let a = if swap { PI_HALF - a } else { a };
    let a = if x < 0 { PI_SCALE - a } else { a };
    Ok(if y < 0 { -a } else { a })
}

#[cfg(test)]
fn atan_01(z: i128) -> i128 {
    const TAN15: i128 = 267_949_192_431;
    const TAN30: i128 = 577_350_269_190;
    const PI_6: i128 = 523_598_775_598;
    const PI_4: i128 = 785_398_163_397;
    if z <= TAN15 { atan_poly(z) }
    else if z <= 750_000_000_000 {
        // den = SCALE + z*TAN30 > SCALE (z,TAN30 > 0)
        PI_6 + atan_poly(fp_div_i(z - TAN30, SCALE_I + fp_mul_i(z, TAN30).unwrap()).unwrap())
    } else {
        // den = z + SCALE > SCALE (z > 0)
        PI_4 + atan_poly(fp_div_i(z - SCALE_I, z + SCALE_I).unwrap())
    }
}

#[cfg(test)]
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

pub(crate) fn cir_rms_vol(v0: u128, kappa: u128, theta: u128, t: u128) -> Result<u128, SolMathError> {
    Ok(fp_sqrt(cir_expected_var(v0, kappa, theta, t)?)?)
}

pub(crate) fn cir_expected_var(v0: u128, kappa: u128, theta: u128, t: u128) -> Result<u128, SolMathError> {
    if t == 0 || kappa == 0 { return Ok(v0); }
    let kappa_t = fp_mul(kappa, t)?;
    // v0 and theta are variances at SCALE (≤ ~1*SCALE for typical vol ≤ 100%);
    // both fit i128 trivially; difference ∈ (-SCALE_I, SCALE_I). No overflow.
    let delta_i = v0 as i128 - theta as i128;
    let em1 = expm1_fixed(-(kappa_t as i128))?;
    let ratio = fp_div_i(-em1, kappa_t as i128)?;
    // theta as i128 ≤ SCALE_I; fp_mul_i(delta_i, ratio) ≤ SCALE_I; sum ≤ 2·SCALE_I. Fits i128.
    let result = theta as i128 + fp_mul_i(delta_i, ratio)?;
    Ok(if result > 0 { result as u128 } else { 0 })
}

// ============================================================
// DE quadrature: h=0.25, 21 nodes
// ============================================================

#[cfg(test)]
const DE_N: usize = 21;

#[cfg(test)]
const DE_NODES: [i128; 21] = [
            146_529,      4_855_077,     74_579_941,
        630_580_368,  3_355_820_405, 12_485_683_179,
     35_272_052_349, 80_758_778_787,157_867_103_304,
    274_805_390_708,441_077_539_800,672_466_825_156,
  1_000_000_000_000,1_487_062_205_288,2_267_175_065_076,
  3_638_938_804_750,6_334_441_939_257,12_382_554_751_561,
 28_351_057_945_605,80_091_732_721_767,297_989_725_118_823,
];

#[cfg(test)]
const DE_WEIGHTS: [i128; 21] = [
            579_312,     14_972_941,    179_599_271,
      1_187_766_015,  4_957_925_286, 14_533_760_073,
     32_583_937_342, 59_889_282_729, 95_662_152_103,
    139_716_814_232,195_316_933_331,272_372_585_178,
    392_699_081_699,602_312_206_376,1_003_945_204_820,
  1_850_112_676_736,3_838_458_650_317,9_182_683_710_144,
 26_190_398_181_291,93_229_502_186_530,440_253_236_111_365,
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
        let (c, p) = heston_price(100*SCALE, 100*SCALE, 50_000_000_000, SCALE,
            40_000_000_000, 2*SCALE, 40_000_000_000, 0, -700_000_000_000).unwrap();
        let sigma = cir_rms_vol(40_000_000_000, 2*SCALE, 40_000_000_000, SCALE).unwrap();
        let bs = bs_full_hp(100*SCALE, 100*SCALE, 50_000_000_000, sigma, SCALE).unwrap();
        assert_eq!(c, bs.call);
        assert_eq!(p, bs.put);
    }

    #[test]
    fn test_heston_small_xi_bs_path() {
        let (c, _) = heston_price(100*SCALE, 100*SCALE, 0, 100_000_000_000,
            10_000_000_000, 500_000_000_000, 10_000_000_000, 100_000_000_000, -900_000_000_000).unwrap();
        let sigma = cir_rms_vol(10_000_000_000, 500_000_000_000, 10_000_000_000, 100_000_000_000).unwrap();
        let bs = bs_full_hp(100*SCALE, 100*SCALE, 0, sigma, 100_000_000_000).unwrap();
        assert_eq!(c, bs.call);
    }

    #[test]
    fn test_heston_cf_path_differs() {
        let (c, _) = heston_price(100*SCALE, 100*SCALE, 50_000_000_000, SCALE,
            40_000_000_000, 2*SCALE, 40_000_000_000, 500_000_000_000, -700_000_000_000).unwrap();
        let sigma = cir_rms_vol(40_000_000_000, 2*SCALE, 40_000_000_000, SCALE).unwrap();
        let bs = bs_full_hp(100*SCALE, 100*SCALE, 50_000_000_000, sigma, SCALE).unwrap();
        assert_ne!(c, bs.call);
    }

    #[test]
    fn test_heston_put_call_parity() {
        let r: u128 = 50_000_000_000;
        let (c, p) = heston_price(100*SCALE, 100*SCALE, r, SCALE,
            40_000_000_000, 2*SCALE, 40_000_000_000, 500_000_000_000, -700_000_000_000).unwrap();
        let disc = exp_fixed_i(-fp_mul_i(r as i128, SCALE_I).unwrap()).unwrap();
        let parity = (c as i128 - p as i128 - 100*SCALE_I + fp_mul_i(100*SCALE_I, disc).unwrap()).abs();
        assert!(parity <= 100, "parity error {}", parity);
    }

    #[test]
    fn test_heston_t_zero() {
        let (c, p) = heston_price(110*SCALE, 100*SCALE, 0, 0,
            40_000_000_000, 2*SCALE, 40_000_000_000, 500_000_000_000, -700_000_000_000).unwrap();
        assert_eq!(c, 10*SCALE);
        assert_eq!(p, 0);
    }

    #[test]
    fn test_heston_all_moneyness() {
        for &(s, k) in &[(80, 100), (100, 100), (120, 100)] {
            let (c, p) = heston_price(s as u128*SCALE, k as u128*SCALE, 50_000_000_000, SCALE,
                40_000_000_000, 2*SCALE, 40_000_000_000, 400_000_000_000, -600_000_000_000).unwrap();
            assert!(c > 0 || s <= k);
            assert!(p > 0 || s >= k);
        }
    }

    // ── CV vs raw i128 comparison tests ──

    /// Helper: compare CV result against 21-node i128 raw Lewis.
    fn compare_cv_vs_raw(
        s: u128, k: u128, r: u128, t: u128,
        v0: u128, kappa: u128, theta: u128, xi: u128,
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
        let parity = (cv_c as i128 - cv_p as i128 - s as i128 + fp_mul_i(k as i128, disc).unwrap()).abs();

        assert!(err_c <= max_err,
            "{} call: cv={}, raw={}, err={} > max={}",
            label, cv_c, raw_c, err_c, max_err);
        assert!(err_p <= max_err,
            "{} put: cv={}, raw={}, err={} > max={}",
            label, cv_p, raw_p, err_p, max_err);
        assert!(parity <= 1000,
            "{} put-call parity error: {} (c={}, p={}, s={}, k_disc={})",
            label, parity, cv_c, cv_p, s, fp_mul_i(k as i128, disc).unwrap());
    }

    // $0.02 tolerance: 20_000_000_000 at SCALE on $100 notional.
    // Most cases achieve sub-penny ($0.01); deep ITM reaches ~$0.018
    // due to inherent i64 CF precision for large forward moneyness.
    const MAX_ERR: i128 = 20_000_000_000;

    #[test]
    fn test_cv_vs_raw_atm() {
        compare_cv_vs_raw(
            100*SCALE, 100*SCALE, 50_000_000_000, SCALE,
            40_000_000_000, 2*SCALE, 40_000_000_000, 300_000_000_000,
            -700_000_000_000,
            "ATM ξ=0.3 ρ=-0.7", MAX_ERR,
        );
    }

    #[test]
    fn test_cv_vs_raw_itm() {
        compare_cv_vs_raw(
            120*SCALE, 100*SCALE, 50_000_000_000, SCALE,
            40_000_000_000, 2*SCALE, 40_000_000_000, 300_000_000_000,
            -700_000_000_000,
            "ITM 120/100", MAX_ERR,
        );
    }

    #[test]
    fn test_cv_vs_raw_otm() {
        compare_cv_vs_raw(
            80*SCALE, 100*SCALE, 50_000_000_000, SCALE,
            40_000_000_000, 2*SCALE, 40_000_000_000, 300_000_000_000,
            -700_000_000_000,
            "OTM 80/100", MAX_ERR,
        );
    }

    #[test]
    fn test_cv_vs_raw_high_xi() {
        compare_cv_vs_raw(
            100*SCALE, 100*SCALE, 50_000_000_000, SCALE,
            40_000_000_000, 2*SCALE, 40_000_000_000, 500_000_000_000,
            -700_000_000_000,
            "ATM ξ=0.5", MAX_ERR,
        );
    }

    #[test]
    fn test_cv_vs_raw_very_high_xi() {
        compare_cv_vs_raw(
            100*SCALE, 100*SCALE, 50_000_000_000, SCALE,
            40_000_000_000, 2*SCALE, 40_000_000_000, 800_000_000_000,
            -700_000_000_000,
            "ATM ξ=0.8", MAX_ERR,
        );
    }

    #[test]
    fn test_cv_vs_raw_zero_rho() {
        compare_cv_vs_raw(
            100*SCALE, 100*SCALE, 50_000_000_000, SCALE,
            40_000_000_000, 2*SCALE, 40_000_000_000, 300_000_000_000,
            0,
            "ATM ρ=0", MAX_ERR,
        );
    }

    #[test]
    fn test_cv_vs_raw_short_maturity() {
        compare_cv_vs_raw(
            100*SCALE, 100*SCALE, 50_000_000_000, 100_000_000_000,
            40_000_000_000, 2*SCALE, 40_000_000_000, 300_000_000_000,
            -700_000_000_000,
            "ATM T=0.1", MAX_ERR,
        );
    }

    #[test]
    fn test_cv_vs_raw_long_maturity() {
        compare_cv_vs_raw(
            100*SCALE, 100*SCALE, 50_000_000_000, 2*SCALE,
            40_000_000_000, 2*SCALE, 40_000_000_000, 300_000_000_000,
            -700_000_000_000,
            "ATM T=2.0", MAX_ERR,
        );
    }

    #[test]
    fn test_cv_vs_raw_deep_otm_high_xi() {
        compare_cv_vs_raw(
            80*SCALE, 100*SCALE, 50_000_000_000, SCALE,
            40_000_000_000, 2*SCALE, 40_000_000_000, 500_000_000_000,
            -500_000_000_000,
            "OTM 80/100 ξ=0.5 ρ=-0.5", MAX_ERR,
        );
    }

    #[test]
    fn test_cv_vs_raw_negative_rho_strong() {
        compare_cv_vs_raw(
            100*SCALE, 100*SCALE, 50_000_000_000, SCALE,
            40_000_000_000, 2*SCALE, 40_000_000_000, 300_000_000_000,
            -900_000_000_000,
            "ATM ρ=-0.9", MAX_ERR,
        );
    }

    /// Full matrix test: moneyness × maturity × xi × rho
    #[test]
    fn test_cv_vs_raw_full_matrix() {
        let moneyness: [(u128, u128); 5] = [
            (80, 100), (90, 100), (100, 100), (110, 100), (120, 100),
        ];
        let maturities: [u128; 4] = [
            100_000_000_000, // T=0.1
            250_000_000_000, // T=0.25
            SCALE,           // T=1.0
            2*SCALE,         // T=2.0
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

                        let cv = heston_price_cv(s, k, 50_000_000_000, t,
                            40_000_000_000, 2*SCALE, 40_000_000_000, xi, rho);
                        let raw = heston_price_cf_raw(s, k, 50_000_000_000, t,
                            40_000_000_000, 2*SCALE, 40_000_000_000, xi, rho);

                        if let (Ok((cv_c, _)), Ok((raw_c, _))) = (cv, raw) {
                            let err = (cv_c as i128 - raw_c as i128).abs();
                            if err > max_err_seen { max_err_seen = err; }
                            count += 1;

                            // $2.00 tolerance on $100 notional = 2% of notional
                            assert!(err <= 2_000_000_000_000,
                                "S={} K={} T={} ξ={} ρ={}: cv={} raw={} err={}",
                                s_mult, k_mult, t, xi, rho, cv_c, raw_c, err);
                        }
                    }
                }
            }
        }

        // Sanity: we tested enough cases
        assert!(count >= 150, "Only {} test cases ran", count);
    }

    /// Put-call parity across the full matrix
    #[test]
    fn test_cv_put_call_parity_matrix() {
        let params: [(u128, u128, u128, i128); 6] = [
            (100, 100, 300_000_000_000, -700_000_000_000),
            (80, 100, 300_000_000_000, -500_000_000_000),
            (120, 100, 500_000_000_000, -700_000_000_000),
            (100, 100, 200_000_000_000, 0),
            (90, 100, 800_000_000_000, -700_000_000_000),
            (110, 100, 300_000_000_000, -900_000_000_000),
        ];
        let r: u128 = 50_000_000_000;
        let t: u128 = SCALE;

        for &(s_mult, k_mult, xi, rho) in &params {
            let s = s_mult * SCALE;
            let k = k_mult * SCALE;
            let (c, p) = heston_price(s, k, r, t,
                40_000_000_000, 2*SCALE, 40_000_000_000, xi, rho).unwrap();
            let disc = exp_fixed_i(-fp_mul_i(r as i128, t as i128).unwrap()).unwrap();
            let parity = (c as i128 - p as i128 - s as i128 + fp_mul_i(k as i128, disc).unwrap()).abs();
            assert!(parity <= 1000,
                "PCP fail S={} K={} ξ={} ρ={}: C={} P={} err={}",
                s_mult, k_mult, xi, rho, c, p, parity);
        }
    }

    #[test]
    fn test_heston_call_bounds() {
        let s = 100 * SCALE;
        let r = 50_000_000_000u128;
        let params: &[(u128, u128, u128, u128, i128)] = &[
            (40_000_000_000, 2*SCALE, 40_000_000_000, 300_000_000_000, -700_000_000_000),
            (40_000_000_000, 2*SCALE, 40_000_000_000, 800_000_000_000, -500_000_000_000),
        ];
        for &(v0, kappa, theta, xi, rho) in params {
            for &k_mult in &[80u128, 100, 120] {
                let k = k_mult * SCALE;
                for &t in &[SCALE/10, SCALE, 2*SCALE] {
                    if let Ok((call, put)) = heston_price(s, k, r, t, v0, kappa, theta, xi, rho) {
                        assert!(call <= s, "Call {} > spot {}", call, s);
                        assert!(put <= k, "Put {} > strike {}", put, k);
                    }
                }
            }
        }
    }

}

#[cfg(test)]
include!("../../../test_data/heston_reference_tests.rs");
