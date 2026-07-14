/// Errors returned by fallible solmath operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolMathError {
    /// Input outside the mathematical domain (e.g. ln of zero or negative)
    DomainError,
    /// Result would overflow the representable range
    Overflow,
    /// Division by zero
    DivisionByZero,
    /// Iterative method did not converge (e.g. implied_vol)
    NoConvergence,
}

impl core::fmt::Display for SolMathError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::DomainError => "input outside the mathematical domain",
            Self::Overflow => "result would overflow the representable range",
            Self::DivisionByZero => "division by zero",
            Self::NoConvergence => "iterative method did not converge",
        })
    }
}

// core::error::Error requires Rust 1.81; the crate's MSRV remains 1.79.
