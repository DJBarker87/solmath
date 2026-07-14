//! # solmath
//!
//! Deterministic fixed-point mathematics and quantitative finance for Solana
//! and `no_std` Rust. The crate combines checked decimal arithmetic,
//! transcendentals, probability functions, Black-Scholes and Greeks, implied
//! volatility, barriers, arithmetic-Asian/TWAP settlement, American KBI,
//! exponential NIG, two-asset rainbow options, deterministic Heston, SABR,
//! and weighted-pool math behind one dependency-free API.
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
//! The HP Black-Scholes path agrees with QuantLib's AnalyticEuropeanEngine to
//! roughly 10-14 significant figures on non-tiny outputs in the reference corpus.

#![forbid(unsafe_code)]
#![no_std]
#[cfg(test)]
extern crate alloc;
#[cfg(test)]
extern crate std;

// Core — always compiled
pub mod arithmetic;
mod constants;
pub mod double_word;
pub mod encoding;
pub mod error;
#[cfg(feature = "transcendental")]
mod exp_coeffs;
#[cfg(feature = "transcendental")]
mod expm1_lut;
#[cfg(feature = "transcendental")]
mod ln2_lut;
#[cfg(feature = "transcendental")]
mod ln_lut;
#[cfg(feature = "transcendental")]
mod lut_budget;
pub mod mul_div;
#[cfg(feature = "transcendental")]
mod norm_cdf_coeffs;
pub mod overflow;
mod utils;

// Transcendental bundle
#[cfg(feature = "transcendental")]
pub mod hp;
#[cfg(feature = "nig")]
pub mod i64_math;
#[cfg(feature = "transcendental")]
pub mod normal;
#[cfg(feature = "transcendental")]
pub mod transcendental;
#[cfg(feature = "transcendental")]
pub mod trig;

// Complex arithmetic
#[cfg(feature = "complex")]
pub mod complex;

// Black-Scholes
#[cfg(feature = "bs")]
pub mod bs;

// Implied volatility
#[cfg(feature = "iv")]
pub mod iv;

// Safe-by-construction validated pricing inputs
#[cfg(any(
    feature = "bs",
    feature = "barrier",
    feature = "asian",
    feature = "pool"
))]
pub mod checked;

// Kim Boundary Integration (KBI): nonlinear exercise-boundary reconstruction
// followed by Kim's early-exercise-premium integral.
#[cfg(feature = "american-kbi")]
pub mod american_kbi;
#[cfg(feature = "american-kbi")]
#[allow(dead_code)]
mod american_kbi_data;

// Two-asset rainbow options
#[cfg(feature = "rainbow")]
pub mod rainbow;

// Barrier options
#[cfg(feature = "barrier")]
pub mod barrier;

// Continuous arithmetic-Asian / partially fixed TWAP options
#[cfg(feature = "asian")]
pub mod asian;

// Exponential NIG pricing on an explicitly bounded production domain.
#[cfg(feature = "nig")]
pub mod nig;

// Deterministic Heston limit (`xi = 0`).
#[cfg(feature = "heston")]
pub mod heston;
// Test-only characteristic-function research retained outside production code.
#[cfg(all(feature = "heston", feature = "complex", test))]
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
pub use constants::{
    BsFull, LN2_HP_LO, LN2_LO, LN_REMEZ_COEFFS, LN_REMEZ_HP_COEFFS, SCALE, SCALE_I,
};
pub use double_word::DoubleWord;
pub use encoding::{fp, fp_i};
pub use error::SolMathError;

// Arithmetic
pub use arithmetic::{
    fp_div, fp_div_ceil, fp_div_floor, fp_div_i, fp_div_round, fp_mul, fp_mul_i, fp_mul_i_round,
    fp_mul_i_round_dw, fp_mul_round, fp_sqrt,
};
pub use overflow::{checked_mul_div_ceil_i, checked_mul_div_floor_i, checked_mul_div_i};

// Integer mul-div (u64, no SCALE)
pub use mul_div::{mul_div_ceil, mul_div_ceil_u128, mul_div_floor, mul_div_floor_u128};

// Transcendentals
#[cfg(feature = "transcendental")]
pub use transcendental::{
    exp_fixed_i, expm1_fixed, ln_1p_fixed, ln_fixed_i, pow_fixed, pow_fixed_i, pow_int,
};

#[cfg(feature = "transcendental")]
pub use trig::{cos_fixed, sin_fixed, sincos_fixed};

#[cfg(feature = "transcendental")]
pub use normal::{
    inverse_norm_cdf, norm_cdf_and_pdf, norm_cdf_and_pdf_poly, norm_cdf_poly, norm_pdf,
};

#[cfg(feature = "transcendental")]
pub use hp::{
    black_scholes_price_hp, bs_full_hp, exp_fixed_hp, fp_div_hp_safe, fp_mul_hp_i, fp_mul_hp_u,
    ln_fixed_hp, norm_cdf_poly_hp, pow_fixed_hp, pow_product_hp,
};

#[cfg(feature = "nig")]
pub use i64_math::{nig_call_64, nig_put_64};

// Complex
#[cfg(feature = "complex")]
pub use complex::{complex_div, complex_exp, complex_mul, complex_sqrt, Complex};

// Black-Scholes
#[cfg(feature = "bs")]
pub use bs::{black_scholes_price, bs_delta, bs_full, bs_gamma, bs_rho, bs_theta, bs_vega};

// Safe-by-construction validated inputs (recommended program-boundary API)
#[cfg(feature = "barrier")]
pub use checked::BarrierInputs;
#[cfg(feature = "bs")]
pub use checked::EuropeanInputs;
#[cfg(feature = "iv")]
pub use checked::ImpliedVolInputs;
#[cfg(feature = "pool")]
pub use checked::PoolSwapInputs;
#[cfg(feature = "asian")]
pub use checked::TwapInputs;
#[cfg(any(feature = "bs", feature = "barrier", feature = "asian"))]
pub use checked::{Price, Rate, Time, Vol};

// Implied volatility
#[cfg(feature = "iv")]
pub use iv::implied_vol;

#[cfg(feature = "american-kbi")]
pub use american_kbi::{
    american_kbi_price, AmericanKbiKind, AMERICAN_KBI_ARTIFACT_SHA256, AMERICAN_KBI_NODES,
    AMERICAN_KBI_PRICE_POINTS,
};

// Rainbow (two-asset) options
#[cfg(feature = "rainbow")]
pub use rainbow::{best_of_call, worst_of_call};

// Barrier options
#[cfg(feature = "barrier")]
pub use barrier::{barrier_option, barrier_option_with_state, BarrierResult, BarrierType};

// Continuous arithmetic-Asian / partially fixed TWAP options
#[cfg(feature = "asian")]
pub use asian::{arithmetic_asian_price, twap_option_price, AsianOptionResult};

// Exponential NIG public pricing API
#[cfg(feature = "nig")]
pub use nig::{
    nig_call_price, nig_price_certified, CertifiedNigPrice, NigParams, NIG_MAX_ALPHA,
    NIG_MAX_DELTA_TIME, NIG_MIN_ALPHA, NIG_MIN_DELTA_TIME, NIG_QUADRATURE_NODES,
};

// Deterministic Heston API (positive-expiry `xi == 0`).
#[cfg(feature = "heston")]
pub use heston::heston_price;

// SABR stochastic volatility
#[cfg(feature = "sabr")]
pub use sabr::{
    certify_sabr_surface, sabr_greeks, sabr_implied_vol, sabr_precompute, sabr_price, sabr_vol_at,
    sabr_z_over_chi_pade, CertifiedSabrQuote, CertifiedSabrSurface, SabrSmile,
    MAX_SABR_SURFACE_MATURITIES, MAX_SABR_SURFACE_QUOTES, MAX_SABR_SURFACE_STRIKES,
};

// Pool math
#[cfg(feature = "pool")]
pub use pool::{fp_to_token_ceil, fp_to_token_floor, token_to_fp, weighted_pool_swap};

// Bivariate normal CDF
#[cfg(feature = "bivariate")]
pub use bvn_cdf::{bvn_cdf, bvn_cdf_hp};
#[cfg(feature = "bivariate")]
pub use phi2table::{
    CertifiedPhi2Evaluator, Phi2Certificate, Phi2DenseTable, Phi2Interpolation, Phi2Reference,
    Phi2Table, PHI2_DENSE_GRID_SIZE, PHI2_GRID_SIZE, PHI2_ROW_DIGEST_BYTES,
};

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
        assert_eq!(
            checked_mul_div_i(SCALE_I, SCALE_I, 0),
            Err(SolMathError::DivisionByZero)
        );
    }

    #[test]
    fn checked_mul_div_i_overflow() {
        assert_eq!(
            checked_mul_div_i(i128::MAX, i128::MAX, 1),
            Err(SolMathError::Overflow)
        );
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
    fn heston_stochastic_rejection_does_not_depend_on_private_research_limits() {
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
            Err(SolMathError::NoConvergence)
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
            sabr_implied_vol(
                i128::MAX as u128 + 1,
                SCALE,
                SCALE,
                SCALE / 5,
                SCALE / 2,
                0,
                SCALE / 5
            ),
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
            sabr_implied_vol(
                SCALE,
                SCALE,
                SCALE,
                SCALE / 5,
                SCALE / 2,
                SCALE_I,
                SCALE / 5
            ),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            sabr_implied_vol(
                SCALE,
                SCALE,
                SCALE,
                SCALE / 5,
                SCALE / 2,
                -SCALE_I,
                SCALE / 5
            ),
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
        assert!(
            error <= 2,
            "Regression: expected ≤2 ULP, got {} ULP (result={}, expected={})",
            error,
            result,
            expected
        );
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
        assert!(
            ln_near_1 >= 0 && ln_near_1 <= 2,
            "ln(1+eps) = {}",
            ln_near_1
        );
    }

    #[test]
    fn exp_overflow() {
        assert_eq!(exp_fixed_i(41 * SCALE_I), Err(SolMathError::Overflow));
    }

    #[test]
    fn exp_underflow_is_zero() {
        assert_eq!(exp_fixed_i(-41 * SCALE_I), Ok(0));
        assert_eq!(exp_fixed_i(-40 * SCALE_I), Ok(0));
    }

    #[test]
    fn exp_zero_is_one() {
        assert_eq!(exp_fixed_i(0), Ok(SCALE_I));
    }

    #[test]
    fn exp_tiny_direct_path_and_seams_are_correctly_rounded() {
        assert_eq!(exp_fixed_i(999_999), Ok(SCALE_I + 999_999));
        assert_eq!(exp_fixed_i(-999_999), Ok(SCALE_I - 999_999));
        assert_eq!(exp_fixed_i(1_000_000), Ok(SCALE_I + 1_000_001));
        assert_eq!(exp_fixed_i(-1_000_000), Ok(SCALE_I - 1_000_000));

        let mut previous = exp_fixed_i(-1_000_008).unwrap();
        for x in -1_000_007..=1_000_008 {
            let current = exp_fixed_i(x).unwrap();
            assert!(current >= previous, "exp reversed at raw input {x}");
            previous = current;
        }
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
        assert_eq!(
            pow_fixed_i(-2 * SCALE_I, SCALE_I / 2),
            Err(SolMathError::DomainError)
        );
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
        assert_eq!(
            pow_product_hp(SCALE, SCALE + 1),
            Err(SolMathError::DomainError)
        );
    }

    #[test]
    fn fp_div_hp_safe_division_by_zero() {
        assert_eq!(
            fp_div_hp_safe(1_000_000_000_000_000, 0),
            Err(SolMathError::DivisionByZero)
        );
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
        assert!(
            (z_lo + z_hi).abs() < 10,
            "asymmetry: {} + {} = {}",
            z_lo,
            z_hi,
            z_lo + z_hi
        );
    }

    #[test]
    fn inverse_norm_cdf_roundtrip() {
        // Φ(Φ⁻¹(p)) ≈ p for a range of probabilities
        let probs: &[i128] = &[
            1_000_000_000,   // 0.001
            25_000_000_000,  // 0.025
            100_000_000_000, // 0.1
            250_000_000_000, // 0.25
            500_000_000_000, // 0.5
            750_000_000_000, // 0.75
            900_000_000_000, // 0.9
            975_000_000_000, // 0.975
            999_000_000_000, // 0.999
        ];
        for &p in probs {
            let z = inverse_norm_cdf(p).unwrap();
            let p_back = norm_cdf_poly(z).unwrap();
            let err = (p_back - p).abs();
            assert!(
                err <= 100,
                "roundtrip failed for p={}: z={}, Φ(z)={}, err={}",
                p,
                z,
                p_back,
                err
            );
        }
    }

    #[test]
    fn inverse_norm_cdf_known_values() {
        // Φ⁻¹(0.975) ≈ 1.95996 (z-score for 97.5th percentile)
        let z = inverse_norm_cdf(975_000_000_000).unwrap();
        let expected = 1_959_963_984_540i128; // 1.95996398454 at SCALE
        assert!(
            (z - expected).abs() < 1000,
            "Φ⁻¹(0.975) = {} (expected ~{})",
            z,
            expected
        );
    }

    // ── Black-Scholes errors ──

    #[test]
    fn bs_full_sigma_zero_is_domain_error() {
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let r = 50_000_000_000u128;
        let t = SCALE;
        assert!(matches!(
            bs_full(s, k, r, 0, t),
            Err(SolMathError::DomainError)
        ));
    }

    #[test]
    fn bs_full_t_zero_is_domain_error() {
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let r = 50_000_000_000u128;
        let sigma = 200_000_000_000u128;
        assert!(matches!(
            bs_full(s, k, r, sigma, 0),
            Err(SolMathError::DomainError)
        ));
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
        assert!(matches!(
            bs_full_hp(s, k, r, 0, t),
            Err(SolMathError::DomainError)
        ));
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
            (
                100 * SCALE,
                105 * SCALE,
                50_000_000_000u128,
                250_000_000_000u128,
                SCALE,
            ),
            (
                100 * SCALE,
                90 * SCALE,
                50_000_000_000u128,
                300_000_000_000u128,
                SCALE / 4,
            ),
            (
                100 * SCALE,
                100 * SCALE,
                50_000_000_000u128,
                200_000_000_000u128,
                SCALE * 2,
            ),
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
        assert!(matches!(
            black_scholes_price_hp(s, k, r, 0, SCALE),
            Err(SolMathError::DomainError)
        ));
        assert!(matches!(
            black_scholes_price_hp(s, k, r, 200_000_000_000, 0),
            Err(SolMathError::DomainError)
        ));
    }

    // ── Implied volatility errors ──

    #[test]
    fn implied_vol_zero_inputs_is_domain_error() {
        assert_eq!(
            implied_vol(0, SCALE, SCALE, 0, SCALE),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            implied_vol(SCALE, 0, SCALE, 0, SCALE),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            implied_vol(SCALE, SCALE, 0, 0, SCALE),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            implied_vol(SCALE, SCALE, SCALE, 0, 0),
            Err(SolMathError::DomainError)
        );
    }

    #[test]
    fn implied_vol_atm_roundtrip() {
        // ATM: S=K=100, r=5%, σ=20%, T=1yr
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let r = 50_000_000_000u128; // 0.05
        let sigma_in = 200_000_000_000u128; // 0.20
        let t = SCALE; // 1.0
        let bs = bs_full(s, k, r, sigma_in, t).unwrap();
        let sigma_out = implied_vol(bs.call, s, k, r, t).unwrap();
        let err = if sigma_out > sigma_in {
            sigma_out - sigma_in
        } else {
            sigma_in - sigma_out
        };
        assert!(
            err <= 1000,
            "ATM IV roundtrip: in={} out={} err={}",
            sigma_in,
            sigma_out,
            err
        );
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
        let err = if sigma_out > sigma_in {
            sigma_out - sigma_in
        } else {
            sigma_in - sigma_out
        };
        assert!(
            err <= 1000,
            "OTM IV roundtrip: in={} out={} err={}",
            sigma_in,
            sigma_out,
            err
        );
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
        let err = if sigma_out > sigma_in {
            sigma_out - sigma_in
        } else {
            sigma_in - sigma_out
        };
        assert!(
            err <= 1000,
            "ITM IV roundtrip: in={} out={} err={}",
            sigma_in,
            sigma_out,
            err
        );
    }

    #[test]
    fn implied_vol_batch_roundtrip() {
        // Test a grid of scenarios
        let spots = [100 * SCALE];
        let strikes = [
            80 * SCALE,
            90 * SCALE,
            100 * SCALE,
            110 * SCALE,
            120 * SCALE,
        ];
        let sigmas = [
            100_000_000_000u128,
            200_000_000_000,
            500_000_000_000,
            1_000_000_000_000,
        ];
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
                            Err(_) => {
                                continue;
                            }
                        };
                        if bs.call < 2 {
                            continue;
                        }
                        match implied_vol(bs.call, s, k, rate, t) {
                            Ok(sigma_out) => {
                                let err = if sigma_out > sigma_in {
                                    sigma_out - sigma_in
                                } else {
                                    sigma_in - sigma_out
                                };
                                if err <= 1000 {
                                    pass += 1;
                                }
                            }
                            Err(_) => {}
                        }
                    }
                }
            }
        }
        let pct = pass as f64 / total as f64 * 100.0;
        assert!(
            pct >= 90.0,
            "IV batch roundtrip: {}/{} passed ({:.1}%), need ≥90%",
            pass,
            total,
            pct
        );
    }

    // ── NIG errors ──

    #[test]
    fn nig_zero_inputs_is_domain_error() {
        assert!(nig_call_price(0, SCALE, 0, SCALE, 10 * SCALE, 0, SCALE).is_err());
        assert!(nig_call_price(SCALE, 0, 0, SCALE, 10 * SCALE, 0, SCALE).is_err());
    }

    #[test]
    fn nig_invalid_shape_is_domain_error() {
        // Invalid NIG shapes are rejected before numerical evaluation.
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let alpha = SCALE;
        let beta = 2 * SCALE_I;
        let delta = SCALE;
        assert_eq!(
            nig_call_price(s, k, 0, SCALE, alpha, beta, delta),
            Err(SolMathError::DomainError)
        );
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
        assert!(matches!(
            complex_div(a, b),
            Err(SolMathError::DivisionByZero)
        ));
    }

    // ── Total function sanity checks ──

    #[test]
    fn sin_fixed_zero() {
        assert_eq!(sin_fixed(0).unwrap(), 0);
    }

    #[test]
    fn trig_handles_extreme_i128_inputs() {
        assert_eq!(sin_fixed(i128::MIN), Err(SolMathError::DomainError));
        assert_eq!(cos_fixed(i128::MIN), Err(SolMathError::DomainError));
        assert_eq!(sincos_fixed(i128::MAX), Err(SolMathError::DomainError));
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
    fn mul_div_floor_exact() {
        assert_eq!(mul_div_floor(10, 20, 5).unwrap(), 40);
    }
    #[test]
    fn mul_div_ceil_exact() {
        assert_eq!(mul_div_ceil(10, 20, 5).unwrap(), 40);
    }
    #[test]
    fn mul_div_floor_truncates() {
        assert_eq!(mul_div_floor(100, 200, 300).unwrap(), 66);
    }
    #[test]
    fn mul_div_ceil_rounds_up() {
        assert_eq!(mul_div_ceil(100, 200, 300).unwrap(), 67);
    }
    #[test]
    fn mul_div_floor_one_third() {
        assert_eq!(mul_div_floor(1, 1, 3).unwrap(), 0);
    }
    #[test]
    fn mul_div_ceil_one_third() {
        assert_eq!(mul_div_ceil(1, 1, 3).unwrap(), 1);
    }
    #[test]
    fn mul_div_floor_two_thirds() {
        assert_eq!(mul_div_floor(2, 1, 3).unwrap(), 0);
    }
    #[test]
    fn mul_div_ceil_two_thirds() {
        assert_eq!(mul_div_ceil(2, 1, 3).unwrap(), 1);
    }
    #[test]
    fn mul_div_floor_half() {
        assert_eq!(mul_div_floor(7, 3, 2).unwrap(), 10);
    }
    #[test]
    fn mul_div_ceil_half() {
        assert_eq!(mul_div_ceil(7, 3, 2).unwrap(), 11);
    }

    // Group 2: Identity and zero
    #[test]
    fn mul_div_floor_zero_a() {
        assert_eq!(mul_div_floor(0, 1000, 1).unwrap(), 0);
    }
    #[test]
    fn mul_div_ceil_zero_a() {
        assert_eq!(mul_div_ceil(0, 1000, 1).unwrap(), 0);
    }
    #[test]
    fn mul_div_floor_zero_b() {
        assert_eq!(mul_div_floor(1000, 0, 1).unwrap(), 0);
    }
    #[test]
    fn mul_div_ceil_zero_b() {
        assert_eq!(mul_div_ceil(1000, 0, 1).unwrap(), 0);
    }
    #[test]
    fn mul_div_floor_identity() {
        assert_eq!(mul_div_floor(1000, 1, 1).unwrap(), 1000);
    }
    #[test]
    fn mul_div_ceil_identity() {
        assert_eq!(mul_div_ceil(1000, 1, 1).unwrap(), 1000);
    }
    #[test]
    fn mul_div_floor_zero_zero() {
        assert_eq!(mul_div_floor(0, 0, 1).unwrap(), 0);
    }
    #[test]
    fn mul_div_ceil_zero_zero() {
        assert_eq!(mul_div_ceil(0, 0, 1).unwrap(), 0);
    }

    // Group 3: Division by zero
    #[test]
    fn mul_div_floor_div_by_zero() {
        assert_eq!(
            mul_div_floor(100, 200, 0),
            Err(SolMathError::DivisionByZero)
        );
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
        assert_eq!(
            mul_div_floor(10_u64.pow(18), 10_u64.pow(12), 10_u64.pow(12)).unwrap(),
            10_u64.pow(18)
        );
    }
    #[test]
    fn mul_div_ceil_large_cancelling() {
        assert_eq!(
            mul_div_ceil(10_u64.pow(18), 10_u64.pow(12), 10_u64.pow(12)).unwrap(),
            10_u64.pow(18)
        );
    }
    #[test]
    fn mul_div_floor_max_identity() {
        assert_eq!(mul_div_floor(u64::MAX, 1, 1).unwrap(), u64::MAX);
    }
    #[test]
    fn mul_div_ceil_max_identity() {
        assert_eq!(mul_div_ceil(u64::MAX, 1, 1).unwrap(), u64::MAX);
    }
    #[test]
    fn mul_div_floor_max_times_2_div_2() {
        assert_eq!(mul_div_floor(u64::MAX, 2, 2).unwrap(), u64::MAX);
    }
    #[test]
    fn mul_div_ceil_max_times_2_div_2() {
        assert_eq!(mul_div_ceil(u64::MAX, 2, 2).unwrap(), u64::MAX);
    }
    #[test]
    fn mul_div_floor_max_cubed() {
        assert_eq!(
            mul_div_floor(u64::MAX, u64::MAX, u64::MAX).unwrap(),
            u64::MAX
        );
    }

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
        assert_eq!(
            mul_div_floor(u64::MAX, u64::MAX, 1),
            Err(SolMathError::Overflow)
        );
    }
    #[test]
    fn mul_div_ceil_overflow_max_sq() {
        assert_eq!(
            mul_div_ceil(u64::MAX, u64::MAX, 1),
            Err(SolMathError::Overflow)
        );
    }

    // Group 6: Folio-specific values
    #[test]
    fn mul_div_ceil_proportional_join() {
        // B_k=10^12, desired_lp=50*10^9, supply=100*10^9
        assert_eq!(
            mul_div_ceil(
                1_000_000_000_000_u64,
                50_000_000_000_u64,
                100_000_000_000_u64
            )
            .unwrap(),
            500_000_000_000
        );
    }
    #[test]
    fn mul_div_floor_ratio_cap() {
        // 30% of 10^12 balance
        assert_eq!(
            mul_div_floor(1_000_000_000_000_u64, 30, 100).unwrap(),
            300_000_000_000
        );
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
        assert_eq!(
            mul_div_floor_u128(100, 200, 0),
            Err(SolMathError::DivisionByZero)
        );
        assert_eq!(
            mul_div_ceil_u128(100, 200, 0),
            Err(SolMathError::DivisionByZero)
        );
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
        assert_eq!(
            mul_div_floor_u128(u128::MAX, u128::MAX, u128::MAX).unwrap(),
            u128::MAX
        );
    }

    #[test]
    fn mul_div_u128_overflow() {
        assert_eq!(
            mul_div_floor_u128(u128::MAX, 2, 1),
            Err(SolMathError::Overflow)
        );
        assert_eq!(
            mul_div_ceil_u128(u128::MAX, 2, 1),
            Err(SolMathError::Overflow)
        );
        assert_eq!(
            mul_div_floor_u128(u128::MAX, u128::MAX, 1),
            Err(SolMathError::Overflow)
        );
        assert_eq!(
            mul_div_ceil_u128(u128::MAX, u128::MAX, 1),
            Err(SolMathError::Overflow)
        );
    }

    #[test]
    fn mul_div_u128_scale_values() {
        // Typical SCALE-valued operations
        assert_eq!(
            mul_div_floor_u128(100 * SCALE, 100 * SCALE, SCALE).unwrap(),
            10000 * SCALE
        );
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
        assert_eq!(
            sum, out.vanilla,
            "In/Out conservation: in={} out={} vanilla={}",
            in_.price, out.price, out.vanilla
        );
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
        assert_eq!(
            sum, out.vanilla,
            "Put In/Out conservation: in={} out={} vanilla={}",
            in_.price, out.price, out.vanilla
        );
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
        assert!(
            diff * 100 < result.vanilla,
            "Far barrier should ≈ vanilla: price={} vanilla={}",
            result.price,
            result.vanilla
        );
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
        assert!(
            result.price <= result.vanilla,
            "Out should be ≤ vanilla: {} vs {}",
            result.price,
            result.vanilla
        );
        assert!(
            result.price > 0,
            "Out should be positive when spot is above barrier"
        );
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
        assert!(
            result.price > 0,
            "Up-and-out call should be positive: {}",
            result.price
        );
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
        assert_eq!(
            sum, out.vanilla,
            "Up call In/Out conservation: in={} out={} vanilla={}",
            in_.price, out.price, out.vanilla
        );
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
        let diff = if sum > out.vanilla {
            sum - out.vanilla
        } else {
            out.vanilla - sum
        };
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
        let diff = if sum > out.vanilla {
            sum - out.vanilla
        } else {
            out.vanilla - sum
        };
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
        let diff = if sum > out.vanilla {
            sum - out.vanilla
        } else {
            out.vanilla - sum
        };
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

        assert!(
            out.price > 0,
            "Up-out put K>H should be positive: out={} in={} vanilla={}",
            out.price,
            in_.price,
            out.vanilla
        );
        assert!(
            out.price <= out.vanilla,
            "Up-out ({}) > vanilla ({}), in={}",
            out.price,
            out.vanilla,
            in_.price
        );
        let sum = out.price + in_.price;
        let diff = if sum > out.vanilla {
            sum - out.vanilla
        } else {
            out.vanilla - sum
        };
        assert!(diff < 100, "Up put K>H conservation: diff={}", diff);
    }
}

// ── Property-based tests for mul_div (u64) ──
#[cfg(test)]
mod mul_div_properties {
    use super::*;

    fn next_u64(state: &mut u64) -> u64 {
        *state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        *state
    }

    /// Returns true when the case is inside the u64 API's non-overflow domain.
    fn check_case(a: u64, b: u64, c: u64) -> bool {
        if c == 0 {
            return false;
        }

        let product = (a as u128) * (b as u128);
        let c128 = c as u128;
        let floor = product / c128;
        let ceil = if product % c128 == 0 {
            floor
        } else {
            floor + 1
        };
        if floor > u64::MAX as u128 || ceil > u64::MAX as u128 {
            return false;
        }

        let f = mul_div_floor(a, b, c).unwrap();
        let ce = mul_div_ceil(a, b, c).unwrap();

        assert!(f <= ce, "floor {} > ceil {}", f, ce);
        assert!(ce - f <= 1, "ceil - floor = {} > 1", ce - f);
        assert_eq!(f as u128, floor);
        assert_eq!(ce as u128, ceil);
        assert!((f as u128) * c128 <= product);
        assert!((f as u128 + 1) * c128 > product);
        assert!((ce as u128) * c128 >= product);
        if ce > 0 {
            assert!((ce as u128 - 1) * c128 < product);
        }
        assert_eq!(mul_div_floor(a, b, c), mul_div_floor(b, a, c));
        assert_eq!(mul_div_ceil(a, b, c), mul_div_ceil(b, a, c));

        if product % c128 == 0 {
            assert_eq!(f, ce);
        }

        true
    }

    #[test]
    fn deterministic_properties() {
        let interesting = [
            0,
            1,
            2,
            3,
            7,
            10,
            99,
            100,
            999,
            1_000_000,
            u32::MAX as u64,
            u64::MAX / 2,
            u64::MAX - 1,
            u64::MAX,
        ];

        let mut checked = 0usize;
        for &a in &interesting {
            for &b in &interesting {
                for &c in &interesting[1..] {
                    checked += usize::from(check_case(a, b, c));
                }
            }
        }

        let mut state = 0x9e37_79b9_7f4a_7c15u64;
        for _ in 0..10_000 {
            let a = next_u64(&mut state);
            let b = next_u64(&mut state);
            let c = next_u64(&mut state) | 1;
            checked += usize::from(check_case(a, b, c));
        }

        assert!(
            checked > 1_000,
            "checked too few non-overflow cases: {}",
            checked
        );
    }
}

// ── Property-based tests for mul_div_u128 ──
#[cfg(test)]
mod mul_div_u128_properties {
    use super::*;

    fn next_u128(state: &mut u128) -> u128 {
        *state = state
            .wrapping_mul(0xda94_2042_e4dd_58b5_da94_2042_e4dd_58b5)
            .wrapping_add(0x9e37_79b9_7f4a_7c15_9e37_79b9_7f4a_7c15);
        *state
    }

    fn check_case(a: u128, b: u128, c: u128) -> bool {
        if c == 0 {
            return false;
        }

        let Some((floor, rem)) = crate::overflow::checked_mul_div_rem_u(a, b, c) else {
            return false;
        };
        if rem > 0 && floor == u128::MAX {
            return false;
        }
        let ceil = floor + u128::from(rem > 0);

        let f = mul_div_floor_u128(a, b, c).unwrap();
        let ce = mul_div_ceil_u128(a, b, c).unwrap();
        assert_eq!(f, floor);
        assert_eq!(ce, ceil);
        assert!(f <= ce, "floor {} > ceil {}", f, ce);
        assert!(ce - f <= 1, "ceil - floor = {} > 1", ce - f);
        assert_eq!(mul_div_floor_u128(a, b, c), mul_div_floor_u128(b, a, c));
        assert_eq!(mul_div_ceil_u128(a, b, c), mul_div_ceil_u128(b, a, c));

        if a <= u64::MAX as u128
            && b <= u64::MAX as u128
            && c <= u64::MAX as u128
            && ceil <= u64::MAX as u128
        {
            assert_eq!(
                mul_div_floor(a as u64, b as u64, c as u64).unwrap() as u128,
                f
            );
            assert_eq!(
                mul_div_ceil(a as u64, b as u64, c as u64).unwrap() as u128,
                ce
            );
        }

        true
    }

    #[test]
    fn deterministic_properties() {
        let interesting = [
            0,
            1,
            2,
            3,
            7,
            10,
            100,
            1_000_000,
            u64::MAX as u128,
            1u128 << 64,
            1u128 << 80,
            (1u128 << 96) - 1,
            u128::MAX / 2,
            u128::MAX - 1,
            u128::MAX,
        ];

        let mut checked = 0usize;
        for &a in &interesting {
            for &b in &interesting {
                for &c in &interesting[1..] {
                    checked += usize::from(check_case(a, b, c));
                }
            }
        }

        let mut state = 0x243f_6a88_85a3_08d3_1319_8a2e_0370_7344u128;
        for _ in 0..10_000 {
            let a = next_u128(&mut state) as u64 as u128;
            let b = next_u128(&mut state) as u64 as u128;
            let c = (next_u128(&mut state) as u64 | 1) as u128;
            checked += usize::from(check_case(a, b, c));
        }

        let mask = (1u128 << 80) - 1;
        for _ in 0..10_000 {
            let a = (1u128 << 80) | (next_u128(&mut state) & mask);
            let b = (1u128 << 80) | (next_u128(&mut state) & mask);
            let c = (1u128 << 80) | (next_u128(&mut state) & mask);
            checked += usize::from(check_case(a, b, c));
        }

        assert!(
            checked > 1_000,
            "checked too few non-overflow cases: {}",
            checked
        );
    }
}

// ── Cross-validation against exact integer reference vectors ──
#[cfg(test)]
mod mul_div_cross_validation {
    use super::*;

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
            Vector {
                a: 0,
                b: 0,
                c: 1,
                floor: 0,
                ceil: 0,
            },
            Vector {
                a: 0,
                b: u64::MAX,
                c: 1,
                floor: 0,
                ceil: 0,
            },
            Vector {
                a: u64::MAX,
                b: 1,
                c: 1,
                floor: u64::MAX,
                ceil: u64::MAX,
            },
            Vector {
                a: u64::MAX,
                b: 2,
                c: 2,
                floor: u64::MAX,
                ceil: u64::MAX,
            },
            Vector {
                a: 100,
                b: 200,
                c: 300,
                floor: 66,
                ceil: 67,
            },
            Vector {
                a: 1,
                b: 1,
                c: 3,
                floor: 0,
                ceil: 1,
            },
            Vector {
                a: 2,
                b: 1,
                c: 3,
                floor: 0,
                ceil: 1,
            },
            Vector {
                a: 7,
                b: 3,
                c: 2,
                floor: 10,
                ceil: 11,
            },
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
            Vector {
                a: u64::MAX,
                b: u64::MAX,
                c: u64::MAX,
                floor: u64::MAX,
                ceil: u64::MAX,
            },
        ];

        for v in &vectors {
            let f = mul_div_floor(v.a, v.b, v.c).unwrap();
            let ce = mul_div_ceil(v.a, v.b, v.c).unwrap();
            if f != v.floor {
                panic!(
                    "FLOOR mismatch: a={} b={} c={} expected={} got={}",
                    v.a, v.b, v.c, v.floor, f
                );
            }
            if ce != v.ceil {
                panic!(
                    "CEIL mismatch: a={} b={} c={} expected={} got={}",
                    v.a, v.b, v.c, v.ceil, ce
                );
            }
        }
    }
}
