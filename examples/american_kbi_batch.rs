//! Batch harness for validating the actual fixed-point KBI runtime.

use std::io::{self, BufRead};

use solmath::{american_kbi_price, AmericanKbiKind};

fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.expect("stdin read failed");
        let fields: Vec<_> = line.split_whitespace().collect();
        if fields.len() != 7 {
            println!("ERR:fields");
            continue;
        }
        let kind = match fields[0] {
            "call" => AmericanKbiKind::Call,
            "put" => AmericanKbiKind::Put,
            _ => {
                println!("ERR:kind");
                continue;
            }
        };
        let mut values = [0u128; 6];
        let mut valid = true;
        for (target, source) in values.iter_mut().zip(&fields[1..]) {
            match source.parse::<u128>() {
                Ok(value) => *target = value,
                Err(_) => valid = false,
            }
        }
        if !valid {
            println!("ERR:number");
            continue;
        }
        match american_kbi_price(
            values[0], values[1], values[2], values[3], values[4], values[5], kind,
        ) {
            Ok(price) => println!("{price}"),
            Err(error) => println!("ERR:{error:?}"),
        }
    }
}
