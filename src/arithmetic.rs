use crate::constants::*;
use crate::double_word::DoubleWord;
use crate::error::SolMathError;
use crate::overflow::{checked_mul_div_i, checked_mul_div_rem_u, checked_mul_div_u};

/// Return whether a proper unsigned remainder is at least one half of its
/// divisor without evaluating `2 * remainder` (which may overflow).
///
/// Callers must maintain `divisor > 0` and `remainder < divisor`.
#[inline]
fn remainder_at_least_half(remainder: u128, divisor: u128) -> bool {
    debug_assert!(divisor > 0);
    debug_assert!(remainder < divisor);
    remainder >= divisor - remainder
}

/// Round an exact unsigned quotient/remainder pair to nearest, with ties up.
///
/// Callers must maintain `divisor > 0` and `remainder < divisor`.
#[inline]
fn round_unsigned_quotient(quotient: u128, remainder: u128, divisor: u128) -> Option<u128> {
    debug_assert!(divisor > 0);
    debug_assert!(remainder < divisor);
    if remainder_at_least_half(remainder, divisor) {
        quotient.checked_add(1)
    } else {
        Some(quotient)
    }
}

/// Round an exact signed truncating quotient/remainder pair to nearest, with
/// ties away from zero.
///
/// Callers must maintain `divisor > 0` and `|remainder| < divisor`.
#[inline]
fn round_signed_quotient(
    quotient: i128,
    remainder: i128,
    divisor: u128,
) -> Result<i128, SolMathError> {
    debug_assert!(divisor > 0);
    debug_assert!(remainder.unsigned_abs() < divisor);
    if !remainder_at_least_half(remainder.unsigned_abs(), divisor) {
        Ok(quotient)
    } else if remainder >= 0 {
        quotient.checked_add(1).ok_or(SolMathError::Overflow)
    } else {
        quotient.checked_sub(1).ok_or(SolMathError::Overflow)
    }
}

/// Clamp a computed European call to hard bounds and derive its put from
/// put-call parity. Shared by standard and HP pricing paths.
#[cfg(feature = "transcendental")]
pub(crate) fn european_prices_from_call(
    call_i: i128,
    s: u128,
    k_disc_i: i128,
) -> Result<(u128, u128), SolMathError> {
    if k_disc_i < 0 || s > i128::MAX as u128 {
        return Err(SolMathError::Overflow);
    }
    let k_disc = k_disc_i as u128;
    let call = if call_i <= 0 { 0 } else { call_i as u128 }.clamp(s.saturating_sub(k_disc), s);
    let put = if call >= s {
        (call - s).checked_add(k_disc)
    } else {
        k_disc.checked_sub(s - call)
    }
    .ok_or(SolMathError::Overflow)?;
    Ok((call, put))
}

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
///
/// The `debug_assert` enforces the precondition in debug/test builds at zero
/// release cost, so any caller that violates the bound is caught by the test
/// and fuzz suites rather than silently wrapping.
#[allow(dead_code)]
#[inline]
pub(crate) fn fp_mul_i_fast(a: i128, b: i128) -> i128 {
    debug_assert!(
        a.checked_mul(b).is_some(),
        "fp_mul_i_fast precondition violated: {a} * {b} overflows i128"
    );
    a * b / SCALE_I
}

/// Signed fixed-point multiply with rounding: round((a × b) / SCALE).
/// Error: ≤ 0.5 ULP. Returns `Err(Overflow)` if the result exceeds `i128` range.
#[inline]
pub fn fp_mul_i_round(a: i128, b: i128) -> Result<i128, SolMathError> {
    match a.checked_mul(b) {
        Some(p) => {
            let quotient = p / SCALE_I;
            let remainder = p % SCALE_I;
            round_signed_quotient(quotient, remainder, SCALE)
        }
        None => {
            // Overflow path: use U256 intermediate with correct rounding.
            let neg = (a < 0) != (b < 0);
            let (q, rem) = checked_mul_div_rem_u(a.unsigned_abs(), b.unsigned_abs(), SCALE)
                .ok_or(SolMathError::Overflow)?;
            let rounded = round_unsigned_quotient(q, rem, SCALE).ok_or(SolMathError::Overflow)?;
            if neg {
                if rounded == (1u128 << 127) {
                    Ok(i128::MIN)
                } else if rounded < (1u128 << 127) {
                    Ok(-(rounded as i128))
                } else {
                    Err(SolMathError::Overflow)
                }
            } else if rounded <= i128::MAX as u128 {
                Ok(rounded as i128)
            } else {
                Err(SolMathError::Overflow)
            }
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
            if rem > 0 {
                Ok(q.checked_add(1).ok_or(SolMathError::Overflow)?)
            } else {
                Ok(q)
            }
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
    if b == 0 {
        return Err(SolMathError::DivisionByZero);
    }
    if a == 0 {
        return Ok(0);
    }

    let neg = (a < 0) ^ (b < 0);
    let (q, r) = fp_div_rem_experimental_u(a.unsigned_abs(), b.unsigned_abs())
        .ok_or(SolMathError::Overflow)?;
    let divisor = b.unsigned_abs();
    // Compare r/divisor with 1/2 without computing 2*r (which could
    // overflow). `divisor - r` is the exact ceiling-half threshold for odd
    // divisors; using `divisor / 2` would incorrectly round a remainder of
    // floor(divisor/2) up when the divisor is odd.
    let q = round_unsigned_quotient(q, r, divisor).ok_or(SolMathError::Overflow)?;
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

/// Return whether `candidate` is the exact floor of `sqrt(radicand)`.
#[inline]
fn floor_sqrt_certificate(candidate: u128, radicand: u128) -> bool {
    candidate
        .checked_mul(candidate)
        .is_some_and(|square| square <= radicand)
        && candidate.checked_add(1).is_some_and(|next| {
            next.checked_mul(next)
                .map_or(true, |square| square > radicand)
        })
}

/// Apply one binary-search decision to an integer-square-root bracket.
#[inline]
fn bisect_sqrt_bracket(low: u128, high: u128, midpoint_is_feasible: bool) -> (u128, u128) {
    let midpoint = low + (high - low) / 2;
    if midpoint_is_feasible {
        (midpoint, high)
    } else {
        (low, midpoint)
    }
}

/// Average a Newton candidate with its exact truncating quotient.
#[inline]
fn average_sqrt_newton_candidate(candidate: u128, quotient: u128) -> u128 {
    debug_assert!(candidate.checked_add(quotient).is_some());
    (candidate + quotient) / 2
}

/// Exact integer square root via monotonically decreasing Newton iteration.
#[allow(dead_code)]
#[inline]
pub(crate) fn isqrt_u128(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let mut candidate = 1u128 << ((128 - n.leading_zeros() + 1) / 2);
    loop {
        let next = average_sqrt_newton_candidate(candidate, n / candidate);
        if next >= candidate {
            debug_assert!(floor_sqrt_certificate(candidate, n));
            return candidate;
        }
        candidate = next;
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
        let new_guess = average_sqrt_newton_candidate(guess, scaled / guess);
        if new_guess == guess || new_guess + 1 == guess {
            guess = guess.min(new_guess);
            break;
        }
        guess = new_guess;
    }
    // Newton preserves `guess >= floor(sqrt(scaled))`; one feasible square is
    // therefore a complete exactness certificate.
    if guess
        .checked_mul(guess)
        .is_some_and(|square| square <= scaled)
    {
        guess
    } else {
        // Restart through the monotonically convergent exact integer kernel.
        // Correctness does not depend on the fixed iteration count above.
        isqrt_u128(scaled)
    }
}

/// Widening 128×128 → 256-bit multiply. Internal — used by fp_sqrt overflow path.
pub(crate) fn wide_mul_u128(a: u128, b: u128) -> (u128, u128) {
    let product = U256::mul_u128(a, b);
    let high = u128::from(product.limbs[2]) | (u128::from(product.limbs[3]) << 64);
    (high, product.low_u128())
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
        reduced = reduced / 4 + (reduced % 4 + 2) / 4;
        scale_back = scale_back.checked_mul(2).ok_or(SolMathError::Overflow)?;
    }

    let approx = sqrt_scaled_newton(reduced * SCALE)
        .checked_mul(scale_back)
        .ok_or(SolMathError::Overflow)?;

    let mut low = approx
        .checked_sub(scale_back)
        .ok_or(SolMathError::Overflow)?;
    while cmp_sqrt_candidate(low, x).is_gt() {
        if low == 0 {
            break;
        }
        low = low.checked_sub(scale_back).ok_or(SolMathError::Overflow)?;
    }

    let mut high = approx
        .checked_add(scale_back)
        .ok_or(SolMathError::Overflow)?;
    while !cmp_sqrt_candidate(high, x).is_gt() {
        low = high;
        high = high.checked_add(scale_back).ok_or(SolMathError::Overflow)?;
    }

    while low + 1 < high {
        let mid = low + (high - low) / 2;
        (low, high) = bisect_sqrt_bracket(low, high, !cmp_sqrt_candidate(mid, x).is_gt());
    }

    Ok(low)
}

/// Unsigned fixed-point multiply with rounding: round((a × b) / SCALE).
/// Error: ≤ 0.5 ULP. Returns `Err(Overflow)` on overflow.
#[inline]
pub fn fp_mul_round(a: u128, b: u128) -> Result<u128, SolMathError> {
    match checked_mul_div_rem_u(a, b, SCALE) {
        Some((q, r)) => round_unsigned_quotient(q, r, SCALE).ok_or(SolMathError::Overflow),
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

    let (q_trunc, r_trunc) = checked_mul_div_rem_u(aa, bb, SCALE).ok_or(SolMathError::Overflow)?;
    // q_trunc = floor(|a*b| / SCALE), r_trunc in [0, SCALE)

    let half = SCALE / 2;
    let (q, lo) = if r_trunc >= half {
        (
            q_trunc.checked_add(1).ok_or(SolMathError::Overflow)?,
            r_trunc as i128 - SCALE_I,
        ) // lo in (-SCALE/2, 0]
    } else {
        (q_trunc, r_trunc as i128) // lo in [0, SCALE/2)
    };

    if neg {
        if q == (1u128 << 127) {
            let signed_lo = -lo;
            if signed_lo < 0 {
                return Err(SolMathError::Overflow);
            }
            Ok(DoubleWord::new_raw(i128::MIN, signed_lo))
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
    // `r >= b - r` is equivalent to `2*r >= b` without overflow and is
    // correct for odd divisors. `r >= b/2` rounds values just below one half
    // upward when b is odd.
    round_unsigned_quotient(q, r, b).ok_or(SolMathError::Overflow)
}

#[cfg(kani)]
mod verification {
    use super::*;

    /// Prove the production Newton averaging transition preserves the floor
    /// lower bound and cannot satisfy the exit condition above the floor. The
    /// assumptions are the exact quotient inequalities derived from
    /// `floor_root² <= n < (floor_root+1)²` and Rust's `/` semantics.
    #[kani::proof]
    fn sqrt_newton_transition_preserves_and_detects_the_floor() {
        let candidate: u128 = kani::any();
        let quotient: u128 = kani::any();
        let floor_root: u128 = kani::any();
        kani::assume(floor_root <= u64::MAX as u128);
        kani::assume(candidate >= floor_root);
        kani::assume(candidate.checked_add(quotient).is_some());
        kani::assume(candidate + quotient >= 2 * floor_root);
        kani::assume(candidate == floor_root || quotient < candidate);

        let next = average_sqrt_newton_candidate(candidate, quotient);

        assert!(next >= floor_root);
        if next >= candidate {
            assert_eq!(candidate, floor_root);
        }
    }

    /// Prove one production bisection transition preserves a bracket around
    /// the mathematical floor root and strictly shrinks it. Exact integer
    /// multiplication makes `midpoint² <= n` equivalent to
    /// `midpoint <= floor(sqrt(n))`; induction from `[0, 2^64)` therefore
    /// proves the fallback's exact floor postcondition.
    #[kani::proof]
    fn sqrt_bisection_preserves_the_exact_floor_bracket() {
        const LIMIT: u128 = 1u128 << 64;

        let low: u128 = kani::any();
        let high: u128 = kani::any();
        let floor_root: u128 = kani::any();
        kani::assume(low <= floor_root);
        kani::assume(floor_root < high);
        kani::assume(high <= LIMIT);
        kani::assume(low + 1 < high);

        let previous_width = high - low;
        let midpoint = low + previous_width / 2;
        let (next_low, next_high) = bisect_sqrt_bracket(low, high, midpoint <= floor_root);

        assert!(next_low <= floor_root);
        assert!(floor_root < next_high);
        assert!(next_low < next_high);
        assert!(next_high - next_low < previous_width);
    }

    /// Prove truncating division by the standard fixed-point scale produces
    /// an exact quotient/remainder decomposition whose residual is strictly
    /// smaller than one output ULP.
    #[kani::proof]
    fn signed_scale_remainder_is_sub_ulp() {
        let product: i128 = kani::any();
        let remainder = product % SCALE_I;

        assert!(remainder.unsigned_abs() < SCALE);
    }

    /// Prove the production signed quotient/remainder rounding primitive is
    /// nearest with ties away from zero. Successful results have at most
    /// one-half ULP error; errors occur exactly when the required one-unit
    /// correction is outside `i128`.
    #[kani::proof]
    fn signed_quotient_rounding_is_half_ulp_or_overflow() {
        let quotient: i128 = kani::any();
        let remainder: i128 = kani::any();
        let divisor: u128 = kani::any();
        kani::assume(divisor > 0);
        kani::assume(remainder.unsigned_abs() < divisor);

        let remainder_magnitude = remainder.unsigned_abs();
        let round_away = remainder_at_least_half(remainder_magnitude, divisor);
        let expected = if !round_away {
            Some(quotient)
        } else if remainder >= 0 {
            quotient.checked_add(1)
        } else {
            quotient.checked_sub(1)
        };
        let actual = round_signed_quotient(quotient, remainder, divisor);

        if let Some(expected) = expected {
            assert_eq!(actual, Ok(expected));
            let error_numerator = if round_away {
                divisor - remainder_magnitude
            } else {
                remainder_magnitude
            };
            assert!(error_numerator <= divisor - error_numerator);
        } else {
            assert_eq!(actual, Err(SolMathError::Overflow));
        }
    }

    /// Prove the production unsigned quotient/remainder rounding primitive is
    /// nearest with ties up. Successful results have at most one-half ULP
    /// error; overflow is reported exactly for `MAX + 1`.
    #[kani::proof]
    fn unsigned_quotient_rounding_is_half_ulp_or_overflow() {
        let quotient: u128 = kani::any();
        let remainder: u128 = kani::any();
        let divisor: u128 = kani::any();
        kani::assume(divisor > 0);
        kani::assume(remainder < divisor);

        let round_up = remainder_at_least_half(remainder, divisor);
        let expected = if round_up {
            quotient.checked_add(1)
        } else {
            Some(quotient)
        };
        let actual = round_unsigned_quotient(quotient, remainder, divisor);

        assert_eq!(actual, expected);
        if expected.is_some() {
            let error_numerator = if round_up {
                divisor - remainder
            } else {
                remainder
            };
            // `e <= divisor - e` is the overflow-free form of `2*e <= divisor`.
            assert!(error_numerator <= divisor - error_numerator);
        }
    }

    /// Bit-precise proof over every `(remainder, divisor)` pair satisfying the
    /// division invariant. This covers the cases where `2 * remainder` would
    /// overflow as well as odd divisors and exact ties.
    #[kani::proof]
    fn remainder_half_comparison_is_exact_and_overflow_free() {
        let remainder: u128 = kani::any();
        let divisor: u128 = kani::any();
        kani::assume(divisor > 0);
        kani::assume(remainder < divisor);

        let actual = remainder_at_least_half(remainder, divisor);
        match remainder.checked_mul(2) {
            Some(twice) => assert_eq!(actual, twice >= divisor),
            None => {
                // If doubling overflows u128 then remainder > MAX/2 while
                // divisor <= MAX, so the exact rational remainder is > 1/2.
                assert!(actual);
            }
        }
    }

    /// Prove the shared European-price constructor enforces hard call/put
    /// bounds and exact put-call parity for its complete accepted domain.
    #[cfg(feature = "transcendental")]
    #[kani::proof]
    fn european_prices_preserve_bounds_and_parity() {
        let call_i: i128 = kani::any();
        let spot: u128 = kani::any();
        let discounted_strike_i: i128 = kani::any();
        kani::assume(spot <= i128::MAX as u128);
        kani::assume(discounted_strike_i >= 0);

        let discounted_strike = discounted_strike_i as u128;
        let (call, put) = european_prices_from_call(call_i, spot, discounted_strike_i)
            .expect("the bounded domain must be representable");

        assert!(call <= spot);
        assert!(call >= spot.saturating_sub(discounted_strike));
        assert!(put <= discounted_strike);
        assert_eq!(
            call.checked_add(discounted_strike).unwrap(),
            put.checked_add(spot).unwrap(),
        );
    }
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
            (SCALE / 3, SCALE),
            (SCALE, SCALE / 3),
            (SCALE / 7, SCALE / 11),
            (2 * SCALE, SCALE / 3),
            (SCALE + 1, SCALE + 1),
            (SCALE * 1000, SCALE / 997),
        ];
        for &(a, b) in cases {
            let trunc = fp_mul(a, b).unwrap();
            let round = fp_mul_round(a, b).unwrap();
            assert!(
                round == trunc || round == trunc + 1,
                "a={}, b={}: trunc={}, round={}",
                a,
                b,
                trunc,
                round
            );
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
        assert!(matches!(
            fp_mul_round(u128::MAX, u128::MAX),
            Err(SolMathError::Overflow)
        ));
    }

    #[test]
    fn test_fp_mul_i_round_near_extrema_uses_the_actual_remainder() {
        let positive = i128::MAX - 300_000_000_000;
        let negative = i128::MIN + 300_000_000_000;
        let expected = 170_141_183_460_469_231_731_687_303i128;
        assert_eq!(fp_mul_i_round(positive, 1).unwrap(), expected);
        assert_eq!(fp_mul_i_round(negative, 1).unwrap(), -expected);
    }

    // ===== fp_mul_i_round_dw tests =====

    #[test]
    fn test_dw_mul_quotient_matches_fp_mul_i_round() {
        let values: &[i128] = &[
            0,
            1,
            -1,
            2,
            -2,
            SCALE_I / 2,
            -SCALE_I / 2,
            SCALE_I,
            -SCALE_I,
            SCALE_I + 1,
            -(SCALE_I + 1),
            SCALE_I - 1,
            -(SCALE_I - 1),
            SCALE_I * 2,
            -(SCALE_I * 2),
            SCALE_I * 50,
            -(SCALE_I * 50),
            SCALE_I / 3,
            -(SCALE_I / 3),
            SCALE_I / 7,
            -(SCALE_I / 7),
            999_999_999_999,
            -999_999_999_999,
            500_000_000_001,
            -500_000_000_001,
        ];
        for &a in values {
            for &b in values {
                if a.checked_mul(b).is_some() {
                    let expected = fp_mul_i_round(a, b).unwrap();
                    let dw = fp_mul_i_round_dw(a, b).unwrap();
                    assert_eq!(
                        dw.hi(),
                        expected,
                        "Quotient mismatch: a={}, b={}, expected={}, got={}",
                        a,
                        b,
                        expected,
                        dw.hi()
                    );
                }
            }
        }
    }

    #[test]
    fn test_dw_mul_remainder_bounded() {
        let values: &[i128] = &[
            0,
            1,
            -1,
            SCALE_I,
            -SCALE_I,
            SCALE_I / 3,
            -SCALE_I / 7,
            SCALE_I * 50,
            -SCALE_I * 50,
            999_999_999_999,
        ];
        for &a in values {
            for &b in values {
                let dw = fp_mul_i_round_dw(a, b).unwrap();
                assert!(
                    dw.lo().abs() < SCALE_I,
                    "Remainder out of bounds: a={}, b={}, lo={}",
                    a,
                    b,
                    dw.lo()
                );
            }
        }
    }

    #[test]
    fn test_dw_mul_remainder_sign_convention() {
        let dw = fp_mul_i_round_dw(SCALE_I / 3, SCALE_I).unwrap();
        assert!(
            dw.lo() >= 0,
            "Expected non-negative lo for 1/3: lo={}",
            dw.lo()
        );

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
    fn test_dw_mul_allows_rounded_i128_min_with_positive_residual() {
        let a = -1_000_000_000_001i128;
        let b = 170_141_183_460_299_090_548_227_004_625_335_878_723i128;
        let dw = fp_mul_i_round_dw(a, b).unwrap();
        assert_eq!(dw.hi(), i128::MIN);
        assert_eq!(dw.lo(), 374_664_121_277);
        assert_eq!(fp_mul_i_round_dw(-a, b), Err(SolMathError::Overflow));
    }

    #[test]
    fn test_dw_mul_to_i128_matches_fp_mul_i_round() {
        let values: &[i128] = &[
            SCALE_I / 3,
            SCALE_I / 7,
            SCALE_I * 2,
            -SCALE_I / 3,
            -SCALE_I * 5,
        ];
        for &a in values {
            for &b in values {
                if a.checked_mul(b).is_some() {
                    let direct = fp_mul_i_round(a, b).unwrap();
                    let via_dw = fp_mul_i_round_dw(a, b).unwrap().to_i128();
                    assert_eq!(direct, via_dw, "Roundtrip mismatch: a={}, b={}", a, b);
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
    fn test_fp_div_round_odd_divisor_below_half_does_not_round_up() {
        // SCALE / 3 = 333_333_333_333 + 1/3. The old `r >= b / 2`
        // comparison treated 1/3 as at least one half because integer b/2 is
        // 1 for b=3.
        assert_eq!(fp_div_round(1, 3).unwrap(), SCALE / 3);
        assert_eq!(fp_div_i_round(1, 3).unwrap(), SCALE_I / 3);
        assert_eq!(fp_div_i_round(-1, 3).unwrap(), -(SCALE_I / 3));
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
            (SCALE, 3 * SCALE),
            (2 * SCALE, 3 * SCALE),
            (7 * SCALE, 11 * SCALE),
            (SCALE, 7 * SCALE),
            (100 * SCALE, 97 * SCALE),
        ];
        for &(a, b) in cases {
            let unsigned_r = fp_div_round(a, b).unwrap();
            let signed_r = fp_div_i_round(a as i128, b as i128).unwrap();
            assert_eq!(
                unsigned_r as i128, signed_r,
                "Disagree for a={}, b={}",
                a, b
            );
        }
    }

    #[test]
    fn test_fp_div_round_within_one_of_truncating() {
        let values: &[u128] = &[1, 2, 3, 7, 10, 13, 100, 997, SCALE, SCALE + 1, SCALE * 1000];
        for &a in values {
            for &b in values {
                let t = fp_div(a, b).unwrap();
                let r = fp_div_round(a, b).unwrap();
                assert!(
                    r == t || r == t + 1,
                    "a={}, b={}: trunc={}, round={}",
                    a,
                    b,
                    t,
                    r
                );
            }
        }
    }

    #[test]
    fn test_fp_div_round_zero_divisor() {
        assert!(matches!(
            fp_div_round(SCALE, 0),
            Err(SolMathError::DivisionByZero)
        ));
    }

    #[test]
    fn test_fp_div_round_large_no_panic() {
        let large = u128::MAX / (SCALE * 2);
        assert!(fp_div_round(large, SCALE).is_ok());
        assert!(fp_div_round(SCALE, large).is_ok());
    }

    #[test]
    fn test_fp_sqrt_maximum_input() {
        assert_eq!(
            fp_sqrt(u128::MAX).unwrap(),
            18_446_744_073_709_551_615_999_999
        );
        assert!(fp_sqrt(u128::MAX - 1).unwrap() <= fp_sqrt(u128::MAX).unwrap());
    }
}
