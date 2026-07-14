//! Line-oriented batch harness for offline Asian/TWAP validation.
//!
//! Each input line contains nine raw `u128` values:
//! `s k r q sigma t averaging_time fixed_average fixed_weight`.

use std::io::{self, BufRead};

use solmath::arithmetic_asian_price;

fn main() {
    for line in io::stdin().lock().lines() {
        let Ok(line) = line else {
            println!("ERR input");
            continue;
        };
        let values = line
            .split_whitespace()
            .map(str::parse::<u128>)
            .collect::<Result<Vec<_>, _>>();
        let Ok(values) = values else {
            println!("ERR parse");
            continue;
        };
        if values.len() != 9 {
            println!("ERR arity");
            continue;
        }
        match arithmetic_asian_price(
            values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7],
            values[8],
        ) {
            Ok(result) => println!(
                "{} {} {} {}",
                result.call, result.put, result.expected_average, result.log_variance
            ),
            Err(error) => println!("ERR {error}"),
        }
    }
}
