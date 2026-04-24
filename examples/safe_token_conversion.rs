use solmath::{fp_to_token_ceil, fp_to_token_floor, token_to_fp, SolMathError};

fn main() -> Result<(), SolMathError> {
    // Run with:
    // cargo run --example safe_token_conversion --features pool
    let usdc_decimals = 6;
    let one_usdc_raw = 1_000_000u64;
    let one_usdc_fp = token_to_fp(one_usdc_raw, usdc_decimals)?;

    // Use floor when the protocol pays out, so it never overpays dust.
    let payout_raw = fp_to_token_floor(one_usdc_fp, usdc_decimals)?;

    // Use ceil when the protocol collects fees/repayments, so it never
    // under-collects dust.
    let repayment_raw = fp_to_token_ceil(one_usdc_fp, usdc_decimals)?;

    assert_eq!(payout_raw, one_usdc_raw);
    assert_eq!(repayment_raw, one_usdc_raw);

    println!("1 USDC raw={} fixed={}", one_usdc_raw, one_usdc_fp);

    Ok(())
}
