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
        let correction = if self.lo >= 0 {
            (self.lo + half) / scale
        } else {
            (self.lo - half) / scale
        };
        self.hi + correction
    }

    /// Exact addition of two DoubleWord values.
    /// Carries overflow from lo into hi. Returns Err on hi overflow.
    #[inline]
    pub(crate) fn add(self, other: Self) -> Result<Self, crate::error::SolMathError> {
        let lo_sum = self.lo + other.lo;
        let carry = if lo_sum >= SCALE_I {
            1
        } else if lo_sum <= -SCALE_I {
            -1
        } else {
            0
        };
        let hi = self.hi.checked_add(other.hi)
            .and_then(|h| h.checked_add(carry))
            .ok_or(crate::error::SolMathError::Overflow)?;
        Ok(Self {
            hi,
            lo: lo_sum - carry * SCALE_I,
        })
    }

    #[inline]
    pub(crate) const fn hi(self) -> i128 {
        self.hi
    }

    #[inline]
    pub(crate) const fn lo(self) -> i128 {
        self.lo
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
        let dw = DoubleWord { hi: 5 * SCALE_I, lo: 0 };
        assert_eq!(dw.to_i128(), 5 * SCALE_I);
    }

    #[test]
    fn test_dw_to_i128_positive_correction() {
        // lo >= SCALE/2 should round hi up by 1
        let dw = DoubleWord { hi: 5 * SCALE_I, lo: SCALE_I / 2 };
        assert_eq!(dw.to_i128(), 5 * SCALE_I + 1);
    }

    #[test]
    fn test_dw_to_i128_negative_correction() {
        let dw = DoubleWord { hi: 5 * SCALE_I, lo: -SCALE_I / 2 };
        assert_eq!(dw.to_i128(), 5 * SCALE_I - 1);
    }

    #[test]
    fn test_dw_to_i128_small_lo_no_correction() {
        // lo < SCALE/2 should not change hi
        let dw = DoubleWord { hi: 5 * SCALE_I, lo: SCALE_I / 2 - 1 };
        assert_eq!(dw.to_i128(), 5 * SCALE_I);
    }

    #[test]
    fn test_dw_add_simple() {
        let a = DoubleWord { hi: 3 * SCALE_I, lo: 100 };
        let b = DoubleWord { hi: 4 * SCALE_I, lo: 200 };
        let c = a.add(b).unwrap();
        assert_eq!(c.hi, 7 * SCALE_I);
        assert_eq!(c.lo, 300);
    }

    #[test]
    fn test_dw_add_lo_carry() {
        let a = DoubleWord { hi: 3 * SCALE_I, lo: SCALE_I - 100 };
        let b = DoubleWord { hi: 4 * SCALE_I, lo: 200 };
        let c = a.add(b).unwrap();
        assert_eq!(c.hi, 7 * SCALE_I + 1);
        assert_eq!(c.lo, 100);
    }

    #[test]
    fn test_dw_add_lo_negative_carry() {
        let a = DoubleWord { hi: 3 * SCALE_I, lo: -(SCALE_I - 100) };
        let b = DoubleWord { hi: 4 * SCALE_I, lo: -200 };
        let c = a.add(b).unwrap();
        assert_eq!(c.hi, 7 * SCALE_I - 1);
        assert_eq!(c.lo, -100);
    }

    #[test]
    fn test_dw_invariant_lo_bounded() {
        // After add, |lo| < SCALE must hold
        // Use hi values that won't overflow when summed with carry
        let extremes = [
            DoubleWord { hi: 0, lo: SCALE_I - 1 },
            DoubleWord { hi: 0, lo: -(SCALE_I - 1) },
            DoubleWord { hi: 1_000 * SCALE_I, lo: SCALE_I - 1 },
            DoubleWord { hi: -1_000 * SCALE_I, lo: -(SCALE_I - 1) },
        ];
        for &a in &extremes {
            for &b in &extremes {
                let c = a.add(b).unwrap();
                assert!(c.lo.abs() < SCALE_I,
                    "lo invariant violated: a={:?}, b={:?}, result={:?}", a, b, c);
            }
        }
    }

    #[test]
    fn test_dw_to_i128_roundtrip() {
        // from_hi(x).to_i128() == x for any x
        for x in [0i128, 1, -1, SCALE_I, -SCALE_I, i128::MAX / 2, i128::MIN / 2] {
            assert_eq!(DoubleWord::from_hi(x).to_i128(), x);
        }
    }

    // ===== to_i128_at_scale tests =====

    #[test]
    fn test_dw_at_scale_matches_to_i128() {
        let cases = [
            DoubleWord { hi: 5 * SCALE_I, lo: 0 },
            DoubleWord { hi: 5 * SCALE_I, lo: SCALE_I / 2 },
            DoubleWord { hi: 5 * SCALE_I, lo: -SCALE_I / 2 },
            DoubleWord { hi: 5 * SCALE_I, lo: SCALE_I / 2 - 1 },
            DoubleWord { hi: 0, lo: 0 },
            DoubleWord { hi: -3 * SCALE_I, lo: 100 },
            DoubleWord { hi: -3 * SCALE_I, lo: -100 },
        ];
        for dw in cases {
            assert_eq!(dw.to_i128(), dw.to_i128_at_scale(SCALE_I),
                "Mismatch for {:?}", dw);
        }
    }

    #[test]
    fn test_dw_at_scale_hp() {
        use crate::constants::SCALE_HP;
        // lo = SCALE_HP/2 should round up by 1
        let dw = DoubleWord { hi: 5 * SCALE_HP, lo: SCALE_HP / 2 };
        assert_eq!(dw.to_i128_at_scale(SCALE_HP), 5 * SCALE_HP + 1);

        // lo = SCALE_HP/2 - 1 should not round
        let dw2 = DoubleWord { hi: 5 * SCALE_HP, lo: SCALE_HP / 2 - 1 };
        assert_eq!(dw2.to_i128_at_scale(SCALE_HP), 5 * SCALE_HP);
    }

    #[test]
    fn test_dw_at_scale_hp_negative() {
        use crate::constants::SCALE_HP;
        // Negative lo at HP scale
        let dw = DoubleWord { hi: 5 * SCALE_HP, lo: -SCALE_HP / 2 };
        assert_eq!(dw.to_i128_at_scale(SCALE_HP), 5 * SCALE_HP - 1);

        let dw2 = DoubleWord { hi: 5 * SCALE_HP, lo: -(SCALE_HP / 2 - 1) };
        assert_eq!(dw2.to_i128_at_scale(SCALE_HP), 5 * SCALE_HP);
    }

    #[test]
    fn test_dw_at_scale_hp_roundtrip() {
        use crate::constants::SCALE_HP;
        for x in [0i128, 1, -1, SCALE_HP, -SCALE_HP] {
            assert_eq!(DoubleWord::from_hi(x).to_i128_at_scale(SCALE_HP), x);
        }
    }
}
