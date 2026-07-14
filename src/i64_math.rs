//! Compatibility NIG interface at 1e6 fixed-point scale.
//!
//! These entry points retain the published signatures, convert into the
//! checked 1e12 production implementation, and use zero dividend yield. New
//! integrations should use [`crate::nig_price_certified`] directly so the
//! dividend yield and requested absolute error are explicit.

use crate::error::SolMathError;
use crate::nig::{nig_price_certified, NigParams};

const SCALE_BRIDGE: i128 = 1_000_000;

#[inline]
fn to_scale12(value: i64) -> Result<i128, SolMathError> {
    (value as i128)
        .checked_mul(SCALE_BRIDGE)
        .ok_or(SolMathError::Overflow)
}

#[inline]
fn to_scale12_unsigned(value: i64) -> Result<u128, SolMathError> {
    if value < 0 {
        return Err(SolMathError::DomainError);
    }
    Ok(to_scale12(value)? as u128)
}

#[inline]
fn from_scale12(value: u128) -> Result<i64, SolMathError> {
    let rounded = value
        .checked_add((SCALE_BRIDGE as u128) / 2)
        .ok_or(SolMathError::Overflow)?
        / SCALE_BRIDGE as u128;
    i64::try_from(rounded).map_err(|_| SolMathError::Overflow)
}

#[allow(clippy::too_many_arguments)]
fn price_64(
    call: bool,
    s: i64,
    k: i64,
    r: i64,
    t: i64,
    alpha: i64,
    beta: i64,
    delta: i64,
) -> Result<i64, SolMathError> {
    if s <= 0 || k <= 0 || t < 0 || alpha <= 0 || delta <= 0 {
        return Err(SolMathError::DomainError);
    }
    if t == 0 {
        return Ok(if call {
            s.saturating_sub(k)
        } else {
            k.saturating_sub(s)
        });
    }

    let spot = to_scale12_unsigned(s)?;
    let strike = to_scale12_unsigned(k)?;
    // Legacy compatibility target: 5e-5 of notional, i.e. $0.005 per $100.
    let requested_max_abs_error = spot.max(strike).checked_div(20_000).unwrap_or(0).max(1);
    let quote = nig_price_certified(
        spot,
        strike,
        to_scale12(r)?,
        0,
        to_scale12_unsigned(t)?,
        NigParams {
            alpha: to_scale12_unsigned(alpha)?,
            beta: to_scale12(beta)?,
            delta_per_year: to_scale12_unsigned(delta)?,
        },
        requested_max_abs_error,
    )?;
    from_scale12(if call { quote.call } else { quote.put })
}

/// NIG call API at 1e6 scale (`q = 0`, default `$0.005 / $100` request).
pub fn nig_call_64(
    s: i64,
    k: i64,
    r: i64,
    t: i64,
    alpha: i64,
    beta: i64,
    delta: i64,
) -> Result<i64, SolMathError> {
    price_64(true, s, k, r, t, alpha, beta, delta)
}

/// NIG put API at 1e6 scale (`q = 0`, default `$0.005 / $100` request).
pub fn nig_put_64(
    s: i64,
    k: i64,
    r: i64,
    t: i64,
    alpha: i64,
    beta: i64,
    delta: i64,
) -> Result<i64, SolMathError> {
    price_64(false, s, k, r, t, alpha, beta, delta)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expiry_is_intrinsic() {
        assert_eq!(nig_call_64(120, 100, 0, 0, 1, 0, 1), Ok(20));
        assert_eq!(nig_put_64(80, 100, 0, 0, 1, 0, 1), Ok(20));
    }

    #[test]
    fn positive_expiry_routes_to_production_pricer() {
        let call = nig_call_64(
            100_000_000,
            100_000_000,
            50_000,
            1_000_000,
            10_000_000,
            -2_000_000,
            200_000,
        )
        .unwrap();
        let put = nig_put_64(
            100_000_000,
            100_000_000,
            50_000,
            1_000_000,
            10_000_000,
            -2_000_000,
            200_000,
        )
        .unwrap();
        assert!(call > 0);
        assert!(put > 0);
        // Put-call parity at the legacy output precision.
        let discounted_strike = 95_122_942;
        assert!((call - put - (100_000_000 - discounted_strike)).abs() <= 2);
    }
}
