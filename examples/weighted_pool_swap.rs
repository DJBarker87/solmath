use solmath::{fp, weighted_pool_swap, SolMathError, SCALE};

fn as_decimal(x: u128) -> f64 {
    x as f64 / SCALE as f64
}

fn main() -> Result<(), SolMathError> {
    // Run with:
    // cargo run --example weighted_pool_swap --features pool
    let balance_in = fp("1000")?;
    let balance_out = fp("1000")?;
    let amount_in = fp("10")?;
    let weight_in = fp("0.5")?;
    let weight_out = fp("0.5")?;
    let fee_rate = fp("0.003")?;

    let (net_out, fee) = weighted_pool_swap(
        balance_in,
        balance_out,
        weight_in,
        weight_out,
        amount_in,
        fee_rate,
    )?;

    println!("net_out: {:.12}", as_decimal(net_out));
    println!("fee:     {:.12}", as_decimal(fee));

    Ok(())
}
