//! Dependency-free accuracy runner for unary production/adversarial corpora.
//!
//! Build the release library, compile this file with `rustc`, then run:
//! `measure-unary exp FILE...`, `measure-unary ln FILE...`, or
//! `measure-unary norm_cdf FILE...`.

use solmath::{exp_fixed_i, ln_fixed_i, norm_cdf_poly};
use std::{collections::BTreeMap, env, fs};

fn quoted_after(input: &str, key: &str, start: usize) -> Option<(String, usize)> {
    let key_at = input[start..].find(key)? + start;
    let colon = input[key_at + key.len()..].find(':')? + key_at + key.len();
    let quote = input[colon + 1..].find('"')? + colon + 1;
    let end = input[quote + 1..].find('"')? + quote + 1;
    Some((input[quote + 1..end].to_owned(), end + 1))
}

fn load(path: &str) -> Vec<(String, i128, String)> {
    let input = fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"));
    let mut rows = Vec::new();
    let mut cursor = input.find("\"vectors\"").expect("vectors array");
    let categorized = input[cursor..].contains("\"category\"");
    while let Some((x, after_x)) = quoted_after(&input, "\"x\"", cursor) {
        let Some((expected, after_expected)) = quoted_after(&input, "\"expected\"", after_x)
        else {
            break;
        };
        let (category, after_category) = if categorized {
            quoted_after(&input, "\"category\"", after_expected)
                .unwrap_or_else(|| ("uncategorized".to_owned(), after_expected))
        } else {
            ("uncategorized".to_owned(), after_expected)
        };
        rows.push((x, expected.parse().expect("i128 expected"), category));
        cursor = after_category;
    }
    rows
}

fn percentile(sorted: &[u128], numerator: usize, denominator: usize) -> u128 {
    sorted[(sorted.len() - 1) * numerator / denominator]
}

fn percentile_f64(sorted: &[f64], numerator: usize, denominator: usize) -> f64 {
    sorted[(sorted.len() - 1) * numerator / denominator]
}

fn stats(
    file: &str,
    category: &str,
    errors: &mut [u128],
    relative_ppb: &mut [f64],
    failures: usize,
) {
    errors.sort_unstable();
    relative_ppb.sort_unstable_by(f64::total_cmp);
    let exact = errors.partition_point(|error| *error == 0);
    println!(
        "{{\"file\":\"{}\",\"category\":\"{}\",\"vectors\":{},\"accepted\":{},\"failures\":{},\"max\":{},\"p99\":{},\"p95\":{},\"median\":{},\"exact_pct\":{:.6},\"max_relative_ppb\":{:.12},\"p99_relative_ppb\":{:.12},\"p95_relative_ppb\":{:.12},\"median_relative_ppb\":{:.12}}}",
        file,
        category,
        errors.len() + failures,
        errors.len(),
        failures,
        errors.last().copied().unwrap_or(0),
        percentile(errors, 99, 100),
        percentile(errors, 95, 100),
        percentile(errors, 1, 2),
        exact as f64 * 100.0 / errors.len() as f64,
        relative_ppb.last().copied().unwrap_or(0.0),
        percentile_f64(relative_ppb, 99, 100),
        percentile_f64(relative_ppb, 95, 100),
        percentile_f64(relative_ppb, 1, 2),
    );
}

fn evaluate(kind: &str, x: &str) -> Result<i128, solmath::SolMathError> {
    match kind {
        "ln" => ln_fixed_i(x.parse().expect("u128 ln input")),
        "exp" => exp_fixed_i(x.parse().expect("i128 exp input")),
        "norm_cdf" => norm_cdf_poly(x.parse().expect("i128 norm_cdf input")),
        _ => panic!("unknown function {kind}; expected exp, ln, or norm_cdf"),
    }
}

fn measure(kind: &str, path: &str) {
    let rows = load(path);
    let mut errors = Vec::with_capacity(rows.len());
    let mut relative_ppb = Vec::with_capacity(rows.len());
    let mut categories: BTreeMap<String, (Vec<u128>, Vec<f64>)> = BTreeMap::new();
    let mut failures = 0usize;
    let mut worst = Vec::new();
    let mut max_error = 0u128;
    for (x, expected, category) in rows {
        match evaluate(kind, &x) {
            Ok(actual) => {
                let error = actual.abs_diff(expected);
                let relative = error as f64 / expected.unsigned_abs().max(1) as f64 * 1e9;
                if error > max_error {
                    max_error = error;
                    worst.clear();
                }
                if error == max_error && worst.len() < 20 {
                    worst.push((x, expected, actual, category.clone()));
                }
                errors.push(error);
                relative_ppb.push(relative);
                let category_values = categories.entry(category).or_default();
                category_values.0.push(error);
                category_values.1.push(relative);
            }
            Err(_) => failures += 1,
        }
    }
    stats(path, "all", &mut errors, &mut relative_ppb, failures);
    for (category, (mut category_errors, mut category_relative_ppb)) in categories {
        stats(
            path,
            &category,
            &mut category_errors,
            &mut category_relative_ppb,
            0,
        );
    }
    for (x, expected, actual, category) in worst {
        println!(
            "{{\"file\":\"{}\",\"worst\":true,\"x\":\"{}\",\"category\":\"{}\",\"expected\":\"{}\",\"actual\":\"{}\",\"error\":{}}}",
            path, x, category, expected, actual, max_error
        );
    }
}

fn main() {
    let mut args = env::args().skip(1);
    let kind = args.next().expect("pass exp, ln, or norm_cdf");
    let paths: Vec<String> = args.collect();
    assert!(!paths.is_empty(), "pass one or more vector files");
    for path in paths {
        measure(&kind, &path);
    }
}
