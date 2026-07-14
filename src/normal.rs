use crate::arithmetic::{fp_div_i, fp_mul_i, fp_mul_i_round, fp_sqrt};
use crate::constants::*;
use crate::error::SolMathError;
use crate::norm_cdf_coeffs::*;
use crate::transcendental::{exp_fixed_i, ln_fixed_i};

/// Standard normal PDF: phi(x) = (1/sqrt(2*pi)) * exp(-x^2/2) at SCALE.
///
/// - **x**: signed fixed-point at `SCALE` (1e12).
/// - **Returns**: `i128` at `SCALE`, always >= 0. Returns 0 for extreme |x|.
/// - **Errors**: `Overflow` on internal arithmetic overflow (extremely unlikely).
/// - **Accuracy**: max 2 ULP.
pub fn norm_pdf(x: i128) -> Result<i128, SolMathError> {
    // Extreme |x|: the pdf underflows to 0 long before 9σ. Short-circuit
    // before squaring so huge |x| (up to i128::MIN/MAX) returns Ok(0) as
    // documented instead of an internal Overflow. Written as two compares —
    // x.abs() would overflow for x == i128::MIN.
    if x > 9 * SCALE_I || x < -9 * SCALE_I {
        return Ok(0);
    }
    let x_sq = fp_mul_i(x, x)?;
    // |x| ≤ 9·SCALE_I after the guard, so x_sq ≤ 81·SCALE_I; /2 and negate: result ∈ [-41·SCALE_I, 0], fits i128.
    let neg_half_x_sq = -(x_sq / 2);
    // Guard: for 8.94 < |x| ≤ 9, -x²/2 underflows past exp's range → pdf is 0.
    if neg_half_x_sq < -40 * SCALE_I {
        return Ok(0);
    }
    // After guard: neg_half_x_sq ∈ [-40·SCALE, 0] — exp_fixed_i cannot fail.
    let exp_term = match exp_fixed_i(neg_half_x_sq) {
        Ok(v) => v,
        Err(_) => return Ok(0), // unreachable after guard
    };
    fp_mul_i(INV_SQRT_2PI, exp_term)
}

#[inline(always)]
fn round_shift_cdf(value: i128) -> i64 {
    const HALF: i128 = 1i128 << (CDF_T_Q - 1);
    if value >= 0 {
        ((value + HALF) >> CDF_T_Q) as i64
    } else {
        -(((-value + HALF) >> CDF_T_Q) as i64)
    }
}

#[inline]
fn horner_guard_q44<const N: usize>(coefficients: &[i64; N], t: i64) -> i64 {
    let mut result = coefficients[N - 1];
    for coefficient in coefficients[..N - 1].iter().rev() {
        result = round_shift_cdf(result as i128 * t as i128) + coefficient;
    }
    let half = 1i64 << (CDF_COEFF_GUARD_Q - 1);
    if result >= 0 {
        (result + half) >> CDF_COEFF_GUARD_Q
    } else {
        -((-result + half) >> CDF_COEFF_GUARD_Q)
    }
}

/// Evaluate a guarded CDF polynomial and its derivative with respect to its
/// normalized coordinate.  The value path is deliberately bit-for-bit the
/// same Horner recurrence as `horner_guard_q44`.
#[inline]
fn horner_guard_q44_with_derivative<const N: usize>(coefficients: &[i64; N], t: i64) -> (i64, i64) {
    let mut value = coefficients[N - 1];
    let mut derivative = 0i64;
    for coefficient in coefficients[..N - 1].iter().rev() {
        derivative = round_shift_cdf(derivative as i128 * t as i128) + value;
        value = round_shift_cdf(value as i128 * t as i128) + coefficient;
    }
    let half = 1i64 << (CDF_COEFF_GUARD_Q - 1);
    let rounded_value = if value >= 0 {
        (value + half) >> CDF_COEFF_GUARD_Q
    } else {
        -((-value + half) >> CDF_COEFF_GUARD_Q)
    };
    (rounded_value, derivative)
}

/// Tail Horner evaluation with 16 evaluation-only guard bits.
///
/// The stored coefficients remain Q23 i64 values. Promoting them to Q39 in
/// an i64 accumulator prevents sub-raw tail increments from wobbling across a
/// final rounding threshold, without increasing the coefficient payload or
/// changing the common body path.
#[inline]
fn horner_tail_guard_q44<const N: usize>(coefficients: &[i64; N], t: i64) -> i64 {
    let mut result = coefficients[N - 1] << CDF_TAIL_EVAL_EXTRA_Q;
    for coefficient in coefficients[..N - 1].iter().rev() {
        result =
            round_shift_cdf(result as i128 * t as i128) + (*coefficient << CDF_TAIL_EVAL_EXTRA_Q);
    }

    const OUTPUT_Q: u32 = CDF_COEFF_GUARD_Q + CDF_TAIL_EVAL_EXTRA_Q;
    const OUTPUT_HALF: i64 = 1i64 << (OUTPUT_Q - 1);
    if result >= 0 {
        (result + OUTPUT_HALF) >> OUTPUT_Q
    } else {
        -((-result + OUTPUT_HALF) >> OUTPUT_Q)
    }
}

#[inline]
fn horner_tail_guard_q44_with_derivative<const N: usize>(
    coefficients: &[i64; N],
    t: i64,
) -> (i64, i64) {
    let mut value = coefficients[N - 1] << CDF_TAIL_EVAL_EXTRA_Q;
    let mut derivative = 0i64;
    for coefficient in coefficients[..N - 1].iter().rev() {
        derivative = round_shift_cdf(derivative as i128 * t as i128) + value;
        value =
            round_shift_cdf(value as i128 * t as i128) + (*coefficient << CDF_TAIL_EVAL_EXTRA_Q);
    }
    const OUTPUT_Q: u32 = CDF_COEFF_GUARD_Q + CDF_TAIL_EVAL_EXTRA_Q;
    const OUTPUT_HALF: i64 = 1i64 << (OUTPUT_Q - 1);
    let rounded_value = if value >= 0 {
        (value + OUTPUT_HALF) >> OUTPUT_Q
    } else {
        -((-value + OUTPUT_HALF) >> OUTPUT_Q)
    };
    (rounded_value, derivative)
}

#[inline(always)]
fn derivative_to_pdf(derivative: i64, bits: u32, tail: bool) -> i128 {
    // Every CDF interval has half-width 0.25, hence dx/dt = 0.25 and
    // d/dx = 4 d/dt.  Tail polynomials approximate Phi(-x), so negate.
    let signed = if tail {
        -(derivative as i128)
    } else {
        derivative as i128
    };
    let value = signed * 4;
    let half = 1i128 << (bits - 1);
    let rounded = if value >= 0 {
        (value + half) >> bits
    } else {
        -((-value + half) >> bits)
    };
    rounded.clamp(0, SCALE_I)
}

/// Map `(|x|-mid)/half_width` to Q44 with a Q44 reciprocal guard.
#[inline(always)]
pub(crate) fn poly_map_t_q44(ax: i128, mid: i128, hw: i128) -> i64 {
    // round(2^88 / half_width), followed by a 44-bit shift, produces Q44.
    // The reciprocal error contributes <0.015 Q44 units at |t|≤1.
    let reciprocal = match hw {
        250_000_000_000 => 1_237_940_039_285_380i128,
        375_000_000_000 => 825_293_359_523_587i128,
        500_000_000_000 => 618_970_019_642_690i128,
        // All current callers use one of the three optimized constants above.
        // Keep the helper total so a future internal caller cannot introduce a
        // runtime panic. `hw <= 0` maps to zero and is harmless because the
        // mapped coordinate is used only by fixed, positive-width intervals.
        _ if hw > 0 => ((1i128 << 88) + hw / 2) / hw,
        _ => 0,
    };
    round_shift_cdf((ax - mid) * reciprocal)
}

/// Direct positive CDF tail for `5 < x <= 8`.
#[inline]
fn norm_cdf_positive_tail(x_abs: i128) -> i128 {
    let tail = if x_abs <= 11 * SCALE_I / 2 {
        let t = poly_map_t_q44(x_abs, 21 * SCALE_I / 4, SCALE_I / 4);
        horner_tail_guard_q44(&NORM_TAIL_50_55_Q23, t)
    } else if x_abs <= 6 * SCALE_I {
        let t = poly_map_t_q44(x_abs, 23 * SCALE_I / 4, SCALE_I / 4);
        horner_tail_guard_q44(&NORM_TAIL_55_60_Q23, t)
    } else if x_abs <= 13 * SCALE_I / 2 {
        let t = poly_map_t_q44(x_abs, 25 * SCALE_I / 4, SCALE_I / 4);
        horner_tail_guard_q44(&NORM_TAIL_60_65_Q23, t)
    } else if x_abs <= 7 * SCALE_I {
        let t = poly_map_t_q44(x_abs, 27 * SCALE_I / 4, SCALE_I / 4);
        horner_tail_guard_q44(&NORM_TAIL_65_70_Q23, t)
    } else if x_abs <= NORM_TAIL_HALF_RAW_CUTOFF {
        1
    } else {
        0
    };
    SCALE_I - tail.clamp(0, SCALE_I as i64) as i128
}

#[inline]
fn norm_cdf_positive_tail_and_pdf(x_abs: i128) -> (i128, i128) {
    const TAIL_Q: u32 = CDF_COEFF_GUARD_Q + CDF_TAIL_EVAL_EXTRA_Q;
    let (tail, derivative) = if x_abs <= 11 * SCALE_I / 2 {
        let t = poly_map_t_q44(x_abs, 21 * SCALE_I / 4, SCALE_I / 4);
        horner_tail_guard_q44_with_derivative(&NORM_TAIL_50_55_Q23, t)
    } else if x_abs <= 6 * SCALE_I {
        let t = poly_map_t_q44(x_abs, 23 * SCALE_I / 4, SCALE_I / 4);
        horner_tail_guard_q44_with_derivative(&NORM_TAIL_55_60_Q23, t)
    } else if x_abs <= 13 * SCALE_I / 2 {
        let t = poly_map_t_q44(x_abs, 25 * SCALE_I / 4, SCALE_I / 4);
        horner_tail_guard_q44_with_derivative(&NORM_TAIL_60_65_Q23, t)
    } else if x_abs <= 7 * SCALE_I {
        let t = poly_map_t_q44(x_abs, 27 * SCALE_I / 4, SCALE_I / 4);
        horner_tail_guard_q44_with_derivative(&NORM_TAIL_65_70_Q23, t)
    } else {
        let tail = if x_abs <= NORM_TAIL_HALF_RAW_CUTOFF {
            1
        } else {
            0
        };
        return (SCALE_I - tail, 0);
    };
    (
        SCALE_I - (tail as i128).clamp(0, SCALE_I),
        derivative_to_pdf(derivative, TAIL_Q, true),
    )
}

/// Standard normal CDF: Phi(x) at SCALE.
///
/// 10-piece guarded body + four direct guarded tail polynomials.
///
/// - **x**: signed fixed-point at `SCALE` (1e12).
/// - **Returns**: `i128` probability in [0, SCALE_I]. Returns 0 for x < -8*SCALE, SCALE_I for x > 8*SCALE.
/// - **Errors**: `Overflow` on internal arithmetic overflow (extremely unlikely).
/// - **Accuracy**: max 2 ULP on the retained 100K production and 10K
///   seam-focused adversarial corpora; exactly symmetric and monotone on the
///   generator's dense interval sweeps.
pub fn norm_cdf_poly(x: i128) -> Result<i128, SolMathError> {
    if x < -8 * SCALE_I {
        return Ok(0);
    }
    if x > 8 * SCALE_I {
        return Ok(SCALE_I);
    }
    if x == 0 {
        return Ok(SCALE_I / 2);
    }

    let ax = x.abs();

    // A balanced decision tree holds the seven degree-8 body pieces to at
    // most four comparisons.  A sequential chain made upper-body and tail
    // inputs pay for every earlier interval (~8 CU per comparison on SBF).
    let cdf_pos = if ax <= 7 * SCALE_I / 2 {
        if ax <= 3 * SCALE_I / 2 {
            if ax <= SCALE_I / 2 {
                horner_guard_q44(
                    &NORM_CDF_0_05_Q23,
                    poly_map_t_q44(ax, SCALE_I / 4, SCALE_I / 4),
                ) as i128
            } else if ax <= SCALE_I {
                horner_guard_q44(
                    &NORM_CDF_05_10_Q23,
                    poly_map_t_q44(ax, 3 * SCALE_I / 4, SCALE_I / 4),
                ) as i128
            } else {
                horner_guard_q44(
                    &NORM_CDF_10_15_Q23,
                    poly_map_t_q44(ax, 5 * SCALE_I / 4, SCALE_I / 4),
                ) as i128
            }
        } else if ax <= 5 * SCALE_I / 2 {
            if ax <= 2 * SCALE_I {
                horner_guard_q44(
                    &NORM_CDF_15_20_Q23,
                    poly_map_t_q44(ax, 7 * SCALE_I / 4, SCALE_I / 4),
                ) as i128
            } else {
                horner_guard_q44(
                    &NORM_CDF_20_25_Q23,
                    poly_map_t_q44(ax, 9 * SCALE_I / 4, SCALE_I / 4),
                ) as i128
            }
        } else if ax <= 3 * SCALE_I {
            horner_guard_q44(
                &NORM_CDF_25_30_Q23,
                poly_map_t_q44(ax, 11 * SCALE_I / 4, SCALE_I / 4),
            ) as i128
        } else {
            horner_guard_q44(
                &NORM_CDF_30_35_Q23,
                poly_map_t_q44(ax, 13 * SCALE_I / 4, SCALE_I / 4),
            ) as i128
        }
    } else if ax <= 5 * SCALE_I {
        if ax <= 4 * SCALE_I {
            horner_guard_q44(
                &NORM_CDF_35_40_Q23,
                poly_map_t_q44(ax, 15 * SCALE_I / 4, SCALE_I / 4),
            ) as i128
        } else if ax <= 9 * SCALE_I / 2 {
            horner_guard_q44(
                &NORM_CDF_40_45_Q23,
                poly_map_t_q44(ax, 17 * SCALE_I / 4, SCALE_I / 4),
            ) as i128
        } else {
            horner_guard_q44(
                &NORM_CDF_45_50_Q23,
                poly_map_t_q44(ax, 19 * SCALE_I / 4, SCALE_I / 4),
            ) as i128
        }
    } else {
        norm_cdf_positive_tail(ax)
    };

    let cdf_pos = cdf_pos.clamp(0, SCALE_I);

    // cdf_pos ∈ [0, SCALE_I] after clamp; SCALE_I - cdf_pos ∈ [0, SCALE_I], fits i128.
    Ok(if x >= 0 { cdf_pos } else { SCALE_I - cdf_pos })
}

/// Direct polynomial CDF together with the analytic derivative of the active
/// CDF piece as an inexpensive PDF approximation.
///
/// The CDF result is bit-identical to [`norm_cdf_poly`].  On `|x| <= 5`, the
/// derived PDF differs from the exponential reference by less than 500 raw
/// SCALE units (5e-10 absolute).  This path is intended for iterative kernels
/// such as the American-option smooth-pasting equation, where a full
/// exponential for every Jacobian sample would dominate compute.
pub fn norm_cdf_and_pdf_poly(x: i128) -> Result<(i128, i128), SolMathError> {
    if x < -8 * SCALE_I {
        return Ok((0, 0));
    }
    if x > 8 * SCALE_I {
        return Ok((SCALE_I, 0));
    }
    if x == 0 {
        return Ok((SCALE_I / 2, INV_SQRT_2PI));
    }

    macro_rules! body_piece {
        ($coefficients:expr, $midpoint:expr, $half_width:expr) => {{
            let t = poly_map_t_q44(x.abs(), $midpoint, $half_width);
            let (cdf, derivative) = horner_guard_q44_with_derivative($coefficients, t);
            (
                cdf as i128,
                derivative_to_pdf(derivative, CDF_COEFF_GUARD_Q, false),
            )
        }};
    }

    let ax = x.abs();
    let (cdf_pos, pdf) = if ax <= 7 * SCALE_I / 2 {
        if ax <= 3 * SCALE_I / 2 {
            if ax <= SCALE_I / 2 {
                body_piece!(&NORM_CDF_0_05_Q23, SCALE_I / 4, SCALE_I / 4)
            } else if ax <= SCALE_I {
                body_piece!(&NORM_CDF_05_10_Q23, 3 * SCALE_I / 4, SCALE_I / 4)
            } else {
                body_piece!(&NORM_CDF_10_15_Q23, 5 * SCALE_I / 4, SCALE_I / 4)
            }
        } else if ax <= 5 * SCALE_I / 2 {
            if ax <= 2 * SCALE_I {
                body_piece!(&NORM_CDF_15_20_Q23, 7 * SCALE_I / 4, SCALE_I / 4)
            } else {
                body_piece!(&NORM_CDF_20_25_Q23, 9 * SCALE_I / 4, SCALE_I / 4)
            }
        } else if ax <= 3 * SCALE_I {
            body_piece!(&NORM_CDF_25_30_Q23, 11 * SCALE_I / 4, SCALE_I / 4)
        } else {
            body_piece!(&NORM_CDF_30_35_Q23, 13 * SCALE_I / 4, SCALE_I / 4)
        }
    } else if ax <= 5 * SCALE_I {
        if ax <= 4 * SCALE_I {
            body_piece!(&NORM_CDF_35_40_Q23, 15 * SCALE_I / 4, SCALE_I / 4)
        } else if ax <= 9 * SCALE_I / 2 {
            body_piece!(&NORM_CDF_40_45_Q23, 17 * SCALE_I / 4, SCALE_I / 4)
        } else {
            body_piece!(&NORM_CDF_45_50_Q23, 19 * SCALE_I / 4, SCALE_I / 4)
        }
    } else {
        norm_cdf_positive_tail_and_pdf(ax)
    };

    let cdf_pos = cdf_pos.clamp(0, SCALE_I);
    Ok((if x >= 0 { cdf_pos } else { SCALE_I - cdf_pos }, pdf))
}

/// Combined normal CDF and PDF: returns `(Phi(x), phi(x))` at SCALE.
///
/// The CDF uses its direct piecewise polynomial in every interval. The PDF
/// performs the only exponential evaluation needed by this fused call.
///
/// - **x**: signed fixed-point at `SCALE` (1e12).
/// - **Returns**: `(i128, i128)` — `(CDF, PDF)` both at `SCALE`. CDF in [0, SCALE_I].
/// - **Errors**: `Overflow` on internal arithmetic overflow.
/// - **Accuracy**: CDF is bit-identical to `norm_cdf_poly`; PDF is
///   bit-identical to `norm_pdf` (2 ULP max on the retained corpora).
pub fn norm_cdf_and_pdf(x: i128) -> Result<(i128, i128), SolMathError> {
    Ok((norm_cdf_poly(x)?, norm_pdf(x)?))
}

#[cfg(test)]
mod cdf_tests {
    use super::*;

    #[test]
    fn norm_cdf_is_symmetric_at_all_piece_seams() {
        let seams = [
            0,
            SCALE_I / 2,
            SCALE_I,
            3 * SCALE_I / 2,
            2 * SCALE_I,
            5 * SCALE_I / 2,
            3 * SCALE_I,
            7 * SCALE_I / 2,
            4 * SCALE_I,
            9 * SCALE_I / 2,
            5 * SCALE_I,
            11 * SCALE_I / 2,
            6 * SCALE_I,
            13 * SCALE_I / 2,
            7 * SCALE_I,
            NORM_TAIL_HALF_RAW_CUTOFF,
            8 * SCALE_I,
        ];
        for seam in seams {
            for offset in -4_096..=4_096 {
                let x = seam + offset;
                let positive = norm_cdf_poly(x).unwrap();
                let negative = norm_cdf_poly(-x).unwrap();
                assert_eq!(positive + negative, SCALE_I, "symmetry failed at {x}");
            }
        }
    }

    #[test]
    fn norm_cdf_is_monotone_at_seams_and_across_domain() {
        let seams = [
            -8 * SCALE_I,
            -NORM_TAIL_HALF_RAW_CUTOFF,
            -7 * SCALE_I,
            -13 * SCALE_I / 2,
            -6 * SCALE_I,
            -11 * SCALE_I / 2,
            -5 * SCALE_I,
            -9 * SCALE_I / 2,
            -4 * SCALE_I,
            -7 * SCALE_I / 2,
            -3 * SCALE_I,
            -5 * SCALE_I / 2,
            -2 * SCALE_I,
            -3 * SCALE_I / 2,
            -SCALE_I,
            -SCALE_I / 2,
            0,
            SCALE_I / 2,
            SCALE_I,
            3 * SCALE_I / 2,
            2 * SCALE_I,
            5 * SCALE_I / 2,
            3 * SCALE_I,
            7 * SCALE_I / 2,
            4 * SCALE_I,
            9 * SCALE_I / 2,
            5 * SCALE_I,
            11 * SCALE_I / 2,
            6 * SCALE_I,
            13 * SCALE_I / 2,
            7 * SCALE_I,
            NORM_TAIL_HALF_RAW_CUTOFF,
            8 * SCALE_I,
        ];
        for seam in seams {
            let mut previous = norm_cdf_poly(seam - 4_096).unwrap();
            for x in (seam - 4_095)..=(seam + 4_096) {
                let value = norm_cdf_poly(x).unwrap();
                assert!(
                    value >= previous,
                    "local inversion at {x}: {value} < {previous}"
                );
                previous = value;
            }
        }

        let step = 20_000_003i128;
        let mut x = -8 * SCALE_I;
        let mut previous = norm_cdf_poly(x).unwrap();
        while x < 8 * SCALE_I {
            x = (x + step).min(8 * SCALE_I);
            let value = norm_cdf_poly(x).unwrap();
            assert!(
                value >= previous,
                "domain inversion at {x}: {value} < {previous}"
            );
            previous = value;
        }
    }

    #[test]
    fn norm_cdf_q23_guard_prevents_upper_body_rounding_reversal() {
        let cases = [
            (4_971_877_961_316, 999_999_668_462),
            (4_971_877_961_317, 999_999_668_462),
            (4_971_877_961_318, 999_999_668_462),
            (4_971_877_961_319, 999_999_668_463),
        ];
        let mut previous = 0;
        for (x, expected) in cases {
            let actual = norm_cdf_poly(x).unwrap();
            assert_eq!(actual, expected, "Q23 regression changed at {x}");
            assert!(actual >= previous, "Q23 rounding reversal at {x}");
            previous = actual;
        }
    }

    #[test]
    fn norm_cdf_q39_tail_guard_prevents_rounding_reversal() {
        let cases = [
            (5_447_923_820_214, 999_999_974_519),
            (5_447_923_820_215, 999_999_974_519),
            (5_447_923_820_216, 999_999_974_520),
            (5_447_923_820_217, 999_999_974_520),
            (5_447_923_820_218, 999_999_974_520),
        ];
        let mut previous = 0;
        for (x, expected) in cases {
            let actual = norm_cdf_poly(x).unwrap();
            assert_eq!(actual, expected, "Q39 tail regression changed at {x}");
            assert!(actual >= previous, "Q39 tail rounding reversal at {x}");
            previous = actual;
        }
    }

    #[test]
    fn fused_cdf_pdf_uses_public_kernels_exactly() {
        for x in (-9_000..=9_000).step_by(17) {
            let x = i128::from(x) * 1_000_000_000;
            assert_eq!(
                norm_cdf_and_pdf(x).unwrap(),
                (norm_cdf_poly(x).unwrap(), norm_pdf(x).unwrap())
            );
        }
    }

    #[test]
    fn polynomial_pdf_reuses_cdf_exactly_and_tracks_density() {
        let mut x = -8 * SCALE_I;
        while x <= 8 * SCALE_I {
            let (cdf, pdf) = norm_cdf_and_pdf_poly(x).unwrap();
            assert_eq!(cdf, norm_cdf_poly(x).unwrap(), "CDF changed at {x}");
            let reference = norm_pdf(x).unwrap();
            assert!(
                pdf.abs_diff(reference) <= 600,
                "polynomial PDF gap at {x}: {pdf} vs {reference}"
            );
            x += 1_000_000_000;
        }

        for seam_half in 0..=14 {
            let seam = i128::from(seam_half) * SCALE_I / 2;
            for offset in [-4_096, -1, 0, 1, 4_096] {
                let x = seam + offset;
                let (cdf, pdf) = norm_cdf_and_pdf_poly(x).unwrap();
                assert_eq!(cdf, norm_cdf_poly(x).unwrap(), "CDF seam changed at {x}");
                assert!(pdf.abs_diff(norm_pdf(x).unwrap()) <= 600);
            }
        }
    }

    #[test]
    fn norm_cdf_retains_two_ulp_regressions() {
        let cases = [
            (499_999_999_979, 691_462_461_267),
            (-4_079, 499_999_998_373),
            (-1_491_753_830_411, 67_881_845_760),
            (-5_754_403_847_342, 4_347),
        ];
        for (x, expected) in cases {
            let actual = norm_cdf_poly(x).unwrap();
            assert!(
                actual.abs_diff(expected) <= 2,
                "CDF regression at {x}: {actual} vs {expected}"
            );
        }
    }
}

/// BS-guarded CDF+PDF: short-circuits to (0,0)/(SCALE,0) beyond ±8σ. Internal.
#[cfg(feature = "bs")]
pub(crate) fn norm_cdf_and_pdf_bs_guarded(x: i128) -> Result<(i128, i128), SolMathError> {
    if x <= -8 * SCALE_I {
        return Ok((0, 0));
    }
    if x >= 8 * SCALE_I {
        return Ok((SCALE_I, 0));
    }
    norm_cdf_and_pdf(x)
}

/// Degree-7 Horner evaluation with rounding.
#[inline]
fn horner_7_round(c: &[i128; 8], r: i128) -> Result<i128, SolMathError> {
    let mut acc = c[7];
    // Each step: |r| ≤ ~1.6·SCALE_I (AS241 branch variables), so 7 Horner steps
    // scale the accumulator by at most ~1.6^7 ≈ 27×. AS241 coefficients reach
    // ~6.7e16, so partial sums stay < ~2e18 ≪ i128::MAX (1.7e38).
    acc = fp_mul_i_round(acc, r)? + c[6];
    acc = fp_mul_i_round(acc, r)? + c[5];
    acc = fp_mul_i_round(acc, r)? + c[4];
    acc = fp_mul_i_round(acc, r)? + c[3];
    acc = fp_mul_i_round(acc, r)? + c[2];
    acc = fp_mul_i_round(acc, r)? + c[1];
    acc = fp_mul_i_round(acc, r)? + c[0];
    Ok(acc)
}

/// Degree-7 Horner evaluation for denominator (constant term = SCALE_I).
#[inline]
fn horner_7_den_round(c: &[i128; 7], r: i128) -> Result<i128, SolMathError> {
    let mut acc = c[6];
    // Same bound as horner_7_round: |r| ≤ ~1.6·SCALE_I and AS241 coefficients
    // reach ~6.7e16, so partial sums stay < ~2e18 ≪ i128::MAX (1.7e38).
    acc = fp_mul_i_round(acc, r)? + c[5];
    acc = fp_mul_i_round(acc, r)? + c[4];
    acc = fp_mul_i_round(acc, r)? + c[3];
    acc = fp_mul_i_round(acc, r)? + c[2];
    acc = fp_mul_i_round(acc, r)? + c[1];
    acc = fp_mul_i_round(acc, r)? + c[0];
    // Final term: fp_mul_i_round ∈ [-SCALE_I, SCALE_I]; + SCALE_I: ∈ [0, 2·SCALE_I], fits i128.
    Ok(fp_mul_i_round(acc, r)? + SCALE_I)
}

/// Inverse normal CDF (quantile function): Phi^-1(p/SCALE) * SCALE.
///
/// AS241 algorithm: 3-branch rational polynomial, ~12 significant digits.
///
/// - **p**: signed fixed-point probability at `SCALE` (1e12), in the open interval `(0, SCALE_I)`.
/// - **Returns**: `i128` z-score at `SCALE` such that Phi(z) ~ p/SCALE.
/// - **Errors**: `DomainError` if `p <= 0` or `p >= SCALE_I`.
/// - **Accuracy**: max 6 ULP, P99 5 ULP, median 1 ULP, 25% exact.
pub fn inverse_norm_cdf(p: i128) -> Result<i128, SolMathError> {
    if p <= 0 || p >= SCALE_I {
        return Err(SolMathError::DomainError);
    }

    // p ∈ (0, SCALE_I); SCALE_I/2 is a compile-time constant ~5e11; q ∈ (-SCALE_I/2, SCALE_I/2), fits i128.
    let q = p - SCALE_I / 2;

    if q.abs() <= AS241_SPLIT1 {
        // Branch 1: near center (|q| ≤ 0.425)
        // AS241_CONST1 ≈ 0.18·SCALE_I; fp_mul_i result ∈ [0, 0.18·SCALE_I]; r ∈ [0, 0.18·SCALE_I], fits i128.
        let r = AS241_CONST1 - fp_mul_i(q, q)?;
        let num = horner_7_round(&AS241_A, r)?;
        let den = horner_7_den_round(&AS241_B, r)?;
        fp_div_i(fp_mul_i_round(q, num)?, den)
    } else {
        // Tail branches: use sqrt(-ln(min(u, 1-u)))
        // p ∈ (0, SCALE_I); SCALE_I - p ∈ (0, SCALE_I), fits i128.
        let tail = if q < 0 { p } else { SCALE_I - p };
        let ln_tail = ln_fixed_i(tail as u128)?;
        // tail ∈ (0, SCALE_I/2]; ln_fixed_i returns a negative value (log < 1); negation gives ∈ (0, ~40·SCALE_I], fits i128.
        let neg_ln = -ln_tail; // positive
        let r = fp_sqrt(neg_ln as u128)? as i128;

        let ret = if r < AS241_SPLIT_2B {
            // Branch 2: near tails (r < 3.0), r_adj ∈ [0, ~1.4]
            // r ∈ [0, 3·SCALE_I), AS241_CONST2 ≈ 1.6·SCALE_I; r_adj ∈ [-1.6·SCALE_I, 1.4·SCALE_I], fits i128.
            let r_adj = r - AS241_CONST2;
            let num = horner_7_round(&AS241_C, r_adj)?;
            let den = horner_7_den_round(&AS241_D, r_adj)?;
            fp_div_i(num, den)?
        } else if r < AS241_SPLIT_2C {
            // Branch 2b: r ∈ [3.0, 4.0), centered at 3.5, r_adj ∈ [-0.5, 0.5]
            // r ∈ [3·SCALE_I, 4·SCALE_I), AS241_CENTER_2B = 3.5·SCALE_I; r_adj ∈ [-0.5·SCALE_I, 0.5·SCALE_I], fits i128.
            let r_adj = r - AS241_CENTER_2B;
            let num = horner_7_round(&AS241_G, r_adj)?;
            let den = horner_7_den_round(&AS241_H, r_adj)?;
            fp_div_i(num, den)?
        } else if r < AS241_SPLIT2 {
            // Branch 2c: r ∈ [4.0, 5.0), centered at 4.5, r_adj ∈ [-0.5, 0.5]
            // r ∈ [4·SCALE_I, 5·SCALE_I), AS241_CENTER_2C = 4.5·SCALE_I; r_adj ∈ [-0.5·SCALE_I, 0.5·SCALE_I], fits i128.
            let r_adj = r - AS241_CENTER_2C;
            let num = horner_7_round(&AS241_I, r_adj)?;
            let den = horner_7_den_round(&AS241_J, r_adj)?;
            fp_div_i(num, den)?
        } else {
            // Branch 3: extreme tails (r ≥ 5.0), r_adj ∈ [0, ~0.26]
            // r ∈ [5·SCALE_I, ~8·SCALE_I), AS241_SPLIT2 = 5·SCALE_I; r_adj ∈ [0, ~3·SCALE_I], fits i128.
            let r_adj = r - AS241_SPLIT2;
            let num = horner_7_round(&AS241_E, r_adj)?;
            let den = horner_7_den_round(&AS241_F, r_adj)?;
            fp_div_i(num, den)?
        };

        // ret is a z-score ∈ [0, ~8·SCALE_I]; negation: ∈ (-8·SCALE_I, 0], fits i128.
        if q < 0 {
            Ok(-ret)
        } else {
            Ok(ret)
        }
    }
}
