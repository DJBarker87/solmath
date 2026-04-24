//! Fast bivariate normal CDF at fixed ρ via Catmull-Rom bicubic lookup.
//!
//! The entire evaluation runs in pure i64 arithmetic: 20 multiply-accumulates,
//! 5 shifts, 4 divides for indexing. No `norm_cdf`, no i128 in the hot path.
//!
//! # Usage
//!
//! ```rust
//! # #[cfg(feature = "bivariate")]
//! # fn example() -> Result<(), solmath::SolMathError> {
//! use solmath::Phi2Table;
//!
//! // Offline: generate a 64×64 table for ρ = 0.75
//! // (requires `table-gen` feature)
//! # #[cfg(feature = "table-gen")]
//! # {
//! let rho = 750_000_000_000i128; // 0.75 × SCALE
//! let table = Phi2Table::generate(rho, 64)?;
//!
//! // On-chain: evaluate at (a, b) — all i128 at SCALE
//! let a = -500_000_000_000i128; // -0.5
//! let b =  300_000_000_000i128; //  0.3
//! let result = table.eval(a, b)?;
//! // result ≈ Φ₂(-0.5, 0.3; 0.75) × SCALE
//! # let _ = result;
//! # }
//! # Ok(())
//! # }
//! ```

use crate::error::SolMathError;
use crate::SCALE_I;

const N: usize = 64;
const S6: i64 = 1_000_000;
const SHIFT: i64 = 1_000_000;
const DOMAIN_MIN: i64 = -4_000_000_000_000;
const DOMAIN_MAX: i64 = 4_000_000_000_000;
const RANGE: i64 = 8_000_000_000_000;
const N_MINUS_1: i64 = 63;

const WN: usize = 1024;
const WS: u32 = 30;
const FRAC_DIVISOR: i64 = RANGE / WN as i64;

/// Precomputed Catmull-Rom basis weights at 2^30.
const CR_W: [[i32; 4]; WN] = {
    let mut out = [[0i32; 4]; WN];
    let s: i64 = 1 << WS;
    let mut k = 0usize;
    while k < WN {
        let t = (k as i64) << (WS - 10);
        let t2 = t * t >> WS;
        let t3 = t2 * t >> WS;
        out[k][0] = ((-t + 2 * t2 - t3) / 2) as i32;
        out[k][1] = ((2 * s - 5 * t2 + 3 * t3) / 2) as i32;
        out[k][2] = ((t + 4 * t2 - 3 * t3) / 2) as i32;
        out[k][3] = ((-t2 + t3) / 2) as i32;
        k += 1;
    }
    out
};

#[inline(always)]
fn cr_dot(w: &[i32; 4], p0: i64, p1: i64, p2: i64, p3: i64) -> i64 {
    (w[0] as i64 * p0 + w[1] as i64 * p1 + w[2] as i64 * p2 + w[3] as i64 * p3) >> WS
}

/// Precomputed bivariate normal CDF table at a fixed correlation.
///
/// Stores Φ₂(a, b; ρ) on a 64×64 grid over `[-4, +4]²` at `SCALE_6` (10⁶)
/// precision. On-chain evaluation via [`Phi2Table::eval`] uses Catmull-Rom
/// bicubic interpolation in pure i64 arithmetic.
///
/// # Performance
///
/// - **943 CU** per evaluation on SBF (constant, input-independent)
/// - **64 KB** storage per table (embedded as `const` in the binary)
///
/// # Accuracy
///
/// Max absolute error < 9.0×10⁻⁵ (validated across 630K vectors).
///
/// # Construction
///
/// Tables are generated offline. Enable the `table-gen` feature for
/// [`Phi2Table::generate`], or pre-generate with a script and embed
/// the raw `[[i32; 64]; 64]` array.
#[derive(Debug, Clone)]
pub struct Phi2Table {
    /// 64×64 grid values at SCALE_6.
    values: [[i32; N]; N],
}

impl Phi2Table {
    /// Create a `Phi2Table` from a pre-generated 64×64 array.
    ///
    /// Values should be Φ₂(a_i, b_j; ρ) × 10⁶ (SCALE_6), where
    /// `a_i = -4 + 8i/63` for `i ∈ 0..64`.
    pub const fn from_array(values: [[i32; N]; N]) -> Self {
        Phi2Table { values }
    }

    /// Evaluate Φ₂(a, b; ρ) via Catmull-Rom bicubic interpolation. ~943 CU.
    ///
    /// All inputs/outputs are signed fixed-point `i128` at `SCALE` (1e12).
    ///
    /// # Domain
    ///
    /// `a`, `b` ∈ `[-4·SCALE, 4·SCALE]`. Values outside are clamped.
    ///
    /// # Accuracy
    ///
    /// Max absolute error < 9.0×10⁻⁵.
    ///
    /// # Errors
    ///
    /// Returns `Ok` for all inputs. Cannot fail in practice — the `Result`
    /// wrapper is for API consistency with [`bvn_cdf`](crate::bvn_cdf()).
    pub fn eval(&self, a: i128, b: i128) -> Result<i128, SolMathError> {
        let a64 = (a.clamp(DOMAIN_MIN as i128, DOMAIN_MAX as i128)) as i64;
        let b64 = (b.clamp(DOMAIN_MIN as i128, DOMAIN_MAX as i128)) as i64;

        let a_off = a64 - DOMAIN_MIN;
        let b_off = b64 - DOMAIN_MIN;

        let ia_scaled = a_off * N_MINUS_1;
        let i0 = (ia_scaled / RANGE) as i32;

        let ib_scaled = b_off * N_MINUS_1;
        let j0 = (ib_scaled / RANGE) as i32;

        let wi_a = ((ia_scaled % RANGE) / FRAC_DIVISOR) as usize;
        let wi_b = ((ib_scaled % RANGE) / FRAC_DIVISOR) as usize;
        let wa = &CR_W[if wi_a < WN { wi_a } else { WN - 1 }];
        let wb = &CR_W[if wi_b < WN { wi_b } else { WN - 1 }];

        let i0 = i0.min(N as i32 - 2);
        let j0 = j0.min(N as i32 - 2);
        let n = N as i32;

        let mut cols = [0i64; 4];
        for di in 0..4i32 {
            let ii = (i0 - 1 + di).clamp(0, n - 1) as usize;
            let p0 = self.values[ii][(j0 - 1).clamp(0, n - 1) as usize] as i64;
            let p1 = self.values[ii][j0.clamp(0, n - 1) as usize] as i64;
            let p2 = self.values[ii][(j0 + 1).clamp(0, n - 1) as usize] as i64;
            let p3 = self.values[ii][(j0 + 2).clamp(0, n - 1) as usize] as i64;
            cols[di as usize] = cr_dot(wb, p0, p1, p2, p3);
        }

        let result_s6 = cr_dot(wa, cols[0], cols[1], cols[2], cols[3]);
        let clamped = result_s6.clamp(0, S6);
        Ok(clamped as i128 * SHIFT as i128)
    }

    /// Generate a Phi2Table offline using the high-precision GL20 bvn_cdf.
    ///
    /// `rho` is the fixed correlation at `SCALE` (1e12), as `i128`.
    /// Generates a 64×64 table covering `[-4, +4]²`.
    ///
    /// This is expensive (~331K CU × 4096 entries) and intended to run as a
    /// native Rust binary, not on-chain.
    ///
    /// # Errors
    ///
    /// Returns `DomainError` if `n != 64` or `|rho| > SCALE`.
    #[cfg(feature = "table-gen")]
    pub fn generate(rho: i128, n: usize) -> Result<Self, SolMathError> {
        use crate::bvn_cdf::bvn_cdf_hp;

        if n != N {
            return Err(SolMathError::DomainError);
        }
        if rho.abs() > SCALE_I {
            return Err(SolMathError::DomainError);
        }

        let mut values = [[0i32; N]; N];
        for i in 0..N {
            let a_fp = DOMAIN_MIN as i128 + (RANGE as i128 * i as i128) / (N as i128 - 1);
            for j in 0..N {
                let b_fp = DOMAIN_MIN as i128 + (RANGE as i128 * j as i128) / (N as i128 - 1);
                let val = bvn_cdf_hp(a_fp, b_fp, rho)?;
                values[i][j] = (val / SHIFT as i128) as i32;
            }
        }

        Ok(Phi2Table { values })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Smoke test: generate a table and eval at the origin.
    #[cfg(feature = "table-gen")]
    #[test]
    fn phi2table_generate_and_eval() {
        let table = Phi2Table::generate(500_000_000_000i128, 64).unwrap();
        let v = table.eval(0, 0).unwrap();
        // Φ₂(0, 0; 0.5) ≈ 0.333 → ~333_000_000_000 at SCALE
        assert!(v > 300_000_000_000 && v < 370_000_000_000, "v={v}");
    }

    /// Boundary: far negative → near 0, far positive → near 1.
    #[cfg(feature = "table-gen")]
    #[test]
    fn phi2table_boundaries() {
        let s: i128 = 1_000_000_000_000;
        let table = Phi2Table::generate(0, 64).unwrap();
        let lo = table.eval(-4 * s, -4 * s).unwrap();
        assert!(lo < s / 1_000, "lo={lo}");
        let hi = table.eval(4 * s, 4 * s).unwrap();
        assert!(hi > s - s / 1_000, "hi={hi}");
    }
}
