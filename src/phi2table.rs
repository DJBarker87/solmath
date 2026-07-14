//! Fast bivariate normal CDF at fixed ρ via monotone bilinear lookup.
//!
//! Bilinear interpolation cannot overshoot the four surrounding grid values,
//! preserving a monotone generated table's probability ordering.
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
//! // This is the compatibility/analytics path. Value-bearing execution must
//! // use generate_certified offline, pin the emitted certificate ID in trusted
//! // program configuration, and call certified()/eval_certified() on-chain.
//! # let _ = result;
//! # }
//! # Ok(())
//! # }
//! ```

use crate::arithmetic::fp_sqrt;
use crate::error::SolMathError;
use crate::SCALE_I;

const N: usize = 64;
const DENSE_N: usize = 129;
const S6: i64 = 1_000_000;
const SHIFT: i64 = 1_000_000;
const DOMAIN_MIN: i64 = -4_000_000_000_000;
const DOMAIN_MAX: i64 = 4_000_000_000_000;
const RANGE: i64 = 8_000_000_000_000;
const CERTIFICATE_VERSION: u16 = 1;
const BILINEAR_INTERPOLATION_ID: u8 = 1;
const BVN_GL20_REFERENCE_ID: u8 = 1;
const MAX_CERTIFIED_RHO: i128 = 990_000_000_000;
/// Bytes retained from SHA-256 for each authenticated table-row commitment.
pub const PHI2_ROW_DIGEST_BYTES: usize = 16;
const ROW_DIGEST_BYTES: usize = PHI2_ROW_DIGEST_BYTES;

// ceil(1 / sqrt(2*pi*e) * SCALE) and ceil(1 / (2*pi) * SCALE).
// These bound |x*phi(x)| and phi(x)*phi(y), respectively, and are used in
// the analytic second-derivative bound for bilinear interpolation.
const MAX_X_PHI: i128 = 241_970_724_520;
const INV_TWO_PI_CEIL: i128 = 159_154_943_092;
// Two staged round-to-nearest interpolations contribute at most one stored
// table unit in total (1e-6 probability).
const BILINEAR_EVALUATION_ROUNDING_ERROR: i128 = SHIFT as i128;

// The GL20 reference implementation's fresh external-reference corpus
// observed 123 raw SCALE units at |rho| <= .99. Certification deliberately
// reserves a much larger 1e-6 probability allowance. This is an explicit
// assumption in every certificate, not a claim of formal verification of
// the GL20 implementation itself.
const GL20_REFERENCE_ABS_ERROR_ALLOWANCE: i128 = 1_000_000;

/// Number of points on each axis in the compatibility lookup table.
pub const PHI2_GRID_SIZE: usize = N;

/// Number of points on each axis in [`Phi2DenseTable`].
pub const PHI2_DENSE_GRID_SIZE: usize = DENSE_N;

/// The interpolation algorithm covered by a [`Phi2Certificate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Phi2Interpolation {
    /// Convex, non-overshooting bilinear interpolation.
    Bilinear = BILINEAR_INTERPOLATION_ID,
}

/// The offline reference covered by a [`Phi2Certificate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Phi2Reference {
    /// `bvn_cdf_hp` (GL20), plus the certificate's explicit error allowance.
    BvnGl20 = BVN_GL20_REFERENCE_ID,
}

/// Precision metadata for one exact table, correlation, grid, and
/// interpolation algorithm.
///
/// Certificates are created by exhaustive offline node comparison through
/// [`Phi2Table::certify`] or [`Phi2DenseTable::certify`]. The certificate ID
/// is SHA-256 over all metadata and a SHA-256 root of 128-bit SHA-256 row
/// commitments. On-chain callers must pin an independently trusted
/// `certificate_id` (normally embedded in the program) when creating a
/// [`CertifiedPhi2Evaluator`]. This prevents an untrusted account from lowering
/// the declared error and recomputing a new certificate for itself.
///
/// `max_abs_error` is measured in the crate's `SCALE` (1e12). It is the sum
/// of the largest generated-node discrepancy from GL20, the declared GL20
/// reference allowance, and a conservative analytic continuous-cell
/// bilinear interpolation bound including fixed-point evaluation rounding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phi2Certificate {
    version: u16,
    grid_size: u16,
    interpolation_id: u8,
    reference_id: u8,
    rho: i128,
    domain_min: i128,
    domain_max: i128,
    value_scale: i128,
    max_node_abs_error: i128,
    interpolation_abs_error_bound: i128,
    reference_abs_error_allowance: i128,
    max_abs_error: i128,
    row_digests: [[u8; ROW_DIGEST_BYTES]; DENSE_N],
    table_digest: [u8; 32],
    certificate_id: [u8; 32],
}

impl Phi2Certificate {
    /// Reconstruct certificate metadata emitted by trusted offline tooling.
    ///
    /// This constructor intentionally does not establish trust. Evaluation
    /// still requires the independently pinned `expected_certificate_id`, and
    /// verifies the ID, all derived bounds, and the complete table digest.
    #[allow(clippy::too_many_arguments)]
    pub const fn from_embedded_parts(
        rho: i128,
        grid_size: u16,
        max_node_abs_error: i128,
        interpolation_abs_error_bound: i128,
        reference_abs_error_allowance: i128,
        max_abs_error: i128,
        row_digests: [[u8; PHI2_ROW_DIGEST_BYTES]; PHI2_DENSE_GRID_SIZE],
        table_digest: [u8; 32],
        certificate_id: [u8; 32],
    ) -> Self {
        Self {
            version: CERTIFICATE_VERSION,
            grid_size,
            interpolation_id: BILINEAR_INTERPOLATION_ID,
            reference_id: BVN_GL20_REFERENCE_ID,
            rho,
            domain_min: DOMAIN_MIN as i128,
            domain_max: DOMAIN_MAX as i128,
            value_scale: S6 as i128,
            max_node_abs_error,
            interpolation_abs_error_bound,
            reference_abs_error_allowance,
            max_abs_error,
            row_digests,
            table_digest,
            certificate_id,
        }
    }

    /// Fixed correlation at `SCALE` (1e12).
    pub const fn rho(&self) -> i128 {
        self.rho
    }

    /// Number of points on each grid axis.
    pub const fn grid_size(&self) -> usize {
        self.grid_size as usize
    }

    /// Interpolation algorithm committed by the certificate.
    pub const fn interpolation(&self) -> Phi2Interpolation {
        Phi2Interpolation::Bilinear
    }

    /// Offline numerical reference committed by the certificate.
    pub const fn reference(&self) -> Phi2Reference {
        Phi2Reference::BvnGl20
    }

    /// Maximum table-node discrepancy from the offline GL20 reference.
    pub const fn max_node_abs_error(&self) -> i128 {
        self.max_node_abs_error
    }

    /// Conservative continuous-cell interpolation plus evaluator-rounding bound.
    pub const fn interpolation_abs_error_bound(&self) -> i128 {
        self.interpolation_abs_error_bound
    }

    /// Explicit allowance for error in the GL20 reference itself.
    pub const fn reference_abs_error_allowance(&self) -> i128 {
        self.reference_abs_error_allowance
    }

    /// Total certified maximum absolute probability error at `SCALE` (1e12).
    pub const fn max_abs_error(&self) -> i128 {
        self.max_abs_error
    }

    /// Truncated SHA-256 commitments for each row.
    ///
    /// Once the certificate ID is independently pinned, changing a committed
    /// row requires a 128-bit second-preimage attack. The generic collision
    /// strength of a 128-bit truncated digest is 64 bits; do not use these row
    /// digests as unpinned, attacker-selected identities.
    pub const fn row_digests(&self) -> &[[u8; PHI2_ROW_DIGEST_BYTES]; PHI2_DENSE_GRID_SIZE] {
        &self.row_digests
    }

    /// SHA-256 digest of the exact table and fixed grid/interpolation metadata.
    pub const fn table_digest(&self) -> [u8; 32] {
        self.table_digest
    }

    /// SHA-256 identity of the table digest plus all certificate metadata.
    pub const fn certificate_id(&self) -> [u8; 32] {
        self.certificate_id
    }
}

/// An evaluator whose certificate identity and economic error budget have
/// already been checked.
///
/// Construct through [`Phi2Table::certified`] or
/// [`Phi2DenseTable::certified`]. Each lookup authenticates only the two rows
/// that can affect its result; reuse this guard to avoid rehashing the complete
/// certificate row-root metadata.
pub struct CertifiedPhi2Evaluator<'a, const M: usize> {
    values: &'a [[i32; M]; M],
    certificate: &'a Phi2Certificate,
}

impl<const M: usize> CertifiedPhi2Evaluator<'_, M> {
    /// Evaluate within the certified `[-4, 4]^2` domain.
    ///
    /// Unlike the compatibility [`Phi2Table::eval`] API, certified evaluation
    /// rejects out-of-domain inputs instead of clamping them, because clamping
    /// would invalidate the error statement relative to the caller's input.
    pub fn eval(&self, a: i128, b: i128) -> Result<i128, SolMathError> {
        if !(DOMAIN_MIN as i128..=DOMAIN_MAX as i128).contains(&a)
            || !(DOMAIN_MIN as i128..=DOMAIN_MAX as i128).contains(&b)
        {
            return Err(SolMathError::DomainError);
        }
        let (first_row, _) = cell_index::<M>(a as i64)?;
        authenticate_rows(self.values, self.certificate, first_row)?;
        eval_bilinear(self.values, a, b)
    }

    /// Certificate used to establish this guard.
    pub const fn certificate(&self) -> &Phi2Certificate {
        self.certificate
    }
}

#[inline]
fn div_ceil_nonnegative(numerator: i128, denominator: i128) -> Result<i128, SolMathError> {
    if numerator < 0 || denominator <= 0 {
        return Err(SolMathError::DomainError);
    }
    let quotient = numerator / denominator;
    if numerator % denominator == 0 {
        Ok(quotient)
    } else {
        quotient.checked_add(1).ok_or(SolMathError::Overflow)
    }
}

#[inline]
fn mul_scale_ceil_nonnegative(a: i128, b: i128) -> Result<i128, SolMathError> {
    let product = a.checked_mul(b).ok_or(SolMathError::Overflow)?;
    div_ceil_nonnegative(product, SCALE_I)
}

#[inline]
fn div_scale_ceil_nonnegative(a: i128, b: i128) -> Result<i128, SolMathError> {
    let numerator = a.checked_mul(SCALE_I).ok_or(SolMathError::Overflow)?;
    div_ceil_nonnegative(numerator, b)
}

/// Conservative global error of the fixed-point bilinear evaluator relative
/// to the mathematical bivariate normal CDF, assuming bounded node errors.
///
/// For each axis,
/// `|F_xx| <= max |x phi(x)| + |rho|/sqrt(1-rho^2) max phi(x)phi(z)`.
/// Tensor-product linear interpolation then contributes at most
/// `h^2/8 * sup|F_xx|` per axis. All fixed-point operations round upward.
fn interpolation_error_bound(rho: i128, cells: usize) -> Result<i128, SolMathError> {
    if cells == 0 || rho.unsigned_abs() > MAX_CERTIFIED_RHO as u128 {
        return Err(SolMathError::DomainError);
    }
    let rho_abs = rho.unsigned_abs() as i128;
    let rho_sq = mul_scale_ceil_nonnegative(rho_abs, rho_abs)?;
    let conditional_variance = SCALE_I
        .checked_sub(rho_sq)
        .ok_or(SolMathError::DomainError)?;
    let conditional_std = fp_sqrt(conditional_variance as u128)? as i128;
    if conditional_std == 0 {
        return Err(SolMathError::DomainError);
    }
    let rho_over_std = div_scale_ceil_nonnegative(rho_abs, conditional_std)?;
    let correlation_term = mul_scale_ceil_nonnegative(INV_TWO_PI_CEIL, rho_over_std)?;
    let second_derivative_bound = MAX_X_PHI
        .checked_add(correlation_term)
        .ok_or(SolMathError::Overflow)?;

    let spacing = div_ceil_nonnegative(RANGE as i128, cells as i128)?;
    let spacing_sq = mul_scale_ceil_nonnegative(spacing, spacing)?;
    let both_axes = mul_scale_ceil_nonnegative(spacing_sq, second_derivative_bound)?;
    div_ceil_nonnegative(both_axes, 4)?
        .checked_add(BILINEAR_EVALUATION_ROUNDING_ERROR)
        .ok_or(SolMathError::Overflow)
}

#[inline]
fn lerp_grid(a: i32, b: i32, fraction: i64) -> i128 {
    let f = fraction as i128;
    let range = RANGE as i128;
    (a as i128 * (range - f) + b as i128 * f + range / 2) / range
}

fn cell_index<const M: usize>(coordinate: i64) -> Result<(usize, i64), SolMathError> {
    if M < 2 || M - 1 > i64::MAX as usize {
        return Err(SolMathError::DomainError);
    }
    let cells = (M - 1) as i64;
    let offset = coordinate
        .checked_sub(DOMAIN_MIN)
        .ok_or(SolMathError::Overflow)?;
    let scaled = offset.checked_mul(cells).ok_or(SolMathError::Overflow)?;
    let index = ((scaled / RANGE) as usize).min(M - 2);
    Ok((index, scaled - index as i64 * RANGE))
}

fn eval_bilinear<const M: usize>(
    values: &[[i32; M]; M],
    a: i128,
    b: i128,
) -> Result<i128, SolMathError> {
    let a64 = (a.clamp(DOMAIN_MIN as i128, DOMAIN_MAX as i128)) as i64;
    let b64 = (b.clamp(DOMAIN_MIN as i128, DOMAIN_MAX as i128)) as i64;
    let (i0, fa) = cell_index::<M>(a64)?;
    let (j0, fb) = cell_index::<M>(b64)?;
    let low = lerp_grid(values[i0][j0], values[i0][j0 + 1], fb);
    let high = lerp_grid(values[i0 + 1][j0], values[i0 + 1][j0 + 1], fb);
    let result_s6 = (low * (RANGE as i128 - fa as i128) + high * fa as i128 + RANGE as i128 / 2)
        / RANGE as i128;
    Ok(result_s6.clamp(0, S6 as i128) * SHIFT as i128)
}

// Minimal streaming SHA-256. The implementation uses fixed-size stack data,
// no allocation, and no unsafe code, so certificate verification is available
// in no_std/SBF builds. A guard verifies the small row-commitment root once;
// each evaluation then hashes only the two rows that affect its result.
struct Sha256 {
    state: [u32; 8],
    block: [u8; 64],
    block_len: usize,
    total_len: u64,
}

impl Sha256 {
    const fn new() -> Self {
        Self {
            state: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
                0x5be0cd19,
            ],
            block: [0; 64],
            block_len: 0,
            total_len: 0,
        }
    }

    fn update(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.block[self.block_len] = byte;
            self.block_len += 1;
            self.total_len = self.total_len.wrapping_add(1);
            if self.block_len == 64 {
                let block = self.block;
                self.compress(&block);
                self.block_len = 0;
            }
        }
    }

    fn finish(mut self) -> [u8; 32] {
        let bit_len = self.total_len.wrapping_mul(8);
        self.block[self.block_len] = 0x80;
        self.block_len += 1;
        if self.block_len > 56 {
            for byte in &mut self.block[self.block_len..] {
                *byte = 0;
            }
            let block = self.block;
            self.compress(&block);
            self.block = [0; 64];
            self.block_len = 0;
        }
        for byte in &mut self.block[self.block_len..56] {
            *byte = 0;
        }
        self.block[56..64].copy_from_slice(&bit_len.to_be_bytes());
        let block = self.block;
        self.compress(&block);

        let mut digest = [0u8; 32];
        for (index, word) in self.state.iter().enumerate() {
            digest[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
        }
        digest
    }

    fn compress(&mut self, block: &[u8; 64]) {
        const K: [u32; 64] = [
            0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
            0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
            0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
            0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
            0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
            0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
            0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
            0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
            0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
            0xc67178f2,
        ];

        let mut words = [0u32; 64];
        for (index, word) in words.iter_mut().enumerate().take(16) {
            let offset = index * 4;
            *word = u32::from_be_bytes([
                block[offset],
                block[offset + 1],
                block[offset + 2],
                block[offset + 3],
            ]);
        }
        for index in 16..64 {
            let s0 = words[index - 15].rotate_right(7)
                ^ words[index - 15].rotate_right(18)
                ^ (words[index - 15] >> 3);
            let s1 = words[index - 2].rotate_right(17)
                ^ words[index - 2].rotate_right(19)
                ^ (words[index - 2] >> 10);
            words[index] = words[index - 16]
                .wrapping_add(s0)
                .wrapping_add(words[index - 7])
                .wrapping_add(s1);
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];
        let mut f = self.state[5];
        let mut g = self.state[6];
        let mut h = self.state[7];
        for index in 0..64 {
            let sigma1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choose = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(sigma1)
                .wrapping_add(choose)
                .wrapping_add(K[index])
                .wrapping_add(words[index]);
            let sigma0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = sigma0.wrapping_add(majority);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
    }
}

fn row_digest<const M: usize>(row: &[i32; M], row_index: usize) -> [u8; ROW_DIGEST_BYTES] {
    let mut hash = Sha256::new();
    hash.update(b"solmath.phi2table.row.v1");
    hash.update(&(M as u64).to_le_bytes());
    hash.update(&(row_index as u64).to_le_bytes());
    hash.update(&(DOMAIN_MIN as i128).to_le_bytes());
    hash.update(&(DOMAIN_MAX as i128).to_le_bytes());
    hash.update(&(S6 as i128).to_le_bytes());
    hash.update(&[BILINEAR_INTERPOLATION_ID]);
    for value in row {
        hash.update(&value.to_le_bytes());
    }
    let full = hash.finish();
    let mut truncated = [0u8; ROW_DIGEST_BYTES];
    truncated.copy_from_slice(&full[..ROW_DIGEST_BYTES]);
    truncated
}

fn table_digest_from_rows(
    grid_size: usize,
    row_digests: &[[u8; ROW_DIGEST_BYTES]; DENSE_N],
) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(b"solmath.phi2table.rows.v1");
    hash.update(&(grid_size as u64).to_le_bytes());
    hash.update(&(DOMAIN_MIN as i128).to_le_bytes());
    hash.update(&(DOMAIN_MAX as i128).to_le_bytes());
    hash.update(&(S6 as i128).to_le_bytes());
    hash.update(&[BILINEAR_INTERPOLATION_ID]);
    for digest in row_digests {
        hash.update(digest);
    }
    hash.finish()
}

#[cfg(feature = "table-gen")]
fn table_commitment<const M: usize>(
    values: &[[i32; M]; M],
) -> ([[u8; ROW_DIGEST_BYTES]; DENSE_N], [u8; 32]) {
    let mut rows = [[0u8; ROW_DIGEST_BYTES]; DENSE_N];
    for index in 0..M {
        rows[index] = row_digest(&values[index], index);
    }
    let root = table_digest_from_rows(M, &rows);
    (rows, root)
}

fn certificate_digest(certificate: &Phi2Certificate) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(b"solmath.phi2table.certificate.v1");
    hash.update(&certificate.version.to_le_bytes());
    hash.update(&certificate.grid_size.to_le_bytes());
    hash.update(&[certificate.interpolation_id, certificate.reference_id]);
    hash.update(&certificate.rho.to_le_bytes());
    hash.update(&certificate.domain_min.to_le_bytes());
    hash.update(&certificate.domain_max.to_le_bytes());
    hash.update(&certificate.value_scale.to_le_bytes());
    hash.update(&certificate.max_node_abs_error.to_le_bytes());
    hash.update(&certificate.interpolation_abs_error_bound.to_le_bytes());
    hash.update(&certificate.reference_abs_error_allowance.to_le_bytes());
    hash.update(&certificate.max_abs_error.to_le_bytes());
    hash.update(&certificate.table_digest);
    hash.finish()
}

fn authenticate_rows<const M: usize>(
    values: &[[i32; M]; M],
    certificate: &Phi2Certificate,
    first_row: usize,
) -> Result<(), SolMathError> {
    if first_row + 1 >= M {
        return Err(SolMathError::DomainError);
    }
    for row_index in first_row..=first_row + 1 {
        let row = &values[row_index];
        if row_digest(row, row_index) != certificate.row_digests[row_index] {
            return Err(SolMathError::DomainError);
        }
        for column in 0..M {
            if !(0..=S6 as i32).contains(&row[column])
                || (column > 0 && row[column] < row[column - 1])
                || (row_index > first_row && row[column] < values[row_index - 1][column])
            {
                return Err(SolMathError::DomainError);
            }
        }
    }
    Ok(())
}

#[cfg(feature = "table-gen")]
fn validate_grid<const M: usize>(values: &[[i32; M]; M]) -> Result<(), SolMathError> {
    if M < 2 || M > u16::MAX as usize {
        return Err(SolMathError::DomainError);
    }
    for i in 0..M {
        for j in 0..M {
            let value = values[i][j];
            if !(0..=S6 as i32).contains(&value) {
                return Err(SolMathError::DomainError);
            }
            if i > 0 && value < values[i - 1][j] {
                return Err(SolMathError::DomainError);
            }
            if j > 0 && value < values[i][j - 1] {
                return Err(SolMathError::DomainError);
            }
        }
    }
    Ok(())
}

fn verify_certificate<const M: usize>(
    certificate: &Phi2Certificate,
    expected_certificate_id: [u8; 32],
    max_abs_error_budget: i128,
) -> Result<(), SolMathError> {
    if M < 2 || M > DENSE_N || !(0..=SCALE_I).contains(&max_abs_error_budget) {
        return Err(SolMathError::DomainError);
    }
    // Check the caller's trust anchor before considering any untrusted
    // certificate claim.
    if certificate.certificate_id != expected_certificate_id {
        return Err(SolMathError::DomainError);
    }
    if certificate.version != CERTIFICATE_VERSION
        || certificate.grid_size as usize != M
        || certificate.interpolation_id != BILINEAR_INTERPOLATION_ID
        || certificate.reference_id != BVN_GL20_REFERENCE_ID
        || certificate.domain_min != DOMAIN_MIN as i128
        || certificate.domain_max != DOMAIN_MAX as i128
        || certificate.value_scale != S6 as i128
        || certificate.reference_abs_error_allowance != GL20_REFERENCE_ABS_ERROR_ALLOWANCE
        || certificate.max_node_abs_error < 0
    {
        return Err(SolMathError::DomainError);
    }
    let interpolation_bound = interpolation_error_bound(certificate.rho, M - 1)?;
    if interpolation_bound != certificate.interpolation_abs_error_bound {
        return Err(SolMathError::DomainError);
    }
    let total = certificate
        .max_node_abs_error
        .checked_add(certificate.interpolation_abs_error_bound)
        .and_then(|value| value.checked_add(certificate.reference_abs_error_allowance))
        .ok_or(SolMathError::Overflow)?;
    if total != certificate.max_abs_error
        || certificate.row_digests[M..]
            .iter()
            .any(|digest| *digest != [0; ROW_DIGEST_BYTES])
        || table_digest_from_rows(M, &certificate.row_digests) != certificate.table_digest
        || certificate_digest(certificate) != certificate.certificate_id
    {
        return Err(SolMathError::DomainError);
    }
    if certificate.max_abs_error > max_abs_error_budget {
        return Err(SolMathError::NoConvergence);
    }
    Ok(())
}

#[cfg(feature = "table-gen")]
fn grid_coordinate(index: usize, points: usize) -> Result<i128, SolMathError> {
    if points < 2 || index >= points {
        return Err(SolMathError::DomainError);
    }
    Ok(DOMAIN_MIN as i128 + (RANGE as i128 * index as i128) / (points as i128 - 1))
}

#[cfg(feature = "table-gen")]
fn generate_grid<const M: usize>(rho: i128) -> Result<[[i32; M]; M], SolMathError> {
    use crate::bvn_cdf::bvn_cdf_hp;

    if M < 2 || M > u16::MAX as usize || rho.unsigned_abs() > SCALE_I as u128 {
        return Err(SolMathError::DomainError);
    }
    let mut values = [[0i32; M]; M];
    for (i, row) in values.iter_mut().enumerate() {
        let a_fp = grid_coordinate(i, M)?;
        for (j, slot) in row.iter_mut().enumerate() {
            let b_fp = grid_coordinate(j, M)?;
            let value = bvn_cdf_hp(a_fp, b_fp, rho)?;
            let rounded = value
                .checked_add(SHIFT as i128 / 2)
                .ok_or(SolMathError::Overflow)?
                / SHIFT as i128;
            if !(0..=S6 as i128).contains(&rounded) {
                return Err(SolMathError::Overflow);
            }
            *slot = rounded as i32;
        }
    }
    validate_grid(&values)?;
    Ok(values)
}

#[cfg(feature = "table-gen")]
fn certify_grid<const M: usize>(
    values: &[[i32; M]; M],
    rho: i128,
) -> Result<Phi2Certificate, SolMathError> {
    use crate::bvn_cdf::bvn_cdf_hp;

    if rho.unsigned_abs() > MAX_CERTIFIED_RHO as u128 {
        return Err(SolMathError::DomainError);
    }
    validate_grid(values)?;
    let mut max_node_abs_error = 0i128;
    for (i, row) in values.iter().enumerate() {
        let a_fp = grid_coordinate(i, M)?;
        for (j, stored_value) in row.iter().enumerate() {
            let b_fp = grid_coordinate(j, M)?;
            let reference = bvn_cdf_hp(a_fp, b_fp, rho)?;
            let stored = (*stored_value as i128)
                .checked_mul(SHIFT as i128)
                .ok_or(SolMathError::Overflow)?;
            let error = stored.abs_diff(reference);
            if error > i128::MAX as u128 {
                return Err(SolMathError::Overflow);
            }
            max_node_abs_error = max_node_abs_error.max(error as i128);
        }
    }
    let interpolation_abs_error_bound = interpolation_error_bound(rho, M - 1)?;
    let max_abs_error = max_node_abs_error
        .checked_add(interpolation_abs_error_bound)
        .and_then(|value| value.checked_add(GL20_REFERENCE_ABS_ERROR_ALLOWANCE))
        .ok_or(SolMathError::Overflow)?;
    let (row_digests, table_digest) = table_commitment(values);
    let mut certificate = Phi2Certificate::from_embedded_parts(
        rho,
        M as u16,
        max_node_abs_error,
        interpolation_abs_error_bound,
        GL20_REFERENCE_ABS_ERROR_ALLOWANCE,
        max_abs_error,
        row_digests,
        table_digest,
        [0; 32],
    );
    certificate.certificate_id = certificate_digest(&certificate);
    Ok(certificate)
}

/// Precomputed bivariate normal CDF table at a fixed correlation.
///
/// Stores Φ₂(a, b; ρ) on a 64×64 grid over `[-4, +4]²` at `SCALE_6` (10⁶)
/// precision. On-chain evaluation via [`Phi2Table::eval`] uses bilinear
/// interpolation without cubic overshoot.
///
/// # Performance
///
/// - Constant, input-independent evaluation cost
/// - **16 KB** storage per table (embed as `static` read-only program data)
///
/// # Accuracy
///
/// [`Phi2Table::eval`] is the compatibility, uncertified API. Interpolation
/// error depends strongly on rho and cell curvature. Use
/// [`Phi2Table::certified`] with a pinned certificate ID and an explicit
/// economic error budget when the result moves value.
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

    /// Borrow the raw compatibility grid for offline code generation/embedding.
    pub const fn as_array(&self) -> &[[i32; N]; N] {
        &self.values
    }

    /// Evaluate Φ₂(a, b; ρ) via non-overshooting bilinear interpolation.
    ///
    /// This compatibility method is **uncertified**: it does not know the
    /// table's rho or validate the contents against a reference. Economic code
    /// should use [`Phi2Table::certified`] or [`Phi2Table::eval_certified`].
    ///
    /// All inputs/outputs are signed fixed-point `i128` at `SCALE` (1e12).
    ///
    /// # Domain
    ///
    /// `a`, `b` ∈ `[-4·SCALE, 4·SCALE]`. Values outside are clamped.
    ///
    /// # Accuracy
    ///
    /// Preserves monotonicity whenever the supplied grid is monotone.
    ///
    /// # Errors
    ///
    /// Returns `Ok` for all inputs. Cannot fail in practice — the `Result`
    /// wrapper is for API consistency with [`bvn_cdf`](crate::bvn_cdf()).
    pub fn eval(&self, a: i128, b: i128) -> Result<i128, SolMathError> {
        eval_bilinear(&self.values, a, b)
    }

    /// Verify certificate metadata against a pinned identity and caller-supplied
    /// economic error budget, returning a row-authenticating evaluator.
    ///
    /// `expected_certificate_id` must come from trusted program configuration,
    /// not from the same untrusted account as `certificate`. Returns
    /// `NoConvergence` when the certified bound exceeds the budget, and
    /// `DomainError` for invalid, forged, mismatched, or corrupt metadata.
    pub fn certified<'a>(
        &'a self,
        certificate: &'a Phi2Certificate,
        expected_certificate_id: [u8; 32],
        max_abs_error_budget: i128,
    ) -> Result<CertifiedPhi2Evaluator<'a, N>, SolMathError> {
        verify_certificate::<N>(certificate, expected_certificate_id, max_abs_error_budget)?;
        Ok(CertifiedPhi2Evaluator {
            values: &self.values,
            certificate,
        })
    }

    /// Verify and evaluate once under an explicit error budget.
    ///
    /// For multiple evaluations, call [`Phi2Table::certified`] once and reuse
    /// the returned guard to avoid repeatedly hashing certificate-root metadata.
    #[allow(clippy::too_many_arguments)]
    pub fn eval_certified(
        &self,
        a: i128,
        b: i128,
        certificate: &Phi2Certificate,
        expected_certificate_id: [u8; 32],
        max_abs_error_budget: i128,
    ) -> Result<i128, SolMathError> {
        self.certified(certificate, expected_certificate_id, max_abs_error_budget)?
            .eval(a, b)
    }

    /// Exhaustively compare all stored nodes with GL20 and create a bound
    /// certificate for this exact table and rho.
    ///
    /// Offline only. Certification is available for `|rho| <= 0.99`; the
    /// continuous bilinear bound becomes singular as `|rho|` approaches one.
    #[cfg(feature = "table-gen")]
    pub fn certify(&self, rho: i128) -> Result<Phi2Certificate, SolMathError> {
        certify_grid(&self.values, rho)
    }

    /// Generate an **uncertified** Phi2Table offline using GL20 `bvn_cdf_hp`.
    ///
    /// `rho` is the fixed correlation at `SCALE` (1e12), as `i128`.
    /// Generates a 64×64 table covering `[-4, +4]²`.
    ///
    /// This is expensive (4096 GL20 evaluations; the final SBF audit observed
    /// up to 468,417 CU for one) and intended to run as a native Rust binary,
    /// not on-chain.
    ///
    /// # Errors
    ///
    /// Returns `DomainError` if `n != 64` or `|rho| > SCALE`.
    #[cfg(feature = "table-gen")]
    pub fn generate(rho: i128, n: usize) -> Result<Self, SolMathError> {
        if n != N {
            return Err(SolMathError::DomainError);
        }
        Ok(Phi2Table {
            values: generate_grid(rho)?,
        })
    }

    /// Generate and then certify a 64×64 table offline.
    #[cfg(feature = "table-gen")]
    pub fn generate_certified(
        rho: i128,
        n: usize,
    ) -> Result<(Self, Phi2Certificate), SolMathError> {
        let table = Self::generate(rho, n)?;
        let certificate = table.certify(rho)?;
        Ok((table, certificate))
    }
}

/// Denser fixed-size bivariate normal lookup table for value-sensitive paths.
///
/// Stores a 129×129 grid (128 cells per axis) at `SCALE_6`, occupying 66,564
/// bytes. It uses no heap allocation and evaluates with the same constant-cost
/// four-node bilinear interpolation as [`Phi2Table`]. Generation is offline;
/// embed the resulting array as `static` program read-only data rather than
/// constructing or copying it on the SBF stack.
///
/// At rho=0.75, the analytic interpolation component of the certificate is
/// about 4.14e-4, versus about 1.70e-3 for the compatibility 64×64 table.
pub struct Phi2DenseTable {
    values: [[i32; DENSE_N]; DENSE_N],
}

impl Phi2DenseTable {
    /// Create an uncertified dense table from a pre-generated 129×129 array.
    pub const fn from_array(values: [[i32; DENSE_N]; DENSE_N]) -> Self {
        Self { values }
    }

    /// Borrow the raw dense grid for offline code generation/embedding.
    pub const fn as_array(&self) -> &[[i32; DENSE_N]; DENSE_N] {
        &self.values
    }

    /// Uncertified compatibility evaluation. Inputs outside `[-4, 4]²` clamp.
    pub fn eval(&self, a: i128, b: i128) -> Result<i128, SolMathError> {
        eval_bilinear(&self.values, a, b)
    }

    /// Verify dense certificate metadata and return a row-authenticating guard.
    pub fn certified<'a>(
        &'a self,
        certificate: &'a Phi2Certificate,
        expected_certificate_id: [u8; 32],
        max_abs_error_budget: i128,
    ) -> Result<CertifiedPhi2Evaluator<'a, DENSE_N>, SolMathError> {
        verify_certificate::<DENSE_N>(certificate, expected_certificate_id, max_abs_error_budget)?;
        Ok(CertifiedPhi2Evaluator {
            values: &self.values,
            certificate,
        })
    }

    /// Verify and evaluate the dense table once under an explicit error budget.
    #[allow(clippy::too_many_arguments)]
    pub fn eval_certified(
        &self,
        a: i128,
        b: i128,
        certificate: &Phi2Certificate,
        expected_certificate_id: [u8; 32],
        max_abs_error_budget: i128,
    ) -> Result<i128, SolMathError> {
        self.certified(certificate, expected_certificate_id, max_abs_error_budget)?
            .eval(a, b)
    }

    /// Exhaustively certify this exact dense table against GL20 at every node.
    #[cfg(feature = "table-gen")]
    pub fn certify(&self, rho: i128) -> Result<Phi2Certificate, SolMathError> {
        certify_grid(&self.values, rho)
    }

    /// Generate an uncertified 129×129 table offline.
    #[cfg(feature = "table-gen")]
    pub fn generate(rho: i128, n: usize) -> Result<Self, SolMathError> {
        if n != DENSE_N {
            return Err(SolMathError::DomainError);
        }
        Ok(Self {
            values: generate_grid(rho)?,
        })
    }

    /// Generate and then certify a 129×129 table offline.
    #[cfg(feature = "table-gen")]
    pub fn generate_certified(
        rho: i128,
        n: usize,
    ) -> Result<(Self, Phi2Certificate), SolMathError> {
        let table = Self::generate(rho, n)?;
        let certificate = table.certify(rho)?;
        Ok((table, certificate))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn certificate_hash_uses_standard_sha256() {
        let mut hash = Sha256::new();
        hash.update(b"abc");
        assert_eq!(
            hash.finish(),
            [
                0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae,
                0x22, 0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61,
                0xf2, 0x00, 0x15, 0xad,
            ]
        );
        // NIST's two-block vector exercises padding across a block boundary,
        // which is also the path used by row commitments.
        let mut long_hash = Sha256::new();
        long_hash.update(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq");
        assert_eq!(
            long_hash.finish(),
            [
                0x24, 0x8d, 0x6a, 0x61, 0xd2, 0x06, 0x38, 0xb8, 0xe5, 0xc0, 0x26, 0x93, 0x0c, 0x3e,
                0x60, 0x39, 0xa3, 0x3c, 0xe4, 0x59, 0x64, 0xff, 0x21, 0x67, 0xf6, 0xec, 0xed, 0xd4,
                0x19, 0xdb, 0x06, 0xc1,
            ]
        );
        // Cross-language serialization vector produced independently with
        // Python's hashlib for a 129-element all-zero row at index zero.
        assert_eq!(
            row_digest(&[0i32; DENSE_N], 0),
            [
                0xe2, 0x1b, 0xe0, 0x59, 0x4e, 0xf0, 0xf6, 0x24, 0x05, 0xc5, 0x35, 0x04, 0x38, 0x1a,
                0x7e, 0x6b,
            ]
        );
        assert!(core::mem::size_of::<Phi2Certificate>() < 3_000);
        assert_eq!(
            core::mem::size_of::<Phi2DenseTable>(),
            DENSE_N * DENSE_N * core::mem::size_of::<i32>()
        );
    }

    #[cfg(feature = "table-gen")]
    const TEST_RHO: i128 = 750_000_000_000;

    #[cfg(feature = "table-gen")]
    fn dense_test_table() -> &'static (Phi2DenseTable, Phi2Certificate) {
        use std::sync::OnceLock;
        static TABLE: OnceLock<(Phi2DenseTable, Phi2Certificate)> = OnceLock::new();
        TABLE.get_or_init(|| {
            Phi2DenseTable::generate_certified(TEST_RHO, DENSE_N)
                .expect("dense test table generation")
        })
    }

    #[cfg(feature = "table-gen")]
    fn compatibility_test_table() -> &'static (Phi2Table, Phi2Certificate) {
        use std::sync::OnceLock;
        static TABLE: OnceLock<(Phi2Table, Phi2Certificate)> = OnceLock::new();
        TABLE.get_or_init(|| {
            Phi2Table::generate_certified(TEST_RHO, N).expect("compatibility test table generation")
        })
    }

    // Independent test-only reference: Phi(a)Phi(b) plus numerical Simpson
    // integration of Plackett's d Phi2 / d rho identity. It shares no fixed-
    // point arithmetic, quadrature nodes, or quadrant folding with bvn_cdf_hp.
    #[cfg(feature = "table-gen")]
    fn normal_cdf_reference(x: f64) -> f64 {
        let z = x / core::f64::consts::SQRT_2;
        let t = 1.0 / (1.0 + 0.5 * z.abs());
        let tau = t
            * (-z * z - 1.265_512_23
                + t * (1.000_023_68
                    + t * (0.374_091_96
                        + t * (0.096_784_18
                            + t * (-0.186_288_06
                                + t * (0.278_868_07
                                    + t * (-1.135_203_98
                                        + t * (1.488_515_87
                                            + t * (-0.822_152_23 + t * 0.170_872_77)))))))))
                .exp();
        let erf = if z >= 0.0 { 1.0 - tau } else { tau - 1.0 };
        0.5 * (1.0 + erf)
    }

    #[cfg(feature = "table-gen")]
    fn bvn_independent_reference(a: f64, b: f64, rho: f64) -> f64 {
        const PANELS: usize = 512;
        let base = normal_cdf_reference(a) * normal_cdf_reference(b);
        if rho == 0.0 {
            return base;
        }
        let integrand = |r: f64| {
            let one_minus_r2 = 1.0 - r * r;
            let exponent = -(a * a - 2.0 * r * a * b + b * b) / (2.0 * one_minus_r2);
            exponent.exp() / (2.0 * core::f64::consts::PI * one_minus_r2.sqrt())
        };
        let step = rho / PANELS as f64;
        let mut weighted = integrand(0.0) + integrand(rho);
        for index in 1..PANELS {
            let weight = if index % 2 == 0 { 2.0 } else { 4.0 };
            weighted += weight * integrand(index as f64 * step);
        }
        (base + step * weighted / 3.0).clamp(0.0, 1.0)
    }

    #[cfg(feature = "table-gen")]
    fn reference_raw(a: i128, b: i128, rho: i128) -> i128 {
        let value = bvn_independent_reference(
            a as f64 / SCALE_I as f64,
            b as f64 / SCALE_I as f64,
            rho as f64 / SCALE_I as f64,
        );
        (value * SCALE_I as f64).round() as i128
    }

    /// Smoke test: generate a table and eval at the origin.
    #[cfg(feature = "table-gen")]
    #[test]
    fn phi2table_generate_and_eval() {
        let table = Phi2Table::generate(500_000_000_000i128, 64).unwrap();
        let v = table.eval(0, 0).unwrap();
        // Φ₂(0, 0; 0.5) ≈ 0.333 → ~333_000_000_000 at SCALE
        assert!(v > 300_000_000_000 && v < 370_000_000_000, "v={v}");
    }

    /// Regression M10: the top edge snapped its interpolation fraction back
    /// to 0, making eval(4, ·) < eval(3.9, ·).
    #[cfg(feature = "table-gen")]
    #[test]
    fn phi2table_edge_monotone() {
        let s: i128 = 1_000_000_000_000;
        let table = Phi2Table::generate(0, 64).unwrap();
        let near = table.eval(3_900_000_000_000, 0).unwrap();
        let edge = table.eval(4 * s, 0).unwrap();
        assert!(edge >= near, "edge {} < near {}", edge, near);
        let near_b = table.eval(0, 3_900_000_000_000).unwrap();
        let edge_b = table.eval(0, 4 * s).unwrap();
        assert!(edge_b >= near_b, "b-axis edge {} < near {}", edge_b, near_b);
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

    #[cfg(feature = "table-gen")]
    #[test]
    fn dense_certificate_meets_budget_that_compatibility_grid_rejects() {
        let (dense, dense_certificate) = dense_test_table();
        let (compatibility, compatibility_certificate) = compatibility_test_table();
        let economic_budget = 500_000_000i128; // 5e-4 probability.

        assert!(dense_certificate.max_abs_error() <= economic_budget);
        assert!(compatibility_certificate.max_abs_error() > economic_budget);
        assert!(dense
            .certified(
                dense_certificate,
                dense_certificate.certificate_id(),
                economic_budget,
            )
            .is_ok());
        assert!(matches!(
            compatibility.certified(
                compatibility_certificate,
                compatibility_certificate.certificate_id(),
                economic_budget,
            ),
            Err(SolMathError::NoConvergence)
        ));

        assert!(matches!(
            dense.certified(
                dense_certificate,
                dense_certificate.certificate_id(),
                dense_certificate.max_abs_error() - 1,
            ),
            Err(SolMathError::NoConvergence)
        ));
        assert!(matches!(
            dense.certified(dense_certificate, dense_certificate.certificate_id(), -1),
            Err(SolMathError::DomainError)
        ));
    }

    #[cfg(feature = "table-gen")]
    #[test]
    fn certificate_rejects_mismatch_corruption_and_recomputed_forgery() {
        let (table, certificate) = dense_test_table();
        let trusted_id = certificate.certificate_id();
        let budget = certificate.max_abs_error();

        let (_, wrong_grid_certificate) = compatibility_test_table();
        assert!(matches!(
            table.certified(
                wrong_grid_certificate,
                wrong_grid_certificate.certificate_id(),
                SCALE_I,
            ),
            Err(SolMathError::DomainError)
        ));

        let mut changed_values = table.values;
        changed_values[64][64] -= 1;
        let changed_table = Phi2DenseTable::from_array(changed_values);
        let changed_evaluator = changed_table
            .certified(certificate, trusted_id, budget)
            .expect("certificate metadata remains valid until the touched row is used");
        assert!(matches!(
            changed_evaluator.eval(0, 0),
            Err(SolMathError::DomainError)
        ));

        let mut corrupt = certificate.clone();
        corrupt.table_digest[0] ^= 1;
        assert!(matches!(
            table.certified(&corrupt, trusted_id, budget),
            Err(SolMathError::DomainError)
        ));

        // Model an attacker who lowers the node error and recomputes every
        // unkeyed digest. The independently pinned original ID still rejects.
        let mut forged = certificate.clone();
        forged.max_node_abs_error = 0;
        forged.max_abs_error =
            forged.interpolation_abs_error_bound + forged.reference_abs_error_allowance;
        forged.certificate_id = certificate_digest(&forged);
        assert_ne!(forged.certificate_id(), trusted_id);
        assert!(matches!(
            table.certified(&forged, trusted_id, budget),
            Err(SolMathError::DomainError)
        ));

        // Trusted offline parts remain embeddable without weakening checks.
        let embedded = Phi2Certificate::from_embedded_parts(
            certificate.rho(),
            certificate.grid_size() as u16,
            certificate.max_node_abs_error(),
            certificate.interpolation_abs_error_bound(),
            certificate.reference_abs_error_allowance(),
            certificate.max_abs_error(),
            *certificate.row_digests(),
            certificate.table_digest(),
            certificate.certificate_id(),
        );
        assert!(table.certified(&embedded, trusted_id, budget).is_ok());
    }

    #[cfg(feature = "table-gen")]
    #[test]
    fn certified_dense_eval_is_monotone_symmetric_and_domain_strict() {
        let (table, certificate) = dense_test_table();
        let evaluator = table
            .certified(
                certificate,
                certificate.certificate_id(),
                certificate.max_abs_error(),
            )
            .unwrap();
        let mut previous_rows = [0i128; 33];
        for i in 0..33 {
            let a = DOMAIN_MIN as i128 + RANGE as i128 * i as i128 / 32;
            let mut previous_in_row = 0i128;
            for j in 0..33 {
                let b = DOMAIN_MIN as i128 + RANGE as i128 * j as i128 / 32;
                let value = evaluator.eval(a, b).unwrap();
                assert!((0..=SCALE_I).contains(&value));
                if j > 0 {
                    assert!(value >= previous_in_row, "row monotonicity at {i},{j}");
                }
                if i > 0 {
                    assert!(value >= previous_rows[j], "column monotonicity at {i},{j}");
                }
                assert_eq!(value, evaluator.eval(b, a).unwrap());
                previous_in_row = value;
                previous_rows[j] = value;
            }
        }

        for &(a, b) in &[
            (DOMAIN_MIN as i128, DOMAIN_MIN as i128),
            (DOMAIN_MIN as i128, DOMAIN_MAX as i128),
            (DOMAIN_MAX as i128, DOMAIN_MIN as i128),
            (DOMAIN_MAX as i128, DOMAIN_MAX as i128),
        ] {
            let actual = evaluator.eval(a, b).unwrap();
            let reference = reference_raw(a, b, TEST_RHO);
            assert!(
                actual.abs_diff(reference) <= certificate.max_abs_error() as u128,
                "endpoint ({a},{b}) actual={actual} reference={reference} bound={}",
                certificate.max_abs_error()
            );
        }
        assert_eq!(
            evaluator.eval(DOMAIN_MIN as i128 - 1, 0),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            evaluator.eval(0, DOMAIN_MAX as i128 + 1),
            Err(SolMathError::DomainError)
        );
    }

    #[cfg(feature = "table-gen")]
    #[test]
    fn independent_reference_measured_error_is_within_certificate() {
        let (dense, dense_certificate) = dense_test_table();
        let (compatibility, compatibility_certificate) = compatibility_test_table();
        let dense_eval = dense
            .certified(
                dense_certificate,
                dense_certificate.certificate_id(),
                dense_certificate.max_abs_error(),
            )
            .unwrap();
        let compatibility_eval = compatibility
            .certified(
                compatibility_certificate,
                compatibility_certificate.certificate_id(),
                compatibility_certificate.max_abs_error(),
            )
            .unwrap();
        let mut dense_max = 0u128;
        let mut compatibility_max = 0u128;
        let mut worst = (0i128, 0i128);
        let mut dense_errors = std::vec::Vec::with_capacity(41 * 41);
        let mut compatibility_errors = std::vec::Vec::with_capacity(41 * 41);

        // Off-grid deterministic lattice, avoiding a validation corpus made
        // only of the same nodes used during certificate generation.
        for i in 0..41 {
            let a = DOMAIN_MIN as i128 + ((RANGE as i128 * (2 * i + 1) as i128) / 82);
            for j in 0..41 {
                let b = DOMAIN_MIN as i128 + ((RANGE as i128 * (2 * j + 1) as i128) / 82);
                let reference = reference_raw(a, b, TEST_RHO);
                let dense_error = dense_eval.eval(a, b).unwrap().abs_diff(reference);
                let compatibility_error =
                    compatibility_eval.eval(a, b).unwrap().abs_diff(reference);
                if dense_error > dense_max {
                    dense_max = dense_error;
                    worst = (a, b);
                }
                compatibility_max = compatibility_max.max(compatibility_error);
                dense_errors.push(dense_error);
                compatibility_errors.push(compatibility_error);
            }
        }

        dense_errors.sort_unstable();
        compatibility_errors.sort_unstable();
        let median_index = dense_errors.len() / 2;
        let p99_index = (dense_errors.len() - 1) * 99 / 100;
        let dense_median = dense_errors[median_index];
        let dense_p99 = dense_errors[p99_index];
        let compatibility_median = compatibility_errors[median_index];
        let compatibility_p99 = compatibility_errors[p99_index];

        std::println!(
            "Phi2 rho=.75 independent 41x41 off-grid: dense median={} p99={} max={} ({:.9e}); 64x64 median={} p99={} max={} ({:.9e}); dense_cert={} = node {} + interpolation {} + GL20 allowance {}; 64x64_cert={} = node {} + interpolation {} + GL20 allowance {}; worst=({},{})",
            dense_median,
            dense_p99,
            dense_max,
            dense_max as f64 / SCALE_I as f64,
            compatibility_median,
            compatibility_p99,
            compatibility_max,
            compatibility_max as f64 / SCALE_I as f64,
            dense_certificate.max_abs_error(),
            dense_certificate.max_node_abs_error(),
            dense_certificate.interpolation_abs_error_bound(),
            dense_certificate.reference_abs_error_allowance(),
            compatibility_certificate.max_abs_error(),
            compatibility_certificate.max_node_abs_error(),
            compatibility_certificate.interpolation_abs_error_bound(),
            compatibility_certificate.reference_abs_error_allowance(),
            worst.0,
            worst.1,
        );
        assert!(dense_max <= dense_certificate.max_abs_error() as u128);
        assert!(compatibility_max <= compatibility_certificate.max_abs_error() as u128);
        assert!(dense_max < compatibility_max / 2);

        // At the origin an independent closed form is available:
        // Phi2(0,0;rho) = 1/4 + asin(rho)/(2*pi).
        let expected_origin = ((0.25 + 0.75f64.asin() / (2.0 * core::f64::consts::PI))
            * SCALE_I as f64)
            .round() as i128;
        assert!(
            dense_eval.eval(0, 0).unwrap().abs_diff(expected_origin) <= SHIFT as u128,
            "origin table={} reference={expected_origin}",
            dense_eval.eval(0, 0).unwrap()
        );
    }

    #[cfg(feature = "table-gen")]
    #[test]
    fn certification_rejects_bad_grid_and_singular_rho() {
        let mut values = [[0i32; N]; N];
        values[0][0] = 1;
        let non_monotone = Phi2Table::from_array(values);
        assert_eq!(non_monotone.certify(0), Err(SolMathError::DomainError));
        assert!(matches!(
            compatibility_test_table().0.certify(MAX_CERTIFIED_RHO + 1),
            Err(SolMathError::DomainError)
        ));
    }
}
