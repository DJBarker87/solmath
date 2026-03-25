use crate::error::SolMathError;
use crate::overflow::checked_mul_div_rem_u;

/// Computes floor(a * b / c) with u128 intermediate to avoid u64 overflow.
/// This is a raw integer utility — it does NOT use SCALE.
///
/// # Errors
/// - `DivisionByZero` if `c == 0`.
/// - `Overflow` if the result exceeds `u64::MAX`.
///
/// # Examples
/// ```
/// use solmath_core::mul_div_floor;
/// assert_eq!(mul_div_floor(100, 200, 300).unwrap(), 66);
/// assert_eq!(mul_div_floor(u64::MAX, 2, 3).unwrap(), 12297829382473034410);
/// ```
pub fn mul_div_floor(a: u64, b: u64, c: u64) -> Result<u64, SolMathError> {
    if c == 0 {
        return Err(SolMathError::DivisionByZero);
    }
    let product = (a as u128) * (b as u128);
    let result = product / (c as u128);
    if result > u64::MAX as u128 {
        return Err(SolMathError::Overflow);
    }
    Ok(result as u64)
}

/// Computes ceil(a * b / c) with u128 intermediate to avoid u64 overflow.
/// This is a raw integer utility — it does NOT use SCALE.
///
/// Uses the identity: ceil(x / y) = (x + y - 1) / y.
///
/// # Errors
/// - `DivisionByZero` if `c == 0`.
/// - `Overflow` if the result exceeds `u64::MAX`.
///
/// # Examples
/// ```
/// use solmath_core::mul_div_ceil;
/// assert_eq!(mul_div_ceil(100, 200, 300).unwrap(), 67);
/// assert_eq!(mul_div_ceil(1, 1, 3).unwrap(), 1);
/// ```
pub fn mul_div_ceil(a: u64, b: u64, c: u64) -> Result<u64, SolMathError> {
    if c == 0 {
        return Err(SolMathError::DivisionByZero);
    }
    let product = (a as u128) * (b as u128);
    let result = (product + (c as u128) - 1) / (c as u128);
    if result > u64::MAX as u128 {
        return Err(SolMathError::Overflow);
    }
    Ok(result as u64)
}

/// Computes floor(a * b / c) with U256 intermediate to avoid u128 overflow.
/// For SCALE-valued barrier pricing, weighted pool math, and anything with u128 inputs.
/// This is a raw integer utility — it does NOT use SCALE.
///
/// # Errors
/// - `DivisionByZero` if `c == 0`.
/// - `Overflow` if the result exceeds `u128::MAX`.
pub fn mul_div_floor_u128(a: u128, b: u128, c: u128) -> Result<u128, SolMathError> {
    if c == 0 {
        return Err(SolMathError::DivisionByZero);
    }
    checked_mul_div_rem_u(a, b, c)
        .map(|(q, _)| q)
        .ok_or(SolMathError::Overflow)
}

/// Computes ceil(a * b / c) with U256 intermediate to avoid u128 overflow.
/// For SCALE-valued barrier pricing, weighted pool math, and anything with u128 inputs.
/// This is a raw integer utility — it does NOT use SCALE.
///
/// # Errors
/// - `DivisionByZero` if `c == 0`.
/// - `Overflow` if the result exceeds `u128::MAX`.
pub fn mul_div_ceil_u128(a: u128, b: u128, c: u128) -> Result<u128, SolMathError> {
    if c == 0 {
        return Err(SolMathError::DivisionByZero);
    }
    let (q, rem) = checked_mul_div_rem_u(a, b, c).ok_or(SolMathError::Overflow)?;
    if rem != 0 {
        q.checked_add(1).ok_or(SolMathError::Overflow)
    } else {
        Ok(q)
    }
}
