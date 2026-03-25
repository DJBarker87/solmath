use crate::constants::SCALE;
use crate::error::SolMathError;
use crate::arithmetic::{fp_mul, fp_div};
use crate::mul_div::mul_div_ceil_u128;
use crate::hp::pow_fixed_hp;

/// Convert raw token amount (lamports/smallest unit) to fixed-point at SCALE (1e12).
///
/// # Parameters
/// - `raw_amount` -- Token amount in smallest denomination (`u64`)
/// - `token_decimals` -- Number of decimal places for the token (e.g. 6 for USDC, 9 for SOL)
///
/// # Returns
/// Fixed-point value at SCALE.
/// Exact when `token_decimals <= 12`; otherwise truncates sub-1e-12 token dust.
///
/// # Errors
/// - `Overflow` if the scaled result exceeds `u128::MAX`.
/// - `DomainError` if `token_decimals > 38`.
pub fn token_to_fp(raw_amount: u64, token_decimals: u8) -> Result<u128, SolMathError> {
    let amount = raw_amount as u128;
    let decimals = token_decimals as u32;
    if decimals > 38 {
        return Err(SolMathError::DomainError);
    }
    if decimals <= 12 {
        amount.checked_mul(10u128.pow(12 - decimals)).ok_or(SolMathError::Overflow)
    } else {
        Ok(amount / 10u128.pow(decimals - 12))
    }
}

/// Convert fixed-point at SCALE to raw token amount, rounding down.
///
/// # Parameters
/// - `fp_amount` -- Fixed-point value at SCALE (`u128`)
/// - `token_decimals` -- Number of decimal places for the token
///
/// # Returns
/// Raw token amount (`u64`), truncated toward zero.
///
/// # Errors
/// - `DomainError` if `token_decimals > 38`.
/// - `Overflow` if the result exceeds `u64::MAX` or intermediate multiplication overflows.
pub fn fp_to_token_floor(fp_amount: u128, token_decimals: u8) -> Result<u64, SolMathError> {
    let decimals = token_decimals as u32;
    if decimals > 38 {
        return Err(SolMathError::DomainError);
    }
    let raw = if decimals <= 12 {
        fp_amount / 10u128.pow(12 - decimals)
    } else {
        fp_amount.checked_mul(10u128.pow(decimals - 12)).ok_or(SolMathError::Overflow)?
    };
    if raw > u64::MAX as u128 {
        return Err(SolMathError::Overflow);
    }
    Ok(raw as u64)
}

/// Convert fixed-point at SCALE to raw token amount, rounding up (protocol-safe).
///
/// Use this when the protocol should not under-count tokens (e.g. fees, repayments).
///
/// # Parameters
/// - `fp_amount` -- Fixed-point value at SCALE (`u128`)
/// - `token_decimals` -- Number of decimal places for the token
///
/// # Returns
/// Raw token amount (`u64`), rounded up.
///
/// # Errors
/// - `DomainError` if `token_decimals > 38`.
/// - `Overflow` if the result exceeds `u64::MAX` or intermediate arithmetic overflows.
pub fn fp_to_token_ceil(fp_amount: u128, token_decimals: u8) -> Result<u64, SolMathError> {
    let decimals = token_decimals as u32;
    if decimals > 38 {
        return Err(SolMathError::DomainError);
    }
    let divisor = if decimals <= 12 {
        10u128.pow(12 - decimals)
    } else {
        let raw = fp_amount.checked_mul(10u128.pow(decimals - 12)).ok_or(SolMathError::Overflow)?;
        if raw > u64::MAX as u128 {
            return Err(SolMathError::Overflow);
        }
        return Ok(raw as u64);
    };
    let raw = fp_amount.checked_add(divisor - 1).ok_or(SolMathError::Overflow)? / divisor;
    if raw > u64::MAX as u128 {
        return Err(SolMathError::Overflow);
    }
    Ok(raw as u64)
}

/// Weighted pool swap via Balancer invariant: compute output for given input.
///
/// Calculates `net_out` and `fee` for a constant-product weighted pool.
/// Invariant preserved to 13+ significant figures.
///
/// # Parameters
/// All at SCALE (`u128`):
/// - `balance_in` -- Reserve of the input token
/// - `balance_out` -- Reserve of the output token
/// - `weight_in` -- Weight of the input token (e.g. SCALE/2 for 50%)
/// - `weight_out` -- Weight of the output token
/// - `amount_in` -- Amount of input token being swapped
/// - `fee_rate` -- Fee as a fraction of SCALE (e.g. 3_000_000_000 = 0.3%)
///
/// # Returns
/// `(net_out, fee)` at SCALE.
///
/// # Errors
/// - `DivisionByZero` if `weight_out == 0`
/// - `DomainError` if `weight_in == 0`, `balance_in == 0`, `balance_out == 0`, or `fee_rate > SCALE`
/// - `Overflow` if `balance_in + amount_in` overflows or power computation fails
///
/// # Example
/// ```
/// use solmath_core::{weighted_pool_swap, SCALE};
/// // Equal-weight pool: 1000 tokens each, swap 10 in, 0.3% fee
/// let (net_out, fee) = weighted_pool_swap(
///     1000 * SCALE, 1000 * SCALE,
///     SCALE / 2, SCALE / 2,
///     10 * SCALE,
///     3_000_000_000, // 0.3%
/// )?;
/// assert!(net_out > 0);
/// # Ok::<(), solmath_core::SolMathError>(())
/// ```
pub fn weighted_pool_swap(
    balance_in: u128,
    balance_out: u128,
    weight_in: u128,
    weight_out: u128,
    amount_in: u128,
    fee_rate: u128,
) -> Result<(u128, u128), SolMathError> {
    if weight_out == 0 {
        return Err(SolMathError::DivisionByZero);
    }
    if weight_in == 0 {
        return Err(SolMathError::DomainError);
    }
    if balance_in == 0 || balance_out == 0 {
        return Err(SolMathError::DomainError);
    }
    if fee_rate > SCALE {
        return Err(SolMathError::DomainError);
    }
    if amount_in == 0 {
        return Ok((0, 0));
    }

    // ratio = B_i / (B_i + a_in)
    let denominator = balance_in.checked_add(amount_in).ok_or(SolMathError::Overflow)?;
    let ratio = fp_div(balance_in, denominator)?;

    // weight_ratio = w_i / w_j
    let weight_ratio = fp_div(weight_in, weight_out)?;

    // power = ratio ^ weight_ratio (using HP for precision)
    let power = pow_fixed_hp(ratio, weight_ratio)?;

    // gross_out = B_j × (1 - power)
    // power ∈ [0, SCALE] from pow_fixed_hp; if power > SCALE, it's an invariant
    // violation from the power computation — surface it as an error.
    if power > SCALE {
        return Err(SolMathError::Overflow);
    }
    let one_minus_power = SCALE - power;
    // gross_out rounds DOWN (trader gets less) — protocol-favorable
    let gross_out = fp_mul(balance_out, one_minus_power)?;

    // fee rounds UP (protocol collects at least the fee) — protocol-favorable
    let fee = mul_div_ceil_u128(gross_out, fee_rate, SCALE)?;

    // net_out = gross_out - fee
    // fee = gross_out * fee_rate / SCALE ≤ gross_out (fee_rate ≤ SCALE, guarded above)
    if fee > gross_out {
        return Err(SolMathError::Overflow);
    }
    let net_out = gross_out - fee;

    Ok((net_out, fee))
}
