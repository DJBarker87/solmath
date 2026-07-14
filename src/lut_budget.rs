//! Compile-time payload budgets for the reduced-domain transcendental tables.
//!
//! These limits cover raw table data. Linked SBF deltas also include the local
//! polynomial kernels and must be measured in the footprint harness before a
//! release changes either table.

const _: () = {
    const I64_BYTES: usize = core::mem::size_of::<i64>();
    const EXPM1_KERNEL_BYTES: usize = crate::expm1_lut::EXPM1_LUT_SEGMENTS * I64_BYTES;
    const LN_1P_KERNEL_BYTES: usize = 2 * crate::ln_lut::LN_LUT_SEGMENTS * I64_BYTES;
    const SHARED_LN2_BYTES: usize = crate::ln2_lut::K_LN2_ENTRIES * I64_BYTES;
    const NORM_CDF_COEFFICIENT_BYTES: usize =
        (7 * 9 + 3 * 8 + 4 * 7) * I64_BYTES + core::mem::size_of::<i128>();
    const EXP_COEFFICIENT_BYTES: usize = (crate::exp_coeffs::EXP_REMEZ_Q22.len()
        + crate::exp_coeffs::EXP2_PHASE_Q62.len())
        * I64_BYTES;

    assert!(
        crate::ln2_lut::K_LN2_ENTRIES
            == (crate::ln2_lut::K_LN2_MAX - crate::ln2_lut::K_LN2_MIN + 1) as usize
    );
    assert!(EXPM1_KERNEL_BYTES + SHARED_LN2_BYTES <= 16 * 1024);
    assert!(LN_1P_KERNEL_BYTES + SHARED_LN2_BYTES <= 20 * 1024);
    assert!(EXPM1_KERNEL_BYTES + LN_1P_KERNEL_BYTES + SHARED_LN2_BYTES <= 32 * 1024);
    assert!(NORM_CDF_COEFFICIENT_BYTES <= 2 * 1024);
    assert!(EXP_COEFFICIENT_BYTES <= 512);
};

#[cfg(test)]
mod tests {
    #[test]
    fn current_payload_accounting_is_explicit() {
        let expm1_bytes = core::mem::size_of_val(&crate::expm1_lut::EXPM1_MID_EXP_RAW_Q22);
        let ln1p_bytes = core::mem::size_of_val(&crate::ln_lut::LN_LUT_MID_LOG)
            + core::mem::size_of_val(&crate::ln_lut::LN_Q42_RECIP_G32);
        let shared_bytes = core::mem::size_of_val(&crate::ln2_lut::K_LN2_RAW);
        let norm_cdf_bytes = core::mem::size_of_val(&crate::norm_cdf_coeffs::NORM_CDF_0_05_Q23)
            + core::mem::size_of_val(&crate::norm_cdf_coeffs::NORM_CDF_05_10_Q23)
            + core::mem::size_of_val(&crate::norm_cdf_coeffs::NORM_CDF_10_15_Q23)
            + core::mem::size_of_val(&crate::norm_cdf_coeffs::NORM_CDF_15_20_Q23)
            + core::mem::size_of_val(&crate::norm_cdf_coeffs::NORM_CDF_20_25_Q23)
            + core::mem::size_of_val(&crate::norm_cdf_coeffs::NORM_CDF_25_30_Q23)
            + core::mem::size_of_val(&crate::norm_cdf_coeffs::NORM_CDF_30_35_Q23)
            + core::mem::size_of_val(&crate::norm_cdf_coeffs::NORM_CDF_35_40_Q23)
            + core::mem::size_of_val(&crate::norm_cdf_coeffs::NORM_CDF_40_45_Q23)
            + core::mem::size_of_val(&crate::norm_cdf_coeffs::NORM_CDF_45_50_Q23)
            + core::mem::size_of_val(&crate::norm_cdf_coeffs::NORM_TAIL_50_55_Q23)
            + core::mem::size_of_val(&crate::norm_cdf_coeffs::NORM_TAIL_55_60_Q23)
            + core::mem::size_of_val(&crate::norm_cdf_coeffs::NORM_TAIL_60_65_Q23)
            + core::mem::size_of_val(&crate::norm_cdf_coeffs::NORM_TAIL_65_70_Q23)
            + core::mem::size_of_val(&crate::norm_cdf_coeffs::NORM_TAIL_HALF_RAW_CUTOFF);
        let exp_bytes = core::mem::size_of_val(&crate::exp_coeffs::EXP_REMEZ_Q22)
            + core::mem::size_of_val(&crate::exp_coeffs::EXP2_PHASE_Q62);

        assert_eq!(expm1_bytes + shared_bytes, 11_560);
        assert_eq!(ln1p_bytes + shared_bytes, 17_608);
        assert_eq!(expm1_bytes + ln1p_bytes + shared_bytes, 27_944);
        assert_eq!(norm_cdf_bytes, 936);
        assert_eq!(exp_bytes, 304);
    }
}
