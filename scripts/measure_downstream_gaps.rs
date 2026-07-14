//! Revalidate the retained downstream gaps after ln/CDF implementation changes.
//!
//! This runner is intentionally dependency-free. Build the library with
//! `full,table-gen`, compile this file with `rustc`, then pass the BVN, Phi2,
//! SABR, Heston, NIG-i128, and NIG-i64 JSON paths in that order.

use solmath::{
    black_scholes_price_hp, bvn_cdf, bvn_cdf_hp, heston_price, nig_call_64, nig_call_price,
    nig_put_64, sabr_greeks, sabr_implied_vol, sabr_precompute, sabr_price, sabr_vol_at,
    Phi2DenseTable, Phi2Table, SolMathError, PHI2_DENSE_GRID_SIZE, PHI2_GRID_SIZE, SCALE,
};
use std::{collections::BTreeMap, env, fs, time::Instant};

#[derive(Default)]
struct Stats {
    errors: Vec<u128>,
    failures: BTreeMap<String, usize>,
}

impl Stats {
    fn accept(&mut self, actual: u128, expected: u128) {
        self.errors.push(actual.abs_diff(expected));
    }

    fn accept_i(&mut self, actual: i128, expected: i128) {
        self.errors.push(actual.abs_diff(expected));
    }

    fn fail(&mut self, error: SolMathError) {
        *self.failures.entry(format!("{error:?}")).or_default() += 1;
    }

    fn failure_count(&self) -> usize {
        self.failures.values().sum()
    }

    fn print(mut self, name: &str) {
        self.errors.sort_unstable();
        let accepted = self.errors.len();
        let percentile = |numerator: usize, denominator: usize| -> u128 {
            if accepted == 0 {
                0
            } else {
                self.errors[(accepted - 1) * numerator / denominator]
            }
        };
        let exact = self.errors.partition_point(|error| *error == 0);
        let failures = self.failure_count();
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
    let quoted = bytes.get(cursor) == Some(&b'"');
    if quoted {
        cursor += 1;
    }
    let start = cursor;
    if bytes.get(cursor) == Some(&b'-') {
        cursor += 1;
    }
    while bytes.get(cursor).is_some_and(u8::is_ascii_digit) {
        cursor += 1;
    }
    if cursor == start || (cursor == start + 1 && bytes.get(start) == Some(&b'-')) {
        return None;
    }
    object[start..cursor].parse().ok()
}

fn unsigned_field(object: &str, key: &str) -> Option<u128> {
    let value = numeric_field(object, key)?;
    u128::try_from(value).ok()
}

fn for_each_object(path: &str, mut visit: impl FnMut(&str)) {
    let input = fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"));
    let mut cursor = input.find("\"vectors\"").unwrap_or(0);
    while let Some(relative_start) = input[cursor..].find('{') {
        let start = cursor + relative_start;
        let Some(relative_end) = input[start..].find('}') else {
            panic!("unterminated object in {path}");
        };
        let end = start + relative_end + 1;
        visit(&input[start..end]);
        cursor = end;
    }
}

#[derive(Clone, Copy)]
struct BvnRow {
    a: i128,
    b: i128,
    rho: i128,
    expected: i128,
}

fn bvn_tier(rho: i128) -> &'static str {
    let absolute = rho.unsigned_abs();
    if absolute <= 900_000_000_000 {
        "abs_rho_le_0.90"
    } else if absolute <= 950_000_000_000 {
        "0.90_lt_abs_rho_le_0.95"
    } else if absolute <= 990_000_000_000 {
        "0.95_lt_abs_rho_le_0.99"
    } else if absolute < SCALE {
        "0.99_lt_abs_rho_lt_1"
    } else {
        "abs_rho_eq_1"
    }
}

fn measure_bvn(path: &str) {
    let mut stats: BTreeMap<String, Stats> = BTreeMap::new();
    let mut rows = 0usize;
    for_each_object(path, |object| {
        let Some(a) = numeric_field(object, "a") else {
            return;
        };
        let row = BvnRow {
            a,
            b: numeric_field(object, "b").expect("b"),
            rho: numeric_field(object, "rho").expect("rho"),
            expected: numeric_field(object, "expected").expect("expected"),
        };
        rows += 1;
        for (function, result) in [
            ("bvn_cdf.GL6", bvn_cdf(row.a, row.b, row.rho)),
            ("bvn_cdf_hp.GL20", bvn_cdf_hp(row.a, row.b, row.rho)),
        ] {
            for tier in ["all", bvn_tier(row.rho)] {
                let entry = stats.entry(format!("{function}.{tier}")).or_default();
                match result {
                    Ok(actual) => entry.accept_i(actual, row.expected),
                    Err(error) => entry.fail(error),
                }
            }
        }
    });
    assert_eq!(rows, 22_500, "BVN corpus size");
    for (name, metric) in stats {
        metric.print(&name);
    }
}

fn load_bvn_rows(path: &str) -> Vec<BvnRow> {
    let mut rows = Vec::new();
    for_each_object(path, |object| {
        let Some(a) = numeric_field(object, "a") else {
            return;
        };
        rows.push(BvnRow {
            a,
            b: numeric_field(object, "b").expect("b"),
            rho: numeric_field(object, "rho").expect("rho"),
            expected: numeric_field(object, "expected").expect("expected"),
        });
    });
    rows
}

fn hex(bytes: [u8; 32]) -> String {
    let mut output = String::with_capacity(64);
    for byte in bytes {
        use std::fmt::Write;
        write!(output, "{byte:02x}").unwrap();
    }
    output
}

fn measure_phi2(path: &str) {
    let rows = load_bvn_rows(path);
    assert_eq!(rows.len(), 10_000, "Phi2 corpus size");
    let rhos = [
        -900_000_000_000i128,
        -500_000_000_000,
        0,
        500_000_000_000,
        900_000_000_000,
    ];
    let mut compatibility = Stats::default();
    let mut dense = Stats::default();
    let mut compatibility_bound_failures = 0usize;
    let mut dense_bound_failures = 0usize;

    for rho in rhos {
        let started = Instant::now();
        let table = Phi2Table::generate(rho, PHI2_GRID_SIZE).expect("64x64 generation");
        let certificate = table.certify(rho).expect("64x64 certification");
        let evaluator = table
            .certified(
                &certificate,
                certificate.certificate_id(),
                certificate.max_abs_error(),
            )
            .expect("64x64 guarded evaluator");
        println!(
            "phi2_certificate grid=64 rho={rho} node={} interpolation={} reference={} total={} id={} elapsed_ms={}",
            certificate.max_node_abs_error(),
            certificate.interpolation_abs_error_bound(),
            certificate.reference_abs_error_allowance(),
            certificate.max_abs_error(),
            hex(certificate.certificate_id()),
            started.elapsed().as_millis(),
        );
        for row in rows.iter().filter(|row| row.rho == rho) {
            match evaluator.eval(row.a, row.b) {
                Ok(actual) => {
                    let error = actual.abs_diff(row.expected);
                    compatibility_bound_failures +=
                        usize::from(error > certificate.max_abs_error() as u128);
                    compatibility.accept_i(actual, row.expected);
                }
                Err(error) => compatibility.fail(error),
            }
        }

        let started = Instant::now();
        let table =
            Phi2DenseTable::generate(rho, PHI2_DENSE_GRID_SIZE).expect("129x129 generation");
        let certificate = table.certify(rho).expect("129x129 certification");
        let evaluator = table
            .certified(
                &certificate,
                certificate.certificate_id(),
                certificate.max_abs_error(),
            )
            .expect("129x129 guarded evaluator");
        println!(
            "phi2_certificate grid=129 rho={rho} node={} interpolation={} reference={} total={} id={} elapsed_ms={}",
            certificate.max_node_abs_error(),
            certificate.interpolation_abs_error_bound(),
            certificate.reference_abs_error_allowance(),
            certificate.max_abs_error(),
            hex(certificate.certificate_id()),
            started.elapsed().as_millis(),
        );
        for row in rows.iter().filter(|row| row.rho == rho) {
            match evaluator.eval(row.a, row.b) {
                Ok(actual) => {
                    let error = actual.abs_diff(row.expected);
                    dense_bound_failures +=
                        usize::from(error > certificate.max_abs_error() as u128);
                    dense.accept_i(actual, row.expected);
                }
                Err(error) => dense.fail(error),
            }
        }
    }

    compatibility.print("Phi2Table.certified.off_grid");
    dense.print("Phi2DenseTable.certified.off_grid");
    println!(
        "phi2_bound_failures compatibility={compatibility_bound_failures} dense={dense_bound_failures}"
    );
    assert_eq!(compatibility_bound_failures, 0);
    assert_eq!(dense_bound_failures, 0);

    // Exercise and cross-check the one-shot generate+certify entrypoints too.
    const TEST_RHO: i128 = 750_000_000_000;
    let separate = Phi2Table::generate(TEST_RHO, PHI2_GRID_SIZE).unwrap();
    let separate_certificate = separate.certify(TEST_RHO).unwrap();
    let (one_shot, one_shot_certificate) =
        Phi2Table::generate_certified(TEST_RHO, PHI2_GRID_SIZE).unwrap();
    assert_eq!(separate.as_array(), one_shot.as_array());
    assert_eq!(separate_certificate, one_shot_certificate);

    let separate = Phi2DenseTable::generate(TEST_RHO, PHI2_DENSE_GRID_SIZE).unwrap();
    let separate_certificate = separate.certify(TEST_RHO).unwrap();
    let (one_shot, one_shot_certificate) =
        Phi2DenseTable::generate_certified(TEST_RHO, PHI2_DENSE_GRID_SIZE).unwrap();
    assert_eq!(separate.as_array(), one_shot.as_array());
    assert_eq!(separate_certificate, one_shot_certificate);
    println!("phi2_generate_certified_equivalence grid_64=true grid_129=true rho={TEST_RHO}");
}

fn measure_sabr(path: &str) {
    let mut implied = Stats::default();
    let mut batch = Stats::default();
    let mut batch_vs_direct = Stats::default();
    let mut price_call = Stats::default();
    let mut price_put = Stats::default();
    let mut greeks_call = Stats::default();
    let mut greeks_put = Stats::default();
    let mut rows = 0usize;

    for_each_object(path, |object| {
        let Some(forward) = unsigned_field(object, "F_fp") else {
            return;
        };
        rows += 1;
        let strike = unsigned_field(object, "K_fp").unwrap();
        let time = unsigned_field(object, "T_fp").unwrap();
        let alpha = unsigned_field(object, "alpha_fp").unwrap();
        let beta = unsigned_field(object, "beta_fp").unwrap();
        let rho = numeric_field(object, "rho_fp").unwrap();
        let nu = unsigned_field(object, "nu_fp").unwrap();
        let expected_vol = unsigned_field(object, "vol_fp").unwrap();

        let direct = sabr_implied_vol(forward, strike, time, alpha, beta, rho, nu);
        match direct {
            Ok(actual) => implied.accept(actual, expected_vol),
            Err(error) => implied.fail(error),
        }
        match sabr_precompute(forward, time, alpha, beta, rho, nu)
            .and_then(|precomputed| sabr_vol_at(&precomputed, strike))
        {
            Ok(actual) => {
                batch.accept(actual, expected_vol);
                if let Ok(direct_actual) = direct {
                    batch_vs_direct.accept(actual, direct_actual);
                }
            }
            Err(error) => {
                batch.fail(error);
                batch_vs_direct.fail(error);
            }
        }

        let reference = black_scholes_price_hp(forward, strike, 0, expected_vol, time);
        match (
            sabr_price(forward, strike, 0, time, alpha, beta, rho, nu),
            reference,
        ) {
            (Ok((call, put)), Ok((expected_call, expected_put))) => {
                price_call.accept(call, expected_call);
                price_put.accept(put, expected_put);
            }
            (Err(error), _) => {
                price_call.fail(error);
                price_put.fail(error);
            }
            (_, Err(error)) => {
                price_call.fail(error);
                price_put.fail(error);
            }
        }
        match (
            sabr_greeks(forward, strike, 0, time, alpha, beta, rho, nu),
            reference,
        ) {
            (Ok(greeks), Ok((expected_call, expected_put))) => {
                greeks_call.accept(greeks.call, expected_call);
                greeks_put.accept(greeks.put, expected_put);
            }
            (Err(error), _) => {
                greeks_call.fail(error);
                greeks_put.fail(error);
            }
            (_, Err(error)) => {
                greeks_call.fail(error);
                greeks_put.fail(error);
            }
        }
    });
    assert_eq!(rows, 100_000, "SABR corpus size");
    implied.print("sabr_implied_vol.QuantLib_100K");
    batch.print("sabr_precompute_vol_at.QuantLib_100K");
    batch_vs_direct.print("sabr_batch_vs_direct_100K");
    price_call.print("sabr_price.call_vs_HP_at_QuantLib_vol");
    price_put.print("sabr_price.put_vs_HP_at_QuantLib_vol");
    greeks_call.print("sabr_greeks.call_vs_HP_at_QuantLib_vol");
    greeks_put.print("sabr_greeks.put_vs_HP_at_QuantLib_vol");
}

fn measure_heston_fail_closed(path: &str) {
    let mut accepted = 0usize;
    let mut failures: BTreeMap<String, usize> = BTreeMap::new();
    let mut rows = 0usize;
    for_each_object(path, |object| {
        let Some(spot) = unsigned_field(object, "S_fp") else {
            return;
        };
        rows += 1;
        match heston_price(
            spot,
            unsigned_field(object, "K_fp").unwrap(),
            unsigned_field(object, "r_fp").unwrap(),
            unsigned_field(object, "T_fp").unwrap(),
            unsigned_field(object, "v0_fp").unwrap(),
            unsigned_field(object, "kappa_fp").unwrap(),
            unsigned_field(object, "theta_fp").unwrap(),
            unsigned_field(object, "xi_fp").unwrap(),
            numeric_field(object, "rho_fp").unwrap(),
        ) {
            Ok(_) => accepted += 1,
            Err(error) => *failures.entry(format!("{error:?}")).or_default() += 1,
        }
    });
    assert_eq!(rows, 100_000, "Heston corpus size");
    println!(
        "metric=heston_stochastic_fail_closed vectors={rows} accepted={accepted} failure_kinds={failures:?}"
    );
    assert_eq!(accepted, 0);
    assert_eq!(failures.get("NoConvergence"), Some(&rows));
}

fn measure_nig_fail_closed(i128_path: &str, i64_path: &str) {
    let mut accepted = 0usize;
    let mut failures: BTreeMap<String, usize> = BTreeMap::new();
    let mut rows = 0usize;
    for_each_object(i128_path, |object| {
        let Some(spot) = unsigned_field(object, "s") else {
            return;
        };
        rows += 1;
        match nig_call_price(
            spot,
            unsigned_field(object, "k").unwrap(),
            unsigned_field(object, "r").unwrap(),
            unsigned_field(object, "t").unwrap(),
            unsigned_field(object, "alpha").unwrap(),
            numeric_field(object, "beta").unwrap(),
            unsigned_field(object, "delta").unwrap(),
        ) {
            Ok(_) => accepted += 1,
            Err(error) => *failures.entry(format!("{error:?}")).or_default() += 1,
        }
    });
    assert_eq!(rows, 1_000, "NIG i128 corpus size");
    println!(
        "metric=nig_call_price_fail_closed vectors={rows} accepted={accepted} failure_kinds={failures:?}"
    );
    assert_eq!(accepted, 0);
    assert_eq!(failures.get("NoConvergence"), Some(&rows));

    let mut call_accepted = 0usize;
    let mut put_accepted = 0usize;
    let mut call_failures: BTreeMap<String, usize> = BTreeMap::new();
    let mut put_failures: BTreeMap<String, usize> = BTreeMap::new();
    let mut rows = 0usize;
    for_each_object(i64_path, |object| {
        let Some(spot) = numeric_field(object, "s").and_then(|value| i64::try_from(value).ok())
        else {
            return;
        };
        rows += 1;
        let strike = i64::try_from(numeric_field(object, "k").unwrap()).unwrap();
        let rate = i64::try_from(numeric_field(object, "r").unwrap()).unwrap();
        let time = i64::try_from(numeric_field(object, "t").unwrap()).unwrap();
        let alpha = i64::try_from(numeric_field(object, "alpha").unwrap()).unwrap();
        let beta = i64::try_from(numeric_field(object, "beta").unwrap()).unwrap();
        let delta = i64::try_from(numeric_field(object, "delta_param").unwrap()).unwrap();
        match nig_call_64(spot, strike, rate, time, alpha, beta, delta) {
            Ok(_) => call_accepted += 1,
            Err(error) => *call_failures.entry(format!("{error:?}")).or_default() += 1,
        }
        match nig_put_64(spot, strike, rate, time, alpha, beta, delta) {
            Ok(_) => put_accepted += 1,
            Err(error) => *put_failures.entry(format!("{error:?}")).or_default() += 1,
        }
    });
    assert_eq!(rows, 200, "NIG i64 corpus size");
    println!(
        "metric=nig_call_64_fail_closed vectors={rows} accepted={call_accepted} failure_kinds={call_failures:?}"
    );
    println!(
        "metric=nig_put_64_fail_closed vectors={rows} accepted={put_accepted} failure_kinds={put_failures:?}"
    );
    assert_eq!(call_accepted, 0);
    assert_eq!(put_accepted, 0);
    assert_eq!(call_failures.get("NoConvergence"), Some(&rows));
    assert_eq!(put_failures.get("NoConvergence"), Some(&rows));
}

fn main() {
    let paths: Vec<String> = env::args().skip(1).collect();
    assert_eq!(
        paths.len(),
        6,
        "pass BVN, Phi2, SABR, Heston, NIG-i128, and NIG-i64 JSON paths"
    );
    measure_bvn(&paths[0]);
    measure_phi2(&paths[1]);
    measure_sabr(&paths[2]);
    measure_heston_fail_closed(&paths[3]);
    measure_nig_fail_closed(&paths[4], &paths[5]);
}
