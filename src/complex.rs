use crate::arithmetic::{fp_div, fp_div_i, fp_mul, fp_mul_i, fp_mul_round, fp_sqrt};
use crate::constants::{INV_SQRT2, SCALE, U256};
use crate::error::SolMathError;
use crate::overflow::checked_mul_div_i;
use crate::transcendental::exp_fixed_i;
use crate::trig::{cos_fixed, sin_fixed};

// ============================================================
// Complex arithmetic
// ============================================================

/// Complex number with real and imaginary parts at SCALE (1e12).
///
/// Both `re` and `im` are signed fixed-point at SCALE.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
        scaled_product_sum(a.re, b.re, a.im, b.im, true)?,
        scaled_product_sum(a.re, b.im, a.im, b.re, false)?,
    ))
}

/// Evaluate `(a*b ± c*d) / SCALE` with one final truncation.  Combining the
/// full 256-bit products before division avoids false overflow when large
/// terms cancel to a representable complex component.
fn scaled_product_sum(
    a: i128,
    b: i128,
    c: i128,
    d: i128,
    subtract_second: bool,
) -> Result<i128, SolMathError> {
    let (negative, quotient) = scaled_product_sum_wide(a, b, c, d, subtract_second)?;
    if negative {
        if quotient == 1u128 << 127 {
            Ok(i128::MIN)
        } else if quotient < 1u128 << 127 {
            Ok(-(quotient as i128))
        } else {
            Err(SolMathError::Overflow)
        }
    } else if quotient <= i128::MAX as u128 {
        Ok(quotient as i128)
    } else {
        Err(SolMathError::Overflow)
    }
}

fn scaled_product_sum_wide(
    a: i128,
    b: i128,
    c: i128,
    d: i128,
    subtract_second: bool,
) -> Result<(bool, u128), SolMathError> {
    let (negative, magnitude) = product_sum_raw(a, b, c, d, subtract_second)?;
    let (quotient, _) = magnitude.div_rem_u64(SCALE as u64);
    if quotient.high_u128_nonzero() {
        return Err(SolMathError::Overflow);
    }
    Ok((negative, quotient.low_u128()))
}

fn product_sum_raw(
    a: i128,
    b: i128,
    c: i128,
    d: i128,
    subtract_second: bool,
) -> Result<(bool, U256), SolMathError> {
    let first = U256::mul_u128(a.unsigned_abs(), b.unsigned_abs());
    let second = U256::mul_u128(c.unsigned_abs(), d.unsigned_abs());
    let first_negative = (a < 0) ^ (b < 0);
    let second_negative = (c < 0) ^ (d < 0) ^ subtract_second;

    let (negative, magnitude) = if first_negative == second_negative {
        (first_negative, add_u256(first, second)?)
    } else {
        match first.cmp_words(&second) {
            core::cmp::Ordering::Greater | core::cmp::Ordering::Equal => {
                let mut difference = first;
                let underflow = difference.overflowing_sub_assign(&second);
                debug_assert!(!underflow);
                (first_negative, difference)
            }
            core::cmp::Ordering::Less => {
                let mut difference = second;
                let underflow = difference.overflowing_sub_assign(&first);
                debug_assert!(!underflow);
                (second_negative, difference)
            }
        }
    };

    Ok((negative, magnitude))
}

fn add_u256(lhs: U256, rhs: U256) -> Result<U256, SolMathError> {
    let mut limbs = [0u64; 4];
    let mut carry = 0u128;
    for (idx, out) in limbs.iter_mut().enumerate() {
        let sum = lhs.limbs[idx] as u128 + rhs.limbs[idx] as u128 + carry;
        *out = sum as u64;
        carry = sum >> 64;
    }
    if carry != 0 {
        Err(SolMathError::Overflow)
    } else {
        Ok(U256 { limbs })
    }
}

fn mul_u256_u64(value: U256, factor: u64) -> Result<U256, SolMathError> {
    let mut limbs = [0u64; 4];
    let mut carry = 0u128;
    for (idx, out) in limbs.iter_mut().enumerate() {
        let product = value.limbs[idx] as u128 * factor as u128 + carry;
        *out = product as u64;
        carry = product >> 64;
    }
    if carry != 0 {
        Err(SolMathError::Overflow)
    } else {
        Ok(U256 { limbs })
    }
}

fn signed_magnitude_to_i128(negative: bool, magnitude: u128) -> Result<i128, SolMathError> {
    if negative {
        if magnitude == 1u128 << 127 {
            Ok(i128::MIN)
        } else if magnitude < 1u128 << 127 {
            Ok(-(magnitude as i128))
        } else {
            Err(SolMathError::Overflow)
        }
    } else if magnitude <= i128::MAX as u128 {
        Ok(magnitude as i128)
    } else {
        Err(SolMathError::Overflow)
    }
}

/// Complex division at SCALE.
///
/// Uses a scaled Smith algorithm and verifies the result by multiplying it
/// back by the divisor. Ill-conditioned cases that cannot meet the backward
/// error bound fail closed with `NoConvergence`.
pub fn complex_div(a: Complex, b: Complex) -> Result<Complex, SolMathError> {
    if b.re == 0 && b.im == 0 {
        return Err(SolMathError::DivisionByZero);
    }
    if a == b {
        return Ok(Complex::new(SCALE as i128, 0));
    }
    if b.im == 0 {
        return verify_division(
            a,
            b,
            Complex::new(fp_div_i(a.re, b.re)?, fp_div_i(a.im, b.re)?),
        );
    }
    if b.re == 0 {
        let im = fp_div_i(a.re, b.im)?
            .checked_neg()
            .ok_or(SolMathError::Overflow)?;
        return verify_division(a, b, Complex::new(fp_div_i(a.im, b.im)?, im));
    }

    if let Some(exact) = exact_complex_div(a, b) {
        return exact;
    }

    // Each Smith numerator is the sum of two terms whose magnitudes are at
    // most the corresponding components of `a`.  Halving a very large
    // numerator first therefore prevents a representable final quotient from
    // failing merely because that intermediate sum exceeds i128.  Recovering
    // the factor of two costs at most one quotient ULP; the backward check
    // below rejects the result if that loss matters for this divisor.
    const SAFE_SUM_COMPONENT: u128 = i128::MAX as u128 / 2;
    if a.re.unsigned_abs().max(a.im.unsigned_abs()) > SAFE_SUM_COMPONENT {
        let half = Complex::new(a.re / 2, a.im / 2);
        let half_q = complex_div(half, b)?;
        let q = Complex::new(
            half_q.re.checked_mul(2).ok_or(SolMathError::Overflow)?,
            half_q.im.checked_mul(2).ok_or(SolMathError::Overflow)?,
        );
        return verify_division(a, b, q);
    }

    // Smith's ratio algorithm avoids both squaring overflow and the
    // small-product truncation that made (1+i)/(1+i) collapse to zero.
    if b.re.unsigned_abs() >= b.im.unsigned_abs() {
        // Form a*(d/c) as the single exact quotient (a*d)/c. Computing d/c
        // first would truncate a sub-ULP ratio to zero before a large
        // numerator can magnify it.
        let den_term = checked_mul_div_i(b.im, b.im, b.re)?;
        let den = match b.re.checked_add(den_term) {
            Some(den) => den,
            None => return divide_with_halved_divisor(a, b),
        };
        let re_num =
            a.re.checked_add(checked_mul_div_i(a.im, b.im, b.re)?)
                .ok_or(SolMathError::Overflow)?;
        let im_num =
            a.im.checked_sub(checked_mul_div_i(a.re, b.im, b.re)?)
                .ok_or(SolMathError::Overflow)?;
        let mut re = fp_div_i(re_num, den)?;
        let mut im = fp_div_i(im_num, den)?;
        if den_term == 0 && b.im != 0 {
            re = correct_sub_ulp_denominator(re, b.im, b.re)?;
            im = correct_sub_ulp_denominator(im, b.im, b.re)?;
        }
        verify_division(a, b, Complex::new(re, im))
    } else {
        let den_term = checked_mul_div_i(b.re, b.re, b.im)?;
        let den = match b.im.checked_add(den_term) {
            Some(den) => den,
            None => return divide_with_halved_divisor(a, b),
        };
        let re_num = checked_mul_div_i(a.re, b.re, b.im)?
            .checked_add(a.im)
            .ok_or(SolMathError::Overflow)?;
        let im_num = checked_mul_div_i(a.im, b.re, b.im)?
            .checked_sub(a.re)
            .ok_or(SolMathError::Overflow)?;
        let mut re = fp_div_i(re_num, den)?;
        let mut im = fp_div_i(im_num, den)?;
        if den_term == 0 && b.re != 0 {
            re = correct_sub_ulp_denominator(re, b.re, b.im)?;
            im = correct_sub_ulp_denominator(im, b.re, b.im)?;
        }
        verify_division(a, b, Complex::new(re, im))
    }
}

fn exact_complex_div(a: Complex, b: Complex) -> Option<Result<Complex, SolMathError>> {
    // When |b|² fits u128, evaluate the textbook formula exactly with U256
    // cross-products. This covers all ordinary and sub-SCALE inputs, where
    // Smith's integer cross-ratios can otherwise discard several output ULP.
    let br = b.re.unsigned_abs();
    let bi = b.im.unsigned_abs();
    let denominator = br.checked_mul(br)?.checked_add(bi.checked_mul(bi)?)?;

    Some((|| {
        let quotient_component = |x1: i128, y1: i128, x2: i128, y2: i128, subtract: bool| {
            let (negative, numerator) = product_sum_raw(x1, y1, x2, y2, subtract)?;
            let scaled = mul_u256_u64(numerator, SCALE as u64)?;
            let quotient = if denominator <= u64::MAX as u128 {
                scaled.div_rem_u64(denominator as u64).0
            } else {
                scaled.div_rem_u128(denominator).0
            };
            if quotient.high_u128_nonzero() {
                return Err(SolMathError::Overflow);
            }
            signed_magnitude_to_i128(negative, quotient.low_u128())
        };

        Ok(Complex::new(
            quotient_component(a.re, b.re, a.im, b.im, false)?,
            quotient_component(a.im, b.re, a.re, b.im, true)?,
        ))
    })())
}

fn divide_with_halved_divisor(a: Complex, b: Complex) -> Result<Complex, SolMathError> {
    // Smith's denominator has the sign of its dominant component.  If its
    // two terms overflow when added, both divisor components are large enough
    // that halving cannot erase a non-zero component.  Dividing by b/2 gives
    // twice the desired quotient; truncate that final factor only after the
    // stable division, then validate against the original operands.
    let half_b = Complex::new(b.re / 2, b.im / 2);
    let double_q = complex_div(a, half_b)?;
    let q = Complex::new(double_q.re / 2, double_q.im / 2);
    verify_division(a, b, q)
}

fn verify_division(a: Complex, b: Complex, q: Complex) -> Result<Complex, SolMathError> {
    let divisor_l1 = b.re.unsigned_abs().saturating_add(b.im.unsigned_abs());
    // A quotient component may carry up to four raw ULP of documented error.
    // Multiplication by the divisor turns that into at most four times its L1
    // magnitude (in raw fixed units), plus the two product truncations.
    let tolerance = (divisor_l1 / SCALE).saturating_mul(5).saturating_add(5);
    let real_residual = product_sum_residual(q.re, b.re, q.im, b.im, true, a.re)?;
    let imag_residual = product_sum_residual(q.re, b.im, q.im, b.re, false, a.im)?;
    if real_residual > tolerance || imag_residual > tolerance {
        return Err(SolMathError::NoConvergence);
    }
    Ok(q)
}

fn product_sum_residual(
    a: i128,
    b: i128,
    c: i128,
    d: i128,
    subtract_second: bool,
    target: i128,
) -> Result<u128, SolMathError> {
    let (negative, magnitude) = scaled_product_sum_wide(a, b, c, d, subtract_second)?;
    let target_magnitude = target.unsigned_abs();
    if magnitude == 0 || target_magnitude == 0 || negative == (target < 0) {
        Ok(magnitude.abs_diff(target_magnitude))
    } else {
        Ok(magnitude.saturating_add(target_magnitude))
    }
}

fn sqrt_half_sum(a: u128, b: u128) -> Result<u128, SolMathError> {
    if (a ^ b) & 1 == 1 {
        // (a+b)/2 contains a half raw unit. Compute sqrt(a+b)/sqrt(2)
        // so that bit is not truncated before the square root.
        if let Some(sum) = a.checked_add(b) {
            fp_mul_round(fp_sqrt(sum)?, INV_SQRT2)
        } else {
            // At this magnitude, discarding the half raw input unit is far
            // below one output ULP. Halve first to keep the sum representable.
            fp_sqrt(a / 2 + b / 2)
        }
    } else {
        let half = a / 2 + b / 2 + (a % 2 + b % 2) / 2;
        fp_sqrt(half)
    }
}

fn correct_sub_ulp_denominator(
    quotient: i128,
    minor: i128,
    major: i128,
) -> Result<i128, SolMathError> {
    // 1/(1+r²) = 1-r²+O(r⁴). This path is used only when minor²/major
    // truncates to zero, so |r| < 1e-6 and the omitted r⁴ term is below one
    // raw output unit even at i128-scale quotients.
    let first = checked_mul_div_i(quotient, minor, major)?;
    let correction = checked_mul_div_i(first, minor, major)?;
    quotient
        .checked_sub(correction)
        .ok_or(SolMathError::Overflow)
}

/// Complex exponential: exp(a + bi) = exp(a) × (cos(b) + i·sin(b)).
/// Error: ~2–4 ULP. Returns Err(Overflow) if exp(z.re) overflows.
pub fn complex_exp(z: Complex) -> Result<Complex, SolMathError> {
    let e = exp_fixed_i(z.re)?;
    Ok(Complex::new(
        fp_mul_i(e, cos_fixed(z.im)?)?,
        fp_mul_i(e, sin_fixed(z.im)?)?,
    ))
}

/// Principal complex square root (re ≥ 0 branch).
/// Error: ~2–4 ULP. Returns Err(Overflow) if |z|² overflows, Err from internal division in degenerate cases.
pub fn complex_sqrt(z: Complex) -> Result<Complex, SolMathError> {
    let a = z.re.unsigned_abs();
    let b = z.im.unsigned_abs();
    let magnitude = a.max(b);
    if magnitude == 0 {
        return Ok(Complex::new(0, 0));
    }
    if magnitude < SCALE {
        // The normalized-hypot path cannot resolve both components when the
        // raw input itself is sub-SCALE.  Scale z by 4^n, take the root at a
        // well-resolved magnitude, then divide the root by 2^n.
        let mut scaled = z;
        let mut root_divisor = 1i128;
        while scaled.re.unsigned_abs().max(scaled.im.unsigned_abs()) < SCALE {
            scaled.re = scaled.re.checked_mul(4).ok_or(SolMathError::Overflow)?;
            scaled.im = scaled.im.checked_mul(4).ok_or(SolMathError::Overflow)?;
            root_divisor = root_divisor.checked_mul(2).ok_or(SolMathError::Overflow)?;
        }
        let scaled_root = complex_sqrt(scaled)?;
        return Ok(Complex::new(
            div_round_i(scaled_root.re, root_divisor)?,
            div_round_i(scaled_root.im, root_divisor)?,
        ));
    }

    // Scaled hypot: squaring tiny components at SCALE can truncate to zero,
    // while squaring large components can overflow. Dividing by the largest
    // component first avoids both failure modes.
    let ratio = fp_div(a.min(b), magnitude)?;
    let ratio_sq = fp_mul(ratio, ratio)?;
    let unit_hypot = fp_sqrt(SCALE.checked_add(ratio_sq).ok_or(SolMathError::Overflow)?)?;
    let modz_u = fp_mul(magnitude, unit_hypot)?;
    if z.re >= 0 {
        // modz ≥ |re|, both terms non-negative: no cancellation here.
        let re_u = z.re as u128;
        let re = sqrt_half_sum(modz_u, re_u)? as i128;
        if re == 0 {
            // Pure imaginary: sqrt(bi) = sqrt(|b|/2)(1 + i·sign(b))
            // modz ≥ |z.re| by definition of modulus, so modz - z.re ≥ 0; no underflow; /2 is safe
            let im = sqrt_half_sum(modz_u, z.re.unsigned_abs())? as i128;
            return refine_large_sqrt(
                z,
                Complex::new(0, if z.im < 0 { -im } else { im }),
                magnitude,
            );
        }
        // re ∈ [0, ~sqrt(10)·SCALE_I] (since |z| ≤ ~10·SCALE_I); 2*re ≤ ~2*sqrt(10)*SCALE_I ≈ 6.3e12, fits i128
        let im = fp_div_i(z.im, 2 * re)?;
        refine_large_sqrt(z, Complex::new(re, im), magnitude)
    } else {
        // z.re < 0: (modz + z.re)/2 cancels catastrophically and zeroes the
        // real part. Compute the imaginary part first from (modz − z.re)/2
        // (both terms positive), then recover re from 2·re·im = z.im.
        let abs_re = z.re.unsigned_abs();
        let im_mag_u = sqrt_half_sum(modz_u, abs_re)?;
        if im_mag_u > i128::MAX as u128 {
            return Err(SolMathError::Overflow);
        }
        let im_mag = im_mag_u as i128;
        if im_mag == 0 {
            return Ok(Complex::new(0, 0)); // |z| rounded to zero
        }
        // re = |z.im| / (2·im_mag) ≥ 0 keeps the principal branch; the sign
        // of im carries z.im's sign so 2·re·im == z.im. z.im != i128::MIN
        // here (its square above would have overflowed).
        let two_im = im_mag.checked_mul(2).ok_or(SolMathError::Overflow)?;
        let re = if z.im < 0 {
            fp_div_i(z.im, -two_im)?
        } else {
            fp_div_i(z.im, two_im)?
        };
        refine_large_sqrt(
            z,
            Complex::new(re, if z.im < 0 { -im_mag } else { im_mag }),
            magnitude,
        )
    }
}

fn div_round_i(value: i128, divisor: i128) -> Result<i128, SolMathError> {
    let half = divisor / 2;
    if value >= 0 {
        value
            .checked_add(half)
            .map(|v| v / divisor)
            .ok_or(SolMathError::Overflow)
    } else {
        value
            .checked_sub(half)
            .map(|v| v / divisor)
            .ok_or(SolMathError::Overflow)
    }
}

fn refine_large_sqrt(
    z: Complex,
    mut root: Complex,
    magnitude: u128,
) -> Result<Complex, SolMathError> {
    // The SCALE-precision normalized hypot is already within a few output
    // ULP while |z| is modest. At very large raw magnitudes its sub-ULP
    // error gets amplified by sqrt(|z|), so refine w via
    // w <- (w + z/w)/2. One step squares the relative seed error; at
    // i128-scale the remaining error is dominated by the division's few raw
    // ULP rather than by the normalized-hypot seed.
    if magnitude <= 16 * SCALE {
        return Ok(root);
    }
    let reciprocal = complex_div(z, root)?;
    root = Complex::new(
        root.re
            .checked_add(reciprocal.re)
            .ok_or(SolMathError::Overflow)?
            / 2,
        root.im
            .checked_add(reciprocal.im)
            .ok_or(SolMathError::Overflow)?
            / 2,
    );
    Ok(root)
}

#[cfg(test)]
mod adversarial_tests {
    use super::*;
    use crate::constants::SCALE_I;

    #[test]
    fn sqrt_preserves_one_raw_ulp() {
        assert_eq!(complex_sqrt(Complex::new(1, 0)).unwrap().re, 1_000_000);
    }

    #[test]
    fn division_by_small_nonzero_value_is_not_zero_division() {
        let q = complex_div(Complex::new(SCALE_I, 0), Complex::new(1, 0)).unwrap();
        assert_eq!(q, Complex::new(SCALE_I * SCALE_I, 0));
    }

    #[test]
    fn division_preserves_tiny_numerator_and_denominator() {
        assert_eq!(
            complex_div(Complex::new(1, 1), Complex::new(1, 1)).unwrap(),
            Complex::new(SCALE_I, 0)
        );
        let q = complex_div(Complex::new(1, 0), Complex::new(1, 1)).unwrap();
        assert!((q.re - SCALE_I / 2).abs() <= 1);
        assert!((q.im + SCALE_I / 2).abs() <= 1);

        assert_eq!(
            complex_div(Complex::new(1, 2), Complex::new(3, 4)),
            Ok(Complex::new(440_000_000_000, 80_000_000_000)),
        );
    }

    #[test]
    fn imaginary_axis_negation_is_checked() {
        assert_eq!(
            complex_div(Complex::new(i128::MIN, 0), Complex::new(0, SCALE_I),),
            Err(SolMathError::Overflow)
        );
    }

    #[test]
    fn sqrt_preserves_half_raw_unit() {
        let q = complex_sqrt(Complex::new(0, 1)).unwrap();
        assert!(q.re.abs_diff(707_107) <= 1, "q={q:?}");
        assert!(q.im.abs_diff(707_107) <= 1, "q={q:?}");
    }

    #[test]
    fn division_preserves_sub_ulp_denominator_ratio_before_magnification() {
        let q = complex_div(Complex::new(0, i128::MAX), Complex::new(2 * SCALE_I, 1)).unwrap();
        let expected_re = 42_535_295_865_117_307_932_921_815i128;
        let expected_im = 85_070_591_730_234_615_865_843_630_590_294_120_304i128;
        assert!(q.re.abs_diff(expected_re) <= 16, "q={q:?}");
        assert!(q.im.abs_diff(expected_im) <= 16, "q={q:?}");
    }

    #[test]
    fn sqrt_large_opposite_parity_does_not_false_overflow() {
        let q = complex_sqrt(Complex::new(i128::MAX - 1, i128::MAX)).unwrap();
        let expected = Complex::new(
            14_331_035_423_661_364_728_695_749,
            5_936_109_235_329_791_326_823_897,
        );
        assert!(q.re.abs_diff(expected.re) <= 2, "q={q:?}");
        assert!(q.im.abs_diff(expected.im) <= 2, "q={q:?}");
    }

    #[test]
    fn multiplication_combines_wide_products_before_range_check() {
        let z = Complex::new(
            14_331_035_423_661_364_728_695_748,
            5_936_109_235_329_791_326_823_896,
        );
        assert_eq!(
            complex_mul(z, z),
            Ok(Complex::new(
                170_141_183_460_469_231_731_687_274_811_518_222_572,
                170_141_183_460_469_231_731_687_270_436_583_769_475,
            )),
        );
    }

    #[test]
    fn division_large_equal_operands_do_not_false_overflow() {
        for z in [
            Complex::new(i128::MAX, i128::MAX),
            Complex::new(i128::MAX / 2 + 1, i128::MAX / 2 + 1),
            Complex::new(i128::MIN, i128::MIN),
            Complex::new(i128::MIN, i128::MAX),
            Complex::new(i128::MAX, i128::MIN),
        ] {
            assert_eq!(complex_div(z, z), Ok(Complex::new(SCALE_I, 0)), "z={z:?}",);
        }
    }

    #[test]
    fn division_large_near_equal_operands_is_backward_stable() {
        let a = Complex::new(i128::MAX, i128::MAX - 1);
        let b = Complex::new(i128::MAX, i128::MAX);
        let q = complex_div(a, b).unwrap();
        assert!(q.re.abs_diff(SCALE_I) <= 1, "q={q:?}");
        assert!(q.im.abs_diff(0) <= 1, "q={q:?}");
    }

    #[test]
    fn division_mixed_scale_representable_quotient_does_not_false_overflow() {
        let a = Complex::new(0, i128::MAX / 10 * 7);
        let b = Complex::new(490_000_000_000, 490_000_000_000);
        let expected = 121_529_416_757_478_022_665_490_931_225_631_504_085;
        assert_eq!(complex_div(a, b), Ok(Complex::new(expected, expected)));
    }

    #[test]
    fn sqrt_negative_real_avoids_cancellation() {
        let q = complex_sqrt(Complex::new(-4 * SCALE_I, 1_000_000)).unwrap();
        assert!(q.re > 0);
        assert!((q.im - 2 * SCALE_I).abs() <= 2);
    }
}
