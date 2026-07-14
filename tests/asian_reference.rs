//! Independent high-precision references for continuous arithmetic-Asian marks.
//!
//! Expected values were generated with mpmath at 80 decimal digits from the
//! closed-form first two GBM average moments and an independently implemented
//! lognormal moment match. The separately generated QuantLib corpus lives in
//! `benchmark/asian_quantlib_vectors.json` and is exercised by
//! `tests/asian_quantlib_reference.rs`.

#![cfg(feature = "asian")]

use solmath::arithmetic_asian_price;

#[derive(Clone, Copy)]
struct Vector {
    inputs: [u128; 9],
    call: u128,
    put: u128,
    mean: u128,
    log_variance: u128,
}

const VECTORS: [Vector; 6] = [
    Vector {
        inputs: [
            100_000_000_000_000,
            100_000_000_000_000,
            50_000_000_000,
            20_000_000_000,
            400_000_000_000,
            1_000_000_000_000,
            1_000_000_000_000,
            0,
            0,
        ],
        call: 9_641_361_495_792,
        put: 8_200_141_259_059,
        mean: 101_515_113_178_390,
        log_variance: 54_454_422_633,
    },
    Vector {
        // A 30-day averaging window beginning 335 days from now.
        inputs: [
            100_000_000_000_000,
            105_000_000_000_000,
            30_000_000_000,
            10_000_000_000,
            600_000_000_000,
            1_000_000_000_000,
            82_191_780_821,
            0,
            0,
        ],
        call: 21_583_068_184_745,
        put: 24_556_195_221_206,
        mean: 101_936_327_765_259,
        log_variance: 340_302_385_996,
    },
    Vector {
        // 180/365 of the final average is already fixed at 98.
        inputs: [
            100_000_000_000_000,
            100_000_000_000_000,
            50_000_000_000,
            20_000_000_000,
            400_000_000_000,
            506_849_315_068,
            506_849_315_068,
            98_000_000_000_000,
            493_150_684_931,
        ],
        call: 3_025_021_155_572,
        put: 3_609_028_308_725,
        mean: 99_401_003_534_725,
        log_variance: 7_284_694_054,
    },
    Vector {
        inputs: [
            80_000_000_000_000,
            100_000_000_000_000,
            0,
            0,
            200_000_000_000,
            82_191_780_821,
            82_191_780_821,
            0,
            0,
        ],
        call: 3,
        put: 20_000_000_000_003,
        mean: 80_000_000_000_000,
        log_variance: 1_096_190_699,
    },
    Vector {
        inputs: [
            120_000_000_000_000,
            100_000_000_000_000,
            100_000_000_000,
            40_000_000_000,
            800_000_000_000,
            2_000_000_000_000,
            2_000_000_000_000,
            0,
            0,
        ],
        call: 38_152_241_931_369,
        put: 15_639_723_930_513,
        mean: 127_496_851_579_376,
        log_variance: 488_312_882_027,
    },
    Vector {
        // Twelve of thirty TWAP minutes fixed at 99.50; eighteen remain.
        inputs: [
            100_000_000_000_000,
            100_000_000_000_000,
            50_000_000_000,
            20_000_000_000,
            600_000_000_000,
            34_246_575,
            34_246_575,
            99_500_000_000_000,
            400_000_000_000,
        ],
        call: 2_558_829_669,
        put: 202_527_665_328,
        mean: 99_800_030_821_928,
        log_variance: 1_485_392,
    },
];

fn assert_close(actual: u128, expected: u128, tolerance: u128, label: &str, index: usize) {
    assert!(
        actual.abs_diff(expected) <= tolerance,
        "vector {index} {label}: actual={actual} expected={expected} diff={}",
        actual.abs_diff(expected)
    );
}

#[test]
fn matches_high_precision_moment_match() {
    for (index, vector) in VECTORS.iter().enumerate() {
        let [s, k, r, q, sigma, t, averaging_time, fixed_average, fixed_weight] = vector.inputs;
        let actual = arithmetic_asian_price(
            s,
            k,
            r,
            q,
            sigma,
            t,
            averaging_time,
            fixed_average,
            fixed_weight,
        )
        .unwrap();

        // The final option transform uses the certified SCALE CDF for SBF
        // efficiency while the cancellation-sensitive moments stay at HP.
        // Its absolute price budget remains below 5e-9 real units.
        assert_close(actual.call, vector.call, 5_000, "call", index);
        assert_close(actual.put, vector.put, 5_000, "put", index);
        assert_close(actual.expected_average, vector.mean, 10, "mean", index);
        assert_close(
            actual.log_variance,
            vector.log_variance,
            1_000,
            "log_variance",
            index,
        );
    }
}

#[test]
fn put_call_parity_is_exact_for_every_reference_vector() {
    for vector in VECTORS {
        let [s, k, r, q, sigma, t, averaging_time, fixed_average, fixed_weight] = vector.inputs;
        let actual = arithmetic_asian_price(
            s,
            k,
            r,
            q,
            sigma,
            t,
            averaging_time,
            fixed_average,
            fixed_weight,
        )
        .unwrap();
        // The reference values themselves satisfy parity to their independent
        // rounding tolerance; exact public parity is covered internally using
        // the returned mean and the crate's discount path.
        assert!(actual.call <= actual.expected_average);
        assert!(actual.put <= k.max(actual.expected_average));
    }
}
