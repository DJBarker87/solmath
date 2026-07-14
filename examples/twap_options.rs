//! Price an option settled against a partially fixed 30-minute TWAP.

use solmath::{twap_option_price, SCALE};

fn main() -> Result<(), solmath::SolMathError> {
    let minutes = |value: u128| value * SCALE / (365 * 24 * 60);

    // Twelve minutes of a thirty-minute settlement TWAP have already fixed at
    // $99.50. Eighteen minutes remain until expiry.
    let result = twap_option_price(
        100 * SCALE,        // current spot
        100 * SCALE,        // strike
        50_000_000_000,     // risk-free rate: 5%
        20_000_000_000,     // continuous yield: 2%
        600_000_000_000,    // volatility: 60%
        minutes(18),        // time to expiry
        minutes(18),        // remaining averaging time
        99_500_000_000_000, // fixed average: $99.50
        400_000_000_000,    // fixed weight: 12 / 30
    )?;

    println!("call: {}", result.call);
    println!("put: {}", result.put);
    println!("expected average: {}", result.expected_average);
    println!("matched log-variance: {}", result.log_variance);
    Ok(())
}
