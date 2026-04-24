//! # solmath
//!
//! Fixed-point financial math for Solana. Black-Scholes, Greeks, implied volatility,
//! fat-tail pricing, and weighted pool math — all in pure integer arithmetic.
//!
//! All values use `u128` or `i128` scaled by 1e12 (12 decimal places).
//! HP variants use 1e15 internally but accept and return 1e12 values.
//!
//! ```
//! # #[cfg(feature = "transcendental")]
//! # fn price() -> Result<(), solmath::SolMathError> {
//! use solmath::*;
//!
//! // 1.5 in fixed-point
//! let x: u128 = 1_500_000_000_000;
//! let ln_x = ln_fixed_i(x)?; // ≈ 0.405 * 1e12
//!
//! // Black-Scholes: S=100, K=100, r=5%, σ=20%, T=1yr
//! let s = 100 * SCALE;
//! let k = 100 * SCALE;
//! let r = 50_000_000_000u128;  // 0.05
//! let sigma = 200_000_000_000u128; // 0.20
//! let t = SCALE; // 1.0
//! let greeks = bs_full_hp(s, k, r, sigma, t)?;
//! // greeks.call, greeks.put, greeks.gamma, greeks.vega, ...
//! # let _ = ln_x;
//! # let _ = greeks;
//! # Ok(())
//! # }
//! ```
//!
//! Agrees with QuantLib's AnalyticEuropeanEngine to 10-14 significant figures.

#![forbid(unsafe_code)]
#![no_std]
#[cfg(test)]
extern crate alloc;
#[cfg(test)]
extern crate std;

// Core — always compiled
mod constants;
pub mod error;
pub mod arithmetic;
pub mod overflow;
pub mod mul_div;
pub mod double_word;
pub mod encoding;
mod utils;

// Transcendental bundle
#[cfg(feature = "transcendental")]
pub mod transcendental;
#[cfg(feature = "transcendental")]
pub mod trig;
#[cfg(feature = "transcendental")]
pub mod normal;
#[cfg(feature = "transcendental")]
pub mod hp;
#[cfg(feature = "transcendental")]
pub mod i64_math;

// Complex arithmetic
#[cfg(feature = "complex")]
pub mod complex;

// Black-Scholes
#[cfg(feature = "bs")]
pub mod bs;

// Implied volatility
#[cfg(feature = "iv")]
pub mod iv;

// Barrier options
#[cfg(feature = "barrier")]
pub mod barrier;

// NIG fat-tail pricing
#[cfg(feature = "nig")]
pub mod nig;

// Heston stochastic volatility
#[cfg(feature = "heston")]
pub mod heston;
#[cfg(feature = "heston")]
mod i64_cf;

// SABR stochastic volatility
#[cfg(feature = "sabr")]
pub mod sabr;

// Pool math
#[cfg(feature = "pool")]
pub mod pool;

// Bivariate normal CDF
#[cfg(feature = "bivariate")]
pub mod bvn_cdf;
#[cfg(feature = "bivariate")]
pub mod phi2table;

// ── Public API ──

// Constants & types
pub use constants::{SCALE, SCALE_I, BsFull, LN2_LO, LN2_HP_LO, LN_REMEZ_COEFFS, LN_REMEZ_HP_COEFFS};
pub use double_word::DoubleWord;
pub use error::SolMathError;
pub use encoding::{fp, fp_i};

// Arithmetic
pub use arithmetic::{
    fp_mul, fp_mul_i, fp_mul_round, fp_mul_i_round, fp_mul_i_round_dw,
    fp_div, fp_div_i, fp_div_round,
    fp_div_floor, fp_div_ceil,
    fp_sqrt,
};
pub use overflow::{
    checked_mul_div_i,
    checked_mul_div_floor_i,
    checked_mul_div_ceil_i,
};

// Integer mul-div (u64, no SCALE)
pub use mul_div::{mul_div_floor, mul_div_ceil, mul_div_floor_u128, mul_div_ceil_u128};

// Transcendentals
#[cfg(feature = "transcendental")]
pub use transcendental::{ln_fixed_i, exp_fixed_i, pow_fixed, pow_int, pow_fixed_i, expm1_fixed};

#[cfg(feature = "transcendental")]
pub use trig::{sin_fixed, cos_fixed, sincos_fixed};

#[cfg(feature = "transcendental")]
pub use normal::{norm_cdf_poly, norm_pdf, norm_cdf_and_pdf, inverse_norm_cdf};

#[cfg(feature = "transcendental")]
pub use hp::{
    ln_fixed_hp, exp_fixed_hp, pow_fixed_hp, pow_product_hp,
    norm_cdf_poly_hp, black_scholes_price_hp, bs_full_hp,
    fp_mul_hp_i, fp_mul_hp_u, fp_div_hp_safe,
};

#[cfg(feature = "transcendental")]
pub use i64_math::{nig_call_64, nig_put_64};

// Complex
#[cfg(feature = "complex")]
pub use complex::{Complex, complex_mul, complex_div, complex_exp, complex_sqrt};

// Black-Scholes
#[cfg(feature = "bs")]
pub use bs::{black_scholes_price, bs_full, bs_delta, bs_gamma, bs_vega, bs_theta, bs_rho};

// Implied volatility
#[cfg(feature = "iv")]
pub use iv::implied_vol;

// Barrier options
#[cfg(feature = "barrier")]
pub use barrier::{barrier_option, BarrierType, BarrierResult};

// NIG fat-tail pricing
#[cfg(feature = "nig")]
pub use nig::nig_call_price;

// Heston stochastic volatility
#[cfg(feature = "heston")]
pub use heston::heston_price;

// SABR stochastic volatility
#[cfg(feature = "sabr")]
pub use sabr::{
    sabr_implied_vol, sabr_price, sabr_greeks,
    SabrSmile, sabr_precompute, sabr_vol_at,
    sabr_z_over_chi_pade,
};

// Pool math
#[cfg(feature = "pool")]
pub use pool::{weighted_pool_swap, token_to_fp, fp_to_token_floor, fp_to_token_ceil};

// Bivariate normal CDF
#[cfg(feature = "bivariate")]
pub use bvn_cdf::{bvn_cdf, bvn_cdf_hp};
#[cfg(feature = "bivariate")]
pub use phi2table::Phi2Table;

#[cfg(all(test, feature = "full"))]
mod tests {
    use super::*;

    #[test]
    fn public_module_paths_are_available() {
        let price = crate::bs::black_scholes_price(
            100 * SCALE,
            100 * SCALE,
            50_000_000_000,
            200_000_000_000,
            SCALE,
        )
        .unwrap();
        let same_price = black_scholes_price(
            100 * SCALE,
            100 * SCALE,
            50_000_000_000,
            200_000_000_000,
            SCALE,
        )
        .unwrap();

        assert_eq!(price, same_price);
        assert_eq!(
            crate::barrier::BarrierType::DownAndOut,
            BarrierType::DownAndOut
        );
    }

    // ── Arithmetic errors ──

    #[test]
    fn fp_div_division_by_zero() {
        assert_eq!(fp_div(SCALE, 0), Err(SolMathError::DivisionByZero));
    }

    #[test]
    fn fp_div_i_division_by_zero() {
        assert_eq!(fp_div_i(SCALE_I, 0), Err(SolMathError::DivisionByZero));
    }

    #[test]
    fn fp_div_floor_division_by_zero() {
        assert_eq!(fp_div_floor(SCALE, 0), Err(SolMathError::DivisionByZero));
    }

    #[test]
    fn fp_div_ceil_division_by_zero() {
        assert_eq!(fp_div_ceil(SCALE, 0), Err(SolMathError::DivisionByZero));
    }

    #[test]
    fn checked_mul_div_i_division_by_zero() {
        assert_eq!(checked_mul_div_i(SCALE_I, SCALE_I, 0), Err(SolMathError::DivisionByZero));
    }

    #[test]
    fn checked_mul_div_i_overflow() {
        assert_eq!(checked_mul_div_i(i128::MAX, i128::MAX, 1), Err(SolMathError::Overflow));
    }

    #[test]
    fn black_scholes_rejects_values_above_i128_range() {
        assert_eq!(
            black_scholes_price(i128::MAX as u128 + 1, SCALE, 0, SCALE / 5, SCALE),
            Err(SolMathError::Overflow)
        );
    }

    #[test]
    fn implied_vol_rejects_values_above_i128_range() {
        assert_eq!(
            implied_vol(i128::MAX as u128 + 1, SCALE, SCALE, 0, SCALE),
            Err(SolMathError::Overflow)
        );
    }

    #[test]
    fn heston_rejects_values_above_i128_range() {
        assert_eq!(
            heston_price(
                i128::MAX as u128 + 1,
                SCALE,
                0,
                SCALE,
                SCALE / 25,
                SCALE,
                SCALE / 25,
                SCALE / 10,
                0,
            ),
            Err(SolMathError::Overflow)
        );
    }

    #[test]
    fn heston_rejects_invalid_rho() {
        assert_eq!(
            heston_price(
                100 * SCALE,
                100 * SCALE,
                SCALE / 20,
                SCALE,
                40_000_000_000,
                2 * SCALE,
                40_000_000_000,
                SCALE / 2,
                SCALE_I,
            ),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            heston_price(
                100 * SCALE,
                100 * SCALE,
                SCALE / 20,
                SCALE,
                40_000_000_000,
                2 * SCALE,
                40_000_000_000,
                SCALE / 2,
                -SCALE_I,
            ),
            Err(SolMathError::DomainError)
        );
    }

    #[test]
    fn heston_rejects_values_above_i64_cv_representable_range() {
        assert_eq!(
            heston_price(
                1_000_000_000_000_000_000_000_000_000_000u128,
                SCALE,
                0,
                SCALE,
                40_000_000_000,
                2 * SCALE,
                40_000_000_000,
                SCALE / 2,
                -700_000_000_000,
            ),
            Err(SolMathError::Overflow)
        );
    }

    #[test]
    fn nig_rejects_values_above_i128_range() {
        assert_eq!(
            nig_call_price(i128::MAX as u128 + 1, SCALE, 0, SCALE, 10 * SCALE, 0, SCALE),
            Err(SolMathError::Overflow)
        );
    }

    #[test]
    fn sabr_rejects_values_above_i128_range() {
        assert_eq!(
            sabr_implied_vol(i128::MAX as u128 + 1, SCALE, SCALE, SCALE / 5, SCALE / 2, 0, SCALE / 5),
            Err(SolMathError::Overflow)
        );
    }

    #[test]
    fn sabr_rejects_beta_above_scale() {
        assert_eq!(
            sabr_implied_vol(SCALE, SCALE, SCALE, SCALE / 5, SCALE + 1, 0, SCALE / 5),
            Err(SolMathError::DomainError)
        );
        assert!(matches!(
            sabr_precompute(SCALE, SCALE, SCALE / 5, SCALE + 1, 0, SCALE / 5),
            Err(SolMathError::DomainError)
        ));
    }

    #[test]
    fn sabr_rejects_invalid_rho() {
        assert_eq!(
            sabr_implied_vol(SCALE, SCALE, SCALE, SCALE / 5, SCALE / 2, SCALE_I, SCALE / 5),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            sabr_implied_vol(SCALE, SCALE, SCALE, SCALE / 5, SCALE / 2, -SCALE_I, SCALE / 5),
            Err(SolMathError::DomainError)
        );
        assert!(matches!(
            sabr_precompute(SCALE, SCALE, SCALE / 5, SCALE / 2, SCALE_I, SCALE / 5),
            Err(SolMathError::DomainError)
        ));
        assert!(matches!(
            sabr_precompute(SCALE, SCALE, SCALE / 5, SCALE / 2, -SCALE_I, SCALE / 5),
            Err(SolMathError::DomainError)
        ));
    }

    // ── Transcendental errors ──

    #[test]
    fn ln_zero_is_domain_error() {
        assert_eq!(ln_fixed_i(0), Err(SolMathError::DomainError));
    }

    #[test]
    fn ln_positive_is_ok() {
        assert!(ln_fixed_i(SCALE).is_ok());
        assert_eq!(ln_fixed_i(SCALE).unwrap(), 0); // ln(1) = 0
    }

    #[test]
    fn ln_worst_case_regression() {
        // Known worst adversarial input: x=14313, expected=-18062097621744
        let result = ln_fixed_i(14313).unwrap();
        let expected: i128 = -18062097621744;
        let error = (result - expected).abs();
        assert!(error <= 3, "Regression: expected ≤3 ULP, got {} ULP (result={}, expected={})",
            error, result, expected);
    }

    #[test]
    fn ln_compensated_spot_checks() {
        use crate::constants::LN2_I;

        // ln(1) = 0, exactly
        assert_eq!(ln_fixed_i(SCALE).unwrap(), 0);

        // ln(2) should be very close to LN2_I
        let ln2 = ln_fixed_i(2 * SCALE).unwrap();
        let ln2_err = (ln2 - LN2_I).abs();
        assert!(ln2_err <= 1, "ln(2) error: {} ULP", ln2_err);

        // ln(e) should be very close to SCALE
        // e * SCALE ≈ 2_718_281_828_459
        let ln_e = ln_fixed_i(2_718_281_828_459u128).unwrap();
        let ln_e_err = (ln_e - SCALE_I).abs();
        assert!(ln_e_err <= 2, "ln(e) error: {} ULP", ln_e_err);

        // Just above 1: ln(SCALE + 1) should be ≈ 1/SCALE * SCALE = 1
        let ln_near_1 = ln_fixed_i(SCALE + 1).unwrap();
        assert!(ln_near_1 >= 0 && ln_near_1 <= 2, "ln(1+eps) = {}", ln_near_1);
    }

    #[test]
    fn exp_overflow() {
        assert_eq!(exp_fixed_i(41 * SCALE_I), Err(SolMathError::Overflow));
    }

    #[test]
    fn exp_underflow_is_zero() {
        assert_eq!(exp_fixed_i(-41 * SCALE_I), Ok(0));
    }

    #[test]
    fn exp_zero_is_one() {
        assert_eq!(exp_fixed_i(0), Ok(SCALE_I));
    }

    #[test]
    fn pow_zero_to_zero_is_domain_error() {
        assert_eq!(pow_fixed(0, 0), Err(SolMathError::DomainError));
    }

    #[test]
    fn pow_zero_to_positive() {
        assert_eq!(pow_fixed(0, SCALE), Ok(0));
    }

    #[test]
    fn pow_positive_to_zero() {
        assert_eq!(pow_fixed(SCALE, 0), Ok(SCALE));
    }

    #[test]
    fn pow_one_to_anything() {
        assert_eq!(pow_fixed(SCALE, 5 * SCALE), Ok(SCALE));
    }

    #[test]
    fn pow_fixed_i_negative_fractional_is_domain_error() {
        // (-2)^0.5 is undefined in reals
        assert_eq!(pow_fixed_i(-2 * SCALE_I, SCALE_I / 2), Err(SolMathError::DomainError));
    }

    // ── HP transcendental errors ──

    #[test]
    fn ln_hp_zero_is_domain_error() {
        assert_eq!(ln_fixed_hp(0), Err(SolMathError::DomainError));
    }

    #[test]
    fn ln_hp_negative_is_domain_error() {
        assert_eq!(ln_fixed_hp(-1), Err(SolMathError::DomainError));
    }

    #[test]
    fn exp_hp_overflow() {
        let scale_hp: i128 = 1_000_000_000_000_000;
        assert_eq!(exp_fixed_hp(41 * scale_hp), Err(SolMathError::Overflow));
    }

    #[test]
    fn pow_hp_zero_to_zero_is_domain_error() {
        assert_eq!(pow_fixed_hp(0, 0), Err(SolMathError::DomainError));
    }

    #[test]
    fn pow_product_hp_zero_x_is_domain_error() {
        assert_eq!(pow_product_hp(0, SCALE / 2), Err(SolMathError::DomainError));
    }

    #[test]
    fn pow_product_hp_w_exceeds_scale_is_domain_error() {
        assert_eq!(pow_product_hp(SCALE, SCALE + 1), Err(SolMathError::DomainError));
    }

    #[test]
    fn fp_div_hp_safe_division_by_zero() {
        assert_eq!(fp_div_hp_safe(1_000_000_000_000_000, 0), Err(SolMathError::DivisionByZero));
    }

    #[test]
    fn fp_div_hp_safe_overflow_is_error() {
        assert_eq!(
            fp_div_hp_safe(170141183460469061590504039530768268312, 999999999999999),
            Err(SolMathError::Overflow)
        );
    }

    // ── Normal distribution (total functions, no errors) ──

    #[test]
    fn norm_cdf_at_zero_is_half() {
        assert_eq!(norm_cdf_poly(0).unwrap(), SCALE_I / 2);
    }

    #[test]
    fn norm_pdf_at_zero() {
        let pdf = norm_pdf(0).unwrap();
        // φ(0) = 1/√(2π) ≈ 0.398942... at SCALE = 398_942_280_401
        assert!((pdf - 398_942_280_401).abs() < 100);
    }

    // ── Inverse normal CDF ──

    #[test]
    fn inverse_norm_cdf_at_half_is_zero() {
        let z = inverse_norm_cdf(SCALE_I / 2).unwrap();
        assert_eq!(z, 0);
    }

    #[test]
    fn inverse_norm_cdf_domain_errors() {
        assert_eq!(inverse_norm_cdf(0), Err(SolMathError::DomainError));
        assert_eq!(inverse_norm_cdf(SCALE_I), Err(SolMathError::DomainError));
        assert_eq!(inverse_norm_cdf(-1), Err(SolMathError::DomainError));
    }

    #[test]
    fn inverse_norm_cdf_symmetry() {
        // Φ⁻¹(0.25) = -Φ⁻¹(0.75)
        let z_lo = inverse_norm_cdf(250_000_000_000).unwrap();
        let z_hi = inverse_norm_cdf(750_000_000_000).unwrap();
        assert!((z_lo + z_hi).abs() < 10, "asymmetry: {} + {} = {}", z_lo, z_hi, z_lo + z_hi);
    }

    #[test]
    fn inverse_norm_cdf_roundtrip() {
        // Φ(Φ⁻¹(p)) ≈ p for a range of probabilities
        let probs: &[i128] = &[
            1_000_000_000,     // 0.001
            25_000_000_000,    // 0.025
            100_000_000_000,   // 0.1
            250_000_000_000,   // 0.25
            500_000_000_000,   // 0.5
            750_000_000_000,   // 0.75
            900_000_000_000,   // 0.9
            975_000_000_000,   // 0.975
            999_000_000_000,   // 0.999
        ];
        for &p in probs {
            let z = inverse_norm_cdf(p).unwrap();
            let p_back = norm_cdf_poly(z).unwrap();
            let err = (p_back - p).abs();
            assert!(err <= 100,
                "roundtrip failed for p={}: z={}, Φ(z)={}, err={}",
                p, z, p_back, err);
        }
    }

    #[test]
    fn inverse_norm_cdf_known_values() {
        // Φ⁻¹(0.975) ≈ 1.95996 (z-score for 97.5th percentile)
        let z = inverse_norm_cdf(975_000_000_000).unwrap();
        let expected = 1_959_963_984_540i128; // 1.95996398454 at SCALE
        assert!((z - expected).abs() < 1000,
            "Φ⁻¹(0.975) = {} (expected ~{})", z, expected);
    }

    // ── Black-Scholes errors ──

    #[test]
    fn bs_full_sigma_zero_is_domain_error() {
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let r = 50_000_000_000u128;
        let t = SCALE;
        assert!(matches!(bs_full(s, k, r, 0, t), Err(SolMathError::DomainError)));
    }

    #[test]
    fn bs_full_t_zero_is_domain_error() {
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let r = 50_000_000_000u128;
        let sigma = 200_000_000_000u128;
        assert!(matches!(bs_full(s, k, r, sigma, 0), Err(SolMathError::DomainError)));
    }

    #[test]
    fn bs_full_s_zero_returns_put() {
        let k = 100 * SCALE;
        let r = 50_000_000_000u128;
        let sigma = 200_000_000_000u128;
        let t = SCALE;
        let result = bs_full(0, k, r, sigma, t).unwrap();
        assert_eq!(result.call, 0);
        assert!(result.put > 0);
    }

    #[test]
    fn bs_full_hp_sigma_zero_is_domain_error() {
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let r = 50_000_000_000u128;
        let t = SCALE;
        assert!(matches!(bs_full_hp(s, k, r, 0, t), Err(SolMathError::DomainError)));
    }

    #[test]
    fn bs_full_hp_valid_inputs() {
        let s = 100 * SCALE;
        let k = 105 * SCALE;
        let r = 50_000_000_000u128;
        let sigma = 200_000_000_000u128;
        let t = SCALE;
        let result = bs_full_hp(s, k, r, sigma, t).unwrap();
        assert!(result.call > 0);
        assert!(result.put > 0);
    }

    // ── HP price-only ──

    #[test]
    fn black_scholes_price_hp_matches_full() {
        let cases = [
            (100 * SCALE, 105 * SCALE, 50_000_000_000u128, 250_000_000_000u128, SCALE),
            (100 * SCALE, 90 * SCALE, 50_000_000_000u128, 300_000_000_000u128, SCALE / 4),
            (100 * SCALE, 100 * SCALE, 50_000_000_000u128, 200_000_000_000u128, SCALE * 2),
        ];

        for (s, k, r, sigma, t) in cases {
            let (hp_call, hp_put) = black_scholes_price_hp(s, k, r, sigma, t).unwrap();
            let full = bs_full_hp(s, k, r, sigma, t).unwrap();

            assert_eq!(hp_call, full.call, "Call mismatch for s={} k={}", s, k);
            assert_eq!(hp_put, full.put, "Put mismatch for s={} k={}", s, k);
        }
    }

    #[test]
    fn black_scholes_price_hp_degenerate() {
        let r = 50_000_000_000u128;
        let sigma = 250_000_000_000u128;
        let t = SCALE;

        // s=0: call=0, put=K*disc
        let (call, put) = black_scholes_price_hp(0, 100 * SCALE, r, sigma, t).unwrap();
        assert_eq!(call, 0);
        assert!(put > 0);

        // k=0: call=S, put=0
        let (call, put) = black_scholes_price_hp(100 * SCALE, 0, r, sigma, t).unwrap();
        assert_eq!(call, 100 * SCALE);
        assert_eq!(put, 0);
    }

    #[test]
    fn black_scholes_price_hp_domain_errors() {
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let r = 50_000_000_000u128;
        assert!(matches!(black_scholes_price_hp(s, k, r, 0, SCALE), Err(SolMathError::DomainError)));
        assert!(matches!(black_scholes_price_hp(s, k, r, 200_000_000_000, 0), Err(SolMathError::DomainError)));
    }

    // ── Implied volatility errors ──

    #[test]
    fn implied_vol_zero_inputs_is_domain_error() {
        assert_eq!(implied_vol(0, SCALE, SCALE, 0, SCALE), Err(SolMathError::DomainError));
        assert_eq!(implied_vol(SCALE, 0, SCALE, 0, SCALE), Err(SolMathError::DomainError));
        assert_eq!(implied_vol(SCALE, SCALE, 0, 0, SCALE), Err(SolMathError::DomainError));
        assert_eq!(implied_vol(SCALE, SCALE, SCALE, 0, 0), Err(SolMathError::DomainError));
    }

    #[test]
    fn implied_vol_atm_roundtrip() {
        // ATM: S=K=100, r=5%, σ=20%, T=1yr
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let r = 50_000_000_000u128;  // 0.05
        let sigma_in = 200_000_000_000u128; // 0.20
        let t = SCALE; // 1.0
        let bs = bs_full(s, k, r, sigma_in, t).unwrap();
        let sigma_out = implied_vol(bs.call, s, k, r, t).unwrap();
        let err = if sigma_out > sigma_in { sigma_out - sigma_in } else { sigma_in - sigma_out };
        assert!(err <= 1000, "ATM IV roundtrip: in={} out={} err={}", sigma_in, sigma_out, err);
    }

    #[test]
    fn implied_vol_otm_roundtrip() {
        // OTM call: S=100, K=110, r=5%, σ=25%, T=0.5yr
        let s = 100 * SCALE;
        let k = 110 * SCALE;
        let r = 50_000_000_000u128;
        let sigma_in = 250_000_000_000u128;
        let t = SCALE / 2;
        let bs = bs_full(s, k, r, sigma_in, t).unwrap();
        let sigma_out = implied_vol(bs.call, s, k, r, t).unwrap();
        let err = if sigma_out > sigma_in { sigma_out - sigma_in } else { sigma_in - sigma_out };
        assert!(err <= 1000, "OTM IV roundtrip: in={} out={} err={}", sigma_in, sigma_out, err);
    }

    #[test]
    fn implied_vol_itm_roundtrip() {
        // ITM call: S=100, K=90, r=5%, σ=30%, T=1yr
        let s = 100 * SCALE;
        let k = 90 * SCALE;
        let r = 50_000_000_000u128;
        let sigma_in = 300_000_000_000u128;
        let t = SCALE;
        let bs = bs_full(s, k, r, sigma_in, t).unwrap();
        let sigma_out = implied_vol(bs.call, s, k, r, t).unwrap();
        let err = if sigma_out > sigma_in { sigma_out - sigma_in } else { sigma_in - sigma_out };
        assert!(err <= 1000, "ITM IV roundtrip: in={} out={} err={}", sigma_in, sigma_out, err);
    }

    #[test]
    fn implied_vol_batch_roundtrip() {
        // Test a grid of scenarios
        let spots = [100 * SCALE];
        let strikes = [80 * SCALE, 90 * SCALE, 100 * SCALE, 110 * SCALE, 120 * SCALE];
        let sigmas = [100_000_000_000u128, 200_000_000_000, 500_000_000_000, 1_000_000_000_000];
        let times = [SCALE / 10, SCALE / 4, SCALE / 2, SCALE];
        let rate = 50_000_000_000u128;

        let mut pass = 0u32;
        let mut total = 0u32;

        for &s in &spots {
            for &k in &strikes {
                for &sigma_in in &sigmas {
                    for &t in &times {
                        total += 1;
                        let bs = match bs_full(s, k, rate, sigma_in, t) {
                            Ok(v) => v,
                            Err(_) => { continue; }
                        };
                        if bs.call < 2 { continue; }
                        match implied_vol(bs.call, s, k, rate, t) {
                            Ok(sigma_out) => {
                                let err = if sigma_out > sigma_in {
                                    sigma_out - sigma_in
                                } else {
                                    sigma_in - sigma_out
                                };
                                if err <= 1000 { pass += 1; }
                            }
                            Err(_) => {}
                        }
                    }
                }
            }
        }
        let pct = pass as f64 / total as f64 * 100.0;
        assert!(pct >= 90.0,
            "IV batch roundtrip: {}/{} passed ({:.1}%), need ≥90%", pass, total, pct);
    }

    #[test]
    fn implied_vol_vector_recovery() {
        use std::vec::Vec;

        // Load iv_vectors.json and report recovery by difficulty
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("benchmark/iv_vectors.json");
        let data = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Cannot read {:?}", path));
        let parsed: serde_json::Value = serde_json::from_str(&data).unwrap();
        let vectors = parsed["vectors"].as_array().unwrap();

        let mut pass_easy = 0u32; let mut total_easy = 0u32;
        let mut pass_mod = 0u32; let mut total_mod = 0u32;
        let mut pass_hard = 0u32; let mut total_hard = 0u32;
        let mut max_ulp: u128 = 0;
        let mut ulps: Vec<u128> = Vec::new();

        for v in vectors {
            let s = v["s"].as_u64().unwrap() as u128;
            let k = v["k"].as_u64().unwrap() as u128;
            let r = v["r"].as_u64().unwrap() as u128;
            let t_val = v["t"].as_u64().unwrap() as u128;
            let sigma_in = v["sigma"].as_u64().unwrap() as u128;
            let call_price = v["call_price"].as_u64().unwrap() as u128;
            let difficulty = v["difficulty"].as_str().unwrap();
            let root_tag = v.get("root_tag").and_then(|rt| rt.as_str()).unwrap_or("");

            // Skip expected failures and zero-price
            if difficulty == "expected_failure" || root_tag == "zero_price" || root_tag == "intrinsic_only" {
                continue;
            }

            let (total_ref, pass_ref) = match difficulty {
                "easy" => (&mut total_easy, &mut pass_easy),
                "moderate" => (&mut total_mod, &mut pass_mod),
                _ => (&mut total_hard, &mut pass_hard),
            };
            *total_ref += 1;

            match implied_vol(call_price, s, k, r, t_val) {
                Ok(sigma_out) => {
                    let ulp = if sigma_out > sigma_in {
                        sigma_out - sigma_in
                    } else {
                        sigma_in - sigma_out
                    };
                    ulps.push(ulp);
                    if ulp > max_ulp { max_ulp = ulp; }
                    if ulp <= 1000 { *pass_ref += 1; }
                }
                Err(_) => {}
            }
        }

        let total = total_easy + total_mod + total_hard;
        let pass = pass_easy + pass_mod + pass_hard;

        #[cfg(feature = "pade-iv")]
        let label = "Pade [4/4]";
        #[cfg(not(feature = "pade-iv"))]
        let label = "Li bivariate";

        std::eprintln!("\n=== IV Vector Recovery ({}) ===", label);
        std::eprintln!("  Easy:     {}/{}", pass_easy, total_easy);
        std::eprintln!("  Moderate: {}/{}", pass_mod, total_mod);
        std::eprintln!("  Hard:     {}/{}", pass_hard, total_hard);
        std::eprintln!("  TOTAL:    {}/{}", pass, total);
        if !ulps.is_empty() {
            ulps.sort();
            let median = ulps[ulps.len() / 2];
            std::eprintln!("  Max ULP:  {}", max_ulp);
            std::eprintln!("  Med ULP:  {}", median);
            std::eprintln!("  P99 ULP:  {}", ulps[ulps.len() * 99 / 100]);
        }
        std::eprintln!("==============================\n");

        assert!(pass >= total * 85 / 100,
            "IV vector recovery: {}/{} ({:.1}%)", pass, total,
            pass as f64 / total as f64 * 100.0);
    }

    // ── NIG errors ──

    #[test]
    fn nig_zero_inputs_is_domain_error() {
        assert!(nig_call_price(0, SCALE, 0, SCALE, 10*SCALE, 0, SCALE).is_err());
        assert!(nig_call_price(SCALE, 0, 0, SCALE, 10*SCALE, 0, SCALE).is_err());
    }

    #[test]
    fn nig_invalid_alpha_beta_is_domain_error() {
        // alpha=1, beta=2 → α² < β², invalid
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let alpha = SCALE;
        let beta = 2 * SCALE_I;
        let delta = SCALE;
        assert_eq!(nig_call_price(s, k, 0, SCALE, alpha, beta, delta), Err(SolMathError::DomainError));
    }

    // ── Pool math errors ──

    #[test]
    fn pool_swap_zero_weight_out_is_division_by_zero() {
        assert_eq!(
            weighted_pool_swap(SCALE, SCALE, SCALE, 0, SCALE, 0),
            Err(SolMathError::DivisionByZero)
        );
    }

    #[test]
    fn pool_swap_zero_balance_is_domain_error() {
        assert_eq!(
            weighted_pool_swap(0, SCALE, SCALE, SCALE, SCALE, 0),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            weighted_pool_swap(SCALE, 0, SCALE, SCALE, SCALE, 0),
            Err(SolMathError::DomainError)
        );
    }

    #[test]
    fn pool_swap_zero_amount_is_ok_noop() {
        assert_eq!(
            weighted_pool_swap(SCALE, SCALE, SCALE, SCALE, 0, 0),
            Ok((0, 0))
        );
    }

    // ── Complex errors ──

    #[test]
    fn complex_div_by_zero_is_error() {
        let a = Complex::new(SCALE_I, 0);
        let b = Complex::new(0, 0);
        assert!(matches!(complex_div(a, b), Err(SolMathError::DivisionByZero)));
    }

    // ── Total function sanity checks ──

    #[test]
    fn sin_fixed_zero() {
        assert_eq!(sin_fixed(0).unwrap(), 0);
    }

    #[test]
    fn trig_handles_extreme_i128_inputs() {
        let sin_min = sin_fixed(i128::MIN).unwrap();
        let cos_min = cos_fixed(i128::MIN).unwrap();
        let (sin_pair, cos_pair) = sincos_fixed(i128::MIN).unwrap();
        assert_eq!(sin_min, sin_pair);
        assert_eq!(cos_min, cos_pair);
        assert!(sin_min.abs() <= SCALE_I);
        assert!(cos_min.abs() <= SCALE_I);

        let sin_max = sin_fixed(i128::MAX).unwrap();
        let cos_max = cos_fixed(i128::MAX).unwrap();
        let (sin_pair_max, cos_pair_max) = sincos_fixed(i128::MAX).unwrap();
        assert_eq!(sin_max, sin_pair_max);
        assert_eq!(cos_max, cos_pair_max);
        assert!(sin_max.abs() <= SCALE_I);
        assert!(cos_max.abs() <= SCALE_I);
    }

    #[test]
    fn cos_fixed_zero() {
        assert_eq!(cos_fixed(0).unwrap(), SCALE_I);
    }

    #[test]
    fn fp_sqrt_perfect_square() {
        // sqrt(4.0) = 2.0
        let result = fp_sqrt(4 * SCALE).unwrap();
        assert_eq!(result, 2 * SCALE);
    }

    #[test]
    fn fp_mul_overflows() {
        // i128::MAX * 2 should return Err(Overflow), not panic or saturate
        let result = fp_mul_i(i128::MAX, 2 * SCALE_I);
        assert!(result.is_err());
    }

    #[test]
    fn token_roundtrip() {
        // 1.5 USDC (6 decimals) = 1_500_000 lamports
        let fp = token_to_fp(1_500_000, 6).unwrap();
        assert_eq!(fp, 1_500_000_000_000); // 1.5 at SCALE
        assert_eq!(fp_to_token_floor(fp, 6).unwrap(), 1_500_000);
    }

    #[test]
    fn token_to_fp_truncates_for_decimals_above_scale_precision() {
        assert_eq!(token_to_fp(1, 13).unwrap(), 0);
        assert_eq!(token_to_fp(19, 13).unwrap(), 1);
    }

    // ── mul_div_floor / mul_div_ceil ──

    // Group 1: Basic arithmetic
    #[test]
    fn mul_div_floor_exact() { assert_eq!(mul_div_floor(10, 20, 5).unwrap(), 40); }
    #[test]
    fn mul_div_ceil_exact() { assert_eq!(mul_div_ceil(10, 20, 5).unwrap(), 40); }
    #[test]
    fn mul_div_floor_truncates() { assert_eq!(mul_div_floor(100, 200, 300).unwrap(), 66); }
    #[test]
    fn mul_div_ceil_rounds_up() { assert_eq!(mul_div_ceil(100, 200, 300).unwrap(), 67); }
    #[test]
    fn mul_div_floor_one_third() { assert_eq!(mul_div_floor(1, 1, 3).unwrap(), 0); }
    #[test]
    fn mul_div_ceil_one_third() { assert_eq!(mul_div_ceil(1, 1, 3).unwrap(), 1); }
    #[test]
    fn mul_div_floor_two_thirds() { assert_eq!(mul_div_floor(2, 1, 3).unwrap(), 0); }
    #[test]
    fn mul_div_ceil_two_thirds() { assert_eq!(mul_div_ceil(2, 1, 3).unwrap(), 1); }
    #[test]
    fn mul_div_floor_half() { assert_eq!(mul_div_floor(7, 3, 2).unwrap(), 10); }
    #[test]
    fn mul_div_ceil_half() { assert_eq!(mul_div_ceil(7, 3, 2).unwrap(), 11); }

    // Group 2: Identity and zero
    #[test]
    fn mul_div_floor_zero_a() { assert_eq!(mul_div_floor(0, 1000, 1).unwrap(), 0); }
    #[test]
    fn mul_div_ceil_zero_a() { assert_eq!(mul_div_ceil(0, 1000, 1).unwrap(), 0); }
    #[test]
    fn mul_div_floor_zero_b() { assert_eq!(mul_div_floor(1000, 0, 1).unwrap(), 0); }
    #[test]
    fn mul_div_ceil_zero_b() { assert_eq!(mul_div_ceil(1000, 0, 1).unwrap(), 0); }
    #[test]
    fn mul_div_floor_identity() { assert_eq!(mul_div_floor(1000, 1, 1).unwrap(), 1000); }
    #[test]
    fn mul_div_ceil_identity() { assert_eq!(mul_div_ceil(1000, 1, 1).unwrap(), 1000); }
    #[test]
    fn mul_div_floor_zero_zero() { assert_eq!(mul_div_floor(0, 0, 1).unwrap(), 0); }
    #[test]
    fn mul_div_ceil_zero_zero() { assert_eq!(mul_div_ceil(0, 0, 1).unwrap(), 0); }

    // Group 3: Division by zero
    #[test]
    fn mul_div_floor_div_by_zero() {
        assert_eq!(mul_div_floor(100, 200, 0), Err(SolMathError::DivisionByZero));
    }
    #[test]
    fn mul_div_ceil_div_by_zero() {
        assert_eq!(mul_div_ceil(100, 200, 0), Err(SolMathError::DivisionByZero));
    }
    #[test]
    fn mul_div_floor_zero_zero_zero() {
        assert_eq!(mul_div_floor(0, 0, 0), Err(SolMathError::DivisionByZero));
    }
    #[test]
    fn mul_div_ceil_zero_zero_zero() {
        assert_eq!(mul_div_ceil(0, 0, 0), Err(SolMathError::DivisionByZero));
    }

    // Group 4: Large values that would overflow u64 without u128
    #[test]
    fn mul_div_floor_large_cancelling() {
        assert_eq!(mul_div_floor(10_u64.pow(18), 10_u64.pow(12), 10_u64.pow(12)).unwrap(), 10_u64.pow(18));
    }
    #[test]
    fn mul_div_ceil_large_cancelling() {
        assert_eq!(mul_div_ceil(10_u64.pow(18), 10_u64.pow(12), 10_u64.pow(12)).unwrap(), 10_u64.pow(18));
    }
    #[test]
    fn mul_div_floor_max_identity() { assert_eq!(mul_div_floor(u64::MAX, 1, 1).unwrap(), u64::MAX); }
    #[test]
    fn mul_div_ceil_max_identity() { assert_eq!(mul_div_ceil(u64::MAX, 1, 1).unwrap(), u64::MAX); }
    #[test]
    fn mul_div_floor_max_times_2_div_2() { assert_eq!(mul_div_floor(u64::MAX, 2, 2).unwrap(), u64::MAX); }
    #[test]
    fn mul_div_ceil_max_times_2_div_2() { assert_eq!(mul_div_ceil(u64::MAX, 2, 2).unwrap(), u64::MAX); }
    #[test]
    fn mul_div_floor_max_cubed() { assert_eq!(mul_div_floor(u64::MAX, u64::MAX, u64::MAX).unwrap(), u64::MAX); }

    // Group 5: Overflow detection
    #[test]
    fn mul_div_floor_overflow() {
        assert_eq!(mul_div_floor(u64::MAX, 2, 1), Err(SolMathError::Overflow));
    }
    #[test]
    fn mul_div_ceil_overflow() {
        assert_eq!(mul_div_ceil(u64::MAX, 2, 1), Err(SolMathError::Overflow));
    }
    #[test]
    fn mul_div_floor_overflow_max_sq() {
        assert_eq!(mul_div_floor(u64::MAX, u64::MAX, 1), Err(SolMathError::Overflow));
    }
    #[test]
    fn mul_div_ceil_overflow_max_sq() {
        assert_eq!(mul_div_ceil(u64::MAX, u64::MAX, 1), Err(SolMathError::Overflow));
    }

    // Group 6: Folio-specific values
    #[test]
    fn mul_div_ceil_proportional_join() {
        // B_k=10^12, desired_lp=50*10^9, supply=100*10^9
        assert_eq!(
            mul_div_ceil(1_000_000_000_000_u64, 50_000_000_000_u64, 100_000_000_000_u64).unwrap(),
            500_000_000_000
        );
    }
    #[test]
    fn mul_div_floor_ratio_cap() {
        // 30% of 10^12 balance
        assert_eq!(mul_div_floor(1_000_000_000_000_u64, 30, 100).unwrap(), 300_000_000_000);
    }
    #[test]
    fn mul_div_floor_fee_calc() {
        // amount_in=10^8, (SCALE - 0.3% fee), SCALE
        assert_eq!(
            mul_div_floor(100_000_000_u64, 997_000_000_000_u64, 1_000_000_000_000_u64).unwrap(),
            99_700_000
        );
    }

    // ── mul_div_floor_u128 / mul_div_ceil_u128 ──

    #[test]
    fn mul_div_u128_basic() {
        assert_eq!(mul_div_floor_u128(10, 20, 5).unwrap(), 40);
        assert_eq!(mul_div_ceil_u128(10, 20, 5).unwrap(), 40);
        assert_eq!(mul_div_floor_u128(100, 200, 300).unwrap(), 66);
        assert_eq!(mul_div_ceil_u128(100, 200, 300).unwrap(), 67);
    }

    #[test]
    fn mul_div_u128_div_by_zero() {
        assert_eq!(mul_div_floor_u128(100, 200, 0), Err(SolMathError::DivisionByZero));
        assert_eq!(mul_div_ceil_u128(100, 200, 0), Err(SolMathError::DivisionByZero));
    }

    #[test]
    fn mul_div_u128_overflow_safe() {
        // 10^30 * 10^30 / 10^30 = 10^30, product overflows u128 but result fits
        let big = 10_u128.pow(30);
        assert_eq!(mul_div_floor_u128(big, big, big).unwrap(), big);
    }

    #[test]
    fn mul_div_u128_h_squared_over_s() {
        // H²/S barrier reflection: H=90*SCALE, S=100*SCALE → 81*SCALE
        let h = 90 * SCALE;
        let s = 100 * SCALE;
        assert_eq!(mul_div_floor_u128(h, h, s).unwrap(), 81 * SCALE);
    }

    #[test]
    fn mul_div_u128_max_identity() {
        assert_eq!(mul_div_floor_u128(u128::MAX, 1, 1).unwrap(), u128::MAX);
        assert_eq!(mul_div_ceil_u128(u128::MAX, 1, 1).unwrap(), u128::MAX);
    }

    #[test]
    fn mul_div_u128_max_times_2_div_2() {
        assert_eq!(mul_div_floor_u128(u128::MAX, 2, 2).unwrap(), u128::MAX);
        assert_eq!(mul_div_ceil_u128(u128::MAX, 2, 2).unwrap(), u128::MAX);
    }

    #[test]
    fn mul_div_u128_max_cubed() {
        assert_eq!(mul_div_floor_u128(u128::MAX, u128::MAX, u128::MAX).unwrap(), u128::MAX);
    }

    #[test]
    fn mul_div_u128_overflow() {
        assert_eq!(mul_div_floor_u128(u128::MAX, 2, 1), Err(SolMathError::Overflow));
        assert_eq!(mul_div_ceil_u128(u128::MAX, 2, 1), Err(SolMathError::Overflow));
        assert_eq!(mul_div_floor_u128(u128::MAX, u128::MAX, 1), Err(SolMathError::Overflow));
        assert_eq!(mul_div_ceil_u128(u128::MAX, u128::MAX, 1), Err(SolMathError::Overflow));
    }

    #[test]
    fn mul_div_u128_scale_values() {
        // Typical SCALE-valued operations
        assert_eq!(mul_div_floor_u128(100 * SCALE, 100 * SCALE, SCALE).unwrap(), 10000 * SCALE);
        assert_eq!(mul_div_floor_u128(SCALE, SCALE, SCALE).unwrap(), SCALE);
    }

    // ── Barrier options ──

    #[test]
    fn barrier_in_out_conservation_call() {
        // Down-and-in + Down-and-out = Vanilla
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let h = 90 * SCALE;
        let r = 50_000_000_000u128; // 5%
        let sigma = 250_000_000_000u128; // 25%
        let t = SCALE / 4; // 3 months

        let out = barrier_option(s, k, h, r, sigma, t, true, BarrierType::DownAndOut).unwrap();
        let in_ = barrier_option(s, k, h, r, sigma, t, true, BarrierType::DownAndIn).unwrap();

        let sum = out.price + in_.price;
        assert_eq!(sum, out.vanilla, "In/Out conservation: in={} out={} vanilla={}", in_.price, out.price, out.vanilla);
    }

    #[test]
    fn barrier_in_out_conservation_put() {
        // Up-and-in + Up-and-out = Vanilla (put)
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let h = 110 * SCALE;
        let r = 50_000_000_000u128;
        let sigma = 250_000_000_000u128;
        let t = SCALE / 2;

        let out = barrier_option(s, k, h, r, sigma, t, false, BarrierType::UpAndOut).unwrap();
        let in_ = barrier_option(s, k, h, r, sigma, t, false, BarrierType::UpAndIn).unwrap();

        let sum = out.price + in_.price;
        assert_eq!(sum, out.vanilla, "Put In/Out conservation: in={} out={} vanilla={}", in_.price, out.price, out.vanilla);
    }

    #[test]
    fn barrier_far_barrier_equals_vanilla() {
        // Down barrier very far from spot → barrier option ≈ vanilla
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let h = 10 * SCALE; // barrier at $10, spot at $100
        let r = 50_000_000_000u128;
        let sigma = 200_000_000_000u128;
        let t = SCALE / 4;

        let result = barrier_option(s, k, h, r, sigma, t, true, BarrierType::DownAndOut).unwrap();
        let diff = if result.price > result.vanilla {
            result.price - result.vanilla
        } else {
            result.vanilla - result.price
        };
        // Should be very close to vanilla (within 1%)
        assert!(diff * 100 < result.vanilla, "Far barrier should ≈ vanilla: price={} vanilla={}", result.price, result.vanilla);
    }

    #[test]
    fn barrier_knocked_out_at_barrier() {
        // Spot at barrier → down-and-out = 0
        let s = 90 * SCALE;
        let k = 100 * SCALE;
        let h = 90 * SCALE;
        let r = 50_000_000_000u128;
        let sigma = 250_000_000_000u128;
        let t = SCALE / 4;

        let result = barrier_option(s, k, h, r, sigma, t, true, BarrierType::DownAndOut).unwrap();
        assert_eq!(result.price, 0);
    }

    #[test]
    fn barrier_knocked_in_at_barrier() {
        // Spot at barrier → down-and-in = vanilla
        let s = 90 * SCALE;
        let k = 100 * SCALE;
        let h = 90 * SCALE;
        let r = 50_000_000_000u128;
        let sigma = 250_000_000_000u128;
        let t = SCALE / 4;

        let result = barrier_option(s, k, h, r, sigma, t, true, BarrierType::DownAndIn).unwrap();
        assert_eq!(result.price, result.vanilla);
    }

    #[test]
    fn barrier_out_less_than_vanilla() {
        // Knock-out is always ≤ vanilla
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let h = 90 * SCALE;
        let r = 50_000_000_000u128;
        let sigma = 250_000_000_000u128;
        let t = SCALE;

        let result = barrier_option(s, k, h, r, sigma, t, true, BarrierType::DownAndOut).unwrap();
        assert!(result.price <= result.vanilla, "Out should be ≤ vanilla: {} vs {}", result.price, result.vanilla);
        assert!(result.price > 0, "Out should be positive when spot is above barrier");
    }

    #[test]
    fn barrier_up_out_call_k_ge_h() {
        // Up-and-out call with K >= H is worthless
        let s = 100 * SCALE;
        let k = 120 * SCALE;
        let h = 110 * SCALE;
        let r = 50_000_000_000u128;
        let sigma = 250_000_000_000u128;
        let t = SCALE;

        let result = barrier_option(s, k, h, r, sigma, t, true, BarrierType::UpAndOut).unwrap();
        assert_eq!(result.price, 0);
    }

    #[test]
    fn barrier_down_out_put_k_le_h() {
        // Down-and-out put with K <= H is worthless
        let s = 100 * SCALE;
        let k = 85 * SCALE;
        let h = 90 * SCALE;
        let r = 50_000_000_000u128;
        let sigma = 250_000_000_000u128;
        let t = SCALE;

        let result = barrier_option(s, k, h, r, sigma, t, false, BarrierType::DownAndOut).unwrap();
        assert_eq!(result.price, 0);
    }

    #[test]
    fn barrier_domain_errors() {
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let h = 90 * SCALE;
        let r = 50_000_000_000u128;
        let sigma = 250_000_000_000u128;
        let t = SCALE;

        assert!(barrier_option(0, k, h, r, sigma, t, true, BarrierType::DownAndOut).is_err());
        assert!(barrier_option(s, 0, h, r, sigma, t, true, BarrierType::DownAndOut).is_err());
        assert!(barrier_option(s, k, 0, r, sigma, t, true, BarrierType::DownAndOut).is_err());
        assert!(barrier_option(s, k, h, r, 0, t, true, BarrierType::DownAndOut).is_err());
        assert!(barrier_option(s, k, h, r, sigma, 0, true, BarrierType::DownAndOut).is_err());
    }

    #[test]
    fn barrier_up_out_call_positive() {
        // Up-and-out call with K < H should have positive value
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let h = 120 * SCALE;
        let r = 50_000_000_000u128;
        let sigma = 200_000_000_000u128;
        let t = SCALE / 4;

        let result = barrier_option(s, k, h, r, sigma, t, true, BarrierType::UpAndOut).unwrap();
        assert!(result.price > 0, "Up-and-out call should be positive: {}", result.price);
        assert!(result.price <= result.vanilla, "Up-and-out ≤ vanilla");
    }

    #[test]
    fn barrier_up_in_out_conservation() {
        // Up-and-in call + Up-and-out call = vanilla
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let h = 115 * SCALE;
        let r = 50_000_000_000u128;
        let sigma = 300_000_000_000u128; // 30%
        let t = SCALE / 4;

        let out = barrier_option(s, k, h, r, sigma, t, true, BarrierType::UpAndOut).unwrap();
        let in_ = barrier_option(s, k, h, r, sigma, t, true, BarrierType::UpAndIn).unwrap();

        let sum = out.price + in_.price;
        assert_eq!(sum, out.vanilla, "Up call In/Out conservation: in={} out={} vanilla={}", in_.price, out.price, out.vanilla);
    }

    #[test]
    fn barrier_down_call_k_lt_h() {
        // Down-and-out call with K < H (K=95, H=105, S=110)
        // Survival means S_t > H > K so call is always ITM when surviving
        let s = 110 * SCALE;
        let k = 95 * SCALE;
        let h = 105 * SCALE;
        let r = 50_000_000_000u128;
        let sigma = 250_000_000_000u128;
        let t = SCALE / 2;

        let out = barrier_option(s, k, h, r, sigma, t, true, BarrierType::DownAndOut).unwrap();
        let in_ = barrier_option(s, k, h, r, sigma, t, true, BarrierType::DownAndIn).unwrap();

        assert!(out.price > 0, "Down-out call K<H should be positive");
        assert!(out.price <= out.vanilla);
        let sum = out.price + in_.price;
        let diff = if sum > out.vanilla { sum - out.vanilla } else { out.vanilla - sum };
        assert!(diff < 100, "Down K<H conservation: diff={}", diff);
    }

    #[test]
    fn barrier_down_put_k_gt_h() {
        // Down-and-out put with K > H (K=100, H=90, S=100)
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let h = 90 * SCALE;
        let r = 50_000_000_000u128;
        let sigma = 250_000_000_000u128;
        let t = SCALE / 2;

        let out = barrier_option(s, k, h, r, sigma, t, false, BarrierType::DownAndOut).unwrap();
        let in_ = barrier_option(s, k, h, r, sigma, t, false, BarrierType::DownAndIn).unwrap();

        assert!(out.price <= out.vanilla);
        let sum = out.price + in_.price;
        let diff = if sum > out.vanilla { sum - out.vanilla } else { out.vanilla - sum };
        assert!(diff < 100, "Down put K>H conservation: diff={}", diff);
    }

    #[test]
    fn barrier_up_put_k_lt_h() {
        // Up-and-out put with K < H (K=100, H=120, S=100)
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let h = 120 * SCALE;
        let r = 50_000_000_000u128;
        let sigma = 300_000_000_000u128;
        let t = SCALE / 2;

        let out = barrier_option(s, k, h, r, sigma, t, false, BarrierType::UpAndOut).unwrap();
        let in_ = barrier_option(s, k, h, r, sigma, t, false, BarrierType::UpAndIn).unwrap();

        assert!(out.price <= out.vanilla);
        let sum = out.price + in_.price;
        let diff = if sum > out.vanilla { sum - out.vanilla } else { out.vanilla - sum };
        assert!(diff < 100, "Up put K<H conservation: diff={}", diff);
    }

    #[test]
    fn barrier_up_put_k_gt_h() {
        // Up-and-out put with K > H (K=130, H=120, S=100)
        // Survival means S_t < H < K, so put always pays K - S_T > 0
        let s = 100 * SCALE;
        let k = 130 * SCALE;
        let h = 120 * SCALE;
        let r = 50_000_000_000u128;
        let sigma = 300_000_000_000u128;
        let t = SCALE / 2;

        let out = barrier_option(s, k, h, r, sigma, t, false, BarrierType::UpAndOut).unwrap();
        let in_ = barrier_option(s, k, h, r, sigma, t, false, BarrierType::UpAndIn).unwrap();

        assert!(out.price > 0, "Up-out put K>H should be positive: out={} in={} vanilla={}", out.price, in_.price, out.vanilla);
        assert!(out.price <= out.vanilla, "Up-out ({}) > vanilla ({}), in={}", out.price, out.vanilla, in_.price);
        let sum = out.price + in_.price;
        let diff = if sum > out.vanilla { sum - out.vanilla } else { out.vanilla - sum };
        assert!(diff < 100, "Up put K>H conservation: diff={}", diff);
    }
}

// ── Property-based tests for mul_div (u64) ──
#[cfg(test)]
mod mul_div_properties {
    use super::*;
    use proptest::prelude::*;

    /// Filter to non-overflow cases: c > 0 and result fits u64.
    fn non_overflow() -> impl Strategy<Value = (u64, u64, u64)> {
        (any::<u64>(), any::<u64>(), 1..=u64::MAX).prop_filter(
            "result must fit u64",
            |&(a, b, c)| {
                let product = (a as u128) * (b as u128);
                let floor = product / (c as u128);
                let ceil = (product + (c as u128) - 1) / (c as u128);
                floor <= u64::MAX as u128 && ceil <= u64::MAX as u128
            },
        )
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10_000))]

        // Property 1: floor <= ceil
        #[test]
        fn floor_le_ceil((a, b, c) in non_overflow()) {
            let f = mul_div_floor(a, b, c).unwrap();
            let ce = mul_div_ceil(a, b, c).unwrap();
            prop_assert!(f <= ce, "floor {} > ceil {}", f, ce);
        }

        // Property 2: ceil - floor <= 1
        #[test]
        fn ceil_minus_floor_le_1((a, b, c) in non_overflow()) {
            let f = mul_div_floor(a, b, c).unwrap();
            let ce = mul_div_ceil(a, b, c).unwrap();
            prop_assert!(ce - f <= 1, "ceil - floor = {} > 1", ce - f);
        }

        // Property 3: floor is correct lower bound
        #[test]
        fn floor_lower_bound((a, b, c) in non_overflow()) {
            let f = mul_div_floor(a, b, c).unwrap();
            let product = (a as u128) * (b as u128);
            let c128 = c as u128;
            // f * c <= a * b
            prop_assert!((f as u128) * c128 <= product);
            // (f + 1) * c > a * b
            prop_assert!((f as u128 + 1) * c128 > product);
        }

        // Property 4: ceil is correct upper bound
        #[test]
        fn ceil_upper_bound((a, b, c) in non_overflow()) {
            let ce = mul_div_ceil(a, b, c).unwrap();
            let product = (a as u128) * (b as u128);
            let c128 = c as u128;
            // ce * c >= a * b
            prop_assert!((ce as u128) * c128 >= product);
            // if ce > 0: (ce - 1) * c < a * b
            if ce > 0 {
                prop_assert!((ce as u128 - 1) * c128 < product);
            }
        }

        // Property 5: commutativity in a and b
        #[test]
        fn commutative((a, b, c) in non_overflow()) {
            prop_assert_eq!(mul_div_floor(a, b, c), mul_div_floor(b, a, c));
            prop_assert_eq!(mul_div_ceil(a, b, c), mul_div_ceil(b, a, c));
        }

        // Property 6: exact division means floor == ceil
        #[test]
        fn exact_div_floor_eq_ceil((a, b, c) in non_overflow()) {
            let product = (a as u128) * (b as u128);
            if product % (c as u128) == 0 {
                prop_assert_eq!(
                    mul_div_floor(a, b, c).unwrap(),
                    mul_div_ceil(a, b, c).unwrap()
                );
            }
        }
    }
}

// ── Property-based tests for mul_div_u128 ──
#[cfg(test)]
mod mul_div_u128_properties {
    use super::*;
    use proptest::prelude::*;

    /// Generate u128 triples where the result fits u128.
    /// Uses u64-range a,b (product fits u128) with any c > 0.
    fn non_overflow_small() -> impl Strategy<Value = (u128, u128, u128)> {
        (any::<u64>(), any::<u64>(), 1..=u64::MAX).prop_map(
            |(a, b, c)| (a as u128, b as u128, c as u128),
        )
    }

    /// Generate large u128 values that exercise the U256 overflow path.
    fn non_overflow_large() -> impl Strategy<Value = (u128, u128, u128)> {
        (1u128 << 64..1u128 << 96, 1u128 << 64..1u128 << 96, 1u128 << 64..1u128 << 96)
            .prop_filter("result must fit u128", |&(a, b, c)| {
                crate::overflow::checked_mul_div_rem_u(a, b, c).is_some()
            })
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10_000))]

        #[test]
        fn floor_le_ceil((a, b, c) in non_overflow_small()) {
            let f = mul_div_floor_u128(a, b, c).unwrap();
            let ce = mul_div_ceil_u128(a, b, c).unwrap();
            prop_assert!(f <= ce, "floor {} > ceil {}", f, ce);
            prop_assert!(ce - f <= 1, "ceil - floor = {} > 1", ce - f);
        }

        #[test]
        fn commutative((a, b, c) in non_overflow_small()) {
            prop_assert_eq!(mul_div_floor_u128(a, b, c), mul_div_floor_u128(b, a, c));
            prop_assert_eq!(mul_div_ceil_u128(a, b, c), mul_div_ceil_u128(b, a, c));
        }

        #[test]
        fn floor_le_ceil_large((a, b, c) in non_overflow_large()) {
            let f = mul_div_floor_u128(a, b, c).unwrap();
            let ce = mul_div_ceil_u128(a, b, c).unwrap();
            prop_assert!(f <= ce, "floor {} > ceil {}", f, ce);
            prop_assert!(ce - f <= 1, "ceil - floor = {} > 1", ce - f);
        }

        #[test]
        fn commutative_large((a, b, c) in non_overflow_large()) {
            prop_assert_eq!(mul_div_floor_u128(a, b, c), mul_div_floor_u128(b, a, c));
            prop_assert_eq!(mul_div_ceil_u128(a, b, c), mul_div_ceil_u128(b, a, c));
        }

        /// u128 variant must agree with u64 variant for u64-range inputs
        #[test]
        fn agrees_with_u64((a, b, c) in (any::<u64>(), any::<u64>(), 1..=u64::MAX).prop_filter(
            "result must fit u64",
            |&(a, b, c)| {
                let product = (a as u128) * (b as u128);
                product / (c as u128) <= u64::MAX as u128
                    && (product + (c as u128) - 1) / (c as u128) <= u64::MAX as u128
            }
        )) {
            let f64 = mul_div_floor(a, b, c).unwrap();
            let f128 = mul_div_floor_u128(a as u128, b as u128, c as u128).unwrap();
            prop_assert_eq!(f64 as u128, f128);

            let c64 = mul_div_ceil(a, b, c).unwrap();
            let c128 = mul_div_ceil_u128(a as u128, b as u128, c as u128).unwrap();
            prop_assert_eq!(c64 as u128, c128);
        }
    }
}

// ── Cross-validation against exact integer reference vectors ──
#[cfg(test)]
mod mul_div_cross_validation {
    use super::*;

    #[derive(serde::Deserialize)]
    struct Vector {
        a: u64,
        b: u64,
        c: u64,
        floor: u64,
        ceil: u64,
    }

    #[test]
    fn validate_against_exact_vectors() {
        let vectors = [
            Vector { a: 0, b: 0, c: 1, floor: 0, ceil: 0 },
            Vector { a: 0, b: u64::MAX, c: 1, floor: 0, ceil: 0 },
            Vector { a: u64::MAX, b: 1, c: 1, floor: u64::MAX, ceil: u64::MAX },
            Vector { a: u64::MAX, b: 2, c: 2, floor: u64::MAX, ceil: u64::MAX },
            Vector { a: 100, b: 200, c: 300, floor: 66, ceil: 67 },
            Vector { a: 1, b: 1, c: 3, floor: 0, ceil: 1 },
            Vector { a: 2, b: 1, c: 3, floor: 0, ceil: 1 },
            Vector { a: 7, b: 3, c: 2, floor: 10, ceil: 11 },
            Vector {
                a: 1_000_000_000_000,
                b: 50_000_000_000,
                c: 100_000_000_000,
                floor: 500_000_000_000,
                ceil: 500_000_000_000,
            },
            Vector {
                a: 10_u64.pow(18),
                b: 10_u64.pow(12),
                c: 10_u64.pow(12),
                floor: 10_u64.pow(18),
                ceil: 10_u64.pow(18),
            },
            Vector { a: u64::MAX, b: u64::MAX, c: u64::MAX, floor: u64::MAX, ceil: u64::MAX },
        ];

        for v in &vectors {
            let f = mul_div_floor(v.a, v.b, v.c).unwrap();
            let ce = mul_div_ceil(v.a, v.b, v.c).unwrap();
            if f != v.floor {
                panic!("FLOOR mismatch: a={} b={} c={} expected={} got={}", v.a, v.b, v.c, v.floor, f);
            }
            if ce != v.ceil {
                panic!("CEIL mismatch: a={} b={} c={} expected={} got={}", v.a, v.b, v.c, v.ceil, ce);
            }
        }
    }

    #[test]
    fn validate_against_full_python_vectors_when_requested() {
        if std::env::var("SOLMATH_FULL_VECTORS").ok().as_deref() != Some("1") {
            std::eprintln!("set SOLMATH_FULL_VECTORS=1 to run the full mul-div vector corpus");
            return;
        }

        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/reference/mul_div_vectors.json");
        let data = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Cannot read {:?}", path));
        let vectors: alloc::vec::Vec<Vector> = serde_json::from_str(&data).expect("parse vectors");

        for v in &vectors {
            let f = mul_div_floor(v.a, v.b, v.c).unwrap();
            let ce = mul_div_ceil(v.a, v.b, v.c).unwrap();
            if f != v.floor {
                panic!("FLOOR mismatch: a={} b={} c={} expected={} got={}", v.a, v.b, v.c, v.floor, f);
            }
            if ce != v.ceil {
                panic!("CEIL mismatch: a={} b={} c={} expected={} got={}", v.a, v.b, v.c, v.ceil, ce);
            }
        }

        assert!(!vectors.is_empty(), "full vector corpus should not be empty");
    }
}
