//! Batch harness for validating the actual fixed-point exponential-NIG runtime.

use std::io::{self, BufRead};

use solmath::{nig_price_certified, NigParams};

fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.expect("stdin read failed");
        let fields: Vec<_> = line.split_whitespace().collect();
        if fields.len() != 9 {
            println!("ERR:fields");
            continue;
        }

        let unsigned = [0usize, 1, 4, 5, 7, 8];
        let mut u = [0u128; 6];
        let mut valid = true;
        for (target, index) in u.iter_mut().zip(unsigned) {
            match fields[index].parse::<u128>() {
                Ok(value) => *target = value,
                Err(_) => valid = false,
            }
        }
        let rate = fields[2].parse::<i128>();
        let dividend = fields[3].parse::<i128>();
        let beta = fields[6].parse::<i128>();
        if !valid || rate.is_err() || dividend.is_err() || beta.is_err() {
            println!("ERR:number");
            continue;
        }

        match nig_price_certified(
            u[0],
            u[1],
            rate.unwrap(),
            dividend.unwrap(),
            u[2],
            NigParams {
                alpha: u[3],
                beta: beta.unwrap(),
                delta_per_year: u[4],
            },
            u[5],
        ) {
            Ok(quote) => println!(
                "OK {} {} {} {}",
                quote.call, quote.put, quote.max_abs_error, quote.tier
            ),
            Err(error) => println!("ERR:{error:?}"),
        }
    }
}
