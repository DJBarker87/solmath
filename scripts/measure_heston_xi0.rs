//! Emit the deterministic 200,704-case Heston grid for independent analysis.

use solmath::{fp_mul, heston_price, SCALE};

fn main() {
    let spots = [1u128, 100];
    let strike_multipliers = [
        250_000_000_000u128,
        500_000_000_000,
        800_000_000_000,
        SCALE,
        1_200_000_000_000,
        2 * SCALE,
        4 * SCALE,
    ];
    let rates = [0u128, 10_000_000_000, 50_000_000_000, 200_000_000_000];
    let times = [
        1_000_000_000u128,
        10_000_000_000,
        100_000_000_000,
        500_000_000_000,
        SCALE,
        2 * SCALE,
        10 * SCALE,
        100 * SCALE,
    ];
    let variances = [
        0u128,
        1_000_000,
        100_000_000,
        10_000_000_000,
        40_000_000_000,
        250_000_000_000,
        SCALE,
        4 * SCALE,
    ];
    let kappas = [
        0u128,
        1_000_000,
        1_000_000_000,
        10_000_000_000,
        500_000_000_000,
        2 * SCALE,
        20 * SCALE,
    ];

    for &spot_units in &spots {
        for &strike_multiplier in &strike_multipliers {
            for &rate in &rates {
                for &time in &times {
                    for &v0 in &variances {
                        for &theta in &variances {
                            for &kappa in &kappas {
                                let spot = spot_units * SCALE;
                                let strike = fp_mul(spot, strike_multiplier).unwrap();
                                match heston_price(
                                    spot, strike, rate, time, v0, kappa, theta, 0, 0,
                                ) {
                                    Ok((call, put)) => println!(
                                        "{spot},{strike},{rate},{time},{v0},{kappa},{theta},{call},{put}"
                                    ),
                                    Err(error) => println!(
                                        "{spot},{strike},{rate},{time},{v0},{kappa},{theta},E{error:?},E{error:?}"
                                    ),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
