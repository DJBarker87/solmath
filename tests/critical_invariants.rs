//! Deterministic assurance checks for the value-sensitive public API.
//!
//! These tests intentionally use no external crates, network services, random
//! seeds, or floating-point reference calculations.  The arithmetic oracle is
//! a test-local four-limb model implemented separately from the production
//! arithmetic. Financial tests check structural invariants rather than
//! claiming an external or independently qualified pricing implementation.
//!
//! Residual limits used below:
//! - integer/fixed-point rounding and overflow: exact (`0` raw units);
//! - weighted-pool payout: returned gross output must be no greater than a
//!   rigorously rounded-down lower bound on the real integer-exponent output;
//!   over the tested domain that lower bound is less than `1.0000004` raw
//!   payout units below the real-valued result;
//! - Black-Scholes/Heston/SABR put-call parity: exact against the discounted
//!   strike used by the corresponding fixed-point path;
//! - barrier knock-in + knock-out conservation: exact (`0` raw units);
//! - SABR coarse-grid convexity: `1_000` raw units, matching the executable
//!   quote guard's documented rounding allowance.

use solmath::{
    checked_mul_div_ceil_i, checked_mul_div_floor_i, checked_mul_div_i, fp_div, fp_div_i,
    fp_div_round, fp_mul, fp_mul_i, fp_mul_i_round, fp_mul_i_round_dw, fp_mul_round, fp_sqrt,
    mul_div_ceil, mul_div_ceil_u128, mul_div_floor, mul_div_floor_u128, SolMathError, SCALE,
    SCALE_I,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RefU256([u64; 4]);

impl RefU256 {
    const fn zero() -> Self {
        Self([0; 4])
    }

    const fn from_u128(value: u128) -> Self {
        Self([value as u64, (value >> 64) as u64, 0, 0])
    }

    fn bit(self, index: usize) -> bool {
        ((self.0[index / 64] >> (index % 64)) & 1) != 0
    }

    fn set_bit(&mut self, index: usize) {
        self.0[index / 64] |= 1u64 << (index % 64);
    }

    fn shl1(&mut self) {
        let mut carry = 0u64;
        for limb in &mut self.0 {
            let next = *limb >> 63;
            *limb = (*limb << 1) | carry;
            carry = next;
        }
    }

    fn add_assign(&mut self, rhs: Self) {
        let mut carry = false;
        for index in 0..4 {
            let (sum, carry_a) = self.0[index].overflowing_add(rhs.0[index]);
            let (sum, carry_b) = sum.overflowing_add(u64::from(carry));
            self.0[index] = sum;
            carry = carry_a || carry_b;
        }
        assert!(!carry, "reference U256 addition overflowed");
    }

    fn ge(self, rhs: Self) -> bool {
        for index in (0..4).rev() {
            if self.0[index] != rhs.0[index] {
                return self.0[index] > rhs.0[index];
            }
        }
        true
    }

    fn sub_assign(&mut self, rhs: Self) {
        assert!(self.ge(rhs));
        let mut borrow = false;
        for index in 0..4 {
            let (difference, borrow_a) = self.0[index].overflowing_sub(rhs.0[index]);
            let (difference, borrow_b) = difference.overflowing_sub(u64::from(borrow));
            self.0[index] = difference;
            borrow = borrow_a || borrow_b;
        }
        assert!(!borrow);
    }

    fn low_u128(self) -> u128 {
        self.0[0] as u128 | ((self.0[1] as u128) << 64)
    }

    fn high_u128_nonzero(self) -> bool {
        self.0[2] != 0 || self.0[3] != 0
    }

    fn mul_u128(a: u128, b: u128) -> Self {
        let mut product = Self::zero();
        let mut shifted = Self::from_u128(a);
        let mut multiplier = b;
        while multiplier != 0 {
            if multiplier & 1 != 0 {
                product.add_assign(shifted);
            }
            multiplier >>= 1;
            if multiplier != 0 {
                shifted.shl1();
            }
        }
        product
    }

    fn div_rem_u128(self, divisor: u128) -> (Self, u128) {
        assert_ne!(divisor, 0);
        let divisor = Self::from_u128(divisor);
        let mut quotient = Self::zero();
        let mut remainder = Self::zero();
        for bit in (0..256).rev() {
            remainder.shl1();
            if self.bit(bit) {
                remainder.0[0] |= 1;
            }
            if remainder.ge(divisor) {
                remainder.sub_assign(divisor);
                quotient.set_bit(bit);
            }
        }
        assert!(!remainder.high_u128_nonzero());
        (quotient, remainder.low_u128())
    }
}

#[derive(Clone, Copy)]
enum UnsignedRounding {
    Floor,
    Ceil,
    NearestHalfUp,
}

#[derive(Clone, Copy)]
enum SignedRounding {
    ToZero,
    Floor,
    Ceil,
    NearestAway,
}

fn ref_unsigned_mul_div(
    a: u128,
    b: u128,
    divisor: u128,
    rounding: UnsignedRounding,
) -> Result<u128, SolMathError> {
    if divisor == 0 {
        return Err(SolMathError::DivisionByZero);
    }
    let (quotient, remainder) = RefU256::mul_u128(a, b).div_rem_u128(divisor);
    if quotient.high_u128_nonzero() {
        return Err(SolMathError::Overflow);
    }
    let mut quotient = quotient.low_u128();
    let increment = match rounding {
        UnsignedRounding::Floor => false,
        UnsignedRounding::Ceil => remainder != 0,
        UnsignedRounding::NearestHalfUp => remainder >= divisor - remainder,
    };
    if increment {
        quotient = quotient.checked_add(1).ok_or(SolMathError::Overflow)?;
    }
    Ok(quotient)
}

fn signed_from_magnitude(magnitude: u128, negative: bool) -> Result<i128, SolMathError> {
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

fn ref_signed_mul_div(
    a: i128,
    b: i128,
    divisor: i128,
    rounding: SignedRounding,
) -> Result<i128, SolMathError> {
    if divisor == 0 {
        return Err(SolMathError::DivisionByZero);
    }
    let negative = (a < 0) ^ (b < 0) ^ (divisor < 0);
    let unsigned_divisor = divisor.unsigned_abs();
    let (quotient, remainder) =
        RefU256::mul_u128(a.unsigned_abs(), b.unsigned_abs()).div_rem_u128(unsigned_divisor);
    if quotient.high_u128_nonzero() {
        return Err(SolMathError::Overflow);
    }
    let mut magnitude = quotient.low_u128();
    let increment = if remainder == 0 {
        false
    } else {
        match rounding {
            SignedRounding::ToZero => false,
            SignedRounding::Floor => negative,
            SignedRounding::Ceil => !negative,
            SignedRounding::NearestAway => remainder >= unsigned_divisor - remainder,
        }
    };
    if increment {
        magnitude = magnitude.checked_add(1).ok_or(SolMathError::Overflow)?;
    }
    signed_from_magnitude(magnitude, negative)
}

fn ref_double_word(a: i128, b: i128) -> Result<(i128, i128), SolMathError> {
    let negative = (a < 0) ^ (b < 0);
    let (quotient, remainder) =
        RefU256::mul_u128(a.unsigned_abs(), b.unsigned_abs()).div_rem_u128(SCALE);
    if quotient.high_u128_nonzero() {
        return Err(SolMathError::Overflow);
    }
    let mut magnitude = quotient.low_u128();
    let rounded_up = remainder >= SCALE / 2;
    let residual_magnitude = if rounded_up {
        magnitude = magnitude.checked_add(1).ok_or(SolMathError::Overflow)?;
        remainder as i128 - SCALE_I
    } else {
        remainder as i128
    };
    let high = signed_from_magnitude(magnitude, negative)?;
    let low = if negative {
        residual_magnitude
            .checked_neg()
            .ok_or(SolMathError::Overflow)?
    } else {
        residual_magnitude
    };
    if high == i128::MIN && low < 0 {
        return Err(SolMathError::Overflow);
    }
    Ok((high, low))
}

fn unsigned_edges() -> [u128; 16] {
    [
        0,
        1,
        2,
        SCALE / 2 - 1,
        SCALE / 2,
        SCALE / 2 + 1,
        SCALE - 1,
        SCALE,
        SCALE + 1,
        2 * SCALE,
        1u128 << 63,
        1u128 << 64,
        u128::MAX / SCALE,
        u128::MAX / 2,
        u128::MAX - 1,
        u128::MAX,
    ]
}

fn signed_edges() -> [i128; 17] {
    [
        i128::MIN,
        i128::MIN + 1,
        -2 * SCALE_I,
        -SCALE_I - 1,
        -SCALE_I,
        -SCALE_I / 2,
        -2,
        -1,
        0,
        1,
        2,
        SCALE_I / 2,
        SCALE_I,
        SCALE_I + 1,
        2 * SCALE_I,
        i128::MAX - 1,
        i128::MAX,
    ]
}

#[test]
fn reference_u256_model_self_checks_against_native_arithmetic() {
    for a in 0u128..64 {
        for b in 0u128..64 {
            for divisor in 1u128..32 {
                let product = a * b;
                let (quotient, remainder) = RefU256::mul_u128(a, b).div_rem_u128(divisor);
                assert!(!quotient.high_u128_nonzero());
                assert_eq!(quotient.low_u128(), product / divisor);
                assert_eq!(remainder, product % divisor);
            }
        }
    }
    let (quotient, remainder) = RefU256::mul_u128(u128::MAX, u128::MAX).div_rem_u128(u128::MAX);
    assert_eq!(quotient.low_u128(), u128::MAX);
    assert!(!quotient.high_u128_nonzero());
    assert_eq!(remainder, 0);
}

fn assert_exact_fixed_point_sqrt_floor(x: u128) {
    let result = fp_sqrt(x).unwrap();
    let radicand = RefU256::mul_u128(x, SCALE);
    let square = RefU256::mul_u128(result, result);
    let successor = result.checked_add(1).unwrap();
    let successor_square = RefU256::mul_u128(successor, successor);
    assert!(
        radicand.ge(square),
        "fp_sqrt({x})={result} is above the exact floor"
    );
    assert!(
        !radicand.ge(successor_square),
        "fp_sqrt({x})={result} is below the exact floor"
    );
}

#[test]
fn fixed_point_sqrt_is_the_exact_floor_across_full_width() {
    for x in 0..=u16::MAX as u128 {
        assert_exact_fixed_point_sqrt_floor(x);
    }

    // Deterministic, evenly spaced coverage across the complete u128 domain,
    // including the path where `x * SCALE` requires 256 bits.
    let step = u128::MAX / 4_096;
    for index in 0u128..=4_096 {
        assert_exact_fixed_point_sqrt_floor(step * index);
    }
    for x in unsigned_edges() {
        assert_exact_fixed_point_sqrt_floor(x);
    }
}

#[test]
fn fixed_point_rounding_matches_exact_256_bit_model() {
    let unsigned = unsigned_edges();
    for &a in &unsigned {
        for &b in &unsigned {
            assert_eq!(
                fp_mul(a, b),
                ref_unsigned_mul_div(a, b, SCALE, UnsignedRounding::Floor),
                "fp_mul({a}, {b})"
            );
            assert_eq!(
                fp_mul_round(a, b),
                ref_unsigned_mul_div(a, b, SCALE, UnsignedRounding::NearestHalfUp),
                "fp_mul_round({a}, {b})"
            );
            assert_eq!(
                fp_div(a, b),
                ref_unsigned_mul_div(a, SCALE, b, UnsignedRounding::Floor),
                "fp_div({a}, {b})"
            );
            assert_eq!(
                fp_div_round(a, b),
                ref_unsigned_mul_div(a, SCALE, b, UnsignedRounding::NearestHalfUp),
                "fp_div_round({a}, {b})"
            );
        }
    }

    let signed = signed_edges();
    for &a in &signed {
        for &b in &signed {
            assert_eq!(
                fp_mul_i(a, b),
                ref_signed_mul_div(a, b, SCALE_I, SignedRounding::ToZero),
                "fp_mul_i({a}, {b})"
            );
            assert_eq!(
                fp_mul_i_round(a, b),
                ref_signed_mul_div(a, b, SCALE_I, SignedRounding::NearestAway),
                "fp_mul_i_round({a}, {b})"
            );
            assert_eq!(
                fp_div_i(a, b),
                ref_signed_mul_div(a, SCALE_I, b, SignedRounding::ToZero),
                "fp_div_i({a}, {b})"
            );
            let expected_dw = ref_double_word(a, b);
            let actual_dw = fp_mul_i_round_dw(a, b).map(|value| (value.hi(), value.lo()));
            assert_eq!(actual_dw, expected_dw, "fp_mul_i_round_dw({a}, {b})");
        }
    }
}

#[test]
fn raw_mul_div_and_signed_rounding_match_exact_256_bit_model() {
    let values = unsigned_edges();
    let divisors = [
        0,
        1,
        2,
        3,
        SCALE - 1,
        SCALE,
        SCALE + 1,
        1u128 << 64,
        u128::MAX,
    ];
    for &a in &values {
        for &b in &values {
            for &divisor in &divisors {
                assert_eq!(
                    mul_div_floor_u128(a, b, divisor),
                    ref_unsigned_mul_div(a, b, divisor, UnsignedRounding::Floor),
                    "mul_div_floor_u128({a}, {b}, {divisor})"
                );
                assert_eq!(
                    mul_div_ceil_u128(a, b, divisor),
                    ref_unsigned_mul_div(a, b, divisor, UnsignedRounding::Ceil),
                    "mul_div_ceil_u128({a}, {b}, {divisor})"
                );
            }
        }
    }

    let signed = signed_edges();
    let signed_divisors = [
        i128::MIN,
        -SCALE_I,
        -3,
        -2,
        -1,
        0,
        1,
        2,
        3,
        SCALE_I,
        i128::MAX,
    ];
    for &a in &signed {
        for &b in &signed {
            for &divisor in &signed_divisors {
                assert_eq!(
                    checked_mul_div_i(a, b, divisor),
                    ref_signed_mul_div(a, b, divisor, SignedRounding::ToZero),
                    "checked_mul_div_i({a}, {b}, {divisor})"
                );
                assert_eq!(
                    checked_mul_div_floor_i(a, b, divisor),
                    ref_signed_mul_div(a, b, divisor, SignedRounding::Floor),
                    "checked_mul_div_floor_i({a}, {b}, {divisor})"
                );
                assert_eq!(
                    checked_mul_div_ceil_i(a, b, divisor),
                    ref_signed_mul_div(a, b, divisor, SignedRounding::Ceil),
                    "checked_mul_div_ceil_i({a}, {b}, {divisor})"
                );
            }
        }
    }
}

#[test]
fn u64_mul_div_matches_native_widened_model() {
    let values = [
        0,
        1,
        2,
        3,
        u32::MAX as u64,
        1u64 << 32,
        u64::MAX - 1,
        u64::MAX,
    ];
    let divisors = [0, 1, 2, 3, u32::MAX as u64, 1u64 << 32, u64::MAX];
    for &a in &values {
        for &b in &values {
            for &divisor in &divisors {
                let expected_floor = if divisor == 0 {
                    Err(SolMathError::DivisionByZero)
                } else {
                    let quotient = (a as u128 * b as u128) / divisor as u128;
                    u64::try_from(quotient).map_err(|_| SolMathError::Overflow)
                };
                let expected_ceil = if divisor == 0 {
                    Err(SolMathError::DivisionByZero)
                } else {
                    let product = a as u128 * b as u128;
                    let quotient =
                        product / divisor as u128 + u128::from(product % divisor as u128 != 0);
                    u64::try_from(quotient).map_err(|_| SolMathError::Overflow)
                };
                assert_eq!(mul_div_floor(a, b, divisor), expected_floor);
                assert_eq!(mul_div_ceil(a, b, divisor), expected_ceil);
            }
        }
    }
}

#[test]
fn exact_half_ties_and_signed_extrema_are_explicit() {
    assert_eq!(fp_mul_round(1, SCALE / 2), Ok(1));
    assert_eq!(fp_mul_i_round(1, SCALE_I / 2), Ok(1));
    assert_eq!(fp_mul_i_round(-1, SCALE_I / 2), Ok(-1));
    assert_eq!(fp_div_round(1, 2 * SCALE), Ok(1));
    assert_eq!(fp_div_round(1, 3), Ok(SCALE / 3));

    assert_eq!(checked_mul_div_i(i128::MIN, 1, 1), Ok(i128::MIN));
    assert_eq!(
        checked_mul_div_i(i128::MIN, -1, 1),
        Err(SolMathError::Overflow)
    );
    assert_eq!(checked_mul_div_i(i128::MIN, -1, -1), Ok(i128::MIN));
    assert_eq!(checked_mul_div_floor_i(-1, 1, 2), Ok(-1));
    assert_eq!(checked_mul_div_ceil_i(-1, 1, 2), Ok(0));
    assert_eq!(checked_mul_div_floor_i(1, 1, -2), Ok(-1));
    assert_eq!(checked_mul_div_ceil_i(1, 1, -2), Ok(0));
}

#[cfg(feature = "pool")]
mod pool_invariants {
    use super::*;
    use solmath::weighted_pool_swap;

    const Q24: u128 = 1_000_000_000_000_000_000_000_000;

    fn ceil_mul_div(a: u128, b: u128, divisor: u128) -> u128 {
        ref_unsigned_mul_div(a, b, divisor, UnsignedRounding::Ceil).unwrap()
    }

    fn conservative_integer_power_gross_lower_bound(
        balance_in: u128,
        balance_out: u128,
        amount_in: u128,
        exponent: u128,
    ) -> u128 {
        let denominator = balance_in + amount_in;
        let ratio_high = ceil_mul_div(balance_in, Q24, denominator);
        let mut power_high = Q24;
        for _ in 0..exponent {
            power_high = ceil_mul_div(power_high, ratio_high, Q24).min(Q24);
        }
        ref_unsigned_mul_div(balance_out, Q24 - power_high, Q24, UnsignedRounding::Floor).unwrap()
    }

    #[test]
    fn pool_payout_is_protocol_favouring_on_integer_weight_ratios() {
        let balances_in = [SCALE, 10 * SCALE, 1_000 * SCALE];
        let balances_out = [SCALE, 7 * SCALE, 10_000 * SCALE];
        let exponents = [1u128, 2, 3, 5, 10, 20];
        let fee_rates = [0, 1, SCALE / 1_000, SCALE / 2, SCALE];

        for &balance_in in &balances_in {
            let amounts = [
                1,
                SCALE / 1_000,
                SCALE / 10,
                SCALE,
                10 * SCALE,
                99 * balance_in,
            ];
            for &balance_out in &balances_out {
                for &amount_in in &amounts {
                    for &exponent in &exponents {
                        for &fee_rate in &fee_rates {
                            let (net_out, fee) = weighted_pool_swap(
                                balance_in,
                                balance_out,
                                exponent * SCALE,
                                SCALE,
                                amount_in,
                                fee_rate,
                            )
                            .unwrap_or_else(|error| {
                                panic!(
                                    "certified pool input failed: bi={balance_in}, bo={balance_out}, \
                                     amount={amount_in}, exponent={exponent}, fee={fee_rate}: {error:?}"
                                )
                            });
                            let gross = net_out.checked_add(fee).unwrap();
                            let exact_lower = conservative_integer_power_gross_lower_bound(
                                balance_in,
                                balance_out,
                                amount_in,
                                exponent,
                            );
                            assert!(
                                gross <= exact_lower,
                                "trader-favouring payout: gross={gross}, exact_lower={exact_lower}, \
                                 bi={balance_in}, bo={balance_out}, amount={amount_in}, exponent={exponent}"
                            );
                            assert!(gross < balance_out, "pool reserve was drained");
                            let exact_fee = ref_unsigned_mul_div(
                                gross,
                                fee_rate,
                                SCALE,
                                UnsignedRounding::Ceil,
                            )
                            .unwrap();
                            assert_eq!(fee, exact_fee, "fee must round toward the protocol");
                            assert_eq!(net_out, gross - fee);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn pool_rejects_shapes_outside_the_certified_domain() {
        assert_eq!(
            weighted_pool_swap(SCALE, SCALE, SCALE, SCALE, 99 * SCALE + 1, 0),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            weighted_pool_swap(SCALE, SCALE, 20 * SCALE + 1, SCALE, SCALE, 0),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            weighted_pool_swap(u128::MAX, SCALE, SCALE, SCALE, 1, 0),
            Err(SolMathError::Overflow)
        );
        assert_eq!(
            weighted_pool_swap(SCALE, SCALE, 0, SCALE, SCALE, 0),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            weighted_pool_swap(SCALE, SCALE, SCALE, 0, SCALE, 0),
            Err(SolMathError::DivisionByZero)
        );
    }
}

#[cfg(feature = "bs")]
mod black_scholes_invariants {
    use super::*;
    use solmath::{
        black_scholes_price, black_scholes_price_hp, bs_full, bs_full_hp, exp_fixed_hp,
        exp_fixed_i, fp_mul_hp_i,
    };

    const HP_FACTOR: i128 = 1_000;

    fn standard_discounted_strike(k: u128, r: u128, t: u128) -> u128 {
        let rt = fp_mul_i(r as i128, t as i128).unwrap();
        fp_mul_i(k as i128, exp_fixed_i(-rt).unwrap()).unwrap() as u128
    }

    fn hp_discounted_strike(k: u128, r: u128, t: u128) -> u128 {
        let r_hp = r as i128 * HP_FACTOR;
        let t_hp = t as i128 * HP_FACTOR;
        let k_hp = k as i128 * HP_FACTOR;
        let rt_hp = fp_mul_hp_i(r_hp, t_hp).unwrap();
        let discount_hp = exp_fixed_hp(-rt_hp).unwrap();
        let strike_hp = fp_mul_hp_i(k_hp, discount_hp).unwrap();
        ((strike_hp + HP_FACTOR / 2) / HP_FACTOR) as u128
    }

    fn assert_bounds_and_parity(
        label: &str,
        call: u128,
        put: u128,
        s: u128,
        k: u128,
        discounted_strike: u128,
    ) {
        assert!(call <= s, "{label}: call exceeds spot");
        assert!(put <= k, "{label}: put exceeds undiscounted strike");
        assert!(
            call >= s.saturating_sub(discounted_strike),
            "{label}: call below lower bound"
        );
        assert!(
            put >= discounted_strike.saturating_sub(s),
            "{label}: put below lower bound"
        );
        let call_side = call.checked_add(discounted_strike).unwrap();
        let put_side = put.checked_add(s).unwrap();
        assert_eq!(
            call_side, put_side,
            "{label}: put-call parity residual must be zero"
        );
    }

    #[test]
    fn price_paths_obey_hard_bounds_and_exact_parity() {
        let spots = [SCALE, 25 * SCALE, 100 * SCALE, 400 * SCALE, 1_000 * SCALE];
        let strikes = [SCALE, 20 * SCALE, 100 * SCALE, 500 * SCALE, 1_000 * SCALE];
        let rates = [0, SCALE / 100, SCALE / 20, SCALE / 5];
        let sigmas = [SCALE / 100, SCALE / 10, SCALE / 2, 2 * SCALE];
        let times = [SCALE / 1_000, SCALE / 10, SCALE, 5 * SCALE];

        for &s in &spots {
            for &k in &strikes {
                for &r in &rates {
                    for &sigma in &sigmas {
                        for &t in &times {
                            let standard = black_scholes_price(s, k, r, sigma, t).unwrap();
                            let standard_full = bs_full(s, k, r, sigma, t).unwrap();
                            assert_eq!(standard, (standard_full.call, standard_full.put));
                            assert_bounds_and_parity(
                                "standard BS",
                                standard.0,
                                standard.1,
                                s,
                                k,
                                standard_discounted_strike(k, r, t),
                            );
                            assert!((0..=SCALE_I).contains(&standard_full.call_delta));
                            assert!((-SCALE_I..=0).contains(&standard_full.put_delta));
                            assert!(standard_full.gamma >= 0);
                            assert!(standard_full.vega >= 0);

                            let hp = black_scholes_price_hp(s, k, r, sigma, t).unwrap();
                            let hp_full = bs_full_hp(s, k, r, sigma, t).unwrap();
                            assert_eq!(hp, (hp_full.call, hp_full.put));
                            assert_bounds_and_parity(
                                "HP BS",
                                hp.0,
                                hp.1,
                                s,
                                k,
                                hp_discounted_strike(k, r, t),
                            );
                            assert!((0..=SCALE_I).contains(&hp_full.call_delta));
                            assert!((-SCALE_I..=0).contains(&hp_full.put_delta));
                            assert!(hp_full.gamma >= 0);
                            assert!(hp_full.vega >= 0);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn extreme_public_inputs_fail_closed_without_panicking() {
        use solmath::{bs_rho, bs_theta};

        let maximum = i128::MAX as u128;
        let cases = [
            (u128::MAX, SCALE, 0, SCALE / 5, SCALE),
            (SCALE, u128::MAX, 0, SCALE / 5, SCALE),
            (SCALE, SCALE, u128::MAX, SCALE / 5, SCALE),
            (SCALE, SCALE, 0, u128::MAX, SCALE),
            (SCALE, SCALE, 0, SCALE / 5, u128::MAX),
            (maximum, SCALE, 0, SCALE / 5, SCALE),
            (SCALE, maximum, 0, SCALE / 5, SCALE),
            (SCALE, SCALE, maximum, SCALE / 5, SCALE),
            (SCALE, SCALE, 0, maximum, SCALE),
            (SCALE, SCALE, 0, SCALE / 5, maximum),
            // Both spot AND strike at the i128 ceiling simultaneously: the
            // discounted-strike Greek terms (theta/rho) can each approach the
            // i128 limits, and their combination once overflowed. Regression for
            // the bs_full / bs_theta subtract-with-overflow panic.
            (maximum, maximum, SCALE, SCALE / 2, SCALE / 100),
            (maximum, maximum, SCALE, SCALE / 5, SCALE),
            (maximum, maximum, maximum, maximum, maximum),
        ];

        for &(s, k, r, sigma, t) in &cases {
            for (label, high_precision) in [("standard", false), ("high precision", true)] {
                let outcome = std::panic::catch_unwind(|| {
                    if high_precision {
                        black_scholes_price_hp(s, k, r, sigma, t)
                    } else {
                        black_scholes_price(s, k, r, sigma, t)
                    }
                });
                assert!(
                    outcome.is_ok(),
                    "{label} BS panicked for s={s}, k={k}, r={r}, sigma={sigma}, t={t}"
                );
                if let Ok((call, put)) = outcome.unwrap() {
                    assert!(call <= s);
                    assert!(put <= k);
                }
            }

            // The full-Greek surface (theta/rho especially) must fail closed too.
            for (label, outcome) in [
                (
                    "bs_full",
                    std::panic::catch_unwind(|| bs_full(s, k, r, sigma, t)).map(|_| ()),
                ),
                (
                    "bs_theta",
                    std::panic::catch_unwind(|| bs_theta(s, k, r, sigma, t)).map(|_| ()),
                ),
                (
                    "bs_rho",
                    std::panic::catch_unwind(|| bs_rho(s, k, r, sigma, t)).map(|_| ()),
                ),
            ] {
                assert!(
                    outcome.is_ok(),
                    "{label} panicked for s={s}, k={k}, r={r}, sigma={sigma}, t={t}"
                );
            }
        }
    }
}

/// No-panic guarantee across the realistic mid-range input space.
///
/// The prior `extreme_*` sweeps only probed input *boundaries* (a single field
/// at `i128::MAX`). The overflow panics fixed here lived in the *interior*:
/// `implied_vol` on a plausible deep-OTM long-dated quote, and `bs_theta` when
/// spot and strike are both large together. This deterministic fuzz walks the
/// realistic financial ranges (including the long maturities and lopsided
/// spot/strike ratios that trigger the degenerate solver bracket) and asserts
/// every public pricing entry point returns a `Result` rather than panicking.
#[cfg(feature = "iv")]
mod pricing_no_panic_fuzz {
    use super::*;
    use solmath::{black_scholes_price, bs_full, bs_rho, bs_theta, bs_vega, implied_vol};

    // xorshift64 — deterministic, dependency-free, reproducible in CI.
    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.0 = x;
            x
        }
        // A value biased toward realistic finance ranges, with occasional
        // extremes. The near-cap band (~1e17) is emphasised because that is
        // where near-ATM volga blows up and the IV Halley step overflowed.
        fn amount(&mut self) -> u128 {
            match self.next() % 16 {
                0 => 0,
                1 => 1,
                2 => SCALE,
                3 => 100_000 * SCALE, // the IV price cap
                4 => 100_001 * SCALE, // just over the cap
                5 => i128::MAX as u128,
                6 => SCALE / 100,                   // 0.01
                7 => (self.next() as u128) % SCALE, // fractional
                8 => 99_999 * SCALE,                // just inside the cap (near-ATM volga)
                9 => 99_998 * SCALE,
                10 => 100_000 * SCALE - 1 - (self.next() as u128) % SCALE,
                _ => SCALE * (1 + (self.next() as u128) % 200_000), // 1 .. 200k units
            }
        }
        fn rate(&mut self) -> u128 {
            (self.next() as u128) % (SCALE / 2) // 0 .. 50%
        }
        fn sigma(&mut self) -> u128 {
            1 + (self.next() as u128) % (5 * SCALE) // ~0 .. 500% vol
        }
        fn time(&mut self) -> u128 {
            // Bias toward both extremes: tiny maturities (where near-ATM volga
            // diverges) and long ones (where sigma*sqrt(T) grows). Both were
            // overflow triggers in the IV solver.
            match self.next() % 6 {
                0 => 1,
                1 => SCALE / 10_000,
                2 => 20 * SCALE,
                3 => 30 * SCALE,
                _ => 1 + (self.next() as u128) % (30 * SCALE),
            }
        }
    }

    #[test]
    fn implied_vol_degenerate_boundary_is_fail_closed_not_panic() {
        // Exact regression vector: deep-OTM (strike 100x spot), 20-year maturity,
        // near-zero rate. Formerly panicked via an unchecked square in
        // `normalised_vega` once the solver bracket ran to the i128::MAX/2
        // sentinel. Must now return an Err, not unwind.
        let outcome = std::panic::catch_unwind(|| {
            implied_vol(
                10_000_000_000,     // market price 0.01
                10_000_000_000,     // spot 0.01
                1_000_000_000_000,  // strike 1.0
                1,                  // rate ~0
                20_000_000_000_000, // t = 20 years
            )
        });
        assert!(
            outcome.is_ok(),
            "implied_vol panicked on the degenerate boundary vector"
        );
        assert!(
            matches!(
                outcome.unwrap(),
                Err(SolMathError::NoConvergence) | Err(SolMathError::Overflow)
            ),
            "degenerate implied_vol should fail closed"
        );
    }

    #[test]
    fn implied_vol_near_atm_huge_price_does_not_overflow_halley_step() {
        // Near-ATM (s ≈ k ≈ 1e17, the price cap), essentially-zero maturity:
        // volga diverges and the Halley step's `f * volga` product formerly
        // overflowed i128 in `halley_step_bracketed`. The routine must now fall
        // back to bisection and return a Result, never unwind. Under debug
        // assertions this also exercises the `mul_fast` precondition guard.
        let cases = [
            (99_999 * SCALE, 99_999 * SCALE, 100_000 * SCALE, 0, 1),
            (
                100_000 * SCALE,
                100_000 * SCALE,
                99_999 * SCALE,
                500_000_000_000,
                1,
            ),
            (99_999 * SCALE, 100_000 * SCALE, 99_999 * SCALE, 1, 1),
        ];
        for (mp, s, k, r, t) in cases {
            let outcome = std::panic::catch_unwind(|| implied_vol(mp, s, k, r, t));
            assert!(
                outcome.is_ok(),
                "implied_vol panicked for mp={mp} s={s} k={k} r={r} t={t}"
            );
        }
    }

    #[test]
    fn pricing_surface_never_panics_over_realistic_space() {
        let mut rng = Rng(0x5eed_1234_9e37_79b9);
        // 150k draws keeps the default test well under a second while covering
        // the interior space the boundary sweeps miss. CI's soak step raises
        // this via SOLMATH_FUZZ_ITERS for a much deeper sweep.
        let iters: u64 = std::env::var("SOLMATH_FUZZ_ITERS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(150_000);
        for _ in 0..iters {
            let s = rng.amount();
            let k = rng.amount();
            let r = rng.rate();
            let sigma = rng.sigma();
            let t = rng.time();
            let mp = rng.amount();

            macro_rules! no_panic {
                ($label:literal, $call:expr) => {{
                    let outcome = std::panic::catch_unwind(|| $call);
                    assert!(
                        outcome.is_ok(),
                        concat!($label, " panicked for s={} k={} r={} sigma={} t={} mp={}"),
                        s,
                        k,
                        r,
                        sigma,
                        t,
                        mp
                    );
                }};
            }

            no_panic!(
                "black_scholes_price",
                black_scholes_price(s, k, r, sigma, t)
            );
            no_panic!("bs_full", bs_full(s, k, r, sigma, t));
            no_panic!("bs_theta", bs_theta(s, k, r, sigma, t));
            no_panic!("bs_rho", bs_rho(s, k, r, sigma, t));
            no_panic!("bs_vega", bs_vega(s, k, r, sigma, t));
            no_panic!("implied_vol", implied_vol(mp, s, k, r, t));
        }
    }
}

#[cfg(feature = "barrier")]
mod barrier_invariants {
    use super::*;
    use solmath::{barrier_option, barrier_option_with_state, BarrierType};

    fn pair(kind_out: BarrierType) -> BarrierType {
        match kind_out {
            BarrierType::DownAndOut => BarrierType::DownAndIn,
            BarrierType::UpAndOut => BarrierType::UpAndIn,
            _ => unreachable!(),
        }
    }

    #[test]
    fn knock_in_plus_knock_out_equals_vanilla_exactly() {
        let spots = [50 * SCALE, 100 * SCALE];
        let rates = [0, SCALE / 20];
        let sigmas = [SCALE / 10, SCALE / 2];
        let times = [SCALE / 4, SCALE];

        for &s in &spots {
            let strikes = [s / 2, s, 3 * s / 2];
            let down_barriers = [s / 2, 3 * s / 4, s - SCALE];
            let up_barriers = [s + SCALE, 5 * s / 4, 3 * s / 2];
            for &(kind_out, barriers) in &[
                (BarrierType::DownAndOut, down_barriers),
                (BarrierType::UpAndOut, up_barriers),
            ] {
                for &h in &barriers {
                    for &k in &strikes {
                        for &r in &rates {
                            for &sigma in &sigmas {
                                for &t in &times {
                                    for is_call in [false, true] {
                                        let out =
                                            barrier_option(s, k, h, r, sigma, t, is_call, kind_out)
                                                .unwrap();
                                        let knocked_in = barrier_option(
                                            s,
                                            k,
                                            h,
                                            r,
                                            sigma,
                                            t,
                                            is_call,
                                            pair(kind_out),
                                        )
                                        .unwrap();
                                        assert_eq!(out.vanilla, knocked_in.vanilla);
                                        assert_eq!(out.price + knocked_in.price, out.vanilla);
                                        assert!(out.price <= out.vanilla);
                                        assert!(knocked_in.price <= knocked_in.vanilla);

                                        let historical_out = barrier_option_with_state(
                                            s, k, h, r, sigma, t, is_call, kind_out, true,
                                        )
                                        .unwrap();
                                        let historical_in = barrier_option_with_state(
                                            s,
                                            k,
                                            h,
                                            r,
                                            sigma,
                                            t,
                                            is_call,
                                            pair(kind_out),
                                            true,
                                        )
                                        .unwrap();
                                        assert_eq!(historical_out.price, 0);
                                        assert_eq!(historical_in.price, historical_in.vanilla);
                                        assert_eq!(historical_out.vanilla, historical_in.vanilla);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn extreme_barrier_inputs_fail_closed_without_panicking() {
        let maximum_hp_input = (i128::MAX / 1_000) as u128;
        let cases = [
            (u128::MAX, SCALE, SCALE / 2, 0, SCALE / 5, SCALE),
            (SCALE, u128::MAX, SCALE / 2, 0, SCALE / 5, SCALE),
            (SCALE, SCALE, u128::MAX, 0, SCALE / 5, SCALE),
            (SCALE, SCALE, SCALE / 2, u128::MAX, SCALE / 5, SCALE),
            (SCALE, SCALE, SCALE / 2, 0, u128::MAX, SCALE),
            (SCALE, SCALE, SCALE / 2, 0, SCALE / 5, u128::MAX),
            (
                maximum_hp_input,
                maximum_hp_input,
                maximum_hp_input / 2,
                0,
                SCALE,
                SCALE,
            ),
            (
                maximum_hp_input,
                maximum_hp_input / 2,
                maximum_hp_input - 1,
                SCALE,
                SCALE,
                SCALE,
            ),
            (1, maximum_hp_input, 2, SCALE, SCALE, SCALE),
        ];

        for &(s, k, h, r, sigma, t) in &cases {
            for &is_call in &[false, true] {
                for barrier_type in [
                    BarrierType::DownAndOut,
                    BarrierType::DownAndIn,
                    BarrierType::UpAndOut,
                    BarrierType::UpAndIn,
                ] {
                    let outcome = std::panic::catch_unwind(|| {
                        barrier_option(s, k, h, r, sigma, t, is_call, barrier_type)
                    });
                    assert!(
                        outcome.is_ok(),
                        "barrier panicked for s={s}, k={k}, h={h}, r={r}, sigma={sigma}, \
                         t={t}, call={is_call}, type={barrier_type:?}"
                    );
                    if let Ok(result) = outcome.unwrap() {
                        assert!(result.price <= result.vanilla);
                    }
                }
            }
        }
    }
}

#[cfg(feature = "heston")]
mod heston_invariants {
    use super::*;
    use solmath::{exp_fixed_i, heston_price};

    fn discounted_strike(k: u128, r: u128, t: u128) -> u128 {
        let rt = fp_mul_i(r as i128, t as i128).unwrap();
        fp_mul_i(k as i128, exp_fixed_i(-rt).unwrap()).unwrap() as u128
    }

    #[test]
    fn deterministic_heston_obeys_bounds_and_stochastic_path_fails_closed() {
        let spots = [SCALE, 25 * SCALE, 100 * SCALE, 1_000 * SCALE];
        let strikes = [SCALE, 20 * SCALE, 100 * SCALE, 1_000 * SCALE];
        let rates = [0, SCALE / 20, SCALE / 5];
        let times = [SCALE / 100, SCALE, 10 * SCALE];
        let variances = [0, SCALE / 10_000, SCALE / 25, SCALE / 4];
        let kappas = [0, SCALE / 10, SCALE, 20 * SCALE];

        for &s in &spots {
            for &k in &strikes {
                for &r in &rates {
                    for &t in &times {
                        for &v0 in &variances {
                            for &kappa in &kappas {
                                for &theta in &variances {
                                    let (call, put) =
                                        heston_price(s, k, r, t, v0, kappa, theta, 0, 0).unwrap();
                                    let kd = discounted_strike(k, r, t);
                                    assert!(call <= s);
                                    assert!(put <= k);
                                    assert!(call >= s.saturating_sub(kd));
                                    assert!(put >= kd.saturating_sub(s));
                                    assert_eq!(call + kd, put + s);
                                }
                            }
                        }
                    }
                }
            }
        }

        for xi in [1, SCALE / 1_000, SCALE / 2, 5 * SCALE] {
            assert_eq!(
                heston_price(
                    100 * SCALE,
                    100 * SCALE,
                    SCALE / 20,
                    SCALE,
                    SCALE / 25,
                    SCALE,
                    SCALE / 25,
                    xi,
                    -SCALE_I / 2,
                ),
                Err(SolMathError::NoConvergence)
            );
        }
    }
}

#[cfg(feature = "nig")]
mod nig_invariants {
    use super::*;
    use solmath::{nig_call_64, nig_call_price, nig_price_certified, nig_put_64, NigParams};

    struct NigRng(u64);

    impl NigRng {
        fn next_u128(&mut self) -> u128 {
            let mut next = || {
                let mut x = self.0;
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                self.0 = x;
                x
            };
            next() as u128 | ((next() as u128) << 64)
        }
    }

    #[test]
    fn positive_expiry_nig_obeys_bounds_and_parity() {
        let params = NigParams {
            alpha: 10 * SCALE,
            beta: -2 * SCALE_I,
            delta_per_year: SCALE / 5,
        };
        let quote = nig_price_certified(
            100 * SCALE,
            100 * SCALE,
            SCALE_I / 20,
            0,
            SCALE,
            params,
            5_000_000_000,
        )
        .unwrap();
        assert!(quote.call <= 100 * SCALE + quote.max_abs_error);
        assert!(quote.put <= 100 * SCALE + quote.max_abs_error);
        let discounted_strike = 95_122_942_450_071u128;
        assert!(
            quote
                .call
                .abs_diff(quote.put + (100 * SCALE - discounted_strike))
                <= 64,
            "quote={quote:?}"
        );

        let call12 = nig_call_price(
            100 * SCALE,
            100 * SCALE,
            SCALE / 20,
            SCALE,
            10 * SCALE,
            -2 * SCALE_I,
            SCALE / 5,
        )
        .unwrap();
        let call6 = nig_call_64(
            100_000_000,
            100_000_000,
            50_000,
            1_000_000,
            10_000_000,
            -2_000_000,
            200_000,
        )
        .unwrap();
        let put6 = nig_put_64(
            100_000_000,
            100_000_000,
            50_000,
            1_000_000,
            10_000_000,
            -2_000_000,
            200_000,
        )
        .unwrap();
        assert!(call6 > 0 && put6 > 0);
        assert!(call12.abs_diff((call6 as u128) * 1_000_000) <= 500_000);

        assert_eq!(
            nig_call_price(100 * SCALE, 100 * SCALE, 0, SCALE, SCALE, 0, SCALE),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            nig_call_price(100 * SCALE, 90 * SCALE, 0, 0, 10 * SCALE, 0, SCALE),
            Ok(10 * SCALE)
        );
        assert_eq!(
            nig_call_64(100_000_000, 90_000_000, 0, 0, 10_000_000, 0, 1_000_000),
            Ok(10_000_000)
        );
        assert_eq!(
            nig_put_64(90_000_000, 100_000_000, 0, 0, 10_000_000, 0, 1_000_000),
            Ok(10_000_000)
        );
    }

    #[test]
    fn full_width_nig_inputs_fail_closed_without_panicking() {
        let cases = [
            (
                u128::MAX,
                u128::MAX,
                i128::MIN,
                i128::MAX,
                u128::MAX,
                NigParams {
                    alpha: u128::MAX,
                    beta: i128::MIN,
                    delta_per_year: u128::MAX,
                },
            ),
            (
                100 * SCALE,
                100 * SCALE,
                0,
                0,
                SCALE,
                NigParams {
                    alpha: 10 * SCALE,
                    beta: i128::MIN,
                    delta_per_year: SCALE,
                },
            ),
            (
                100 * SCALE,
                100 * SCALE,
                i128::MAX,
                i128::MIN,
                SCALE,
                NigParams {
                    alpha: 10 * SCALE,
                    beta: 0,
                    delta_per_year: SCALE,
                },
            ),
        ];
        for (spot, strike, rate, dividend, time, params) in cases {
            let result = std::panic::catch_unwind(|| {
                nig_price_certified(spot, strike, rate, dividend, time, params, u128::MAX)
            });
            assert!(result.is_ok(), "NIG panicked for {params:?}");
            assert!(result.unwrap().is_err());
        }

        let iters: u64 = std::env::var("SOLMATH_FUZZ_ITERS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(100_000);
        let mut rng = NigRng(0x4e49_475f_6675_7a7a);
        for index in 0..iters {
            let (spot, strike, rate, dividend, time, params, requested) = if index % 1_024 == 0 {
                // Periodically force execution past the cheap domain gates so
                // the fixed-point density and quadrature paths are included
                // in the no-panic soak as well as arbitrary full-width input.
                let spot = 1 + rng.next_u128() % (100_000 * SCALE);
                let strike = 1 + rng.next_u128() % (100_000 * SCALE);
                let rate = (rng.next_u128() % (SCALE / 2 + 1)) as i128 - SCALE_I / 4;
                let dividend = (rng.next_u128() % (SCALE / 2 + 1)) as i128 - SCALE_I / 4;
                let days = 1 + rng.next_u128() % 1_825;
                let time = days * SCALE / 365;
                let params = NigParams {
                    alpha: 2 * SCALE + rng.next_u128() % (98 * SCALE + 1),
                    beta: -SCALE_I / 2,
                    delta_per_year: SCALE / 5 + rng.next_u128() % SCALE,
                };
                let requested = spot.max(strike) / 1_000 + 1;
                (spot, strike, rate, dividend, time, params, requested)
            } else {
                (
                    rng.next_u128(),
                    rng.next_u128(),
                    rng.next_u128() as i128,
                    rng.next_u128() as i128,
                    rng.next_u128(),
                    NigParams {
                        alpha: rng.next_u128(),
                        beta: rng.next_u128() as i128,
                        delta_per_year: rng.next_u128(),
                    },
                    rng.next_u128(),
                )
            };
            let _ = nig_price_certified(spot, strike, rate, dividend, time, params, requested);
        }
    }
}

#[cfg(feature = "american-kbi")]
mod american_kbi_invariants {
    use super::*;
    use solmath::{american_kbi_price, AmericanKbiKind};

    struct Rng(u64);

    impl Rng {
        fn next_u128(&mut self) -> u128 {
            let mut next = || {
                let mut x = self.0;
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                self.0 = x;
                x
            };
            (u128::from(next()) << 64) | u128::from(next())
        }
    }

    #[test]
    fn accepted_kbi_quotes_obey_intrinsic_and_hard_price_bounds() {
        let strike = 100 * SCALE;
        for spot_dollars in [50u128, 75, 100, 125, 200] {
            for rate in [0, 60_000_000_000, 120_000_000_000] {
                for dividend_yield in [0, 60_000_000_000, 120_000_000_000] {
                    for sigma in [100_000_000_000, 300_000_000_000, 1_200_000_000_000] {
                        for maturity in [30 * SCALE / 365, SCALE / 2, 2 * SCALE] {
                            let spot = spot_dollars * SCALE;
                            let call = american_kbi_price(
                                spot,
                                strike,
                                rate,
                                dividend_yield,
                                sigma,
                                maturity,
                                AmericanKbiKind::Call,
                            )
                            .expect("grid is inside the documented KBI domain");
                            let put = american_kbi_price(
                                spot,
                                strike,
                                rate,
                                dividend_yield,
                                sigma,
                                maturity,
                                AmericanKbiKind::Put,
                            )
                            .expect("grid is inside the documented KBI domain");

                            assert!(call >= spot.saturating_sub(strike));
                            assert!(call <= spot);
                            assert!(put >= strike.saturating_sub(spot));
                            assert!(put <= strike);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn full_width_kbi_inputs_fail_closed_without_panicking() {
        let iters: u64 = std::env::var("SOLMATH_FUZZ_ITERS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(40_000);
        let mut rng = Rng(0x6b62_695f_6675_7a7a);

        for index in 0..iters {
            let spot = if index % 8 == 0 { 0 } else { rng.next_u128() };
            let strike = if index % 8 == 1 { 0 } else { rng.next_u128() };
            let rate = rng.next_u128();
            let dividend_yield = rng.next_u128();
            let sigma = rng.next_u128();
            let maturity = rng.next_u128();
            let kind = if index & 1 == 0 {
                AmericanKbiKind::Call
            } else {
                AmericanKbiKind::Put
            };
            let _ = american_kbi_price(spot, strike, rate, dividend_yield, sigma, maturity, kind);
        }

        let extrema = [0, 1, SCALE, u128::MAX];
        for &value in &extrema {
            for kind in [AmericanKbiKind::Call, AmericanKbiKind::Put] {
                let _ = american_kbi_price(value, value, value, value, value, value, kind);
                let _ =
                    american_kbi_price(u128::MAX, 100 * SCALE, value, value, value, value, kind);
            }
        }
    }
}

#[cfg(feature = "sabr")]
mod sabr_invariants {
    use super::*;
    use solmath::{
        exp_fixed_hp, fp_mul_hp_i,
        sabr::{
            certify_sabr_surface, CertifiedSabrSurface, MAX_SABR_SURFACE_MATURITIES,
            MAX_SABR_SURFACE_QUOTES, MAX_SABR_SURFACE_STRIKES,
        },
        sabr_price,
    };

    const HP_FACTOR: i128 = 1_000;
    const CONVEXITY_TOLERANCE: u128 = 1_000;

    fn hp_discounted_strike(k: u128, r: u128, t: u128) -> u128 {
        let rt = fp_mul_hp_i(r as i128 * HP_FACTOR, t as i128 * HP_FACTOR).unwrap();
        let discount = exp_fixed_hp(-rt).unwrap();
        let kd = fp_mul_hp_i(k as i128 * HP_FACTOR, discount).unwrap();
        ((kd + HP_FACTOR / 2) / HP_FACTOR) as u128
    }

    fn assert_certificate_is_exactly_borrowed(
        certificate: CertifiedSabrSurface<'_>,
        strikes: &[u128],
        maturities: &[u128],
        calls: &[u128],
        puts: &[u128],
    ) {
        assert_eq!(certificate.spot(), 100 * SCALE);
        assert_eq!(certificate.rate(), 0);
        assert_eq!(certificate.strikes(), strikes);
        assert_eq!(certificate.maturities(), maturities);
        assert_eq!(certificate.quote_count(), calls.len());
        for maturity_index in 0..maturities.len() {
            for strike_index in 0..strikes.len() {
                let index = maturity_index * strikes.len() + strike_index;
                let quote = certificate.quote_at(maturity_index, strike_index).unwrap();
                assert_eq!(quote.spot(), 100 * SCALE);
                assert_eq!(quote.rate(), 0);
                assert_eq!(quote.strike(), strikes[strike_index]);
                assert_eq!(quote.maturity(), maturities[maturity_index]);
                assert_eq!(quote.call(), calls[index]);
                assert_eq!(quote.put(), puts[index]);
            }
        }
        assert_eq!(
            certificate.quote_at(maturities.len(), 0),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            certificate.quote_at(0, strikes.len()),
            Err(SolMathError::DomainError)
        );
    }

    #[test]
    fn public_surface_certificate_enforces_global_static_arbitrage() {
        let strikes = [80 * SCALE, 100 * SCALE, 120 * SCALE];
        let maturities = [SCALE, 2 * SCALE, 3 * SCALE];
        let calls = [
            25 * SCALE,
            12 * SCALE,
            5 * SCALE,
            27 * SCALE,
            15 * SCALE,
            8 * SCALE,
            30 * SCALE,
            18 * SCALE,
            10 * SCALE,
        ];
        let puts = [
            5 * SCALE,
            12 * SCALE,
            25 * SCALE,
            7 * SCALE,
            15 * SCALE,
            28 * SCALE,
            10 * SCALE,
            18 * SCALE,
            30 * SCALE,
        ];

        let certificate =
            certify_sabr_surface(100 * SCALE, 0, &strikes, &maturities, &calls, &puts).unwrap();
        assert_certificate_is_exactly_borrowed(certificate, &strikes, &maturities, &calls, &puts);

        // One raw unit of parity residual must fail closed.
        let mut parity_calls = calls;
        parity_calls[4] += 1;
        assert_eq!(
            certify_sabr_surface(100 * SCALE, 0, &strikes, &maturities, &parity_calls, &puts,),
            Err(SolMathError::NoConvergence)
        );

        // Preserve node-level parity and bounds but introduce a call calendar
        // inversion.  Only whole-surface validation can detect this.
        let mut calendar_calls = calls;
        let mut calendar_puts = puts;
        calendar_calls[3] = 24 * SCALE;
        calendar_puts[3] = 4 * SCALE;
        assert_eq!(
            certify_sabr_surface(
                100 * SCALE,
                0,
                &strikes,
                &maturities,
                &calendar_calls,
                &calendar_puts,
            ),
            Err(SolMathError::NoConvergence)
        );

        // Every distant-wing node below has exact parity and hard bounds, but
        // the irregular-strike butterfly is negative.
        let wing_strikes = [50 * SCALE, 100 * SCALE, 200 * SCALE];
        let wing_maturities = [SCALE, 2 * SCALE];
        let wing_calls = [55 * SCALE, 54 * SCALE, SCALE, 55 * SCALE, 54 * SCALE, SCALE];
        let wing_puts = [
            5 * SCALE,
            54 * SCALE,
            101 * SCALE,
            5 * SCALE,
            54 * SCALE,
            101 * SCALE,
        ];
        assert_eq!(
            certify_sabr_surface(
                100 * SCALE,
                0,
                &wing_strikes,
                &wing_maturities,
                &wing_calls,
                &wing_puts,
            ),
            Err(SolMathError::NoConvergence)
        );

        assert_eq!(
            certify_sabr_surface(
                100 * SCALE,
                0,
                &strikes,
                &maturities,
                &calls[..calls.len() - 1],
                &puts,
            ),
            Err(SolMathError::DomainError)
        );

        let oversized_strikes = [SCALE; MAX_SABR_SURFACE_STRIKES + 1];
        let oversized_strike_calls = [0; (MAX_SABR_SURFACE_STRIKES + 1) * 2];
        assert_eq!(
            certify_sabr_surface(
                100 * SCALE,
                0,
                &oversized_strikes,
                &maturities[..2],
                &oversized_strike_calls,
                &oversized_strike_calls,
            ),
            Err(SolMathError::DomainError)
        );
        let oversized_maturities = [SCALE; MAX_SABR_SURFACE_MATURITIES + 1];
        let oversized_maturity_calls = [0; 3 * (MAX_SABR_SURFACE_MATURITIES + 1)];
        assert_eq!(
            certify_sabr_surface(
                100 * SCALE,
                0,
                &strikes,
                &oversized_maturities,
                &oversized_maturity_calls,
                &oversized_maturity_calls,
            ),
            Err(SolMathError::DomainError)
        );
        let quote_limit_strikes = [SCALE; MAX_SABR_SURFACE_QUOTES / 16 + 1];
        let quote_limit_maturities = [SCALE; 16];
        let quote_limit_calls = [0; (MAX_SABR_SURFACE_QUOTES / 16 + 1) * 16];
        assert_eq!(
            certify_sabr_surface(
                100 * SCALE,
                0,
                &quote_limit_strikes,
                &quote_limit_maturities,
                &quote_limit_calls,
                &quote_limit_calls,
            ),
            Err(SolMathError::DomainError)
        );
    }

    #[test]
    fn accepted_sabr_surfaces_are_bounded_monotone_convex_and_in_parity() {
        let surfaces = [
            (SCALE / 5, SCALE, 0, 0, SCALE, 0),
            (
                SCALE / 5,
                SCALE / 2,
                -3 * SCALE_I / 10,
                2 * SCALE / 5,
                SCALE,
                0,
            ),
            (
                SCALE / 10,
                SCALE,
                -SCALE_I / 5,
                SCALE / 5,
                SCALE / 2,
                SCALE / 20,
            ),
        ];
        let strikes = [
            60 * SCALE,
            70 * SCALE,
            80 * SCALE,
            90 * SCALE,
            100 * SCALE,
            110 * SCALE,
            120 * SCALE,
            130 * SCALE,
            140 * SCALE,
        ];
        let s = 100 * SCALE;

        for &(alpha, beta, rho, nu, t, r) in &surfaces {
            let mut calls = [0u128; 9];
            let mut puts = [0u128; 9];
            for (index, &k) in strikes.iter().enumerate() {
                let (call, put) =
                    sabr_price(s, k, r, t, alpha, beta, rho, nu).unwrap_or_else(|error| {
                        panic!(
                            "certified SABR surface point failed: alpha={alpha}, beta={beta}, \
                             rho={rho}, nu={nu}, t={t}, r={r}, k={k}: {error:?}"
                        )
                    });
                let kd = hp_discounted_strike(k, r, t);
                assert!(call <= s);
                assert!(put <= k);
                assert!(call >= s.saturating_sub(kd));
                assert!(put >= kd.saturating_sub(s));
                assert_eq!(call + kd, put + s);
                calls[index] = call;
                puts[index] = put;
            }

            for index in 1..strikes.len() {
                assert!(
                    calls[index - 1] >= calls[index],
                    "SABR calls increase with strike"
                );
                assert!(
                    puts[index - 1] <= puts[index],
                    "SABR puts decrease with strike"
                );
            }
            for index in 1..strikes.len() - 1 {
                assert!(
                    calls[index - 1]
                        .checked_add(calls[index + 1])
                        .and_then(|value| value.checked_add(CONVEXITY_TOLERANCE))
                        .unwrap()
                        >= calls[index].checked_mul(2).unwrap(),
                    "SABR call surface is not convex at strike {}",
                    strikes[index]
                );
                assert!(
                    puts[index - 1]
                        .checked_add(puts[index + 1])
                        .and_then(|value| value.checked_add(CONVEXITY_TOLERANCE))
                        .unwrap()
                        >= puts[index].checked_mul(2).unwrap(),
                    "SABR put surface is not convex at strike {}",
                    strikes[index]
                );
            }
        }
    }

    #[test]
    fn sabr_execution_rejects_the_uncertified_asymptotic_regime() {
        assert_eq!(
            sabr_price(
                100 * SCALE,
                100 * SCALE,
                0,
                SCALE,
                SCALE / 5,
                SCALE,
                0,
                SCALE,
            ),
            Err(SolMathError::NoConvergence)
        );
    }

    #[test]
    fn extreme_sabr_inputs_fail_closed_without_panicking() {
        let maximum = i128::MAX as u128;
        let cases = [
            (u128::MAX, SCALE, 0, SCALE, SCALE / 5, SCALE, 0, 0),
            (SCALE, u128::MAX, 0, SCALE, SCALE / 5, SCALE, 0, 0),
            (SCALE, SCALE, u128::MAX, SCALE, SCALE / 5, SCALE, 0, 0),
            (SCALE, SCALE, 0, u128::MAX, SCALE / 5, SCALE, 0, 0),
            (SCALE, SCALE, 0, SCALE, u128::MAX, SCALE, 0, 0),
            (SCALE, SCALE, 0, SCALE, SCALE / 5, u128::MAX, 0, 0),
            (SCALE, SCALE, 0, SCALE, SCALE / 5, SCALE, i128::MIN, 0),
            (SCALE, SCALE, 0, SCALE, SCALE / 5, SCALE, i128::MAX, 0),
            (SCALE, SCALE, 0, SCALE, SCALE / 5, SCALE, 0, u128::MAX),
            (maximum, SCALE, 0, SCALE, SCALE / 5, SCALE, 0, 0),
            (SCALE, maximum, 0, SCALE, SCALE / 5, SCALE, 0, 0),
        ];

        for &(s, k, r, t, alpha, beta, rho, nu) in &cases {
            let outcome = std::panic::catch_unwind(|| sabr_price(s, k, r, t, alpha, beta, rho, nu));
            assert!(
                outcome.is_ok(),
                "SABR panicked for s={s}, k={k}, r={r}, t={t}, alpha={alpha}, \
                 beta={beta}, rho={rho}, nu={nu}"
            );
            if let Ok((call, put)) = outcome.unwrap() {
                assert!(call <= s);
                assert!(put <= k);
            }
        }
    }
}
