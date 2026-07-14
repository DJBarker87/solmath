//! Dependency-free accuracy runner for the generated ln_1p vector corpora.
//!
//! Build and run from the repository root:
//! `cargo build --release --all-features`
//! `rustc --edition=2021 scripts/measure_ln_1p_vectors.rs \
//!    --extern solmath=target/release/libsolmath.rlib \
//!    -L dependency=target/release/deps -o target/measure-ln-1p`
//! `target/measure-ln-1p benchmark/prod_ln_1p_vectors.json \
//!    benchmark/adv_ln_1p_vectors.json`

use solmath::ln_1p_fixed;
use std::{collections::BTreeMap, env, fs};

fn quoted_i128_after(input: &str, key: &str, start: usize) -> Option<(i128, usize)> {
    let key_at = input[start..].find(key)? + start;
    let colon = input[key_at + key.len()..].find(':')? + key_at + key.len();
    let mut cursor = colon + 1;
    while input.as_bytes().get(cursor)?.is_ascii_whitespace() {
        cursor += 1;
    }
    let quoted = input.as_bytes().get(cursor) == Some(&b'"');
    if quoted {
        cursor += 1;
    }
    let number_start = cursor;
    if input.as_bytes().get(cursor) == Some(&b'-') {
        cursor += 1;
    }
    while input.as_bytes().get(cursor).is_some_and(u8::is_ascii_digit) {
        cursor += 1;
    }
    if cursor == number_start || (cursor == number_start + 1 && &input[number_start..cursor] == "-") {
        return None;
    }
    if quoted && input.as_bytes().get(cursor) != Some(&b'"') {
        return None;
    }
    Some((input[number_start..cursor].parse().ok()?, cursor + usize::from(quoted)))
}

fn quoted_string_after(input: &str, key: &str, start: usize) -> Option<(String, usize)> {
    let key_at = input[start..].find(key)? + start;
    let colon = input[key_at + key.len()..].find(':')? + key_at + key.len();
    let quote = input[colon + 1..].find('"')? + colon + 1;
    let end = input[quote + 1..].find('"')? + quote + 1;
    Some((input[quote + 1..end].to_owned(), end + 1))
}

fn load(path: &str) -> Vec<(i128, i128, String)> {
    let input = fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"));
    let mut rows = Vec::new();
    let mut cursor = input.find("\"vectors\"").expect("vectors array");
    while let Some((x, after_x)) = quoted_i128_after(&input, "\"x\"", cursor) {
        let Some((expected, after_expected)) =
            quoted_i128_after(&input, "\"expected\"", after_x)
        else {
            break;
        };
        let (category, after_category) = quoted_string_after(
            &input,
            "\"category\"",
            after_expected,
        )
        .unwrap_or_else(|| ("uncategorized".to_owned(), after_expected));
        rows.push((x, expected, category));
        cursor = after_category;
    }
    rows
}

fn percentile(sorted: &[u128], numerator: usize, denominator: usize) -> u128 {
    sorted[(sorted.len() - 1) * numerator / denominator]
}

fn print_stats(file: &str, category: &str, errors: &mut [u128], failures: usize) {
    errors.sort_unstable();
    let exact = errors.partition_point(|error| *error == 0);
    println!(
        "{{\"file\":\"{}\",\"category\":\"{}\",\"vectors\":{},\"accepted\":{},\"failures\":{},\"max\":{},\"p99\":{},\"p95\":{},\"median\":{},\"exact\":{},\"exact_pct\":{:.6}}}",
        file,
        category,
        errors.len() + failures,
        errors.len(),
        failures,
        errors.last().copied().unwrap_or(0),
        percentile(errors, 99, 100),
        percentile(errors, 95, 100),
        percentile(errors, 1, 2),
        exact,
        exact as f64 * 100.0 / errors.len() as f64,
    );
}

fn measure(path: &str) {
    let rows = load(path);
    let mut errors = Vec::with_capacity(rows.len());
    let mut failures = 0usize;
    let mut categories: BTreeMap<String, Vec<u128>> = BTreeMap::new();
    let mut max_error = 0u128;
    let mut worst_rows = Vec::new();
    for (x, expected, category) in rows {
        match ln_1p_fixed(x) {
            Ok(actual) => {
                let error = actual.abs_diff(expected);
                if error > max_error {
                    max_error = error;
                    worst_rows.clear();
                }
                if error == max_error && worst_rows.len() < 20 {
                    worst_rows.push((x, expected, actual, category.clone()));
                }
                errors.push(error);
                categories.entry(category).or_default().push(error);
            }
            Err(_) => failures += 1,
        }
    }
    print_stats(path, "all", &mut errors, failures);
    for (category, mut category_errors) in categories {
        print_stats(path, &category, &mut category_errors, 0);
    }
    for (x, expected, actual, category) in worst_rows {
        println!(
            "{{\"file\":\"{}\",\"worst\":true,\"x\":\"{}\",\"category\":\"{}\",\"expected\":\"{}\",\"actual\":\"{}\",\"error\":{}}}",
            path, x, category, expected, actual, max_error
        );
    }
}

fn main() {
    let paths: Vec<String> = env::args().skip(1).collect();
    assert!(!paths.is_empty(), "pass one or more vector JSON files");
    for path in paths {
        measure(&path);
    }
}
