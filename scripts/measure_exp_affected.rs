//! Dependency-free retained-corpus runner for paths that call `exp_fixed_i`.
//!
//! Build `solmath` with all features, compile this file with `rustc`, then
//! pass the benchmark directory containing the retained JSON vectors.

use solmath::{
    black_scholes_price, bs_delta, bs_full, bs_gamma, bs_rho, bs_theta, bs_vega, exp_fixed_i,
    fp_mul_i, implied_vol, ln_fixed_i, norm_cdf_and_pdf, norm_pdf, pow_fixed, pow_fixed_i,
    SolMathError,
};
use std::{collections::BTreeMap, env, fs, path::Path};

#[derive(Default)]
struct Stats {
    errors: Vec<u128>,
    failures: BTreeMap<String, usize>,
}

impl Stats {
    fn accept_u(&mut self, actual: u128, expected: u128) {
        self.errors.push(actual.abs_diff(expected));
    }

    fn accept_i(&mut self, actual: i128, expected: i128) {
        self.errors.push(actual.abs_diff(expected));
    }

    fn fail(&mut self, error: SolMathError) {
        *self.failures.entry(format!("{error:?}")).or_default() += 1;
    }

    fn print(mut self, name: &str) {
        self.errors.sort_unstable();
        let accepted = self.errors.len();
        let failures: usize = self.failures.values().sum();
        let percentile = |numerator: usize, denominator: usize| -> u128 {
            if accepted == 0 {
                0
            } else {
                self.errors[(accepted - 1) * numerator / denominator]
            }
        };
        let exact = self.errors.partition_point(|error| *error == 0);
        println!(
            "metric={name} vectors={} accepted={accepted} failures={failures} max={} p99={} p95={} median={} exact={} exact_pct={:.6} failure_kinds={:?}",
            accepted + failures,
            self.errors.last().copied().unwrap_or(0),
            percentile(99, 100),
            percentile(95, 100),
            percentile(1, 2),
            exact,
            if accepted == 0 { 0.0 } else { exact as f64 * 100.0 / accepted as f64 },
            self.failures,
        );
    }
}

fn numeric_field(object: &str, key: &str) -> Option<i128> {
    let needle = format!("\"{key}\"");
    let key_at = object.find(&needle)?;
    let colon = object[key_at + needle.len()..].find(':')? + key_at + needle.len();
    let bytes = object.as_bytes();
    let mut cursor = colon + 1;
    while bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
        cursor += 1;
    }
    if bytes.get(cursor) == Some(&b'\"') {
        cursor += 1;
    }
    let start = cursor;
    if bytes.get(cursor) == Some(&b'-') {
        cursor += 1;
    }
    while bytes.get(cursor).is_some_and(u8::is_ascii_digit) {
        cursor += 1;
    }
    object[start..cursor].parse().ok()
}

fn unsigned_field(object: &str, key: &str) -> Option<u128> {
    u128::try_from(numeric_field(object, key)?).ok()
}

fn for_each_object(path: &Path, mut visit: impl FnMut(&str)) -> usize {
    let input = fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path:?}: {error}"));
    let mut cursor = input.find("\"vectors\"").unwrap_or(0);
    let mut rows = 0;
    while let Some(relative_start) = input[cursor..].find('{') {
        let start = cursor + relative_start;
        let end = start + input[start..].find('}').expect("terminated JSON object") + 1;
        let object = &input[start..end];
        if object.contains("\"expected\"")
            || object.contains("\"expected_pdf\"")
            || object.contains("\"call\"")
            || object.contains("\"call_price\"")
        {
            visit(object);
            rows += 1;
        }
        cursor = end;
    }
    rows
}

fn measure_norm_pdf(directory: &Path) {
    let mut direct = Stats::default();
    let rows = for_each_object(&directory.join("prod_norm_pdf_vectors.json"), |object| {
        let x = numeric_field(object, "x").expect("x");
        let expected = numeric_field(object, "expected").expect("expected");
        match norm_pdf(x) {
            Ok(actual) => direct.accept_i(actual, expected),
            Err(error) => direct.fail(error),
        }
    });
    assert_eq!(rows, 100_000);
    direct.print("norm_pdf.production");

    let mut cdf = Stats::default();
    let mut pdf = Stats::default();
    let rows = for_each_object(&directory.join("prod_cdf_pdf_vectors.json"), |object| {
        let x = numeric_field(object, "x").expect("x");
        let expected_cdf = numeric_field(object, "expected_cdf").expect("expected_cdf");
        let expected_pdf = numeric_field(object, "expected_pdf").expect("expected_pdf");
        match norm_cdf_and_pdf(x) {
            Ok((actual_cdf, actual_pdf)) => {
                cdf.accept_i(actual_cdf, expected_cdf);
                pdf.accept_i(actual_pdf, expected_pdf);
            }
            Err(error) => {
                cdf.fail(error);
                pdf.fail(error);
            }
        }
    });
    assert_eq!(rows, 50_000);
    cdf.print("norm_cdf_and_pdf.cdf.production");
    pdf.print("norm_cdf_and_pdf.pdf.production");
}

fn measure_pow(directory: &Path) {
    for file in ["prod_pow_fixed_vectors.json", "adv_pow_fixed_vectors.json"] {
        let mut stats = Stats::default();
        let rows = for_each_object(&directory.join(file), |object| {
            let base = unsigned_field(object, "base").expect("base");
            let exponent = unsigned_field(object, "exp").expect("exp");
            let expected = unsigned_field(object, "expected").expect("expected");
            match pow_fixed(base, exponent) {
                Ok(actual) => stats.accept_u(actual, expected),
                Err(error) => stats.fail(error),
            }
        });
        stats.print(&format!("pow_fixed.{file}.rows_{rows}"));
    }

    let mut signed = Stats::default();
    let rows = for_each_object(&directory.join("prod_pow_fixed_i_vectors.json"), |object| {
        let base = numeric_field(object, "base").expect("base");
        let exponent = numeric_field(object, "exp").expect("exp");
        let expected = numeric_field(object, "expected").expect("expected");
        match pow_fixed_i(base, exponent) {
            Ok(actual) => signed.accept_i(actual, expected),
            Err(error) => signed.fail(error),
        }
    });
    assert_eq!(rows, 50_000);
    signed.print("pow_fixed_i.production");
}

#[derive(Clone, Copy)]
struct BsRow {
    s: u128,
    k: u128,
    r: u128,
    sigma: u128,
    t: u128,
    call: u128,
    put: u128,
    call_delta: Option<i128>,
    put_delta: Option<i128>,
    gamma: Option<i128>,
    vega: Option<i128>,
    call_theta: Option<i128>,
    put_theta: Option<i128>,
    call_rho: Option<i128>,
    put_rho: Option<i128>,
}

fn bs_row(object: &str) -> BsRow {
    BsRow {
        s: unsigned_field(object, "s").expect("s"),
        k: unsigned_field(object, "k").expect("k"),
        r: unsigned_field(object, "r").expect("r"),
        sigma: unsigned_field(object, "sigma").expect("sigma"),
        t: unsigned_field(object, "t").expect("t"),
        call: unsigned_field(object, "call").expect("call"),
        put: unsigned_field(object, "put").expect("put"),
        call_delta: numeric_field(object, "call_delta"),
        put_delta: numeric_field(object, "put_delta"),
        gamma: numeric_field(object, "gamma"),
        vega: numeric_field(object, "vega"),
        call_theta: numeric_field(object, "call_theta"),
        put_theta: numeric_field(object, "put_theta"),
        call_rho: numeric_field(object, "call_rho"),
        put_rho: numeric_field(object, "put_rho"),
    }
}

fn measure_bs_price(directory: &Path) {
    let mut call = Stats::default();
    let mut put = Stats::default();
    let rows = for_each_object(
        &directory.join("prod_black_scholes_price_vectors.json"),
        |object| {
            let row = bs_row(object);
            match black_scholes_price(row.s, row.k, row.r, row.sigma, row.t) {
                Ok((actual_call, actual_put)) => {
                    call.accept_u(actual_call, row.call);
                    put.accept_u(actual_put, row.put);
                }
                Err(error) => {
                    call.fail(error);
                    put.fail(error);
                }
            }
        },
    );
    assert_eq!(rows, 50_000);
    call.print("black_scholes_price.call.production");
    put.print("black_scholes_price.put.production");
}

fn measure_bs_full_file(directory: &Path, file: &str) {
    let mut stats: BTreeMap<&'static str, Stats> = BTreeMap::new();
    let rows = for_each_object(&directory.join(file), |object| {
        let row = bs_row(object);
        match bs_full(row.s, row.k, row.r, row.sigma, row.t) {
            Ok(actual) => {
                stats
                    .entry("full.call")
                    .or_default()
                    .accept_u(actual.call, row.call);
                stats
                    .entry("full.put")
                    .or_default()
                    .accept_u(actual.put, row.put);
                stats
                    .entry("full.call_delta")
                    .or_default()
                    .accept_i(actual.call_delta, row.call_delta.unwrap());
                stats
                    .entry("full.put_delta")
                    .or_default()
                    .accept_i(actual.put_delta, row.put_delta.unwrap());
                stats
                    .entry("full.gamma")
                    .or_default()
                    .accept_i(actual.gamma, row.gamma.unwrap());
                stats
                    .entry("full.vega")
                    .or_default()
                    .accept_i(actual.vega, row.vega.unwrap());
                stats
                    .entry("full.call_theta")
                    .or_default()
                    .accept_i(actual.call_theta, row.call_theta.unwrap());
                stats
                    .entry("full.put_theta")
                    .or_default()
                    .accept_i(actual.put_theta, row.put_theta.unwrap());
                stats
                    .entry("full.call_rho")
                    .or_default()
                    .accept_i(actual.call_rho, row.call_rho.unwrap());
                stats
                    .entry("full.put_rho")
                    .or_default()
                    .accept_i(actual.put_rho, row.put_rho.unwrap());
            }
            Err(error) => {
                for name in [
                    "full.call",
                    "full.put",
                    "full.call_delta",
                    "full.put_delta",
                    "full.gamma",
                    "full.vega",
                    "full.call_theta",
                    "full.put_theta",
                    "full.call_rho",
                    "full.put_rho",
                ] {
                    stats.entry(name).or_default().fail(error);
                }
            }
        }

        let mut pair =
            |name_a, name_b, result: Result<(i128, i128), SolMathError>, expected_a, expected_b| {
                match result {
                    Ok((a, b)) => {
                        stats.entry(name_a).or_default().accept_i(a, expected_a);
                        stats.entry(name_b).or_default().accept_i(b, expected_b);
                    }
                    Err(error) => {
                        stats.entry(name_a).or_default().fail(error);
                        stats.entry(name_b).or_default().fail(error);
                    }
                }
            };
        pair(
            "delta.call",
            "delta.put",
            bs_delta(row.s, row.k, row.r, row.sigma, row.t),
            row.call_delta.unwrap(),
            row.put_delta.unwrap(),
        );
        pair(
            "theta.call",
            "theta.put",
            bs_theta(row.s, row.k, row.r, row.sigma, row.t),
            row.call_theta.unwrap(),
            row.put_theta.unwrap(),
        );
        pair(
            "rho.call",
            "rho.put",
            bs_rho(row.s, row.k, row.r, row.sigma, row.t),
            row.call_rho.unwrap(),
            row.put_rho.unwrap(),
        );
        for (name, result, expected) in [
            (
                "gamma",
                bs_gamma(row.s, row.k, row.r, row.sigma, row.t),
                row.gamma.unwrap(),
            ),
            (
                "vega",
                bs_vega(row.s, row.k, row.r, row.sigma, row.t),
                row.vega.unwrap(),
            ),
        ] {
            match result {
                Ok(actual) => stats.entry(name).or_default().accept_i(actual, expected),
                Err(error) => stats.entry(name).or_default().fail(error),
            }
        }
    });
    for (name, metric) in stats {
        metric.print(&format!("bs.{name}.{file}.rows_{rows}"));
    }
}

fn measure_iv(directory: &Path) {
    for file in [
        "prod_implied_vol_vectors.json",
        "adv_implied_vol_vectors.json",
    ] {
        let mut stats = Stats::default();
        let rows = for_each_object(&directory.join(file), |object| {
            let market = unsigned_field(object, "call_price").expect("call_price");
            let s = unsigned_field(object, "s").expect("s");
            let k = unsigned_field(object, "k").expect("k");
            let r = unsigned_field(object, "r").expect("r");
            let t = unsigned_field(object, "t").expect("t");
            let expected = unsigned_field(object, "sigma").expect("sigma");
            match implied_vol(market, s, k, r, t) {
                Ok(actual) => stats.accept_u(actual, expected),
                Err(error) => stats.fail(error),
            }
        });
        stats.print(&format!("implied_vol.{file}.rows_{rows}"));
    }
}

fn dump_iv_outcomes(directory: &Path, file: &str) {
    let mut index = 0usize;
    for_each_object(&directory.join(file), |object| {
        let market = unsigned_field(object, "call_price").expect("call_price");
        let s = unsigned_field(object, "s").expect("s");
        let k = unsigned_field(object, "k").expect("k");
        let r = unsigned_field(object, "r").expect("r");
        let t = unsigned_field(object, "t").expect("t");
        let expected = unsigned_field(object, "sigma").expect("sigma");
        let r_t = fp_mul_i(r as i128, t as i128).expect("r*t");
        let discount = exp_fixed_i(-r_t).expect("discount");
        let k_disc = fp_mul_i(k as i128, discount).expect("discounted strike");
        let lower = (s as i128).saturating_sub(k_disc).max(0) as u128;
        let lower_margin = market as i128 - lower as i128;
        let expected_call = black_scholes_price(s, k, r, expected, t)
            .expect("BS at expected sigma")
            .0;
        let expected_price_error = expected_call.abs_diff(market);
        match implied_vol(market, s, k, r, t) {
            Ok(actual) => {
                let result_call = black_scholes_price(s, k, r, actual, t)
                    .expect("BS at recovered sigma")
                    .0;
                let result_price_error = result_call.abs_diff(market);
                println!(
                    "iv_outcome\t{index}\t{s}\t{k}\t{r}\t{t}\t{expected}\t{market}\t{discount}\t{lower}\t{lower_margin}\tOk\t{actual}\t{expected_call}\t{expected_price_error}\t{result_call}\t{result_price_error}"
                );
            }
            Err(error) => println!(
                "iv_outcome\t{index}\t{s}\t{k}\t{r}\t{t}\t{expected}\t{market}\t{discount}\t{lower}\t{lower_margin}\tErr\t{error:?}\t{expected_call}\t{expected_price_error}\t-\t-"
            ),
        }
        index += 1;
    });
}

fn dump_pow_bs_outcomes(directory: &Path) {
    let mut index = 0usize;
    for_each_object(&directory.join("prod_pow_fixed_vectors.json"), |object| {
        let base = unsigned_field(object, "base").expect("base");
        let exponent = unsigned_field(object, "exp").expect("exp");
        let expected = unsigned_field(object, "expected").expect("expected");
        let actual = pow_fixed(base, exponent).expect("pow_fixed");
        let ln_base = ln_fixed_i(base).expect("ln(base)");
        let product = i128::try_from(exponent)
            .ok()
            .and_then(|value| fp_mul_i(value, ln_base).ok());
        let exp_value = product.and_then(|value| exp_fixed_i(value).ok());
        println!(
            "pow_outcome\t{index}\t{base}\t{exponent}\t{expected}\t{actual}\t{}\t{}",
            product.map_or_else(|| "-".into(), |value| value.to_string()),
            exp_value.map_or_else(|| "-".into(), |value| value.to_string()),
        );
        index += 1;
    });

    index = 0;
    for_each_object(
        &directory.join("prod_black_scholes_price_vectors.json"),
        |object| {
            let row = bs_row(object);
            let (call, put) = black_scholes_price(row.s, row.k, row.r, row.sigma, row.t)
                .expect("black_scholes_price");
            let r_t = fp_mul_i(row.r as i128, row.t as i128).expect("r*t");
            let discount = exp_fixed_i(-r_t).expect("discount");
            println!(
                "bs_outcome\t{index}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{call}\t{put}\t{r_t}\t{discount}",
                row.s, row.k, row.r, row.sigma, row.t, row.call, row.put,
            );
            index += 1;
        },
    );
}

fn main() {
    let directory = env::args().nth(1).expect("benchmark directory");
    let directory = Path::new(&directory);
    if env::args().nth(2).as_deref() == Some("dump-iv") {
        dump_iv_outcomes(directory, "prod_implied_vol_vectors.json");
        return;
    }
    if env::args().nth(2).as_deref() == Some("dump-pow-bs") {
        dump_pow_bs_outcomes(directory);
        return;
    }
    measure_norm_pdf(directory);
    measure_pow(directory);
    measure_bs_price(directory);
    measure_bs_full_file(directory, "prod_bs_full_vectors.json");
    measure_bs_full_file(directory, "adv_bs_full_vectors.json");
    measure_iv(directory);
}
