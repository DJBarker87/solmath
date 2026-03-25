/// CU logging stub (no-op outside Solana). Internal.
#[cfg(target_os = "solana")]
#[inline(always)]
pub(crate) fn log_cu() {
    // no-op outside Solana runtime
}

/// Fixed-point value logger (no-op outside Solana). Internal.
#[cfg(target_os = "solana")]
#[inline]
pub(crate) fn msg_fp(_label: &str, _x: i128) {
    // no-op outside Solana runtime
}
