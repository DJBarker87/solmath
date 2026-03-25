// ============================================================
// i64 Heston CF evaluation for control-variate quadrature.
//
// Internal arithmetic at SCALE_H = 2^20 = 1_048_576.
// mul_h(a,b) = (a*b) >> 20 — a shift, no division. ~20 CU on BPF
// vs ~150 CU for i128 division by 1e6.
//
// Boundary conversion: inputs downscaled from SCALE (1e12) to SCALE_H,
// output upscaled back. The loop internals never touch SCALE or SCALE_6.
//
// All transcendentals (exp, ln, sincos, sqrt, atan2) reimplemented
// at SCALE_H inside this module. Nothing from i64_math.rs is used
// in the hot loop.
// ============================================================

use crate::constants::*;

/// SCALE_H = 2^20 = 1_048_576. Binary fixed-point for shift-only mul.
const SH: i64 = 1 << 20; // 1_048_576
const SHIFT: u32 = 20;

// Precomputed constants at SCALE_H
const LN2_H: i64 = 726_817;      // ln(2) × 2^20 = 0.693147... × 1048576
const PI_H: i64 = 3_294_199;     // π × 2^20
const PIH_H: i64 = 1_647_099;    // π/2 × 2^20
const PIQ_H: i64 = 823_550;      // π/4 × 2^20

// ============================================================
// Core arithmetic
// ============================================================

/// Fixed-point multiply: (a × b) >> 20. No division.
#[inline(always)]
fn mul_h(a: i64, b: i64) -> i64 {
    let p = a as i128 * b as i128;
    (if p >= 0 { p >> SHIFT } else { -(-p >> SHIFT) }) as i64
}

/// Fixed-point divide: (a << 20) / b. True division — only used for
/// the few places that genuinely need division (ratio, mu, atan arg).
/// SAFETY: saturates to i64::MAX/MIN on b==0 for CU efficiency.
/// All production callers guard b!=0; debug_assert catches regressions in tests.
#[inline(always)]
fn div_h(a: i64, b: i64) -> i64 {
    debug_assert!(b != 0, "div_h: divisor must be non-zero");
    if b == 0 { return if a >= 0 { i64::MAX } else { i64::MIN }; }
    (((a as i128) << SHIFT) / b as i128) as i64
}

// ============================================================
// Transcendentals at SCALE_H
// ============================================================

/// exp at SCALE_H. Range reduction via shift, Taylor remainder.
fn exp_h(x: i64) -> i64 {
    if x >= 15 * SH { return i64::MAX / 4; }
    if x <= -15 * SH { return 0; }
    if x == 0 { return SH; }

    // Range reduce: x = k·ln2 + r, |r| ≤ ln2/2
    let half_ln2 = LN2_H / 2;
    let k = if x >= 0 { (x + half_ln2) / LN2_H } else { (x - half_ln2) / LN2_H };
    let r = x - k * LN2_H;

    // Taylor degree 10: exp(r) = Σ rⁿ/n!  (converges fast for |r| < 0.347×SH)
    let mut term: i64 = SH;
    let mut sum: i64 = SH;
    let mut n: i64 = 1;
    while n <= 10 {
        term = mul_h(term, r) / n;
        sum += term;
        if term == 0 { break; }
        n += 1;
    }

    if k >= 0 { sum << (k as u32) } else { sum >> ((-k) as u32) }
}

/// ln at SCALE_H. Arctanh series: ln(m) = 2·t·(1 + t²/3 + t⁴/5 + ...)
/// where t = (m − SH)/(m + SH).
fn ln_h(x: i64) -> i64 {
    if x <= 0 { return i64::MIN; }
    let mut m = x;
    let mut k: i32 = 0;
    while m < SH { m *= 2; k -= 1; }
    while m >= 2 * SH { m /= 2; k += 1; }

    let t = div_h(m - SH, m + SH);
    let t2 = mul_h(t, t);
    let mut sum: i64 = 0;
    let mut pw = t;
    let mut d: i64 = 1;
    let mut i = 0;
    while i < 10 {
        sum += pw / d;
        pw = mul_h(pw, t2);
        d += 2;
        if pw.abs() < 1 { break; }
        i += 1;
    }
    2 * sum + (k as i64) * LN2_H
}

/// sqrt at SCALE_H. Newton iteration on scaled value.
fn sqrt_h(x: i64) -> i64 {
    if x <= 0 { return 0; }
    let scaled = (x as i128) << SHIFT;
    let bl = 128 - (scaled as u128).leading_zeros();
    let mut g: i128 = 1i128 << ((bl + 1) / 2).min(62);
    let mut i = 0;
    while i < 6 {
        if g == 0 { break; }
        let ng = (g + scaled / g) / 2;
        if ng >= g { break; }
        g = ng;
        i += 1;
    }
    g as i64
}

/// Reduce angle to (−π, π] at SCALE_H.
#[inline]
fn mod_2pi_h(x: i64) -> i64 {
    let pi2 = 2 * PI_H;
    let mut r = x % pi2;
    if r > PI_H { r -= pi2; }
    if r < -PI_H { r += pi2; }
    r
}

/// sin/cos core on [0, π/4] using minimax polynomials at SCALE_H.
fn sin_core_h(x: i64) -> i64 {
    let x2 = mul_h(x, x);
    // sin(x)/x ≈ 1 − x²/6 + x⁴/120 − x⁶/5040
    let mut r = -SH / 5040;       // c6 (tiny)
    r = mul_h(r, x2) + SH / 120;  // c4
    r = mul_h(r, x2) - SH / 6;    // c2
    r = mul_h(r, x2) + SH;        // c0
    mul_h(r, x)
}

fn cos_core_h(x: i64) -> i64 {
    let x2 = mul_h(x, x);
    // cos(x) ≈ 1 − x²/2 + x⁴/24 − x⁶/720
    let mut r = -SH / 720;
    r = mul_h(r, x2) + SH / 24;
    r = mul_h(r, x2) - SH / 2;
    r = mul_h(r, x2) + SH;
    r
}

/// Fused sin+cos at SCALE_H.
fn sincos_h(x: i64) -> (i64, i64) {
    let mut xx = mod_2pi_h(x);
    let sin_sign: i64 = if xx < 0 { xx = -xx; -1 } else { 1 };
    let cos_sign: i64 = if xx > PIH_H { xx = PI_H - xx; -1 } else { 1 };
    if xx > PIQ_H {
        let y = PIH_H - xx;
        (cos_core_h(y) * sin_sign, sin_core_h(y) * cos_sign)
    } else {
        (sin_core_h(xx) * sin_sign, cos_core_h(xx) * cos_sign)
    }
}

/// Complex sqrt at SCALE_H.
fn complex_sqrt_h(re: i64, im: i64) -> (i64, i64) {
    let nsq = mul_h(re, re) + mul_h(im, im);
    if nsq == 0 { return (0, 0); }
    let modz = sqrt_h(nsq);
    let re_arg = (modz + re) / 2;
    let out_re = if re_arg > 0 { sqrt_h(re_arg) } else { 0 };
    if out_re == 0 {
        let out_im = sqrt_h((modz - re) / 2);
        return (0, if im < 0 { -out_im } else { out_im });
    }
    let out_im = div_h(im, 2 * out_re);
    (out_re, out_im)
}

// ============================================================
// atan2 at SCALE_H
// ============================================================

fn atan_poly_h(t: i64) -> i64 {
    let t2 = mul_h(t, t);
    let t3 = mul_h(t2, t);
    let t5 = mul_h(t3, t2);
    t - t3 / 3 + t5 / 5
}

fn atan_01_h(z: i64) -> i64 {
    const TAN15: i64 = 280_870;  // tan(π/12) × 2^20
    const TAN30: i64 = 605_382;  // tan(π/6) × 2^20
    const PI_6C: i64 = 549_033;  // π/6 × 2^20
    if z <= TAN15 {
        atan_poly_h(z)
    } else if z <= 786_432 { // ~0.75 × SH
        let num = z - TAN30;
        let den = SH + mul_h(z, TAN30);
        PI_6C + atan_poly_h(div_h(num, den))
    } else {
        PIQ_H + atan_poly_h(div_h(z - SH, z + SH))
    }
}

pub(crate) fn atan2_h(y: i64, x: i64) -> i64 {
    if x == 0 && y == 0 { return 0; }
    if x == 0 { return if y > 0 { PIH_H } else { -PIH_H }; }
    if y == 0 { return if x > 0 { 0 } else { PI_H }; }
    let ax = x.unsigned_abs() as i64;
    let ay = y.unsigned_abs() as i64;
    let swap = ay > ax;
    let (num, den) = if swap { (ax, ay) } else { (ay, ax) };
    let z = div_h(num, den);
    let a = atan_01_h(z);
    let a = if swap { PIH_H - a } else { a };
    let a = if x < 0 { PI_H - a } else { a };
    if y < 0 { -a } else { a }
}

// ============================================================
// Heston CV node evaluator at SCALE_H
// ============================================================

/// Evaluate `Re[(φ_BS − φ_H) · e^{iux}] / (u² + ¼)` at a single
/// quadrature node. All arithmetic at SCALE_H = 2^20.
///
/// `mu` = κθ/ξ² is loop-invariant — precomputed by caller.
pub(crate) fn heston_cv_node_h(
    u: i64, x: i64, t: i64, v0: i64,
    m_re: i64, m_im_coeff: i64, // m_re = κ − ρξ/2, m_im_coeff = −ρξ
    xi_sq: i64, xi_sq_1mrho: i64, // ξ², ξ²(1−ρ²)
    mu: i64, // κθ/ξ² (loop-invariant)
    seff_sq_t_half: i64,
) -> i64 {
    let u_sq = mul_h(u, u);
    let uq = u_sq + SH / 4;

    let m_im = mul_h(m_im_coeff, u);

    let d2_re = mul_h(m_re, m_re)
        + mul_h(xi_sq_1mrho, u_sq)
        + xi_sq / 4;
    let d2_im = 2 * mul_h(m_re, m_im);
    let (d_re, d_im) = complex_sqrt_h(d2_re, d2_im);

    let d_re_t = mul_h(d_re, t);
    let d_im_t = mul_h(d_im, t);
    let exp_mag = exp_h(-d_re_t);
    let (sin_dt, cos_dt) = sincos_h(d_im_t);
    let exp_dt_re = mul_h(exp_mag, cos_dt);
    let exp_dt_im = -mul_h(exp_mag, sin_dt);

    let mm_re = m_re - d_re;
    let mm_im = m_im - d_im;
    let mmexp_re = mul_h(mm_re, exp_dt_re) - mul_h(mm_im, exp_dt_im);
    let mmexp_im = mul_h(mm_re, exp_dt_im) + mul_h(mm_im, exp_dt_re);
    let p_re = (m_re + d_re) - mmexp_re;
    let p_im = (m_im + d_im) - mmexp_im;

    let dn_re = -mul_h(uq, SH - exp_dt_re);
    let dn_im = mul_h(uq, exp_dt_im);
    let p_mod2 = mul_h(p_re, p_re) + mul_h(p_im, p_im);

    // Compute reciprocal of p_mod2 once, use mul_h for all 4 "divisions"
    // inv_p = SH / p_mod2  (one true division, then 4 shifts)
    let (d_coeff_re, d_coeff_im, ratio_re, ratio_im) = if p_mod2 != 0 {
        let inv_p = div_h(SH, p_mod2); // single division
        (mul_h(mul_h(dn_re, p_re) + mul_h(dn_im, p_im), inv_p),
         mul_h(mul_h(dn_im, p_re) - mul_h(dn_re, p_im), inv_p),
         mul_h(mul_h(2*d_re, p_re) + mul_h(2*d_im, p_im), inv_p),
         mul_h(mul_h(2*d_im, p_re) - mul_h(2*d_re, p_im), inv_p))
    } else { (0, 0, SH, 0) };

    let ratio_mod = sqrt_h(mul_h(ratio_re, ratio_re) + mul_h(ratio_im, ratio_im));
    let ln_ratio_mod = if ratio_mod > 0 { ln_h(ratio_mod) } else { 0 };
    let arg_ratio = atan2_h(ratio_im, ratio_re);

    let two_mu = 2 * mu;

    let real_exp = mul_h(d_coeff_re, v0)
        + mul_h(mu, mul_h(mm_re, t))
        + mul_h(two_mu, ln_ratio_mod);
    let imag_exp = mul_h(d_coeff_im, v0)
        + mul_h(mu, mul_h(mm_im, t))
        + mul_h(two_mu, arg_ratio);

    let total_angle = imag_exp + mul_h(u, x);
    let final_mag = exp_h(real_exp);
    let (_, cos_hv) = sincos_h(total_angle);
    let re_phi_h = mul_h(final_mag, cos_hv);

    let phi_bs = exp_h(-mul_h(seff_sq_t_half, uq));
    let (_, cos_bs) = sincos_h(mul_h(u, x));
    let re_phi_bs = mul_h(phi_bs, cos_bs);

    div_h(re_phi_bs - re_phi_h, uq)
}

// ============================================================
// Boundary conversion helpers
// ============================================================

/// SCALE (1e12) → SCALE_H (2^20)
pub(crate) const SCALE_TO_H: i128 = SCALE_I / SH as i128; // 953_674 (approx)
pub(crate) const SH_I64: i64 = SH;

#[inline]
pub(crate) fn to_h(v: u128) -> i64 {
    (v as i128 / SCALE_TO_H) as i64
}

#[inline]
pub(crate) fn to_h_i(v: i128) -> i64 {
    (v / SCALE_TO_H) as i64
}

// ============================================================
// Pub wrappers for use from heston.rs
// ============================================================

pub(crate) const PI_H_PUB: i64 = PI_H;

#[inline(always)]
pub(crate) fn mul_h_pub(a: i64, b: i64) -> i64 { mul_h(a, b) }
#[inline(always)]
pub(crate) fn div_h_pub(a: i64, b: i64) -> i64 { div_h(a, b) }
#[inline(always)]
pub(crate) fn exp_h_pub(x: i64) -> i64 { exp_h(x) }
#[inline(always)]
pub(crate) fn ln_h_pub(x: i64) -> i64 { ln_h(x) }
#[inline(always)]
pub(crate) fn sqrt_h_pub(x: i64) -> i64 { sqrt_h(x) }

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transcendental::exp_fixed_i;
    use crate::arithmetic::{fp_mul_i, fp_sqrt};

    // Downscale from SCALE (1e12) to SCALE_H (2^20)
    fn to_h_test(v: i128) -> i64 {
        (v / SCALE_TO_H) as i64
    }

    // ── exp_h accuracy vs i128 ──

    #[test]
    fn test_exp_h_vs_i128() {
        for x_int in [-15, -10, -5, -2, -1, 0, 1, 2, 3, 5] {
            let x_128 = x_int as i128 * SCALE_I;
            let x_h = x_int as i64 * SH;
            let ref_val = exp_fixed_i(x_128).unwrap();
            let test_val = exp_h(x_h);
            let ref_h = to_h_test(ref_val);
            if ref_h == 0 { continue; }
            let rel_err_ppm = ((ref_h - test_val).abs() as i128 * 1_000_000) / ref_h as i128;
            assert!(rel_err_ppm < 2000,
                "exp_h({}) = {}, expected {}, rel_err = {} ppm",
                x_int, test_val, ref_h, rel_err_ppm);
        }
    }

    // ── ln_h accuracy vs i128 ──

    #[test]
    fn test_ln_h_vs_i128() {
        for &mult in &[524_288i64, 1_048_576, 2_097_152, 5_242_880, 10_485_760] {
            let x_128 = mult as i128 * SCALE_TO_H;
            let ref_val = crate::transcendental::ln_fixed_i(x_128 as u128).unwrap();
            let test_val = ln_h(mult);
            let ref_h = to_h_test(ref_val);
            let err = (ref_h - test_val).abs();
            assert!(err < 1000,
                "ln_h({}) = {}, expected {}, err = {}",
                mult, test_val, ref_h, err);
        }
    }

    // ── sincos_h accuracy vs i128 ──

    #[test]
    fn test_sincos_h_vs_i128() {
        use crate::trig::sincos_fixed;
        for &angle_x10 in &[-31i64, -15, -7, 0, 7, 15, 31, 62] {
            let angle_128 = angle_x10 as i128 * SCALE_I / 10;
            let angle_h = angle_x10 as i64 * SH / 10;
            let (ref_s, ref_c) = sincos_fixed(angle_128).unwrap();
            let (test_s, test_c) = sincos_h(angle_h);
            let ref_sh = to_h_test(ref_s);
            let ref_ch = to_h_test(ref_c);
            let err_s = (ref_sh - test_s).abs();
            let err_c = (ref_ch - test_c).abs();
            assert!(err_s < 1000 && err_c < 1000,
                "sincos_h({:.1}) sin_err={} cos_err={}",
                angle_x10 as f64 / 10.0, err_s, err_c);
        }
    }

    // ── atan2_h accuracy vs i128 ──

    #[test]
    fn test_atan2_h_vs_i128() {
        let cases: [(i64, i64); 4] = [
            (SH, SH),           // π/4
            (SH, -SH),          // 3π/4
            (-SH, SH),          // -π/4
            (SH / 10, SH),      // ~0.0997
        ];
        for (y, x) in cases {
            let y_128 = y as i128 * SCALE_TO_H;
            let x_128 = x as i128 * SCALE_TO_H;
            let ref_val = crate::heston::atan2_fixed(y_128, x_128).unwrap();
            let test_val = atan2_h(y, x);
            let ref_h = to_h_test(ref_val);
            let err = (ref_h - test_val).abs();
            assert!(err < 2000,
                "atan2_h({}, {}) = {}, expected {}, err = {}",
                y, x, test_val, ref_h, err);
        }
    }

    // ── CF node at SCALE_H: sanity check ──

    #[test]
    fn test_cv_node_h_atm() {
        let v0: u128 = 40_000_000_000;
        let kappa: u128 = 2 * SCALE;
        let theta: u128 = 40_000_000_000;
        let xi: u128 = 300_000_000_000;
        let rho: i128 = -700_000_000_000;
        let t: u128 = SCALE;
        let r: u128 = 50_000_000_000;
        let s: u128 = 100 * SCALE;
        let k: u128 = 100 * SCALE;

        let sigma_eff_sq = crate::heston::cir_expected_var(v0, kappa, theta, t).unwrap();
        let r_t = fp_mul_i(r as i128, t as i128).unwrap();
        let ln_s = crate::transcendental::ln_fixed_i(s).unwrap();
        let ln_k = crate::transcendental::ln_fixed_i(k).unwrap();
        let x_128 = ln_s - ln_k + r_t;

        let kh = to_h(kappa);
        let xih = to_h(xi);
        let rhoh = to_h_i(rho);
        let rhoxi = mul_h(rhoh, xih);
        let m_re = kh - rhoxi / 2;
        let m_im_coeff = -rhoxi;
        let xi_sq = mul_h(xih, xih);
        let rho_sq = mul_h(rhoh, rhoh);
        let xi_sq_1mrho = mul_h(xi_sq, SH - rho_sq);
        let mu = if xi_sq != 0 { div_h(mul_h(kh, to_h(theta)), xi_sq) } else { 0 };

        let u_h = 5 * SH;

        let result = heston_cv_node_h(
            u_h, to_h_i(x_128), to_h(t), to_h(v0),
            m_re, m_im_coeff, xi_sq, xi_sq_1mrho,
            mu, to_h(crate::arithmetic::fp_mul(sigma_eff_sq, t).unwrap() / 2),
        );

        assert!(result.abs() < SH,
            "CV node result out of range: {}", result);
    }
}
