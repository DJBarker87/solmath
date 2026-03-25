use crate::constants::{U256, MulDivRounding};
use crate::error::SolMathError;

/// U256 long division. Internal — fallback for checked_mul_div_rem_u.
#[inline]
pub(crate) fn div_rem_u256_long(numerator: U256, divisor: U256) -> (U256, U256) {
    let mut rem = U256::zero();
    let mut quo = U256::zero();

    for bit_idx in (0..256).rev() {
        rem.shl1();
        if numerator.bit(bit_idx) {
            rem.limbs[0] |= 1;
        }
        if rem.ge(&divisor) {
            rem.overflowing_sub_assign(&divisor);
            quo.set_bit(bit_idx);
        }
    }

    (quo, rem)
}

/// Overflow-safe (a × b) / c with remainder via U256. Internal — core of checked_mul_div.
#[inline]
pub(crate) fn checked_mul_div_rem_u(a: u128, b: u128, c: u128) -> Option<(u128, u128)> {
    if c == 0 {
        return None;
    }
    if a == 0 || b == 0 {
        return Some((0, 0));
    }
    if let Some(product) = a.checked_mul(b) {
        return Some((product / c, product % c));
    }

    let numerator = U256::mul_u128(a, b);
    let (quo, rem) = if c <= u64::MAX as u128 {
        let (quo, rem) = numerator.div_rem_u64(c as u64);
        (quo, rem as u128)
    } else {
        numerator.div_rem_u128(c)
    };

    if quo.high_u128_nonzero() {
        return None;
    }

    debug_assert_eq!(
        (quo, U256::from_u128(rem)),
        div_rem_u256_long(numerator, U256::from_u128(c))
    );

    Some((quo.low_u128(), rem))
}

/// Overflow-safe (a × b) / c, discarding remainder. Internal.
#[inline]
pub(crate) fn checked_mul_div_u(a: u128, b: u128, c: u128) -> Option<u128> {
    checked_mul_div_rem_u(a, b, c).map(|(q, _)| q)
}

/// Overflow-safe signed (a × b) / c with configurable rounding. Internal.
#[inline]
pub(crate) fn checked_mul_div_round_i(a: i128, b: i128, c: i128, rounding: MulDivRounding) -> Result<i128, SolMathError> {
    if c == 0 {
        return Err(SolMathError::DivisionByZero);
    }
    if a == 0 || b == 0 {
        return Ok(0);
    }

    let neg = (a < 0) ^ (b < 0) ^ (c < 0);
    let aa = a.unsigned_abs();
    let bb = b.unsigned_abs();
    let cc = c.unsigned_abs();
    let (mut mag, rem) = checked_mul_div_rem_u(aa, bb, cc).ok_or(SolMathError::Overflow)?;

    if rem != 0 {
        match rounding {
            MulDivRounding::ToZero => {}
            MulDivRounding::Floor if neg => mag = mag.checked_add(1).ok_or(SolMathError::Overflow)?,
            MulDivRounding::Ceil if !neg => mag = mag.checked_add(1).ok_or(SolMathError::Overflow)?,
            _ => {}
        }
    }

    if neg {
        if mag == (1u128 << 127) {
            Ok(i128::MIN)
        } else if mag < (1u128 << 127) {
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

/// Overflow-safe signed multiply-then-divide: `(a * b) / c`, truncating toward zero.
///
/// All parameters and the return value are plain `i128` (not fixed-point).
/// Uses a `U256` intermediate so the `a * b` product cannot overflow.
/// Exact: 0 ULP. Returns `Err(DivisionByZero)` or `Err(Overflow)`.
#[inline]
pub fn checked_mul_div_i(a: i128, b: i128, c: i128) -> Result<i128, SolMathError> {
    checked_mul_div_round_i(a, b, c, MulDivRounding::ToZero)
}

/// Overflow-safe signed multiply-then-divide: `(a * b) / c`, rounding toward negative infinity.
///
/// All parameters and the return value are plain `i128` (not fixed-point).
/// Uses a `U256` intermediate so the `a * b` product cannot overflow.
/// Exact: 0 ULP. Returns `Err(DivisionByZero)` or `Err(Overflow)`.
#[inline]
pub fn checked_mul_div_floor_i(a: i128, b: i128, c: i128) -> Result<i128, SolMathError> {
    checked_mul_div_round_i(a, b, c, MulDivRounding::Floor)
}

/// Overflow-safe signed multiply-then-divide: `(a * b) / c`, rounding toward positive infinity.
///
/// All parameters and the return value are plain `i128` (not fixed-point).
/// Uses a `U256` intermediate so the `a * b` product cannot overflow.
/// Exact: 0 ULP. Returns `Err(DivisionByZero)` or `Err(Overflow)`.
#[inline]
pub fn checked_mul_div_ceil_i(a: i128, b: i128, c: i128) -> Result<i128, SolMathError> {
    checked_mul_div_round_i(a, b, c, MulDivRounding::Ceil)
}
