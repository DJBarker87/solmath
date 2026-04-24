use crate::constants::SCALE;
use crate::error::SolMathError;

/// Parse a decimal string into SolMath fixed-point (`SCALE = 1e12`).
///
/// This helper is intended for tests, examples, fixtures, and off-chain
/// configuration. On-chain programs should pass already-validated integer
/// values across the instruction boundary.
///
/// The parser accepts plain decimal notation with up to 12 fractional digits.
/// Extra fractional digits are accepted only when they are zero, so precision
/// is never silently truncated.
///
/// ```
/// use solmath::{fp, SCALE};
///
/// assert_eq!(fp("100")?, 100 * SCALE);
/// assert_eq!(fp("0.05")?, 50_000_000_000);
/// assert_eq!(fp(".20")?, 200_000_000_000);
/// # Ok::<(), solmath::SolMathError>(())
/// ```
pub fn fp(decimal: &str) -> Result<u128, SolMathError> {
    if decimal.starts_with('-') {
        return Err(SolMathError::DomainError);
    }
    let unsigned = if decimal.starts_with('+') {
        &decimal[1..]
    } else {
        decimal
    };
    parse_decimal_magnitude(unsigned)
}

/// Parse a signed decimal string into SolMath fixed-point (`SCALE = 1e12`).
///
/// ```
/// use solmath::fp_i;
///
/// assert_eq!(fp_i("-0.25")?, -250_000_000_000);
/// assert_eq!(fp_i("0.25")?, 250_000_000_000);
/// # Ok::<(), solmath::SolMathError>(())
/// ```
pub fn fp_i(decimal: &str) -> Result<i128, SolMathError> {
    let (negative, unsigned) = if decimal.starts_with('-') {
        (true, &decimal[1..])
    } else if decimal.starts_with('+') {
        (false, &decimal[1..])
    } else {
        (false, decimal)
    };

    let magnitude = parse_decimal_magnitude(unsigned)?;
    if negative {
        let min_magnitude = (i128::MAX as u128)
            .checked_add(1)
            .ok_or(SolMathError::Overflow)?;
        if magnitude == min_magnitude {
            Ok(i128::MIN)
        } else if magnitude <= i128::MAX as u128 {
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

fn parse_decimal_magnitude(decimal: &str) -> Result<u128, SolMathError> {
    let bytes = decimal.as_bytes();
    if bytes.is_empty() {
        return Err(SolMathError::DomainError);
    }

    let mut whole = 0u128;
    let mut frac = 0u128;
    let mut frac_digits = 0u8;
    let mut seen_dot = false;
    let mut seen_digit = false;

    for &byte in bytes {
        match byte {
            b'0'..=b'9' => {
                seen_digit = true;
                let digit = (byte - b'0') as u128;
                if seen_dot {
                    if frac_digits < 12 {
                        frac = frac
                            .checked_mul(10)
                            .ok_or(SolMathError::Overflow)?
                            .checked_add(digit)
                            .ok_or(SolMathError::Overflow)?;
                        frac_digits += 1;
                    } else if digit != 0 {
                        return Err(SolMathError::DomainError);
                    }
                } else {
                    whole = whole
                        .checked_mul(10)
                        .ok_or(SolMathError::Overflow)?
                        .checked_add(digit)
                        .ok_or(SolMathError::Overflow)?;
                }
            }
            b'.' if !seen_dot => {
                seen_dot = true;
            }
            _ => return Err(SolMathError::DomainError),
        }
    }

    if !seen_digit {
        return Err(SolMathError::DomainError);
    }

    while frac_digits < 12 {
        frac = frac.checked_mul(10).ok_or(SolMathError::Overflow)?;
        frac_digits += 1;
    }

    whole
        .checked_mul(SCALE)
        .and_then(|scaled| scaled.checked_add(frac))
        .ok_or(SolMathError::Overflow)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::SCALE_I;

    #[test]
    fn parses_unsigned_decimals() {
        assert_eq!(fp("0").unwrap(), 0);
        assert_eq!(fp("1").unwrap(), SCALE);
        assert_eq!(fp("1.5").unwrap(), SCALE + SCALE / 2);
        assert_eq!(fp(".05").unwrap(), 50_000_000_000);
        assert_eq!(fp("+2.000000000001").unwrap(), 2 * SCALE + 1);
    }

    #[test]
    fn parses_signed_decimals() {
        assert_eq!(fp_i("-0.25").unwrap(), -250_000_000_000);
        assert_eq!(fp_i("+0.25").unwrap(), 250_000_000_000);
        assert_eq!(fp_i("0").unwrap(), 0);
        assert_eq!(fp_i("1").unwrap(), SCALE_I);
    }

    #[test]
    fn rejects_ambiguous_or_lossy_inputs() {
        assert_eq!(fp("").unwrap_err(), SolMathError::DomainError);
        assert_eq!(fp(".").unwrap_err(), SolMathError::DomainError);
        assert_eq!(fp("1.2.3").unwrap_err(), SolMathError::DomainError);
        assert_eq!(fp("1e-3").unwrap_err(), SolMathError::DomainError);
        assert_eq!(fp("-1").unwrap_err(), SolMathError::DomainError);
        assert_eq!(
            fp("0.0000000000001").unwrap_err(),
            SolMathError::DomainError
        );
        assert_eq!(fp("0.0000000000000").unwrap(), 0);
    }

    #[test]
    fn reports_overflow() {
        assert_eq!(
            fp("340282366920938463463374607.431768211456").unwrap_err(),
            SolMathError::Overflow
        );
        assert_eq!(
            fp_i("170141183460469231731687304.000000000000").unwrap_err(),
            SolMathError::Overflow
        );
    }
}
