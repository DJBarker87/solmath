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
