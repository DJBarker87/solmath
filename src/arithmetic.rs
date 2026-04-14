use crate::constants::*;
use crate::double_word::DoubleWord;
use crate::error::SolMathError;
use crate::overflow::{checked_mul_div_i, checked_mul_div_u, checked_mul_div_rem_u};

/// Unsigned fixed-point multiply: `(a * b) / SCALE`.
///
/// Both `a` and `b` are `u128` values at SCALE (1e12).
/// Returns the product at SCALE. Error: 1 ULP max (truncation toward zero).
///
/// Returns `Err(Overflow)` if the result exceeds `u128::MAX`.
///
/// # Example
/// ```
/// use solmath::{fp_mul, SCALE};
/// let result = fp_mul(2 * SCALE, 3 * SCALE).unwrap();
/// assert_eq!(result, 6 * SCALE);
/// # Ok::<(), solmath::SolMathError>(())
/// ```
#[inline]
pub fn fp_mul(a: u128, b: u128) -> Result<u128, SolMathError> {
    match a.checked_mul(b) {
        Some(p) => Ok(p / SCALE),
        None => checked_mul_div_u(a, b, SCALE).ok_or(SolMathError::Overflow),
    }
}

/// Signed fixed-point multiply: `(a * b) / SCALE`.
///
/// Both `a` and `b` are `i128` values at SCALE (1e12).
/// Returns the product at SCALE. Exact (truncation toward zero).
///
/// Returns `Err(Overflow)` if the result exceeds `i128` range.
///
/// # Example
/// ```
/// use solmath::{fp_mul_i, SCALE_I};
/// let result = fp_mul_i(-2 * SCALE_I, 3 * SCALE_I).unwrap();
/// assert_eq!(result, -6 * SCALE_I);
/// ```
#[inline]
pub fn fp_mul_i(a: i128, b: i128) -> Result<i128, SolMathError> {
    match a.checked_mul(b) {
        Some(p) => Ok(p / SCALE_I),
        None => checked_mul_div_i(a, b, SCALE_I),
    }
}

/// Internal unchecked fixed-point multiply. Inputs MUST be bounded
/// such that |a * b| < i128::MAX. No overflow check — caller's
/// responsibility. Not exposed in public API.
#[allow(dead_code)]
#[inline]
pub(crate) fn fp_mul_i_fast(a: i128, b: i128) -> i128 {
    a * b / SCALE_I
}

/// Signed fixed-point multiply with rounding: round((a × b) / SCALE).
/// Error: ≤ 0.5 ULP. Returns `Err(Overflow)` if the result exceeds `i128` range.
#[inline]
pub fn fp_mul_i_round(a: i128, b: i128) -> Result<i128, SolMathError> {
    match a.checked_mul(b) {
        Some(p) => {
            if p >= 0 {
                match p.checked_add(SCALE_I / 2) {
                    Some(v) => Ok(v / SCALE_I),
                    None => Ok(p / SCALE_I + 1),
                }
            } else {
                match p.checked_sub(SCALE_I / 2) {
                    Some(v) => Ok(v / SCALE_I),
                    None => Ok(p / SCALE_I - 1),
                }
            }
        }
        None => {
            // Overflow path: use U256 intermediate with correct rounding.
            let neg = (a < 0) != (b < 0);
            let (q, rem) = checked_mul_div_rem_u(a.unsigned_abs(), b.unsigned_abs(), SCALE)
                .ok_or(SolMathError::Overflow)?;
            let rounded = if rem >= SCALE / 2 {
                q.checked_add(1).ok_or(SolMathError::Overflow)?
            } else {
                q
            };
            if neg {
                if rounded == (1u128 << 127) { Ok(i128::MIN) }
                else if rounded < (1u128 << 127) { Ok(-(rounded as i128)) }
                else { Err(SolMathError::Overflow) }
            } else if rounded <= i128::MAX as u128 { Ok(rounded as i128) }
            else { Err(SolMathError::Overflow) }
        }
    }
}

/// Unsigned fixed-point division: `(a * SCALE) / b`.
///
/// Both `a` and `b` are `u128` values at SCALE (1e12).
/// Returns the quotient at SCALE. Error: 1 ULP max (truncation toward zero).
///
/// Returns `Err(DivisionByZero)` if `b == 0`, or `Err(Overflow)` if the result exceeds `u128::MAX`.
///
/// # Example
/// ```
/// use solmath::{fp_div, SCALE};
/// let result = fp_div(10 * SCALE, 2 * SCALE).unwrap();
/// assert_eq!(result, 5 * SCALE);
/// # Ok::<(), solmath::SolMathError>(())
/// ```
#[inline]
pub fn fp_div(a: u128, b: u128) -> Result<u128, SolMathError> {
    if b == 0 {
        return Err(SolMathError::DivisionByZero);
    }
    fp_div_rem_experimental_u(a, b)
        .map(|(q, _)| q)
        .ok_or(SolMathError::Overflow)
}

/// Signed fixed-point division: (a × SCALE) / b.
/// Error: < 1 ULP (truncation). Returns Err on division by zero or overflow.
#[inline]
pub fn fp_div_i(a: i128, b: i128) -> Result<i128, SolMathError> {
    if b == 0 {
        return Err(SolMathError::DivisionByZero);
    }
    if a == 0 {
        return Ok(0);
    }

    let neg = (a < 0) ^ (b < 0);
    let mag = fp_div_rem_experimental_u(a.unsigned_abs(), b.unsigned_abs());

    match mag {
        Some((q, _)) => {
            if neg {
                if q == (1u128 << 127) {
                    Ok(i128::MIN)
                } else if q < (1u128 << 127) {
                    Ok(-(q as i128))
                } else {
                    Err(SolMathError::Overflow)
                }
            } else if q <= i128::MAX as u128 {
                Ok(q as i128)
            } else {
                Err(SolMathError::Overflow)
            }
        }
        None => Err(SolMathError::Overflow),
    }
}

/// Unsigned fixed-point floor division: (a × SCALE) / b, truncated.
/// Exact: 0 ULP (unsigned truncation is floor). Returns Err on division by zero or overflow.
#[inline]
pub fn fp_div_floor(a: u128, b: u128) -> Result<u128, SolMathError> {
    fp_div(a, b) // unsigned truncation is already floor
}

/// Unsigned fixed-point ceil division: (a × SCALE) / b, rounded up.
/// Exact: 0 ULP (exact ceil). Returns Err on division by zero or overflow.
#[inline]
pub fn fp_div_ceil(a: u128, b: u128) -> Result<u128, SolMathError> {
    if b == 0 {
        return Err(SolMathError::DivisionByZero);
    }
    match fp_div_rem_experimental_u(a, b) {
        Some((q, rem)) => {
            if rem > 0 { Ok(q.checked_add(1).ok_or(SolMathError::Overflow)?) } else { Ok(q) }
        }
        None => Err(SolMathError::Overflow),
    }
}

/// Rounding signed division: (a × SCALE) / b, rounded to nearest.
/// Same as fp_div_i but rounds instead of truncating. Returns `Err(Overflow)` on overflow.
/// Internal — called by mills_ratio_cf8.
#[allow(dead_code)]
#[inline]
pub(crate) fn fp_div_i_round(a: i128, b: i128) -> Result<i128, SolMathError> {
    if b == 0 { return Err(SolMathError::DivisionByZero); }
    if a == 0 { return Ok(0); }

    let neg = (a < 0) ^ (b < 0);
    let (q, r) = fp_div_rem_experimental_u(a.unsigned_abs(), b.unsigned_abs())
        .ok_or(SolMathError::Overflow)?;
    let round_up = r >= b.unsigned_abs() / 2;
    let q = if round_up { q.checked_add(1).ok_or(SolMathError::Overflow)? } else { q };
    if neg {
        if q == (1u128 << 127) { Ok(i128::MIN) }
        else if q < (1u128 << 127) { Ok(-(q as i128)) }
        else { Err(SolMathError::Overflow) }
    } else if q <= i128::MAX as u128 { Ok(q as i128) }
    else { Err(SolMathError::Overflow) }
}

/// Fractional tail of fixed-point division. Internal — called by fp_div_rem_experimental_u.
#[inline]
pub(crate) fn fp_div_fractional_tail_u(a: u128, b: u128) -> Option<(u128, u128)> {
    debug_assert!(b != 0);
    if a == 0 {
        return Some((0, 0));
    }
    if a <= FP_DIV_THIN_MAX {
        // a ≤ FP_DIV_THIN_MAX = u128::MAX / SCALE; a * SCALE ≤ u128::MAX, no overflow
        let scaled = a * SCALE;
        return Some((scaled / b, scaled % b));
    }
    checked_mul_div_rem_u(a, SCALE, b)
}

/// Three-path fixed-point division with remainder. Internal — core of fp_div.
#[inline]
pub(crate) fn fp_div_rem_experimental_u(a: u128, b: u128) -> Option<(u128, u128)> {
    if b == 0 {
        return None;
    }
    if a == 0 {
        return Some((0, 0));
    }

    // Cheapest path when the classic scaled intermediate still fits.
    if a <= FP_DIV_THIN_MAX {
        // a ≤ FP_DIV_THIN_MAX = u128::MAX / SCALE; a * SCALE ≤ u128::MAX, no overflow
        let scaled = a * SCALE;
        return Some((scaled / b, scaled % b));
    }

    // Quotient/remainder decomposition:
    // a = q*b + r  =>  (a*SCALE)/b = q*SCALE + (r*SCALE)/b
    // This avoids forming the huge scaled product on many overflow-prone inputs.
    let q = a / b;
    if q == 0 {
        return checked_mul_div_rem_u(a, SCALE, b);
    }
    if q > FP_DIV_THIN_MAX {
        return None;
    }

    // q = a / b, so q * b ≤ a; remainder r = a - q*b ≥ 0 and < b; no underflow
    let r = a - q * b;
    // q ≤ FP_DIV_THIN_MAX = u128::MAX / SCALE (checked above); q * SCALE ≤ u128::MAX, no overflow
    let base = q * SCALE;
    if r == 0 {
        return Some((base, 0));
    }

    let (frac, rem) = fp_div_fractional_tail_u(r, b)?;
    Some((base.checked_add(frac)?, rem))
}

/// Integer square root via Newton's method. Internal — used by fp_sqrt.
#[allow(dead_code)]
#[inline]
pub(crate) fn isqrt_u128(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let mut x = 1u128 << ((128 - n.leading_zeros() + 1) / 2);
    loop {
        let x1 = (x + n / x) / 2;
        if x1 >= x {
            return x;
        }
        x = x1;
    }
}

/// Newton's method sqrt on pre-scaled input. Internal — used by fp_sqrt fast path.
pub(crate) fn sqrt_scaled_newton(scaled: u128) -> u128 {
    let bit_len = 128 - scaled.leading_zeros();
    let mut guess: u128 = 1u128 << ((bit_len + 1) / 2);
    for _ in 0..8 {
        if guess == 0 {
            break;
        }
        let new_guess = (guess + scaled / guess) / 2;
        if new_guess == guess || new_guess + 1 == guess {
            guess = guess.min(new_guess);
            break;
        }
        guess = new_guess;
    }
    debug_assert!(
        guess.checked_mul(guess).map_or(false, |sq| sq <= scaled)
            && (guess + 1).checked_mul(guess + 1).map_or(true, |sq| sq > scaled),
        "sqrt_scaled_newton: post-check failed for scaled={}, guess={}",
        scaled, guess
    );
    guess
}

/// Widening 128×128 → 256-bit multiply. Internal — used by fp_sqrt overflow path.
pub(crate) fn wide_mul_u128(a: u128, b: u128) -> (u128, u128) {
    let mask = u128::from(u64::MAX);
    let a0 = a & mask;
    let a1 = a >> 64;
    let b0 = b & mask;
    let b1 = b >> 64;

    let p0 = a0 * b0;
    let p1 = a0 * b1;
    let p2 = a1 * b0;
    let p3 = a1 * b1;

    let carry0 = p0 >> 64;
    let mid = (p1 & mask) + (p2 & mask) + carry0;
    let lo = (p0 & mask) | ((mid & mask) << 64);
    let hi = p3 + (p1 >> 64) + (p2 >> 64) + (mid >> 64);
    (hi, lo)
}

/// Compare two 256-bit (hi, lo) pairs. Internal — used by fp_sqrt bisection.
pub(crate) fn cmp_wide(lhs: (u128, u128), rhs: (u128, u128)) -> core::cmp::Ordering {
    if lhs.0 != rhs.0 {
        lhs.0.cmp(&rhs.0)
    } else {
        lhs.1.cmp(&rhs.1)
    }
}

/// Check if candidate² vs x×SCALE via wide multiply. Internal — used by fp_sqrt.
pub(crate) fn cmp_sqrt_candidate(candidate: u128, x: u128) -> core::cmp::Ordering {
    cmp_wide(wide_mul_u128(candidate, candidate), wide_mul_u128(x, SCALE))
}

/// Fixed-point square root: `sqrt(x)` at SCALE.
///
/// `x` is a `u128` value at SCALE (1e12). Returns `sqrt(x)` at SCALE.
/// Error: 1 ULP max. Returns `Ok(0)` if `x == 0`.
///
/// # Example
/// ```
/// use solmath::{fp_sqrt, SCALE};
/// // sqrt(4.0) = 2.0
/// let result = fp_sqrt(4 * SCALE).unwrap();
/// assert_eq!(result, 2 * SCALE);
/// # Ok::<(), solmath::SolMathError>(())
/// ```
pub fn fp_sqrt(x: u128) -> Result<u128, SolMathError> {
    if x == 0 {
        return Ok(0);
    }

    if let Some(scaled) = x.checked_mul(SCALE) {
        return Ok(sqrt_scaled_newton(scaled));
    }

    let mut reduced = x;
    let mut scale_back = 1u128;
    while reduced > u128::MAX / SCALE {
        // sqrt(4a) = 2*sqrt(a); round while reducing to preserve monotonicity.
        reduced = (reduced + 2) / 4;
        scale_back = scale_back.checked_mul(2).ok_or(SolMathError::Overflow)?;
    }

    let approx = sqrt_scaled_newton(reduced * SCALE)
        .checked_mul(scale_back).ok_or(SolMathError::Overflow)?;

    let mut low = approx.checked_sub(scale_back).ok_or(SolMathError::Overflow)?;
    while cmp_sqrt_candidate(low, x).is_gt() {
        if low == 0 {
            break;
        }
        low = low.checked_sub(scale_back).ok_or(SolMathError::Overflow)?;
    }

    let mut high = approx.checked_add(scale_back).ok_or(SolMathError::Overflow)?;
    while !cmp_sqrt_candidate(high, x).is_gt() {
        low = high;
        high = high.checked_add(scale_back).ok_or(SolMathError::Overflow)?;
    }

    while low + 1 < high {
        let mid = low + (high - low) / 2;
        if cmp_sqrt_candidate(mid, x).is_gt() {
            high = mid;
        } else {
            low = mid;
        }
    }

    Ok(low)
}

/// Unsigned fixed-point multiply with rounding: round((a × b) / SCALE).
/// Error: ≤ 0.5 ULP. Returns `Err(Overflow)` on overflow.
#[inline]
pub fn fp_mul_round(a: u128, b: u128) -> Result<u128, SolMathError> {
    match checked_mul_div_rem_u(a, b, SCALE) {
        Some((q, r)) => {
            if r >= SCALE / 2 { q.checked_add(1).ok_or(SolMathError::Overflow) } else { Ok(q) }
        }
        None => Err(SolMathError::Overflow),
    }
}

/// Signed fixed-point multiply returning a DoubleWord: exact result split
/// into standard-precision quotient and sub-ULP remainder.
///
/// dw.hi == fp_mul_i_round(a, b) for all inputs where a*b fits in i128.
/// dw.lo is the exact remainder: |dw.lo| < SCALE_I.
/// true_product(a, b) / SCALE = dw.hi + dw.lo / SCALE.
///
/// For inputs where a*b overflows i128, this function still rounds correctly
/// (unlike fp_mul_i_round which truncates on its overflow path).
#[inline]
pub fn fp_mul_i_round_dw(a: i128, b: i128) -> Result<DoubleWord, SolMathError> {
    if a == 0 || b == 0 {
        return Ok(DoubleWord::new_raw(0, 0));
    }

    let neg = (a < 0) != (b < 0);
    let aa = a.unsigned_abs();
    let bb = b.unsigned_abs();

    let (q_trunc, r_trunc) = checked_mul_div_rem_u(aa, bb, SCALE)
        .ok_or(SolMathError::Overflow)?;
    // q_trunc = floor(|a*b| / SCALE), r_trunc in [0, SCALE)

    let half = SCALE / 2;
    let (q, lo) = if r_trunc >= half {
        (q_trunc + 1, r_trunc as i128 - SCALE_I)  // lo in (-SCALE/2, 0]
    } else {
        (q_trunc, r_trunc as i128)                  // lo in [0, SCALE/2)
    };

    if neg {
        if q == (1u128 << 127) {
            if lo != 0 {
                return Err(SolMathError::Overflow);
            }
            Ok(DoubleWord::new_raw(i128::MIN, 0))
        } else if q < (1u128 << 127) {
            Ok(DoubleWord::new_raw(-(q as i128), -lo))
        } else {
            Err(SolMathError::Overflow)
        }
    } else if q <= i128::MAX as u128 {
        Ok(DoubleWord::new_raw(q as i128, lo))
    } else {
        Err(SolMathError::Overflow)
    }
}

/// Unsigned fixed-point division with rounding: round((a × SCALE) / b).
/// Error: ≤ 0.5 ULP. Returns Err on division by zero or overflow.
#[inline]
pub fn fp_div_round(a: u128, b: u128) -> Result<u128, SolMathError> {
    if b == 0 {
        return Err(SolMathError::DivisionByZero);
    }
    let (q, r) = fp_div_rem_experimental_u(a, b).ok_or(SolMathError::Overflow)?;
    Ok(if r >= b / 2 {
        q.checked_add(1).ok_or(SolMathError::Overflow)?
    } else {
        q
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{SCALE, SCALE_I};
    use crate::error::SolMathError;

    // ===== fp_mul_round tests =====

    #[test]
    fn test_fp_mul_round_exact() {
        assert_eq!(fp_mul_round(2 * SCALE, 3 * SCALE).unwrap(), 6 * SCALE);
        assert_eq!(fp_mul_round(SCALE, SCALE).unwrap(), SCALE);
        assert_eq!(fp_mul_round(0, SCALE).unwrap(), 0);
        assert_eq!(fp_mul_round(SCALE, 0).unwrap(), 0);
    }

    #[test]
    fn test_fp_mul_round_vs_truncating() {
        let cases: &[(u128, u128)] = &[
            (SCALE / 3, SCALE), (SCALE, SCALE / 3),
            (SCALE / 7, SCALE / 11), (2 * SCALE, SCALE / 3),
            (SCALE + 1, SCALE + 1), (SCALE * 1000, SCALE / 997),
        ];
        for &(a, b) in cases {
            let trunc = fp_mul(a, b).unwrap();
            let round = fp_mul_round(a, b).unwrap();
            assert!(round == trunc || round == trunc + 1,
                "a={}, b={}: trunc={}, round={}", a, b, trunc, round);
        }
    }

    #[test]
    fn test_fp_mul_round_large_no_panic() {
        let large = u128::MAX / SCALE;
        let _ = fp_mul_round(large, SCALE);
        let _ = fp_mul_round(large, 2);
        let _ = fp_mul_round(SCALE, large);
    }

    #[test]
    fn test_fp_mul_round_overflow_is_error() {
        assert!(matches!(fp_mul_round(u128::MAX, u128::MAX), Err(SolMathError::Overflow)));
    }

    // ===== fp_mul_i_round_dw tests =====

    #[test]
    fn test_dw_mul_quotient_matches_fp_mul_i_round() {
        let values: &[i128] = &[
            0, 1, -1, 2, -2,
            SCALE_I / 2, -SCALE_I / 2,
            SCALE_I, -SCALE_I,
            SCALE_I + 1, -(SCALE_I + 1),
            SCALE_I - 1, -(SCALE_I - 1),
            SCALE_I * 2, -(SCALE_I * 2),
            SCALE_I * 50, -(SCALE_I * 50),
            SCALE_I / 3, -(SCALE_I / 3),
            SCALE_I / 7, -(SCALE_I / 7),
            999_999_999_999, -999_999_999_999,
            500_000_000_001, -500_000_000_001,
        ];
        for &a in values {
            for &b in values {
                if a.checked_mul(b).is_some() {
                    let expected = fp_mul_i_round(a, b).unwrap();
                    let dw = fp_mul_i_round_dw(a, b).unwrap();
                    assert_eq!(dw.hi(), expected,
                        "Quotient mismatch: a={}, b={}, expected={}, got={}", a, b, expected, dw.hi());
                }
            }
        }
    }

    #[test]
    fn test_dw_mul_remainder_bounded() {
        let values: &[i128] = &[
            0, 1, -1, SCALE_I, -SCALE_I, SCALE_I / 3, -SCALE_I / 7,
            SCALE_I * 50, -SCALE_I * 50, 999_999_999_999,
        ];
        for &a in values {
            for &b in values {
                let dw = fp_mul_i_round_dw(a, b).unwrap();
                assert!(dw.lo().abs() < SCALE_I,
                    "Remainder out of bounds: a={}, b={}, lo={}", a, b, dw.lo());
            }
        }
    }

    #[test]
    fn test_dw_mul_remainder_sign_convention() {
        let dw = fp_mul_i_round_dw(SCALE_I / 3, SCALE_I).unwrap();
        assert!(dw.lo() >= 0, "Expected non-negative lo for 1/3: lo={}", dw.lo());

        let dw2 = fp_mul_i_round_dw(2 * SCALE_I / 3, SCALE_I).unwrap();
        assert!(dw2.lo().abs() < SCALE_I);
    }

    #[test]
    fn test_dw_mul_zero() {
        let dw = fp_mul_i_round_dw(0, SCALE_I).unwrap();
        assert_eq!(dw.hi(), 0);
        assert_eq!(dw.lo(), 0);
        let dw2 = fp_mul_i_round_dw(SCALE_I, 0).unwrap();
        assert_eq!(dw2.hi(), 0);
        assert_eq!(dw2.lo(), 0);
    }

    #[test]
    fn test_dw_mul_symmetry() {
        let pos = fp_mul_i_round_dw(SCALE_I / 3, SCALE_I / 7).unwrap();
        let neg = fp_mul_i_round_dw(-SCALE_I / 3, SCALE_I / 7).unwrap();
        assert_eq!(pos.hi(), -neg.hi(), "hi not symmetric");
        assert_eq!(pos.lo(), -neg.lo(), "lo not symmetric");
    }

    #[test]
    fn test_dw_mul_allows_exact_i128_min() {
        let dw = fp_mul_i_round_dw(i128::MIN, SCALE_I).unwrap();
        assert_eq!(dw.hi(), i128::MIN);
        assert_eq!(dw.lo(), 0);
    }

    #[test]
    fn test_dw_mul_to_i128_matches_fp_mul_i_round() {
        let values: &[i128] = &[
            SCALE_I / 3, SCALE_I / 7, SCALE_I * 2, -SCALE_I / 3, -SCALE_I * 5,
        ];
        for &a in values {
            for &b in values {
                if a.checked_mul(b).is_some() {
                    let direct = fp_mul_i_round(a, b).unwrap();
                    let via_dw = fp_mul_i_round_dw(a, b).unwrap().to_i128();
                    assert_eq!(direct, via_dw,
                        "Roundtrip mismatch: a={}, b={}", a, b);
                }
            }
        }
    }

    // ===== fp_div_round tests =====

    #[test]
    fn test_fp_div_round_exact() {
        assert_eq!(fp_div_round(10 * SCALE, 5 * SCALE).unwrap(), 2 * SCALE);
        assert_eq!(fp_div_round(SCALE, SCALE).unwrap(), SCALE);
        assert_eq!(fp_div_round(0, SCALE).unwrap(), 0);
    }

    #[test]
    fn test_fp_div_round_rounds_up() {
        let t = fp_div(2 * SCALE, 3 * SCALE).unwrap();
        let r = fp_div_round(2 * SCALE, 3 * SCALE).unwrap();
        assert_eq!(r, t + 1, "2/3 should round up");
    }

    #[test]
    fn test_fp_div_round_no_round() {
        let t = fp_div(SCALE, 3 * SCALE).unwrap();
        let r = fp_div_round(SCALE, 3 * SCALE).unwrap();
        assert_eq!(r, t, "1/3 should not round up");
    }

    #[test]
    fn test_fp_div_round_agrees_with_signed() {
        let cases: &[(u128, u128)] = &[
            (SCALE, 3 * SCALE), (2 * SCALE, 3 * SCALE),
            (7 * SCALE, 11 * SCALE), (SCALE, 7 * SCALE),
            (100 * SCALE, 97 * SCALE),
        ];
        for &(a, b) in cases {
            let unsigned_r = fp_div_round(a, b).unwrap();
            let signed_r = fp_div_i_round(a as i128, b as i128).unwrap();
            assert_eq!(unsigned_r as i128, signed_r,
                "Disagree for a={}, b={}", a, b);
        }
    }

    #[test]
    fn test_fp_div_round_within_one_of_truncating() {
        let values: &[u128] = &[1, 2, 3, 7, 10, 13, 100, 997, SCALE, SCALE + 1, SCALE * 1000];
        for &a in values {
            for &b in values {
                let t = fp_div(a, b).unwrap();
                let r = fp_div_round(a, b).unwrap();
                assert!(r == t || r == t + 1,
                    "a={}, b={}: trunc={}, round={}", a, b, t, r);
            }
        }
    }

    #[test]
    fn test_fp_div_round_zero_divisor() {
        assert!(matches!(fp_div_round(SCALE, 0), Err(SolMathError::DivisionByZero)));
    }

    #[test]
    fn test_fp_div_round_large_no_panic() {
        let large = u128::MAX / (SCALE * 2);
        assert!(fp_div_round(large, SCALE).is_ok());
        assert!(fp_div_round(SCALE, large).is_ok());
    }

}
