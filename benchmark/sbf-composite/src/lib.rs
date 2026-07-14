use anchor_lang::prelude::*;
use solmath::*;

declare_id!("BdR4cSgZGQgXNo33SZSYQXy7XgEK61sHT4NQaAkc3PBm");

const fn measurement_phi2_grid() -> [[i32; PHI2_GRID_SIZE]; PHI2_GRID_SIZE] {
    let mut values = [[0; PHI2_GRID_SIZE]; PHI2_GRID_SIZE];
    let mut row = 0;
    while row < PHI2_GRID_SIZE {
        let mut column = 0;
        while column < PHI2_GRID_SIZE {
            values[row][column] = ((row * PHI2_GRID_SIZE + column) * 244) as i32;
            column += 1;
        }
        row += 1;
    }
    values
}

static MEASUREMENT_PHI2_TABLE: Phi2Table = Phi2Table::from_array(measurement_phi2_grid());

#[inline(always)]
fn log_cu() {
    #[cfg(target_os = "solana")]
    unsafe {
        solana_msg::syscalls::sol_log_compute_units_();
    }
}

macro_rules! measured {
    ($expr:expr) => {{
        log_cu();
        let result = $expr;
        log_cu();
        let succeeded = result.is_ok();
        let _ = core::hint::black_box(result);
        msg!("succeeded = {}", succeeded);
        Ok(())
    }};
}

#[program]
pub mod solmath_sbf_composite {
    use super::*;

    pub fn compute_ln(_ctx: Context<Measure>, x: u128) -> Result<()> {
        measured!(ln_fixed_i(x))
    }

    pub fn compute_exp(_ctx: Context<Measure>, x: i128) -> Result<()> {
        measured!(exp_fixed_i(x))
    }

    pub fn compute_cdf(_ctx: Context<Measure>, x: i128) -> Result<()> {
        measured!(norm_cdf_poly(x))
    }

    pub fn compute_ln_hp(_ctx: Context<Measure>, x: i128) -> Result<()> {
        measured!(ln_fixed_hp(x))
    }

    pub fn compute_exp_hp(_ctx: Context<Measure>, x: i128) -> Result<()> {
        measured!(exp_fixed_hp(x))
    }

    pub fn compute_cdf_hp(_ctx: Context<Measure>, x: i128) -> Result<()> {
        measured!(norm_cdf_poly_hp(x))
    }

    pub fn compute_div_hp(_ctx: Context<Measure>, a: i128, b: i128) -> Result<()> {
        measured!(fp_div_hp_safe(a, b))
    }

    pub fn compute_pdf(_ctx: Context<Measure>, x: i128) -> Result<()> {
        measured!(norm_pdf(x))
    }

    pub fn compute_pow(_ctx: Context<Measure>, base: u128, exponent: u128) -> Result<()> {
        measured!(pow_fixed(base, exponent))
    }

    pub fn compute_pow_i(_ctx: Context<Measure>, base: i128, exponent: i128) -> Result<()> {
        measured!(pow_fixed_i(base, exponent))
    }

    pub fn compute_pow_int(_ctx: Context<Measure>, base: u128, exponent: u128) -> Result<()> {
        measured!(pow_int(base, exponent))
    }

    pub fn compute_inverse_cdf(_ctx: Context<Measure>, probability: i128) -> Result<()> {
        measured!(inverse_norm_cdf(probability))
    }

    pub fn compute_cdf_pdf(_ctx: Context<Measure>, x: i128) -> Result<()> {
        measured!(norm_cdf_and_pdf(x))
    }

    pub fn compute_cdf_pdf_poly(_ctx: Context<Measure>, x: i128) -> Result<()> {
        measured!(norm_cdf_and_pdf_poly(x))
    }

    pub fn compute_mul_i_round(_ctx: Context<Measure>, a: i128, b: i128) -> Result<()> {
        measured!(fp_mul_i_round(a, b))
    }

    pub fn compute_mul_i_fast(_ctx: Context<Measure>, a: i128, b: i128) -> Result<()> {
        measured!(Ok::<i128, SolMathError>(a.wrapping_mul(b) / SCALE_I))
    }

    pub fn compute_mul_i_fast_round(_ctx: Context<Measure>, a: i128, b: i128) -> Result<()> {
        measured!({
            let product = a.wrapping_mul(b);
            let half = SCALE_I / 2;
            Ok::<i128, SolMathError>(if product >= 0 {
                product.wrapping_add(half) / SCALE_I
            } else {
                -product.wrapping_neg().wrapping_add(half) / SCALE_I
            })
        })
    }

    pub fn compute_div_i(_ctx: Context<Measure>, a: i128, b: i128) -> Result<()> {
        measured!(fp_div_i(a, b))
    }

    pub fn compute_bs_price(
        _ctx: Context<Measure>,
        s: u128,
        k: u128,
        r: u128,
        sigma: u128,
        t: u128,
    ) -> Result<()> {
        measured!(black_scholes_price(s, k, r, sigma, t))
    }

    pub fn compute_bs_full(
        _ctx: Context<Measure>,
        s: u128,
        k: u128,
        r: u128,
        sigma: u128,
        t: u128,
    ) -> Result<()> {
        measured!(bs_full(s, k, r, sigma, t))
    }

    pub fn compute_bs_delta(
        _ctx: Context<Measure>,
        s: u128,
        k: u128,
        r: u128,
        sigma: u128,
        t: u128,
    ) -> Result<()> {
        measured!(bs_delta(s, k, r, sigma, t))
    }

    pub fn compute_bs_gamma(
        _ctx: Context<Measure>,
        s: u128,
        k: u128,
        r: u128,
        sigma: u128,
        t: u128,
    ) -> Result<()> {
        measured!(bs_gamma(s, k, r, sigma, t))
    }

    pub fn compute_bs_vega(
        _ctx: Context<Measure>,
        s: u128,
        k: u128,
        r: u128,
        sigma: u128,
        t: u128,
    ) -> Result<()> {
        measured!(bs_vega(s, k, r, sigma, t))
    }

    pub fn compute_bs_theta(
        _ctx: Context<Measure>,
        s: u128,
        k: u128,
        r: u128,
        sigma: u128,
        t: u128,
    ) -> Result<()> {
        measured!(bs_theta(s, k, r, sigma, t))
    }

    pub fn compute_bs_rho(
        _ctx: Context<Measure>,
        s: u128,
        k: u128,
        r: u128,
        sigma: u128,
        t: u128,
    ) -> Result<()> {
        measured!(bs_rho(s, k, r, sigma, t))
    }

    pub fn compute_iv(
        _ctx: Context<Measure>,
        market_price: u128,
        s: u128,
        k: u128,
        r: u128,
        t: u128,
    ) -> Result<()> {
        measured!(implied_vol(market_price, s, k, r, t))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_asian(
        _ctx: Context<Measure>,
        s: u128,
        k: u128,
        r: u128,
        q: u128,
        sigma: u128,
        t: u128,
        averaging_time: u128,
        fixed_average: u128,
        fixed_weight: u128,
    ) -> Result<()> {
        measured!(arithmetic_asian_price(
            s,
            k,
            r,
            q,
            sigma,
            t,
            averaging_time,
            fixed_average,
            fixed_weight,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_sabr_vol(
        _ctx: Context<Measure>,
        f: u128,
        k: u128,
        t: u128,
        alpha: u128,
        beta: u128,
        rho: i128,
        nu: u128,
    ) -> Result<()> {
        measured!(sabr_implied_vol(f, k, t, alpha, beta, rho, nu))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_sabr_price(
        _ctx: Context<Measure>,
        s: u128,
        k: u128,
        r: u128,
        t: u128,
        alpha: u128,
        beta: u128,
        rho: i128,
        nu: u128,
    ) -> Result<()> {
        measured!(sabr_price(s, k, r, t, alpha, beta, rho, nu))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_sabr_greeks(
        _ctx: Context<Measure>,
        s: u128,
        k: u128,
        r: u128,
        t: u128,
        alpha: u128,
        beta: u128,
        rho: i128,
        nu: u128,
    ) -> Result<()> {
        measured!(sabr_greeks(s, k, r, t, alpha, beta, rho, nu))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_sabr_precomputed_vol(
        _ctx: Context<Measure>,
        f: u128,
        k: u128,
        t: u128,
        alpha: u128,
        beta: u128,
        rho: i128,
        nu: u128,
    ) -> Result<()> {
        measured!(sabr_precompute(f, t, alpha, beta, rho, nu)
            .and_then(|precomputed| sabr_vol_at(&precomputed, k)))
    }

    pub fn compute_sabr_precompute(
        _ctx: Context<Measure>,
        f: u128,
        t: u128,
        alpha: u128,
        beta: u128,
        rho: i128,
        nu: u128,
    ) -> Result<()> {
        measured!(sabr_precompute(f, t, alpha, beta, rho, nu))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_sabr_vol_at(
        _ctx: Context<Measure>,
        f: u128,
        k: u128,
        t: u128,
        alpha: u128,
        beta: u128,
        rho: i128,
        nu: u128,
    ) -> Result<()> {
        let precomputed = sabr_precompute(f, t, alpha, beta, rho, nu);
        match precomputed {
            Ok(precomputed) => measured!(sabr_vol_at(&precomputed, k)),
            Err(error) => measured!(Err::<u128, _>(error)),
        }
    }

    pub fn compute_sabr_z_over_chi(_ctx: Context<Measure>, z: i128, rho: i128) -> Result<()> {
        measured!(sabr_z_over_chi_pade(z, rho))
    }

    pub fn compute_bvn(_ctx: Context<Measure>, a: i128, b: i128, rho: i128) -> Result<()> {
        measured!(bvn_cdf(a, b, rho))
    }

    pub fn compute_bvn_hp(_ctx: Context<Measure>, a: i128, b: i128, rho: i128) -> Result<()> {
        measured!(bvn_cdf_hp(a, b, rho))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_heston(
        _ctx: Context<Measure>,
        s: u128,
        k: u128,
        r: u128,
        t: u128,
        v0: u128,
        kappa: u128,
        theta: u128,
        xi: u128,
        rho: i128,
    ) -> Result<()> {
        measured!(heston_price(s, k, r, t, v0, kappa, theta, xi, rho))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_nig(
        _ctx: Context<Measure>,
        s: u128,
        k: u128,
        r: u128,
        t: u128,
        alpha: u128,
        beta: i128,
        delta: u128,
    ) -> Result<()> {
        measured!(nig_call_price(s, k, r, t, alpha, beta, delta))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_nig_certified(
        _ctx: Context<Measure>,
        s: u128,
        k: u128,
        r: i128,
        q: i128,
        t: u128,
        alpha: u128,
        beta: i128,
        delta: u128,
        requested_max_abs_error: u128,
    ) -> Result<()> {
        measured!(nig_price_certified(
            s,
            k,
            r,
            q,
            t,
            NigParams {
                alpha,
                beta,
                delta_per_year: delta,
            },
            requested_max_abs_error,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_nig_64(
        _ctx: Context<Measure>,
        s: i64,
        k: i64,
        r: i64,
        t: i64,
        alpha: i64,
        beta: i64,
        delta: i64,
    ) -> Result<()> {
        measured!(nig_call_64(s, k, r, t, alpha, beta, delta))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_nig_put_64(
        _ctx: Context<Measure>,
        s: i64,
        k: i64,
        r: i64,
        t: i64,
        alpha: i64,
        beta: i64,
        delta: i64,
    ) -> Result<()> {
        measured!(nig_put_64(s, k, r, t, alpha, beta, delta))
    }

    pub fn compute_phi2_eval(_ctx: Context<Measure>, a: i128, b: i128) -> Result<()> {
        let table = core::hint::black_box(&MEASUREMENT_PHI2_TABLE);
        measured!(table.eval(a, b))
    }

    pub fn compute_american_kbi_call(
        _ctx: Context<Measure>,
        spot: u128,
        strike: u128,
        rate: u128,
        dividend_yield: u128,
        sigma: u128,
        maturity: u128,
    ) -> Result<()> {
        measured!(american_kbi_price(
            spot,
            strike,
            rate,
            dividend_yield,
            sigma,
            maturity,
            AmericanKbiKind::Call,
        ))
    }

    pub fn compute_american_kbi_put(
        _ctx: Context<Measure>,
        spot: u128,
        strike: u128,
        rate: u128,
        dividend_yield: u128,
        sigma: u128,
        maturity: u128,
    ) -> Result<()> {
        measured!(american_kbi_price(
            spot,
            strike,
            rate,
            dividend_yield,
            sigma,
            maturity,
            AmericanKbiKind::Put,
        ))
    }

}

#[derive(Accounts)]
pub struct Measure {}
