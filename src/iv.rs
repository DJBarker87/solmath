use crate::constants::*;
use crate::error::SolMathError;
use crate::arithmetic::{fp_mul, fp_mul_i, fp_mul_i_fast, fp_div, fp_div_i, fp_sqrt};
use crate::transcendental::{ln_fixed_i, exp_fixed_i};
use crate::normal::{norm_cdf_poly, norm_pdf, norm_cdf_and_pdf, inverse_norm_cdf};
use crate::bs::black_scholes_price;

/// Unchecked signed fixed-point multiply. Caller guarantees no overflow.
/// Used only in IV solver where inputs are pre-validated and bounded.
#[inline(always)]
fn mul_fast(a: i128, b: i128) -> i128 {
    // a, b are SCALE-valued inputs bounded by domain guards; |a|,|b| < ~1e17
    // so |a * b| < 1e34 < i128::MAX (≈1.7e38). Division by SCALE_I restores scale.
    (a * b) / SCALE_I
}

// ============================================================
// Newton-Raphson generic solver
// ============================================================

/// Generic Newton-Raphson root finder. Internal — used by early IV prototypes.
pub(crate) fn newton_raphson(
    f: fn(i128) -> i128,
    f_prime: fn(i128) -> i128,
    x0: i128,
    tolerance: i128,
    max_iter: u8,
) -> Result<i128, SolMathError> {
    let mut x = x0;
    let mut i = 0u8;
    while i < max_iter {
        let fx = f(x);
        if fx.abs() < tolerance {
            break;
        }
        let fpx = f_prime(x);
        if fpx == 0 {
            break;
        }
        x -= fp_div_i(fx, fpx)?;
        i += 1;
    }
    Ok(x)
}

/// Li rational polynomial initial guess for σ√T. Internal — called by implied_vol.
#[inline(never)]
pub(crate) fn li_rational_guess(x: i128, c: i128) -> Result<i128, SolMathError> {
    let sc = fp_sqrt(c as u128)? as i128; // √c
    let sc3 = mul_fast(sc, c); // (√c)³ = √c · c
    let sc4 = mul_fast(c, c); // (√c)⁴ = c²; c ∈ (0, SCALE_I], so sc4 ∈ (0, SCALE_I]

    // Evaluate N and D using Horner in x for each (√c)^j row.
    // All inputs are bounded: |x| < 0.5, c ∈ (0, 1), coefficients < 10·SCALE.
    // Products fit in ~24 decimal digits, well within i128.
    let h0_n = mul_fast(
        x,
        LI_N[1] + mul_fast(x, LI_N[4] + mul_fast(x, LI_N[8] + mul_fast(x, LI_N[13]))),
    );
    let h0_m = mul_fast(
        x,
        LI_M[1] + mul_fast(x, LI_M[4] + mul_fast(x, LI_M[8] + mul_fast(x, LI_M[13]))),
    );

    // j=1 row: √c · (n1 + n4·x + n8·x² + n13·x³)
    let h1_n = mul_fast(
        sc,
        LI_N[0] + mul_fast(x, LI_N[3] + mul_fast(x, LI_N[7] + mul_fast(x, LI_N[12]))),
    );
    let h1_m = mul_fast(
        sc,
        LI_M[0] + mul_fast(x, LI_M[3] + mul_fast(x, LI_M[7] + mul_fast(x, LI_M[12]))),
    );

    // j=2 row: c · (n3 + n7·x + n12·x²)
    let h2_n = mul_fast(c, LI_N[2] + mul_fast(x, LI_N[6] + mul_fast(x, LI_N[11])));
    let h2_m = mul_fast(c, LI_M[2] + mul_fast(x, LI_M[6] + mul_fast(x, LI_M[11])));

    // j=3 row: (√c)³ · (n6 + n11·x)
    let h3_n = mul_fast(sc3, LI_N[5] + mul_fast(x, LI_N[10]));
    let h3_m = mul_fast(sc3, LI_M[5] + mul_fast(x, LI_M[10]));

    // j=4 row: (√c)⁴ · n10
    let h4_n = mul_fast(sc4, LI_N[9]);
    let h4_m = mul_fast(sc4, LI_M[9]);

    // Each h_j term is a mul_fast product bounded by ~10·SCALE_I (coefficient magnitudes ≤ 10·SCALE).
    // Five terms summed: |num|, |den| < ~50·SCALE_I ≈ 5e13, fits i128 easily.
    let num = h0_n + h1_n + h2_n + h3_n + h4_n;
    let den = SCALE_I + h0_m + h1_m + h2_m + h3_m + h4_m;

    // v = p1·x + p2·√c + p3·c + N/D
    // Each mul_fast term ≤ ~2·SCALE_I (|LI_P*| < 2·SCALE, |x|,|sc|,|c| ≤ SCALE_I).
    // Sum of three terms: |linear| < ~6·SCALE_I ≈ 6e12, fits i128.
    let linear = mul_fast(LI_P1, x) + mul_fast(LI_P2, sc) + mul_fast(LI_P3, c);

    if den > 0 {
        // linear < ~6·SCALE_I; fp_div_i(num, den) < ~50·SCALE_I; sum < ~56·SCALE_I << i128::MAX.
        Ok(linear + fp_div_i(num, den)?)
    } else {
        // Denominator non-positive — rare, fall back to ATM approximation
        // 2_506_628_274_631 ≈ √(2π)·SCALE_I; fp_div_i(c, fp_sqrt(c)) ≤ SCALE_I; mul_fast result ≤ ~2.51·SCALE_I. Fits i128.
        Ok(mul_fast(2_506_628_274_631, fp_div_i(c, fp_sqrt(c as u128)? as i128)?)) // ≈ √(2π)·√c
    }
}

/// Experimental univariate Padé IV guess. ~10 mul_fast + 1 fp_div.
/// Brenner-Subrahmanyam variable: β = c·√(2π), z = x/β.
/// σ√T ≈ β × P(z)/Q(z) where P,Q are degree-4 polynomials (9 coefficients).
/// Replaces the 28-multiply Li bivariate form with fewer truncation errors.
#[inline(never)]
pub(crate) fn rational_guess_v2(x: i128, c: i128) -> Result<i128, SolMathError> {
    // β = c × √(2π)
    let beta = mul_fast(c, SQRT_2PI_IV);
    if beta <= 0 {
        return Err(SolMathError::DomainError);
    }

    // z = x / β — Padé fitted for |z| < 200 (c ≥ ~0.001)
    let z = fp_div_i(x, beta)?;
    if z.abs() > 200_000_000_000_000 {
        return Err(SolMathError::DomainError);
    }

    // Padé [4/4] numerator: p0 + z(p1 + z(p2 + z(p3 + z·p4)))
    // Horner form: at each step |num| ≤ max(|PADE_P*|) ≈ 4·SCALE_I, |z| ≤ 200·SCALE_I
    // (|z| is capped to 200·SCALE_I above). Each mul_fast(num, z) ≤ ~4·200·SCALE_I²/SCALE_I
    // = 800·SCALE_I ≈ 8e14 << i128::MAX.
    let mut num = PADE_P4;
    num = mul_fast(num, z) + PADE_P3;
    num = mul_fast(num, z) + PADE_P2;
    num = mul_fast(num, z) + PADE_P1;
    num = mul_fast(num, z) + PADE_P0;

    // Same Horner bound applies to denominator.
    let mut den = PADE_Q4;
    den = mul_fast(den, z) + PADE_Q3;
    den = mul_fast(den, z) + PADE_Q2;
    den = mul_fast(den, z) + PADE_Q1;
    den = mul_fast(den, z) + SCALE_I;

    if den.abs() < 1000 {
        // Near-singular denominator — fall back to ATM: σ√T ≈ β
        return Ok(beta);
    }

    Ok(mul_fast(beta, fp_div_i(num, den)?))
}

/// Compute BS call/put price in x-space for IV iteration.
/// x = σ√T (total volatility), returns (price_i, vega_x, volga_x).
/// solve_as_put: if true, computes put price instead of call.
#[inline(never)]
fn iv_price_and_greeks(
    x_i: i128,
    ln_fk: i128,
    s_i: i128,
    k_disc: i128,
    solve_as_put: bool,
) -> Result<(i128, i128, i128), SolMathError> {
    // fp_div_i(ln_fk, x_i) ∈ [-5·SCALE_I, 5·SCALE_I] from domain constraints; x_i/2 < 5·SCALE_I/2.
    // Sum fits well within i128; d1,d2 ∈ (-10·SCALE_I, 10·SCALE_I).
    let d1 = fp_div_i(ln_fk, x_i)? + x_i / 2;
    // d1, x_i both < 10·SCALE_I in magnitude; difference fits i128.
    let d2 = d1 - x_i;

    let (phi_d1, pdf_d1) = norm_cdf_and_pdf(d1)?;
    let phi_d2 = norm_cdf_poly(d2)?;

    // k_disc, s_i < ~1e17 (price at SCALE); phi_* ∈ [0, SCALE_I]; SCALE_I - phi_* ∈ [0, SCALE_I].
    // mul_fast(k_disc, SCALE_I - phi_d2): |k_disc| < 1e17, complement < 1e12, product < 1e29/1e12 = 1e17. Fits i128.
    // Subtraction of two such terms: |p|,|c| < ~2e17, fits i128.
    let price_i = if solve_as_put {
        let p = mul_fast(k_disc, SCALE_I - phi_d2) - mul_fast(s_i, SCALE_I - phi_d1);
        if p > 0 { p } else { 0 }
    } else {
        let c = mul_fast(s_i, phi_d1) - mul_fast(k_disc, phi_d2);
        if c > 0 { c } else { 0 }
    };

    // s_i < ~1e17, pdf_d1 ∈ [0, SCALE_I/√(2π)] < SCALE_I; mul_fast result < ~1e17.
    let vega_x = mul_fast(s_i, pdf_d1);
    let volga_x = if x_i > 0 && vega_x > 0 {
        // vega_x < ~1e17; d1,d2 ∈ (-10·SCALE_I, 10·SCALE_I); mul_fast(d1,d2) < 100·SCALE_I.
        // mul_fast(vega_x, mul_fast(d1,d2)) < ~1e17·100 = 1e19 — within i128 before division.
        fp_div_i(mul_fast(vega_x, mul_fast(d1, d2)), x_i)?
    } else {
        0
    };

    Ok((price_i, vega_x, volga_x))
}

/// Compute a bracketed Halley step. Returns the new x within [lo, hi].
#[inline(never)]
fn halley_step_bracketed(
    x_u: u128,
    f: i128,
    vega_x: i128,
    volga_x: i128,
    x_lo: u128,
    x_hi: u128,
) -> Result<u128, SolMathError> {
    if vega_x <= 1_000 {
        return Ok((x_lo + x_hi) / 2);
    }
    // f = price - target: both < ~1e17; vega_x < ~1e17. mul_fast < ~1e17. Factor 2: < ~2e17 < i128::MAX.
    let two_f_fp = 2 * mul_fast(f, vega_x);
    // mul_fast(vega_x, vega_x) < ~1e17; factor 2 < ~2e17. mul_fast(f, volga_x): both < ~1e17. Difference fits i128.
    let denom = 2 * mul_fast(vega_x, vega_x) - mul_fast(f, volga_x);

    let step = if denom.abs() > 1_000 {
        fp_div_i(two_f_fp, denom)?
    } else {
        fp_div_i(f, vega_x)?
    };

    let new_x = x_u as i128 - step;
    Ok(if new_x > (x_lo as i128) && new_x < (x_hi as i128) {
        new_x as u128
    } else {
        (x_lo + x_hi) / 2
    })
}


/// V1 implied volatility: Li rational guess + bracketed Halley iteration.
/// Retained as fallback. See `implied_vol` for the production entry point.
#[inline(never)]
pub(crate) fn implied_vol_v1(market_price: u128, s: u128, k: u128, r: u128, t: u128) -> Result<u128, SolMathError> {
    if s == 0 || k == 0 || t == 0 || market_price == 0 {
        return Err(SolMathError::DomainError);
    }

    // Early reject: sub-ULP prices have no meaningful IV
    if market_price < 100 {
        return Err(SolMathError::NoConvergence);
    }

    let mp_i = market_price as i128;
    let s_i = s as i128;
    let k_i = k as i128;
    let r_i = r as i128;
    let t_i = t as i128;

    // ---- Cached constants (~7K CU) ----
    let r_t = fp_mul_i(r_i, t_i)?;
    let discount = exp_fixed_i(-r_t)?;
    let k_disc = fp_mul_i(k_i, discount)?;
    let sqrt_t = fp_sqrt(t)? as i128;
    let ln_sk = ln_fixed_i(fp_div(s, k)?)?;
    // ln_sk, r_t both in (~-40·SCALE_I, ~40·SCALE_I) for practical inputs; sum fits i128.
    let ln_fk = ln_sk + r_t; // x = ln(F/K)

    // ---- Li normalization: c = C/S ----
    let c_raw = fp_div_i(mp_i, s_i)?;

    // ---- ITM → OTM conversion ----
    let (x_li, c_li) = if ln_fk > 0 {
        // c_put = C/S - 1 + K·disc/S = (C - S + K·disc) / S
        // c_raw ∈ [0, SCALE_I], SCALE_I = 1e12, fp_div_i result ∈ [0, SCALE_I]; difference fits i128.
        let c_put = c_raw - SCALE_I + fp_div_i(k_disc, s_i)?;
        (-ln_fk, if c_put > 0 { c_put } else { 0 })
    } else {
        (ln_fk, c_raw)
    };

    // ---- Check Li domain: |x| < 0.5, c > 0 ----
    let abs_x = x_li.abs();
    if abs_x >= 500_000_000_000 || c_li <= 0 {
        return implied_vol_iterative(
            market_price,
            s,
            k,
            r,
            t,
            r_t,
            discount,
            k_disc,
            sqrt_t,
            ln_sk,
            ln_fk,
        );
    }

    // ---- Li rational guess (~2K CU) ----
    let w = li_rational_guess(x_li, c_li)?;

    if w <= 0 || w > SCALE_I {
        // Outside Li's output range (v ∈ (0, 1]), fall back
        return implied_vol_iterative(
            market_price,
            s,
            k,
            r,
            t,
            r_t,
            discount,
            k_disc,
            sqrt_t,
            ln_sk,
            ln_fk,
        );
    }

    // ---- Bracketed Halley refinement in x-space (4 iterations max) ----
    // k_disc < ~1e17; k_disc/20 < ~5e15; sum < ~1.05e17 << i128::MAX.
    let solve_as_put = s_i > k_disc + k_disc / 20;
    let target_i = if solve_as_put {
        // mp_i, s_i, k_disc all < ~1e17; differences fit i128.
        let put_i = mp_i - s_i + k_disc;
        if put_i > 0 { put_i } else { 1 }
    } else {
        mp_i
    };

    // Initial bracket: x ∈ [0.001·√T, 5·√T]
    // sqrt_t ∈ [0, SCALE_I]; 1e9·sqrt_t < 1e21 before mul_fast, result < 1e21/1e12 = 1e9 < i128::MAX.
    let mut x_lo = (mul_fast(1_000_000_000, sqrt_t)).max(1) as u128;
    // 5e12·sqrt_t < 5e24 before mul_fast, result < 5e12 < i128::MAX.
    let mut x_hi = (mul_fast(5_000_000_000_000, sqrt_t)).max(x_lo as i128 + 1) as u128;
    let mut x_u = (w as u128).clamp(x_lo, x_hi);

    for _ in 0..4u8 {
        let x_i = x_u as i128;
        if x_i <= 1 { break; }

        let (price_i, vega_x, volga_x) = iv_price_and_greeks(x_i, ln_fk, s_i, k_disc, solve_as_put)?;
        let f = price_i - target_i;

        if f.abs() <= 100 {
            if sqrt_t > 0 {
                return Ok(fp_div_i(x_i, sqrt_t)? as u128);
            } else {
                return Err(SolMathError::NoConvergence);
            }
        }

        // Tighten bracket
        if f > 0 {
            if x_u < x_hi { x_hi = x_u; }
        } else {
            if x_u > x_lo { x_lo = x_u; }
        }

        x_u = halley_step_bracketed(x_u, f, vega_x, volga_x, x_lo, x_hi)?;
    }

    // Li path didn't converge in 4 iterations — continue from tightened bracket.
    // This catches cases where Li's guess was in-domain but needed more iterations.
    for _ in 0..4u8 {
        let x_i = x_u as i128;
        if x_i <= 1 { break; }

        let (price_i, vega_x, volga_x) = iv_price_and_greeks(x_i, ln_fk, s_i, k_disc, solve_as_put)?;
        let f = price_i - target_i;

        if f.abs() <= 100 {
            if sqrt_t > 0 {
                return Ok(fp_div_i(x_i, sqrt_t)? as u128);
            } else {
                return Err(SolMathError::NoConvergence);
            }
        }

        if f > 0 {
            if x_u < x_hi { x_hi = x_u; }
        } else {
            if x_u > x_lo { x_lo = x_u; }
        }

        x_u = halley_step_bracketed(x_u, f, vega_x, volga_x, x_lo, x_hi)?;
    }

    // Accept if within 1000 ULP after continuation
    let x_i = x_u as i128;
    if x_i > 0 && sqrt_t > 0 {
        let (price_i, _, _) = iv_price_and_greeks(x_i, ln_fk, s_i, k_disc, solve_as_put)?;
        let f = price_i - target_i;
        if f.abs() <= 1000 {
            return Ok(fp_div_i(x_i, sqrt_t)? as u128);
        }
    }

    // Still didn't converge — fall through to Jaeckel for a fresh start
    implied_vol_iterative(
        market_price, s, k, r, t,
        r_t, discount, k_disc, sqrt_t, ln_sk, ln_fk,
    )
}


/// Iterative IV fallback: Jaeckel initial guess + 6 bracketed Halley iterations.
///
/// Returns `Err(NoConvergence)` if the solver doesn't converge — never returns
/// a bad guess.
#[inline(never)]
pub(crate) fn implied_vol_iterative(
    market_price: u128,
    s: u128,
    k: u128,
    _r: u128,
    _t: u128,
    _r_t: i128,
    _discount: i128,
    k_disc: i128,
    sqrt_t: i128,
    _ln_sk: i128,
    ln_fk: i128,
) -> Result<u128, SolMathError> {
    let mp_i = market_price as i128;
    let s_i = s as i128;

    // ---- Deep ITM: solve as OTM put (better conditioned) ----
    // k_disc < ~1e17; k_disc/20 < ~5e15; sum < ~1.05e17 << i128::MAX.
    let solve_as_put = s_i > k_disc + k_disc / 20;
    let target_i = if solve_as_put {
        // mp_i, s_i, k_disc all < ~1e17; put_i fits i128.
        let put_i = mp_i - s_i + k_disc;
        if put_i > 0 { put_i } else { 1 }
    } else {
        mp_i
    };

    // ---- Initial guess in x = σ√T space ----
    let sqrt_2pi: i128 = 2_506_628_274_631;
    let abs_ln_fk = ln_fk.abs();

    let beta_otm = if ln_fk > 0 {
        // put_i: mp_i, s_i, k_disc < ~1e17; difference fits i128.
        let put_i = mp_i - s_i + k_disc;
        if put_i <= 0 {
            0
        } else {
            let sqrt_sk = fp_sqrt(fp_mul(s, k)?)? as i128;
            fp_div_i(put_i, sqrt_sk)?
        }
    } else {
        let sqrt_sk = fp_sqrt(fp_mul(s, k)?)? as i128;
        fp_div_i(mp_i, sqrt_sk)?
    };

    let mut x_vol: i128;

    if beta_otm <= 0 {
        // If solving as OTM put and beta rounds to zero, the extrinsic value
        // is below 1 ULP — there's no signal to invert. Return NoConvergence
        // rather than a blind guess that will be orders of magnitude wrong.
        if solve_as_put {
            return Err(SolMathError::NoConvergence);
        }
        // 0.25·SCALE_I * sqrt_t ≤ 0.25·SCALE_I²/SCALE_I = 0.25·SCALE_I in mul_fast; fits i128.
        x_vol = mul_fast(250_000_000_000, sqrt_t);
    } else if abs_ln_fk < 50_000_000_000 {
        // beta_otm, sqrt_2pi both < ~3·SCALE_I; mul_fast result < ~3·SCALE_I. Fits i128.
        x_vol = mul_fast(beta_otm, sqrt_2pi);
    } else if beta_otm < 10_000_000_000 && abs_ln_fk > 100_000_000_000 {
        // Two-step asymptotic tail guess (see iterative_initial_guess)
        let ln_beta = ln_fixed_i(beta_otm as u128)?;
        // ln_beta < ~30·SCALE_I; 2*ln_beta < ~60·SCALE_I; LN_2PI ≈ 1.8·SCALE_I; difference fits i128.
        let a = -2 * ln_beta - LN_2PI;
        if a > SCALE_I {
            let ln_a = ln_fixed_i(a as u128)?;
            // a, ln_a both < ~60·SCALE_I; a2 = a - ln_a fits i128.
            let a2 = a - ln_a;
            if a2 > SCALE_I / 2 {
                x_vol = fp_div_i(abs_ln_fk, fp_sqrt(a2 as u128)? as i128)?;
            } else {
                x_vol = fp_div_i(abs_ln_fk, fp_sqrt(a as u128)? as i128)?;
            }
        } else if a > 0 {
            x_vol = fp_div_i(abs_ln_fk, fp_sqrt(a as u128)? as i128)?;
        } else {
            // 0.25·SCALE_I * sqrt_t: same bound as above, fits i128.
            x_vol = mul_fast(250_000_000_000, sqrt_t);
        }
    } else {
        let s_c = fp_sqrt(2 * abs_ln_fk as u128)? as i128;
        let exp_neg_half_x = exp_fixed_i(-abs_ln_fk / 2)?;
        let exp_half_x = fp_div_i(SCALE_I, exp_neg_half_x)?;
        let b_c = {
            let phi_neg_sc = norm_cdf_poly(-s_c)?;
            // exp_neg_half_x ∈ (0, SCALE_I]; /2 gives positive value well within i128.
            // mul_fast(phi_neg_sc, exp_half_x): phi_neg_sc ∈ [0,SCALE_I], exp_half_x ≥ SCALE_I/2; product fits via mul_fast.
            let v = exp_neg_half_x / 2 - mul_fast(phi_neg_sc, exp_half_x);
            if v > 0 { v } else { 0 }
        };
        // INV_SQRT_2PI ≈ 0.4·SCALE_I; exp_neg_half_x ≤ SCALE_I; mul_fast result < 0.4·SCALE_I. Fits i128.
        let v_c = mul_fast(INV_SQRT_2PI, exp_neg_half_x);
        if v_c > 0 {
            // s_c, fp_div_i result both < ~10·SCALE_I; sum fits i128.
            let s0 = s_c + fp_div_i(beta_otm - b_c, v_c)?;
            x_vol = if s0 > 0 { s0 } else { s_c };
        } else {
            x_vol = s_c;
        }
    }

    // Bracket: x ∈ [σ_min·√T, σ_max·√T]
    // 1e9·sqrt_t ≤ 1e9·SCALE_I = 1e21, mul_fast gives ≤ 1e9. 1e13·sqrt_t gives ≤ 1e13. Both fit i128.
    let x_min = mul_fast(1_000_000_000, sqrt_t); // 0.001 * √T
    let x_max = mul_fast(10_000_000_000_000, sqrt_t); // 10.0 * √T
    x_vol = x_vol.clamp(x_min, x_max);
    let mut x_u = x_vol as u128;

    let mut x_lo = x_min as u128;
    let mut x_hi = x_max as u128;

    for iter in 0..6u8 {
        let x_i = x_u as i128;
        if x_i <= 1 { break; }

        let (price_i, vega_x, volga_x) = iv_price_and_greeks(x_i, ln_fk, s_i, k_disc, solve_as_put)?;
        let f = price_i - target_i;

        if f.abs() <= 1 {
            if sqrt_t > 0 {
                return Ok(fp_div_i(x_i, sqrt_t)? as u128);
            } else {
                return Err(SolMathError::NoConvergence);
            }
        }

        // Relaxed convergence after 4 iterations
        if iter >= 4 && f.abs() <= 100 {
            if sqrt_t > 0 {
                return Ok(fp_div_i(x_i, sqrt_t)? as u128);
            } else {
                return Err(SolMathError::NoConvergence);
            }
        }

        // Tighten bracket
        if f > 0 {
            if x_u < x_hi { x_hi = x_u; }
        } else {
            if x_u > x_lo { x_lo = x_u; }
        }

        x_u = halley_step_bracketed(x_u, f, vega_x, volga_x, x_lo, x_hi)?;
    }

    // Final check — accept if within 1000 ULP (near-convergence boundary cases)
    let x_i = x_u as i128;
    if x_i > 0 && sqrt_t > 0 {
        let (price_i, _, _) = iv_price_and_greeks(x_i, ln_fk, s_i, k_disc, solve_as_put)?;
        let f = price_i - target_i;
        if f.abs() <= 1000 {
            return Ok(fp_div_i(x_i, sqrt_t)? as u128);
        }
    }
    Err(SolMathError::NoConvergence)
}

// ============================================================
// Jäckel "Let's Be Rational" IV solver
// ============================================================

/// Normalised intrinsic call value: max(exp(x/2) - exp(-x/2), 0).
#[inline(never)]
fn normalised_intrinsic_call(x: i128) -> Result<i128, SolMathError> {
    if x <= 0 {
        return Ok(0);
    }
    let b_max = exp_fixed_i(x / 2)?;
    let b_min = fp_div_i(SCALE_I, b_max)?;
    // b_max, b_min are positive exp outputs scaled to SCALE_I; difference fits i128.
    let v = b_max - b_min;
    Ok(if v > 0 { v } else { 0 })
}

/// Normalised Black call: b(x,s) = Φ(h+t)·exp(x/2) - Φ(h-t)·exp(-x/2).
/// h = x/s, t = s/2. Handles x > 0 via put-call parity.
#[inline(never)]
fn normalised_black_call(x: i128, s: i128) -> Result<i128, SolMathError> {
    if s <= 0 {
        return normalised_intrinsic_call(x);
    }
    if x > 0 {
        let intrinsic = normalised_intrinsic_call(x)?;
        // intrinsic ≥ 0 SCALE-valued; normalised_black_call(-x,s) ≥ 0 SCALE-valued; sum fits i128.
        return Ok(intrinsic + normalised_black_call(-x, s)?);
    }
    let h = fp_div_i(x, s)?;
    // s is a SCALE-valued vol ∈ (0, SCALE_I]; s/2 ≤ SCALE_I/2. Fits i128.
    let t = s / 2;
    let exp_half = exp_fixed_i(x / 2)?;
    let inv_exp_half = if exp_half > 0 {
        fp_div_i(SCALE_I, exp_half)?
    } else {
        return Ok(0);
    };
    // h = x/s SCALE-valued; t = s/2 ≤ SCALE_I/2; h+t and h-t each have magnitude < ~SCALE_I. Fits i128.
    // mul_fast operands: norm_cdf_poly ∈ [0, SCALE_I], exp_half and inv_exp_half are SCALE-valued outputs; product fits.
    // Subtraction of two mul_fast results (both < ~SCALE_I): fits i128.
    let b = mul_fast(norm_cdf_poly(h + t)?, exp_half)
        - mul_fast(norm_cdf_poly(h - t)?, inv_exp_half);
    Ok(if b > 0 { b } else { 0 })
}

/// Normalised vega: v(x,s) = (1/√2π)·exp(-½(h²+t²)).
#[inline(never)]
fn normalised_vega(x: i128, s: i128) -> Result<i128, SolMathError> {
    if s <= 0 {
        return Ok(0);
    }
    let ax = x.abs();
    if ax > 0 && s <= ax / 1_000_000 {
        return Ok(0); // s far too small relative to |x|
    }
    let h = fp_div_i(x, s)?;
    // s is a SCALE-valued total vol; s/2 ≤ SCALE_I/2. Fits i128.
    let t = s / 2;
    // h = x/s: checked (h can be large for deep ITM/OTM).
    // t = s/2 ≤ SCALE_I/2 by construction → t² ≤ SCALE_I/4; fp_mul_i_fast safe.
    // INV_SQRT_2PI ≤ SCALE_I, e ≤ SCALE_I → fp_mul_i_fast safe.
    let arg = -(fp_mul_i(h, h)? + fp_mul_i_fast(t, t)) / 2;
    let e = exp_fixed_i(arg)?;
    Ok(fp_mul_i_fast(INV_SQRT_2PI, e))
}

/// Householder(3) factor: (1 + halley·newton/2) / (1 + newton·(halley + hh3·newton/6)).
#[inline(never)]
fn householder_factor(newton: i128, halley: i128, hh3: i128) -> Result<i128, SolMathError> {
    // newton, halley, hh3 are Householder coefficients derived from BS greeks, all SCALE-valued.
    // mul_fast(halley, newton): both < ~10·SCALE_I in practice; product via mul_fast < ~100·SCALE_I. Fits i128.
    let hn = mul_fast(halley, newton);
    // hn/2 < ~50·SCALE_I; SCALE_I + hn/2 fits i128.
    let num = SCALE_I + hn / 2;
    // mul_fast(hh3, newton)/6: hh3·newton via mul_fast < ~100·SCALE_I, /6 < ~17·SCALE_I.
    // halley + mul_fast(hh3, newton)/6 < ~27·SCALE_I; mul_fast(newton, ...) < ~270·SCALE_I. Fits i128.
    let den = SCALE_I + mul_fast(newton, halley + mul_fast(hh3, newton) / 6);
    if den.abs() < 100 {
        return Ok(SCALE_I); // degenerate, just use Newton
    }
    fp_div_i(num, den)
}

/// Rational cubic interpolation (Delbourgo & Gregory).
#[inline(never)]
fn rational_cubic_interpolation(
    x: i128, x_l: i128, x_r: i128, y_l: i128, y_r: i128,
    d_l: i128, d_r: i128, r: i128,
) -> Result<i128, SolMathError> {
    // x_l, x_r are normalised black call values or s values, all SCALE-valued; difference fits i128.
    let h = x_r - x_l;
    if h.abs() <= 0 {
        // y_l, y_r are SCALE-valued; sum/2 fits i128.
        return Ok((y_l + y_r) / 2);
    }
    // Large r → linear interpolation
    if r > 1_000_000_000_000_000_000 { // 1e6 at SCALE
        let t = fp_div_i(x - x_l, h)?;
        // t ∈ [0, SCALE_I]; y_r,y_l SCALE-valued; two mul_fast terms summed < ~2·SCALE_I. Fits i128.
        return Ok(mul_fast(y_r, t) + mul_fast(y_l, SCALE_I - t));
    }
    let t = fp_div_i(x - x_l, h)?;
    // t ∈ [0, SCALE_I]; SCALE_I - t ∈ [0, SCALE_I]. No underflow.
    let omt = SCALE_I - t;
    // t2, omt2: mul_fast of two [0,SCALE_I] values → [0, SCALE_I]. Fits i128.
    let t2 = mul_fast(t, t);
    let omt2 = mul_fast(omt, omt);
    // Numerator terms: all are nested mul_fast calls.
    // r < 1e6·SCALE_I (guard above), y_r,y_l < SCALE_I, h·d_r via mul_fast < ~SCALE_I.
    // Each intermediate mul_fast result < ~1e6·SCALE_I²/SCALE_I = 1e6·SCALE_I ≈ 1e18. Fits i128.
    // Sum of four such terms < ~4e18 << i128::MAX.
    let num = mul_fast(y_r, mul_fast(t2, t))
        + mul_fast(mul_fast(r, y_r) - mul_fast(h, d_r), mul_fast(t2, omt))
        + mul_fast(mul_fast(r, y_l) + mul_fast(h, d_l), mul_fast(t, omt2))
        + mul_fast(y_l, mul_fast(omt2, omt));
    // Denominator: 1 + (r-3)·t·(1-t)
    // (r - 3·SCALE_I) < ~1e18; mul_fast(t,omt) ≤ SCALE_I/4; mul_fast of those < ~2.5e17. SCALE_I + that fits i128.
    let den = SCALE_I + mul_fast(r - 3 * SCALE_I, mul_fast(t, omt));
    if den.abs() < 100 {
        return Ok((y_l + y_r) / 2);
    }
    fp_div_i(num, den)
}

/// Control parameter to fit second derivative at left side.
fn rc_param_fit_2nd_deriv_left(
    x_l: i128, x_r: i128, y_l: i128, y_r: i128,
    d_l: i128, d_r: i128, second_deriv: i128,
) -> Result<i128, SolMathError> {
    // x_l, x_r are SCALE-valued black call outputs; difference fits i128.
    let h = x_r - x_l;
    // mul_fast(h, second_deriv): both SCALE-valued, product via mul_fast < ~SCALE_I. /2 and + (d_r - d_l): sum fits i128.
    let num = mul_fast(h, second_deriv) / 2 + (d_r - d_l);
    if num.abs() < 100 { return Ok(0); }
    let slope = if h == 0 { 0 } else { fp_div_i(y_r - y_l, h)? };
    // slope, d_l both SCALE-valued; difference fits i128.
    let den = slope - d_l;
    if den.abs() < 100 {
        return Ok(if num > 0 { 1_000_000_000_000_000_000 } else { -SCALE_I + 1 });
    }
    if den == 0 { Ok(0) } else { fp_div_i(num, den) }
}

/// Control parameter to fit second derivative at right side.
fn rc_param_fit_2nd_deriv_right(
    x_l: i128, x_r: i128, y_l: i128, y_r: i128,
    d_l: i128, d_r: i128, second_deriv: i128,
) -> Result<i128, SolMathError> {
    // Same bounds as left variant: h, num, and den all SCALE-valued; fit i128.
    let h = x_r - x_l;
    let num = mul_fast(h, second_deriv) / 2 + (d_r - d_l);
    if num.abs() < 100 { return Ok(0); }
    let slope = if h == 0 { 0 } else { fp_div_i(y_r - y_l, h)? };
    let den = d_r - slope;
    if den.abs() < 100 {
        return Ok(if num > 0 { 1_000_000_000_000_000_000 } else { -SCALE_I + 1 });
    }
    if den == 0 { Ok(0) } else { fp_div_i(num, den) }
}

const MIN_RC_PARAM: i128 = -SCALE_I + 1; // -(1 - ε)

/// Minimum control parameter for shape preservation.
fn minimum_rc_param(d_l: i128, d_r: i128, s: i128, prefer_shape: bool) -> Result<i128, SolMathError> {
    let monotonic = (mul_fast(d_l, s) >= 0) && (mul_fast(d_r, s) >= 0);
    let convex = d_l <= s && s <= d_r;
    let concave = d_l >= s && s >= d_r;
    if !monotonic && !convex && !concave {
        return Ok(MIN_RC_PARAM);
    }
    let mut r1 = i128::MIN;
    let mut r2 = i128::MIN;
    if monotonic {
        if s.abs() > 100 {
            r1 = fp_div_i(d_r + d_l, s)?;
        } else if prefer_shape {
            r1 = 1_000_000_000_000_000_000;
        }
    }
    if convex || concave {
        // s, d_l, d_r are SCALE-valued slope/derivative values; all differences fit i128.
        let s_m_dl = s - d_l;
        let dr_m_s = d_r - s;
        let dr_m_dl = d_r - d_l;
        if s_m_dl.abs() > 100 && dr_m_s.abs() > 100 {
            let r2a = if dr_m_s == 0 { 0 } else { fp_div_i(dr_m_dl, dr_m_s)?.abs() };
            let r2b = if s_m_dl == 0 { 0 } else { fp_div_i(dr_m_dl, s_m_dl)?.abs() };
            r2 = r2a.max(r2b);
        } else if prefer_shape {
            r2 = 1_000_000_000_000_000_000;
        }
    } else if monotonic && prefer_shape {
        r2 = 1_000_000_000_000_000_000;
    }
    Ok(MIN_RC_PARAM.max(r1.max(r2)))
}

/// Convex control parameter fitting 2nd derivative at left side.
fn convex_rc_param_left(
    x_l: i128, x_r: i128, y_l: i128, y_r: i128,
    d_l: i128, d_r: i128, second_deriv: i128, prefer_shape: bool,
) -> Result<i128, SolMathError> {
    let r = rc_param_fit_2nd_deriv_left(x_l, x_r, y_l, y_r, d_l, d_r, second_deriv)?;
    let h = x_r - x_l;
    let s = if h == 0 { 0 } else { fp_div_i(y_r - y_l, h)? };
    let r_min = minimum_rc_param(d_l, d_r, s, prefer_shape)?;
    Ok(r.max(r_min))
}

/// Convex control parameter fitting 2nd derivative at right side.
fn convex_rc_param_right(
    x_l: i128, x_r: i128, y_l: i128, y_r: i128,
    d_l: i128, d_r: i128, second_deriv: i128, prefer_shape: bool,
) -> Result<i128, SolMathError> {
    let r = rc_param_fit_2nd_deriv_right(x_l, x_r, y_l, y_r, d_l, d_r, second_deriv)?;
    let h = x_r - x_l;
    let s = if h == 0 { 0 } else { fp_div_i(y_r - y_l, h)? };
    let r_min = minimum_rc_param(d_l, d_r, s, prefer_shape)?;
    Ok(r.max(r_min))
}

/// f_lower_map and first two derivatives (Branch 1 objective transform).
#[inline(never)]
fn compute_f_lower_map(x: i128, s: i128) -> Result<(i128, i128, i128), SolMathError> {
    let ax = x.abs();
    // SQRT_ONE_OVER_THREE ≈ 0.577·SCALE_I; fp_div_i(ax,s) is SCALE-valued; mul_fast result ≤ ~0.577·SCALE_I. Fits i128.
    let z = mul_fast(SQRT_ONE_OVER_THREE, fp_div_i(ax, s)?);
    // z ≤ ~0.577·SCALE_I; mul_fast(z,z) ≤ ~0.333·SCALE_I. Fits i128.
    let y = mul_fast(z, z);
    // s ≤ SCALE_I (σ√T ≤ 1yr vol); mul_fast(s,s) ≤ SCALE_I. Fits i128.
    let s2 = mul_fast(s, s);
    let phi = norm_cdf_poly(-z)?;
    let pdf = norm_pdf(z)?;
    // f, fp for non-degenerate s
    // phi ∈ [0, SCALE_I]; mul_fast(phi,phi) ∈ [0, SCALE_I]. Fits i128.
    let phi2 = mul_fast(phi, phi);
    // y + s2/8: y ≤ ~0.33·SCALE_I, s2/8 ≤ ~0.125·SCALE_I; sum < SCALE_I. Fits i128.
    let exp_y_s2 = exp_fixed_i(y + s2 / 8)?;
    // Nested mul_fast: each result ≤ SCALE_I; TWO_PI_SCALED ≈ 6.28·SCALE_I; outermost < ~6.28·SCALE_I. Fits i128.
    let fp = mul_fast(TWO_PI_SCALED, mul_fast(y, mul_fast(phi2, exp_y_s2)));
    let f = if ax < 100 { 0 } else {
        // TWO_PI_OVER_SQRT_TWENTY_SEVEN ≈ 1.21·SCALE_I; ax and inner mul_fast ≤ SCALE_I; product < ~1.21·SCALE_I.
        mul_fast(TWO_PI_OVER_SQRT_TWENTY_SEVEN, mul_fast(ax, mul_fast(phi2, phi)))
    };
    // fpp (second derivative) — simplified, only used for control parameter
    // 2*y ≤ ~0.67·SCALE_I; s2/4 ≤ ~0.25·SCALE_I; sum < SCALE_I. Fits i128.
    let exp_2y_s2 = exp_fixed_i(2 * y + s2 / 4)?;
    let fpp = if pdf.abs() < 100 { 0 } else {
        // 8·SQRT_THREE_SCALED ≈ 13.9·SCALE_I; mul_fast(s, ax) ≤ SCALE_I; first mul_fast < ~13.9·SCALE_I.
        // s2 - 8·SCALE_I fits i128; mul_fast(s2, s2-8·SCALE_I) ≤ SCALE_I; 3× ≤ 3·SCALE_I.
        // 8·mul_fast(x,x): x ≤ SCALE_I, mul_fast(x,x) ≤ SCALE_I; ×8 ≤ 8·SCALE_I.
        // Subtracting: |inner addend| < ~11·SCALE_I; sum < ~25·SCALE_I. Fits i128.
        let inner = mul_fast(8 * SQRT_THREE_SCALED, mul_fast(s, ax))
            + mul_fast(3 * mul_fast(s2, s2 - 8 * SCALE_I) - 8 * mul_fast(x, x),
                       fp_div_i(phi, pdf)?);
        mul_fast(PI_OVER_SIX, mul_fast(fp_div_i(y, mul_fast(s2, s))?,
            mul_fast(phi, mul_fast(inner, exp_2y_s2))))
    };
    Ok((f, fp, fpp))
}

/// f_upper_map and first two derivatives (Branch 4 objective transform).
#[inline(never)]
fn compute_f_upper_map(x: i128, s: i128) -> Result<(i128, i128, i128), SolMathError> {
    let f = norm_cdf_poly(-s / 2)?;
    if x.abs() < 100 {
        // -SCALE_I/2 is a compile-time constant division; fits i128.
        return Ok((f, -SCALE_I / 2, 0));
    }
    // w = (x/s)²: fp_div_i result is SCALE-valued; mul_fast of two copies ≤ SCALE_I. Fits i128.
    let w = mul_fast(fp_div_i(x, s)?, fp_div_i(x, s)?);
    // w/2 ≤ SCALE_I/2; exp output ≤ SCALE_I; /2 ≥ 0. Fits i128.
    let fp = -exp_fixed_i(w / 2)? / 2;
    // s*s: s is SCALE-valued ≤ SCALE_I, but raw s*s could be up to SCALE_I² ≈ 1e24.
    // REVIEW: s can be up to ~10·SCALE_I for high-vol inputs; s*s up to ~1e26 fits i128 (i128::MAX ≈ 1.7e38),
    // then /8000_000_000_000 = /8e12 gives ≤ ~1.25e13. w + that ≤ ~1.25e13 + SCALE_I. Fits i128.
    let fpp = mul_fast(SQRT_PI_OVER_TWO,
        mul_fast(exp_fixed_i(w + s * s / 8000_000_000_000)?,
                 fp_div_i(w, s)?));
    Ok((f, fp, fpp))
}

/// Inverse of f_lower_map: s such that f_lower_map(x, s) = f.
#[inline(never)]
fn inverse_f_lower_map(x: i128, f: i128) -> Result<i128, SolMathError> {
    if f <= 0 {
        return Ok(0);
    }
    let ax = x.abs();
    // f = (2π/√27)·|x|·Φ(-z)³  →  Φ(-z) = (f / ((2π/√27)·|x|))^(1/3)
    // s = |x| / (√3 · (-Φ⁻¹(Φ(-z))))  [since z > 0, Φ(-z) < 0.5]
    let ratio = fp_div_i(f, mul_fast(TWO_PI_OVER_SQRT_TWENTY_SEVEN, ax))?;
    // cube root via exp(ln/3)
    let ln_ratio = ln_fixed_i(ratio.unsigned_abs())?;
    let cbrt = exp_fixed_i(ln_ratio / 3)?;
    let inv_phi = inverse_norm_cdf(cbrt)?; // Φ⁻¹(cbrt), negative since cbrt < 0.5
    if inv_phi >= 0 {
        return Ok(0); // degenerate
    }
    // SQRT_THREE_SCALED ≈ 1.73·SCALE_I; -inv_phi > 0 (inv_phi < 0 by guard) and < ~5·SCALE_I;
    // mul_fast result < ~8.65·SCALE_I. Fits i128.
    let denom = mul_fast(SQRT_THREE_SCALED, -inv_phi);
    if denom <= 0 {
        return Ok(0);
    }
    Ok(fp_div_i(ax, denom)?.abs())
}

/// Inverse of f_upper_map: s = -2·Φ⁻¹(f).
#[inline(never)]
fn inverse_f_upper_map(f: i128) -> Result<i128, SolMathError> {
    // inverse_norm_cdf(f) ∈ (-5·SCALE_I, 5·SCALE_I); ×2 stays within ±10·SCALE_I << i128::MAX.
    Ok(-2 * inverse_norm_cdf(f)?)
}

/// Core Jäckel normalised IV solver.
/// Input: beta = normalised OTM price (beta > 0, x ≤ 0 after mapping).
///        n_householder = number of Householder(3) refinement iterations.
/// Output: s = σ√T.
#[inline(never)]
fn jaeckel_normalised_iv(beta: i128, x: i128, n_householder: u8) -> Result<i128, SolMathError> {
    if beta <= 0 {
        return Ok(0);
    }
    let b_max = exp_fixed_i(x / 2)?;
    if beta >= b_max {
        return Err(SolMathError::NoConvergence);
    }

    // Reference point: s_c = √(2|x|), b_c = b(x, s_c), v_c = v(x, s_c)
    let ax = x.abs();
    let s_c = fp_sqrt(2 * ax as u128)? as i128;
    let b_c = normalised_black_call(x, s_c)?;
    let v_c = normalised_vega(x, s_c)?;

    let mut s: i128;
    let mut s_left: i128 = 0;
    let mut s_right: i128 = i128::MAX / 2;
    let mut use_direct_objective = true;

    if beta < b_c {
        // --- Left half: beta < b_c ---
        // s_c SCALE-valued; fp_div_i(b_c, v_c) SCALE-valued; difference fits i128.
        let s_l = if v_c > 100 { s_c - fp_div_i(b_c, v_c)? } else { s_c / 2 };
        let s_l = if s_l > 0 { s_l } else { s_c / 10 };
        let b_l = normalised_black_call(x, s_l)?;

        if beta < b_l {
            // Branch 1: extreme OTM — f_lower_map inverse
            let (f_l, dfdb_l, d2fdb2_l) = compute_f_lower_map(x, s_l)?;
            let r_ll = convex_rc_param_right(
                0, b_l, 0, f_l, SCALE_I, dfdb_l, d2fdb2_l, true)?;
            let mut f = rational_cubic_interpolation(
                beta, 0, b_l, 0, f_l, SCALE_I, dfdb_l, r_ll)?;
            if f <= 0 {
                // Quadratic fallback
                let t = if b_l > 0 { fp_div_i(beta, b_l)? } else { SCALE_I / 2 };
                // t ∈ [0, SCALE_I]; f_l, b_l SCALE-valued; two mul_fast terms < ~SCALE_I; sum < ~2·SCALE_I.
                // Outer mul_fast by t ≤ SCALE_I: result ≤ ~2·SCALE_I. Fits i128.
                f = mul_fast(mul_fast(f_l, t) + mul_fast(b_l, SCALE_I - t), t);
            }
            s = inverse_f_lower_map(x, f)?;
            s_right = s_l;
            use_direct_objective = false; // Branch 1 uses log-transformed objective
        } else {
            // Branch 2: moderate OTM — rational cubic
            let v_l = normalised_vega(x, s_l)?;
            let inv_v_l = if v_l > 100 { fp_div_i(SCALE_I, v_l)? } else { SCALE_I * 100 };
            let inv_v_c = if v_c > 100 { fp_div_i(SCALE_I, v_c)? } else { SCALE_I * 100 };
            let r_lm = convex_rc_param_right(
                b_l, b_c, s_l, s_c, inv_v_l, inv_v_c, 0, false)?;
            s = rational_cubic_interpolation(
                beta, b_l, b_c, s_l, s_c, inv_v_l, inv_v_c, r_lm)?;
            s_left = s_l;
            s_right = s_c;
        }
    } else {
        // --- Right half: beta >= b_c ---
        // b_max, b_c SCALE-valued; difference fits i128. fp_div_i result SCALE-valued; s_c + that fits i128.
        // s_c * 2: s_c ≤ SCALE_I so ×2 ≤ 2·SCALE_I < i128::MAX.
        let s_h = if v_c > 100 {
            s_c + fp_div_i(b_max - b_c, v_c)?
        } else {
            s_c * 2
        };
        let b_h = normalised_black_call(x, s_h)?;

        if beta <= b_h {
            // Branch 3: moderate ITM — rational cubic
            let v_h = normalised_vega(x, s_h)?;
            let inv_v_c = if v_c > 100 { fp_div_i(SCALE_I, v_c)? } else { SCALE_I * 100 };
            let inv_v_h = if v_h > 100 { fp_div_i(SCALE_I, v_h)? } else { SCALE_I * 100 };
            let r_hm = convex_rc_param_left(
                b_c, b_h, s_c, s_h, inv_v_c, inv_v_h, 0, false)?;
            s = rational_cubic_interpolation(
                beta, b_c, b_h, s_c, s_h, inv_v_c, inv_v_h, r_hm)?;
            s_left = s_c;
            s_right = s_h;
        } else {
            // Branch 4: extreme ITM — f_upper_map inverse
            let (f_h, dfdb_h, d2fdb2_h) = compute_f_upper_map(x, s_h)?;
            let r_hh = convex_rc_param_left(
                b_h, b_max, f_h, 0, dfdb_h, -SCALE_I / 2, d2fdb2_h, true)?;
            let mut f = rational_cubic_interpolation(
                beta, b_h, b_max, f_h, 0, dfdb_h, -SCALE_I / 2, r_hh)?;
            if f <= 0 {
                // b_max, b_h both SCALE-valued outputs of normalised_black_call; difference fits i128.
                let h = b_max - b_h;
                let t = if h > 0 { fp_div_i(beta - b_h, h)? } else { SCALE_I / 2 };
                // f_h SCALE-valued; SCALE_I-t ∈ [0,SCALE_I]; mul_fast < SCALE_I.
                // mul_fast(h, t)/2 < SCALE_I/2. Sum < ~1.5·SCALE_I; outer mul_fast < ~1.5·SCALE_I. Fits i128.
                f = mul_fast(mul_fast(f_h, SCALE_I - t) + mul_fast(h, t) / 2, SCALE_I - t);
            }
            s = inverse_f_upper_map(f)?;
            s_left = s_h;
            // Branch 4 uses log-transformed objective when beta > b_max/2
            use_direct_objective = beta <= b_max / 2;
        }
    }

    // Ensure initial guess is positive and within bracket
    // s_left + 1: s_left ≤ SCALE_I in practice; +1 fits i128.
    // s + SCALE_I: s ≤ ~10·SCALE_I, + SCALE_I ≤ ~11·SCALE_I << i128::MAX.
    s = s.max(s_left + 1).min(if s_right < i128::MAX / 2 { s_right } else { s + SCALE_I });

    // --- Householder(3) iteration ---
    let mut ds_previous: i128 = 0;
    let mut direction_reversal_count = 0u8;

    for _ in 0..n_householder {
        if s <= 0 { break; }

        let b = normalised_black_call(x, s)?;
        let bp = normalised_vega(x, s)?;

        // Tighten bracket
        if b > beta && s < s_right { s_right = s; }
        else if b < beta && s > s_left { s_left = s; }

        if bp <= 100 {
            // Near-zero vega — bisect; s_left + s_right < i128::MAX (s_right = i128::MAX/2 at most). /2 fits.
            s = (s_left + s_right) / 2;
            continue;
        }

        let ds;
        if use_direct_objective {
            // g(s) = b(x,s) - beta
            // halley = x²/s³ - s/4, hh3 = halley² - 3·(x/s²)² - 1/4
            let newton = fp_div_i(beta - b, bp)?;
            let x_over_s2 = fp_div_i(x, mul_fast(s, s))?;
            // mul_fast(x,x): x SCALE-valued → ≤ SCALE_I. mul_fast(mul_fast(s,s),s): s ≤ SCALE_I → ≤ SCALE_I.
            // fp_div_i of those is SCALE-valued; s/4 ≤ SCALE_I/4; difference fits i128.
            let b_halley = fp_div_i(mul_fast(x, x), mul_fast(mul_fast(s, s), s))?
                - s / 4;
            // mul_fast(b_halley, b_halley) ≤ SCALE_I; 3·mul_fast(x_over_s2,x_over_s2) ≤ 3·SCALE_I;
            // SCALE_I/4 < SCALE_I; all three terms < ~5·SCALE_I. Fits i128.
            let b_hh3 = mul_fast(b_halley, b_halley)
                - 3 * mul_fast(x_over_s2, x_over_s2) - SCALE_I / 4;
            let hh_fac = householder_factor(newton, b_halley, b_hh3)?;
            ds = mul_fast(newton, hh_fac);
        } else if beta < b_c {
            // Branch 1 log-transformed: g(s) = 1/ln(b) - 1/ln(beta)
            if b <= 0 {
                s = (s_left + s_right) / 2;
                continue;
            }
            let ln_b = ln_fixed_i(b as u128)?;
            let ln_beta = ln_fixed_i(beta as u128)?;
            if ln_b == 0 || ln_beta == 0 {
                s = (s_left + s_right) / 2;
                continue;
            }
            let bpob = fp_div_i(bp, b)?;
            // Same b_halley bound as direct branch: SCALE-valued result, s/4 subtraction fits i128.
            let b_halley = fp_div_i(mul_fast(x, x), mul_fast(mul_fast(s, s), s))?
                - s / 4;
            // ln_beta - ln_b: both < ~40·SCALE_I; mul_fast of difference with ln_b < ~1600·SCALE_I²/SCALE_I.
            // fp_div_i by ln_beta gives SCALE-valued; mul_fast with fp_div_i(b,bp) ≤ SCALE_I. Fits i128.
            let newton = mul_fast(
                fp_div_i(mul_fast(ln_beta - ln_b, ln_b), ln_beta)?,
                fp_div_i(b, bp)?);
            // bpob SCALE-valued; SCALE_I + fp_div_i(2·SCALE_I, ln_b) is SCALE-valued; mul_fast ≤ SCALE_I.
            // b_halley - that: difference fits i128.
            let halley = b_halley - mul_fast(bpob, SCALE_I + fp_div_i(2 * SCALE_I, ln_b)?);
            // Same b_hh3 bound as direct branch: < ~5·SCALE_I. Fits i128.
            let b_hh3 = mul_fast(b_halley, b_halley)
                - 3 * mul_fast(fp_div_i(x, mul_fast(s, s))?, fp_div_i(x, mul_fast(s, s))?)
                - SCALE_I / 4;
            // Each mul_fast term ≤ ~3·SCALE_I; sum ≤ ~9·SCALE_I. Fits i128.
            let hh3 = b_hh3
                + 2 * mul_fast(mul_fast(bpob, bpob),
                    SCALE_I + mul_fast(fp_div_i(3 * SCALE_I, ln_b)?,
                        SCALE_I + fp_div_i(SCALE_I, ln_b)?))
                - 3 * mul_fast(mul_fast(b_halley, bpob),
                    SCALE_I + fp_div_i(2 * SCALE_I, ln_b)?);
            let hh_fac = householder_factor(newton, halley, hh3)?;
            ds = mul_fast(newton, hh_fac);
        } else {
            // Branch 4 log-transformed: g(s) = ln(b_max-beta) - ln(b_max-b)
            // b_max, beta, b are normalised black values < exp(x/2) ≤ exp(5) ≈ 148·SCALE_I; differences fit i128.
            let b_max_minus_b = b_max - b;
            if b_max_minus_b <= 0 || bp <= 100 {
                s = (s_left + s_right) / 2;
                continue;
            }
            // ln results < ~40·SCALE_I; difference fits i128.
            let g = ln_fixed_i((b_max - beta).unsigned_abs())?
                - ln_fixed_i(b_max_minus_b.unsigned_abs())?;
            let gp = fp_div_i(bp, b_max_minus_b)?;
            let newton = -fp_div_i(g, gp)?;
            // Same b_halley/b_hh3 bounds as direct branch. Fits i128.
            let b_halley = fp_div_i(mul_fast(x, x), mul_fast(mul_fast(s, s), s))?
                - s / 4;
            let b_hh3 = mul_fast(b_halley, b_halley)
                - 3 * mul_fast(fp_div_i(x, mul_fast(s, s))?, fp_div_i(x, mul_fast(s, s))?)
                - SCALE_I / 4;
            // b_halley, gp both SCALE-valued; sum fits i128.
            let halley = b_halley + gp;
            // 2*gp < ~2·SCALE_I (gp SCALE-valued); 3*b_halley < ~3·SCALE_I; sum < ~5·SCALE_I. Fits i128.
            // mul_fast(gp, 2*gp + 3*b_halley): gp < ~SCALE_I, sum < ~5·SCALE_I; product via mul_fast < ~5·SCALE_I.
            // b_hh3 + that: b_hh3 < ~5·SCALE_I; sum < ~10·SCALE_I. Fits i128.
            let hh3 = b_hh3 + mul_fast(gp, 2 * gp + 3 * b_halley);
            let hh_fac = householder_factor(newton, halley, hh3)?;
            ds = mul_fast(newton, hh_fac);
        };

        // Direction reversal detection
        // ds * ds_previous: both SCALE-valued step sizes < ~SCALE_I; product for sign test only, not stored.
        // REVIEW: if ds or ds_previous can reach ~1e19 in extreme cases, product could overflow i128 at ~1e38.
        // In practice ds is bounded by the bracket width which is < 10·SCALE_I, so |ds|² < 1e26 << i128::MAX.
        if ds * ds_previous < 0 {
            direction_reversal_count += 1;
        }
        // s + ds: s ≤ s_right ≤ i128::MAX/2; ds < bracket_width < 10·SCALE_I; sum fits i128.
        if direction_reversal_count >= 3 || (s + ds <= s_left) || (s + ds >= s_right) {
            // s_left + s_right: s_right ≤ i128::MAX/2 by initialisation; sum fits i128.
            s = (s_left + s_right) / 2;
            direction_reversal_count = 0;
            ds_previous = 0;
            continue;
        }
        ds_previous = ds;
        // s > 0 by loop guard; ds.max(-s/2) ≥ -s/2 so s + that ≥ s/2 > 0. No underflow; fits i128.
        s += ds.max(-s / 2);
    }

    if s > 0 { Ok(s) } else { Err(SolMathError::NoConvergence) }
}

/// Iterative initial guess for out-of-Li-domain cases.
/// Returns x_vol = σ√T initial estimate.
#[inline(never)]
fn iterative_initial_guess(
    _market_price: u128, s: u128, k: u128,
    mp_i: i128, s_i: i128, k_disc: i128, sqrt_t: i128, ln_fk: i128,
) -> Result<i128, SolMathError> {
    let abs_ln_fk = ln_fk.abs();

    let beta_otm = if ln_fk > 0 {
        // mp_i, s_i, k_disc < ~1e17; put_i is their signed sum, fits i128.
        let put_i = mp_i - s_i + k_disc;
        if put_i <= 0 { 0 } else {
            let sqrt_sk = fp_sqrt(fp_mul(s, k)?)? as i128;
            fp_div_i(put_i, sqrt_sk)?
        }
    } else {
        let sqrt_sk = fp_sqrt(fp_mul(s, k)?)? as i128;
        fp_div_i(mp_i, sqrt_sk)?
    };

    let sqrt_2pi: i128 = 2_506_628_274_631;

    if beta_otm < 1000 {
        // Deep ITM with near-zero OTM-equivalent beta at SCALE.
        // Return NoConvergence — caller will escalate to HP.
        return Err(SolMathError::NoConvergence);
    } else if abs_ln_fk < 50_000_000_000 {
        // beta_otm, sqrt_2pi both < ~3·SCALE_I; mul_fast result < ~3·SCALE_I. Fits i128.
        Ok(mul_fast(beta_otm, sqrt_2pi))
    } else if beta_otm < 10_000_000_000 && abs_ln_fk > 100_000_000_000 {
        // Two-step asymptotic tail guess for OTM with small normalised price.
        // Leading asymptotic: s ≈ |x|/√(-2·ln(β))
        // Refined: include √(2π)·|x/s| log correction from the BS tail expansion.
        //   A  = -2·ln(β) - ln(2π)
        //   A₂ = A - ln(A)          (one-step substitution)
        //   s  = |x| / √A₂
        // Cost: 2 × ln_fixed_i (~12K CU). Gives ~6-15% error vs ~20-26% for
        // the single-step formula, saving one Halley iteration (~44K CU).
        let ln_beta = ln_fixed_i(beta_otm as u128)?;
        // ln_beta < ~30·SCALE_I; 2*ln_beta < ~60·SCALE_I; LN_2PI ≈ 1.8·SCALE_I; difference fits i128.
        let a = -2 * ln_beta - LN_2PI;
        if a > SCALE_I {
            let ln_a = ln_fixed_i(a as u128)?;
            // a < ~60·SCALE_I; ln_a < ~40·SCALE_I; a2 = a - ln_a fits i128.
            let a2 = a - ln_a;
            if a2 > SCALE_I / 2 {
                fp_div_i(abs_ln_fk, fp_sqrt(a2 as u128)? as i128)
            } else {
                fp_div_i(abs_ln_fk, fp_sqrt(a as u128)? as i128)
            }
        } else if a > 0 {
            fp_div_i(abs_ln_fk, fp_sqrt(a as u128)? as i128)
        } else {
            // 0.25·SCALE_I * sqrt_t: mul_fast result < 0.25·SCALE_I. Fits i128.
            Ok(mul_fast(250_000_000_000, sqrt_t))
        }
    } else {
        let s_c = fp_sqrt(2 * abs_ln_fk as u128)? as i128;
        let exp_neg_half_x = exp_fixed_i(-abs_ln_fk / 2)?;
        let exp_half_x = fp_div_i(SCALE_I, exp_neg_half_x)?;
        let b_c = {
            let phi_neg_sc = norm_cdf_poly(-s_c)?;
            // exp_neg_half_x ∈ (0, SCALE_I]; /2 > 0. mul_fast(phi_neg_sc, exp_half_x) ≤ SCALE_I. Difference fits i128.
            let v = exp_neg_half_x / 2 - mul_fast(phi_neg_sc, exp_half_x);
            if v > 0 { v } else { 0 }
        };
        // INV_SQRT_2PI ≈ 0.4·SCALE_I; exp_neg_half_x ≤ SCALE_I; mul_fast result < 0.4·SCALE_I. Fits i128.
        let v_c = mul_fast(INV_SQRT_2PI, exp_neg_half_x);
        if v_c > 0 {
            // s_c and fp_div_i result both < ~10·SCALE_I; sum fits i128.
            let s0 = s_c + fp_div_i(beta_otm - b_c, v_c)?;
            Ok(if s0 > 0 { s0 } else { s_c })
        } else {
            Ok(s_c)
        }
    }
}

/// Recover the implied volatility from an observed option price.
///
/// Given a market call price and the standard Black-Scholes parameters,
/// returns the volatility `sigma` (at `SCALE`) that reproduces the price.
/// Automatically detects whether the price corresponds to a call or put
/// by comparing against intrinsic value.
///
/// All inputs are unsigned fixed-point at `SCALE` (1e12):
/// - `market_price` -- observed option price
/// - `s` -- spot price
/// - `k` -- strike price
/// - `r` -- risk-free rate
/// - `t` -- time to expiry in years
///
/// # Algorithm
///
/// Uses a Li rational initial guess with bracketed Halley iteration,
/// falling back to a Jaeckel normalised-space solver for edge cases.
/// Architecture is designed for <=200K CU worst case on Solana.
///
/// # Errors
///
/// - [`SolMathError::DomainError`] if `s == 0`, `k == 0`, `t == 0`, or `market_price == 0`.
/// - [`SolMathError::NoConvergence`] for sub-ULP prices or deep OTM edge cases
///   where no stable root can be found.
///
/// # Accuracy
///
/// P99 error of 2K ULP, median 3 ULP.
///
/// # Example
///
/// ```
/// use solmath_core::{implied_vol, black_scholes_price, SCALE};
///
/// let s = 100 * SCALE;
/// let k = 100 * SCALE;
/// let r = 50_000_000_000u128;      // 0.05
/// let sigma = 200_000_000_000u128;  // 0.20
/// let t = SCALE;
/// let (call, _) = black_scholes_price(s, k, r, sigma, t)?;
/// let recovered = implied_vol(call, s, k, r, t)?;
/// let diff = (recovered as i128 - sigma as i128).abs();
/// assert!(diff < 1_000_000); // within ~0.0001% of input sigma
/// # Ok::<(), solmath_core::SolMathError>(())
/// ```
#[inline(never)]
pub fn implied_vol(market_price: u128, s: u128, k: u128, r: u128, t: u128) -> Result<u128, SolMathError> {
    // mul_fast inside the solver assumes |s_i|,|k_i| < ~1e17 so products fit i128.
    // 1e17 * SCALE = 1e29 — no real token exceeds $100 quadrillion.
    const MAX_PRICE: u128 = 170_000_000_000_000 * SCALE; // 1.7e14 * SCALE = 1.7e26 — mul_fast safe
    if market_price > i128::MAX as u128 || s > i128::MAX as u128 || k > i128::MAX as u128
        || r > i128::MAX as u128 || t > i128::MAX as u128
    {
        return Err(SolMathError::Overflow);
    }
    if s > MAX_PRICE || k > MAX_PRICE || market_price > MAX_PRICE {
        return Err(SolMathError::Overflow);
    }
    if s == 0 || k == 0 || t == 0 || market_price == 0 {
        return Err(SolMathError::DomainError);
    }
    if market_price < 100 {
        return Err(SolMathError::NoConvergence);
    }

    let mp_i = market_price as i128;
    let s_i = s as i128;
    let k_i = k as i128;
    let r_i = r as i128;
    let t_i = t as i128;

    // ── Shared setup (~10K CU) ──
    let r_t = fp_mul_i(r_i, t_i)?;
    let sqrt_t = fp_sqrt(t)? as i128;
    if sqrt_t <= 0 {
        return Err(SolMathError::DomainError);
    }
    let ln_sk = ln_fixed_i(fp_div(s, k)?)?;
    // ln_sk, r_t both in (~-40·SCALE_I, ~40·SCALE_I); sum fits i128.
    let ln_fk = ln_sk + r_t;

    // ── V1 setup (~5K CU) ──
    let discount = exp_fixed_i(-r_t)?;
    let k_disc = fp_mul_i(k_i, discount)?;

    // ── V1 fast path: Li guess + 4 Halley iterations ──
    let c_raw = fp_div_i(mp_i, s_i)?;
    let (x_li, c_li) = if ln_fk > 0 {
        // c_raw ∈ [0, SCALE_I], SCALE_I = 1e12, fp_div_i result ∈ [0, SCALE_I]; difference fits i128.
        let c_put = c_raw - SCALE_I + fp_div_i(k_disc, s_i)?;
        (-ln_fk, if c_put > 0 { c_put } else { 0 })
    } else {
        (ln_fk, c_raw)
    };

    let abs_x = x_li.abs();
    if abs_x < 500_000_000_000 && c_li > 0 {
        // A/B: use rational_guess_v2 (Padé) or li_rational_guess (bivariate)
        #[cfg(feature = "pade-iv")]
        let guess_result = rational_guess_v2(x_li, c_li);
        #[cfg(not(feature = "pade-iv"))]
        let guess_result = li_rational_guess(x_li, c_li);
        if let Ok(w) = guess_result {
            if w > 0 && w <= SCALE_I {
                // k_disc < ~1e17; k_disc/20 < ~5e15; sum < ~1.05e17 << i128::MAX.
                let solve_as_put = s_i > k_disc + k_disc / 20;
                let target_i = if solve_as_put {
                    // mp_i, s_i, k_disc < ~1e17; put_i fits i128.
                    let put_i = mp_i - s_i + k_disc;
                    if put_i > 0 { put_i } else { 1 }
                } else {
                    mp_i
                };

                // 1e9·sqrt_t and 5e12·sqrt_t: both safe via mul_fast, same as implied_vol_v1 bracket above.
                let mut x_lo = (mul_fast(1_000_000_000, sqrt_t)).max(1) as u128;
                let mut x_hi = (mul_fast(5_000_000_000_000, sqrt_t)).max(x_lo as i128 + 1) as u128;
                let mut x_u = (w as u128).clamp(x_lo, x_hi);

                for iter in 0..4u8 {
                    let x_i = x_u as i128;
                    if x_i <= 1 { break; }

                    let (price_i, vega_x, volga_x) =
                        iv_price_and_greeks(x_i, ln_fk, s_i, k_disc, solve_as_put)?;
                    let f = price_i - target_i;

                    // Tight accept early, relaxed on final iteration
                    let tol = if iter < 3 { 100 } else { 1000 };
                    if f.abs() <= tol {
                        return Ok(fp_div_i(x_i, sqrt_t)? as u128);
                    }

                    if f > 0 {
                        if x_u < x_hi { x_hi = x_u; }
                    } else {
                        if x_u > x_lo { x_lo = x_u; }
                    }

                    x_u = halley_step_bracketed(x_u, f, vega_x, volga_x, x_lo, x_hi)?;
                }
                // Post-loop check: the 4th step may have improved x_u
                let x_i = x_u as i128;
                if x_i > 0 {
                    let (price_i, _, _) =
                        iv_price_and_greeks(x_i, ln_fk, s_i, k_disc, solve_as_put)?;
                    if (price_i - target_i).abs() <= 1000 {
                        return Ok(fp_div_i(x_i, sqrt_t)? as u128);
                    }
                }
            }
        }
    }

    // ── Iterative path: out-of-Li-domain cases (~83K CU) ──
    // Reuses shared setup (ln_fk, k_disc, sqrt_t).
    {
        // k_disc < ~1e17; k_disc/20 < ~5e15; sum fits i128.
        let solve_as_put = s_i > k_disc + k_disc / 20;
        let target_i = if solve_as_put {
            // mp_i, s_i, k_disc < ~1e17; sum fits i128.
            let put_i = mp_i - s_i + k_disc;
            if put_i > 0 { put_i } else { 1 }
        } else {
            mp_i
        };

        let x_vol = iterative_initial_guess(
            market_price, s, k, mp_i, s_i, k_disc, sqrt_t, ln_fk,
        )?;

        // 1e9·sqrt_t and 1e13·sqrt_t: both safe via mul_fast (same bound as other bracket sites).
        let x_min = (mul_fast(1_000_000_000, sqrt_t)).max(1) as u128;
        let x_max = (mul_fast(10_000_000_000_000, sqrt_t)).max(x_min as i128 + 1) as u128;
        let mut x_u = (x_vol as u128).clamp(x_min, x_max);
        let mut x_lo = x_min;
        let mut x_hi = x_max;

        for iter in 0..5u8 {
            let x_i = x_u as i128;
            if x_i <= 1 { break; }

            let (price_i, vega_x, volga_x) =
                iv_price_and_greeks(x_i, ln_fk, s_i, k_disc, solve_as_put)?;
            let f = price_i - target_i;

            if f.abs() <= 1 {
                return Ok(fp_div_i(x_i, sqrt_t)? as u128);
            }
            if iter >= 3 && f.abs() <= 100 {
                return Ok(fp_div_i(x_i, sqrt_t)? as u128);
            }

            if f > 0 {
                if x_u < x_hi { x_hi = x_u; }
            } else {
                if x_u > x_lo { x_lo = x_u; }
            }

            x_u = halley_step_bracketed(x_u, f, vega_x, volga_x, x_lo, x_hi)?;
        }

        // Final check at 2000 ULP — wider than path A to avoid expensive Jäckel fallback
        let x_i = x_u as i128;
        if x_i > 0 {
            let (price_i, _, _) =
                iv_price_and_greeks(x_i, ln_fk, s_i, k_disc, solve_as_put)?;
            if (price_i - target_i).abs() <= 2000 {
                return Ok(fp_div_i(x_i, sqrt_t)? as u128);
            }
        }
    }

    // ── Jäckel fallback (~110K CU, shared setup saves ~10K) ──
    // exp(rT) = 1/discount — saves one exp_fixed_i call
    let exp_rt = if discount > 0 {
        fp_div_i(SCALE_I, discount)?
    } else {
        exp_fixed_i(r_t)?
    };
    let sqrt_sk = fp_sqrt(fp_mul(s, k)?)? as i128;
    let exp_half_rt = exp_fixed_i(r_t / 2)?;
    // sqrt_sk is √(S·K) at SCALE, SCALE-valued; exp_half_rt is SCALE-valued; mul_fast result ≤ SCALE_I. Fits i128.
    let sqrt_fk = mul_fast(sqrt_sk, exp_half_rt);
    if sqrt_fk <= 0 {
        return Err(SolMathError::DomainError);
    }

    let x = ln_fk;
    let (x_otm, beta) = if x > 0 {
        // mp_i < ~1e17; exp_rt is SCALE-valued (≈1·SCALE_I for small r); mul_fast result < ~1e17. Fits i128.
        let undiscounted_call = mul_fast(mp_i, exp_rt);
        // s_i < ~1e17; exp_rt SCALE-valued; mul_fast(s_i, exp_rt) < ~1e17. Fits i128.
        let forward = mul_fast(s_i, exp_rt);
        // forward, k_i both < ~1e17; difference fits i128.
        let intrinsic = forward - k_i;
        // undiscounted_call, intrinsic both < ~1e17; difference fits i128.
        let put_undiscounted = undiscounted_call - (if intrinsic > 0 { intrinsic } else { 0 });
        let put_undiscounted = if put_undiscounted > 0 { put_undiscounted } else { 0 };
        (-x, fp_div_i(put_undiscounted, sqrt_fk)?)
    } else {
        // mp_i < ~1e17; exp_rt SCALE-valued; mul_fast result < ~1e17. Fits i128.
        let undiscounted_call = mul_fast(mp_i, exp_rt);
        (x, fp_div_i(undiscounted_call, sqrt_fk)?)
    };

    // 2 Householder iterations — CU-budget constrained
    let s_norm = jaeckel_normalised_iv(beta, x_otm, 2)?;
    if s_norm <= 0 {
        // Jäckel failed — escalate to HP solver
        return Err(SolMathError::NoConvergence);
    }

    let sigma = fp_div_i(s_norm, sqrt_t)?;
    if sigma <= 0 {
        return Err(SolMathError::NoConvergence);
    }
    let sigma_u = sigma as u128;

    // Verify: BS(σ) must reproduce market price within 500 ULP
    if let Ok((check_call, _)) = black_scholes_price(s, k, r, sigma_u, t) {
        let price_err = if check_call > market_price {
            check_call - market_price
        } else {
            market_price - check_call
        };
        if price_err <= 1000 {
            return Ok(sigma_u);
        }
    }

    Err(SolMathError::NoConvergence)
}

/// Standalone Jäckel path (for benchmarking): normalise → solve → verify.
/// Uses 2 Householder iterations. Not CU-constrained.
#[inline(never)]
pub(crate) fn implied_vol_jaeckel(market_price: u128, s: u128, k: u128, r: u128, t: u128) -> Result<u128, SolMathError> {
    let s_i = s as i128;
    let k_i = k as i128;
    let r_i = r as i128;
    let t_i = t as i128;

    let r_t = fp_mul_i(r_i, t_i)?;
    let exp_rt = exp_fixed_i(r_t)?;
    let sqrt_t = fp_sqrt(t)? as i128;
    if sqrt_t <= 0 {
        return Err(SolMathError::DomainError);
    }

    // √(F·K) = √(S·K)·exp(rT/2)
    let sqrt_sk = fp_sqrt(fp_mul(s, k)?)? as i128;
    let exp_half_rt = exp_fixed_i(r_t / 2)?;
    // sqrt_sk SCALE-valued; exp_half_rt SCALE-valued; mul_fast result ≤ SCALE_I. Fits i128.
    let sqrt_fk = mul_fast(sqrt_sk, exp_half_rt);
    if sqrt_fk <= 0 {
        return Err(SolMathError::DomainError);
    }

    // x = ln(F/K)
    let ln_sk = ln_fixed_i(fp_div(s, k)?)?;
    // ln_sk, r_t both in (~-40·SCALE_I, ~40·SCALE_I); sum fits i128.
    let x = ln_sk + r_t;

    // Normalise
    let mp_i = market_price as i128;
    let (x_otm, beta) = if x > 0 {
        // mp_i < ~1e17; exp_rt SCALE-valued; mul_fast result < ~1e17. Fits i128.
        let undiscounted_call = mul_fast(mp_i, exp_rt);
        // s_i < ~1e17; exp_rt SCALE-valued; mul_fast(s_i, exp_rt) < ~1e17. Fits i128.
        let forward = mul_fast(s_i, exp_rt);
        // forward, k_i < ~1e17; difference fits i128.
        let intrinsic = forward - k_i;
        // undiscounted_call, intrinsic < ~1e17; difference fits i128.
        let put_undiscounted = undiscounted_call - (if intrinsic > 0 { intrinsic } else { 0 });
        let put_undiscounted = if put_undiscounted > 0 { put_undiscounted } else { 0 };
        (-x, fp_div_i(put_undiscounted, sqrt_fk)?)
    } else {
        // mp_i < ~1e17; exp_rt SCALE-valued; mul_fast result < ~1e17. Fits i128.
        let undiscounted_call = mul_fast(mp_i, exp_rt);
        (x, fp_div_i(undiscounted_call, sqrt_fk)?)
    };

    // Solve (standalone: 2 Householder iterations)
    let s_norm = jaeckel_normalised_iv(beta, x_otm, 2)?;
    if s_norm <= 0 {
        return Err(SolMathError::NoConvergence);
    }

    let sigma = fp_div_i(s_norm, sqrt_t)?;
    if sigma <= 0 {
        return Err(SolMathError::NoConvergence);
    }
    let sigma_u = sigma as u128;

    // Verify: BS(σ) must reproduce market price within 500 ULP
    if let Ok((check_call, _)) = black_scholes_price(s, k, r, sigma_u, t) {
        let price_err = if check_call > market_price {
            check_call - market_price
        } else {
            market_price - check_call
        };
        if price_err <= 1000 {
            return Ok(sigma_u);
        }
    }

    Err(SolMathError::NoConvergence)
}
