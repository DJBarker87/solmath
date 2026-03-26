// Auto-generated from QuantLib. Do not edit.
// 500 of 100000 vectors (every 200th)

#[cfg(test)]
mod quantlib_sabr {
    use crate::sabr::sabr_implied_vol;

    #[test]
    fn ql_sabr_0000() {
        // F=100.0, K=50.000000, T=0.1, vol=0.0069802948
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 100000000000u128,
            20000000000u128, 0u128, -900000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 6980294771u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#0: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0001() {
        // F=100.0, K=80.000000, T=2.0, vol=0.0083094374
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 2000000000000u128,
            20000000000u128, 0u128, -900000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 8309437356u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#1: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0002() {
        // F=100.0, K=95.000000, T=0.25, vol=0.0060623859
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 250000000000u128,
            20000000000u128, 0u128, -900000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 6062385919u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#2: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0003() {
        // F=100.0, K=105.000000, T=5.0, vol=0.0005322089
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 5000000000000u128,
            20000000000u128, 0u128, -760000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 532208890u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#3: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0004() {
        // F=100.0, K=120.000000, T=0.5, vol=0.0069881448
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 500000000000u128,
            20000000000u128, 0u128, -760000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 6988144817u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#4: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0005() {
        // F=100.0, K=200.000000, T=10.0, vol=0.0419552343
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 10000000000000u128,
            20000000000u128, 0u128, -760000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 41955234299u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#5: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0006() {
        // F=100.0, K=70.000000, T=2.0, vol=0.0068012848
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 2000000000000u128,
            20000000000u128, 0u128, -620000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 6801284832u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#6: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0007() {
        // F=100.0, K=90.000000, T=0.25, vol=0.0076536009
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 250000000000u128,
            20000000000u128, 0u128, -620000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 7653600943u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#7: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0008() {
        // F=100.0, K=100.000000, T=5.0, vol=0.0002225813
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 5000000000000u128,
            20000000000u128, 0u128, -620000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 222581335u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#8: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0009() {
        // F=100.0, K=110.000000, T=0.5, vol=0.0032075628
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 500000000000u128,
            20000000000u128, 0u128, -480000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 3207562849u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#9: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0010() {
        // F=100.0, K=150.000000, T=10.0, vol=0.0212349265
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 10000000000000u128,
            20000000000u128, 0u128, -480000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 21234926520u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#10: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0011() {
        // F=100.0, K=50.000000, T=2.0, vol=0.0065209583
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 2000000000000u128,
            20000000000u128, 0u128, -340000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 6520958275u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#11: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0012() {
        // F=100.0, K=80.000000, T=0.25, vol=0.0109625380
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 250000000000u128,
            20000000000u128, 0u128, -340000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 10962537992u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#12: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0013() {
        // F=100.0, K=95.000000, T=5.0, vol=0.0063894931
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 5000000000000u128,
            20000000000u128, 0u128, -340000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 6389493097u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#13: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0014() {
        // F=100.0, K=105.000000, T=0.5, vol=0.0011824428
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 500000000000u128,
            20000000000u128, 0u128, -200000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 1182442784u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#14: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0015() {
        // F=100.0, K=120.000000, T=10.0, vol=0.0088351983
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 10000000000000u128,
            20000000000u128, 0u128, -200000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 8835198304u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#15: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0016() {
        // F=100.0, K=200.000000, T=1.0, vol=0.0621109890
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 1000000000000u128,
            20000000000u128, 0u128, -200000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 62110988983u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#16: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0017() {
        // F=100.0, K=70.000000, T=0.25, vol=0.0112128041
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 250000000000u128,
            20000000000u128, 0u128, -60000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 11212804148u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#17: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0018() {
        // F=100.0, K=90.000000, T=5.0, vol=0.0075708024
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 5000000000000u128,
            20000000000u128, 0u128, -60000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 7570802411u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#18: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0019() {
        // F=100.0, K=100.000000, T=0.5, vol=0.0002000206
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 500000000000u128,
            20000000000u128, 0u128, 80000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 200020634u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#19: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0020() {
        // F=100.0, K=110.000000, T=10.0, vol=0.0037695577
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 10000000000000u128,
            20000000000u128, 0u128, 80000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 3769557695u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#20: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0021() {
        // F=100.0, K=150.000000, T=1.0, vol=0.0314037004
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 1000000000000u128,
            20000000000u128, 0u128, 80000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 31403700427u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#21: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0022() {
        // F=100.0, K=50.000000, T=0.25, vol=0.0105490497
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 250000000000u128,
            20000000000u128, 0u128, 220000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 10549049732u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#22: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0023() {
        // F=100.0, K=80.000000, T=5.0, vol=0.0104067426
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 5000000000000u128,
            20000000000u128, 0u128, 220000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 10406742640u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#23: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0024() {
        // F=100.0, K=95.000000, T=0.5, vol=0.0067393359
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 500000000000u128,
            20000000000u128, 0u128, 220000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 6739335934u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#24: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0025() {
        // F=100.0, K=105.000000, T=10.0, vol=0.0013570546
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 10000000000000u128,
            20000000000u128, 0u128, 360000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 1357054593u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#25: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0026() {
        // F=100.0, K=120.000000, T=1.0, vol=0.0115436852
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 1000000000000u128,
            20000000000u128, 0u128, 360000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 11543685240u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#26: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0027() {
        // F=100.0, K=200.000000, T=0.1, vol=0.0058663746
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 100000000000u128,
            20000000000u128, 0u128, 500000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 5866374575u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#27: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0028() {
        // F=100.0, K=70.000000, T=5.0, vol=0.0101223110
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 5000000000000u128,
            20000000000u128, 0u128, 500000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 10122310973u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#28: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0029() {
        // F=100.0, K=90.000000, T=0.5, vol=0.0089982970
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 500000000000u128,
            20000000000u128, 0u128, 500000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 8998297049u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#29: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0030() {
        // F=100.0, K=100.000000, T=10.0, vol=0.0006321611
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 10000000000000u128,
            20000000000u128, 250000000000u128, -900000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 632161054u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#30: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0031() {
        // F=100.0, K=110.000000, T=1.0, vol=0.0041827439
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 1000000000000u128,
            20000000000u128, 250000000000u128, -900000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 4182743867u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#31: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0032() {
        // F=100.0, K=150.000000, T=0.1, vol=0.0343856404
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 100000000000u128,
            20000000000u128, 250000000000u128, -900000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 34385640444u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#32: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0033() {
        // F=100.0, K=50.000000, T=5.0, vol=0.0149938850
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 5000000000000u128,
            20000000000u128, 250000000000u128, -760000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 14993884998u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#33: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0034() {
        // F=100.0, K=80.000000, T=0.5, vol=0.0178476151
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 500000000000u128,
            20000000000u128, 250000000000u128, -760000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 17847615126u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#34: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0035() {
        // F=100.0, K=95.000000, T=10.0, vol=0.0102364678
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 10000000000000u128,
            20000000000u128, 250000000000u128, -760000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 10236467781u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#35: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0036() {
        // F=100.0, K=105.000000, T=1.0, vol=0.0022330714
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 1000000000000u128,
            20000000000u128, 250000000000u128, -620000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 2233071395u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#36: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0037() {
        // F=100.0, K=120.000000, T=0.1, vol=0.0159105854
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 100000000000u128,
            20000000000u128, 250000000000u128, -620000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 15910585414u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#37: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0038() {
        // F=100.0, K=200.000000, T=2.0, vol=0.0061170473
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 2000000000000u128,
            20000000000u128, 250000000000u128, -480000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 6117047264u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#38: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0039() {
        // F=100.0, K=70.000000, T=0.5, vol=0.0201723330
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 500000000000u128,
            20000000000u128, 250000000000u128, -480000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 20172333047u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#39: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0040() {
        // F=100.0, K=90.000000, T=10.0, vol=0.0155194060
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 10000000000000u128,
            20000000000u128, 250000000000u128, -480000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 15519406034u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#40: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0041() {
        // F=100.0, K=100.000000, T=1.0, vol=0.0006328903
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 1000000000000u128,
            20000000000u128, 250000000000u128, -340000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 632890344u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#41: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0042() {
        // F=100.0, K=110.000000, T=0.1, vol=0.0072848377
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 100000000000u128,
            20000000000u128, 250000000000u128, -340000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 7284837732u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#42: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0043() {
        // F=100.0, K=150.000000, T=2.0, vol=0.0468801865
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 2000000000000u128,
            20000000000u128, 250000000000u128, -340000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 46880186505u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#43: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0044() {
        // F=100.0, K=50.000000, T=0.5, vol=0.0243343403
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 500000000000u128,
            20000000000u128, 250000000000u128, -200000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 24334340327u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#44: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0045() {
        // F=100.0, K=80.000000, T=10.0, vol=0.0186532647
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 10000000000000u128,
            20000000000u128, 250000000000u128, -200000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 18653264675u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#45: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0046() {
        // F=100.0, K=95.000000, T=1.0, vol=0.0012539740
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 1000000000000u128,
            20000000000u128, 250000000000u128, -60000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 1253974009u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#46: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0047() {
        // F=100.0, K=105.000000, T=0.1, vol=0.0037432769
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 100000000000u128,
            20000000000u128, 250000000000u128, -60000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 3743276878u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#47: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0048() {
        // F=100.0, K=120.000000, T=2.0, vol=0.0193820711
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 2000000000000u128,
            20000000000u128, 250000000000u128, -60000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 19382071055u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#48: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0049() {
        // F=100.0, K=200.000000, T=0.25, vol=0.0123000853
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 250000000000u128,
            20000000000u128, 250000000000u128, 80000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 12300085285u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#49: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0050() {
        // F=100.0, K=70.000000, T=10.0, vol=0.0198518846
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 10000000000000u128,
            20000000000u128, 250000000000u128, 80000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 19851884631u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#50: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0051() {
        // F=100.0, K=90.000000, T=1.0, vol=0.0157614048
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 1000000000000u128,
            20000000000u128, 250000000000u128, 80000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 15761404848u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#51: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0052() {
        // F=100.0, K=100.000000, T=0.1, vol=0.0006326512
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 100000000000u128,
            20000000000u128, 250000000000u128, 220000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 632651156u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#52: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0053() {
        // F=100.0, K=110.000000, T=2.0, vol=0.0084312925
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 2000000000000u128,
            20000000000u128, 250000000000u128, 220000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 8431292502u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#53: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0054() {
        // F=100.0, K=150.000000, T=0.25, vol=0.0050302952
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 250000000000u128,
            20000000000u128, 250000000000u128, 360000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 5030295214u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#54: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0055() {
        // F=100.0, K=50.000000, T=10.0, vol=0.0224604059
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 10000000000000u128,
            20000000000u128, 250000000000u128, 360000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 22460405936u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#55: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0056() {
        // F=100.0, K=80.000000, T=1.0, vol=0.0213703022
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 1000000000000u128,
            20000000000u128, 250000000000u128, 360000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 21370302197u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#56: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0057() {
        // F=100.0, K=95.000000, T=0.1, vol=0.0015092222
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 100000000000u128,
            20000000000u128, 250000000000u128, 500000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 1509222153u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#57: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0058() {
        // F=100.0, K=105.000000, T=2.0, vol=0.0042587612
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 2000000000000u128,
            20000000000u128, 250000000000u128, 500000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 4258761228u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#58: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0059() {
        // F=100.0, K=120.000000, T=0.25, vol=0.0253427291
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 250000000000u128,
            20000000000u128, 250000000000u128, 500000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 25342729132u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#59: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0060() {
        // F=100.0, K=200.000000, T=5.0, vol=0.0102952469
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 5000000000000u128,
            20000000000u128, 500000000000u128, -900000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 10295246937u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#60: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0061() {
        // F=100.0, K=70.000000, T=1.0, vol=0.0334789590
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 1000000000000u128,
            20000000000u128, 500000000000u128, -900000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 33478958998u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#61: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0062() {
        // F=100.0, K=90.000000, T=0.1, vol=0.0039342044
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 100000000000u128,
            20000000000u128, 500000000000u128, -760000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 3934204440u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#62: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0063() {
        // F=100.0, K=100.000000, T=2.0, vol=0.0020016295
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 2000000000000u128,
            20000000000u128, 500000000000u128, -760000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 2001629500u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#63: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0064() {
        // F=100.0, K=110.000000, T=0.25, vol=0.0104616125
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 250000000000u128,
            20000000000u128, 500000000000u128, -760000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 10461612468u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#64: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0065() {
        // F=100.0, K=150.000000, T=5.0, vol=0.0050330542
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 5000000000000u128,
            20000000000u128, 500000000000u128, -620000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 5033054176u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#65: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0066() {
        // F=100.0, K=50.000000, T=1.0, vol=0.0442671150
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 1000000000000u128,
            20000000000u128, 500000000000u128, -620000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 44267114959u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#66: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0067() {
        // F=100.0, K=80.000000, T=0.1, vol=0.0384251342
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 100000000000u128,
            20000000000u128, 500000000000u128, -620000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 38425134216u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#67: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0068() {
        // F=100.0, K=95.000000, T=2.0, vol=0.0036051199
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 2000000000000u128,
            20000000000u128, 500000000000u128, -480000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 3605119850u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#68: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0069() {
        // F=100.0, K=105.000000, T=0.25, vol=0.0054483150
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 250000000000u128,
            20000000000u128, 500000000000u128, -480000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 5448314980u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#69: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0070() {
        // F=100.0, K=120.000000, T=5.0, vol=0.0301600380
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 5000000000000u128,
            20000000000u128, 500000000000u128, -480000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 30160037969u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#70: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0071() {
        // F=100.0, K=200.000000, T=0.5, vol=0.0250388745
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 500000000000u128,
            20000000000u128, 500000000000u128, -340000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 25038874504u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#71: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0072() {
        // F=100.0, K=70.000000, T=0.1, vol=0.0429561741
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 100000000000u128,
            20000000000u128, 500000000000u128, -340000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 42956174055u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#72: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0073() {
        // F=100.0, K=90.000000, T=2.0, vol=0.0033820625
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 2000000000000u128,
            20000000000u128, 500000000000u128, -200000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 3382062539u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#73: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0074() {
        // F=100.0, K=100.000000, T=0.25, vol=0.0020035175
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 250000000000u128,
            20000000000u128, 500000000000u128, -200000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 2003517521u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#74: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0075() {
        // F=100.0, K=110.000000, T=5.0, vol=0.0152179122
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 5000000000000u128,
            20000000000u128, 500000000000u128, -200000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 15217912243u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#75: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0076() {
        // F=100.0, K=150.000000, T=0.5, vol=0.0104811163
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 500000000000u128,
            20000000000u128, 500000000000u128, -60000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 10481116267u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#76: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0077() {
        // F=100.0, K=50.000000, T=0.1, vol=0.0512134625
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 100000000000u128,
            20000000000u128, 500000000000u128, -60000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 51213462453u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#77: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0078() {
        // F=100.0, K=80.000000, T=2.0, vol=0.0389139439
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 2000000000000u128,
            20000000000u128, 500000000000u128, -60000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 38913943850u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#78: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0079() {
        // F=100.0, K=95.000000, T=0.25, vol=0.0042910697
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 250000000000u128,
            20000000000u128, 500000000000u128, 80000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 4291069687u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#79: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0080() {
        // F=100.0, K=105.000000, T=5.0, vol=0.0071322059
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 5000000000000u128,
            20000000000u128, 500000000000u128, 80000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 7132205883u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#80: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0081() {
        // F=100.0, K=120.000000, T=0.5, vol=0.0043167430
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 500000000000u128,
            20000000000u128, 500000000000u128, 220000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 4316743008u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#81: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0082() {
        // F=100.0, K=200.000000, T=10.0, vol=0.0289706059
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 10000000000000u128,
            20000000000u128, 500000000000u128, 220000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 28970605874u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#82: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0083() {
        // F=100.0, K=70.000000, T=2.0, vol=0.0408521977
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 2000000000000u128,
            20000000000u128, 500000000000u128, 220000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 40852197728u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#83: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0084() {
        // F=100.0, K=90.000000, T=0.25, vol=0.0038860013
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 250000000000u128,
            20000000000u128, 500000000000u128, 360000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 3886001298u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#84: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0085() {
        // F=100.0, K=100.000000, T=5.0, vol=0.0020606904
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 5000000000000u128,
            20000000000u128, 500000000000u128, 360000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 2060690417u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#85: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0086() {
        // F=100.0, K=110.000000, T=0.5, vol=0.0191842614
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 500000000000u128,
            20000000000u128, 500000000000u128, 360000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 19184261414u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#86: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0087() {
        // F=100.0, K=150.000000, T=10.0, vol=0.0118958080
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 10000000000000u128,
            20000000000u128, 500000000000u128, 500000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 11895807994u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#87: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0088() {
        // F=100.0, K=50.000000, T=2.0, vol=0.0456812367
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 2000000000000u128,
            20000000000u128, 500000000000u128, 500000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 45681236657u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#88: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0089() {
        // F=100.0, K=80.000000, T=0.25, vol=0.0109404671
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 250000000000u128,
            20000000000u128, 750000000000u128, -900000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 10940467062u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#89: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0090() {
        // F=100.0, K=95.000000, T=5.0, vol=0.0104225283
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 5000000000000u128,
            20000000000u128, 750000000000u128, -900000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 10422528347u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#90: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0091() {
        // F=100.0, K=105.000000, T=0.5, vol=0.0067481579
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 500000000000u128,
            20000000000u128, 750000000000u128, -900000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 6748157910u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#91: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0092() {
        // F=100.0, K=120.000000, T=10.0, vol=0.0046827815
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 10000000000000u128,
            20000000000u128, 750000000000u128, -760000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 4682781546u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#92: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0093() {
        // F=100.0, K=200.000000, T=1.0, vol=0.0366020196
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 1000000000000u128,
            20000000000u128, 750000000000u128, -760000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 36602019606u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#93: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0094() {
        // F=100.0, K=70.000000, T=0.25, vol=0.0730689037
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 250000000000u128,
            20000000000u128, 750000000000u128, -760000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 73068903742u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#94: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0095() {
        // F=100.0, K=90.000000, T=5.0, vol=0.0099847538
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 5000000000000u128,
            20000000000u128, 750000000000u128, -620000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 9984753836u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#95: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0096() {
        // F=100.0, K=100.000000, T=0.5, vol=0.0063414778
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 500000000000u128,
            20000000000u128, 750000000000u128, -620000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 6341477761u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#96: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0097() {
        // F=100.0, K=110.000000, T=10.0, vol=0.0226262274
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 10000000000000u128,
            20000000000u128, 750000000000u128, -620000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 22626227415u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#97: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0098() {
        // F=100.0, K=150.000000, T=1.0, vol=0.0207537588
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 1000000000000u128,
            20000000000u128, 750000000000u128, -480000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 20753758777u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#98: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0099() {
        // F=100.0, K=50.000000, T=0.25, vol=0.0946752875
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 250000000000u128,
            20000000000u128, 750000000000u128, -480000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 94675287526u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#99: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0100() {
        // F=100.0, K=80.000000, T=5.0, vol=0.0095446935
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 5000000000000u128,
            20000000000u128, 750000000000u128, -340000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 9544693459u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#100: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0101() {
        // F=100.0, K=95.000000, T=0.5, vol=0.0106985254
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 500000000000u128,
            20000000000u128, 750000000000u128, -340000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 10698525405u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#101: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0102() {
        // F=100.0, K=105.000000, T=10.0, vol=0.0141171963
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 10000000000000u128,
            20000000000u128, 750000000000u128, -340000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 14117196344u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#102: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0103() {
        // F=100.0, K=120.000000, T=1.0, vol=0.0093181601
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 1000000000000u128,
            20000000000u128, 750000000000u128, -200000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 9318160057u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#103: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0104() {
        // F=100.0, K=200.000000, T=0.1, vol=0.0580113146
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 100000000000u128,
            20000000000u128, 750000000000u128, -200000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 58011314608u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#104: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0105() {
        // F=100.0, K=70.000000, T=5.0, vol=0.0832814540
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 5000000000000u128,
            20000000000u128, 750000000000u128, -200000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 83281453997u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#105: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0106() {
        // F=100.0, K=90.000000, T=0.5, vol=0.0113181565
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 500000000000u128,
            20000000000u128, 750000000000u128, -60000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 11318156526u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#106: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0107() {
        // F=100.0, K=100.000000, T=10.0, vol=0.0071614823
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 10000000000000u128,
            20000000000u128, 750000000000u128, -60000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 7161482271u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#107: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0108() {
        // F=100.0, K=110.000000, T=1.0, vol=0.0069336593
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 1000000000000u128,
            20000000000u128, 750000000000u128, 80000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 6933659311u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#108: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0109() {
        // F=100.0, K=150.000000, T=0.1, vol=0.0335382580
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 100000000000u128,
            20000000000u128, 750000000000u128, 80000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 33538258045u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#109: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0110() {
        // F=100.0, K=50.000000, T=5.0, vol=0.0978961131
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 5000000000000u128,
            20000000000u128, 750000000000u128, 80000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 97896113056u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#110: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0111() {
        // F=100.0, K=80.000000, T=0.5, vol=0.0104763419
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 500000000000u128,
            20000000000u128, 750000000000u128, 220000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 10476341924u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#111: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0112() {
        // F=100.0, K=95.000000, T=10.0, vol=0.0092795347
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 10000000000000u128,
            20000000000u128, 750000000000u128, 220000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 9279534682u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#112: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0113() {
        // F=100.0, K=105.000000, T=1.0, vol=0.0173540498
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 1000000000000u128,
            20000000000u128, 750000000000u128, 220000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 17354049849u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#113: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0114() {
        // F=100.0, K=120.000000, T=0.1, vol=0.0163908363
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 100000000000u128,
            20000000000u128, 750000000000u128, 360000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 16390836321u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#114: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0115() {
        // F=100.0, K=200.000000, T=2.0, vol=0.0664132354
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 2000000000000u128,
            20000000000u128, 750000000000u128, 360000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 66413235403u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#115: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0116() {
        // F=100.0, K=70.000000, T=0.5, vol=0.0080673096
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 500000000000u128,
            20000000000u128, 750000000000u128, 500000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 8067309649u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#116: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0117() {
        // F=100.0, K=90.000000, T=10.0, vol=0.0088443516
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 10000000000000u128,
            20000000000u128, 750000000000u128, 500000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 8844351647u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#117: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0118() {
        // F=100.0, K=100.000000, T=1.0, vol=0.0064453914
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 1000000000000u128,
            20000000000u128, 750000000000u128, 500000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 6445391391u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#118: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0119() {
        // F=100.0, K=110.000000, T=0.1, vol=0.0155936708
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 100000000000u128,
            20000000000u128, 1000000000000u128, -900000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 15593670836u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#119: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0120() {
        // F=100.0, K=150.000000, T=2.0, vol=0.0260450297
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 2000000000000u128,
            20000000000u128, 1000000000000u128, -900000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 26045029666u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#120: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0121() {
        // F=100.0, K=50.000000, T=0.5, vol=0.1615956787
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 500000000000u128,
            20000000000u128, 1000000000000u128, -900000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 161595678734u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#121: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0122() {
        // F=100.0, K=80.000000, T=10.0, vol=0.0283289382
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 10000000000000u128,
            20000000000u128, 1000000000000u128, -760000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 28328938175u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#122: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0123() {
        // F=100.0, K=95.000000, T=1.0, vol=0.0277590757
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 1000000000000u128,
            20000000000u128, 1000000000000u128, -760000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 27759075661u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#123: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0124() {
        // F=100.0, K=105.000000, T=0.1, vol=0.0192673001
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 100000000000u128,
            20000000000u128, 1000000000000u128, -620000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 19267300137u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#124: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0125() {
        // F=100.0, K=120.000000, T=2.0, vol=0.0188429937
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 2000000000000u128,
            20000000000u128, 1000000000000u128, -620000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 18842993678u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#125: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0126() {
        // F=100.0, K=200.000000, T=0.25, vol=0.0893823591
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 250000000000u128,
            20000000000u128, 1000000000000u128, -620000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 89382359132u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#126: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0127() {
        // F=100.0, K=70.000000, T=10.0, vol=0.0252450115
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 10000000000000u128,
            20000000000u128, 1000000000000u128, -480000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 25245011482u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#127: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0128() {
        // F=100.0, K=90.000000, T=1.0, vol=0.0297606561
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 1000000000000u128,
            20000000000u128, 1000000000000u128, -480000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 29760656096u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#128: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0129() {
        // F=100.0, K=100.000000, T=0.1, vol=0.0200659627
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 100000000000u128,
            20000000000u128, 1000000000000u128, -480000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 20065962667u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#129: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0130() {
        // F=100.0, K=110.000000, T=2.0, vol=0.0191281718
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 2000000000000u128,
            20000000000u128, 1000000000000u128, -340000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 19128171797u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#130: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0131() {
        // F=100.0, K=150.000000, T=0.25, vol=0.0514169400
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 250000000000u128,
            20000000000u128, 1000000000000u128, -340000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 51416940027u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#131: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0132() {
        // F=100.0, K=50.000000, T=10.0, vol=0.2118797805
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 10000000000000u128,
            20000000000u128, 1000000000000u128, -340000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 211879780537u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#132: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0133() {
        // F=100.0, K=80.000000, T=1.0, vol=0.0311342401
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 1000000000000u128,
            20000000000u128, 1000000000000u128, -200000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 31134240063u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#133: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0134() {
        // F=100.0, K=95.000000, T=0.1, vol=0.0271822730
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 100000000000u128,
            20000000000u128, 1000000000000u128, -200000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 27182272963u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#134: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0135() {
        // F=100.0, K=105.000000, T=2.0, vol=0.0199840641
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 2000000000000u128,
            20000000000u128, 1000000000000u128, -60000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 19984064127u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#135: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0136() {
        // F=100.0, K=120.000000, T=0.25, vol=0.0309233733
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 250000000000u128,
            20000000000u128, 1000000000000u128, -60000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 30923373296u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#136: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0137() {
        // F=100.0, K=200.000000, T=5.0, vol=0.1260919998
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 5000000000000u128,
            20000000000u128, 1000000000000u128, -60000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 126091999756u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#137: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0138() {
        // F=100.0, K=70.000000, T=1.0, vol=0.0257662232
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 1000000000000u128,
            20000000000u128, 1000000000000u128, 80000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 25766223189u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#138: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0139() {
        // F=100.0, K=90.000000, T=0.1, vol=0.0274244956
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 100000000000u128,
            20000000000u128, 1000000000000u128, 80000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 27424495640u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#139: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0140() {
        // F=100.0, K=100.000000, T=2.0, vol=0.0221256533
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 2000000000000u128,
            20000000000u128, 1000000000000u128, 80000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 22125653333u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#140: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0141() {
        // F=100.0, K=110.000000, T=0.25, vol=0.0240287495
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 250000000000u128,
            20000000000u128, 1000000000000u128, 220000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 24028749468u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#141: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0142() {
        // F=100.0, K=150.000000, T=5.0, vol=0.0659297399
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 5000000000000u128,
            20000000000u128, 1000000000000u128, 220000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 65929739947u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#142: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0143() {
        // F=100.0, K=50.000000, T=1.0, vol=0.0222826508
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 1000000000000u128,
            20000000000u128, 1000000000000u128, 360000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 22282650833u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#143: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0144() {
        // F=100.0, K=80.000000, T=0.1, vol=0.0296758444
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 100000000000u128,
            20000000000u128, 1000000000000u128, 360000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 29675844356u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#144: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0145() {
        // F=100.0, K=95.000000, T=2.0, vol=0.0225627633
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 2000000000000u128,
            20000000000u128, 1000000000000u128, 360000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 22562763271u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#145: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0146() {
        // F=100.0, K=105.000000, T=0.25, vol=0.0213280917
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 250000000000u128,
            20000000000u128, 1000000000000u128, 500000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 21328091735u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#146: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0147() {
        // F=100.0, K=120.000000, T=5.0, vol=0.0379846452
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 5000000000000u128,
            20000000000u128, 1000000000000u128, 500000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 37984645205u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#147: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0148() {
        // F=100.0, K=200.000000, T=0.5, vol=0.1555374377
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 500000000000u128,
            20000000000u128, 1000000000000u128, 500000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 155537437741u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#148: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0149() {
        // F=100.0, K=70.000000, T=0.1, vol=0.0143635013
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 100000000000u128,
            45263157895u128, 0u128, -900000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 14363501269u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#149: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0150() {
        // F=100.0, K=90.000000, T=2.0, vol=0.0092202367
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 2000000000000u128,
            45263157895u128, 0u128, -900000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 9220236697u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#150: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0151() {
        // F=100.0, K=100.000000, T=0.25, vol=0.0004526347
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 250000000000u128,
            45263157895u128, 0u128, -760000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 452634729u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#151: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0152() {
        // F=100.0, K=110.000000, T=5.0, vol=0.0032417709
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 5000000000000u128,
            45263157895u128, 0u128, -760000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 3241770866u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#152: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0153() {
        // F=100.0, K=150.000000, T=0.5, vol=0.0281231765
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 500000000000u128,
            45263157895u128, 0u128, -760000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 28123176452u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#153: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0154() {
        // F=100.0, K=50.000000, T=0.1, vol=0.0138636833
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 100000000000u128,
            45263157895u128, 0u128, -620000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 13863683310u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#154: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0155() {
        // F=100.0, K=80.000000, T=2.0, vol=0.0131811765
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 2000000000000u128,
            45263157895u128, 0u128, -620000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 13181176478u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#155: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0156() {
        // F=100.0, K=95.000000, T=0.25, vol=0.0087806328
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 250000000000u128,
            45263157895u128, 0u128, -620000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 8780632779u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#156: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0157() {
        // F=100.0, K=105.000000, T=5.0, vol=0.0013198421
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 5000000000000u128,
            45263157895u128, 0u128, -480000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 1319842112u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#157: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0158() {
        // F=100.0, K=120.000000, T=0.5, vol=0.0112226262
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 500000000000u128,
            45263157895u128, 0u128, -480000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 11222626217u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#158: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0159() {
        // F=100.0, K=200.000000, T=10.0, vol=0.0832885082
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 10000000000000u128,
            45263157895u128, 0u128, -480000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 83288508208u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#159: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0160() {
        // F=100.0, K=70.000000, T=2.0, vol=0.0135008710
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 2000000000000u128,
            45263157895u128, 0u128, -340000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 13500871015u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#160: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0161() {
        // F=100.0, K=90.000000, T=0.25, vol=0.0120195603
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 250000000000u128,
            45263157895u128, 0u128, -340000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 12019560321u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#161: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0162() {
        // F=100.0, K=100.000000, T=5.0, vol=0.0004530748
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 5000000000000u128,
            45263157895u128, 0u128, -200000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 453074800u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#162: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0163() {
        // F=100.0, K=110.000000, T=0.5, vol=0.0056166062
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 500000000000u128,
            45263157895u128, 0u128, -200000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 5616606241u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#163: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0164() {
        // F=100.0, K=150.000000, T=10.0, vol=0.0418274416
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 10000000000000u128,
            45263157895u128, 0u128, -200000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 41827441595u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#164: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0165() {
        // F=100.0, K=50.000000, T=2.0, vol=0.0127925260
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 2000000000000u128,
            45263157895u128, 0u128, -60000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 12792526001u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#165: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0166() {
        // F=100.0, K=80.000000, T=0.25, vol=0.0153874015
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 250000000000u128,
            45263157895u128, 0u128, -60000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 15387401451u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#166: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0167() {
        // F=100.0, K=95.000000, T=5.0, vol=0.0101450196
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 5000000000000u128,
            45263157895u128, 0u128, -60000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 10145019622u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#167: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0168() {
        // F=100.0, K=105.000000, T=0.5, vol=0.0026304005
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 500000000000u128,
            45263157895u128, 0u128, 80000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 2630400534u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#168: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0169() {
        // F=100.0, K=120.000000, T=10.0, vol=0.0142392646
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 10000000000000u128,
            45263157895u128, 0u128, 80000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 14239264616u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#169: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0170() {
        // F=100.0, K=200.000000, T=1.0, vol=0.0065583931
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 1000000000000u128,
            45263157895u128, 0u128, 220000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 6558393122u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#170: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0171() {
        // F=100.0, K=70.000000, T=0.25, vol=0.0171207015
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 250000000000u128,
            45263157895u128, 0u128, 220000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 17120701530u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#171: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0172() {
        // F=100.0, K=90.000000, T=5.0, vol=0.0123533510
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 5000000000000u128,
            45263157895u128, 0u128, 220000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 12353351013u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#172: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0173() {
        // F=100.0, K=100.000000, T=0.5, vol=0.0004527835
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 500000000000u128,
            45263157895u128, 0u128, 360000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 452783514u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#173: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0174() {
        // F=100.0, K=110.000000, T=10.0, vol=0.0066110462
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 10000000000000u128,
            45263157895u128, 0u128, 360000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 6611046198u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#174: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0175() {
        // F=100.0, K=150.000000, T=1.0, vol=0.0469056195
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 1000000000000u128,
            45263157895u128, 0u128, 360000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 46905619469u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#175: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0176() {
        // F=100.0, K=50.000000, T=0.25, vol=0.0201073021
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 250000000000u128,
            45263157895u128, 0u128, 500000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 20107302103u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#176: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0177() {
        // F=100.0, K=80.000000, T=5.0, vol=0.0141523147
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 5000000000000u128,
            45263157895u128, 0u128, 500000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 14152314709u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#177: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0178() {
        // F=100.0, K=95.000000, T=0.5, vol=0.0024767823
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 500000000000u128,
            45263157895u128, 250000000000u128, -900000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 2476782290u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#178: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0179() {
        // F=100.0, K=105.000000, T=10.0, vol=0.0020196151
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 10000000000000u128,
            45263157895u128, 250000000000u128, -900000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 2019615141u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#179: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0180() {
        // F=100.0, K=120.000000, T=1.0, vol=0.0146964648
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 1000000000000u128,
            45263157895u128, 250000000000u128, -900000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 14696464837u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#180: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0181() {
        // F=100.0, K=200.000000, T=0.1, vol=0.0109699226
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 100000000000u128,
            45263157895u128, 250000000000u128, -760000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 10969922563u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#181: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0182() {
        // F=100.0, K=70.000000, T=5.0, vol=0.0248168509
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 5000000000000u128,
            45263157895u128, 250000000000u128, -760000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 24816850924u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#182: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0183() {
        // F=100.0, K=90.000000, T=0.5, vol=0.0202436742
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 500000000000u128,
            45263157895u128, 250000000000u128, -760000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 20243674170u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#183: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0184() {
        // F=100.0, K=100.000000, T=10.0, vol=0.0014363183
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 10000000000000u128,
            45263157895u128, 250000000000u128, -620000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 1436318297u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#184: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0185() {
        // F=100.0, K=110.000000, T=1.0, vol=0.0077338158
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 1000000000000u128,
            45263157895u128, 250000000000u128, -620000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 7733815812u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#185: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0186() {
        // F=100.0, K=150.000000, T=0.1, vol=0.0049006383
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 100000000000u128,
            45263157895u128, 250000000000u128, -480000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 4900638273u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#186: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0187() {
        // F=100.0, K=50.000000, T=5.0, vol=0.0299878804
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 5000000000000u128,
            45263157895u128, 250000000000u128, -480000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 29987880412u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#187: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0188() {
        // F=100.0, K=80.000000, T=0.5, vol=0.0283616422
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 500000000000u128,
            45263157895u128, 250000000000u128, -480000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 28361642238u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#188: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0189() {
        // F=100.0, K=95.000000, T=10.0, vol=0.0021621241
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 10000000000000u128,
            45263157895u128, 250000000000u128, -340000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 2162124129u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#189: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0190() {
        // F=100.0, K=105.000000, T=1.0, vol=0.0043049858
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 1000000000000u128,
            45263157895u128, 250000000000u128, -340000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 4304985780u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#190: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0191() {
        // F=100.0, K=120.000000, T=0.1, vol=0.0252478925
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 100000000000u128,
            45263157895u128, 250000000000u128, -340000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 25247892536u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#191: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0192() {
        // F=100.0, K=200.000000, T=2.0, vol=0.0135851336
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 2000000000000u128,
            45263157895u128, 250000000000u128, -200000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 13585133578u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#192: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0193() {
        // F=100.0, K=70.000000, T=0.5, vol=0.0287346510
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 500000000000u128,
            45263157895u128, 250000000000u128, -200000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 28734650968u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#193: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0194() {
        // F=100.0, K=90.000000, T=10.0, vol=0.0277987738
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 10000000000000u128,
            45263157895u128, 250000000000u128, -200000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 27798773781u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#194: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0195() {
        // F=100.0, K=100.000000, T=1.0, vol=0.0014360907
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 1000000000000u128,
            45263157895u128, 250000000000u128, -60000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 1436090654u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#195: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0196() {
        // F=100.0, K=110.000000, T=0.1, vol=0.0128082490
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 100000000000u128,
            45263157895u128, 250000000000u128, -60000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 12808249023u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#196: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0197() {
        // F=100.0, K=150.000000, T=2.0, vol=0.0058994656
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 2000000000000u128,
            45263157895u128, 250000000000u128, 80000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 5899465618u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#197: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0198() {
        // F=100.0, K=50.000000, T=0.5, vol=0.0375616912
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 500000000000u128,
            45263157895u128, 250000000000u128, 80000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 37561691250u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#198: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0199() {
        // F=100.0, K=80.000000, T=10.0, vol=0.0331662218
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 10000000000000u128,
            45263157895u128, 250000000000u128, 80000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 33166221824u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#199: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0200() {
        // F=100.0, K=95.000000, T=1.0, vol=0.0023815361
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 1000000000000u128,
            45263157895u128, 250000000000u128, 220000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 2381536138u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#200: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0201() {
        // F=100.0, K=105.000000, T=0.1, vol=0.0062184388
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 100000000000u128,
            45263157895u128, 250000000000u128, 220000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 6218438794u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#201: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0202() {
        // F=100.0, K=120.000000, T=2.0, vol=0.0308680212
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 2000000000000u128,
            45263157895u128, 250000000000u128, 220000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 30868021181u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#202: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0203() {
        // F=100.0, K=200.000000, T=0.25, vol=0.0262699758
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 250000000000u128,
            45263157895u128, 250000000000u128, 360000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 26269975799u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#203: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0204() {
        // F=100.0, K=70.000000, T=10.0, vol=0.0281156090
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 10000000000000u128,
            45263157895u128, 250000000000u128, 360000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 28115608978u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#204: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0205() {
        // F=100.0, K=90.000000, T=1.0, vol=0.0020925606
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 1000000000000u128,
            45263157895u128, 250000000000u128, 500000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 2092560619u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#205: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0206() {
        // F=100.0, K=100.000000, T=0.1, vol=0.0014320196
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 100000000000u128,
            45263157895u128, 250000000000u128, 500000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 1432019602u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#206: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0207() {
        // F=100.0, K=110.000000, T=2.0, vol=0.0147446072
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 2000000000000u128,
            45263157895u128, 250000000000u128, 500000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 14744607157u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#207: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0208() {
        // F=100.0, K=150.000000, T=0.25, vol=0.0077910456
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 250000000000u128,
            45263157895u128, 500000000000u128, -900000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 7791045646u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#208: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0209() {
        // F=100.0, K=50.000000, T=10.0, vol=0.0544989642
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 10000000000000u128,
            45263157895u128, 500000000000u128, -900000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 54498964238u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#209: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0210() {
        // F=100.0, K=80.000000, T=1.0, vol=0.0477240385
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 1000000000000u128,
            45263157895u128, 500000000000u128, -900000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 47724038509u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#210: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0211() {
        // F=100.0, K=95.000000, T=0.1, vol=0.0082920007
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 100000000000u128,
            45263157895u128, 500000000000u128, -760000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 8292000739u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#211: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0212() {
        // F=100.0, K=105.000000, T=2.0, vol=0.0057406408
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 2000000000000u128,
            45263157895u128, 500000000000u128, -760000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 5740640843u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#212: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0213() {
        // F=100.0, K=120.000000, T=0.25, vol=0.0042959123
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 250000000000u128,
            45263157895u128, 500000000000u128, -620000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 4295912270u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#213: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0214() {
        // F=100.0, K=200.000000, T=5.0, vol=0.0265078130
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 5000000000000u128,
            45263157895u128, 500000000000u128, -620000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 26507812986u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#214: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0215() {
        // F=100.0, K=70.000000, T=1.0, vol=0.0542057855
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 1000000000000u128,
            45263157895u128, 500000000000u128, -620000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 54205785484u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#215: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0216() {
        // F=100.0, K=90.000000, T=0.1, vol=0.0078842023
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 100000000000u128,
            45263157895u128, 500000000000u128, -480000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 7884202322u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#216: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0217() {
        // F=100.0, K=100.000000, T=2.0, vol=0.0045700105
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 2000000000000u128,
            45263157895u128, 500000000000u128, -480000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 4570010486u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#217: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0218() {
        // F=100.0, K=110.000000, T=0.25, vol=0.0184530321
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 250000000000u128,
            45263157895u128, 500000000000u128, -480000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 18453032141u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#218: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0219() {
        // F=100.0, K=150.000000, T=5.0, vol=0.0120510896
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 5000000000000u128,
            45263157895u128, 500000000000u128, -340000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 12051089602u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#219: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0220() {
        // F=100.0, K=50.000000, T=1.0, vol=0.0641292677
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 1000000000000u128,
            45263157895u128, 500000000000u128, -340000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 64129267725u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#220: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0221() {
        // F=100.0, K=80.000000, T=0.1, vol=0.0075602150
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 100000000000u128,
            45263157895u128, 500000000000u128, -200000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 7560215026u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#221: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0222() {
        // F=100.0, K=95.000000, T=2.0, vol=0.0071683375
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 2000000000000u128,
            45263157895u128, 500000000000u128, -200000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 7168337524u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#222: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0223() {
        // F=100.0, K=105.000000, T=0.25, vol=0.0106417053
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 250000000000u128,
            45263157895u128, 500000000000u128, -200000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 10641705311u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#223: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0224() {
        // F=100.0, K=120.000000, T=5.0, vol=0.0059757028
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 5000000000000u128,
            45263157895u128, 500000000000u128, -60000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 5975702776u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#224: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0225() {
        // F=100.0, K=200.000000, T=0.5, vol=0.0436788602
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 500000000000u128,
            45263157895u128, 500000000000u128, -60000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 43678860159u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#225: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0226() {
        // F=100.0, K=70.000000, T=0.1, vol=0.0610738709
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 100000000000u128,
            45263157895u128, 500000000000u128, -60000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 61073870947u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#226: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0227() {
        // F=100.0, K=90.000000, T=2.0, vol=0.0065646325
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 2000000000000u128,
            45263157895u128, 500000000000u128, 80000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 6564632479u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#227: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0228() {
        // F=100.0, K=100.000000, T=0.25, vol=0.0045412794
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 250000000000u128,
            45263157895u128, 500000000000u128, 80000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 4541279396u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#228: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0229() {
        // F=100.0, K=110.000000, T=5.0, vol=0.0277817584
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 5000000000000u128,
            45263157895u128, 500000000000u128, 80000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 27781758385u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#229: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0230() {
        // F=100.0, K=150.000000, T=0.5, vol=0.0232123921
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 500000000000u128,
            45263157895u128, 500000000000u128, 220000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 23212392079u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#230: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0231() {
        // F=100.0, K=50.000000, T=0.1, vol=0.0785031598
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 100000000000u128,
            45263157895u128, 500000000000u128, 220000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 78503159785u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#231: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0232() {
        // F=100.0, K=80.000000, T=2.0, vol=0.0059657921
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 2000000000000u128,
            45263157895u128, 500000000000u128, 360000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 5965792074u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#232: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0233() {
        // F=100.0, K=95.000000, T=0.25, vol=0.0068183511
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 250000000000u128,
            45263157895u128, 500000000000u128, 360000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 6818351056u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#233: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0234() {
        // F=100.0, K=105.000000, T=5.0, vol=0.0141336649
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 5000000000000u128,
            45263157895u128, 500000000000u128, 360000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 14133664917u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#234: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0235() {
        // F=100.0, K=120.000000, T=0.5, vol=0.0098708109
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 500000000000u128,
            45263157895u128, 500000000000u128, 500000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 9870810858u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#235: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0236() {
        // F=100.0, K=200.000000, T=10.0, vol=0.0504371674
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 10000000000000u128,
            45263157895u128, 500000000000u128, 500000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 50437167389u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#236: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0237() {
        // F=100.0, K=70.000000, T=2.0, vol=0.0559806681
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 2000000000000u128,
            45263157895u128, 500000000000u128, 500000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 55980668103u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#237: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0238() {
        // F=100.0, K=90.000000, T=0.25, vol=0.0230113464
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 250000000000u128,
            45263157895u128, 750000000000u128, -900000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 23011346422u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#238: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0239() {
        // F=100.0, K=100.000000, T=5.0, vol=0.0140392004
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 5000000000000u128,
            45263157895u128, 750000000000u128, -900000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 14039200360u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#239: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0240() {
        // F=100.0, K=110.000000, T=0.5, vol=0.0124033015
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 500000000000u128,
            45263157895u128, 750000000000u128, -760000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 12403301465u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#240: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0241() {
        // F=100.0, K=150.000000, T=10.0, vol=0.0214973084
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 10000000000000u128,
            45263157895u128, 750000000000u128, -760000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 21497308418u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#241: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0242() {
        // F=100.0, K=50.000000, T=2.0, vol=0.1214393109
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 2000000000000u128,
            45263157895u128, 750000000000u128, -760000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 121439310932u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#242: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0243() {
        // F=100.0, K=80.000000, T=0.25, vol=0.0222715197
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 250000000000u128,
            45263157895u128, 750000000000u128, -620000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 22271519680u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#243: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0244() {
        // F=100.0, K=95.000000, T=5.0, vol=0.0198778728
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 5000000000000u128,
            45263157895u128, 750000000000u128, -620000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 19877872797u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#244: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0245() {
        // F=100.0, K=105.000000, T=0.5, vol=0.0161246808
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 500000000000u128,
            45263157895u128, 750000000000u128, -620000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 16124680752u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#245: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0246() {
        // F=100.0, K=120.000000, T=10.0, vol=0.0134606497
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 10000000000000u128,
            45263157895u128, 750000000000u128, -480000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 13460649661u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#246: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0247() {
        // F=100.0, K=200.000000, T=1.0, vol=0.0638175884
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 1000000000000u128,
            45263157895u128, 750000000000u128, -480000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 63817588406u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#247: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0248() {
        // F=100.0, K=70.000000, T=0.25, vol=0.0195922535
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 250000000000u128,
            45263157895u128, 750000000000u128, -340000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 19592253477u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#248: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0249() {
        // F=100.0, K=90.000000, T=5.0, vol=0.0203856395
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 5000000000000u128,
            45263157895u128, 750000000000u128, -340000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 20385639450u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#249: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0250() {
        // F=100.0, K=100.000000, T=0.5, vol=0.0144870256
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 500000000000u128,
            45263157895u128, 750000000000u128, -340000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 14487025562u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#250: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0251() {
        // F=100.0, K=110.000000, T=10.0, vol=0.0139561731
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 10000000000000u128,
            45263157895u128, 750000000000u128, -200000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 13956173066u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#251: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0252() {
        // F=100.0, K=150.000000, T=1.0, vol=0.0396542755
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 1000000000000u128,
            45263157895u128, 750000000000u128, -200000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 39654275536u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#252: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0253() {
        // F=100.0, K=50.000000, T=0.25, vol=0.1371953133
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 250000000000u128,
            45263157895u128, 750000000000u128, -200000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 137195313333u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#253: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0254() {
        // F=100.0, K=80.000000, T=5.0, vol=0.0190220243
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 5000000000000u128,
            45263157895u128, 750000000000u128, -60000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 19022024328u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#254: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0255() {
        // F=100.0, K=95.000000, T=0.5, vol=0.0183183019
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 500000000000u128,
            45263157895u128, 750000000000u128, -60000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 18318301923u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#255: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0256() {
        // F=100.0, K=105.000000, T=10.0, vol=0.0336241518
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 10000000000000u128,
            45263157895u128, 750000000000u128, -60000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 33624151784u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#256: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0257() {
        // F=100.0, K=120.000000, T=1.0, vol=0.0223460853
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 1000000000000u128,
            45263157895u128, 750000000000u128, 80000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 22346085293u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#257: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0258() {
        // F=100.0, K=200.000000, T=0.1, vol=0.1022396533
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 100000000000u128,
            45263157895u128, 750000000000u128, 80000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 102239653268u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#258: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0259() {
        // F=100.0, K=70.000000, T=5.0, vol=0.0161928151
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 5000000000000u128,
            45263157895u128, 750000000000u128, 220000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 16192815105u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#259: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0260() {
        // F=100.0, K=90.000000, T=0.5, vol=0.0189811658
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 500000000000u128,
            45263157895u128, 750000000000u128, 220000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 18981165805u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#260: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0261() {
        // F=100.0, K=100.000000, T=10.0, vol=0.0183465432
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 10000000000000u128,
            45263157895u128, 750000000000u128, 220000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 18346543192u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#261: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0262() {
        // F=100.0, K=110.000000, T=1.0, vol=0.0164751959
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 1000000000000u128,
            45263157895u128, 750000000000u128, 360000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 16475195875u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#262: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0263() {
        // F=100.0, K=150.000000, T=0.1, vol=0.0560594437
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 100000000000u128,
            45263157895u128, 750000000000u128, 360000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 56059443661u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#263: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0264() {
        // F=100.0, K=50.000000, T=5.0, vol=0.1436558882
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 5000000000000u128,
            45263157895u128, 750000000000u128, 360000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 143655888153u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#264: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0265() {
        // F=100.0, K=80.000000, T=0.5, vol=0.0190643118
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 500000000000u128,
            45263157895u128, 750000000000u128, 500000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 19064311769u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#265: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0266() {
        // F=100.0, K=95.000000, T=10.0, vol=0.0151192908
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 10000000000000u128,
            45263157895u128, 750000000000u128, 500000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 15119290807u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#266: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0267() {
        // F=100.0, K=105.000000, T=1.0, vol=0.0441362471
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 1000000000000u128,
            45263157895u128, 1000000000000u128, -900000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 44136247128u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#267: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0268() {
        // F=100.0, K=120.000000, T=0.1, vol=0.0256544163
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 100000000000u128,
            45263157895u128, 1000000000000u128, -900000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 25654416344u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#268: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0269() {
        // F=100.0, K=200.000000, T=2.0, vol=0.0793230629
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 2000000000000u128,
            45263157895u128, 1000000000000u128, -900000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 79323062919u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#269: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0270() {
        // F=100.0, K=70.000000, T=0.5, vol=0.0588148708
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 500000000000u128,
            45263157895u128, 1000000000000u128, -760000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 58814870751u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#270: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0271() {
        // F=100.0, K=90.000000, T=10.0, vol=0.0564152221
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 10000000000000u128,
            45263157895u128, 1000000000000u128, -760000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 56415222087u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#271: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0272() {
        // F=100.0, K=100.000000, T=1.0, vol=0.0452742625
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 1000000000000u128,
            45263157895u128, 1000000000000u128, -760000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 45274262456u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#272: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0273() {
        // F=100.0, K=110.000000, T=0.1, vol=0.0401994197
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 100000000000u128,
            45263157895u128, 1000000000000u128, -620000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 40199419708u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#273: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0274() {
        // F=100.0, K=150.000000, T=2.0, vol=0.0590095255
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 2000000000000u128,
            45263157895u128, 1000000000000u128, -620000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 59009525455u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#274: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0275() {
        // F=100.0, K=50.000000, T=0.5, vol=0.0553113335
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 500000000000u128,
            45263157895u128, 1000000000000u128, -480000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 55311333539u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#275: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0276() {
        // F=100.0, K=80.000000, T=10.0, vol=0.0591255260
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 10000000000000u128,
            45263157895u128, 1000000000000u128, -480000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 59125525965u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#276: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0277() {
        // F=100.0, K=95.000000, T=1.0, vol=0.0549795052
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 1000000000000u128,
            45263157895u128, 1000000000000u128, -480000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 54979505229u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#277: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0278() {
        // F=100.0, K=105.000000, T=0.1, vol=0.0445108912
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 100000000000u128,
            45263157895u128, 1000000000000u128, -340000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 44510891190u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#278: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0279() {
        // F=100.0, K=120.000000, T=2.0, vol=0.0468100173
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 2000000000000u128,
            45263157895u128, 1000000000000u128, -340000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 46810017269u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#279: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0280() {
        // F=100.0, K=200.000000, T=0.25, vol=0.1561463016
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 250000000000u128,
            45263157895u128, 1000000000000u128, -340000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 156146301639u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#280: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0281() {
        // F=100.0, K=70.000000, T=10.0, vol=0.0523936779
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 10000000000000u128,
            45263157895u128, 1000000000000u128, -200000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 52393677929u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#281: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0282() {
        // F=100.0, K=90.000000, T=1.0, vol=0.0543992609
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 1000000000000u128,
            45263157895u128, 1000000000000u128, -200000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 54399260912u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#282: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0283() {
        // F=100.0, K=100.000000, T=0.1, vol=0.0452639421
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 100000000000u128,
            45263157895u128, 1000000000000u128, -60000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 45263942129u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#283: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0284() {
        // F=100.0, K=110.000000, T=2.0, vol=0.0462915203
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 2000000000000u128,
            45263157895u128, 1000000000000u128, -60000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 46291520312u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#284: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0285() {
        // F=100.0, K=150.000000, T=0.25, vol=0.1006767548
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 250000000000u128,
            45263157895u128, 1000000000000u128, -60000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 100676754777u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#285: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0286() {
        // F=100.0, K=50.000000, T=10.0, vol=0.0480634536
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 10000000000000u128,
            45263157895u128, 1000000000000u128, 80000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 48063453626u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#286: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0287() {
        // F=100.0, K=80.000000, T=1.0, vol=0.0552861310
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 1000000000000u128,
            45263157895u128, 1000000000000u128, 80000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 55286130985u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#287: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0288() {
        // F=100.0, K=95.000000, T=0.1, vol=0.0493764132
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 100000000000u128,
            45263157895u128, 1000000000000u128, 80000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 49376413228u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#288: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0289() {
        // F=100.0, K=105.000000, T=2.0, vol=0.0459722720
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 2000000000000u128,
            45263157895u128, 1000000000000u128, 220000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 45972271967u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#289: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0290() {
        // F=100.0, K=120.000000, T=0.25, vol=0.0628802564
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 250000000000u128,
            45263157895u128, 1000000000000u128, 220000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 62880256361u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#290: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0291() {
        // F=100.0, K=200.000000, T=5.0, vol=0.2309119755
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 5000000000000u128,
            45263157895u128, 1000000000000u128, 220000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 230911975461u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#291: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0292() {
        // F=100.0, K=70.000000, T=1.0, vol=0.0491178832
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 1000000000000u128,
            45263157895u128, 1000000000000u128, 360000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 49117883163u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#292: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0293() {
        // F=100.0, K=90.000000, T=0.1, vol=0.0474895589
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 100000000000u128,
            45263157895u128, 1000000000000u128, 360000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 47489558894u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#293: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0294() {
        // F=100.0, K=100.000000, T=2.0, vol=0.0453005546
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 2000000000000u128,
            45263157895u128, 1000000000000u128, 500000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 45300554594u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#294: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0295() {
        // F=100.0, K=110.000000, T=0.25, vol=0.0537035647
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 250000000000u128,
            45263157895u128, 1000000000000u128, 500000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 53703564719u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#295: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0296() {
        // F=100.0, K=150.000000, T=5.0, vol=0.1309351382
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 5000000000000u128,
            45263157895u128, 1000000000000u128, 500000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 130935138234u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#296: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0297() {
        // F=100.0, K=50.000000, T=1.0, vol=0.0157786891
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 1000000000000u128,
            70526315789u128, 0u128, -900000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 15778689059u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#297: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0298() {
        // F=100.0, K=80.000000, T=0.1, vol=0.0185965754
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 100000000000u128,
            70526315789u128, 0u128, -900000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 18596575375u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#298: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0299() {
        // F=100.0, K=95.000000, T=2.0, vol=0.0097659222
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 2000000000000u128,
            70526315789u128, 0u128, -900000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 9765922178u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#299: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0300() {
        // F=100.0, K=105.000000, T=0.25, vol=0.0020685338
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 250000000000u128,
            70526315789u128, 0u128, -760000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 2068533805u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#300: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0301() {
        // F=100.0, K=120.000000, T=5.0, vol=0.0107368752
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 5000000000000u128,
            70526315789u128, 0u128, -760000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 10736875234u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#301: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0302() {
        // F=100.0, K=200.000000, T=0.5, vol=0.0057651899
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 500000000000u128,
            70526315789u128, 0u128, -620000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 5765189863u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#302: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0303() {
        // F=100.0, K=70.000000, T=0.1, vol=0.0210446417
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 100000000000u128,
            70526315789u128, 0u128, -620000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 21044641699u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#303: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0304() {
        // F=100.0, K=90.000000, T=2.0, vol=0.0139007234
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 2000000000000u128,
            70526315789u128, 0u128, -620000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 13900723447u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#304: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0305() {
        // F=100.0, K=100.000000, T=0.25, vol=0.0007053593
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 250000000000u128,
            70526315789u128, 0u128, -480000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 705359312u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#305: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0306() {
        // F=100.0, K=110.000000, T=5.0, vol=0.0057578857
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 5000000000000u128,
            70526315789u128, 0u128, -480000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 5757885699u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#306: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0307() {
        // F=100.0, K=150.000000, T=0.5, vol=0.0426798660
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 500000000000u128,
            70526315789u128, 0u128, -480000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 42679866042u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#307: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0308() {
        // F=100.0, K=50.000000, T=0.1, vol=0.0254677307
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 100000000000u128,
            70526315789u128, 0u128, -340000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 25467730665u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#308: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0309() {
        // F=100.0, K=80.000000, T=2.0, vol=0.0177366946
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 2000000000000u128,
            70526315789u128, 0u128, -340000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 17736694583u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#309: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0310() {
        // F=100.0, K=95.000000, T=0.25, vol=0.0013878881
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 250000000000u128,
            70526315789u128, 0u128, -200000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 1387888107u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#310: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0311() {
        // F=100.0, K=105.000000, T=5.0, vol=0.0027876182
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 5000000000000u128,
            70526315789u128, 0u128, -200000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 2787618166u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#311: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0312() {
        // F=100.0, K=120.000000, T=0.5, vol=0.0183098046
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 500000000000u128,
            70526315789u128, 0u128, -200000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 18309804597u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#312: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0313() {
        // F=100.0, K=200.000000, T=10.0, vol=0.0068148765
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 10000000000000u128,
            70526315789u128, 0u128, -60000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 6814876476u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#313: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0314() {
        // F=100.0, K=70.000000, T=2.0, vol=0.0197160165
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 2000000000000u128,
            70526315789u128, 0u128, -60000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 19716016469u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#314: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0315() {
        // F=100.0, K=90.000000, T=0.25, vol=0.0159082955
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 250000000000u128,
            70526315789u128, 0u128, -60000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 15908295511u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#315: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0316() {
        // F=100.0, K=100.000000, T=5.0, vol=0.0007081736
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 5000000000000u128,
            70526315789u128, 0u128, 80000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 708173617u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#316: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0317() {
        // F=100.0, K=110.000000, T=0.5, vol=0.0082402400
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 500000000000u128,
            70526315789u128, 0u128, 80000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 8240239992u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#317: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0318() {
        // F=100.0, K=150.000000, T=10.0, vol=0.0708375039
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 10000000000000u128,
            70526315789u128, 0u128, 80000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 70837503855u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#318: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0319() {
        // F=100.0, K=50.000000, T=2.0, vol=0.0232758513
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 2000000000000u128,
            70526315789u128, 0u128, 220000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 23275851322u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#319: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0320() {
        // F=100.0, K=80.000000, T=0.25, vol=0.0221464679
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 250000000000u128,
            70526315789u128, 0u128, 220000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 22146467882u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#320: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0321() {
        // F=100.0, K=95.000000, T=5.0, vol=0.0011069842
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 5000000000000u128,
            70526315789u128, 0u128, 360000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 1106984210u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#321: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0322() {
        // F=100.0, K=105.000000, T=0.5, vol=0.0042426601
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 500000000000u128,
            70526315789u128, 0u128, 360000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 4242660093u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#322: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0323() {
        // F=100.0, K=120.000000, T=10.0, vol=0.0245593293
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 10000000000000u128,
            70526315789u128, 0u128, 360000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 24559329336u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#323: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0324() {
        // F=100.0, K=200.000000, T=1.0, vol=0.0130091201
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 1000000000000u128,
            70526315789u128, 0u128, 500000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 13009120103u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#324: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0325() {
        // F=100.0, K=70.000000, T=0.25, vol=0.0218293513
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 250000000000u128,
            70526315789u128, 0u128, 500000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 21829351347u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#325: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0326() {
        // F=100.0, K=90.000000, T=5.0, vol=0.0160799682
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 5000000000000u128,
            70526315789u128, 0u128, 500000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 16079968222u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#326: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0327() {
        // F=100.0, K=100.000000, T=0.5, vol=0.0022294109
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 500000000000u128,
            70526315789u128, 250000000000u128, -900000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 2229410912u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#327: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0328() {
        // F=100.0, K=110.000000, T=10.0, vol=0.0063601485
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 10000000000000u128,
            70526315789u128, 250000000000u128, -900000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 6360148453u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#328: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0329() {
        // F=100.0, K=150.000000, T=1.0, vol=0.0045836287
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 1000000000000u128,
            70526315789u128, 250000000000u128, -760000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 4583628688u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#329: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0330() {
        // F=100.0, K=50.000000, T=0.25, vol=0.0465992223
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 250000000000u128,
            70526315789u128, 250000000000u128, -760000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 46599222269u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#330: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0331() {
        // F=100.0, K=80.000000, T=5.0, vol=0.0328370720
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 5000000000000u128,
            70526315789u128, 250000000000u128, -760000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 32837072008u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#331: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0332() {
        // F=100.0, K=95.000000, T=0.5, vol=0.0039913354
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 500000000000u128,
            70526315789u128, 250000000000u128, -620000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 3991335351u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#332: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0333() {
        // F=100.0, K=105.000000, T=10.0, vol=0.0043542066
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 10000000000000u128,
            70526315789u128, 250000000000u128, -620000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 4354206577u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#333: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0334() {
        // F=100.0, K=120.000000, T=1.0, vol=0.0252552795
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 1000000000000u128,
            70526315789u128, 250000000000u128, -620000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 25255279489u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#334: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0335() {
        // F=100.0, K=200.000000, T=0.1, vol=0.0239253481
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 100000000000u128,
            70526315789u128, 250000000000u128, -480000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 23925348107u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#335: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0336() {
        // F=100.0, K=70.000000, T=5.0, vol=0.0342447030
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 5000000000000u128,
            70526315789u128, 250000000000u128, -480000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 34244702983u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#336: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0337() {
        // F=100.0, K=90.000000, T=0.5, vol=0.0037865033
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 500000000000u128,
            70526315789u128, 250000000000u128, -340000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 3786503279u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#337: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0338() {
        // F=100.0, K=100.000000, T=10.0, vol=0.0022914796
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 10000000000000u128,
            70526315789u128, 250000000000u128, -340000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 2291479624u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#338: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0339() {
        // F=100.0, K=110.000000, T=1.0, vol=0.0133888231
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 1000000000000u128,
            70526315789u128, 250000000000u128, -340000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 13388823064u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#339: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0340() {
        // F=100.0, K=150.000000, T=0.1, vol=0.0102011557
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 100000000000u128,
            70526315789u128, 250000000000u128, -200000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 10201155661u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#340: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0341() {
        // F=100.0, K=50.000000, T=5.0, vol=0.0444484027
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 5000000000000u128,
            70526315789u128, 250000000000u128, -200000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 44448402736u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#341: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0342() {
        // F=100.0, K=80.000000, T=0.5, vol=0.0379792882
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 500000000000u128,
            70526315789u128, 250000000000u128, -200000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 37979288152u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#342: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0343() {
        // F=100.0, K=95.000000, T=10.0, vol=0.0034061987
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 10000000000000u128,
            70526315789u128, 250000000000u128, -60000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 3406198715u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#343: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0344() {
        // F=100.0, K=105.000000, T=1.0, vol=0.0067283666
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 1000000000000u128,
            70526315789u128, 250000000000u128, -60000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 6728366624u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#344: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0345() {
        // F=100.0, K=120.000000, T=0.1, vol=0.0042909496
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 100000000000u128,
            70526315789u128, 250000000000u128, 80000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 4290949623u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#345: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0346() {
        // F=100.0, K=200.000000, T=2.0, vol=0.0275595237
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 2000000000000u128,
            70526315789u128, 250000000000u128, 80000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 27559523748u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#346: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0347() {
        // F=100.0, K=70.000000, T=0.5, vol=0.0415961009
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 500000000000u128,
            70526315789u128, 250000000000u128, 80000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 41596100896u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#347: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0348() {
        // F=100.0, K=90.000000, T=10.0, vol=0.0030822505
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 10000000000000u128,
            70526315789u128, 250000000000u128, 220000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 3082250530u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#348: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0349() {
        // F=100.0, K=100.000000, T=1.0, vol=0.0022457711
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 1000000000000u128,
            70526315789u128, 250000000000u128, 220000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 2245771126u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#349: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0350() {
        // F=100.0, K=110.000000, T=0.1, vol=0.0188307546
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 100000000000u128,
            70526315789u128, 250000000000u128, 220000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 18830754625u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#350: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0351() {
        // F=100.0, K=150.000000, T=2.0, vol=0.0117049905
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 2000000000000u128,
            70526315789u128, 250000000000u128, 360000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 11704990523u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#351: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0352() {
        // F=100.0, K=50.000000, T=0.5, vol=0.0483692160
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 500000000000u128,
            70526315789u128, 250000000000u128, 360000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 48369215983u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#352: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0353() {
        // F=100.0, K=80.000000, T=10.0, vol=0.0469304895
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 10000000000000u128,
            70526315789u128, 250000000000u128, 360000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 46930489522u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#353: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0354() {
        // F=100.0, K=95.000000, T=1.0, vol=0.0036882889
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 1000000000000u128,
            70526315789u128, 250000000000u128, 500000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 3688288944u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#354: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0355() {
        // F=100.0, K=105.000000, T=0.1, vol=0.0100496227
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 100000000000u128,
            70526315789u128, 250000000000u128, 500000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 10049622746u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#355: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0356() {
        // F=100.0, K=120.000000, T=2.0, vol=0.0038269584
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 2000000000000u128,
            70526315789u128, 500000000000u128, -900000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 3826958422u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#356: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0357() {
        // F=100.0, K=200.000000, T=0.25, vol=0.0316861812
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 250000000000u128,
            70526315789u128, 500000000000u128, -900000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 31686181246u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#357: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0358() {
        // F=100.0, K=70.000000, T=10.0, vol=0.0583632381
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 10000000000000u128,
            70526315789u128, 500000000000u128, -900000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 58363238113u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#358: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0359() {
        // F=100.0, K=90.000000, T=1.0, vol=0.0111604828
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 1000000000000u128,
            70526315789u128, 500000000000u128, -760000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 11160482810u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#359: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0360() {
        // F=100.0, K=100.000000, T=0.1, vol=0.0070536992
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 100000000000u128,
            70526315789u128, 500000000000u128, -760000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 7053699243u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#360: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0361() {
        // F=100.0, K=110.000000, T=2.0, vol=0.0173412451
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 2000000000000u128,
            70526315789u128, 500000000000u128, -760000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 17341245072u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#361: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0362() {
        // F=100.0, K=150.000000, T=0.25, vol=0.0194881466
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 250000000000u128,
            70526315789u128, 500000000000u128, -620000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 19488146613u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#362: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0363() {
        // F=100.0, K=50.000000, T=10.0, vol=0.0779930904
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 10000000000000u128,
            70526315789u128, 500000000000u128, -620000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 77993090357u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#363: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0364() {
        // F=100.0, K=80.000000, T=1.0, vol=0.0108464015
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 1000000000000u128,
            70526315789u128, 500000000000u128, -480000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 10846401465u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#364: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0365() {
        // F=100.0, K=95.000000, T=0.1, vol=0.0118762202
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 100000000000u128,
            70526315789u128, 500000000000u128, -480000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 11876220197u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#365: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0366() {
        // F=100.0, K=105.000000, T=2.0, vol=0.0113680449
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 2000000000000u128,
            70526315789u128, 500000000000u128, -480000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 11368044949u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#366: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0367() {
        // F=100.0, K=120.000000, T=0.25, vol=0.0090822104
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 250000000000u128,
            70526315789u128, 500000000000u128, -340000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 9082210361u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#367: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0368() {
        // F=100.0, K=200.000000, T=5.0, vol=0.0457970478
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 5000000000000u128,
            70526315789u128, 500000000000u128, -340000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 45797047783u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#368: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0369() {
        // F=100.0, K=70.000000, T=1.0, vol=0.0739761757
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 1000000000000u128,
            70526315789u128, 500000000000u128, -340000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 73976175675u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#369: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0370() {
        // F=100.0, K=90.000000, T=0.1, vol=0.0126247370
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 100000000000u128,
            70526315789u128, 500000000000u128, -200000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 12624737028u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#370: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0371() {
        // F=100.0, K=100.000000, T=2.0, vol=0.0072284301
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 2000000000000u128,
            70526315789u128, 500000000000u128, -200000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 7228430060u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#371: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0372() {
        // F=100.0, K=110.000000, T=0.25, vol=0.0072519460
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 250000000000u128,
            70526315789u128, 500000000000u128, -60000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 7251945966u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#372: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0373() {
        // F=100.0, K=150.000000, T=5.0, vol=0.0249718907
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 5000000000000u128,
            70526315789u128, 500000000000u128, -60000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 24971890713u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#373: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0374() {
        // F=100.0, K=50.000000, T=1.0, vol=0.0938665139
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 1000000000000u128,
            70526315789u128, 500000000000u128, -60000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 93866513870u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#374: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0375() {
        // F=100.0, K=80.000000, T=0.1, vol=0.0119050068
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 100000000000u128,
            70526315789u128, 500000000000u128, 80000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 11905006824u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#375: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0376() {
        // F=100.0, K=95.000000, T=2.0, vol=0.0100149943
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 2000000000000u128,
            70526315789u128, 500000000000u128, 80000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 10014994296u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#376: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0377() {
        // F=100.0, K=105.000000, T=0.25, vol=0.0167490791
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 250000000000u128,
            70526315789u128, 500000000000u128, 80000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 16749079098u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#377: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0378() {
        // F=100.0, K=120.000000, T=5.0, vol=0.0114754398
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 5000000000000u128,
            70526315789u128, 500000000000u128, 220000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 11475439841u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#378: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0379() {
        // F=100.0, K=200.000000, T=0.5, vol=0.0639030561
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 500000000000u128,
            70526315789u128, 500000000000u128, 220000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 63903056122u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#379: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0380() {
        // F=100.0, K=70.000000, T=0.1, vol=0.0095678859
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 100000000000u128,
            70526315789u128, 500000000000u128, 360000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 9567885900u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#380: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0381() {
        // F=100.0, K=90.000000, T=2.0, vol=0.0100525391
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 2000000000000u128,
            70526315789u128, 500000000000u128, 360000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 10052539078u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#381: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0382() {
        // F=100.0, K=100.000000, T=0.25, vol=0.0070955802
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 250000000000u128,
            70526315789u128, 500000000000u128, 360000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 7095580235u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#382: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0383() {
        // F=100.0, K=110.000000, T=5.0, vol=0.0082950431
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 5000000000000u128,
            70526315789u128, 500000000000u128, 500000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 8295043089u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#383: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0384() {
        // F=100.0, K=150.000000, T=0.5, vol=0.0373034841
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 500000000000u128,
            70526315789u128, 500000000000u128, 500000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 37303484101u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#384: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0385() {
        // F=100.0, K=50.000000, T=0.1, vol=0.0993893883
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 100000000000u128,
            70526315789u128, 500000000000u128, 500000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 99389388313u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#385: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0386() {
        // F=100.0, K=80.000000, T=2.0, vol=0.0322002104
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 2000000000000u128,
            70526315789u128, 750000000000u128, -900000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 32200210375u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#386: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0387() {
        // F=100.0, K=95.000000, T=0.25, vol=0.0310054503
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 250000000000u128,
            70526315789u128, 750000000000u128, -900000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 31005450256u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#387: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0388() {
        // F=100.0, K=105.000000, T=5.0, vol=0.0124666467
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 5000000000000u128,
            70526315789u128, 750000000000u128, -900000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 12466646742u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#388: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0389() {
        // F=100.0, K=120.000000, T=0.5, vol=0.0170628002
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 500000000000u128,
            70526315789u128, 750000000000u128, -760000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 17062800210u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#389: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0390() {
        // F=100.0, K=200.000000, T=10.0, vol=0.0596601698
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 10000000000000u128,
            70526315789u128, 750000000000u128, -760000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 59660169773u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#390: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0391() {
        // F=100.0, K=70.000000, T=2.0, vol=0.0293061288
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 2000000000000u128,
            70526315789u128, 750000000000u128, -620000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 29306128768u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#391: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0392() {
        // F=100.0, K=90.000000, T=0.25, vol=0.0333267867
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 250000000000u128,
            70526315789u128, 750000000000u128, -620000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 33326786671u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#392: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0393() {
        // F=100.0, K=100.000000, T=5.0, vol=0.0235454809
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 5000000000000u128,
            70526315789u128, 750000000000u128, -620000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 23545480940u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#393: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0394() {
        // F=100.0, K=110.000000, T=0.5, vol=0.0203320708
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 500000000000u128,
            70526315789u128, 750000000000u128, -480000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 20332070799u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#394: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0395() {
        // F=100.0, K=150.000000, T=10.0, vol=0.0420952962
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 10000000000000u128,
            70526315789u128, 750000000000u128, -480000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 42095296236u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#395: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0396() {
        // F=100.0, K=50.000000, T=2.0, vol=0.1712193349
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 2000000000000u128,
            70526315789u128, 750000000000u128, -480000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 171219334886u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#396: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0397() {
        // F=100.0, K=80.000000, T=0.25, vol=0.0352370300
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 250000000000u128,
            70526315789u128, 750000000000u128, -340000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 35237029961u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#397: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0398() {
        // F=100.0, K=95.000000, T=5.0, vol=0.0289930314
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 5000000000000u128,
            70526315789u128, 750000000000u128, -340000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 28993031419u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#398: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0399() {
        // F=100.0, K=105.000000, T=0.5, vol=0.0219675096
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 500000000000u128,
            70526315789u128, 750000000000u128, -200000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 21967509628u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#399: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0400() {
        // F=100.0, K=120.000000, T=10.0, vol=0.0268330091
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 10000000000000u128,
            70526315789u128, 750000000000u128, -200000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 26833009067u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#400: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0401() {
        // F=100.0, K=200.000000, T=1.0, vol=0.1089184338
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 1000000000000u128,
            70526315789u128, 750000000000u128, -200000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 108918433776u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#401: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0402() {
        // F=100.0, K=70.000000, T=0.25, vol=0.0300983557
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 250000000000u128,
            70526315789u128, 750000000000u128, -60000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 30098355665u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#402: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0403() {
        // F=100.0, K=90.000000, T=5.0, vol=0.0294453042
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 5000000000000u128,
            70526315789u128, 750000000000u128, -60000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 29445304231u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#403: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0404() {
        // F=100.0, K=100.000000, T=0.5, vol=0.0228916740
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 500000000000u128,
            70526315789u128, 750000000000u128, -60000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 22891674021u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#404: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0405() {
        // F=100.0, K=110.000000, T=10.0, vol=0.0232394857
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 10000000000000u128,
            70526315789u128, 750000000000u128, 80000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 23239485682u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#405: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0406() {
        // F=100.0, K=150.000000, T=1.0, vol=0.0616323653
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 1000000000000u128,
            70526315789u128, 750000000000u128, 80000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 61632365279u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#406: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0407() {
        // F=100.0, K=50.000000, T=0.25, vol=0.0274293405
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 250000000000u128,
            70526315789u128, 750000000000u128, 220000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 27429340460u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#407: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0408() {
        // F=100.0, K=80.000000, T=5.0, vol=0.0290900690
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 5000000000000u128,
            70526315789u128, 750000000000u128, 220000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 29090068971u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#408: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0409() {
        // F=100.0, K=95.000000, T=0.5, vol=0.0254425113
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 500000000000u128,
            70526315789u128, 750000000000u128, 220000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 25442511316u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#409: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0410() {
        // F=100.0, K=105.000000, T=10.0, vol=0.0226951281
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 10000000000000u128,
            70526315789u128, 750000000000u128, 360000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 22695128140u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#410: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0411() {
        // F=100.0, K=120.000000, T=1.0, vol=0.0375722468
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 1000000000000u128,
            70526315789u128, 750000000000u128, 360000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 37572246817u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#411: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0412() {
        // F=100.0, K=200.000000, T=0.1, vol=0.1503749172
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 100000000000u128,
            70526315789u128, 750000000000u128, 360000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 150374917229u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#412: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0413() {
        // F=100.0, K=70.000000, T=5.0, vol=0.0229689890
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 5000000000000u128,
            70526315789u128, 750000000000u128, 500000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 22968988990u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#413: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0414() {
        // F=100.0, K=90.000000, T=0.5, vol=0.0237104961
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 500000000000u128,
            70526315789u128, 750000000000u128, 500000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 23710496058u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#414: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0415() {
        // F=100.0, K=100.000000, T=10.0, vol=0.0301098417
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 10000000000000u128,
            70526315789u128, 750000000000u128, 500000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 30109841691u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#415: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0416() {
        // F=100.0, K=110.000000, T=1.0, vol=0.0615401609
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 1000000000000u128,
            70526315789u128, 1000000000000u128, -900000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 61540160937u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#416: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0417() {
        // F=100.0, K=150.000000, T=0.1, vol=0.0616653326
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 100000000000u128,
            70526315789u128, 1000000000000u128, -900000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 61665332586u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#417: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0418() {
        // F=100.0, K=50.000000, T=5.0, vol=0.0835281584
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 5000000000000u128,
            70526315789u128, 1000000000000u128, -760000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 83528158397u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#418: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0419() {
        // F=100.0, K=80.000000, T=0.5, vol=0.0957367018
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 500000000000u128,
            70526315789u128, 1000000000000u128, -760000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 95736701848u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#419: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0420() {
        // F=100.0, K=95.000000, T=10.0, vol=0.0789987002
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 10000000000000u128,
            70526315789u128, 1000000000000u128, -760000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 78998700231u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#420: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0421() {
        // F=100.0, K=105.000000, T=1.0, vol=0.0689882085
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 1000000000000u128,
            70526315789u128, 1000000000000u128, -620000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 68988208514u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#421: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0422() {
        // F=100.0, K=120.000000, T=0.1, vol=0.0592564211
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 100000000000u128,
            70526315789u128, 1000000000000u128, -620000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 59256421095u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#422: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0423() {
        // F=100.0, K=200.000000, T=2.0, vol=0.1563997406
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 2000000000000u128,
            70526315789u128, 1000000000000u128, -620000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 156399740574u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#423: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0424() {
        // F=100.0, K=70.000000, T=0.5, vol=0.0917244557
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 500000000000u128,
            70526315789u128, 1000000000000u128, -480000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 91724455671u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#424: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0425() {
        // F=100.0, K=90.000000, T=10.0, vol=0.0868791951
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 10000000000000u128,
            70526315789u128, 1000000000000u128, -480000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 86879195120u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#425: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0426() {
        // F=100.0, K=100.000000, T=1.0, vol=0.0705173217
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 1000000000000u128,
            70526315789u128, 1000000000000u128, -340000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 70517321674u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#426: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0427() {
        // F=100.0, K=110.000000, T=0.1, vol=0.0675290469
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 100000000000u128,
            70526315789u128, 1000000000000u128, -340000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 67529046881u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#427: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0428() {
        // F=100.0, K=150.000000, T=2.0, vol=0.1119505407
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 2000000000000u128,
            70526315789u128, 1000000000000u128, -340000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 111950540703u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#428: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0429() {
        // F=100.0, K=50.000000, T=0.5, vol=0.0847707223
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 500000000000u128,
            70526315789u128, 1000000000000u128, -200000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 84770722320u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#429: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0430() {
        // F=100.0, K=80.000000, T=10.0, vol=0.0891645521
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 10000000000000u128,
            70526315789u128, 1000000000000u128, -200000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 89164552115u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#430: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0431() {
        // F=100.0, K=95.000000, T=1.0, vol=0.0813418452
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 1000000000000u128,
            70526315789u128, 1000000000000u128, -200000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 81341845242u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#431: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0432() {
        // F=100.0, K=105.000000, T=0.1, vol=0.0704803632
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 100000000000u128,
            70526315789u128, 1000000000000u128, -60000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 70480363176u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#432: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0433() {
        // F=100.0, K=120.000000, T=2.0, vol=0.0809322180
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 2000000000000u128,
            70526315789u128, 1000000000000u128, -60000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 80932218048u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#433: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0434() {
        // F=100.0, K=200.000000, T=0.25, vol=0.0744444672
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 250000000000u128,
            70526315789u128, 1000000000000u128, 80000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 74444467217u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#434: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0435() {
        // F=100.0, K=70.000000, T=10.0, vol=0.0808053706
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 10000000000000u128,
            70526315789u128, 1000000000000u128, 80000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 80805370609u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#435: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0436() {
        // F=100.0, K=90.000000, T=1.0, vol=0.0787203907
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 1000000000000u128,
            70526315789u128, 1000000000000u128, 80000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 78720390732u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#436: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0437() {
        // F=100.0, K=100.000000, T=0.1, vol=0.0705345020
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 100000000000u128,
            70526315789u128, 1000000000000u128, 220000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 70534501977u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#437: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0438() {
        // F=100.0, K=110.000000, T=2.0, vol=0.0764430789
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 2000000000000u128,
            70526315789u128, 1000000000000u128, 220000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 76443078892u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#438: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0439() {
        // F=100.0, K=150.000000, T=0.25, vol=0.1582244990
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 250000000000u128,
            70526315789u128, 1000000000000u128, 220000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 158224498991u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#439: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0440() {
        // F=100.0, K=50.000000, T=10.0, vol=0.0700245719
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 10000000000000u128,
            70526315789u128, 1000000000000u128, 360000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 70024571876u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#440: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0441() {
        // F=100.0, K=80.000000, T=1.0, vol=0.0730928524
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 1000000000000u128,
            70526315789u128, 1000000000000u128, 360000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 73092852388u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#441: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0442() {
        // F=100.0, K=95.000000, T=0.1, vol=0.0698991048
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 100000000000u128,
            70526315789u128, 1000000000000u128, 500000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 69899104766u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#442: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0443() {
        // F=100.0, K=105.000000, T=2.0, vol=0.0736556326
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 2000000000000u128,
            70526315789u128, 1000000000000u128, 500000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 73655632584u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#443: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0444() {
        // F=100.0, K=120.000000, T=0.25, vol=0.1050190201
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 250000000000u128,
            70526315789u128, 1000000000000u128, 500000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 105019020138u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#444: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0445() {
        // F=100.0, K=200.000000, T=5.0, vol=0.0049132132
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 5000000000000u128,
            95789473684u128, 0u128, -900000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 4913213154u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#445: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0446() {
        // F=100.0, K=70.000000, T=1.0, vol=0.0231092200
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 1000000000000u128,
            95789473684u128, 0u128, -900000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 23109219954u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#446: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0447() {
        // F=100.0, K=90.000000, T=0.1, vol=0.0187564609
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 100000000000u128,
            95789473684u128, 0u128, -900000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 18756460942u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#447: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0448() {
        // F=100.0, K=100.000000, T=2.0, vol=0.0009581081
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 2000000000000u128,
            95789473684u128, 0u128, -760000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 958108101u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#448: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0449() {
        // F=100.0, K=110.000000, T=0.25, vol=0.0065358383
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 250000000000u128,
            95789473684u128, 0u128, -760000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 6535838349u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#449: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0450() {
        // F=100.0, K=150.000000, T=5.0, vol=0.0409569558
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 5000000000000u128,
            95789473684u128, 0u128, -760000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 40956955843u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#450: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0451() {
        // F=100.0, K=50.000000, T=1.0, vol=0.0280874724
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 1000000000000u128,
            95789473684u128, 0u128, -620000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 28087472414u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#451: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0452() {
        // F=100.0, K=80.000000, T=0.1, vol=0.0265217024
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 100000000000u128,
            95789473684u128, 0u128, -620000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 26521702429u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#452: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0453() {
        // F=100.0, K=95.000000, T=2.0, vol=0.0017705853
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 2000000000000u128,
            95789473684u128, 0u128, -480000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 1770585348u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#453: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0454() {
        // F=100.0, K=105.000000, T=0.25, vol=0.0036023818
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 250000000000u128,
            95789473684u128, 0u128, -480000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 3602381822u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#454: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0455() {
        // F=100.0, K=120.000000, T=5.0, vol=0.0194357774
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 5000000000000u128,
            95789473684u128, 0u128, -480000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 19435777376u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#455: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0456() {
        // F=100.0, K=200.000000, T=0.5, vol=0.0118546276
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 500000000000u128,
            95789473684u128, 0u128, -340000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 11854627640u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#456: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0457() {
        // F=100.0, K=70.000000, T=0.1, vol=0.0271731517
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 100000000000u128,
            95789473684u128, 0u128, -340000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 27173151732u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#457: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0458() {
        // F=100.0, K=90.000000, T=2.0, vol=0.0189842262
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 2000000000000u128,
            95789473684u128, 0u128, -340000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 18984226190u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#458: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0459() {
        // F=100.0, K=100.000000, T=0.25, vol=0.0009586451
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 250000000000u128,
            95789473684u128, 0u128, -200000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 958645097u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#459: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0460() {
        // F=100.0, K=110.000000, T=5.0, vol=0.0087220739
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 5000000000000u128,
            95789473684u128, 0u128, -200000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 8722073917u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#460: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0461() {
        // F=100.0, K=150.000000, T=0.5, vol=0.0050240716
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 500000000000u128,
            95789473684u128, 0u128, -60000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 5024071576u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#461: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0462() {
        // F=100.0, K=50.000000, T=0.1, vol=0.0359836527
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 100000000000u128,
            95789473684u128, 0u128, -60000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 35983652692u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#462: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0463() {
        // F=100.0, K=80.000000, T=2.0, vol=0.0259126921
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 2000000000000u128,
            95789473684u128, 0u128, -60000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 25912692061u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#463: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0464() {
        // F=100.0, K=95.000000, T=0.25, vol=0.0021173977
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 250000000000u128,
            95789473684u128, 0u128, 80000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 2117397741u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#464: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0465() {
        // F=100.0, K=105.000000, T=5.0, vol=0.0044998611
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 5000000000000u128,
            95789473684u128, 0u128, 80000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 4499861058u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#465: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0466() {
        // F=100.0, K=120.000000, T=0.5, vol=0.0260753072
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 500000000000u128,
            95789473684u128, 0u128, 80000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 26075307239u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#466: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0467() {
        // F=100.0, K=200.000000, T=10.0, vol=0.0133610296
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 10000000000000u128,
            95789473684u128, 0u128, 220000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 13361029629u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#467: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0468() {
        // F=100.0, K=70.000000, T=2.0, vol=0.0252254016
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 2000000000000u128,
            95789473684u128, 0u128, 220000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 25225401554u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#468: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0469() {
        // F=100.0, K=90.000000, T=0.25, vol=0.0019295552
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 250000000000u128,
            95789473684u128, 0u128, 360000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 1929555204u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#469: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0470() {
        // F=100.0, K=100.000000, T=5.0, vol=0.0009707563
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 5000000000000u128,
            95789473684u128, 0u128, 360000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 970756253u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#470: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0471() {
        // F=100.0, K=110.000000, T=0.5, vol=0.0127759942
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 500000000000u128,
            95789473684u128, 0u128, 360000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 12775994153u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#471: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0472() {
        // F=100.0, K=150.000000, T=10.0, vol=0.0056599248
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 10000000000000u128,
            95789473684u128, 0u128, 500000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 5659924842u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#472: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0473() {
        // F=100.0, K=50.000000, T=2.0, vol=0.0320645095
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 2000000000000u128,
            95789473684u128, 0u128, 500000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 32064509511u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#473: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0474() {
        // F=100.0, K=80.000000, T=0.25, vol=0.0276378064
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 250000000000u128,
            95789473684u128, 0u128, 500000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 27637806387u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#474: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0475() {
        // F=100.0, K=95.000000, T=5.0, vol=0.0051297000
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 5000000000000u128,
            95789473684u128, 250000000000u128, -900000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 5129699952u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#475: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0476() {
        // F=100.0, K=105.000000, T=0.5, vol=0.0041189129
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 500000000000u128,
            95789473684u128, 250000000000u128, -900000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 4118912943u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#476: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0477() {
        // F=100.0, K=120.000000, T=10.0, vol=0.0186171057
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 10000000000000u128,
            95789473684u128, 250000000000u128, -900000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 18617105664u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#477: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0478() {
        // F=100.0, K=200.000000, T=1.0, vol=0.0221517694
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 1000000000000u128,
            95789473684u128, 250000000000u128, -760000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 22151769449u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#478: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0479() {
        // F=100.0, K=70.000000, T=0.25, vol=0.0500877319
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 250000000000u128,
            95789473684u128, 250000000000u128, -760000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 50087731859u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#479: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0480() {
        // F=100.0, K=90.000000, T=5.0, vol=0.0049327609
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 5000000000000u128,
            95789473684u128, 250000000000u128, -620000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 4932760886u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#480: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0481() {
        // F=100.0, K=100.000000, T=0.5, vol=0.0030338856
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 500000000000u128,
            95789473684u128, 250000000000u128, -620000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 3033885619u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#481: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0482() {
        // F=100.0, K=110.000000, T=10.0, vol=0.0139911202
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 10000000000000u128,
            95789473684u128, 250000000000u128, -620000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 13991120230u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#482: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0483() {
        // F=100.0, K=150.000000, T=1.0, vol=0.0099468045
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 1000000000000u128,
            95789473684u128, 250000000000u128, -480000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 9946804480u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#483: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0484() {
        // F=100.0, K=50.000000, T=0.25, vol=0.0601919121
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 250000000000u128,
            95789473684u128, 250000000000u128, -480000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 60191912067u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#484: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0485() {
        // F=100.0, K=80.000000, T=5.0, vol=0.0486559481
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 5000000000000u128,
            95789473684u128, 250000000000u128, -480000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 48655948058u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#485: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0486() {
        // F=100.0, K=95.000000, T=0.5, vol=0.0059908839
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 500000000000u128,
            95789473684u128, 250000000000u128, -340000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 5990883925u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#486: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0487() {
        // F=100.0, K=105.000000, T=10.0, vol=0.0073616940
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 10000000000000u128,
            95789473684u128, 250000000000u128, -340000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 7361693965u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#487: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0488() {
        // F=100.0, K=120.000000, T=1.0, vol=0.0044493796
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 1000000000000u128,
            95789473684u128, 250000000000u128, -200000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 4449379643u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#488: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0489() {
        // F=100.0, K=200.000000, T=0.1, vol=0.0380836250
        let vol = sabr_implied_vol(
            100000000000000u128, 200000000000000u128, 100000000000u128,
            95789473684u128, 250000000000u128, -200000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 38083625047u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#489: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0490() {
        // F=100.0, K=70.000000, T=5.0, vol=0.0524840463
        let vol = sabr_implied_vol(
            100000000000000u128, 70000000000000u128, 5000000000000u128,
            95789473684u128, 250000000000u128, -200000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 52484046262u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#490: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0491() {
        // F=100.0, K=90.000000, T=0.5, vol=0.0056035827
        let vol = sabr_implied_vol(
            100000000000000u128, 90000000000000u128, 500000000000u128,
            95789473684u128, 250000000000u128, -60000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 5603582743u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#491: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0492() {
        // F=100.0, K=100.000000, T=10.0, vol=0.0032549903
        let vol = sabr_implied_vol(
            100000000000000u128, 100000000000000u128, 10000000000000u128,
            95789473684u128, 250000000000u128, -60000000000i128, 300000000000u128,
        ).unwrap();
        let expected = 3254990303u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#492: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0493() {
        // F=100.0, K=110.000000, T=1.0, vol=0.0199960324
        let vol = sabr_implied_vol(
            100000000000000u128, 110000000000000u128, 1000000000000u128,
            95789473684u128, 250000000000u128, -60000000000i128, 800000000000u128,
        ).unwrap();
        let expected = 19996032434u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#493: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0494() {
        // F=100.0, K=150.000000, T=0.1, vol=0.0199114389
        let vol = sabr_implied_vol(
            100000000000000u128, 150000000000000u128, 100000000000u128,
            95789473684u128, 250000000000u128, 80000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 19911438938u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#494: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0495() {
        // F=100.0, K=50.000000, T=5.0, vol=0.0580809512
        let vol = sabr_implied_vol(
            100000000000000u128, 50000000000000u128, 5000000000000u128,
            95789473684u128, 250000000000u128, 80000000000i128, 400000000000u128,
        ).unwrap();
        let expected = 58080951157u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#495: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0496() {
        // F=100.0, K=80.000000, T=0.5, vol=0.0052634067
        let vol = sabr_implied_vol(
            100000000000000u128, 80000000000000u128, 500000000000u128,
            95789473684u128, 250000000000u128, 220000000000i128, 50000000000u128,
        ).unwrap();
        let expected = 5263406718u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#496: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0497() {
        // F=100.0, K=95.000000, T=10.0, vol=0.0050420740
        let vol = sabr_implied_vol(
            100000000000000u128, 95000000000000u128, 10000000000000u128,
            95789473684u128, 250000000000u128, 220000000000i128, 200000000000u128,
        ).unwrap();
        let expected = 5042073970u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#497: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0498() {
        // F=100.0, K=105.000000, T=1.0, vol=0.0107246329
        let vol = sabr_implied_vol(
            100000000000000u128, 105000000000000u128, 1000000000000u128,
            95789473684u128, 250000000000u128, 220000000000i128, 600000000000u128,
        ).unwrap();
        let expected = 10724632950u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#498: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

    #[test]
    fn ql_sabr_0499() {
        // F=100.0, K=120.000000, T=0.1, vol=0.0078940502
        let vol = sabr_implied_vol(
            100000000000000u128, 120000000000000u128, 100000000000u128,
            95789473684u128, 250000000000u128, 360000000000i128, 100000000000u128,
        ).unwrap();
        let expected = 7894050172u128;
        let tol = expected / 200; // 0.5%
        let diff = if vol > expected { vol - expected } else { expected - vol };
        assert!(diff <= tol,
            "SABR#499: vol={} exp={} diff={} tol={}", vol, expected, diff, tol);
    }

}
