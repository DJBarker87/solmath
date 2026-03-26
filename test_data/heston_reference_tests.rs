// Auto-generated from QuantLib. Do not edit.
// 500 of 100000 vectors (every 200th)

#[cfg(test)]
mod quantlib_heston {
    use crate::heston::heston_price;

    #[test]
    fn ql_heston_0000() {
        // S=100.0, K=80.0, T=0.1, r=0.0
        // v0=0.01, kappa=0.5, theta=0.01, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 500000000000u128, 10000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 20000000072912u128;
        let exp_put = 72912u128;
        let tol = 325000000000u128; // $0.33
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#0 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#0 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0001() {
        // S=100.0, K=90.0, T=0.5, r=0.0
        // v0=0.01, kappa=0.5, theta=0.01, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 500000000000u128, 10000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 10217470847310u128;
        let exp_put = 217470847310u128;
        let tol = 175000000000u128; // $0.17
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#1 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#1 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0002() {
        // S=100.0, K=105.0, T=2.0, r=0.0
        // v0=0.01, kappa=0.5, theta=0.01, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 500000000000u128, 10000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 2746636557217u128;
        let exp_put = 7746636557217u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#2 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#2 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0003() {
        // S=100.0, K=120.0, T=0.25, r=0.0
        // v0=0.01, kappa=0.5, theta=0.01, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 500000000000u128, 10000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 1646919993u128;
        let exp_put = 20001646919993u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#3 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#3 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0004() {
        // S=100.0, K=90.0, T=2.0, r=0.0
        // v0=0.01, kappa=0.5, theta=0.01, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 500000000000u128, 10000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 11547100966760u128;
        let exp_put = 1547100966760u128;
        let tol = 283333333333u128; // $0.28
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#4 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#4 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0005() {
        // S=100.0, K=115.0, T=0.25, r=0.0
        // v0=0.01, kappa=0.5, theta=0.01, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 500000000000u128, 10000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 21988666967u128;
        let exp_put = 15021988666967u128;
        let tol = 589130434783u128; // $0.59
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#5 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#5 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0006() {
        // S=100.0, K=115.0, T=2.0, r=0.0
        // v0=0.01, kappa=0.5, theta=0.04, xi=0.1, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 500000000000u128, 40000000000u128, 100000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 2544193485949u128;
        let exp_put = 17544193485949u128;
        let tol = 89130434783u128; // $0.09
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#6 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#6 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0007() {
        // S=100.0, K=90.0, T=0.5, r=0.0
        // v0=0.01, kappa=0.5, theta=0.04, xi=0.2, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 500000000000u128, 40000000000u128, 200000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 10627224257469u128;
        let exp_put = 627224257469u128;
        let tol = 83333333333u128; // $0.08
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#7 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#7 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0008() {
        // S=100.0, K=115.0, T=2.0, r=0.0
        // v0=0.01, kappa=0.5, theta=0.04, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 500000000000u128, 40000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 602174638364u128;
        let exp_put = 15602174638364u128;
        let tol = 289130434783u128; // $0.29
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#8 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#8 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0009() {
        // S=100.0, K=90.0, T=0.5, r=0.0
        // v0=0.01, kappa=0.5, theta=0.04, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 500000000000u128, 40000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 10852386149813u128;
        let exp_put = 852386149813u128;
        let tol = 483333333333u128; // $0.48
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#9 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#9 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0010() {
        // S=100.0, K=115.0, T=2.0, r=0.0
        // v0=0.01, kappa=0.5, theta=0.04, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 500000000000u128, 40000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 2595429472756u128;
        let exp_put = 17595429472756u128;
        let tol = 289130434783u128; // $0.29
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#10 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#10 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0011() {
        // S=100.0, K=110.0, T=0.5, r=0.0
        // v0=0.01, kappa=0.5, theta=0.04, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 500000000000u128, 40000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 587757090564u128;
        let exp_put = 10587757090564u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#11 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#11 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0012() {
        // S=100.0, K=80.0, T=0.1, r=0.0
        // v0=0.01, kappa=0.5, theta=0.09, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 500000000000u128, 90000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 20000000000659u128;
        let exp_put = 659u128;
        let tol = 415000000000u128; // $0.41
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#12 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#12 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0013() {
        // S=100.0, K=90.0, T=0.5, r=0.0
        // v0=0.01, kappa=0.5, theta=0.09, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 500000000000u128, 90000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 10784941945958u128;
        let exp_put = 784941945958u128;
        let tol = 83333333333u128; // $0.08
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#13 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#13 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0014() {
        // S=100.0, K=100.0, T=2.0, r=0.0
        // v0=0.01, kappa=0.5, theta=0.09, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 500000000000u128, 90000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 10236940976666u128;
        let exp_put = 10236940976666u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#14 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#14 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0015() {
        // S=100.0, K=110.0, T=0.25, r=0.0
        // v0=0.01, kappa=0.5, theta=0.09, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 500000000000u128, 90000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 81987040677u128;
        let exp_put = 10081987040677u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#15 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#15 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0016() {
        // S=100.0, K=120.0, T=1.0, r=0.0
        // v0=0.01, kappa=0.5, theta=0.09, xi=0.8, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 1000000000000u128,
            10000000000u128, 500000000000u128, 90000000000u128, 800000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 233690466201u128;
        let exp_put = 20233690466201u128;
        let tol = 600000000000u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#16 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#16 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0017() {
        // S=100.0, K=110.0, T=0.25, r=0.0
        // v0=0.01, kappa=0.5, theta=0.16, xi=0.1, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 500000000000u128, 160000000000u128, 100000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 194453665794u128;
        let exp_put = 10194453665794u128;
        let tol = 625000000000u128; // $0.62
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#17 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#17 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0018() {
        // S=100.0, K=80.0, T=2.0, r=0.0
        // v0=0.01, kappa=0.5, theta=0.16, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 500000000000u128, 160000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 25673362533135u128;
        let exp_put = 5673362533135u128;
        let tol = 325000000000u128; // $0.33
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#18 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#18 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0019() {
        // S=100.0, K=95.0, T=0.25, r=0.0
        // v0=0.01, kappa=0.5, theta=0.16, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 500000000000u128, 160000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 6089501970565u128;
        let exp_put = 1089501970565u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#19 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#19 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0020() {
        // S=100.0, K=110.0, T=1.0, r=0.0
        // v0=0.01, kappa=0.5, theta=0.16, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 1000000000000u128,
            10000000000u128, 500000000000u128, 160000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 4249652725013u128;
        let exp_put = 14249652725013u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#20 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#20 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0021() {
        // S=100.0, K=80.0, T=0.25, r=0.0
        // v0=0.01, kappa=0.5, theta=0.16, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 500000000000u128, 160000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 20017446619429u128;
        let exp_put = 17446619429u128;
        let tol = 325000000000u128; // $0.33
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#21 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#21 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0022() {
        // S=100.0, K=90.0, T=1.0, r=0.0
        // v0=0.01, kappa=0.5, theta=0.16, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 1000000000000u128,
            10000000000u128, 500000000000u128, 160000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 13251120983244u128;
        let exp_put = 3251120983244u128;
        let tol = 583333333333u128; // $0.58
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#22 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#22 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0023() {
        // S=100.0, K=115.0, T=0.1, r=0.0
        // v0=0.01, kappa=1.0, theta=0.01, xi=0.1, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 1000000000000u128, 10000000000u128, 100000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 1427895u128;
        let exp_put = 15000001427895u128;
        let tol = 289130434783u128; // $0.29
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#23 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#23 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0024() {
        // S=100.0, K=85.0, T=1.0, r=0.0
        // v0=0.01, kappa=1.0, theta=0.01, xi=0.2, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 1000000000000u128,
            10000000000u128, 1000000000000u128, 10000000000u128, 200000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 15492129403718u128;
        let exp_put = 492129403718u128;
        let tol = 102941176471u128; // $0.10
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#24 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#24 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0025() {
        // S=100.0, K=100.0, T=0.1, r=0.0
        // v0=0.01, kappa=1.0, theta=0.01, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 1000000000000u128, 10000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 1209105026582u128;
        let exp_put = 1209105026582u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#25 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#25 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0026() {
        // S=100.0, K=110.0, T=0.5, r=0.0
        // v0=0.01, kappa=1.0, theta=0.01, xi=0.5, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 1000000000000u128, 10000000000u128, 500000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 62458342955u128;
        let exp_put = 10062458342955u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#26 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#26 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0027() {
        // S=100.0, K=100.0, T=0.1, r=0.0
        // v0=0.01, kappa=1.0, theta=0.01, xi=0.8, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 1000000000000u128, 10000000000u128, 800000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 986502422738u128;
        let exp_put = 986502422738u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#27 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#27 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0028() {
        // S=100.0, K=105.0, T=1.0, r=0.0
        // v0=0.01, kappa=1.0, theta=0.04, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 1000000000000u128,
            10000000000u128, 1000000000000u128, 40000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 3475809079502u128;
        let exp_put = 8475809079502u128;
        let tol = 340000000000u128; // $0.34
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#28 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#28 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0029() {
        // S=100.0, K=115.0, T=0.1, r=0.0
        // v0=0.01, kappa=1.0, theta=0.04, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 1000000000000u128, 40000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 0u128;
        let exp_put = 15000000000000u128;
        let tol = 340000000000u128; // $0.34
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#29 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#29 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0030() {
        // S=100.0, K=85.0, T=1.0, r=0.0
        // v0=0.01, kappa=1.0, theta=0.04, xi=0.2, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 1000000000000u128,
            10000000000u128, 1000000000000u128, 40000000000u128, 200000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 15901067620488u128;
        let exp_put = 901067620488u128;
        let tol = 102941176471u128; // $0.10
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#30 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#30 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0031() {
        // S=100.0, K=100.0, T=0.1, r=0.0
        // v0=0.01, kappa=1.0, theta=0.04, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 1000000000000u128, 40000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 1302253906083u128;
        let exp_put = 1302253906083u128;
        let tol = 340000000000u128; // $0.34
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#31 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#31 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0032() {
        // S=100.0, K=115.0, T=0.5, r=0.0
        // v0=0.01, kappa=1.0, theta=0.04, xi=0.5, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 1000000000000u128, 40000000000u128, 500000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 232823102847u128;
        let exp_put = 15232823102847u128;
        let tol = 289130434783u128; // $0.29
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#32 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#32 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0033() {
        // S=100.0, K=80.0, T=0.1, r=0.0
        // v0=0.01, kappa=1.0, theta=0.04, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 1000000000000u128, 40000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 20001029474924u128;
        let exp_put = 1029474924u128;
        let tol = 625000000000u128; // $0.62
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#33 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#33 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0034() {
        // S=100.0, K=110.0, T=0.5, r=0.0
        // v0=0.01, kappa=1.0, theta=0.09, xi=0.1, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 1000000000000u128, 90000000000u128, 100000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 1285534701322u128;
        let exp_put = 11285534701322u128;
        let tol = 490000000000u128; // $0.49
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#34 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#34 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0035() {
        // S=100.0, K=85.0, T=0.1, r=0.0
        // v0=0.01, kappa=1.0, theta=0.09, xi=0.2, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 1000000000000u128, 90000000000u128, 200000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 15000258648164u128;
        let exp_put = 258648164u128;
        let tol = 490000000000u128; // $0.49
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#35 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#35 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0036() {
        // S=100.0, K=100.0, T=0.5, r=0.0
        // v0=0.01, kappa=1.0, theta=0.09, xi=0.3, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 1000000000000u128, 90000000000u128, 300000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 4423794463564u128;
        let exp_put = 4423794463564u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#36 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#36 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0037() {
        // S=100.0, K=115.0, T=2.0, r=0.0
        // v0=0.01, kappa=1.0, theta=0.09, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 1000000000000u128, 90000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 4517136176190u128;
        let exp_put = 19517136176190u128;
        let tol = 489130434783u128; // $0.49
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#37 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#37 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0038() {
        // S=100.0, K=85.0, T=0.5, r=0.0
        // v0=0.01, kappa=1.0, theta=0.09, xi=0.8, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 1000000000000u128, 90000000000u128, 800000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 16020744950161u128;
        let exp_put = 1020744950161u128;
        let tol = 802941176471u128; // $0.80
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#38 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#38 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0039() {
        // S=100.0, K=100.0, T=2.0, r=0.0
        // v0=0.01, kappa=1.0, theta=0.09, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 1000000000000u128, 90000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 11413611504566u128;
        let exp_put = 11413611504566u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#39 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#39 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0040() {
        // S=100.0, K=115.0, T=0.25, r=0.0
        // v0=0.01, kappa=1.0, theta=0.16, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 1000000000000u128, 160000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 166537936981u128;
        let exp_put = 15166537936981u128;
        let tol = 700000000000u128; // $0.70
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#40 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#40 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0041() {
        // S=100.0, K=80.0, T=2.0, r=0.0
        // v0=0.01, kappa=1.0, theta=0.16, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 1000000000000u128, 160000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 27531780213383u128;
        let exp_put = 7531780213383u128;
        let tol = 125000000000u128; // $0.12
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#41 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#41 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0042() {
        // S=100.0, K=90.0, T=0.25, r=0.0
        // v0=0.01, kappa=1.0, theta=0.16, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 1000000000000u128, 160000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 10463897732052u128;
        let exp_put = 463897732052u128;
        let tol = 283333333333u128; // $0.28
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#42 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#42 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0043() {
        // S=100.0, K=100.0, T=1.0, r=0.0
        // v0=0.01, kappa=1.0, theta=0.16, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 1000000000000u128,
            10000000000u128, 1000000000000u128, 160000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 9463217277342u128;
        let exp_put = 9463217277342u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#43 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#43 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0044() {
        // S=100.0, K=110.0, T=0.1, r=0.0
        // v0=0.01, kappa=1.0, theta=0.16, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 1000000000000u128, 160000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 10306704805u128;
        let exp_put = 10010306704805u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#44 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#44 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0045() {
        // S=100.0, K=120.0, T=0.5, r=0.0
        // v0=0.01, kappa=2.0, theta=0.01, xi=0.1, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 2000000000000u128, 10000000000u128, 100000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 787353719u128;
        let exp_put = 20000787353719u128;
        let tol = 400000000000u128; // $0.40
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#45 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#45 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0046() {
        // S=100.0, K=95.0, T=0.1, r=0.0
        // v0=0.01, kappa=2.0, theta=0.01, xi=0.2, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 2000000000000u128, 10000000000u128, 200000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 5121388057155u128;
        let exp_put = 121388057155u128;
        let tol = 400000000000u128; // $0.40
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#46 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#46 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0047() {
        // S=100.0, K=110.0, T=0.5, r=0.0
        // v0=0.01, kappa=2.0, theta=0.01, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 2000000000000u128, 10000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 2782490942u128;
        let exp_put = 10002782490942u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#47 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#47 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0048() {
        // S=100.0, K=90.0, T=0.1, r=0.0
        // v0=0.01, kappa=2.0, theta=0.01, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 2000000000000u128, 10000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 10036677482495u128;
        let exp_put = 36677482495u128;
        let tol = 483333333333u128; // $0.48
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#48 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#48 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0049() {
        // S=100.0, K=110.0, T=0.5, r=0.0
        // v0=0.01, kappa=2.0, theta=0.01, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 2000000000000u128, 10000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 406712758634u128;
        let exp_put = 10406712758634u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#49 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#49 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0050() {
        // S=100.0, K=120.0, T=2.0, r=0.0
        // v0=0.01, kappa=2.0, theta=0.01, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 2000000000000u128, 10000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 526866458396u128;
        let exp_put = 20526866458396u128;
        let tol = 600000000000u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#50 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#50 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0051() {
        // S=100.0, K=115.0, T=0.5, r=0.0
        // v0=0.01, kappa=2.0, theta=0.04, xi=0.1, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 2000000000000u128, 40000000000u128, 100000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 382701794457u128;
        let exp_put = 15382701794457u128;
        let tol = 490000000000u128; // $0.49
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#51 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#51 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0052() {
        // S=100.0, K=90.0, T=0.1, r=0.0
        // v0=0.01, kappa=2.0, theta=0.04, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 2000000000000u128, 40000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 10005400008218u128;
        let exp_put = 5400008218u128;
        let tol = 490000000000u128; // $0.49
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#52 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#52 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0053() {
        // S=100.0, K=100.0, T=0.5, r=0.0
        // v0=0.01, kappa=2.0, theta=0.04, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 2000000000000u128, 40000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 3914108254975u128;
        let exp_put = 3914108254975u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#53 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#53 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0054() {
        // S=100.0, K=120.0, T=2.0, r=0.0
        // v0=0.01, kappa=2.0, theta=0.04, xi=0.5, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 2000000000000u128, 40000000000u128, 500000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 1983540124941u128;
        let exp_put = 21983540124941u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#54 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#54 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0055() {
        // S=100.0, K=85.0, T=0.5, r=0.0
        // v0=0.01, kappa=2.0, theta=0.04, xi=0.8, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 2000000000000u128, 40000000000u128, 800000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 15707303988532u128;
        let exp_put = 707303988532u128;
        let tol = 602941176471u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#55 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#55 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0056() {
        // S=100.0, K=105.0, T=2.0, r=0.0
        // v0=0.01, kappa=2.0, theta=0.09, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 2000000000000u128, 90000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 12650817261573u128;
        let exp_put = 17650817261573u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#56 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#56 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0057() {
        // S=100.0, K=120.0, T=0.25, r=0.0
        // v0=0.01, kappa=2.0, theta=0.09, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 2000000000000u128, 90000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 2706194406u128;
        let exp_put = 20002706194406u128;
        let tol = 640000000000u128; // $0.64
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#57 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#57 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0058() {
        // S=100.0, K=85.0, T=2.0, r=0.0
        // v0=0.01, kappa=2.0, theta=0.09, xi=0.2, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 2000000000000u128, 90000000000u128, 200000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 22464587393111u128;
        let exp_put = 7464587393111u128;
        let tol = 102941176471u128; // $0.10
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#58 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#58 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0059() {
        // S=100.0, K=100.0, T=0.25, r=0.0
        // v0=0.01, kappa=2.0, theta=0.09, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 2000000000000u128, 90000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 3216803195276u128;
        let exp_put = 3216803195276u128;
        let tol = 80000000000u128; // $0.08
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#59 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#59 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0060() {
        // S=100.0, K=110.0, T=1.0, r=0.0
        // v0=0.01, kappa=2.0, theta=0.09, xi=0.5, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 1000000000000u128,
            10000000000u128, 2000000000000u128, 90000000000u128, 500000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 4989370518224u128;
        let exp_put = 14989370518224u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#60 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#60 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0061() {
        // S=100.0, K=120.0, T=0.1, r=0.0
        // v0=0.01, kappa=2.0, theta=0.09, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 2000000000000u128, 90000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 487702539u128;
        let exp_put = 20000487702539u128;
        let tol = 600000000000u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#61 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#61 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0062() {
        // S=100.0, K=85.0, T=1.0, r=0.0
        // v0=0.01, kappa=2.0, theta=0.16, xi=0.1, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 1000000000000u128,
            10000000000u128, 2000000000000u128, 160000000000u128, 100000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 20422508690872u128;
        let exp_put = 5422508690872u128;
        let tol = 850000000000u128; // $0.85
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#62 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#62 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0063() {
        // S=100.0, K=100.0, T=0.1, r=0.0
        // v0=0.01, kappa=2.0, theta=0.16, xi=0.2, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 2000000000000u128, 160000000000u128, 200000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 1925325107266u128;
        let exp_put = 1925325107266u128;
        let tol = 850000000000u128; // $0.85
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#63 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#63 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0064() {
        // S=100.0, K=115.0, T=0.5, r=0.0
        // v0=0.01, kappa=2.0, theta=0.16, xi=0.3, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 2000000000000u128, 160000000000u128, 300000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 1952135092022u128;
        let exp_put = 16952135092022u128;
        let tol = 150000000000u128; // $0.15
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#64 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#64 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0065() {
        // S=100.0, K=85.0, T=0.1, r=0.0
        // v0=0.01, kappa=2.0, theta=0.16, xi=0.5, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 2000000000000u128, 160000000000u128, 500000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 15014699509377u128;
        let exp_put = 14699509377u128;
        let tol = 302941176471u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#65 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#65 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0066() {
        // S=100.0, K=95.0, T=0.5, r=0.0
        // v0=0.01, kappa=2.0, theta=0.16, xi=0.8, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 2000000000000u128, 160000000000u128, 800000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 9650035367743u128;
        let exp_put = 4650035367743u128;
        let tol = 750000000000u128; // $0.75
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#66 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#66 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0067() {
        // S=100.0, K=105.0, T=2.0, r=0.0
        // v0=0.01, kappa=2.0, theta=0.16, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 2000000000000u128, 160000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 16879042608144u128;
        let exp_put = 21879042608144u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#67 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#67 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0068() {
        // S=100.0, K=80.0, T=0.5, r=0.0
        // v0=0.01, kappa=3.0, theta=0.01, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 3000000000000u128, 10000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 20002643416593u128;
        let exp_put = 2643416593u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#68 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#68 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0069() {
        // S=100.0, K=105.0, T=2.0, r=0.0
        // v0=0.01, kappa=3.0, theta=0.01, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 3000000000000u128, 10000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 3392159841233u128;
        let exp_put = 8392159841233u128;
        let tol = 180000000000u128; // $0.18
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#69 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#69 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0070() {
        // S=100.0, K=120.0, T=0.25, r=0.0
        // v0=0.01, kappa=3.0, theta=0.01, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 3000000000000u128, 10000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 848344291u128;
        let exp_put = 20000848344291u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#70 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#70 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0071() {
        // S=100.0, K=90.0, T=2.0, r=0.0
        // v0=0.01, kappa=3.0, theta=0.01, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 3000000000000u128, 10000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 11964792817183u128;
        let exp_put = 1964792817183u128;
        let tol = 283333333333u128; // $0.28
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#71 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#71 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0072() {
        // S=100.0, K=105.0, T=0.25, r=0.0
        // v0=0.01, kappa=3.0, theta=0.01, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 3000000000000u128, 10000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 201817011081u128;
        let exp_put = 5201817011081u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#72 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#72 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0073() {
        // S=100.0, K=100.0, T=2.0, r=0.0
        // v0=0.01, kappa=3.0, theta=0.04, xi=0.1, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 3000000000000u128, 40000000000u128, 100000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 10456648183536u128;
        let exp_put = 10456648183536u128;
        let tol = 180000000000u128; // $0.18
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#73 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#73 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0074() {
        // S=100.0, K=115.0, T=0.25, r=0.0
        // v0=0.01, kappa=3.0, theta=0.04, xi=0.2, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 3000000000000u128, 40000000000u128, 200000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 12893351783u128;
        let exp_put = 15012893351783u128;
        let tol = 640000000000u128; // $0.64
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#74 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#74 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0075() {
        // S=100.0, K=80.0, T=2.0, r=0.0
        // v0=0.01, kappa=3.0, theta=0.04, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 3000000000000u128, 40000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 23096672552674u128;
        let exp_put = 3096672552674u128;
        let tol = 325000000000u128; // $0.33
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#75 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#75 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0076() {
        // S=100.0, K=95.0, T=0.25, r=0.0
        // v0=0.01, kappa=3.0, theta=0.04, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 3000000000000u128, 40000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 6115859835684u128;
        let exp_put = 1115859835684u128;
        let tol = 450000000000u128; // $0.45
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#76 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#76 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0077() {
        // S=100.0, K=105.0, T=1.0, r=0.0
        // v0=0.01, kappa=3.0, theta=0.04, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 1000000000000u128,
            10000000000u128, 3000000000000u128, 40000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 4624329075782u128;
        let exp_put = 9624329075782u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#77 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#77 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0078() {
        // S=100.0, K=115.0, T=0.1, r=0.0
        // v0=0.01, kappa=3.0, theta=0.04, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 3000000000000u128, 40000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 7576847911u128;
        let exp_put = 15007576847911u128;
        let tol = 589130434783u128; // $0.59
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#78 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#78 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0079() {
        // S=100.0, K=80.0, T=1.0, r=0.0
        // v0=0.01, kappa=3.0, theta=0.09, xi=0.1, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 1000000000000u128,
            10000000000u128, 3000000000000u128, 90000000000u128, 100000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 22421589134092u128;
        let exp_put = 2421589134092u128;
        let tol = 790000000000u128; // $0.79
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#79 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#79 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0080() {
        // S=100.0, K=90.0, T=0.1, r=0.0
        // v0=0.01, kappa=3.0, theta=0.09, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 3000000000000u128, 90000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 10023795153900u128;
        let exp_put = 23795153900u128;
        let tol = 790000000000u128; // $0.79
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#80 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#80 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0081() {
        // S=100.0, K=105.0, T=0.5, r=0.0
        // v0=0.01, kappa=3.0, theta=0.09, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 3000000000000u128, 90000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 3944676072766u128;
        let exp_put = 8944676072766u128;
        let tol = 180000000000u128; // $0.18
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#81 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#81 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0082() {
        // S=100.0, K=115.0, T=2.0, r=0.0
        // v0=0.01, kappa=3.0, theta=0.09, xi=0.5, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 3000000000000u128, 90000000000u128, 500000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 8948961913866u128;
        let exp_put = 23948961913866u128;
        let tol = 289130434783u128; // $0.29
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#82 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#82 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0083() {
        // S=100.0, K=80.0, T=0.5, r=0.0
        // v0=0.01, kappa=3.0, theta=0.09, xi=0.8, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 3000000000000u128, 90000000000u128, 800000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 20994648170951u128;
        let exp_put = 994648170951u128;
        let tol = 625000000000u128; // $0.62
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#83 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#83 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0084() {
        // S=100.0, K=90.0, T=2.0, r=0.0
        // v0=0.01, kappa=3.0, theta=0.16, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 3000000000000u128, 160000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 24832398358106u128;
        let exp_put = 14832398358106u128;
        let tol = 283333333333u128; // $0.28
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#84 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#84 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0085() {
        // S=100.0, K=100.0, T=0.25, r=0.0
        // v0=0.01, kappa=3.0, theta=0.16, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 3000000000000u128, 160000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 4610957660322u128;
        let exp_put = 4610957660322u128;
        let tol = 1000000000000u128; // $1.00
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#85 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#85 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0086() {
        // S=100.0, K=110.0, T=1.0, r=0.0
        // v0=0.01, kappa=3.0, theta=0.16, xi=0.2, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 1000000000000u128,
            10000000000u128, 3000000000000u128, 160000000000u128, 200000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 9511262193491u128;
        let exp_put = 19511262193491u128;
        let tol = 225000000000u128; // $0.22
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#86 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#86 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0087() {
        // S=100.0, K=120.0, T=0.1, r=0.0
        // v0=0.01, kappa=3.0, theta=0.16, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 3000000000000u128, 160000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 1337916344u128;
        let exp_put = 20001337916344u128;
        let tol = 1000000000000u128; // $1.00
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#87 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#87 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0088() {
        // S=100.0, K=85.0, T=1.0, r=0.0
        // v0=0.01, kappa=3.0, theta=0.16, xi=0.5, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 1000000000000u128,
            10000000000u128, 3000000000000u128, 160000000000u128, 500000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 21249231211586u128;
        let exp_put = 6249231211586u128;
        let tol = 302941176471u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#88 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#88 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0089() {
        // S=100.0, K=95.0, T=0.1, r=0.0
        // v0=0.01, kappa=3.0, theta=0.16, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 3000000000000u128, 160000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 5568924944820u128;
        let exp_put = 568924944820u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#89 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#89 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0090() {
        // S=100.0, K=105.0, T=0.5, r=0.0
        // v0=0.01, kappa=5.0, theta=0.01, xi=0.1, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 5000000000000u128, 10000000000u128, 100000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 967845746365u128;
        let exp_put = 5967845746365u128;
        let tol = 850000000000u128; // $0.85
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#90 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#90 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0091() {
        // S=100.0, K=120.0, T=2.0, r=0.0
        // v0=0.01, kappa=5.0, theta=0.01, xi=0.2, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 5000000000000u128, 10000000000u128, 200000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 417888596221u128;
        let exp_put = 20417888596221u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#91 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#91 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0092() {
        // S=100.0, K=85.0, T=0.5, r=0.0
        // v0=0.01, kappa=5.0, theta=0.01, xi=0.3, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 5000000000000u128, 10000000000u128, 300000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 15145763458158u128;
        let exp_put = 145763458158u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#92 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#92 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0093() {
        // S=100.0, K=95.0, T=2.0, r=0.0
        // v0=0.01, kappa=5.0, theta=0.01, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 5000000000000u128, 10000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 8359545875700u128;
        let exp_put = 3359545875700u128;
        let tol = 450000000000u128; // $0.45
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#93 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#93 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0094() {
        // S=100.0, K=110.0, T=0.25, r=0.0
        // v0=0.01, kappa=5.0, theta=0.01, xi=0.8, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 5000000000000u128, 10000000000u128, 800000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 57619412u128;
        let exp_put = 10000057619412u128;
        let tol = 750000000000u128; // $0.75
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#94 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#94 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0095() {
        // S=100.0, K=95.0, T=2.0, r=0.0
        // v0=0.01, kappa=5.0, theta=0.01, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 5000000000000u128, 10000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 7954132723156u128;
        let exp_put = 2954132723156u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#95 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#95 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0096() {
        // S=100.0, K=110.0, T=0.25, r=0.0
        // v0=0.01, kappa=5.0, theta=0.04, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 5000000000000u128, 40000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 391131231311u128;
        let exp_put = 10391131231311u128;
        let tol = 940000000000u128; // $0.94
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#96 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#96 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0097() {
        // S=100.0, K=80.0, T=2.0, r=0.0
        // v0=0.01, kappa=5.0, theta=0.04, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 5000000000000u128, 40000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 22889397646742u128;
        let exp_put = 2889397646742u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#97 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#97 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0098() {
        // S=100.0, K=95.0, T=0.25, r=0.0
        // v0=0.01, kappa=5.0, theta=0.04, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 5000000000000u128, 40000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 6132802198194u128;
        let exp_put = 1132802198194u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#98 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#98 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0099() {
        // S=100.0, K=115.0, T=1.0, r=0.0
        // v0=0.01, kappa=5.0, theta=0.04, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 1000000000000u128,
            10000000000u128, 5000000000000u128, 40000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 1980287223451u128;
        let exp_put = 16980287223451u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#99 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#99 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0100() {
        // S=100.0, K=80.0, T=0.25, r=0.0
        // v0=0.01, kappa=5.0, theta=0.04, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 5000000000000u128, 40000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 20084810828321u128;
        let exp_put = 84810828321u128;
        let tol = 625000000000u128; // $0.62
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#100 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#100 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0101() {
        // S=100.0, K=90.0, T=1.0, r=0.0
        // v0=0.01, kappa=5.0, theta=0.09, xi=0.1, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 1000000000000u128,
            10000000000u128, 5000000000000u128, 90000000000u128, 100000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 16082002516306u128;
        let exp_put = 6082002516306u128;
        let tol = 1090000000000u128; // $1.09
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#101 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#101 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0102() {
        // S=100.0, K=110.0, T=0.1, r=0.0
        // v0=0.01, kappa=5.0, theta=0.09, xi=0.2, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 5000000000000u128, 90000000000u128, 200000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 37724419995u128;
        let exp_put = 10037724419995u128;
        let tol = 1090000000000u128; // $1.09
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#102 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#102 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0103() {
        // S=100.0, K=80.0, T=1.0, r=0.0
        // v0=0.01, kappa=5.0, theta=0.09, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 1000000000000u128,
            10000000000u128, 5000000000000u128, 90000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 23116327051182u128;
        let exp_put = 3116327051182u128;
        let tol = 325000000000u128; // $0.33
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#103 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#103 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0104() {
        // S=100.0, K=90.0, T=0.1, r=0.0
        // v0=0.01, kappa=5.0, theta=0.09, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 5000000000000u128, 90000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 10125471175335u128;
        let exp_put = 125471175335u128;
        let tol = 483333333333u128; // $0.48
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#104 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#104 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0105() {
        // S=100.0, K=100.0, T=0.5, r=0.0
        // v0=0.01, kappa=5.0, theta=0.09, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 500000000000u128,
            10000000000u128, 5000000000000u128, 90000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 6819988863432u128;
        let exp_put = 6819988863432u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#105 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#105 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0106() {
        // S=100.0, K=110.0, T=2.0, r=0.0
        // v0=0.01, kappa=5.0, theta=0.09, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 5000000000000u128, 90000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 11693343261017u128;
        let exp_put = 21693343261017u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#106 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#106 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0107() {
        // S=100.0, K=120.0, T=0.25, r=0.0
        // v0=0.01, kappa=5.0, theta=0.16, xi=0.1, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 5000000000000u128, 160000000000u128, 100000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 596596852387u128;
        let exp_put = 20596596852387u128;
        let tol = 1300000000000u128; // $1.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#107 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#107 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0108() {
        // S=100.0, K=90.0, T=2.0, r=0.0
        // v0=0.01, kappa=5.0, theta=0.16, xi=0.2, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 2000000000000u128,
            10000000000u128, 5000000000000u128, 160000000000u128, 200000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 25503697008900u128;
        let exp_put = 15503697008900u128;
        let tol = 375000000000u128; // $0.38
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#108 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#108 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0109() {
        // S=100.0, K=100.0, T=0.25, r=0.0
        // v0=0.01, kappa=5.0, theta=0.16, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 250000000000u128,
            10000000000u128, 5000000000000u128, 160000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 5388289965726u128;
        let exp_put = 5388289965726u128;
        let tol = 375000000000u128; // $0.38
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#109 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#109 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0110() {
        // S=100.0, K=110.0, T=1.0, r=0.0
        // v0=0.01, kappa=5.0, theta=0.16, xi=0.5, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 1000000000000u128,
            10000000000u128, 5000000000000u128, 160000000000u128, 500000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 9972019250748u128;
        let exp_put = 19972019250748u128;
        let tol = 375000000000u128; // $0.38
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#110 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#110 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0111() {
        // S=100.0, K=120.0, T=0.1, r=0.0
        // v0=0.01, kappa=5.0, theta=0.16, xi=0.8, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 100000000000u128,
            10000000000u128, 5000000000000u128, 160000000000u128, 800000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 144433089u128;
        let exp_put = 20000144433089u128;
        let tol = 600000000000u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#111 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#111 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0112() {
        // S=100.0, K=85.0, T=1.0, r=0.0
        // v0=0.04, kappa=0.5, theta=0.01, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 500000000000u128, 10000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 17077147074133u128;
        let exp_put = 2077147074133u128;
        let tol = 302941176471u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#112 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#112 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0113() {
        // S=100.0, K=100.0, T=0.1, r=0.0
        // v0=0.04, kappa=0.5, theta=0.01, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 500000000000u128, 10000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 2468888851964u128;
        let exp_put = 2468888851964u128;
        let tol = 265000000000u128; // $0.27
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#113 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#113 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0114() {
        // S=100.0, K=110.0, T=0.5, r=0.0
        // v0=0.04, kappa=0.5, theta=0.01, xi=0.2, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 500000000000u128, 10000000000u128, 200000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 1960478887943u128;
        let exp_put = 11960478887943u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#114 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#114 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0115() {
        // S=100.0, K=120.0, T=2.0, r=0.0
        // v0=0.04, kappa=0.5, theta=0.01, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 500000000000u128, 10000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 2318391529129u128;
        let exp_put = 22318391529129u128;
        let tol = 100000000000u128; // $0.10
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#115 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#115 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0116() {
        // S=100.0, K=90.0, T=0.5, r=0.0
        // v0=0.04, kappa=0.5, theta=0.01, xi=0.5, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 500000000000u128, 10000000000u128, 500000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 11729017809293u128;
        let exp_put = 1729017809293u128;
        let tol = 283333333333u128; // $0.28
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#116 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#116 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0117() {
        // S=100.0, K=100.0, T=2.0, r=0.0
        // v0=0.04, kappa=0.5, theta=0.01, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 500000000000u128, 10000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 5570388535195u128;
        let exp_put = 5570388535195u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#117 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#117 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0118() {
        // S=100.0, K=110.0, T=0.25, r=0.0
        // v0=0.04, kappa=0.5, theta=0.04, xi=0.1, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 500000000000u128, 40000000000u128, 100000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 868257474759u128;
        let exp_put = 10868257474759u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#118 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#118 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0119() {
        // S=100.0, K=120.0, T=1.0, r=0.0
        // v0=0.04, kappa=0.5, theta=0.04, xi=0.2, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 500000000000u128, 40000000000u128, 200000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 1205228433364u128;
        let exp_put = 21205228433364u128;
        let tol = 100000000000u128; // $0.10
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#119 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#119 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0120() {
        // S=100.0, K=85.0, T=0.25, r=0.0
        // v0=0.04, kappa=0.5, theta=0.04, xi=0.3, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 500000000000u128, 40000000000u128, 300000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 15398776292243u128;
        let exp_put = 398776292243u128;
        let tol = 302941176471u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#120 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#120 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0121() {
        // S=100.0, K=100.0, T=1.0, r=0.0
        // v0=0.04, kappa=0.5, theta=0.04, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 500000000000u128, 40000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 6271058219239u128;
        let exp_put = 6271058219239u128;
        let tol = 450000000000u128; // $0.45
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#121 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#121 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0122() {
        // S=100.0, K=110.0, T=0.1, r=0.0
        // v0=0.04, kappa=0.5, theta=0.04, xi=0.8, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 500000000000u128, 40000000000u128, 800000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 2020324602u128;
        let exp_put = 10002020324602u128;
        let tol = 750000000000u128; // $0.75
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#122 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#122 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0123() {
        // S=100.0, K=120.0, T=0.5, r=0.0
        // v0=0.04, kappa=0.5, theta=0.04, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 500000000000u128, 40000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 942340136756u128;
        let exp_put = 20942340136756u128;
        let tol = 600000000000u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#123 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#123 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0124() {
        // S=100.0, K=85.0, T=0.1, r=0.0
        // v0=0.04, kappa=0.5, theta=0.09, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 500000000000u128, 90000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 15010575514772u128;
        let exp_put = 10575514772u128;
        let tol = 325000000000u128; // $0.33
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#124 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#124 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0125() {
        // S=100.0, K=95.0, T=0.5, r=0.0
        // v0=0.04, kappa=0.5, theta=0.09, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 500000000000u128, 90000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 8699345964140u128;
        let exp_put = 3699345964140u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#125 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#125 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0126() {
        // S=100.0, K=105.0, T=2.0, r=0.0
        // v0=0.04, kappa=0.5, theta=0.09, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 500000000000u128, 90000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 10210638294363u128;
        let exp_put = 15210638294363u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#126 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#126 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0127() {
        // S=100.0, K=115.0, T=0.25, r=0.0
        // v0=0.04, kappa=0.5, theta=0.09, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 500000000000u128, 90000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 229901611232u128;
        let exp_put = 15229901611232u128;
        let tol = 289130434783u128; // $0.29
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#127 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#127 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0128() {
        // S=100.0, K=80.0, T=2.0, r=0.0
        // v0=0.04, kappa=0.5, theta=0.09, xi=0.8, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 500000000000u128, 90000000000u128, 800000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 24296619193437u128;
        let exp_put = 4296619193437u128;
        let tol = 625000000000u128; // $0.62
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#128 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#128 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0129() {
        // S=100.0, K=90.0, T=0.25, r=0.0
        // v0=0.04, kappa=0.5, theta=0.16, xi=0.1, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 500000000000u128, 160000000000u128, 100000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 10986281841679u128;
        let exp_put = 986281841679u128;
        let tol = 535000000000u128; // $0.53
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#129 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#129 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0130() {
        // S=100.0, K=100.0, T=1.0, r=0.0
        // v0=0.04, kappa=0.5, theta=0.16, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 500000000000u128, 160000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 9867310807067u128;
        let exp_put = 9867310807067u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#130 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#130 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0131() {
        // S=100.0, K=110.0, T=0.1, r=0.0
        // v0=0.04, kappa=0.5, theta=0.16, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 500000000000u128, 160000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 84160584600u128;
        let exp_put = 10084160584600u128;
        let tol = 535000000000u128; // $0.53
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#131 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#131 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0132() {
        // S=100.0, K=120.0, T=0.5, r=0.0
        // v0=0.04, kappa=0.5, theta=0.16, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 500000000000u128, 160000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 1221394926798u128;
        let exp_put = 21221394926798u128;
        let tol = 100000000000u128; // $0.10
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#132 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#132 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0133() {
        // S=100.0, K=85.0, T=0.1, r=0.0
        // v0=0.04, kappa=0.5, theta=0.16, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 500000000000u128, 160000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 15023403222136u128;
        let exp_put = 23403222136u128;
        let tol = 302941176471u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#133 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#133 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0134() {
        // S=100.0, K=95.0, T=0.5, r=0.0
        // v0=0.04, kappa=0.5, theta=0.16, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 500000000000u128, 160000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 8628132213538u128;
        let exp_put = 3628132213538u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#134 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#134 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0135() {
        // S=100.0, K=105.0, T=2.0, r=0.0
        // v0=0.04, kappa=1.0, theta=0.01, xi=0.1, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 1000000000000u128, 10000000000u128, 100000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 6155808936625u128;
        let exp_put = 11155808936625u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#135 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#135 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0136() {
        // S=100.0, K=120.0, T=0.25, r=0.0
        // v0=0.04, kappa=1.0, theta=0.01, xi=0.2, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 1000000000000u128, 10000000000u128, 200000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 55735008126u128;
        let exp_put = 20055735008126u128;
        let tol = 340000000000u128; // $0.34
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#136 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#136 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0137() {
        // S=100.0, K=85.0, T=2.0, r=0.0
        // v0=0.04, kappa=1.0, theta=0.01, xi=0.3, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 1000000000000u128, 10000000000u128, 300000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 17992591656508u128;
        let exp_put = 2992591656508u128;
        let tol = 102941176471u128; // $0.10
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#137 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#137 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0138() {
        // S=100.0, K=95.0, T=0.25, r=0.0
        // v0=0.04, kappa=1.0, theta=0.01, xi=0.5, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 1000000000000u128, 10000000000u128, 500000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 6902604209626u128;
        let exp_put = 1902604209626u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#138 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#138 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0139() {
        // S=100.0, K=105.0, T=1.0, r=0.0
        // v0=0.04, kappa=1.0, theta=0.01, xi=0.8, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 1000000000000u128, 10000000000u128, 800000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 1101339853969u128;
        let exp_put = 6101339853969u128;
        let tol = 750000000000u128; // $0.75
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#139 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#139 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0140() {
        // S=100.0, K=115.0, T=0.1, r=0.0
        // v0=0.04, kappa=1.0, theta=0.04, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 1000000000000u128, 40000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 15437995764u128;
        let exp_put = 15015437995764u128;
        let tol = 289130434783u128; // $0.29
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#140 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#140 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0141() {
        // S=100.0, K=80.0, T=1.0, r=0.0
        // v0=0.04, kappa=1.0, theta=0.04, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 1000000000000u128, 40000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 21190739397855u128;
        let exp_put = 1190739397855u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#141 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#141 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0142() {
        // S=100.0, K=90.0, T=0.1, r=0.0
        // v0=0.04, kappa=1.0, theta=0.04, xi=0.2, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 1000000000000u128, 40000000000u128, 200000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 10119036864084u128;
        let exp_put = 119036864084u128;
        let tol = 283333333333u128; // $0.28
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#142 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#142 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0143() {
        // S=100.0, K=100.0, T=0.5, r=0.0
        // v0=0.04, kappa=1.0, theta=0.04, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 1000000000000u128, 40000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 5424160362750u128;
        let exp_put = 5424160362750u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#143 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#143 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0144() {
        // S=100.0, K=110.0, T=2.0, r=0.0
        // v0=0.04, kappa=1.0, theta=0.04, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 1000000000000u128, 40000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 5243412863777u128;
        let exp_put = 15243412863777u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#144 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#144 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0145() {
        // S=100.0, K=120.0, T=0.25, r=0.0
        // v0=0.04, kappa=1.0, theta=0.04, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 1000000000000u128, 40000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 77730186525u128;
        let exp_put = 20077730186525u128;
        let tol = 600000000000u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#145 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#145 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0146() {
        // S=100.0, K=85.0, T=2.0, r=0.0
        // v0=0.04, kappa=1.0, theta=0.09, xi=0.1, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 1000000000000u128, 90000000000u128, 100000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 22452186811313u128;
        let exp_put = 7452186811313u128;
        let tol = 102941176471u128; // $0.10
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#146 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#146 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0147() {
        // S=100.0, K=95.0, T=0.25, r=0.0
        // v0=0.04, kappa=1.0, theta=0.09, xi=0.2, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 1000000000000u128, 90000000000u128, 200000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 7211247648713u128;
        let exp_put = 2211247648713u128;
        let tol = 400000000000u128; // $0.40
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#147 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#147 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0148() {
        // S=100.0, K=105.0, T=1.0, r=0.0
        // v0=0.04, kappa=1.0, theta=0.09, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 1000000000000u128, 90000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 6665610161132u128;
        let exp_put = 11665610161132u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#148 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#148 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0149() {
        // S=100.0, K=115.0, T=0.1, r=0.0
        // v0=0.04, kappa=1.0, theta=0.09, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 1000000000000u128, 90000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 74950646u128;
        let exp_put = 15000074950646u128;
        let tol = 489130434783u128; // $0.49
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#149 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#149 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0150() {
        // S=100.0, K=80.0, T=1.0, r=0.0
        // v0=0.04, kappa=1.0, theta=0.09, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 1000000000000u128, 90000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 22026941060981u128;
        let exp_put = 2026941060981u128;
        let tol = 325000000000u128; // $0.33
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#150 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#150 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0151() {
        // S=100.0, K=90.0, T=0.1, r=0.0
        // v0=0.04, kappa=1.0, theta=0.09, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 1000000000000u128, 90000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 10180882145093u128;
        let exp_put = 180882145093u128;
        let tol = 583333333333u128; // $0.58
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#151 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#151 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0152() {
        // S=100.0, K=100.0, T=0.5, r=0.0
        // v0=0.04, kappa=1.0, theta=0.16, xi=0.1, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 1000000000000u128, 160000000000u128, 100000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 7176899386439u128;
        let exp_put = 7176899386439u128;
        let tol = 610000000000u128; // $0.61
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#152 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#152 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0153() {
        // S=100.0, K=110.0, T=2.0, r=0.0
        // v0=0.04, kappa=1.0, theta=0.16, xi=0.2, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 1000000000000u128, 160000000000u128, 200000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 14117464595916u128;
        let exp_put = 24117464595916u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#153 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#153 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0154() {
        // S=100.0, K=120.0, T=0.25, r=0.0
        // v0=0.04, kappa=1.0, theta=0.16, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 1000000000000u128, 160000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 184708053627u128;
        let exp_put = 20184708053627u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#154 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#154 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0155() {
        // S=100.0, K=85.0, T=2.0, r=0.0
        // v0=0.04, kappa=1.0, theta=0.16, xi=0.5, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 1000000000000u128, 160000000000u128, 500000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 25114529880105u128;
        let exp_put = 10114529880105u128;
        let tol = 302941176471u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#155 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#155 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0156() {
        // S=100.0, K=100.0, T=0.25, r=0.0
        // v0=0.04, kappa=1.0, theta=0.16, xi=0.8, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 1000000000000u128, 160000000000u128, 800000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 4157963043754u128;
        let exp_put = 4157963043754u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#156 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#156 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0157() {
        // S=100.0, K=110.0, T=1.0, r=0.0
        // v0=0.04, kappa=2.0, theta=0.01, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 2000000000000u128, 10000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 2163212147938u128;
        let exp_put = 12163212147938u128;
        let tol = 490000000000u128; // $0.49
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#157 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#157 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0158() {
        // S=100.0, K=80.0, T=0.25, r=0.0
        // v0=0.04, kappa=2.0, theta=0.01, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 2000000000000u128, 10000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 20082409779811u128;
        let exp_put = 82409779811u128;
        let tol = 490000000000u128; // $0.49
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#158 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#158 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0159() {
        // S=100.0, K=90.0, T=1.0, r=0.0
        // v0=0.04, kappa=2.0, theta=0.01, xi=0.2, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 2000000000000u128, 10000000000u128, 200000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 12016295241642u128;
        let exp_put = 2016295241642u128;
        let tol = 83333333333u128; // $0.08
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#159 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#159 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0160() {
        // S=100.0, K=100.0, T=0.1, r=0.0
        // v0=0.04, kappa=2.0, theta=0.01, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 2000000000000u128, 10000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 2395469288186u128;
        let exp_put = 2395469288186u128;
        let tol = 490000000000u128; // $0.49
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#160 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#160 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0161() {
        // S=100.0, K=110.0, T=0.5, r=0.0
        // v0=0.04, kappa=2.0, theta=0.01, xi=0.5, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 2000000000000u128, 10000000000u128, 500000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 1120478995084u128;
        let exp_put = 11120478995084u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#161 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#161 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0162() {
        // S=100.0, K=120.0, T=2.0, r=0.0
        // v0=0.04, kappa=2.0, theta=0.01, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 2000000000000u128, 10000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 653091967667u128;
        let exp_put = 20653091967667u128;
        let tol = 600000000000u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#162 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#162 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0163() {
        // S=100.0, K=85.0, T=0.5, r=0.0
        // v0=0.04, kappa=2.0, theta=0.04, xi=0.1, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 2000000000000u128, 40000000000u128, 100000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 15897933898405u128;
        let exp_put = 897933898405u128;
        let tol = 400000000000u128; // $0.40
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#163 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#163 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0164() {
        // S=100.0, K=95.0, T=2.0, r=0.0
        // v0=0.04, kappa=2.0, theta=0.04, xi=0.2, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 2000000000000u128, 40000000000u128, 200000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 13549607179924u128;
        let exp_put = 8549607179924u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#164 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#164 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0165() {
        // S=100.0, K=110.0, T=0.25, r=0.0
        // v0=0.04, kappa=2.0, theta=0.04, xi=0.3, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 2000000000000u128, 40000000000u128, 300000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 628683276152u128;
        let exp_put = 10628683276152u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#165 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#165 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0166() {
        // S=100.0, K=120.0, T=1.0, r=0.0
        // v0=0.04, kappa=2.0, theta=0.04, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 2000000000000u128, 40000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 332315907025u128;
        let exp_put = 20332315907025u128;
        let tol = 500000000000u128; // $0.50
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#166 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#166 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0167() {
        // S=100.0, K=85.0, T=0.25, r=0.0
        // v0=0.04, kappa=2.0, theta=0.04, xi=0.8, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 2000000000000u128, 40000000000u128, 800000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 15691663229526u128;
        let exp_put = 691663229526u128;
        let tol = 802941176471u128; // $0.80
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#167 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#167 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0168() {
        // S=100.0, K=100.0, T=1.0, r=0.0
        // v0=0.04, kappa=2.0, theta=0.04, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 2000000000000u128, 40000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 6965405871923u128;
        let exp_put = 6965405871923u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#168 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#168 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0169() {
        // S=100.0, K=110.0, T=0.1, r=0.0
        // v0=0.04, kappa=2.0, theta=0.09, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 2000000000000u128, 90000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 235919899760u128;
        let exp_put = 10235919899760u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#169 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#169 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0170() {
        // S=100.0, K=120.0, T=0.5, r=0.0
        // v0=0.04, kappa=2.0, theta=0.09, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 2000000000000u128, 90000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 1219336488589u128;
        let exp_put = 21219336488589u128;
        let tol = 100000000000u128; // $0.10
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#170 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#170 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0171() {
        // S=100.0, K=85.0, T=0.1, r=0.0
        // v0=0.04, kappa=2.0, theta=0.09, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 2000000000000u128, 90000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 15028129838453u128;
        let exp_put = 28129838453u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#171 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#171 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0172() {
        // S=100.0, K=95.0, T=0.5, r=0.0
        // v0=0.04, kappa=2.0, theta=0.09, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 2000000000000u128, 90000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 9359365009677u128;
        let exp_put = 4359365009677u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#172 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#172 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0173() {
        // S=100.0, K=105.0, T=2.0, r=0.0
        // v0=0.04, kappa=2.0, theta=0.09, xi=0.8, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 2000000000000u128, 90000000000u128, 800000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 11331960813057u128;
        let exp_put = 16331960813057u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#173 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#173 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0174() {
        // S=100.0, K=115.0, T=0.25, r=0.0
        // v0=0.04, kappa=2.0, theta=0.16, xi=0.1, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 2000000000000u128, 160000000000u128, 100000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 859772256267u128;
        let exp_put = 15859772256267u128;
        let tol = 760000000000u128; // $0.76
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#174 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#174 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0175() {
        // S=100.0, K=80.0, T=2.0, r=0.0
        // v0=0.04, kappa=2.0, theta=0.16, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 2000000000000u128, 160000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 29920218574627u128;
        let exp_put = 9920218574627u128;
        let tol = 325000000000u128; // $0.33
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#175 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#175 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0176() {
        // S=100.0, K=90.0, T=0.25, r=0.0
        // v0=0.04, kappa=2.0, theta=0.16, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 2000000000000u128, 160000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 11618227498124u128;
        let exp_put = 1618227498124u128;
        let tol = 283333333333u128; // $0.28
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#176 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#176 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0177() {
        // S=100.0, K=100.0, T=1.0, r=0.0
        // v0=0.04, kappa=2.0, theta=0.16, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 2000000000000u128, 160000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 12951449326905u128;
        let exp_put = 12951449326905u128;
        let tol = 120000000000u128; // $0.12
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#177 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#177 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0178() {
        // S=100.0, K=110.0, T=0.1, r=0.0
        // v0=0.04, kappa=2.0, theta=0.16, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 2000000000000u128, 160000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 323922112739u128;
        let exp_put = 10323922112739u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#178 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#178 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0179() {
        // S=100.0, K=120.0, T=0.5, r=0.0
        // v0=0.04, kappa=2.0, theta=0.16, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 2000000000000u128, 160000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 1744533240439u128;
        let exp_put = 21744533240439u128;
        let tol = 600000000000u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#179 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#179 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0180() {
        // S=100.0, K=85.0, T=0.1, r=0.0
        // v0=0.04, kappa=3.0, theta=0.01, xi=0.1, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 3000000000000u128, 10000000000u128, 100000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 15007419873350u128;
        let exp_put = 7419873350u128;
        let tol = 640000000000u128; // $0.64
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#180 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#180 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0181() {
        // S=100.0, K=105.0, T=0.5, r=0.0
        // v0=0.04, kappa=3.0, theta=0.01, xi=0.2, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 3000000000000u128, 10000000000u128, 200000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 2320358882645u128;
        let exp_put = 7320358882645u128;
        let tol = 180000000000u128; // $0.18
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#181 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#181 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0182() {
        // S=100.0, K=115.0, T=2.0, r=0.0
        // v0=0.04, kappa=3.0, theta=0.01, xi=0.3, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 3000000000000u128, 10000000000u128, 300000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 1293753186250u128;
        let exp_put = 16293753186250u128;
        let tol = 180000000000u128; // $0.18
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#182 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#182 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0183() {
        // S=100.0, K=80.0, T=0.5, r=0.0
        // v0=0.04, kappa=3.0, theta=0.01, xi=0.5, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 3000000000000u128, 10000000000u128, 500000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 20442175972707u128;
        let exp_put = 442175972707u128;
        let tol = 325000000000u128; // $0.33
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#183 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#183 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0184() {
        // S=100.0, K=90.0, T=2.0, r=0.0
        // v0=0.04, kappa=3.0, theta=0.01, xi=0.8, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 3000000000000u128, 10000000000u128, 800000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 12765301229591u128;
        let exp_put = 2765301229591u128;
        let tol = 783333333333u128; // $0.78
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#184 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#184 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0185() {
        // S=100.0, K=100.0, T=0.25, r=0.0
        // v0=0.04, kappa=3.0, theta=0.04, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 3000000000000u128, 40000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 3967469622134u128;
        let exp_put = 3967469622134u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#185 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#185 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0186() {
        // S=100.0, K=110.0, T=1.0, r=0.0
        // v0=0.04, kappa=3.0, theta=0.04, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 3000000000000u128, 40000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 4281331754373u128;
        let exp_put = 14281331754373u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#186 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#186 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0187() {
        // S=100.0, K=80.0, T=0.25, r=0.0
        // v0=0.04, kappa=3.0, theta=0.04, xi=0.2, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 3000000000000u128, 40000000000u128, 200000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 20046444660584u128;
        let exp_put = 46444660584u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#187 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#187 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0188() {
        // S=100.0, K=90.0, T=1.0, r=0.0
        // v0=0.04, kappa=3.0, theta=0.04, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 3000000000000u128, 40000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 13642069827986u128;
        let exp_put = 3642069827986u128;
        let tol = 180000000000u128; // $0.18
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#188 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#188 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0189() {
        // S=100.0, K=100.0, T=0.1, r=0.0
        // v0=0.04, kappa=3.0, theta=0.04, xi=0.5, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 3000000000000u128, 40000000000u128, 500000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 2450256059438u128;
        let exp_put = 2450256059438u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#189 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#189 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0190() {
        // S=100.0, K=110.0, T=0.5, r=0.0
        // v0=0.04, kappa=3.0, theta=0.04, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 3000000000000u128, 40000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 1306787469521u128;
        let exp_put = 11306787469521u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#190 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#190 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0191() {
        // S=100.0, K=120.0, T=2.0, r=0.0
        // v0=0.04, kappa=3.0, theta=0.09, xi=0.1, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 3000000000000u128, 90000000000u128, 100000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 9097505077201u128;
        let exp_put = 29097505077201u128;
        let tol = 180000000000u128; // $0.18
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#191 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#191 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0192() {
        // S=100.0, K=85.0, T=0.5, r=0.0
        // v0=0.04, kappa=3.0, theta=0.09, xi=0.2, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 3000000000000u128, 90000000000u128, 200000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 16810284206541u128;
        let exp_put = 1810284206541u128;
        let tol = 180000000000u128; // $0.18
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#192 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#192 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0193() {
        // S=100.0, K=95.0, T=2.0, r=0.0
        // v0=0.04, kappa=3.0, theta=0.09, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 3000000000000u128, 90000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 18006785711461u128;
        let exp_put = 13006785711461u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#193 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#193 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0194() {
        // S=100.0, K=105.0, T=0.25, r=0.0
        // v0=0.04, kappa=3.0, theta=0.09, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 3000000000000u128, 90000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 2188963889894u128;
        let exp_put = 7188963889894u128;
        let tol = 450000000000u128; // $0.45
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#194 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#194 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0195() {
        // S=100.0, K=115.0, T=1.0, r=0.0
        // v0=0.04, kappa=3.0, theta=0.09, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 3000000000000u128, 90000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 5472504680855u128;
        let exp_put = 20472504680855u128;
        let tol = 289130434783u128; // $0.29
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#195 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#195 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0196() {
        // S=100.0, K=80.0, T=0.25, r=0.0
        // v0=0.04, kappa=3.0, theta=0.09, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 3000000000000u128, 90000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 20195064064743u128;
        let exp_put = 195064064743u128;
        let tol = 625000000000u128; // $0.62
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#196 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#196 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0197() {
        // S=100.0, K=90.0, T=1.0, r=0.0
        // v0=0.04, kappa=3.0, theta=0.16, xi=0.1, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 3000000000000u128, 160000000000u128, 100000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 18766033299134u128;
        let exp_put = 8766033299134u128;
        let tol = 910000000000u128; // $0.91
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#197 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#197 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0198() {
        // S=100.0, K=100.0, T=0.1, r=0.0
        // v0=0.04, kappa=3.0, theta=0.16, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 3000000000000u128, 160000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 2959652489784u128;
        let exp_put = 2959652489784u128;
        let tol = 910000000000u128; // $0.91
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#198 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#198 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0199() {
        // S=100.0, K=110.0, T=0.5, r=0.0
        // v0=0.04, kappa=3.0, theta=0.16, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 3000000000000u128, 160000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 4808434462763u128;
        let exp_put = 14808434462763u128;
        let tol = 180000000000u128; // $0.18
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#199 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#199 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0200() {
        // S=100.0, K=120.0, T=2.0, r=0.0
        // v0=0.04, kappa=3.0, theta=0.16, xi=0.5, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 3000000000000u128, 160000000000u128, 500000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 12926482553838u128;
        let exp_put = 32926482553838u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#200 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#200 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0201() {
        // S=100.0, K=85.0, T=0.5, r=0.0
        // v0=0.04, kappa=3.0, theta=0.16, xi=0.8, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 3000000000000u128, 160000000000u128, 800000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 18130439546874u128;
        let exp_put = 3130439546874u128;
        let tol = 602941176471u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#201 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#201 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0202() {
        // S=100.0, K=95.0, T=2.0, r=0.0
        // v0=0.04, kappa=5.0, theta=0.01, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 5000000000000u128, 10000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 9132838632813u128;
        let exp_put = 4132838632813u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#202 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#202 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0203() {
        // S=100.0, K=120.0, T=0.25, r=0.0
        // v0=0.04, kappa=5.0, theta=0.01, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 5000000000000u128, 10000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 1525798969u128;
        let exp_put = 20001525798969u128;
        let tol = 940000000000u128; // $0.94
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#203 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#203 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0204() {
        // S=100.0, K=85.0, T=2.0, r=0.0
        // v0=0.04, kappa=5.0, theta=0.01, xi=0.2, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 5000000000000u128, 10000000000u128, 200000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 16217705206080u128;
        let exp_put = 1217705206080u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#204 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#204 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0205() {
        // S=100.0, K=95.0, T=0.25, r=0.0
        // v0=0.04, kappa=5.0, theta=0.01, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 5000000000000u128, 10000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 6274932844769u128;
        let exp_put = 1274932844769u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#205 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#205 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0206() {
        // S=100.0, K=105.0, T=1.0, r=0.0
        // v0=0.04, kappa=5.0, theta=0.01, xi=0.5, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 5000000000000u128, 10000000000u128, 500000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 2632914285423u128;
        let exp_put = 7632914285423u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#206 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#206 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0207() {
        // S=100.0, K=120.0, T=0.1, r=0.0
        // v0=0.04, kappa=5.0, theta=0.01, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 5000000000000u128, 10000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 3365779286u128;
        let exp_put = 20003365779286u128;
        let tol = 600000000000u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#207 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#207 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0208() {
        // S=100.0, K=85.0, T=1.0, r=0.0
        // v0=0.04, kappa=5.0, theta=0.04, xi=0.1, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 5000000000000u128, 40000000000u128, 100000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 17232435594620u128;
        let exp_put = 2232435594620u128;
        let tol = 850000000000u128; // $0.85
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#208 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#208 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0209() {
        // S=100.0, K=100.0, T=0.1, r=0.0
        // v0=0.04, kappa=5.0, theta=0.04, xi=0.2, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 5000000000000u128, 40000000000u128, 200000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 2495690564752u128;
        let exp_put = 2495690564752u128;
        let tol = 850000000000u128; // $0.85
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#209 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#209 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0210() {
        // S=100.0, K=115.0, T=0.5, r=0.0
        // v0=0.04, kappa=5.0, theta=0.04, xi=0.3, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 5000000000000u128, 40000000000u128, 300000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 902835057232u128;
        let exp_put = 15902835057232u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#210 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#210 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0211() {
        // S=100.0, K=85.0, T=0.1, r=0.0
        // v0=0.04, kappa=5.0, theta=0.04, xi=0.5, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 5000000000000u128, 40000000000u128, 500000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 15051599916178u128;
        let exp_put = 51599916178u128;
        let tol = 302941176471u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#211 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#211 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0212() {
        // S=100.0, K=95.0, T=0.5, r=0.0
        // v0=0.04, kappa=5.0, theta=0.04, xi=0.8, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 5000000000000u128, 40000000000u128, 800000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 8332712497439u128;
        let exp_put = 3332712497439u128;
        let tol = 750000000000u128; // $0.75
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#212 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#212 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0213() {
        // S=100.0, K=105.0, T=2.0, r=0.0
        // v0=0.04, kappa=5.0, theta=0.04, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 5000000000000u128, 40000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 8865675979308u128;
        let exp_put = 13865675979308u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#213 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#213 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0214() {
        // S=100.0, K=120.0, T=0.25, r=0.0
        // v0=0.04, kappa=5.0, theta=0.09, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 5000000000000u128, 90000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 422840090236u128;
        let exp_put = 20422840090236u128;
        let tol = 1000000000000u128; // $1.00
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#214 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#214 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0215() {
        // S=100.0, K=85.0, T=2.0, r=0.0
        // v0=0.04, kappa=5.0, theta=0.09, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 5000000000000u128, 90000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 23767360831968u128;
        let exp_put = 8767360831968u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#215 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#215 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0216() {
        // S=100.0, K=95.0, T=0.25, r=0.0
        // v0=0.04, kappa=5.0, theta=0.09, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 5000000000000u128, 90000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 7739501263405u128;
        let exp_put = 2739501263405u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#216 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#216 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0217() {
        // S=100.0, K=105.0, T=1.0, r=0.0
        // v0=0.04, kappa=5.0, theta=0.09, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 5000000000000u128, 90000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 8832806075122u128;
        let exp_put = 13832806075122u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#217 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#217 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0218() {
        // S=100.0, K=115.0, T=0.1, r=0.0
        // v0=0.04, kappa=5.0, theta=0.09, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 5000000000000u128, 90000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 26602517889u128;
        let exp_put = 15026602517889u128;
        let tol = 589130434783u128; // $0.59
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#218 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#218 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0219() {
        // S=100.0, K=80.0, T=1.0, r=0.0
        // v0=0.04, kappa=5.0, theta=0.16, xi=0.1, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 1000000000000u128,
            40000000000u128, 5000000000000u128, 160000000000u128, 100000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 25536649181632u128;
        let exp_put = 5536649181632u128;
        let tol = 1210000000000u128; // $1.21
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#219 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#219 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0220() {
        // S=100.0, K=90.0, T=0.1, r=0.0
        // v0=0.04, kappa=5.0, theta=0.16, xi=0.2, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 100000000000u128,
            40000000000u128, 5000000000000u128, 160000000000u128, 200000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 10389642489025u128;
        let exp_put = 389642489025u128;
        let tol = 1210000000000u128; // $1.21
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#220 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#220 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0221() {
        // S=100.0, K=100.0, T=0.5, r=0.0
        // v0=0.04, kappa=5.0, theta=0.16, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 500000000000u128,
            40000000000u128, 5000000000000u128, 160000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 9453043140215u128;
        let exp_put = 9453043140215u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#221 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#221 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0222() {
        // S=100.0, K=110.0, T=2.0, r=0.0
        // v0=0.04, kappa=5.0, theta=0.16, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 5000000000000u128, 160000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 17819386147559u128;
        let exp_put = 27819386147559u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#222 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#222 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0223() {
        // S=100.0, K=120.0, T=0.25, r=0.0
        // v0=0.04, kappa=5.0, theta=0.16, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 250000000000u128,
            40000000000u128, 5000000000000u128, 160000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 925353213673u128;
        let exp_put = 20925353213673u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#223 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#223 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0224() {
        // S=100.0, K=90.0, T=2.0, r=0.0
        // v0=0.04, kappa=5.0, theta=0.16, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 2000000000000u128,
            40000000000u128, 5000000000000u128, 160000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 25494207623440u128;
        let exp_put = 15494207623440u128;
        let tol = 583333333333u128; // $0.58
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#224 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#224 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0225() {
        // S=100.0, K=100.0, T=0.25, r=0.0
        // v0=0.09, kappa=0.5, theta=0.01, xi=0.1, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 500000000000u128, 10000000000u128, 100000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 5797938827614u128;
        let exp_put = 5797938827614u128;
        let tol = 415000000000u128; // $0.41
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#225 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#225 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0226() {
        // S=100.0, K=110.0, T=1.0, r=0.0
        // v0=0.09, kappa=0.5, theta=0.01, xi=0.2, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 500000000000u128, 10000000000u128, 200000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 6361856522311u128;
        let exp_put = 16361856522311u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#226 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#226 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0227() {
        // S=100.0, K=120.0, T=0.1, r=0.0
        // v0=0.09, kappa=0.5, theta=0.01, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 500000000000u128, 10000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 59381698251u128;
        let exp_put = 20059381698251u128;
        let tol = 415000000000u128; // $0.41
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#227 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#227 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0228() {
        // S=100.0, K=85.0, T=1.0, r=0.0
        // v0=0.09, kappa=0.5, theta=0.01, xi=0.5, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 500000000000u128, 10000000000u128, 500000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 19486084027341u128;
        let exp_put = 4486084027341u128;
        let tol = 302941176471u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#228 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#228 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0229() {
        // S=100.0, K=95.0, T=0.1, r=0.0
        // v0=0.09, kappa=0.5, theta=0.01, xi=0.8, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 500000000000u128, 10000000000u128, 800000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 6812220084864u128;
        let exp_put = 1812220084864u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#229 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#229 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0230() {
        // S=100.0, K=105.0, T=0.5, r=0.0
        // v0=0.09, kappa=0.5, theta=0.04, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 500000000000u128, 40000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 5944057806162u128;
        let exp_put = 10944057806162u128;
        let tol = 325000000000u128; // $0.33
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#230 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#230 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0231() {
        // S=100.0, K=115.0, T=2.0, r=0.0
        // v0=0.09, kappa=0.5, theta=0.04, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 500000000000u128, 40000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 9621366957411u128;
        let exp_put = 24621366957411u128;
        let tol = 89130434783u128; // $0.09
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#231 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#231 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0232() {
        // S=100.0, K=80.0, T=0.5, r=0.0
        // v0=0.09, kappa=0.5, theta=0.04, xi=0.2, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 500000000000u128, 40000000000u128, 200000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 21288665861476u128;
        let exp_put = 1288665861476u128;
        let tol = 125000000000u128; // $0.12
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#232 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#232 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0233() {
        // S=100.0, K=90.0, T=2.0, r=0.0
        // v0=0.09, kappa=0.5, theta=0.04, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 500000000000u128, 40000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 19234013635066u128;
        let exp_put = 9234013635066u128;
        let tol = 83333333333u128; // $0.08
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#233 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#233 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0234() {
        // S=100.0, K=100.0, T=0.25, r=0.0
        // v0=0.09, kappa=0.5, theta=0.04, xi=0.5, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 500000000000u128, 40000000000u128, 500000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 5684332635047u128;
        let exp_put = 5684332635047u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#234 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#234 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0235() {
        // S=100.0, K=110.0, T=1.0, r=0.0
        // v0=0.09, kappa=0.5, theta=0.04, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 500000000000u128, 40000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 4336665236963u128;
        let exp_put = 14336665236963u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#235 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#235 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0236() {
        // S=100.0, K=120.0, T=0.1, r=0.0
        // v0=0.09, kappa=0.5, theta=0.09, xi=0.1, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 500000000000u128, 90000000000u128, 100000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 88799885950u128;
        let exp_put = 20088799885950u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#236 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#236 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0237() {
        // S=100.0, K=90.0, T=1.0, r=0.0
        // v0=0.09, kappa=0.5, theta=0.09, xi=0.2, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 500000000000u128, 90000000000u128, 200000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 17097423323581u128;
        let exp_put = 7097423323581u128;
        let tol = 83333333333u128; // $0.08
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#237 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#237 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0238() {
        // S=100.0, K=100.0, T=0.1, r=0.0
        // v0=0.09, kappa=0.5, theta=0.09, xi=0.3, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 500000000000u128, 90000000000u128, 300000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 3734545075507u128;
        let exp_put = 3734545075507u128;
        let tol = 175000000000u128; // $0.17
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#238 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#238 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0239() {
        // S=100.0, K=110.0, T=0.5, r=0.0
        // v0=0.09, kappa=0.5, theta=0.09, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 500000000000u128, 90000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 3174410554080u128;
        let exp_put = 13174410554080u128;
        let tol = 450000000000u128; // $0.45
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#239 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#239 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0240() {
        // S=100.0, K=120.0, T=2.0, r=0.0
        // v0=0.09, kappa=0.5, theta=0.09, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 500000000000u128, 90000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 8891983151663u128;
        let exp_put = 28891983151663u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#240 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#240 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0241() {
        // S=100.0, K=85.0, T=0.5, r=0.0
        // v0=0.09, kappa=0.5, theta=0.09, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 500000000000u128, 90000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 17302762377119u128;
        let exp_put = 2302762377119u128;
        let tol = 602941176471u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#241 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#241 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0242() {
        // S=100.0, K=95.0, T=2.0, r=0.0
        // v0=0.09, kappa=0.5, theta=0.16, xi=0.1, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 500000000000u128, 160000000000u128, 100000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 21024707492639u128;
        let exp_put = 16024707492639u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#242 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#242 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0243() {
        // S=100.0, K=110.0, T=0.25, r=0.0
        // v0=0.09, kappa=0.5, theta=0.16, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 500000000000u128, 160000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 2512240584579u128;
        let exp_put = 12512240584579u128;
        let tol = 385000000000u128; // $0.39
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#243 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#243 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0244() {
        // S=100.0, K=120.0, T=1.0, r=0.0
        // v0=0.09, kappa=0.5, theta=0.16, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 500000000000u128, 160000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 5277219947450u128;
        let exp_put = 25277219947450u128;
        let tol = 100000000000u128; // $0.10
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#244 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#244 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0245() {
        // S=100.0, K=85.0, T=0.25, r=0.0
        // v0=0.09, kappa=0.5, theta=0.16, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 500000000000u128, 160000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 16314765695511u128;
        let exp_put = 1314765695511u128;
        let tol = 302941176471u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#245 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#245 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0246() {
        // S=100.0, K=95.0, T=1.0, r=0.0
        // v0=0.09, kappa=0.5, theta=0.16, xi=0.8, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 500000000000u128, 160000000000u128, 800000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 13507656978845u128;
        let exp_put = 8507656978845u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#246 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#246 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0247() {
        // S=100.0, K=110.0, T=0.1, r=0.0
        // v0=0.09, kappa=1.0, theta=0.01, xi=0.1, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 1000000000000u128, 10000000000u128, 100000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 709107010477u128;
        let exp_put = 10709107010477u128;
        let tol = 490000000000u128; // $0.49
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#247 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#247 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0248() {
        // S=100.0, K=120.0, T=0.5, r=0.0
        // v0=0.09, kappa=1.0, theta=0.01, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 1000000000000u128, 10000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 1239896837000u128;
        let exp_put = 21239896837000u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#248 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#248 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0249() {
        // S=100.0, K=85.0, T=0.1, r=0.0
        // v0=0.09, kappa=1.0, theta=0.01, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 1000000000000u128, 10000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 15223158469825u128;
        let exp_put = 223158469825u128;
        let tol = 490000000000u128; // $0.49
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#249 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#249 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0250() {
        // S=100.0, K=95.0, T=0.5, r=0.0
        // v0=0.09, kappa=1.0, theta=0.01, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 1000000000000u128, 10000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 10045582832431u128;
        let exp_put = 5045582832431u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#250 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#250 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0251() {
        // S=100.0, K=105.0, T=2.0, r=0.0
        // v0=0.09, kappa=1.0, theta=0.01, xi=0.5, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 1000000000000u128, 10000000000u128, 500000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 7861100704981u128;
        let exp_put = 12861100704981u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#251 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#251 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0252() {
        // S=100.0, K=115.0, T=0.25, r=0.0
        // v0=0.09, kappa=1.0, theta=0.01, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 1000000000000u128, 10000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 979525338241u128;
        let exp_put = 15979525338241u128;
        let tol = 589130434783u128; // $0.59
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#252 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#252 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0253() {
        // S=100.0, K=80.0, T=2.0, r=0.0
        // v0=0.09, kappa=1.0, theta=0.04, xi=0.1, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 1000000000000u128, 40000000000u128, 100000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 25115208859463u128;
        let exp_put = 5115208859463u128;
        let tol = 125000000000u128; // $0.12
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#253 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#253 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0254() {
        // S=100.0, K=90.0, T=0.25, r=0.0
        // v0=0.09, kappa=1.0, theta=0.04, xi=0.2, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 1000000000000u128, 40000000000u128, 200000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 11972469301249u128;
        let exp_put = 1972469301249u128;
        let tol = 400000000000u128; // $0.40
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#254 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#254 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0255() {
        // S=100.0, K=100.0, T=1.0, r=0.0
        // v0=0.09, kappa=1.0, theta=0.04, xi=0.3, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 1000000000000u128, 40000000000u128, 300000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 10121692420855u128;
        let exp_put = 10121692420855u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#255 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#255 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0256() {
        // S=100.0, K=110.0, T=0.1, r=0.0
        // v0=0.09, kappa=1.0, theta=0.04, xi=0.5, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 1000000000000u128, 40000000000u128, 500000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 536342938864u128;
        let exp_put = 10536342938864u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#256 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#256 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0257() {
        // S=100.0, K=120.0, T=0.5, r=0.0
        // v0=0.09, kappa=1.0, theta=0.04, xi=0.8, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 1000000000000u128, 40000000000u128, 800000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 105522611441u128;
        let exp_put = 20105522611441u128;
        let tol = 800000000000u128; // $0.80
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#257 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#257 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0258() {
        // S=100.0, K=85.0, T=0.1, r=0.0
        // v0=0.09, kappa=1.0, theta=0.09, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 1000000000000u128, 90000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 15179593442609u128;
        let exp_put = 179593442609u128;
        let tol = 302941176471u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#258 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#258 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0259() {
        // S=100.0, K=100.0, T=0.5, r=0.0
        // v0=0.09, kappa=1.0, theta=0.09, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 1000000000000u128, 90000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 8421755462272u128;
        let exp_put = 8421755462272u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#259 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#259 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0260() {
        // S=100.0, K=110.0, T=2.0, r=0.0
        // v0=0.09, kappa=1.0, theta=0.09, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 1000000000000u128, 90000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 12606135761676u128;
        let exp_put = 22606135761676u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#260 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#260 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0261() {
        // S=100.0, K=80.0, T=0.5, r=0.0
        // v0=0.09, kappa=1.0, theta=0.09, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 1000000000000u128, 90000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 21594514434046u128;
        let exp_put = 1594514434046u128;
        let tol = 125000000000u128; // $0.12
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#261 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#261 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0262() {
        // S=100.0, K=90.0, T=2.0, r=0.0
        // v0=0.09, kappa=1.0, theta=0.09, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 1000000000000u128, 90000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 20620978836744u128;
        let exp_put = 10620978836744u128;
        let tol = 283333333333u128; // $0.28
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#262 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#262 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0263() {
        // S=100.0, K=100.0, T=0.25, r=0.0
        // v0=0.09, kappa=1.0, theta=0.09, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 1000000000000u128, 90000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 5551055362645u128;
        let exp_put = 5551055362645u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#263 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#263 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0264() {
        // S=100.0, K=110.0, T=1.0, r=0.0
        // v0=0.09, kappa=1.0, theta=0.16, xi=0.1, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 1000000000000u128, 160000000000u128, 100000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 9489037804809u128;
        let exp_put = 19489037804809u128;
        let tol = 460000000000u128; // $0.46
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#264 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#264 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0265() {
        // S=100.0, K=80.0, T=0.25, r=0.0
        // v0=0.09, kappa=1.0, theta=0.16, xi=0.2, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 1000000000000u128, 160000000000u128, 200000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 20616659453161u128;
        let exp_put = 616659453161u128;
        let tol = 460000000000u128; // $0.46
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#265 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#265 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0266() {
        // S=100.0, K=95.0, T=1.0, r=0.0
        // v0=0.09, kappa=1.0, theta=0.16, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 1000000000000u128, 160000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 15581518623246u128;
        let exp_put = 10581518623246u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#266 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#266 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0267() {
        // S=100.0, K=105.0, T=0.1, r=0.0
        // v0=0.09, kappa=1.0, theta=0.16, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 1000000000000u128, 160000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 1666758967794u128;
        let exp_put = 6666758967794u128;
        let tol = 450000000000u128; // $0.45
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#267 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#267 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0268() {
        // S=100.0, K=115.0, T=0.5, r=0.0
        // v0=0.09, kappa=1.0, theta=0.16, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 1000000000000u128, 160000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 3898967665072u128;
        let exp_put = 18898967665072u128;
        let tol = 289130434783u128; // $0.29
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#268 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#268 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0269() {
        // S=100.0, K=80.0, T=0.1, r=0.0
        // v0=0.09, kappa=1.0, theta=0.16, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 1000000000000u128, 160000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 20052083120541u128;
        let exp_put = 52083120541u128;
        let tol = 625000000000u128; // $0.62
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#269 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#269 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0270() {
        // S=100.0, K=90.0, T=0.5, r=0.0
        // v0=0.09, kappa=2.0, theta=0.01, xi=0.1, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 2000000000000u128, 10000000000u128, 100000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 12791779265727u128;
        let exp_put = 2791779265727u128;
        let tol = 640000000000u128; // $0.64
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#270 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#270 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0271() {
        // S=100.0, K=100.0, T=2.0, r=0.0
        // v0=0.09, kappa=2.0, theta=0.01, xi=0.2, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 2000000000000u128, 10000000000u128, 200000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 9408887841441u128;
        let exp_put = 9408887841441u128;
        let tol = 80000000000u128; // $0.08
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#271 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#271 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0272() {
        // S=100.0, K=110.0, T=0.25, r=0.0
        // v0=0.09, kappa=2.0, theta=0.01, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 2000000000000u128, 10000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 1752280036372u128;
        let exp_put = 11752280036372u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#272 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#272 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0273() {
        // S=100.0, K=120.0, T=1.0, r=0.0
        // v0=0.09, kappa=2.0, theta=0.01, xi=0.5, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 2000000000000u128, 10000000000u128, 500000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 854582779852u128;
        let exp_put = 20854582779852u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#273 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#273 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0274() {
        // S=100.0, K=85.0, T=0.25, r=0.0
        // v0=0.09, kappa=2.0, theta=0.01, xi=0.8, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 2000000000000u128, 10000000000u128, 800000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 16190818041198u128;
        let exp_put = 1190818041198u128;
        let tol = 602941176471u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#274 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#274 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0275() {
        // S=100.0, K=95.0, T=1.0, r=0.0
        // v0=0.09, kappa=2.0, theta=0.04, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 2000000000000u128, 40000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 12352122787586u128;
        let exp_put = 7352122787586u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#275 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#275 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0276() {
        // S=100.0, K=105.0, T=0.1, r=0.0
        // v0=0.09, kappa=2.0, theta=0.04, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 2000000000000u128, 40000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 1675884047078u128;
        let exp_put = 6675884047078u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#276 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#276 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0277() {
        // S=100.0, K=120.0, T=0.5, r=0.0
        // v0=0.09, kappa=2.0, theta=0.04, xi=0.2, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 2000000000000u128, 40000000000u128, 200000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 1837081011619u128;
        let exp_put = 21837081011619u128;
        let tol = 100000000000u128; // $0.10
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#277 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#277 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0278() {
        // S=100.0, K=85.0, T=0.1, r=0.0
        // v0=0.09, kappa=2.0, theta=0.04, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 2000000000000u128, 40000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 15135418300611u128;
        let exp_put = 135418300611u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#278 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#278 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0279() {
        // S=100.0, K=95.0, T=0.5, r=0.0
        // v0=0.09, kappa=2.0, theta=0.04, xi=0.5, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 2000000000000u128, 40000000000u128, 500000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 9954465086124u128;
        let exp_put = 4954465086124u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#279 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#279 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0280() {
        // S=100.0, K=105.0, T=2.0, r=0.0
        // v0=0.09, kappa=2.0, theta=0.04, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 2000000000000u128, 40000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 8695499475025u128;
        let exp_put = 13695499475025u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#280 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#280 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0281() {
        // S=100.0, K=115.0, T=0.25, r=0.0
        // v0=0.09, kappa=2.0, theta=0.09, xi=0.1, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 2000000000000u128, 90000000000u128, 100000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 1439913648866u128;
        let exp_put = 16439913648866u128;
        let tol = 400000000000u128; // $0.40
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#281 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#281 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0282() {
        // S=100.0, K=85.0, T=2.0, r=0.0
        // v0=0.09, kappa=2.0, theta=0.09, xi=0.2, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 2000000000000u128, 90000000000u128, 200000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 24215039987487u128;
        let exp_put = 9215039987487u128;
        let tol = 102941176471u128; // $0.10
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#282 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#282 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0283() {
        // S=100.0, K=95.0, T=0.25, r=0.0
        // v0=0.09, kappa=2.0, theta=0.09, xi=0.3, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 2000000000000u128, 90000000000u128, 300000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 8730034588369u128;
        let exp_put = 3730034588369u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#283 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#283 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0284() {
        // S=100.0, K=105.0, T=1.0, r=0.0
        // v0=0.09, kappa=2.0, theta=0.09, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 2000000000000u128, 90000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 8628403902371u128;
        let exp_put = 13628403902371u128;
        let tol = 450000000000u128; // $0.45
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#284 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#284 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0285() {
        // S=100.0, K=120.0, T=0.1, r=0.0
        // v0=0.09, kappa=2.0, theta=0.09, xi=0.8, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 2000000000000u128, 90000000000u128, 800000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 490738126u128;
        let exp_put = 20000490738126u128;
        let tol = 800000000000u128; // $0.80
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#285 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#285 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0286() {
        // S=100.0, K=85.0, T=1.0, r=0.0
        // v0=0.09, kappa=2.0, theta=0.09, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 2000000000000u128, 90000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 19655181163146u128;
        let exp_put = 4655181163146u128;
        let tol = 602941176471u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#286 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#286 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0287() {
        // S=100.0, K=95.0, T=0.1, r=0.0
        // v0=0.09, kappa=2.0, theta=0.16, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 2000000000000u128, 160000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 6803892172681u128;
        let exp_put = 1803892172681u128;
        let tol = 610000000000u128; // $0.61
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#287 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#287 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0288() {
        // S=100.0, K=110.0, T=0.5, r=0.0
        // v0=0.09, kappa=2.0, theta=0.16, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 2000000000000u128, 160000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 5682163015486u128;
        let exp_put = 15682163015486u128;
        let tol = 70000000000u128; // $0.07
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#288 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#288 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0289() {
        // S=100.0, K=120.0, T=2.0, r=0.0
        // v0=0.09, kappa=2.0, theta=0.16, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 2000000000000u128, 160000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 13687241467521u128;
        let exp_put = 33687241467521u128;
        let tol = 100000000000u128; // $0.10
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#289 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#289 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0290() {
        // S=100.0, K=85.0, T=0.5, r=0.0
        // v0=0.09, kappa=2.0, theta=0.16, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 2000000000000u128, 160000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 18510404441865u128;
        let exp_put = 3510404441865u128;
        let tol = 302941176471u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#290 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#290 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0291() {
        // S=100.0, K=95.0, T=2.0, r=0.0
        // v0=0.09, kappa=2.0, theta=0.16, xi=0.8, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 2000000000000u128, 160000000000u128, 800000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 21621712353191u128;
        let exp_put = 16621712353191u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#291 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#291 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0292() {
        // S=100.0, K=105.0, T=0.25, r=0.0
        // v0=0.09, kappa=3.0, theta=0.01, xi=0.1, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 3000000000000u128, 10000000000u128, 100000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 3059510806000u128;
        let exp_put = 8059510806000u128;
        let tol = 790000000000u128; // $0.79
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#292 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#292 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0293() {
        // S=100.0, K=115.0, T=1.0, r=0.0
        // v0=0.09, kappa=3.0, theta=0.01, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 3000000000000u128, 10000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 1930746098199u128;
        let exp_put = 16930746098199u128;
        let tol = 289130434783u128; // $0.29
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#293 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#293 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0294() {
        // S=100.0, K=80.0, T=0.25, r=0.0
        // v0=0.09, kappa=3.0, theta=0.01, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 3000000000000u128, 10000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 20389236694888u128;
        let exp_put = 389236694888u128;
        let tol = 325000000000u128; // $0.33
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#294 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#294 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0295() {
        // S=100.0, K=90.0, T=1.0, r=0.0
        // v0=0.09, kappa=3.0, theta=0.01, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 3000000000000u128, 10000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 13106948486329u128;
        let exp_put = 3106948486329u128;
        let tol = 180000000000u128; // $0.18
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#295 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#295 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0296() {
        // S=100.0, K=100.0, T=0.1, r=0.0
        // v0=0.09, kappa=3.0, theta=0.01, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 3000000000000u128, 10000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 3486775262935u128;
        let exp_put = 3486775262935u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#296 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#296 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0297() {
        // S=100.0, K=110.0, T=0.5, r=0.0
        // v0=0.09, kappa=3.0, theta=0.01, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 3000000000000u128, 10000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 2101145561757u128;
        let exp_put = 12101145561757u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#297 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#297 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0298() {
        // S=100.0, K=120.0, T=2.0, r=0.0
        // v0=0.09, kappa=3.0, theta=0.04, xi=0.1, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 3000000000000u128, 40000000000u128, 100000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 5627921006745u128;
        let exp_put = 25627921006745u128;
        let tol = 180000000000u128; // $0.18
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#298 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#298 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0299() {
        // S=100.0, K=90.0, T=0.5, r=0.0
        // v0=0.09, kappa=3.0, theta=0.04, xi=0.2, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 3000000000000u128, 40000000000u128, 200000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 13092396883726u128;
        let exp_put = 3092396883726u128;
        let tol = 180000000000u128; // $0.18
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#299 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#299 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0300() {
        // S=100.0, K=100.0, T=2.0, r=0.0
        // v0=0.09, kappa=3.0, theta=0.04, xi=0.3, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 3000000000000u128, 40000000000u128, 300000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 12007829681873u128;
        let exp_put = 12007829681873u128;
        let tol = 180000000000u128; // $0.18
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#300 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#300 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0301() {
        // S=100.0, K=110.0, T=0.25, r=0.0
        // v0=0.09, kappa=3.0, theta=0.04, xi=0.5, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 3000000000000u128, 40000000000u128, 500000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 1516974801003u128;
        let exp_put = 11516974801003u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#301 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#301 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0302() {
        // S=100.0, K=120.0, T=1.0, r=0.0
        // v0=0.09, kappa=3.0, theta=0.04, xi=0.8, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 3000000000000u128, 40000000000u128, 800000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 657611714374u128;
        let exp_put = 20657611714374u128;
        let tol = 800000000000u128; // $0.80
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#302 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#302 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0303() {
        // S=100.0, K=85.0, T=0.25, r=0.0
        // v0=0.09, kappa=3.0, theta=0.09, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 3000000000000u128, 90000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 16062913809297u128;
        let exp_put = 1062913809297u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#303 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#303 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0304() {
        // S=100.0, K=100.0, T=1.0, r=0.0
        // v0=0.09, kappa=3.0, theta=0.09, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 3000000000000u128, 90000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 11913617178364u128;
        let exp_put = 11913617178364u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#304 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#304 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0305() {
        // S=100.0, K=110.0, T=0.1, r=0.0
        // v0=0.09, kappa=3.0, theta=0.09, xi=0.2, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 3000000000000u128, 90000000000u128, 200000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 804758113482u128;
        let exp_put = 10804758113482u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#305 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#305 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0306() {
        // S=100.0, K=120.0, T=0.5, r=0.0
        // v0=0.09, kappa=3.0, theta=0.09, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 3000000000000u128, 90000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 2272319182776u128;
        let exp_put = 22272319182776u128;
        let tol = 180000000000u128; // $0.18
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#306 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#306 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0307() {
        // S=100.0, K=85.0, T=0.1, r=0.0
        // v0=0.09, kappa=3.0, theta=0.09, xi=0.5, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 3000000000000u128, 90000000000u128, 500000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 15207042579543u128;
        let exp_put = 207042579543u128;
        let tol = 302941176471u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#307 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#307 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0308() {
        // S=100.0, K=95.0, T=0.5, r=0.0
        // v0=0.09, kappa=3.0, theta=0.09, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 3000000000000u128, 90000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 10714513365136u128;
        let exp_put = 5714513365136u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#308 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#308 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0309() {
        // S=100.0, K=105.0, T=2.0, r=0.0
        // v0=0.09, kappa=3.0, theta=0.16, xi=0.1, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 3000000000000u128, 160000000000u128, 100000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 19442688398693u128;
        let exp_put = 24442688398693u128;
        let tol = 180000000000u128; // $0.18
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#309 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#309 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0310() {
        // S=100.0, K=115.0, T=0.25, r=0.0
        // v0=0.09, kappa=3.0, theta=0.16, xi=0.2, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 3000000000000u128, 160000000000u128, 200000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 1795921316274u128;
        let exp_put = 16795921316274u128;
        let tol = 760000000000u128; // $0.76
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#310 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#310 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0311() {
        // S=100.0, K=80.0, T=2.0, r=0.0
        // v0=0.09, kappa=3.0, theta=0.16, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 3000000000000u128, 160000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 30941759062393u128;
        let exp_put = 10941759062393u128;
        let tol = 325000000000u128; // $0.33
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#311 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#311 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0312() {
        // S=100.0, K=90.0, T=0.25, r=0.0
        // v0=0.09, kappa=3.0, theta=0.16, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 3000000000000u128, 160000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 12786280175535u128;
        let exp_put = 2786280175535u128;
        let tol = 483333333333u128; // $0.48
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#312 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#312 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0313() {
        // S=100.0, K=100.0, T=1.0, r=0.0
        // v0=0.09, kappa=3.0, theta=0.16, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 3000000000000u128, 160000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 14544542930660u128;
        let exp_put = 14544542930660u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#313 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#313 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0314() {
        // S=100.0, K=110.0, T=0.1, r=0.0
        // v0=0.09, kappa=3.0, theta=0.16, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 3000000000000u128, 160000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 926360342562u128;
        let exp_put = 10926360342562u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#314 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#314 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0315() {
        // S=100.0, K=120.0, T=0.5, r=0.0
        // v0=0.09, kappa=5.0, theta=0.01, xi=0.1, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 5000000000000u128, 10000000000u128, 100000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 643664532334u128;
        let exp_put = 20643664532334u128;
        let tol = 1090000000000u128; // $1.09
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#315 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#315 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0316() {
        // S=100.0, K=90.0, T=0.1, r=0.0
        // v0=0.09, kappa=5.0, theta=0.01, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 5000000000000u128, 10000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 10444626343870u128;
        let exp_put = 444626343870u128;
        let tol = 1090000000000u128; // $1.09
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#316 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#316 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0317() {
        // S=100.0, K=100.0, T=0.5, r=0.0
        // v0=0.09, kappa=5.0, theta=0.01, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 5000000000000u128, 10000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 5478679773196u128;
        let exp_put = 5478679773196u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#317 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#317 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0318() {
        // S=100.0, K=110.0, T=2.0, r=0.0
        // v0=0.09, kappa=5.0, theta=0.01, xi=0.5, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 5000000000000u128, 10000000000u128, 500000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 3000145635923u128;
        let exp_put = 13000145635923u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#318 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#318 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0319() {
        // S=100.0, K=120.0, T=0.25, r=0.0
        // v0=0.09, kappa=5.0, theta=0.01, xi=0.8, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 5000000000000u128, 10000000000u128, 800000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 44309020906u128;
        let exp_put = 20044309020906u128;
        let tol = 600000000000u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#319 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#319 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0320() {
        // S=100.0, K=85.0, T=2.0, r=0.0
        // v0=0.09, kappa=5.0, theta=0.04, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 5000000000000u128, 40000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 20185031582566u128;
        let exp_put = 5185031582566u128;
        let tol = 302941176471u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#320 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#320 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0321() {
        // S=100.0, K=95.0, T=0.25, r=0.0
        // v0=0.09, kappa=5.0, theta=0.04, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 5000000000000u128, 40000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 8048565765560u128;
        let exp_put = 3048565765560u128;
        let tol = 1000000000000u128; // $1.00
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#321 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#321 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0322() {
        // S=100.0, K=105.0, T=1.0, r=0.0
        // v0=0.09, kappa=5.0, theta=0.04, xi=0.2, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 5000000000000u128, 40000000000u128, 200000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 6808692217506u128;
        let exp_put = 11808692217506u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#322 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#322 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0323() {
        // S=100.0, K=115.0, T=0.1, r=0.0
        // v0=0.09, kappa=5.0, theta=0.04, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 5000000000000u128, 40000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 237392356782u128;
        let exp_put = 15237392356782u128;
        let tol = 1000000000000u128; // $1.00
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#323 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#323 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0324() {
        // S=100.0, K=80.0, T=1.0, r=0.0
        // v0=0.09, kappa=5.0, theta=0.04, xi=0.5, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 5000000000000u128, 40000000000u128, 500000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 21863158652403u128;
        let exp_put = 1863158652403u128;
        let tol = 325000000000u128; // $0.33
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#324 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#324 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0325() {
        // S=100.0, K=90.0, T=0.1, r=0.0
        // v0=0.09, kappa=5.0, theta=0.04, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 5000000000000u128, 40000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 10594366318603u128;
        let exp_put = 594366318603u128;
        let tol = 583333333333u128; // $0.58
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#325 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#325 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0326() {
        // S=100.0, K=100.0, T=0.5, r=0.0
        // v0=0.09, kappa=5.0, theta=0.09, xi=0.1, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 5000000000000u128, 90000000000u128, 100000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 8417415448363u128;
        let exp_put = 8417415448363u128;
        let tol = 850000000000u128; // $0.85
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#326 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#326 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0327() {
        // S=100.0, K=115.0, T=2.0, r=0.0
        // v0=0.09, kappa=5.0, theta=0.09, xi=0.2, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 5000000000000u128, 90000000000u128, 200000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 11223755393053u128;
        let exp_put = 26223755393053u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#327 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#327 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0328() {
        // S=100.0, K=80.0, T=0.5, r=0.0
        // v0=0.09, kappa=5.0, theta=0.09, xi=0.3, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 5000000000000u128, 90000000000u128, 300000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 21654554696428u128;
        let exp_put = 1654554696428u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#328 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#328 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0329() {
        // S=100.0, K=90.0, T=2.0, r=0.0
        // v0=0.09, kappa=5.0, theta=0.09, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 2000000000000u128,
            90000000000u128, 5000000000000u128, 90000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 21345455454080u128;
        let exp_put = 11345455454080u128;
        let tol = 483333333333u128; // $0.48
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#329 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#329 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0330() {
        // S=100.0, K=100.0, T=0.25, r=0.0
        // v0=0.09, kappa=5.0, theta=0.09, xi=0.8, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 5000000000000u128, 90000000000u128, 800000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 5668000454222u128;
        let exp_put = 5668000454222u128;
        let tol = 750000000000u128; // $0.75
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#330 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#330 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0331() {
        // S=100.0, K=110.0, T=1.0, r=0.0
        // v0=0.09, kappa=5.0, theta=0.09, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 5000000000000u128, 90000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 7895817221501u128;
        let exp_put = 17895817221501u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#331 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#331 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0332() {
        // S=100.0, K=80.0, T=0.25, r=0.0
        // v0=0.09, kappa=5.0, theta=0.16, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 250000000000u128,
            90000000000u128, 5000000000000u128, 160000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 20717960109842u128;
        let exp_put = 717960109842u128;
        let tol = 1060000000000u128; // $1.06
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#332 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#332 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0333() {
        // S=100.0, K=95.0, T=1.0, r=0.0
        // v0=0.09, kappa=5.0, theta=0.16, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 1000000000000u128,
            90000000000u128, 5000000000000u128, 160000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 17383918769463u128;
        let exp_put = 12383918769463u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#333 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#333 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0334() {
        // S=100.0, K=105.0, T=0.1, r=0.0
        // v0=0.09, kappa=5.0, theta=0.16, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 5000000000000u128, 160000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 2081433901951u128;
        let exp_put = 7081433901951u128;
        let tol = 1060000000000u128; // $1.06
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#334 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#334 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0335() {
        // S=100.0, K=115.0, T=0.5, r=0.0
        // v0=0.09, kappa=5.0, theta=0.16, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 500000000000u128,
            90000000000u128, 5000000000000u128, 160000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 4695387344320u128;
        let exp_put = 19695387344320u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#335 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#335 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0336() {
        // S=100.0, K=80.0, T=0.1, r=0.0
        // v0=0.09, kappa=5.0, theta=0.16, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 100000000000u128,
            90000000000u128, 5000000000000u128, 160000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 20122615297806u128;
        let exp_put = 122615297806u128;
        let tol = 625000000000u128; // $0.62
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#336 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#336 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0337() {
        // S=100.0, K=90.0, T=0.5, r=0.0
        // v0=0.16, kappa=0.5, theta=0.01, xi=0.1, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 500000000000u128,
            160000000000u128, 500000000000u128, 10000000000u128, 100000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 15917524346743u128;
        let exp_put = 5917524346743u128;
        let tol = 625000000000u128; // $0.62
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#337 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#337 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0338() {
        // S=100.0, K=100.0, T=2.0, r=0.0
        // v0=0.16, kappa=0.5, theta=0.01, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 500000000000u128, 10000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 17135116367727u128;
        let exp_put = 17135116367727u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#338 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#338 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0339() {
        // S=100.0, K=110.0, T=0.25, r=0.0
        // v0=0.16, kappa=0.5, theta=0.01, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 500000000000u128, 10000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 3668860486732u128;
        let exp_put = 13668860486732u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#339 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#339 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0340() {
        // S=100.0, K=120.0, T=1.0, r=0.0
        // v0=0.16, kappa=0.5, theta=0.01, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 500000000000u128, 10000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 7352534514818u128;
        let exp_put = 27352534514818u128;
        let tol = 100000000000u128; // $0.10
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#340 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#340 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0341() {
        // S=100.0, K=85.0, T=0.25, r=0.0
        // v0=0.16, kappa=0.5, theta=0.01, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 500000000000u128, 10000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 16985120886159u128;
        let exp_put = 1985120886159u128;
        let tol = 302941176471u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#341 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#341 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0342() {
        // S=100.0, K=95.0, T=1.0, r=0.0
        // v0=0.16, kappa=0.5, theta=0.01, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 500000000000u128, 10000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 14652431061734u128;
        let exp_put = 9652431061734u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#342 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#342 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0343() {
        // S=100.0, K=105.0, T=0.1, r=0.0
        // v0=0.16, kappa=0.5, theta=0.04, xi=0.1, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 500000000000u128, 40000000000u128, 100000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 2958285662886u128;
        let exp_put = 7958285662886u128;
        let tol = 535000000000u128; // $0.53
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#343 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#343 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0344() {
        // S=100.0, K=115.0, T=0.5, r=0.0
        // v0=0.16, kappa=0.5, theta=0.04, xi=0.2, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 500000000000u128,
            160000000000u128, 500000000000u128, 40000000000u128, 200000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 5216477271098u128;
        let exp_put = 20216477271098u128;
        let tol = 89130434783u128; // $0.09
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#344 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#344 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0345() {
        // S=100.0, K=80.0, T=0.1, r=0.0
        // v0=0.16, kappa=0.5, theta=0.04, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 500000000000u128, 40000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 20210489076124u128;
        let exp_put = 210489076124u128;
        let tol = 535000000000u128; // $0.53
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#345 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#345 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0346() {
        // S=100.0, K=90.0, T=0.5, r=0.0
        // v0=0.16, kappa=0.5, theta=0.04, xi=0.5, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 500000000000u128,
            160000000000u128, 500000000000u128, 40000000000u128, 500000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 16082503501803u128;
        let exp_put = 6082503501803u128;
        let tol = 283333333333u128; // $0.28
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#346 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#346 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0347() {
        // S=100.0, K=100.0, T=2.0, r=0.0
        // v0=0.16, kappa=0.5, theta=0.04, xi=0.8, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 500000000000u128, 40000000000u128, 800000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 12828955203833u128;
        let exp_put = 12828955203833u128;
        let tol = 750000000000u128; // $0.75
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#347 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#347 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0348() {
        // S=100.0, K=115.0, T=0.25, r=0.0
        // v0=0.16, kappa=0.5, theta=0.09, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 500000000000u128, 90000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 2815478823903u128;
        let exp_put = 17815478823903u128;
        let tol = 385000000000u128; // $0.39
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#348 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#348 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0349() {
        // S=100.0, K=80.0, T=2.0, r=0.0
        // v0=0.16, kappa=0.5, theta=0.09, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 500000000000u128, 90000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 29985274082798u128;
        let exp_put = 9985274082798u128;
        let tol = 125000000000u128; // $0.12
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#349 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#349 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0350() {
        // S=100.0, K=90.0, T=0.25, r=0.0
        // v0=0.16, kappa=0.5, theta=0.09, xi=0.2, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 500000000000u128, 90000000000u128, 200000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 13482668718421u128;
        let exp_put = 3482668718421u128;
        let tol = 385000000000u128; // $0.39
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#350 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#350 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0351() {
        // S=100.0, K=100.0, T=1.0, r=0.0
        // v0=0.16, kappa=0.5, theta=0.09, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 500000000000u128, 90000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 14675406882212u128;
        let exp_put = 14675406882212u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#351 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#351 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0352() {
        // S=100.0, K=110.0, T=0.1, r=0.0
        // v0=0.16, kappa=0.5, theta=0.09, xi=0.5, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 500000000000u128, 90000000000u128, 500000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 1573546901652u128;
        let exp_put = 11573546901652u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#352 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#352 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0353() {
        // S=100.0, K=120.0, T=0.5, r=0.0
        // v0=0.16, kappa=0.5, theta=0.09, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 500000000000u128,
            160000000000u128, 500000000000u128, 90000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 2945425727600u128;
        let exp_put = 22945425727600u128;
        let tol = 600000000000u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#353 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#353 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0354() {
        // S=100.0, K=85.0, T=0.1, r=0.0
        // v0=0.16, kappa=0.5, theta=0.16, xi=0.1, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 500000000000u128, 160000000000u128, 100000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 15557449544643u128;
        let exp_put = 557449544643u128;
        let tol = 302941176471u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#354 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#354 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0355() {
        // S=100.0, K=95.0, T=0.5, r=0.0
        // v0=0.16, kappa=0.5, theta=0.16, xi=0.2, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 500000000000u128,
            160000000000u128, 500000000000u128, 160000000000u128, 200000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 13612026102483u128;
        let exp_put = 8612026102483u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#355 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#355 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0356() {
        // S=100.0, K=105.0, T=2.0, r=0.0
        // v0=0.16, kappa=0.5, theta=0.16, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 500000000000u128, 160000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 18466799734656u128;
        let exp_put = 23466799734656u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#356 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#356 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0357() {
        // S=100.0, K=115.0, T=0.25, r=0.0
        // v0=0.16, kappa=0.5, theta=0.16, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 500000000000u128, 160000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 2209295852399u128;
        let exp_put = 17209295852399u128;
        let tol = 489130434783u128; // $0.49
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#357 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#357 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0358() {
        // S=100.0, K=80.0, T=2.0, r=0.0
        // v0=0.16, kappa=0.5, theta=0.16, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 500000000000u128, 160000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 30610911047230u128;
        let exp_put = 10610911047230u128;
        let tol = 325000000000u128; // $0.33
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#358 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#358 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0359() {
        // S=100.0, K=90.0, T=0.25, r=0.0
        // v0=0.16, kappa=0.5, theta=0.16, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 500000000000u128, 160000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 13416279806562u128;
        let exp_put = 3416279806562u128;
        let tol = 583333333333u128; // $0.58
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#359 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#359 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0360() {
        // S=100.0, K=100.0, T=1.0, r=0.0
        // v0=0.16, kappa=1.0, theta=0.01, xi=0.1, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 1000000000000u128, 10000000000u128, 100000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 12789512065334u128;
        let exp_put = 12789512065334u128;
        let tol = 700000000000u128; // $0.70
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#360 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#360 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0361() {
        // S=100.0, K=110.0, T=0.1, r=0.0
        // v0=0.16, kappa=1.0, theta=0.01, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 1000000000000u128, 10000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 1572431138366u128;
        let exp_put = 11572431138366u128;
        let tol = 700000000000u128; // $0.70
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#361 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#361 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0362() {
        // S=100.0, K=120.0, T=0.5, r=0.0
        // v0=0.16, kappa=1.0, theta=0.01, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 500000000000u128,
            160000000000u128, 1000000000000u128, 10000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 3249102369171u128;
        let exp_put = 23249102369171u128;
        let tol = 100000000000u128; // $0.10
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#362 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#362 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0363() {
        // S=100.0, K=85.0, T=0.1, r=0.0
        // v0=0.16, kappa=1.0, theta=0.01, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 1000000000000u128, 10000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 15606858426463u128;
        let exp_put = 606858426463u128;
        let tol = 302941176471u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#363 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#363 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0364() {
        // S=100.0, K=95.0, T=0.5, r=0.0
        // v0=0.16, kappa=1.0, theta=0.01, xi=0.8, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 500000000000u128,
            160000000000u128, 1000000000000u128, 10000000000u128, 800000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 11991024057293u128;
        let exp_put = 6991024057293u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#364 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#364 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0365() {
        // S=100.0, K=105.0, T=2.0, r=0.0
        // v0=0.16, kappa=1.0, theta=0.04, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 1000000000000u128, 40000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 14575044767493u128;
        let exp_put = 19575044767493u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#365 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#365 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0366() {
        // S=100.0, K=115.0, T=0.25, r=0.0
        // v0=0.16, kappa=1.0, theta=0.04, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 1000000000000u128, 40000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 2456960464423u128;
        let exp_put = 17456960464423u128;
        let tol = 610000000000u128; // $0.61
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#366 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#366 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0367() {
        // S=100.0, K=80.0, T=2.0, r=0.0
        // v0=0.16, kappa=1.0, theta=0.04, xi=0.2, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 1000000000000u128, 40000000000u128, 200000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 27137611540144u128;
        let exp_put = 7137611540144u128;
        let tol = 125000000000u128; // $0.12
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#367 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#367 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0368() {
        // S=100.0, K=90.0, T=0.25, r=0.0
        // v0=0.16, kappa=1.0, theta=0.04, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 1000000000000u128, 40000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 13269693682429u128;
        let exp_put = 3269693682429u128;
        let tol = 283333333333u128; // $0.28
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#368 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#368 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0369() {
        // S=100.0, K=100.0, T=1.0, r=0.0
        // v0=0.16, kappa=1.0, theta=0.04, xi=0.5, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 1000000000000u128, 40000000000u128, 500000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 12653904763652u128;
        let exp_put = 12653904763652u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#369 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#369 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0370() {
        // S=100.0, K=110.0, T=0.1, r=0.0
        // v0=0.16, kappa=1.0, theta=0.04, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 1000000000000u128, 40000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 1453540711057u128;
        let exp_put = 11453540711057u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#370 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#370 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0371() {
        // S=100.0, K=120.0, T=0.5, r=0.0
        // v0=0.16, kappa=1.0, theta=0.09, xi=0.1, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 500000000000u128,
            160000000000u128, 1000000000000u128, 90000000000u128, 100000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 4188914270777u128;
        let exp_put = 24188914270777u128;
        let tol = 460000000000u128; // $0.46
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#371 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#371 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0372() {
        // S=100.0, K=85.0, T=0.1, r=0.0
        // v0=0.16, kappa=1.0, theta=0.09, xi=0.2, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 1000000000000u128, 90000000000u128, 200000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 15559846862115u128;
        let exp_put = 559846862115u128;
        let tol = 460000000000u128; // $0.46
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#372 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#372 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0373() {
        // S=100.0, K=95.0, T=0.5, r=0.0
        // v0=0.16, kappa=1.0, theta=0.09, xi=0.3, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 500000000000u128,
            160000000000u128, 1000000000000u128, 90000000000u128, 300000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 13080564299984u128;
        let exp_put = 8080564299984u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#373 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#373 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0374() {
        // S=100.0, K=105.0, T=2.0, r=0.0
        // v0=0.16, kappa=1.0, theta=0.09, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 1000000000000u128, 90000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 14552434925316u128;
        let exp_put = 19552434925316u128;
        let tol = 450000000000u128; // $0.45
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#374 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#374 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0375() {
        // S=100.0, K=115.0, T=0.25, r=0.0
        // v0=0.16, kappa=1.0, theta=0.09, xi=0.8, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 1000000000000u128, 90000000000u128, 800000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 1424167034493u128;
        let exp_put = 16424167034493u128;
        let tol = 789130434783u128; // $0.79
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#375 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#375 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0376() {
        // S=100.0, K=80.0, T=2.0, r=0.0
        // v0=0.16, kappa=1.0, theta=0.09, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 1000000000000u128, 90000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 27932441650788u128;
        let exp_put = 7932441650788u128;
        let tol = 625000000000u128; // $0.62
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#376 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#376 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0377() {
        // S=100.0, K=90.0, T=0.25, r=0.0
        // v0=0.16, kappa=1.0, theta=0.16, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 1000000000000u128, 160000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 13577536812077u128;
        let exp_put = 3577536812077u128;
        let tol = 283333333333u128; // $0.28
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#377 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#377 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0378() {
        // S=100.0, K=100.0, T=1.0, r=0.0
        // v0=0.16, kappa=1.0, theta=0.16, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 1000000000000u128, 160000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 15681697676496u128;
        let exp_put = 15681697676496u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#378 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#378 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0379() {
        // S=100.0, K=110.0, T=0.1, r=0.0
        // v0=0.16, kappa=1.0, theta=0.16, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 1000000000000u128, 160000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 1640861214558u128;
        let exp_put = 11640861214558u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#379 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#379 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0380() {
        // S=100.0, K=120.0, T=0.5, r=0.0
        // v0=0.16, kappa=1.0, theta=0.16, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 500000000000u128,
            160000000000u128, 1000000000000u128, 160000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 3926019836137u128;
        let exp_put = 23926019836137u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#380 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#380 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0381() {
        // S=100.0, K=85.0, T=0.1, r=0.0
        // v0=0.16, kappa=1.0, theta=0.16, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 1000000000000u128, 160000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 15718028854316u128;
        let exp_put = 718028854316u128;
        let tol = 602941176471u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#381 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#381 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0382() {
        // S=100.0, K=95.0, T=0.5, r=0.0
        // v0=0.16, kappa=2.0, theta=0.01, xi=0.1, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 500000000000u128,
            160000000000u128, 2000000000000u128, 10000000000u128, 100000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 11618916484509u128;
        let exp_put = 6618916484509u128;
        let tol = 850000000000u128; // $0.85
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#382 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#382 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0383() {
        // S=100.0, K=105.0, T=2.0, r=0.0
        // v0=0.16, kappa=2.0, theta=0.01, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 2000000000000u128, 10000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 9433711016946u128;
        let exp_put = 14433711016946u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#383 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#383 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0384() {
        // S=100.0, K=115.0, T=0.25, r=0.0
        // v0=0.16, kappa=2.0, theta=0.01, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 2000000000000u128, 10000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 1910605473832u128;
        let exp_put = 16910605473832u128;
        let tol = 289130434783u128; // $0.29
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#384 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#384 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0385() {
        // S=100.0, K=80.0, T=2.0, r=0.0
        // v0=0.16, kappa=2.0, theta=0.01, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 2000000000000u128, 10000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 23606751857644u128;
        let exp_put = 3606751857644u128;
        let tol = 150000000000u128; // $0.15
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#385 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#385 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0386() {
        // S=100.0, K=90.0, T=0.25, r=0.0
        // v0=0.16, kappa=2.0, theta=0.01, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 2000000000000u128, 10000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 12846880499352u128;
        let exp_put = 2846880499352u128;
        let tol = 283333333333u128; // $0.28
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#386 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#386 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0387() {
        // S=100.0, K=100.0, T=1.0, r=0.0
        // v0=0.16, kappa=2.0, theta=0.01, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 2000000000000u128, 10000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 9543780405216u128;
        let exp_put = 9543780405216u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#387 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#387 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0388() {
        // S=100.0, K=110.0, T=0.1, r=0.0
        // v0=0.16, kappa=2.0, theta=0.04, xi=0.1, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 2000000000000u128, 40000000000u128, 100000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 1544118699716u128;
        let exp_put = 11544118699716u128;
        let tol = 760000000000u128; // $0.76
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#388 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#388 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0389() {
        // S=100.0, K=80.0, T=1.0, r=0.0
        // v0=0.16, kappa=2.0, theta=0.04, xi=0.2, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 2000000000000u128, 40000000000u128, 200000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 23824965620189u128;
        let exp_put = 3824965620189u128;
        let tol = 125000000000u128; // $0.12
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#389 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#389 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0390() {
        // S=100.0, K=90.0, T=0.1, r=0.0
        // v0=0.16, kappa=2.0, theta=0.04, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 2000000000000u128, 40000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 11289759528583u128;
        let exp_put = 1289759528583u128;
        let tol = 760000000000u128; // $0.76
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#390 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#390 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0391() {
        // S=100.0, K=100.0, T=0.5, r=0.0
        // v0=0.16, kappa=2.0, theta=0.04, xi=0.5, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 500000000000u128,
            160000000000u128, 2000000000000u128, 40000000000u128, 500000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 9158671428307u128;
        let exp_put = 9158671428307u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#391 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#391 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0392() {
        // S=100.0, K=110.0, T=2.0, r=0.0
        // v0=0.16, kappa=2.0, theta=0.04, xi=0.8, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 2000000000000u128, 40000000000u128, 800000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 7166862064717u128;
        let exp_put = 17166862064717u128;
        let tol = 750000000000u128; // $0.75
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#392 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#392 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0393() {
        // S=100.0, K=120.0, T=0.25, r=0.0
        // v0=0.16, kappa=2.0, theta=0.09, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 2000000000000u128, 90000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 1726492017566u128;
        let exp_put = 21726492017566u128;
        let tol = 610000000000u128; // $0.61
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#393 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#393 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0394() {
        // S=100.0, K=85.0, T=2.0, r=0.0
        // v0=0.16, kappa=2.0, theta=0.09, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 2000000000000u128, 90000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 25431360729475u128;
        let exp_put = 10431360729475u128;
        let tol = 102941176471u128; // $0.10
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#394 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#394 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0395() {
        // S=100.0, K=95.0, T=0.25, r=0.0
        // v0=0.16, kappa=2.0, theta=0.09, xi=0.2, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 2000000000000u128, 90000000000u128, 200000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 10139754535331u128;
        let exp_put = 5139754535331u128;
        let tol = 610000000000u128; // $0.61
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#395 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#395 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0396() {
        // S=100.0, K=105.0, T=1.0, r=0.0
        // v0=0.16, kappa=2.0, theta=0.09, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 2000000000000u128, 90000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 11446597343437u128;
        let exp_put = 16446597343437u128;
        let tol = 70000000000u128; // $0.07
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#396 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#396 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0397() {
        // S=100.0, K=115.0, T=0.1, r=0.0
        // v0=0.16, kappa=2.0, theta=0.09, xi=0.5, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 2000000000000u128, 90000000000u128, 500000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 752316771406u128;
        let exp_put = 15752316771406u128;
        let tol = 289130434783u128; // $0.29
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#397 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#397 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0398() {
        // S=100.0, K=80.0, T=1.0, r=0.0
        // v0=0.16, kappa=2.0, theta=0.09, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 2000000000000u128, 90000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 25034994863481u128;
        let exp_put = 5034994863481u128;
        let tol = 625000000000u128; // $0.62
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#398 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#398 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0399() {
        // S=100.0, K=90.0, T=0.1, r=0.0
        // v0=0.16, kappa=2.0, theta=0.16, xi=0.1, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 2000000000000u128, 160000000000u128, 100000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 11359191412867u128;
        let exp_put = 1359191412867u128;
        let tol = 400000000000u128; // $0.40
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#399 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#399 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0400() {
        // S=100.0, K=100.0, T=0.5, r=0.0
        // v0=0.16, kappa=2.0, theta=0.16, xi=0.2, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 500000000000u128,
            160000000000u128, 2000000000000u128, 160000000000u128, 200000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 11129592277718u128;
        let exp_put = 11129592277718u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#400 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#400 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0401() {
        // S=100.0, K=110.0, T=2.0, r=0.0
        // v0=0.16, kappa=2.0, theta=0.16, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 2000000000000u128, 160000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 17638328776117u128;
        let exp_put = 27638328776117u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#401 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#401 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0402() {
        // S=100.0, K=120.0, T=0.25, r=0.0
        // v0=0.16, kappa=2.0, theta=0.16, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 2000000000000u128, 160000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 1368024470103u128;
        let exp_put = 21368024470103u128;
        let tol = 500000000000u128; // $0.50
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#402 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#402 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0403() {
        // S=100.0, K=85.0, T=2.0, r=0.0
        // v0=0.16, kappa=2.0, theta=0.16, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 2000000000000u128, 160000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 28641253024863u128;
        let exp_put = 13641253024863u128;
        let tol = 302941176471u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#403 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#403 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0404() {
        // S=100.0, K=95.0, T=0.25, r=0.0
        // v0=0.16, kappa=2.0, theta=0.16, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 2000000000000u128, 160000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 10312161886840u128;
        let exp_put = 5312161886840u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#404 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#404 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0405() {
        // S=100.0, K=105.0, T=1.0, r=0.0
        // v0=0.16, kappa=3.0, theta=0.01, xi=0.1, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 3000000000000u128, 10000000000u128, 100000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 7415080359966u128;
        let exp_put = 12415080359966u128;
        let tol = 1000000000000u128; // $1.00
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#405 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#405 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0406() {
        // S=100.0, K=115.0, T=0.1, r=0.0
        // v0=0.16, kappa=3.0, theta=0.01, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 3000000000000u128, 10000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 684237268233u128;
        let exp_put = 15684237268233u128;
        let tol = 1000000000000u128; // $1.00
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#406 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#406 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0407() {
        // S=100.0, K=80.0, T=1.0, r=0.0
        // v0=0.16, kappa=3.0, theta=0.01, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 3000000000000u128, 10000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 22339467069189u128;
        let exp_put = 2339467069189u128;
        let tol = 225000000000u128; // $0.22
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#407 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#407 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0408() {
        // S=100.0, K=90.0, T=0.1, r=0.0
        // v0=0.16, kappa=3.0, theta=0.01, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 3000000000000u128, 10000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 11236930999358u128;
        let exp_put = 1236930999358u128;
        let tol = 283333333333u128; // $0.28
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#408 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#408 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0409() {
        // S=100.0, K=100.0, T=0.5, r=0.0
        // v0=0.16, kappa=3.0, theta=0.01, xi=0.8, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 500000000000u128,
            160000000000u128, 3000000000000u128, 10000000000u128, 800000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 7512319516909u128;
        let exp_put = 7512319516909u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#409 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#409 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0410() {
        // S=100.0, K=110.0, T=2.0, r=0.0
        // v0=0.16, kappa=3.0, theta=0.04, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 3000000000000u128, 40000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 9713709541516u128;
        let exp_put = 19713709541516u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#410 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#410 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0411() {
        // S=100.0, K=120.0, T=0.25, r=0.0
        // v0=0.16, kappa=3.0, theta=0.04, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 3000000000000u128, 40000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 1216177225581u128;
        let exp_put = 21216177225581u128;
        let tol = 910000000000u128; // $0.91
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#411 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#411 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0412() {
        // S=100.0, K=85.0, T=2.0, r=0.0
        // v0=0.16, kappa=3.0, theta=0.04, xi=0.2, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 3000000000000u128, 40000000000u128, 200000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 21538165643768u128;
        let exp_put = 6538165643768u128;
        let tol = 180000000000u128; // $0.18
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#412 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#412 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0413() {
        // S=100.0, K=95.0, T=0.25, r=0.0
        // v0=0.16, kappa=3.0, theta=0.04, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 3000000000000u128, 40000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 9601438162823u128;
        let exp_put = 4601438162823u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#413 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#413 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0414() {
        // S=100.0, K=105.0, T=1.0, r=0.0
        // v0=0.16, kappa=3.0, theta=0.04, xi=0.5, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 3000000000000u128, 40000000000u128, 500000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 8530528175256u128;
        let exp_put = 13530528175256u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#414 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#414 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0415() {
        // S=100.0, K=115.0, T=0.1, r=0.0
        // v0=0.16, kappa=3.0, theta=0.04, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 3000000000000u128, 40000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 627618054460u128;
        let exp_put = 15627618054460u128;
        let tol = 589130434783u128; // $0.59
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#415 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#415 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0416() {
        // S=100.0, K=80.0, T=1.0, r=0.0
        // v0=0.16, kappa=3.0, theta=0.09, xi=0.1, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 3000000000000u128, 90000000000u128, 100000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 24572264570590u128;
        let exp_put = 4572264570590u128;
        let tol = 760000000000u128; // $0.76
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#416 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#416 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0417() {
        // S=100.0, K=90.0, T=0.1, r=0.0
        // v0=0.16, kappa=3.0, theta=0.09, xi=0.2, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 3000000000000u128, 90000000000u128, 200000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 11283258196997u128;
        let exp_put = 1283258196997u128;
        let tol = 760000000000u128; // $0.76
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#417 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#417 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0418() {
        // S=100.0, K=100.0, T=0.5, r=0.0
        // v0=0.16, kappa=3.0, theta=0.09, xi=0.3, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 500000000000u128,
            160000000000u128, 3000000000000u128, 90000000000u128, 300000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 9834445121644u128;
        let exp_put = 9834445121644u128;
        let tol = 180000000000u128; // $0.18
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#418 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#418 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0419() {
        // S=100.0, K=110.0, T=2.0, r=0.0
        // v0=0.16, kappa=3.0, theta=0.09, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 3000000000000u128, 90000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 12721572578210u128;
        let exp_put = 22721572578210u128;
        let tol = 450000000000u128; // $0.45
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#419 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#419 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0420() {
        // S=100.0, K=120.0, T=0.25, r=0.0
        // v0=0.16, kappa=3.0, theta=0.09, xi=0.8, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 3000000000000u128, 90000000000u128, 800000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 574147845297u128;
        let exp_put = 20574147845297u128;
        let tol = 800000000000u128; // $0.80
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#420 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#420 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0421() {
        // S=100.0, K=85.0, T=2.0, r=0.0
        // v0=0.16, kappa=3.0, theta=0.09, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 3000000000000u128, 90000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 24625497249220u128;
        let exp_put = 9625497249220u128;
        let tol = 602941176471u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#421 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#421 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0422() {
        // S=100.0, K=95.0, T=0.25, r=0.0
        // v0=0.16, kappa=3.0, theta=0.16, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 3000000000000u128, 160000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 10506564895139u128;
        let exp_put = 5506564895139u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#422 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#422 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0423() {
        // S=100.0, K=105.0, T=1.0, r=0.0
        // v0=0.16, kappa=3.0, theta=0.16, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 3000000000000u128, 160000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 13749169450215u128;
        let exp_put = 18749169450215u128;
        let tol = 180000000000u128; // $0.18
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#423 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#423 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0424() {
        // S=100.0, K=115.0, T=0.1, r=0.0
        // v0=0.16, kappa=3.0, theta=0.16, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 3000000000000u128, 160000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 846853849005u128;
        let exp_put = 15846853849005u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#424 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#424 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0425() {
        // S=100.0, K=80.0, T=1.0, r=0.0
        // v0=0.16, kappa=3.0, theta=0.16, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 3000000000000u128, 160000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 26555698036473u128;
        let exp_put = 6555698036473u128;
        let tol = 325000000000u128; // $0.33
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#425 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#425 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0426() {
        // S=100.0, K=90.0, T=0.1, r=0.0
        // v0=0.16, kappa=3.0, theta=0.16, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 3000000000000u128, 160000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 11493031396815u128;
        let exp_put = 1493031396815u128;
        let tol = 583333333333u128; // $0.58
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#426 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#426 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0427() {
        // S=100.0, K=100.0, T=0.5, r=0.0
        // v0=0.16, kappa=5.0, theta=0.01, xi=0.1, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 500000000000u128,
            160000000000u128, 5000000000000u128, 10000000000u128, 100000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 7156513229036u128;
        let exp_put = 7156513229036u128;
        let tol = 1300000000000u128; // $1.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#427 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#427 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0428() {
        // S=100.0, K=110.0, T=2.0, r=0.0
        // v0=0.16, kappa=5.0, theta=0.01, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 5000000000000u128, 10000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 4775751942470u128;
        let exp_put = 14775751942470u128;
        let tol = 375000000000u128; // $0.38
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#428 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#428 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0429() {
        // S=100.0, K=120.0, T=0.25, r=0.0
        // v0=0.16, kappa=5.0, theta=0.01, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 5000000000000u128, 10000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 611794535394u128;
        let exp_put = 20611794535394u128;
        let tol = 375000000000u128; // $0.38
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#429 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#429 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0430() {
        // S=100.0, K=85.0, T=2.0, r=0.0
        // v0=0.16, kappa=5.0, theta=0.01, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 5000000000000u128, 10000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 17779477797136u128;
        let exp_put = 2779477797136u128;
        let tol = 375000000000u128; // $0.38
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#430 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#430 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0431() {
        // S=100.0, K=95.0, T=0.25, r=0.0
        // v0=0.16, kappa=5.0, theta=0.01, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 5000000000000u128, 10000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 8758949923748u128;
        let exp_put = 3758949923748u128;
        let tol = 375000000000u128; // $0.38
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#431 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#431 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0432() {
        // S=100.0, K=105.0, T=1.0, r=0.0
        // v0=0.16, kappa=5.0, theta=0.01, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 5000000000000u128, 10000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 5177603511983u128;
        let exp_put = 10177603511983u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#432 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#432 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0433() {
        // S=100.0, K=115.0, T=0.1, r=0.0
        // v0=0.16, kappa=5.0, theta=0.04, xi=0.1, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 5000000000000u128, 40000000000u128, 100000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 658180705875u128;
        let exp_put = 15658180705875u128;
        let tol = 1210000000000u128; // $1.21
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#433 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#433 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0434() {
        // S=100.0, K=80.0, T=1.0, r=0.0
        // v0=0.16, kappa=5.0, theta=0.04, xi=0.2, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 5000000000000u128, 40000000000u128, 200000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 22472832731115u128;
        let exp_put = 2472832731115u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#434 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#434 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0435() {
        // S=100.0, K=90.0, T=0.1, r=0.0
        // v0=0.16, kappa=5.0, theta=0.04, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 5000000000000u128, 40000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 11136896468768u128;
        let exp_put = 1136896468768u128;
        let tol = 1210000000000u128; // $1.21
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#435 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#435 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0436() {
        // S=100.0, K=100.0, T=0.5, r=0.0
        // v0=0.16, kappa=5.0, theta=0.04, xi=0.5, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 500000000000u128,
            160000000000u128, 5000000000000u128, 40000000000u128, 500000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 7903924636624u128;
        let exp_put = 7903924636624u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#436 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#436 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0437() {
        // S=100.0, K=110.0, T=2.0, r=0.0
        // v0=0.16, kappa=5.0, theta=0.04, xi=0.8, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 5000000000000u128, 40000000000u128, 800000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 7413218256048u128;
        let exp_put = 17413218256048u128;
        let tol = 750000000000u128; // $0.75
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#437 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#437 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0438() {
        // S=100.0, K=120.0, T=0.25, r=0.0
        // v0=0.16, kappa=5.0, theta=0.09, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 5000000000000u128, 90000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 1481370574679u128;
        let exp_put = 21481370574679u128;
        let tol = 1060000000000u128; // $1.06
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#438 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#438 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0439() {
        // S=100.0, K=85.0, T=2.0, r=0.0
        // v0=0.16, kappa=5.0, theta=0.09, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 5000000000000u128, 90000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 24685191506216u128;
        let exp_put = 9685191506216u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#439 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#439 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0440() {
        // S=100.0, K=95.0, T=0.25, r=0.0
        // v0=0.16, kappa=5.0, theta=0.09, xi=0.2, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 5000000000000u128, 90000000000u128, 200000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 9765288303972u128;
        let exp_put = 4765288303972u128;
        let tol = 1060000000000u128; // $1.06
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#440 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#440 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0441() {
        // S=100.0, K=105.0, T=1.0, r=0.0
        // v0=0.16, kappa=5.0, theta=0.09, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 5000000000000u128, 90000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 10638529781921u128;
        let exp_put = 15638529781921u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#441 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#441 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0442() {
        // S=100.0, K=115.0, T=0.1, r=0.0
        // v0=0.16, kappa=5.0, theta=0.09, xi=0.5, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 5000000000000u128, 90000000000u128, 500000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 686765451395u128;
        let exp_put = 15686765451395u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#442 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#442 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0443() {
        // S=100.0, K=80.0, T=1.0, r=0.0
        // v0=0.16, kappa=5.0, theta=0.09, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 1000000000000u128,
            160000000000u128, 5000000000000u128, 90000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 24418850658714u128;
        let exp_put = 4418850658714u128;
        let tol = 625000000000u128; // $0.62
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#443 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#443 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0444() {
        // S=100.0, K=90.0, T=0.1, r=0.0
        // v0=0.16, kappa=5.0, theta=0.16, xi=0.1, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 100000000000u128,
            160000000000u128, 5000000000000u128, 160000000000u128, 100000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 11357146283813u128;
        let exp_put = 1357146283813u128;
        let tol = 850000000000u128; // $0.85
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#444 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#444 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0445() {
        // S=100.0, K=100.0, T=0.5, r=0.0
        // v0=0.16, kappa=5.0, theta=0.16, xi=0.2, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 500000000000u128,
            160000000000u128, 5000000000000u128, 160000000000u128, 200000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 11166330548915u128;
        let exp_put = 11166330548915u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#445 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#445 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0446() {
        // S=100.0, K=115.0, T=2.0, r=0.0
        // v0=0.16, kappa=5.0, theta=0.16, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 5000000000000u128, 160000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 16595265953463u128;
        let exp_put = 31595265953463u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#446 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#446 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0447() {
        // S=100.0, K=80.0, T=0.5, r=0.0
        // v0=0.16, kappa=5.0, theta=0.16, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 500000000000u128,
            160000000000u128, 5000000000000u128, 160000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 23460703357625u128;
        let exp_put = 3460703357625u128;
        let tol = 525000000000u128; // $0.53
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#447 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#447 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0448() {
        // S=100.0, K=90.0, T=2.0, r=0.0
        // v0=0.16, kappa=5.0, theta=0.16, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 2000000000000u128,
            160000000000u128, 5000000000000u128, 160000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 26443487852861u128;
        let exp_put = 16443487852861u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#448 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#448 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0449() {
        // S=100.0, K=100.0, T=0.25, r=0.0
        // v0=0.16, kappa=5.0, theta=0.16, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 250000000000u128,
            160000000000u128, 5000000000000u128, 160000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 7815125559061u128;
        let exp_put = 7815125559061u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#449 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#449 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0450() {
        // S=100.0, K=110.0, T=1.0, r=0.0
        // v0=0.25, kappa=0.5, theta=0.01, xi=0.1, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 1000000000000u128,
            250000000000u128, 500000000000u128, 10000000000u128, 100000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 13792508005768u128;
        let exp_put = 23792508005768u128;
        let tol = 895000000000u128; // $0.89
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#450 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#450 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0451() {
        // S=100.0, K=120.0, T=0.1, r=0.0
        // v0=0.25, kappa=0.5, theta=0.01, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 100000000000u128,
            250000000000u128, 500000000000u128, 10000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 962439813664u128;
        let exp_put = 20962439813664u128;
        let tol = 895000000000u128; // $0.89
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#451 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#451 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0452() {
        // S=100.0, K=85.0, T=1.0, r=0.0
        // v0=0.25, kappa=0.5, theta=0.01, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 1000000000000u128,
            250000000000u128, 500000000000u128, 10000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 24845018538156u128;
        let exp_put = 9845018538156u128;
        let tol = 102941176471u128; // $0.10
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#452 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#452 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0453() {
        // S=100.0, K=95.0, T=0.1, r=0.0
        // v0=0.25, kappa=0.5, theta=0.01, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 100000000000u128,
            250000000000u128, 500000000000u128, 10000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 8892357892924u128;
        let exp_put = 3892357892924u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#453 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#453 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0454() {
        // S=100.0, K=105.0, T=0.5, r=0.0
        // v0=0.25, kappa=0.5, theta=0.01, xi=0.8, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 500000000000u128,
            250000000000u128, 500000000000u128, 10000000000u128, 800000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 9668731672095u128;
        let exp_put = 14668731672095u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#454 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#454 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0455() {
        // S=100.0, K=115.0, T=2.0, r=0.0
        // v0=0.25, kappa=0.5, theta=0.04, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 2000000000000u128,
            250000000000u128, 500000000000u128, 40000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 17189273146232u128;
        let exp_put = 32189273146232u128;
        let tol = 289130434783u128; // $0.29
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#455 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#455 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0456() {
        // S=100.0, K=80.0, T=0.5, r=0.0
        // v0=0.25, kappa=0.5, theta=0.04, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 500000000000u128,
            250000000000u128, 500000000000u128, 40000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 24779105427703u128;
        let exp_put = 4779105427703u128;
        let tol = 325000000000u128; // $0.33
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#456 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#456 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0457() {
        // S=100.0, K=90.0, T=2.0, r=0.0
        // v0=0.25, kappa=0.5, theta=0.04, xi=0.2, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 2000000000000u128,
            250000000000u128, 500000000000u128, 40000000000u128, 200000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 27054117749420u128;
        let exp_put = 17054117749420u128;
        let tol = 83333333333u128; // $0.08
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#457 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#457 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0458() {
        // S=100.0, K=100.0, T=0.25, r=0.0
        // v0=0.25, kappa=0.5, theta=0.04, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 250000000000u128,
            250000000000u128, 500000000000u128, 40000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 9646528505959u128;
        let exp_put = 9646528505959u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#458 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#458 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0459() {
        // S=100.0, K=110.0, T=1.0, r=0.0
        // v0=0.25, kappa=0.5, theta=0.04, xi=0.5, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 1000000000000u128,
            250000000000u128, 500000000000u128, 40000000000u128, 500000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 12949478746992u128;
        let exp_put = 22949478746992u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#459 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#459 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0460() {
        // S=100.0, K=120.0, T=0.1, r=0.0
        // v0=0.25, kappa=0.5, theta=0.04, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 100000000000u128,
            250000000000u128, 500000000000u128, 40000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 861265612256u128;
        let exp_put = 20861265612256u128;
        let tol = 600000000000u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#460 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#460 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0461() {
        // S=100.0, K=85.0, T=1.0, r=0.0
        // v0=0.25, kappa=0.5, theta=0.09, xi=0.1, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 1000000000000u128,
            250000000000u128, 500000000000u128, 90000000000u128, 100000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 25525107014758u128;
        let exp_put = 10525107014758u128;
        let tol = 655000000000u128; // $0.65
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#461 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#461 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0462() {
        // S=100.0, K=95.0, T=0.1, r=0.0
        // v0=0.25, kappa=0.5, theta=0.09, xi=0.2, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 100000000000u128,
            250000000000u128, 500000000000u128, 90000000000u128, 200000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 8897764581935u128;
        let exp_put = 3897764581935u128;
        let tol = 655000000000u128; // $0.65
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#462 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#462 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0463() {
        // S=100.0, K=105.0, T=0.5, r=0.0
        // v0=0.25, kappa=0.5, theta=0.09, xi=0.3, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 500000000000u128,
            250000000000u128, 500000000000u128, 90000000000u128, 300000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 11074108582625u128;
        let exp_put = 16074108582625u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#463 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#463 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0464() {
        // S=100.0, K=115.0, T=2.0, r=0.0
        // v0=0.25, kappa=0.5, theta=0.09, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 2000000000000u128,
            250000000000u128, 500000000000u128, 90000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 13933436807140u128;
        let exp_put = 28933436807140u128;
        let tol = 489130434783u128; // $0.49
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#464 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#464 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0465() {
        // S=100.0, K=80.0, T=0.5, r=0.0
        // v0=0.25, kappa=0.5, theta=0.09, xi=0.8, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 500000000000u128,
            250000000000u128, 500000000000u128, 90000000000u128, 800000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 25303252304328u128;
        let exp_put = 5303252304328u128;
        let tol = 825000000000u128; // $0.83
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#465 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#465 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0466() {
        // S=100.0, K=90.0, T=2.0, r=0.0
        // v0=0.25, kappa=0.5, theta=0.09, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 2000000000000u128,
            250000000000u128, 500000000000u128, 90000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 25602498394389u128;
        let exp_put = 15602498394389u128;
        let tol = 583333333333u128; // $0.58
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#466 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#466 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0467() {
        // S=100.0, K=100.0, T=0.25, r=0.0
        // v0=0.25, kappa=0.5, theta=0.16, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 250000000000u128,
            250000000000u128, 500000000000u128, 160000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 9823199644115u128;
        let exp_put = 9823199644115u128;
        let tol = 445000000000u128; // $0.45
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#467 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#467 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0468() {
        // S=100.0, K=110.0, T=1.0, r=0.0
        // v0=0.25, kappa=0.5, theta=0.16, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 1000000000000u128,
            250000000000u128, 500000000000u128, 160000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 14987130945306u128;
        let exp_put = 24987130945306u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#468 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#468 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0469() {
        // S=100.0, K=120.0, T=0.1, r=0.0
        // v0=0.25, kappa=0.5, theta=0.16, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 100000000000u128,
            250000000000u128, 500000000000u128, 160000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 968482995312u128;
        let exp_put = 20968482995312u128;
        let tol = 445000000000u128; // $0.45
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#469 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#469 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0470() {
        // S=100.0, K=85.0, T=1.0, r=0.0
        // v0=0.25, kappa=0.5, theta=0.16, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 1000000000000u128,
            250000000000u128, 500000000000u128, 160000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 25751720226581u128;
        let exp_put = 10751720226581u128;
        let tol = 302941176471u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#470 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#470 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0471() {
        // S=100.0, K=95.0, T=0.1, r=0.0
        // v0=0.25, kappa=0.5, theta=0.16, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 100000000000u128,
            250000000000u128, 500000000000u128, 160000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 8936890703257u128;
        let exp_put = 3936890703257u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#471 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#471 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0472() {
        // S=100.0, K=105.0, T=0.5, r=0.0
        // v0=0.25, kappa=1.0, theta=0.01, xi=0.1, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 500000000000u128,
            250000000000u128, 1000000000000u128, 10000000000u128, 100000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 10370840540075u128;
        let exp_put = 15370840540075u128;
        let tol = 970000000000u128; // $0.97
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#472 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#472 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0473() {
        // S=100.0, K=115.0, T=2.0, r=0.0
        // v0=0.25, kappa=1.0, theta=0.01, xi=0.2, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 2000000000000u128,
            250000000000u128, 1000000000000u128, 10000000000u128, 200000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 12077290568236u128;
        let exp_put = 27077290568236u128;
        let tol = 289130434783u128; // $0.29
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#473 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#473 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0474() {
        // S=100.0, K=80.0, T=0.5, r=0.0
        // v0=0.25, kappa=1.0, theta=0.01, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 500000000000u128,
            250000000000u128, 1000000000000u128, 10000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 24330195958399u128;
        let exp_put = 4330195958399u128;
        let tol = 325000000000u128; // $0.33
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#474 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#474 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0475() {
        // S=100.0, K=90.0, T=2.0, r=0.0
        // v0=0.25, kappa=1.0, theta=0.01, xi=0.3, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 2000000000000u128,
            250000000000u128, 1000000000000u128, 10000000000u128, 300000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 22928823363539u128;
        let exp_put = 12928823363539u128;
        let tol = 83333333333u128; // $0.08
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#475 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#475 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0476() {
        // S=100.0, K=100.0, T=0.25, r=0.0
        // v0=0.25, kappa=1.0, theta=0.01, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 250000000000u128,
            250000000000u128, 1000000000000u128, 10000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 9275479642567u128;
        let exp_put = 9275479642567u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#476 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#476 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0477() {
        // S=100.0, K=110.0, T=1.0, r=0.0
        // v0=0.25, kappa=1.0, theta=0.01, xi=0.8, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 1000000000000u128,
            250000000000u128, 1000000000000u128, 10000000000u128, 800000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 10013669260973u128;
        let exp_put = 20013669260973u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#477 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#477 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0478() {
        // S=100.0, K=120.0, T=0.1, r=0.0
        // v0=0.25, kappa=1.0, theta=0.04, xi=0.1, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 100000000000u128,
            250000000000u128, 1000000000000u128, 40000000000u128, 100000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 953232960033u128;
        let exp_put = 20953232960033u128;
        let tol = 880000000000u128; // $0.88
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#478 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#478 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0479() {
        // S=100.0, K=85.0, T=1.0, r=0.0
        // v0=0.25, kappa=1.0, theta=0.04, xi=0.2, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 1000000000000u128,
            250000000000u128, 1000000000000u128, 40000000000u128, 200000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 23910149802238u128;
        let exp_put = 8910149802238u128;
        let tol = 102941176471u128; // $0.10
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#479 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#479 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0480() {
        // S=100.0, K=95.0, T=0.1, r=0.0
        // v0=0.25, kappa=1.0, theta=0.04, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 100000000000u128,
            250000000000u128, 1000000000000u128, 40000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 8833603068256u128;
        let exp_put = 3833603068256u128;
        let tol = 880000000000u128; // $0.88
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#480 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#480 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0481() {
        // S=100.0, K=105.0, T=0.5, r=0.0
        // v0=0.25, kappa=1.0, theta=0.04, xi=0.5, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 500000000000u128,
            250000000000u128, 1000000000000u128, 40000000000u128, 500000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 9948355016333u128;
        let exp_put = 14948355016333u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#481 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#481 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0482() {
        // S=100.0, K=115.0, T=2.0, r=0.0
        // v0=0.25, kappa=1.0, theta=0.04, xi=0.8, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 2000000000000u128,
            250000000000u128, 1000000000000u128, 40000000000u128, 800000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 7883653730338u128;
        let exp_put = 22883653730338u128;
        let tol = 789130434783u128; // $0.79
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#482 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#482 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0483() {
        // S=100.0, K=80.0, T=0.5, r=0.0
        // v0=0.25, kappa=1.0, theta=0.09, xi=0.1, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 500000000000u128,
            250000000000u128, 1000000000000u128, 90000000000u128, 100000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 24441495697698u128;
        let exp_put = 4441495697698u128;
        let tol = 730000000000u128; // $0.73
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#483 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#483 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0484() {
        // S=100.0, K=90.0, T=2.0, r=0.0
        // v0=0.25, kappa=1.0, theta=0.09, xi=0.1, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 2000000000000u128,
            250000000000u128, 1000000000000u128, 90000000000u128, 100000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 26424299815644u128;
        let exp_put = 16424299815644u128;
        let tol = 83333333333u128; // $0.08
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#484 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#484 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0485() {
        // S=100.0, K=100.0, T=0.25, r=0.0
        // v0=0.25, kappa=1.0, theta=0.09, xi=0.2, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 250000000000u128,
            250000000000u128, 1000000000000u128, 90000000000u128, 200000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 9548669136428u128;
        let exp_put = 9548669136428u128;
        let tol = 730000000000u128; // $0.73
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#485 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#485 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0486() {
        // S=100.0, K=110.0, T=1.0, r=0.0
        // v0=0.25, kappa=1.0, theta=0.09, xi=0.3, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 1000000000000u128,
            250000000000u128, 1000000000000u128, 90000000000u128, 300000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 13093065135260u128;
        let exp_put = 23093065135260u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#486 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#486 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0487() {
        // S=100.0, K=120.0, T=0.1, r=0.0
        // v0=0.25, kappa=1.0, theta=0.09, xi=0.5, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 120000000000000u128, 0u128, 100000000000u128,
            250000000000u128, 1000000000000u128, 90000000000u128, 500000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 895266824097u128;
        let exp_put = 20895266824097u128;
        let tol = 300000000000u128; // $0.30
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#487 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#487 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0488() {
        // S=100.0, K=85.0, T=1.0, r=0.0
        // v0=0.25, kappa=1.0, theta=0.09, xi=0.8, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 85000000000000u128, 0u128, 1000000000000u128,
            250000000000u128, 1000000000000u128, 90000000000u128, 800000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 24004543634154u128;
        let exp_put = 9004543634154u128;
        let tol = 602941176471u128; // $0.60
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#488 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#488 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0489() {
        // S=100.0, K=95.0, T=0.1, r=0.0
        // v0=0.25, kappa=1.0, theta=0.16, xi=0.1, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 95000000000000u128, 0u128, 100000000000u128,
            250000000000u128, 1000000000000u128, 160000000000u128, 100000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 8884017896851u128;
        let exp_put = 3884017896851u128;
        let tol = 520000000000u128; // $0.52
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#489 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#489 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0490() {
        // S=100.0, K=105.0, T=0.5, r=0.0
        // v0=0.25, kappa=1.0, theta=0.16, xi=0.2, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 105000000000000u128, 0u128, 500000000000u128,
            250000000000u128, 1000000000000u128, 160000000000u128, 200000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 11225074665206u128;
        let exp_put = 16225074665206u128;
        let tol = 50000000000u128; // $0.05
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#490 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#490 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0491() {
        // S=100.0, K=115.0, T=2.0, r=0.0
        // v0=0.25, kappa=1.0, theta=0.16, xi=0.3, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 115000000000000u128, 0u128, 2000000000000u128,
            250000000000u128, 1000000000000u128, 160000000000u128, 300000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 17723519092482u128;
        let exp_put = 32723519092482u128;
        let tol = 289130434783u128; // $0.29
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#491 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#491 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0492() {
        // S=100.0, K=80.0, T=0.5, r=0.0
        // v0=0.25, kappa=1.0, theta=0.16, xi=0.5, rho=-0.9
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 500000000000u128,
            250000000000u128, 1000000000000u128, 160000000000u128, 500000000000u128,
            -900000000000i128,
        ).unwrap();
        let exp_call = 25115184639606u128;
        let exp_put = 5115184639606u128;
        let tol = 525000000000u128; // $0.53
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#492 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#492 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0493() {
        // S=100.0, K=90.0, T=2.0, r=0.0
        // v0=0.25, kappa=1.0, theta=0.16, xi=0.5, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 2000000000000u128,
            250000000000u128, 1000000000000u128, 160000000000u128, 500000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 28125918603475u128;
        let exp_put = 18125918603475u128;
        let tol = 283333333333u128; // $0.28
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#493 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#493 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0494() {
        // S=100.0, K=100.0, T=0.25, r=0.0
        // v0=0.25, kappa=1.0, theta=0.16, xi=0.8, rho=0.0
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 250000000000u128,
            250000000000u128, 1000000000000u128, 160000000000u128, 800000000000u128,
            0i128,
        ).unwrap();
        let exp_call = 9497667766455u128;
        let exp_put = 9497667766455u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#494 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#494 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0495() {
        // S=100.0, K=110.0, T=1.0, r=0.0
        // v0=0.25, kappa=2.0, theta=0.01, xi=0.1, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 1000000000000u128,
            250000000000u128, 2000000000000u128, 10000000000u128, 100000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 9505218558504u128;
        let exp_put = 19505218558504u128;
        let tol = 1120000000000u128; // $1.12
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#495 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#495 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0496() {
        // S=100.0, K=80.0, T=0.25, r=0.0
        // v0=0.25, kappa=2.0, theta=0.01, xi=0.2, rho=-0.3
        let (call, put) = heston_price(
            100000000000000u128, 80000000000000u128, 0u128, 250000000000u128,
            250000000000u128, 2000000000000u128, 10000000000u128, 200000000000u128,
            -300000000000i128,
        ).unwrap();
        let exp_call = 21711359696654u128;
        let exp_put = 1711359696654u128;
        let tol = 1120000000000u128; // $1.12
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#496 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#496 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0497() {
        // S=100.0, K=90.0, T=1.0, r=0.0
        // v0=0.25, kappa=2.0, theta=0.01, xi=0.3, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 90000000000000u128, 0u128, 1000000000000u128,
            250000000000u128, 2000000000000u128, 10000000000u128, 300000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 18315487305291u128;
        let exp_put = 8315487305291u128;
        let tol = 240000000000u128; // $0.24
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#497 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#497 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0498() {
        // S=100.0, K=100.0, T=0.1, r=0.0
        // v0=0.25, kappa=2.0, theta=0.01, xi=0.5, rho=-0.5
        let (call, put) = heston_price(
            100000000000000u128, 100000000000000u128, 0u128, 100000000000u128,
            250000000000u128, 2000000000000u128, 10000000000u128, 500000000000u128,
            -500000000000i128,
        ).unwrap();
        let exp_call = 5933630432005u128;
        let exp_put = 5933630432005u128;
        let tol = 250000000000u128; // $0.25
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#498 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#498 put: got={} exp={} diff={}", put, exp_put, dp);
    }

    #[test]
    fn ql_heston_0499() {
        // S=100.0, K=110.0, T=0.5, r=0.0
        // v0=0.25, kappa=2.0, theta=0.01, xi=0.8, rho=-0.7
        let (call, put) = heston_price(
            100000000000000u128, 110000000000000u128, 0u128, 500000000000u128,
            250000000000u128, 2000000000000u128, 10000000000u128, 800000000000u128,
            -700000000000i128,
        ).unwrap();
        let exp_call = 5852729522769u128;
        let exp_put = 15852729522769u128;
        let tol = 550000000000u128; // $0.55
        let dc = if call > exp_call { call - exp_call } else { exp_call - call };
        let dp = if put > exp_put { put - exp_put } else { exp_put - put };
        assert!(dc <= tol,
            "Heston#499 call: got={} exp={} diff={}", call, exp_call, dc);
        assert!(dp <= tol,
            "Heston#499 put: got={} exp={} diff={}", put, exp_put, dp);
    }

}
