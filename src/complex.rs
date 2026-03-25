use crate::error::SolMathError;
use crate::arithmetic::{fp_mul_i, fp_div_i, fp_sqrt};
use crate::transcendental::exp_fixed_i;
use crate::trig::{cos_fixed, sin_fixed};

// ============================================================
// Complex arithmetic
// ============================================================

/// Complex number with real and imaginary parts at SCALE (1e12).
///
/// Used internally by Heston and NIG pricing for characteristic function evaluation.
/// Both `re` and `im` are signed fixed-point at SCALE.
#[derive(Clone, Copy)]
pub struct Complex {
    pub re: i128,
    pub im: i128,
}

impl Complex {
    /// Construct a complex number. No computation.
    pub fn new(re: i128, im: i128) -> Self {
        Self { re, im }
    }
}

/// Complex multiplication at SCALE.
/// Error: ~2–4 ULP. Returns Err(Overflow) on arithmetic overflow.
pub fn complex_mul(a: Complex, b: Complex) -> Result<Complex, SolMathError> {
    Ok(Complex::new(
        fp_mul_i(a.re, b.re)?.checked_sub(fp_mul_i(a.im, b.im)?).ok_or(SolMathError::Overflow)?,
        fp_mul_i(a.re, b.im)?.checked_add(fp_mul_i(a.im, b.re)?).ok_or(SolMathError::Overflow)?,
    ))
}

/// Complex division at SCALE.
/// Error: ~2–4 ULP. Returns Err(DivisionByZero) if b == 0+0i, Err(Overflow) if |b|² overflows.
pub fn complex_div(a: Complex, b: Complex) -> Result<Complex, SolMathError> {
    let b_re_sq = fp_mul_i(b.re, b.re)?;
    let b_im_sq = fp_mul_i(b.im, b.im)?;
    let denom = b_re_sq.checked_add(b_im_sq).ok_or(SolMathError::Overflow)?;
    if denom == 0 {
        return Err(SolMathError::DivisionByZero);
    }
    Ok(Complex::new(
        fp_div_i(fp_mul_i(a.re, b.re)?.checked_add(fp_mul_i(a.im, b.im)?).ok_or(SolMathError::Overflow)?, denom)?,
        fp_div_i(fp_mul_i(a.im, b.re)?.checked_sub(fp_mul_i(a.re, b.im)?).ok_or(SolMathError::Overflow)?, denom)?,
    ))
}

/// Complex exponential: exp(a + bi) = exp(a) × (cos(b) + i·sin(b)).
/// Error: ~2–4 ULP. Returns Err(Overflow) if exp(z.re) overflows.
pub fn complex_exp(z: Complex) -> Result<Complex, SolMathError> {
    let e = exp_fixed_i(z.re)?;
    Ok(Complex::new(fp_mul_i(e, cos_fixed(z.im)?)?, fp_mul_i(e, sin_fixed(z.im)?)?))
}

/// Principal complex square root (re ≥ 0 branch).
/// Error: ~2–4 ULP. Returns Err(Overflow) if |z|² overflows, Err from internal division in degenerate cases.
pub fn complex_sqrt(z: Complex) -> Result<Complex, SolMathError> {
    let a_sq = fp_mul_i(z.re, z.re)?;
    let b_sq = fp_mul_i(z.im, z.im)?;
    let norm_sq = a_sq.checked_add(b_sq).ok_or(SolMathError::Overflow)?;
    if norm_sq == 0 {
        return Ok(Complex::new(0, 0));
    }
    let modz = fp_sqrt(norm_sq as u128)? as i128;
    // modz = |z| ≥ |re| ≥ z.re by definition of modulus; sum ≤ 2·modz ≤ ~20·SCALE_I, fits i128; /2 is safe
    let re_arg = (modz + z.re) / 2;
    let re = if re_arg > 0 {
        fp_sqrt(re_arg as u128)? as i128
    } else {
        0
    };
    if re == 0 {
        // Pure imaginary: sqrt(bi) = sqrt(|b|/2)(1 + i·sign(b))
        // modz ≥ |z.re| by definition of modulus, so modz - z.re ≥ 0; no underflow; /2 is safe
        let im_arg = (modz - z.re) / 2;
        let im = fp_sqrt(im_arg as u128)? as i128;
        return Ok(Complex::new(0, if z.im < 0 { -im } else { im }));
    }
    // re ∈ [0, ~sqrt(10)·SCALE_I] (since |z| ≤ ~10·SCALE_I); 2*re ≤ ~2*sqrt(10)*SCALE_I ≈ 6.3e12, fits i128
    let im = fp_div_i(z.im, 2 * re)?;
    Ok(Complex::new(re, im))
}
