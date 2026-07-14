use anchor_lang::prelude::*;

#[cfg(feature = "legacy-expm1")]
use solmath::{exp_fixed_i, fp_mul_i_round, SolMathError, SCALE_I};

#[cfg(any(
    all(feature = "exp", feature = "expm1"),
    all(feature = "exp", feature = "ln1p"),
    all(feature = "exp", feature = "both"),
    all(feature = "exp", feature = "legacy-expm1"),
    all(feature = "expm1", feature = "ln1p"),
    all(feature = "expm1", feature = "both"),
    all(feature = "expm1", feature = "legacy-expm1"),
    all(feature = "ln1p", feature = "both"),
    all(feature = "ln1p", feature = "legacy-expm1"),
    all(feature = "both", feature = "legacy-expm1"),
))]
compile_error!("select at most one footprint variant");

#[cfg(all(
    feature = "kbi",
    any(
        feature = "exp",
        feature = "expm1",
        feature = "ln1p",
        feature = "both",
        feature = "legacy-expm1"
    )
))]
compile_error!("select at most one footprint variant");

#[cfg(all(
    feature = "nig",
    any(
        feature = "exp",
        feature = "expm1",
        feature = "ln1p",
        feature = "both",
        feature = "legacy-expm1",
        feature = "kbi"
    )
))]
compile_error!("select at most one footprint variant");

declare_id!("Dr5QfdFEChkNpR9bPcAdRXNLuc1gTu955EnhS5bBg8m5");

#[cfg(feature = "legacy-expm1")]
#[inline(never)]
fn legacy_expm1_fixed(x: i128) -> core::result::Result<i128, SolMathError> {
    let half = SCALE_I / 2;
    if x > half || x < -half {
        return Ok(exp_fixed_i(x)? - SCALE_I);
    }
    if x == 0 {
        return Ok(0);
    }

    const C11: i128 = 25_052;
    const C10: i128 = 275_573;
    const C9: i128 = 2_755_732;
    const C8: i128 = 24_801_587;
    const C7: i128 = 198_412_698;
    const C6: i128 = 1_388_888_889;
    const C5: i128 = 8_333_333_333;
    const C4: i128 = 41_666_666_667;
    const C3: i128 = 166_666_666_667;
    const C2: i128 = 500_000_000_000;

    let p = fp_mul_i_round(x, C11)? + C10;
    let p = fp_mul_i_round(x, p)? + C9;
    let p = fp_mul_i_round(x, p)? + C8;
    let p = fp_mul_i_round(x, p)? + C7;
    let p = fp_mul_i_round(x, p)? + C6;
    let p = fp_mul_i_round(x, p)? + C5;
    let p = fp_mul_i_round(x, p)? + C4;
    let p = fp_mul_i_round(x, p)? + C3;
    let p = fp_mul_i_round(x, p)? + C2;
    let p = fp_mul_i_round(x, p)? + SCALE_I;
    fp_mul_i_round(x, p)
}

#[program]
pub mod solmath_sbf_footprint {
    use super::*;

    #[allow(unused_variables)]
    pub fn measure(_ctx: Context<Measure>, x: i128) -> Result<()> {
        #[cfg(feature = "exp")]
        let result = solmath::exp_fixed_i(x).map_err(|_| FootprintError::MathError)?;
        #[cfg(feature = "expm1")]
        let result = solmath::expm1_fixed(x).map_err(|_| FootprintError::MathError)?;
        #[cfg(feature = "ln1p")]
        let result = solmath::ln_1p_fixed(x).map_err(|_| FootprintError::MathError)?;
        #[cfg(feature = "both")]
        let result = {
            let expm1 = solmath::expm1_fixed(x).map_err(|_| FootprintError::MathError)?;
            let ln1p = solmath::ln_1p_fixed(x).map_err(|_| FootprintError::MathError)?;
            expm1.wrapping_add(ln1p)
        };
        #[cfg(feature = "legacy-expm1")]
        let result = legacy_expm1_fixed(x).map_err(|_| FootprintError::MathError)?;
        #[cfg(feature = "kbi")]
        let result = solmath::american_kbi_price(
            100 * solmath::SCALE,
            100 * solmath::SCALE,
            50_000_000_000,
            30_000_000_000,
            300_000_000_000,
            solmath::SCALE,
            solmath::AmericanKbiKind::Put,
        )
        .map_err(|_| FootprintError::MathError)? as i128;
        #[cfg(feature = "nig")]
        let result = solmath::nig_call_price(
            100 * solmath::SCALE,
            100 * solmath::SCALE,
            50_000_000_000,
            solmath::SCALE,
            10 * solmath::SCALE,
            -2 * solmath::SCALE_I,
            solmath::SCALE / 5,
        )
        .map_err(|_| FootprintError::MathError)? as i128;
        #[cfg(not(any(
            feature = "exp",
            feature = "expm1",
            feature = "ln1p",
            feature = "both",
            feature = "legacy-expm1",
            feature = "kbi",
            feature = "nig"
        )))]
        let result = x;

        msg!("result = {}", result);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Measure {}

#[error_code]
pub enum FootprintError {
    MathError,
}
