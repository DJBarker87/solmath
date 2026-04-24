use solmath::{bs_full_hp, fp, SolMathError, SCALE};

fn as_decimal(x: u128) -> f64 {
    x as f64 / SCALE as f64
}

fn main() -> Result<(), SolMathError> {
    // Parse once in tests, clients, scripts, or off-chain config. On-chain
    // programs should pass already-validated integers across instruction data.
    let spot = fp("100")?;
    let strike = fp("105")?;
    let risk_free_rate = fp("0.05")?;
    let volatility = fp("0.20")?;
    let years_to_expiry = fp("1")?;

    let greeks = bs_full_hp(spot, strike, risk_free_rate, volatility, years_to_expiry)?;

    println!("call:  {:.12}", as_decimal(greeks.call));
    println!("put:   {:.12}", as_decimal(greeks.put));
    println!("gamma: {:.12}", greeks.gamma as f64 / SCALE as f64);
    println!("vega:  {:.12}", greeks.vega as f64 / SCALE as f64);

    Ok(())
}
