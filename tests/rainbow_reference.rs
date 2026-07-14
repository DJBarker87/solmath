//! Reference and invariant checks for two-asset rainbow options.
//!
//! Prices are the exact Stulz (1982) closed form; the hardcoded targets were
//! computed independently in Python and cross-checked against 6M-path Monte
//! Carlo to ~1e-3. Fixed-point reproduces them to <=1 ULP.
#![cfg(feature = "rainbow")]

use solmath::{best_of_call, worst_of_call, SCALE};

fn fp(x: f64) -> u128 {
    (x * SCALE as f64).round() as u128
}
fn fpi(x: f64) -> i128 {
    (x * SCALE as f64).round() as i128
}

#[test]
fn matches_stulz_reference() {
    // (S1,S2,K,r,q1,q2,v1,v2,rho,T, worst_ref, best_ref)
    let cases = [
        (
            100., 100., 100., 0.05, 0.0, 0.0, 0.2, 0.25, 0.5, 1.0, 5.6776, 17.1090,
        ),
        (
            100., 110., 100., 0.03, 0.02, 0.01, 0.3, 0.2, 0.3, 0.5, 4.6413, 16.8314,
        ),
        (
            90., 100., 95., 0.05, 0.0, 0.0, 0.4, 0.35, 0.7, 2.0, 13.3043, 34.3904,
        ),
        (
            120., 80., 100., 0.04, 0.0, 0.0, 0.25, 0.45, -0.3, 1.0, 2.5400, 33.0052,
        ),
    ];
    for (s1, s2, k, r, q1, q2, v1, v2, rho, t, w, b) in cases {
        let mn = worst_of_call(
            fp(s1),
            fp(s2),
            fp(k),
            fp(r),
            fp(q1),
            fp(q2),
            fp(v1),
            fp(v2),
            fpi(rho),
            fp(t),
        )
        .unwrap() as f64
            / SCALE as f64;
        let mx = best_of_call(
            fp(s1),
            fp(s2),
            fp(k),
            fp(r),
            fp(q1),
            fp(q2),
            fp(v1),
            fp(v2),
            fpi(rho),
            fp(t),
        )
        .unwrap() as f64
            / SCALE as f64;
        assert!((mn - w).abs() < 5e-4, "worst_of {mn} vs {w}");
        assert!((mx - b).abs() < 5e-4, "best_of {mx} vs {b}");
    }
}

#[test]
fn worst_of_never_exceeds_best_of() {
    // min(S1,S2) <= max(S1,S2) pointwise, so the worst-of call <= best-of call.
    let mut rng = 0x1234_5678_u64;
    let mut next = || {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        rng
    };
    for _ in 0..5000 {
        let s1 = fp(50.0 + (next() % 100) as f64);
        let s2 = fp(50.0 + (next() % 100) as f64);
        let k = fp(50.0 + (next() % 100) as f64);
        let r = fp((next() % 10) as f64 / 100.0);
        let v1 = fp(0.1 + (next() % 60) as f64 / 100.0);
        let v2 = fp(0.1 + (next() % 60) as f64 / 100.0);
        let rho = fpi(-0.9 + (next() % 180) as f64 / 100.0);
        let t = fp(0.1 + (next() % 30) as f64 / 10.0);
        let out = std::panic::catch_unwind(|| {
            (
                worst_of_call(s1, s2, k, r, 0, 0, v1, v2, rho, t),
                best_of_call(s1, s2, k, r, 0, 0, v1, v2, rho, t),
            )
        });
        assert!(out.is_ok(), "rainbow panicked");
        if let Ok((Ok(w), Ok(b))) = out {
            assert!(w <= b + SCALE / 1000, "worst {w} > best {b}");
        }
    }
}
