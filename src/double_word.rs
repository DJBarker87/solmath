use crate::constants::SCALE_I;

/// A double-word fixed-point value: true_value = hi + lo / SCALE.
///
/// hi carries the standard SCALE-precision result.
/// lo carries the sub-ULP residual from the computation that produced hi.
/// Invariant: |lo| < SCALE_I.
///
/// This enables error-free propagation through multiply chains:
/// instead of discarding rounding remainders, they accumulate in lo
/// and can be folded back when precision matters (e.g. pow, IV).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DoubleWord {
    hi: i128,
    lo: i128,
}

impl DoubleWord {
    /// Create from a standard-precision value (lo = 0).
    #[inline]
    pub const fn from_hi(hi: i128) -> Self {
        Self { hi, lo: 0 }
    }

    /// Internal constructor for values whose residual invariant is already established.
    #[inline]
    pub(crate) const fn new_raw(hi: i128, lo: i128) -> Self {
        Self { hi, lo }
    }

    /// Collapse to standard precision by rounding lo into hi.
    #[inline]
    pub fn to_i128(self) -> i128 {
        self.to_i128_at_scale(SCALE_I)
    }

    /// Collapse by rounding lo into hi using the provided scale.
    /// Use `to_i128()` for standard SCALE_I, this for HP or other scales.
    #[inline]
    pub(crate) fn to_i128_at_scale(self, scale: i128) -> i128 {
        let half = scale / 2;
        let abs_lo = self.lo.unsigned_abs();
        let correction = if abs_lo < half as u128 {
            0
        } else if abs_lo > half as u128 {
            self.lo.signum()
        } else if self.lo > 0 && self.hi >= 0 {
            1
        } else if self.lo < 0 && self.hi <= 0 {
            -1
        } else {
            // `hi` was already rounded away from zero and the opposite-sign
            // half-ULP residual records the exact tie. Applying another
            // correction here would undo the original rounding.
            0
        };
        self.hi + correction
    }

    /// Exact addition of two DoubleWord values.
    /// Carries overflow from lo into hi. Returns Err on hi overflow.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn checked_add(self, other: Self) -> Result<Self, crate::error::SolMathError> {
        let lo_sum = self.lo + other.lo;
        let carry = if lo_sum >= SCALE_I {
            1
        } else if lo_sum <= -SCALE_I {
            -1
        } else {
            0
        };
        // Try all safe association orders. The first pair can overflow even
        // when the carry cancels it and the exact three-term sum is in range.
        let hi = self
            .hi
            .checked_add(other.hi)
            .and_then(|h| h.checked_add(carry))
            .or_else(|| {
                self.hi
                    .checked_add(carry)
                    .and_then(|h| h.checked_add(other.hi))
            })
            .or_else(|| {
                other
                    .hi
                    .checked_add(carry)
                    .and_then(|h| h.checked_add(self.hi))
            })
            .ok_or(crate::error::SolMathError::Overflow)?;
        Ok(Self {
            hi,
            lo: lo_sum - carry * SCALE_I,
        })
    }

    #[allow(dead_code)]
    #[inline]
    pub const fn hi(self) -> i128 {
        self.hi
    }

    #[allow(dead_code)]
    #[inline]
    pub const fn lo(self) -> i128 {
        self.lo
    }
}

#[cfg(kani)]
mod verification {
    use super::*;
    use crate::constants::SCALE_HP;

    fn prove_collapse_is_half_ulp(scale: i128) {
        let hi: i128 = kani::any();
        let lo: i128 = kani::any();
        kani::assume(lo > -scale);
        kani::assume(lo < scale);

        let half = scale / 2;
        let magnitude = lo.unsigned_abs();
        let correction = if magnitude < half as u128 {
            0
        } else if magnitude > half as u128 {
            lo.signum()
        } else if lo > 0 && hi >= 0 {
            1
        } else if lo < 0 && hi <= 0 {
            -1
        } else {
            0
        };
        let expected = hi.checked_add(correction);
        kani::assume(expected.is_some());

        let actual = DoubleWord::new_raw(hi, lo).to_i128_at_scale(scale);
        let error_numerator = if correction == 0 {
            magnitude
        } else {
            scale as u128 - magnitude
        };

        assert_eq!(actual, expected.unwrap());
        assert!(error_numerator <= half as u128);
    }

    /// Prove collapsing every valid standard-scale residual rounds to nearest
    /// with ties away from zero and at most one-half output ULP of error.
    #[kani::proof]
    fn standard_collapse_is_half_ulp_for_every_valid_residual() {
        prove_collapse_is_half_ulp(SCALE_I);
    }

    /// Prove the same collapse property at the crate's high-precision scale.
    #[kani::proof]
    fn hp_collapse_is_half_ulp_for_every_valid_residual() {
        prove_collapse_is_half_ulp(SCALE_HP);
    }

    /// Prove exact double-word addition re-normalizes every pair of valid
    /// sub-ULP residuals back into `(-SCALE, SCALE)` whenever the high word is
    /// representable. This preserves the invariant required by later ULP
    /// collapse proofs.
    #[kani::proof]
    fn checked_add_preserves_the_sub_ulp_residual_invariant() {
        let a_hi: i128 = kani::any();
        let a_lo: i128 = kani::any();
        let b_hi: i128 = kani::any();
        let b_lo: i128 = kani::any();
        kani::assume(a_lo > -SCALE_I);
        kani::assume(a_lo < SCALE_I);
        kani::assume(b_lo > -SCALE_I);
        kani::assume(b_lo < SCALE_I);

        let a = DoubleWord::new_raw(a_hi, a_lo);
        let b = DoubleWord::new_raw(b_hi, b_lo);
        if let Ok(sum) = a.checked_add(b) {
            assert!(sum.lo.unsigned_abs() < SCALE_I as u128);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::SCALE_I;

    #[test]
    fn test_dw_from_hi() {
        let dw = DoubleWord::from_hi(42 * SCALE_I);
        assert_eq!(dw.hi, 42 * SCALE_I);
        assert_eq!(dw.lo, 0);
    }

    #[test]
    fn test_dw_to_i128_no_correction() {
        let dw = DoubleWord {
            hi: 5 * SCALE_I,
            lo: 0,
        };
        assert_eq!(dw.to_i128(), 5 * SCALE_I);
    }

    #[test]
    fn test_dw_to_i128_positive_correction() {
        // lo >= SCALE/2 should round hi up by 1
        let dw = DoubleWord {
            hi: 5 * SCALE_I,
            lo: SCALE_I / 2,
        };
        assert_eq!(dw.to_i128(), 5 * SCALE_I + 1);
    }

    #[test]
    fn test_dw_to_i128_negative_correction() {
        let dw = DoubleWord {
            hi: 5 * SCALE_I,
            lo: -SCALE_I / 2,
        };
        assert_eq!(dw.to_i128(), 5 * SCALE_I);
    }

    #[test]
    fn test_dw_to_i128_small_lo_no_correction() {
        // lo < SCALE/2 should not change hi
        let dw = DoubleWord {
            hi: 5 * SCALE_I,
            lo: SCALE_I / 2 - 1,
        };
        assert_eq!(dw.to_i128(), 5 * SCALE_I);
    }

    #[test]
    fn test_dw_add_simple() {
        let a = DoubleWord {
            hi: 3 * SCALE_I,
            lo: 100,
        };
        let b = DoubleWord {
            hi: 4 * SCALE_I,
            lo: 200,
        };
        let c = a.checked_add(b).unwrap();
        assert_eq!(c.hi, 7 * SCALE_I);
        assert_eq!(c.lo, 300);
    }

    #[test]
    fn test_dw_add_lo_carry() {
        let a = DoubleWord {
            hi: 3 * SCALE_I,
            lo: SCALE_I - 100,
        };
        let b = DoubleWord {
            hi: 4 * SCALE_I,
            lo: 200,
        };
        let c = a.checked_add(b).unwrap();
        assert_eq!(c.hi, 7 * SCALE_I + 1);
        assert_eq!(c.lo, 100);
    }

    #[test]
    fn test_dw_add_lo_negative_carry() {
        let a = DoubleWord {
            hi: 3 * SCALE_I,
            lo: -(SCALE_I - 100),
        };
        let b = DoubleWord {
            hi: 4 * SCALE_I,
            lo: -200,
        };
        let c = a.checked_add(b).unwrap();
        assert_eq!(c.hi, 7 * SCALE_I - 1);
        assert_eq!(c.lo, -100);
    }

    #[test]
    fn test_dw_invariant_lo_bounded() {
        // After add, |lo| < SCALE must hold
        // Use hi values that won't overflow when summed with carry
        let extremes = [
            DoubleWord {
                hi: 0,
                lo: SCALE_I - 1,
            },
            DoubleWord {
                hi: 0,
                lo: -(SCALE_I - 1),
            },
            DoubleWord {
                hi: 1_000 * SCALE_I,
                lo: SCALE_I - 1,
            },
            DoubleWord {
                hi: -1_000 * SCALE_I,
                lo: -(SCALE_I - 1),
            },
        ];
        for &a in &extremes {
            for &b in &extremes {
                let c = a.checked_add(b).unwrap();
                assert!(
                    c.lo.abs() < SCALE_I,
                    "lo invariant violated: a={:?}, b={:?}, result={:?}",
                    a,
                    b,
                    c
                );
            }
        }
    }

    #[test]
    fn test_dw_to_i128_roundtrip() {
        // from_hi(x).to_i128() == x for any x
        for x in [
            0i128,
            1,
            -1,
            SCALE_I,
            -SCALE_I,
            i128::MAX / 2,
            i128::MIN / 2,
        ] {
            assert_eq!(DoubleWord::from_hi(x).to_i128(), x);
        }
    }

    // ===== to_i128_at_scale tests =====

    #[test]
    fn test_dw_at_scale_matches_to_i128() {
        let cases = [
            DoubleWord {
                hi: 5 * SCALE_I,
                lo: 0,
            },
            DoubleWord {
                hi: 5 * SCALE_I,
                lo: SCALE_I / 2,
            },
            DoubleWord {
                hi: 5 * SCALE_I,
                lo: -SCALE_I / 2,
            },
            DoubleWord {
                hi: 5 * SCALE_I,
                lo: SCALE_I / 2 - 1,
            },
            DoubleWord { hi: 0, lo: 0 },
            DoubleWord {
                hi: -3 * SCALE_I,
                lo: 100,
            },
            DoubleWord {
                hi: -3 * SCALE_I,
                lo: -100,
            },
        ];
        for dw in cases {
            assert_eq!(
                dw.to_i128(),
                dw.to_i128_at_scale(SCALE_I),
                "Mismatch for {:?}",
                dw
            );
        }
    }

    #[test]
    fn test_dw_at_scale_hp() {
        use crate::constants::SCALE_HP;
        // lo = SCALE_HP/2 should round up by 1
        let dw = DoubleWord {
            hi: 5 * SCALE_HP,
            lo: SCALE_HP / 2,
        };
        assert_eq!(dw.to_i128_at_scale(SCALE_HP), 5 * SCALE_HP + 1);

        // lo = SCALE_HP/2 - 1 should not round
        let dw2 = DoubleWord {
            hi: 5 * SCALE_HP,
            lo: SCALE_HP / 2 - 1,
        };
        assert_eq!(dw2.to_i128_at_scale(SCALE_HP), 5 * SCALE_HP);
    }

    #[test]
    fn test_dw_at_scale_hp_negative() {
        use crate::constants::SCALE_HP;
        // Negative lo at HP scale
        let dw = DoubleWord {
            hi: 5 * SCALE_HP,
            lo: -SCALE_HP / 2,
        };
        assert_eq!(dw.to_i128_at_scale(SCALE_HP), 5 * SCALE_HP);

        let dw2 = DoubleWord {
            hi: 5 * SCALE_HP,
            lo: -(SCALE_HP / 2 - 1),
        };
        assert_eq!(dw2.to_i128_at_scale(SCALE_HP), 5 * SCALE_HP);
    }

    #[test]
    fn test_dw_at_scale_hp_roundtrip() {
        use crate::constants::SCALE_HP;
        for x in [0i128, 1, -1, SCALE_HP, -SCALE_HP] {
            assert_eq!(DoubleWord::from_hi(x).to_i128_at_scale(SCALE_HP), x);
        }
    }

    #[test]
    fn test_dw_exact_half_ties_round_away_from_zero_once() {
        let pos = DoubleWord::new_raw(1, -SCALE_I / 2);
        let neg = DoubleWord::new_raw(-1, SCALE_I / 2);
        assert_eq!(pos.to_i128(), 1);
        assert_eq!(neg.to_i128(), -1);

        let pos_from_zero = DoubleWord::new_raw(0, SCALE_I / 2);
        let neg_from_zero = DoubleWord::new_raw(0, -SCALE_I / 2);
        assert_eq!(pos_from_zero.to_i128(), 1);
        assert_eq!(neg_from_zero.to_i128(), -1);
    }

    #[test]
    fn test_dw_add_carry_can_cancel_intermediate_overflow() {
        let a = DoubleWord::new_raw(i128::MAX, -SCALE_I / 2);
        let b = DoubleWord::new_raw(1, -SCALE_I / 2);
        let sum = a.checked_add(b).unwrap();
        assert_eq!(sum, DoubleWord::from_hi(i128::MAX));
    }
}
