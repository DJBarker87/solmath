//! Contract tests for the safe-by-construction [`solmath::checked`] layer.
//!
//! These establish, reproducibly and in CI, the property the layer exists to
//! provide: **once inputs are validated into the typed bundles, no pricing
//! method panics or silently wraps for any in-domain input.** A returned `Err`
//! is a valid, handled outcome (errors-as-values); an unwinding panic is not,
//! because on-chain it aborts the instruction.
//!
//! The domain sweep is deterministic (a fixed xorshift seed, no external
//! crates). Its iteration count defaults low enough for CI but can be scaled up
//! for soak testing with `SOLMATH_FUZZ_ITERS`, e.g.
//!
//! ```text
//! SOLMATH_FUZZ_ITERS=20000000 cargo test --features full --test checked_layer -- --nocapture
//! ```

#![cfg(feature = "bs")]

use solmath::{EuropeanInputs, Price, Rate, SolMathError, Time, Vol, SCALE};

fn fuzz_iters(default: u64) -> u64 {
    std::env::var("SOLMATH_FUZZ_ITERS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

// xorshift64 — deterministic, dependency-free.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    /// A value in `[0, max]`, biased to include 0, max, and their neighbours.
    fn in_domain(&mut self, max: u128) -> u128 {
        match self.next() % 8 {
            0 => 0,
            1 => max,
            2 => max.saturating_sub(1),
            3 => 1,
            4 => max / 2,
            _ => {
                let raw = (self.next() as u128) << 64 | self.next() as u128;
                match max.checked_add(1) {
                    Some(m) => raw % m,
                    None => raw, // max == u128::MAX: full range, no modulo
                }
            }
        }
    }
}

#[test]
fn constructors_enforce_domain_boundaries() {
    // Accept exactly at the maximum.
    assert!(Price::new(Price::MAX).is_ok());
    assert!(Rate::new(Rate::MAX).is_ok());
    assert!(Vol::new(Vol::MAX).is_ok());
    assert!(Time::new(Time::MAX).is_ok());

    // Reject one past the maximum.
    assert!(matches!(
        Price::new(Price::MAX + 1),
        Err(SolMathError::DomainError)
    ));
    assert!(matches!(
        Rate::new(Rate::MAX + 1),
        Err(SolMathError::DomainError)
    ));
    assert!(matches!(
        Vol::new(Vol::MAX + 1),
        Err(SolMathError::DomainError)
    ));
    assert!(matches!(
        Time::new(Time::MAX + 1),
        Err(SolMathError::DomainError)
    ));

    // Zero: allowed for Price/Rate, rejected for Vol/Time (BS needs sigma,t > 0).
    assert!(Price::new(0).is_ok());
    assert!(Rate::new(0).is_ok());
    assert!(matches!(Vol::new(0), Err(SolMathError::DomainError)));
    assert!(matches!(Time::new(0), Err(SolMathError::DomainError)));

    // Bundle validation rejects if any single field is out of range.
    assert!(EuropeanInputs::from_raw(Price::MAX + 1, 1, 0, 1, 1).is_err());
    assert!(EuropeanInputs::from_raw(1, 1, 0, 0, 1).is_err()); // sigma == 0
    assert!(EuropeanInputs::from_raw(1, 1, 0, 1, 0).is_err()); // t == 0
                                                               // Every field at its ceiling still constructs.
    assert!(
        EuropeanInputs::from_raw(Price::MAX, Price::MAX, Rate::MAX, Vol::MAX, Time::MAX).is_ok()
    );
}

#[test]
fn checked_bundle_matches_the_raw_functions_exactly() {
    // The wrapper must be a pure pass-through: identical results to the raw
    // functions for validated inputs, adding validation but no behaviour change.
    use solmath::{black_scholes_price, bs_delta, bs_full, bs_gamma, bs_rho, bs_theta, bs_vega};

    let cases = [
        (100u128, 105u128, 5u128, 20u128, 1u128),
        (1, 1, 0, 1, 1),
        (50_000, 40_000, 3, 80, 2),
        (100, 100, 0, 200, 1),
    ];
    let scale = 1_000_000_000_000u128;
    for (s, k, r_pct, sig_pct, t_y) in cases {
        let s = s * scale;
        let k = k * scale;
        let r = r_pct * scale / 100;
        let sigma = sig_pct * scale / 100;
        let t = t_y * scale;

        let inp = EuropeanInputs::from_raw(s, k, r, sigma, t).unwrap();
        assert_eq!(
            inp.price().ok(),
            black_scholes_price(s, k, r, sigma, t).ok()
        );
        assert_eq!(
            inp.full().map(|f| (f.call, f.put)).ok(),
            bs_full(s, k, r, sigma, t).map(|f| (f.call, f.put)).ok()
        );
        assert_eq!(inp.delta().ok(), bs_delta(s, k, r, sigma, t).ok());
        assert_eq!(inp.gamma().ok(), bs_gamma(s, k, r, sigma, t).ok());
        assert_eq!(inp.vega().ok(), bs_vega(s, k, r, sigma, t).ok());
        assert_eq!(inp.theta().ok(), bs_theta(s, k, r, sigma, t).ok());
        assert_eq!(inp.rho().ok(), bs_rho(s, k, r, sigma, t).ok());
    }
}

#[test]
fn checked_inputs_never_panic_over_domain() {
    let iters = fuzz_iters(200_000);
    let mut rng = Rng(0x0913_2f7c_a5e1_bb42);
    let mut bs_ok = 0u64;
    let mut bs_err = 0u64;

    for _ in 0..iters {
        let s = rng.in_domain(Price::MAX);
        let k = rng.in_domain(Price::MAX);
        let r = rng.in_domain(Rate::MAX);
        let sigma = 1 + rng.in_domain(Vol::MAX - 1);
        let t = 1 + rng.in_domain(Time::MAX - 1);

        // Every draw is in-domain, so construction must succeed.
        let inp = EuropeanInputs::from_raw(s, k, r, sigma, t)
            .expect("in-domain construction must succeed");

        // No method may panic. Under debug assertions this also exercises the
        // `mul_fast` / `fp_mul_i_fast` overflow preconditions inside the kernels.
        let outcome = std::panic::catch_unwind(|| {
            let full = inp.full();
            let _ = inp.price();
            let _ = inp.delta();
            let _ = inp.gamma();
            let _ = inp.vega();
            let _ = inp.theta();
            let _ = inp.rho();
            full.is_ok()
        });
        match outcome {
            Ok(true) => bs_ok += 1,
            Ok(false) => bs_err += 1,
            Err(_) => {
                panic!("checked EuropeanInputs panicked for s={s} k={k} r={r} sigma={sigma} t={t}")
            }
        }
    }

    // Sanity: the sweep must actually reach the successful pricing path, not
    // just the degenerate Err corners, or it would prove nothing.
    assert!(
        bs_ok > iters / 2,
        "expected a majority of in-domain inputs to price successfully (ok={bs_ok}, err={bs_err})"
    );
}

#[cfg(feature = "iv")]
#[test]
fn checked_implied_vol_never_panics_over_domain() {
    use solmath::ImpliedVolInputs;

    let iters = fuzz_iters(200_000);
    let mut rng = Rng(0x51ab_c0de_1234_9e37);
    for _ in 0..iters {
        let mp = rng.in_domain(Price::MAX);
        let s = rng.in_domain(Price::MAX);
        let k = rng.in_domain(Price::MAX);
        let r = rng.in_domain(Rate::MAX);
        let t = 1 + rng.in_domain(Time::MAX - 1);

        let inp = ImpliedVolInputs::from_raw(mp, s, k, r, t)
            .expect("in-domain construction must succeed");
        let outcome = std::panic::catch_unwind(|| inp.solve());
        assert!(
            outcome.is_ok(),
            "checked ImpliedVolInputs panicked for mp={mp} s={s} k={k} r={r} t={t}"
        );
    }
}

#[cfg(feature = "barrier")]
#[test]
fn checked_barrier_never_panics_over_domain() {
    use solmath::barrier::BarrierType;
    use solmath::BarrierInputs;

    const TYPES: [BarrierType; 4] = [
        BarrierType::DownAndOut,
        BarrierType::DownAndIn,
        BarrierType::UpAndOut,
        BarrierType::UpAndIn,
    ];

    let iters = fuzz_iters(100_000);
    let mut rng = Rng(0x7a1e_9d3c_44b0_1122);
    for _ in 0..iters {
        let s = rng.in_domain(Price::MAX);
        let k = rng.in_domain(Price::MAX);
        let h = rng.in_domain(Price::MAX);
        let r = rng.in_domain(Rate::MAX);
        let sigma = 1 + rng.in_domain(Vol::MAX - 1);
        let t = 1 + rng.in_domain(Time::MAX - 1);

        let inp = BarrierInputs::from_raw(s, k, h, r, sigma, t)
            .expect("in-domain construction must succeed");
        let ty = TYPES[(rng.next() % 4) as usize];
        let is_call = rng.next() & 1 == 0;
        let breached = rng.next() & 1 == 0;
        let outcome = std::panic::catch_unwind(|| {
            let _ = inp.price(is_call, ty);
            let _ = inp.price_with_state(is_call, ty, breached);
        });
        assert!(
            outcome.is_ok(),
            "checked BarrierInputs panicked for s={s} k={k} h={h} r={r} sigma={sigma} t={t}"
        );
    }
}

#[cfg(feature = "asian")]
#[test]
fn checked_twap_validates_state_and_never_panics() {
    use solmath::{twap_option_price, TwapInputs};

    let raw = [
        100 * SCALE,
        100 * SCALE,
        50_000_000_000,
        20_000_000_000,
        600_000_000_000,
        18 * SCALE / (365 * 24 * 60),
        18 * SCALE / (365 * 24 * 60),
        99_500_000_000_000,
        400_000_000_000,
    ];
    let checked = TwapInputs::from_raw(
        raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6], raw[7], raw[8],
    )
    .unwrap();
    assert_eq!(
        checked.price(),
        twap_option_price(raw[0], raw[1], raw[2], raw[3], raw[4], raw[5], raw[6], raw[7], raw[8])
    );

    assert!(TwapInputs::from_raw(100, 100, 0, 0, 1, SCALE, SCALE + 1, 0, 0).is_err());
    assert!(TwapInputs::from_raw(100, 100, 0, 0, 1, SCALE, SCALE, 100, 0).is_err());
    assert!(TwapInputs::from_raw(100, 100, 0, 0, 1, SCALE, 0, 100, SCALE / 2).is_err());

    let iters = fuzz_iters(50_000);
    let mut rng = Rng(0xa51a_7a2a_5eed_2026);
    for _ in 0..iters {
        let s = (1 + rng.in_domain(1_000)) * SCALE;
        let k = (1 + rng.in_domain(1_000)) * SCALE;
        let r = rng.in_domain(SCALE / 2);
        let q = rng.in_domain(SCALE / 2);
        let sigma = 1 + rng.in_domain(3 * SCALE - 1);
        let t = 1 + rng.in_domain(2 * SCALE - 1);
        let averaging_time = if t == 1 {
            1
        } else {
            1 + rng.in_domain(t - 1).min(t - 1)
        };
        let fixed_weight = rng.in_domain(SCALE - 1);
        let fixed_average = if fixed_weight == 0 {
            0
        } else {
            (1 + rng.in_domain(1_000)) * SCALE
        };

        let inputs = TwapInputs::from_raw(
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
        .unwrap_or_else(|error| {
            panic!(
                "generated TWAP state rejected ({error:?}) for s={s} k={k} r={r} q={q} sigma={sigma} t={t} averaging_time={averaging_time} fixed_average={fixed_average} fixed_weight={fixed_weight}"
            )
        });
        let outcome = std::panic::catch_unwind(|| inputs.price());
        assert!(
            outcome.is_ok(),
            "checked TwapInputs panicked for s={s} k={k} r={r} q={q} sigma={sigma} t={t} averaging_time={averaging_time} fixed_average={fixed_average} fixed_weight={fixed_weight}"
        );
    }
}

#[cfg(feature = "pool")]
#[test]
fn checked_pool_swap_validates_domain_and_never_panics() {
    use solmath::{weighted_pool_swap, PoolSwapInputs};

    // A validated bundle must quote identically to the raw kernel.
    let inp = PoolSwapInputs::from_raw(
        1_000_000 * SCALE, // balance_in
        2_000_000 * SCALE, // balance_out
        2,                 // weight_in
        1,                 // weight_out (ratio 2 <= 20)
        10_000 * SCALE,    // amount_in
        3_000_000_000,     // 0.3% fee
    )
    .expect("balanced swap is in the certified domain");
    assert_eq!(
        inp.quote().ok(),
        weighted_pool_swap(
            1_000_000 * SCALE,
            2_000_000 * SCALE,
            2,
            1,
            10_000 * SCALE,
            3_000_000_000
        )
        .ok()
    );

    // Domain rejections mirror the kernel's guards.
    assert!(PoolSwapInputs::from_raw(0, 1, 1, 1, 1, 0).is_err()); // zero balance
    assert!(PoolSwapInputs::from_raw(1, 1, 0, 1, 1, 0).is_err()); // zero weight
    assert!(PoolSwapInputs::from_raw(1, 1, 21, 1, 1, 0).is_err()); // weight ratio > 20
    assert!(PoolSwapInputs::from_raw(1, 1, 1, 1, 1, SCALE + 1).is_err()); // fee > 100%
    assert!(PoolSwapInputs::from_raw(1, 1, 1, 1, 1_000_000, 0).is_err()); // balance ratio < 0.01

    // No validated swap panics on quote, across a wide value sweep.
    let iters = fuzz_iters(200_000);
    let mut rng = Rng(0x900d_51ab_c0de_f00d);
    let mut validated = 0u64;
    for _ in 0..iters {
        let bin = rng.in_domain(u128::MAX);
        let bout = rng.in_domain(u128::MAX);
        let win = 1 + rng.in_domain(1_000);
        let wout = 1 + rng.in_domain(1_000);
        let ain = rng.in_domain(u128::MAX);
        let fee = rng.in_domain(SCALE);
        if let Ok(swap) = PoolSwapInputs::from_raw(bin, bout, win, wout, ain, fee) {
            validated += 1;
            let outcome = std::panic::catch_unwind(|| swap.quote());
            assert!(
                outcome.is_ok(),
                "checked PoolSwapInputs panicked for bin={bin} bout={bout} win={win} wout={wout} ain={ain} fee={fee}"
            );
        }
    }
    assert!(
        validated > 0,
        "the pool sweep must exercise at least some in-domain swaps"
    );
}
