use crate::constants::*;
use crate::error::SolMathError;

#[derive(Clone, Copy)]
pub(crate) struct Complex6 {
    pub re: i64,
    pub im: i64,
}

impl Complex6 {
    /// Construct a complex number at 1e6 scale. Internal.
    pub(crate) fn new(re: i64, im: i64) -> Self {
        Self { re, im }
    }
}

/// Fixed-point multiply at 1e6 scale. Internal.
/// Returns `Err(Overflow)` if the result exceeds `i64` range.
#[inline]
pub(crate) fn mul6(a: i64, b: i64) -> Result<i64, SolMathError> {
    let wide = (a as i128 * b as i128) / SCALE_6 as i128;
    if wide > i64::MAX as i128 || wide < i64::MIN as i128 {
        return Err(SolMathError::Overflow);
    }
    Ok(wide as i64)
}

/// Fixed-point divide at 1e6 scale. Internal.
/// Returns `Err(DivisionByZero)` if `b == 0`, `Err(Overflow)` if result exceeds `i64` range.
#[inline]
pub(crate) fn div6(a: i64, b: i64) -> Result<i64, SolMathError> {
    if b == 0 {
        return Err(SolMathError::DivisionByZero);
    }
    let wide = (a as i128 * SCALE_6 as i128) / b as i128;
    if wide > i64::MAX as i128 || wide < i64::MIN as i128 {
        return Err(SolMathError::Overflow);
    }
    Ok(wide as i64)
}

/// Natural logarithm at 1e6 scale. Internal.
/// Returns `Err(DomainError)` if `x <= 0`.
pub(crate) fn ln6(x: i64) -> Result<i64, SolMathError> {
    if x <= 0 {
        return Err(SolMathError::DomainError);
    }
    let mut m = x;
    let mut k: i32 = 0;
    while m < SCALE_6 {
        m *= 2;
        k -= 1;
    }
    while m >= 2 * SCALE_6 {
        m /= 2;
        k += 1;
    }
    let t = div6(m - SCALE_6, m + SCALE_6)?;
    let t2 = mul6(t, t)?;
    let mut sum = 0i64;
    let mut pw = t;
    let mut d = 1i64;
    for _ in 0..10 {
        sum += pw / d;
        pw = mul6(pw, t2)?;
        d += 2;
        if pw.unsigned_abs() < 1 {
            break;
        }
    }
    Ok(2 * sum + (k as i64) * LN2_6)
}

/// Exponential at 1e6 scale. Internal.
/// Returns `Err(Overflow)` if `x >= 20 * SCALE_6`. Returns `Ok(0)` for large negative x.
pub(crate) fn exp6(x: i64) -> Result<i64, SolMathError> {
    let max_x = 20 * SCALE_6;
    if x >= max_x {
        return Err(SolMathError::Overflow);
    }
    if x <= -max_x {
        return Ok(0);
    }
    if x == 0 {
        return Ok(SCALE_6);
    }

    let mut k = x / LN2_6;
    let mut r = x - k * LN2_6;
    let half = LN2_6 / 2;
    if r > half {
        k += 1;
        r -= LN2_6;
    } else if r < -half {
        k -= 1;
        r += LN2_6;
    }

    let mut term = SCALE_6;
    let mut sum = SCALE_6;
    for n in 1..=10i64 {
        term = mul6(term, r)? / n;
        sum += term;
        if term == 0 {
            break;
        }
    }

    if k >= 0 {
        let result = (sum as i128).checked_shl(k as u32).ok_or(SolMathError::Overflow)?;
        if result > i64::MAX as i128 {
            return Err(SolMathError::Overflow);
        }
        Ok(result as i64)
    } else {
        Ok(sum >> ((-k) as u32))
    }
}

/// Square root at 1e6 scale. Internal.
/// Returns `Err(DomainError)` if `x < 0`, `Ok(0)` if `x == 0`.
pub(crate) fn sqrt6(x: i64) -> Result<i64, SolMathError> {
    if x < 0 {
        return Err(SolMathError::DomainError);
    }
    if x == 0 {
        return Ok(0);
    }
    let scaled = x as i128 * SCALE_6 as i128;
    let bl = 128 - scaled.leading_zeros();
    let mut g: i128 = 1i128 << ((bl + 1) / 2).min(62);
    for _ in 0..6 {
        if g == 0 {
            break;
        }
        let ng = (g + scaled / g) / 2;
        if ng >= g {
            break;
        }
        g = ng;
    }
    Ok(g as i64)
}

/// Reduce angle to (−π, π] at 1e6 scale. Internal.
#[inline]
pub(crate) fn mod_2pi_6(x: i64) -> i64 {
    const PI2_12: i128 = 6_283_185_307_180;
    const UP: i128 = 1_000_000;
    let x_hi = x as i128 * UP;
    let pi_12 = PI2_12 / 2;
    let mut r = x_hi % PI2_12;
    if r > pi_12 {
        r -= PI2_12;
    }
    if r < -pi_12 {
        r += PI2_12;
    }
    (r / UP) as i64
}

/// Core sin on [−π/4, π/4] at 1e6 scale. Internal.
pub(crate) fn sin_core6(x: i64) -> Result<i64, SolMathError> {
    let t = mul6(x, x)?;
    let mut r = SC4_6;
    r = mul6(r, t)? + SC3_6;
    r = mul6(r, t)? + SC2_6;
    r = mul6(r, t)? + SC1_6;
    r = mul6(r, t)? + SC0_6;
    mul6(r, x)
}

/// Core cos on [−π/4, π/4] at 1e6 scale. Internal.
pub(crate) fn cos_core6(x: i64) -> Result<i64, SolMathError> {
    let t = mul6(x, x)?;
    let mut r = CC4_6;
    r = mul6(r, t)? + CC3_6;
    r = mul6(r, t)? + CC2_6;
    r = mul6(r, t)? + CC1_6;
    r = mul6(r, t)? + CC0_6;
    Ok(r)
}

/// Fused sin+cos at 1e6 scale. Internal.
pub(crate) fn sincos6(x: i64) -> Result<(i64, i64), SolMathError> {
    let mut xx = mod_2pi_6(x);
    let sin_sign = if xx < 0 {
        xx = -xx;
        -1i64
    } else {
        1
    };
    let cos_sign = if xx > PIH_6 {
        xx = PI6 - xx;
        -1i64
    } else {
        1
    };
    if xx > PIQ_6 {
        let y = PIH_6 - xx;
        Ok((cos_core6(y)? * sin_sign, sin_core6(y)? * cos_sign))
    } else {
        Ok((sin_core6(xx)? * sin_sign, cos_core6(xx)? * cos_sign))
    }
}

/// Complex multiply at 1e6 scale. Internal.
/// Uses i128 intermediates for subtraction/addition to prevent i64 overflow.
pub(crate) fn complex_mul6(a: Complex6, b: Complex6) -> Result<Complex6, SolMathError> {
    let re_wide = mul6(a.re, b.re)? as i128 - mul6(a.im, b.im)? as i128;
    let im_wide = mul6(a.re, b.im)? as i128 + mul6(a.im, b.re)? as i128;
    if re_wide > i64::MAX as i128 || re_wide < i64::MIN as i128
        || im_wide > i64::MAX as i128 || im_wide < i64::MIN as i128
    {
        return Err(SolMathError::Overflow);
    }
    Ok(Complex6::new(re_wide as i64, im_wide as i64))
}

/// Complex exponential at 1e6 scale. Internal.
pub(crate) fn complex_exp6(z: Complex6) -> Result<Complex6, SolMathError> {
    let e = exp6(z.re)?;
    let (s, c) = sincos6(z.im)?;
    Ok(Complex6::new(mul6(e, c)?, mul6(e, s)?))
}

/// Complex square root at 1e6 scale. Internal.
pub(crate) fn complex_sqrt6(z: Complex6) -> Result<Complex6, SolMathError> {
    let nsq = mul6(z.re, z.re)? + mul6(z.im, z.im)?;
    if nsq == 0 {
        return Ok(Complex6::new(0, 0));
    }
    let modz = sqrt6(nsq)?;
    let re_arg = (modz + z.re) / 2;
    let re = if re_arg > 0 { sqrt6(re_arg)? } else { 0 };
    if re == 0 {
        let im = sqrt6((modz - z.re) / 2)?;
        return Ok(Complex6::new(0, if z.im < 0 { -im } else { im }));
    }
    let im = div6(z.im, 2 * re)?;
    Ok(Complex6::new(re, im))
}

const NIG_N_6: usize = 17;
const NIG_L_6: i64 = 6_750_000; // 6.75 * SCALE_6

/// NIG characteristic function at i64 scale.
pub(crate) fn nig_char6(u: i64, drift: i64, dt: i64, gamma: i64, asq: i64, beta: i64) -> Result<Complex6, SolMathError> {
    let usq = mul6(u, u)?;
    let bsq = mul6(beta, beta)?;
    let inner = complex_sqrt6(Complex6::new(asq - bsq + usq, -2 * mul6(beta, u)?))?;
    let exp_arg = Complex6::new(
        mul6(dt, gamma - inner.re)?,
        mul6(u, drift)? - mul6(dt, inner.im)?,
    );
    complex_exp6(exp_arg)
}

/// On-chain NIG call pricing via COS method (17 terms, i64 arithmetic).
/// ~120K CU. Inputs at SCALE (1e12), computed internally at 1e6.
///
/// # Errors
/// - `DomainError` if parameters are invalid (α² ≤ β², γ < 5, |β/α| ≥ 0.9, etc.)
/// - `Overflow` if intermediate arithmetic overflows.
///
/// # Precision
/// 95% within 0.5% for α ≥ 10, prices > $1.
///
/// # CU cost
/// ~120,000 CU.
pub fn nig_call_64(
    s: i64,
    k: i64,
    r: i64,
    t: i64,
    alpha: i64,
    beta: i64,
    delta: i64,
) -> Result<i64, SolMathError> {
    if s <= 0 || k <= 0 || t <= 0 || alpha <= 0 || delta <= 0 {
        return Err(SolMathError::DomainError);
    }
    // Domain: alpha ≤ 10,000. Real NIG calibrations have alpha in [1, 100].
    if alpha > 10_000 * SCALE_6 {
        return Err(SolMathError::DomainError);
    }
    let asq = mul6(alpha, alpha)?;
    let bsq = mul6(beta, beta)?;
    if asq <= bsq {
        return Err(SolMathError::DomainError);
    }
    let gamma = sqrt6(asq - bsq)?;
    if gamma < 5 * SCALE_6 {
        return Err(SolMathError::DomainError);
    }
    if beta == i64::MIN {
        return Err(SolMathError::DomainError);
    }
    if beta.abs() * 10 >= alpha * 9 {
        return Err(SolMathError::DomainError);
    }
    if gamma <= 0 {
        return Err(SolMathError::DomainError);
    }
    let gcu = mul6(mul6(gamma, gamma)?, gamma)?;
    if gcu == 0 {
        return Err(SolMathError::Overflow);
    }

    let bp1 = beta + SCALE_6;
    let omega = mul6(delta, gamma - sqrt6(asq - mul6(bp1, bp1)?)?)?;

    let ln_s = ln6(s)?;
    let ln_k = ln6(k)?;
    let dr = r - omega;
    let c1 = ln_s + mul6(dr, t)? + div6(mul6(mul6(delta, t)?, beta)?, gamma)?;
    let c2 = div6(mul6(mul6(delta, t)?, asq)?, gcu)?;
    let std = sqrt6(c2)?;

    let l_std = mul6(NIG_L_6, std)?;
    let mut a = c1 - l_std;
    let mut b = c1 + l_std;
    if ln_k - std < a {
        a = ln_k - std;
    }
    if ln_k + std > b {
        b = ln_k + std;
    }
    let ba = b - a;
    if ba <= 0 {
        return Err(SolMathError::DomainError);
    }

    let disc = exp6(-mul6(r, t)?)?;
    let exp_b = exp6(b)?;
    let is_otm = ln_k > c1;
    let exp_a = if is_otm { exp6(a)? } else { 0 };

    let cf_drift = ln_s + mul6(dr, t)?;
    let dt = mul6(delta, t)?;
    let gsq = asq - bsq;

    let lk_a = ln_k - a;
    let theta_v = div6(mul6(PI6, lk_a)?, ba)?;
    let (sin_tv, cos_tv) = sincos6(theta_v)?;
    let theta_r = div6(mul6(PI6, a)?, ba)?;
    let (sin_tr, cos_tr) = sincos6(theta_r)?;

    let (mut vc0, mut vs0) = (SCALE_6, 0i64);
    let (mut vc1, mut vs1) = (cos_tv, sin_tv);
    let (mut rc0, mut rs0) = (SCALE_6, 0i64);
    let (mut rc1, mut rs1) = (cos_tr, sin_tr);

    let vk0 = if is_otm {
        let chi = k - exp_a;
        let psi = ln_k - a;
        div6(2 * (mul6(k, psi)? - chi), ba)?
    } else {
        let chi = exp_b - k;
        let psi = b - ln_k;
        div6(2 * (chi - mul6(k, psi)?), ba)?
    };
    let mut total: i64 = mul6(SCALE_6 / 2, vk0)?;

    let mut i = 1usize;
    while i < NIG_N_6 {
        let w = div6((i as i64) * PI6, ba)?;

        if mul6(dt, gamma - w)? < -3 * SCALE_6 {
            break;
        }

        let wsq = mul6(w, w)?;
        let phi = if wsq > 4 * gsq {
            let inner_re = w + div6(gsq, 2 * w)?;
            let z_im = -2 * mul6(beta, w)?;
            let inner_im = div6(z_im, 2 * inner_re)?;
            let exp_arg = Complex6::new(
                mul6(dt, gamma - inner_re)?,
                mul6(w, cf_drift)? - mul6(dt, inner_im)?,
            );
            complex_exp6(exp_arg)?
        } else {
            nig_char6(w, cf_drift, dt, gamma, asq, beta)?
        };
        let rot = Complex6::new(rc1, -rs1);
        let ct = complex_mul6(phi, rot)?.re;

        let cost = vc1;
        let sint = vs1;
        let vk = if is_otm {
            let chi = div6(mul6(k, cost + mul6(w, sint)?)? - exp_a, SCALE_6 + wsq)?;
            let psi = div6(sint, w)?;
            div6(2 * (mul6(k, psi)? - chi), ba)?
        } else {
            let sk: i64 = if i % 2 == 0 { 1 } else { -1 };
            let chi = div6(sk * exp_b - mul6(k, cost + mul6(w, sint)?)?, SCALE_6 + wsq)?;
            let psi = -div6(sint, w)?;
            div6(2 * (chi - mul6(k, psi)?), ba)?
        };

        total += mul6(ct, vk)?;

        let vc_next = (2 * mul6(cos_tv, vc1)? - vc0).clamp(-SCALE_6, SCALE_6);
        let vs_next = (2 * mul6(cos_tv, vs1)? - vs0).clamp(-SCALE_6, SCALE_6);
        vc0 = vc1;
        vs0 = vs1;
        vc1 = vc_next;
        vs1 = vs_next;

        let rc_next = (2 * mul6(cos_tr, rc1)? - rc0).clamp(-SCALE_6, SCALE_6);
        let rs_next = (2 * mul6(cos_tr, rs1)? - rs0).clamp(-SCALE_6, SCALE_6);
        rc0 = rc1;
        rs0 = rs1;
        rc1 = rc_next;
        rs1 = rs_next;

        i += 1;
    }

    if is_otm {
        let put = mul6(disc, total)?;
        let put = if put > 0 { put } else { 0 };
        let call = put + s - mul6(k, disc)?;
        Ok(if call > 0 { call } else { 0 })
    } else {
        let call = mul6(disc, total)?;
        Ok(if call > 0 { call } else { 0 })
    }
}

/// On-chain NIG put pricing via put-call parity. ~120K CU.
/// Inputs at SCALE (1e12), computed internally at 1e6.
///
/// Put = Call - S + K × e^(-rT), computed using i64 helpers internally.
///
/// # Errors
/// - `DomainError` if parameters are invalid (same as `nig_call_64`).
/// - `Overflow` if intermediate arithmetic overflows.
///
/// # Precision
/// Same as `nig_call_64` — 95% within 0.5% for α ≥ 10, prices > $1.
///
/// # CU cost
/// ~120,000 CU.
pub fn nig_put_64(
    s: i64,
    k: i64,
    r: i64,
    t: i64,
    alpha: i64,
    beta: i64,
    delta: i64,
) -> Result<i64, SolMathError> {
    let call = nig_call_64(s, k, r, t, alpha, beta, delta)?;
    let disc = exp6(-mul6(r, t)?)?;
    let put_i = call - s + mul6(k, disc)?;
    Ok(if put_i > 0 { put_i } else { 0 })
}
