use crate::arithmetic::{
    cmp_wide, fp_div, fp_div_i, fp_mul, fp_mul_i, fp_mul_i_fast, fp_sqrt, wide_mul_u128,
};
use crate::constants::*;
use crate::error::SolMathError;
use crate::hp::{bs_full_hp, pow_fixed_hp};
use crate::transcendental::{exp_fixed_i, ln_fixed_i};

// ============================================================
// Whole-surface certification API
// ============================================================

/// Maximum number of strikes accepted by [`certify_sabr_surface`].
pub const MAX_SABR_SURFACE_STRIKES: usize = 32;

/// Maximum number of maturities accepted by [`certify_sabr_surface`].
pub const MAX_SABR_SURFACE_MATURITIES: usize = 16;

/// Maximum number of quote pairs accepted by [`certify_sabr_surface`].
///
/// These limits bound all validation loops and discount-factor evaluations.
/// Raise them only after re-metering the target SBF program.
pub const MAX_SABR_SURFACE_QUOTES: usize = 256;

/// One immutable quote read from a [`CertifiedSabrSurface`].
///
/// Values are at [`SCALE`]. The quote is returned from the caller-supplied
/// surface that was certified atomically; no SABR repricing occurs here. Its
/// fields are private so safe callers cannot forge a certified execution
/// value. Execution entrypoints should accept this type, or a certificate and
/// grid indices, instead of accepting raw quote fields independently.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CertifiedSabrQuote {
    spot: u128,
    rate: u128,
    strike: u128,
    maturity: u128,
    call: u128,
    put: u128,
}

impl CertifiedSabrQuote {
    /// Spot against which this quote was certified.
    pub fn spot(&self) -> u128 {
        self.spot
    }

    /// Rate against which this quote was certified.
    pub fn rate(&self) -> u128 {
        self.rate
    }

    /// Strike of this certified grid node.
    pub fn strike(&self) -> u128 {
        self.strike
    }

    /// Maturity of this certified grid node.
    pub fn maturity(&self) -> u128 {
        self.maturity
    }

    /// Stored certified call value.
    pub fn call(&self) -> u128 {
        self.call
    }

    /// Stored certified put value.
    pub fn put(&self) -> u128 {
        self.put
    }
}

/// Borrowed proof that a complete rectangular option grid passed static
/// no-arbitrage validation.
///
/// The underlying slices remain immutably borrowed for the certificate's
/// lifetime, so safe Rust cannot mutate certified values before execution
/// consumes them. Quotes are stored maturity-major: index
/// `maturity_index * strike_count + strike_index`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CertifiedSabrSurface<'a> {
    spot: u128,
    rate: u128,
    strikes: &'a [u128],
    maturities: &'a [u128],
    calls: &'a [u128],
    puts: &'a [u128],
}

impl<'a> CertifiedSabrSurface<'a> {
    /// Spot shared by the certified grid.
    pub fn spot(&self) -> u128 {
        self.spot
    }

    /// Non-negative continuously-compounded rate shared by the grid.
    pub fn rate(&self) -> u128 {
        self.rate
    }

    /// Strictly increasing certified strike axis.
    pub fn strikes(&self) -> &'a [u128] {
        self.strikes
    }

    /// Strictly increasing certified maturity axis.
    pub fn maturities(&self) -> &'a [u128] {
        self.maturities
    }

    /// Number of immutable quotes in the certificate.
    pub fn quote_count(&self) -> usize {
        self.calls.len()
    }

    /// Read a certified quote by `(maturity_index, strike_index)`.
    ///
    /// This returns the stored, certified value and never calls
    /// [`sabr_price`]. Invalid indices return `DomainError`.
    pub fn quote_at(
        &self,
        maturity_index: usize,
        strike_index: usize,
    ) -> Result<CertifiedSabrQuote, SolMathError> {
        if maturity_index >= self.maturities.len() || strike_index >= self.strikes.len() {
            return Err(SolMathError::DomainError);
        }
        let index = maturity_index
            .checked_mul(self.strikes.len())
            .and_then(|v| v.checked_add(strike_index))
            .ok_or(SolMathError::Overflow)?;
        Ok(CertifiedSabrQuote {
            spot: self.spot,
            rate: self.rate,
            strike: self.strikes[strike_index],
            maturity: self.maturities[maturity_index],
            call: self.calls[index],
            put: self.puts[index],
        })
    }
}

/// Atomically certify a complete ordered SABR quote surface.
///
/// `calls` and `puts` must be rectangular, maturity-major grids with exactly
/// `maturities.len() * strikes.len()` entries. The function requires at least
/// three strikes (so butterflies can be checked) and two maturities (so
/// calendars can be checked). It validates every entry before constructing the
/// returned immutable certificate. Axes and quote count must not exceed
/// [`MAX_SABR_SURFACE_STRIKES`], [`MAX_SABR_SURFACE_MATURITIES`], and
/// [`MAX_SABR_SURFACE_QUOTES`]; resource-limit violations return `DomainError`.
///
/// The certificate enforces:
/// - strictly increasing, non-zero strikes and strictly increasing maturities;
/// - exact fixed-point put-call parity against `K * exp(-rT)`;
/// - hard call and put bounds;
/// - calls non-increasing and puts non-decreasing in strike;
/// - non-negative call and put butterflies on discounted, irregular strike
///   spacing, including the synthetic zero-strike values `(call=spot, put=0)`;
///   and
/// - piecewise-linear calls non-decreasing in maturity at equal discounted
///   strike over adjacent rows' common quoted support (non-negative rates, no
///   dividends).
///
/// This certifies the supplied values, not their model provenance. Generate or
/// calibrate SABR quotes off-chain, project rounding noise to the exact static
/// constraints, certify once, then execute only values read via
/// [`CertifiedSabrSurface::quote_at`].
pub fn certify_sabr_surface<'a>(
    spot: u128,
    rate: u128,
    strikes: &'a [u128],
    maturities: &'a [u128],
    calls: &'a [u128],
    puts: &'a [u128],
) -> Result<CertifiedSabrSurface<'a>, SolMathError> {
    if spot > i128::MAX as u128 || rate > i128::MAX as u128 {
        return Err(SolMathError::Overflow);
    }
    if spot == 0 {
        return Err(SolMathError::DomainError);
    }
    if strikes.len() < 3
        || maturities.len() < 2
        || strikes.len() > MAX_SABR_SURFACE_STRIKES
        || maturities.len() > MAX_SABR_SURFACE_MATURITIES
    {
        return Err(SolMathError::DomainError);
    }
    let quote_count = strikes
        .len()
        .checked_mul(maturities.len())
        .ok_or(SolMathError::Overflow)?;
    if quote_count > MAX_SABR_SURFACE_QUOTES
        || calls.len() != quote_count
        || puts.len() != quote_count
    {
        return Err(SolMathError::DomainError);
    }

    let mut strike_index = 0usize;
    while strike_index < strikes.len() {
        let strike = strikes[strike_index];
        if strike > i128::MAX as u128 {
            return Err(SolMathError::Overflow);
        }
        if strike == 0 {
            return Err(SolMathError::DomainError);
        }
        if strike_index > 0 && strike <= strikes[strike_index - 1] {
            return Err(SolMathError::DomainError);
        }
        strike_index += 1;
    }

    let mut discounts = [0i128; MAX_SABR_SURFACE_MATURITIES];
    let mut maturity_index = 0usize;
    while maturity_index < maturities.len() {
        let maturity = maturities[maturity_index];
        if maturity > i128::MAX as u128 {
            return Err(SolMathError::Overflow);
        }
        if maturity_index > 0 && maturity <= maturities[maturity_index - 1] {
            return Err(SolMathError::DomainError);
        }

        let discount = sabr_surface_discount(rate, maturity)?;
        discounts[maturity_index] = discount;
        let mut discounted_strikes = [0u128; MAX_SABR_SURFACE_STRIKES];
        fill_discounted_strikes(strikes, discount, &mut discounted_strikes)?;
        let row_start = maturity_index
            .checked_mul(strikes.len())
            .ok_or(SolMathError::Overflow)?;
        let mut column = 0usize;
        while column < strikes.len() {
            let index = row_start
                .checked_add(column)
                .ok_or(SolMathError::Overflow)?;
            validate_sabr_surface_quote(
                spot,
                discounted_strikes[column],
                calls[index],
                puts[index],
            )?;
            column += 1;
        }
        let row_end = row_start
            .checked_add(strikes.len())
            .ok_or(SolMathError::Overflow)?;
        validate_sabr_surface_row(
            spot,
            &discounted_strikes[..strikes.len()],
            &calls[row_start..row_end],
            &puts[row_start..row_end],
        )?;
        maturity_index += 1;
    }

    // With a non-negative rate and no dividends, the discounted stock is a
    // martingale. Calendar comparisons therefore use equal discounted strike,
    // not equal nominal strike. Checking both rows' breakpoints is sufficient:
    // their piecewise-linear difference is linear between the union points.
    maturity_index = 1;
    while maturity_index < maturities.len() {
        let previous = (maturity_index - 1)
            .checked_mul(strikes.len())
            .ok_or(SolMathError::Overflow)?;
        let current = maturity_index
            .checked_mul(strikes.len())
            .ok_or(SolMathError::Overflow)?;
        validate_sabr_surface_calendar(
            spot,
            strikes,
            discounts[maturity_index - 1],
            &calls[previous..previous + strikes.len()],
            discounts[maturity_index],
            &calls[current..current + strikes.len()],
        )?;
        maturity_index += 1;
    }

    Ok(CertifiedSabrSurface {
        spot,
        rate,
        strikes,
        maturities,
        calls,
        puts,
    })
}

fn sabr_surface_discount(rate: u128, maturity: u128) -> Result<i128, SolMathError> {
    let rt = fp_mul_i(rate as i128, maturity as i128)?;
    exp_fixed_i(rt.checked_neg().ok_or(SolMathError::Overflow)?)
}

fn fill_discounted_strikes(
    strikes: &[u128],
    discount: i128,
    output: &mut [u128; MAX_SABR_SURFACE_STRIKES],
) -> Result<(), SolMathError> {
    if discount <= 0 {
        return Err(SolMathError::NoConvergence);
    }
    let mut index = 0usize;
    while index < strikes.len() {
        let discounted = fp_mul_i(strikes[index] as i128, discount)?;
        if discounted <= 0 {
            return Err(SolMathError::NoConvergence);
        }
        let discounted = discounted as u128;
        if index > 0 && discounted <= output[index - 1] {
            return Err(SolMathError::NoConvergence);
        }
        output[index] = discounted;
        index += 1;
    }
    Ok(())
}

fn validate_sabr_surface_quote(
    spot: u128,
    discounted_strike: u128,
    call: u128,
    put: u128,
) -> Result<(), SolMathError> {
    let call_lower = spot.saturating_sub(discounted_strike);
    let put_lower = discounted_strike.saturating_sub(spot);
    if call < call_lower || call > spot || put < put_lower || put > discounted_strike {
        return Err(SolMathError::NoConvergence);
    }
    let parity_left = call
        .checked_add(discounted_strike)
        .ok_or(SolMathError::Overflow)?;
    let parity_right = put.checked_add(spot).ok_or(SolMathError::Overflow)?;
    if parity_left != parity_right {
        return Err(SolMathError::NoConvergence);
    }
    Ok(())
}

fn validate_sabr_surface_row(
    spot: u128,
    discounted_strikes: &[u128],
    calls: &[u128],
    puts: &[u128],
) -> Result<(), SolMathError> {
    let mut i = 1usize;
    while i < discounted_strikes.len() {
        if calls[i] > calls[i - 1] || puts[i] < puts[i - 1] {
            return Err(SolMathError::NoConvergence);
        }
        i += 1;
    }

    let mut strike_left = discounted_strikes[0];
    let mut call_drop_left = spot
        .checked_sub(calls[0])
        .ok_or(SolMathError::NoConvergence)?;
    let mut put_rise_left = puts[0];
    i = 0;
    while i + 1 < discounted_strikes.len() {
        let strike_right = discounted_strikes[i + 1]
            .checked_sub(discounted_strikes[i])
            .ok_or(SolMathError::Overflow)?;
        let call_drop_right = calls[i]
            .checked_sub(calls[i + 1])
            .ok_or(SolMathError::NoConvergence)?;
        let call_left = wide_mul_u128(call_drop_left, strike_right);
        let call_right = wide_mul_u128(call_drop_right, strike_left);
        if cmp_wide(call_left, call_right).is_lt() {
            return Err(SolMathError::NoConvergence);
        }

        let put_rise_right = puts[i + 1]
            .checked_sub(puts[i])
            .ok_or(SolMathError::NoConvergence)?;
        let put_left = wide_mul_u128(put_rise_left, strike_right);
        let put_right = wide_mul_u128(put_rise_right, strike_left);
        if cmp_wide(put_left, put_right).is_gt() {
            return Err(SolMathError::NoConvergence);
        }
        strike_left = strike_right;
        call_drop_left = call_drop_right;
        put_rise_left = put_rise_right;
        i += 1;
    }
    Ok(())
}

fn add_wide(left: (u128, u128), right: (u128, u128)) -> Result<(u128, u128), SolMathError> {
    let (low, carry) = left.1.overflowing_add(right.1);
    let high = left
        .0
        .checked_add(right.0)
        .and_then(|value| value.checked_add(carry as u128))
        .ok_or(SolMathError::Overflow)?;
    Ok((high, low))
}

fn compare_piecewise_call_to_value(
    spot: u128,
    discounted_strikes: &[u128],
    calls: &[u128],
    discounted_strike: u128,
    value: u128,
) -> Result<core::cmp::Ordering, SolMathError> {
    if discounted_strikes.len() != calls.len()
        || discounted_strikes.is_empty()
        || discounted_strike > discounted_strikes[discounted_strikes.len() - 1]
    {
        return Err(SolMathError::DomainError);
    }

    let mut right_index = 0usize;
    while discounted_strikes[right_index] < discounted_strike {
        right_index += 1;
    }
    if discounted_strikes[right_index] == discounted_strike {
        return Ok(calls[right_index].cmp(&value));
    }

    let (left_strike, left_call) = if right_index == 0 {
        (0, spot)
    } else {
        (discounted_strikes[right_index - 1], calls[right_index - 1])
    };
    let right_strike = discounted_strikes[right_index];
    let right_call = calls[right_index];
    let width = right_strike
        .checked_sub(left_strike)
        .ok_or(SolMathError::Overflow)?;
    let left_weight = right_strike
        .checked_sub(discounted_strike)
        .ok_or(SolMathError::Overflow)?;
    let right_weight = discounted_strike
        .checked_sub(left_strike)
        .ok_or(SolMathError::Overflow)?;
    let numerator = add_wide(
        wide_mul_u128(left_call, left_weight),
        wide_mul_u128(right_call, right_weight),
    )?;
    Ok(cmp_wide(numerator, wide_mul_u128(value, width)))
}

fn validate_sabr_surface_calendar(
    spot: u128,
    strikes: &[u128],
    previous_discount: i128,
    previous_calls: &[u128],
    current_discount: i128,
    current_calls: &[u128],
) -> Result<(), SolMathError> {
    let mut previous_strikes = [0u128; MAX_SABR_SURFACE_STRIKES];
    let mut current_strikes = [0u128; MAX_SABR_SURFACE_STRIKES];
    fill_discounted_strikes(strikes, previous_discount, &mut previous_strikes)?;
    fill_discounted_strikes(strikes, current_discount, &mut current_strikes)?;
    let previous_strikes = &previous_strikes[..strikes.len()];
    let current_strikes = &current_strikes[..strikes.len()];
    let common_max = previous_strikes[previous_strikes.len() - 1]
        .min(current_strikes[current_strikes.len() - 1]);

    // At every previous-row breakpoint in the common support, the later curve
    // must be at least the earlier stored call.
    let mut index = 0usize;
    while index < previous_strikes.len() && previous_strikes[index] <= common_max {
        if compare_piecewise_call_to_value(
            spot,
            current_strikes,
            current_calls,
            previous_strikes[index],
            previous_calls[index],
        )?
        .is_lt()
        {
            return Err(SolMathError::NoConvergence);
        }
        index += 1;
    }

    // At every later-row breakpoint in the common support, the earlier curve
    // must be no greater than the later stored call. Together with the loop
    // above this checks the union of all piecewise-linear breakpoints.
    index = 0;
    while index < current_strikes.len() && current_strikes[index] <= common_max {
        if compare_piecewise_call_to_value(
            spot,
            previous_strikes,
            previous_calls,
            current_strikes[index],
            current_calls[index],
        )?
        .is_gt()
        {
            return Err(SolMathError::NoConvergence);
        }
        index += 1;
    }
    Ok(())
}

// ============================================================
// Single-strike API
// ============================================================

/// SABR implied Black volatility (Hagan 2002 + Obloj 2008 correction).
///
/// Returns sigma_B at SCALE such that BS(F, K, sigma_B, T) approximates the SABR price.
///
/// # Parameters
/// All at SCALE (`u128`) except `rho` (`i128`):
/// - `f` -- Forward price
/// - `k` -- Strike price
/// - `t` -- Time to expiry (years)
/// - `alpha` -- Initial volatility
/// - `beta` -- CEV exponent in `[0, SCALE]` (0 = normal, SCALE = lognormal)
/// - `rho` -- Correlation in `(-SCALE, SCALE)`
/// - `nu` -- Vol of vol
///
/// # Returns
/// Implied Black vol at SCALE. Returns 0 for zero inputs.
///
/// # Accuracy and CU
/// The final 100,000-case corpus observed max/P99/median errors 660/102/3 raw
/// volatility units. Final SBF audit: 51,516 CU average, 74,338 max.
///
/// # Example
/// ```
/// use solmath::{sabr_implied_vol, SCALE};
/// // ATM SABR vol: F=100, K=100, T=1yr, alpha=0.2, beta=0.5, rho=-0.3, nu=0.4
/// let vol = sabr_implied_vol(
///     100 * SCALE, 100 * SCALE, SCALE,
///     200_000_000_000, SCALE / 2, -300_000_000_000, 400_000_000_000,
/// )?;
/// assert!(vol > 0);
/// # Ok::<(), solmath::SolMathError>(())
/// ```
pub fn sabr_implied_vol(
    f: u128,
    k: u128,
    t: u128,
    alpha: u128,
    beta: u128,
    rho: i128,
    nu: u128,
) -> Result<u128, SolMathError> {
    if f > i128::MAX as u128
        || k > i128::MAX as u128
        || t > i128::MAX as u128
        || alpha > i128::MAX as u128
        || beta > i128::MAX as u128
        || nu > i128::MAX as u128
    {
        return Err(SolMathError::Overflow);
    }
    if f == 0 || k == 0 || t == 0 || alpha == 0 {
        return Ok(0);
    }
    if beta > SCALE {
        return Err(SolMathError::DomainError);
    }
    if rho <= -SCALE_I || rho >= SCALE_I {
        return Err(SolMathError::DomainError);
    }
    let s = SCALE;
    let si = SCALE_I;

    // s = SCALE = 1e12, beta ∈ [0, SCALE]; one_minus_beta ∈ [0, SCALE]. No underflow (u128).
    let one_minus_beta = s - beta;
    let beta_i = beta as i128;
    let alpha_i = alpha as i128;
    let nu_i = nu as i128;

    // Use the ATM limit only at exact ATM. A tolerance branch introduced a
    // discontinuity in executable quotes at its boundary.
    let is_atm = f == k;

    if is_atm {
        let f_pow = if one_minus_beta == 0 {
            s
        } else if one_minus_beta == s {
            f
        } else {
            pow_fixed_hp(f, one_minus_beta)?
        };

        let base_vol = fp_div_i(alpha_i, f_pow as i128)?;

        let h = compute_h(one_minus_beta, alpha, beta_i, rho, nu_i, alpha_i, f_pow)?;
        // h ∈ [-SCALE_I, SCALE_I] by design; fp_mul_i(h, t) ≤ SCALE_I for t ≤ SCALE_I;
        // si + correction ≤ 2·SCALE_I. Fits i128.
        let correction = si
            .checked_add(fp_mul_i(h, t as i128)?)
            .ok_or(SolMathError::Overflow)?;
        if correction <= 0 {
            return Err(SolMathError::NoConvergence);
        }
        let sigma_i = fp_mul_i(base_vol, correction)?;
        return if sigma_i > 0 {
            Ok(sigma_i as u128)
        } else {
            Err(SolMathError::NoConvergence)
        };
    }

    // --- General (OTM/ITM) formula ---

    let f_mid = fp_sqrt(fp_mul(f, k)?)?;

    let f_mid_pow = if one_minus_beta == 0 {
        s
    } else if one_minus_beta == s {
        f_mid
    } else {
        pow_fixed_hp(f_mid, one_minus_beta)?
    };

    // log(F/K) via single ln — saves ~5K CU vs two separate ln calls
    let fk_ratio = fp_div(f, k)?;
    let log_fk = ln_fixed_i(fk_ratio)?;

    sabr_assemble(
        f_mid_pow,
        log_fk,
        one_minus_beta,
        alpha,
        alpha_i,
        beta_i,
        rho,
        nu_i,
        t,
    )
}

/// SABR European option price via implied vol into Black-Scholes.
///
/// Computes `sabr_implied_vol` then prices with `bs_full_hp`.
///
/// # Parameters
/// All at SCALE (`u128`) except `rho` (`i128`):
/// - `s` -- Spot price
/// - `k` -- Strike price
/// - `r` -- Risk-free rate
/// - `t` -- Time to expiry (years)
/// - `alpha`, `beta`, `rho`, `nu` -- SABR parameters (see [`sabr_implied_vol`])
///
/// # Returns
/// `(call, put)` prices at SCALE.
///
/// Internal component of the guarded public price; not a standalone CU
/// contract. Meter [`sabr_price`] in the consuming program.
fn sabr_price_raw(
    s: u128,
    k: u128,
    r: u128,
    t: u128,
    alpha: u128,
    beta: u128,
    rho: i128,
    nu: u128,
) -> Result<(u128, u128), SolMathError> {
    if s > i128::MAX as u128
        || k > i128::MAX as u128
        || r > i128::MAX as u128
        || t > i128::MAX as u128
        || alpha > i128::MAX as u128
        || beta > i128::MAX as u128
        || nu > i128::MAX as u128
    {
        return Err(SolMathError::Overflow);
    }
    if fp_mul(fp_mul(nu, nu)?, t)? > SCALE / 2 {
        // Executable prices fail closed outside the supported first-order
        // asymptotic regime. Analytics-only implied vol remains available.
        return Err(SolMathError::NoConvergence);
    }
    let r_t = fp_mul_i(r as i128, t as i128)?;
    let f = fp_mul_i(s as i128, exp_fixed_i(r_t)?)? as u128;
    let sigma = sabr_implied_vol(f, k, t, alpha, beta, rho, nu)?;
    if sigma == 0 {
        let k_disc = fp_mul_i(k as i128, exp_fixed_i(-r_t)?)? as u128;
        let call = if s > k_disc { s - k_disc } else { 0 };
        let put = if k_disc > s { k_disc - s } else { 0 };
        return Ok((call, put));
    }
    let bs = bs_full_hp(s, k, r, sigma, t)?;
    Ok((bs.call, bs.put))
}

/// SABR European price with a local static-arbitrage guard.
///
/// In addition to hard Black-Scholes bounds, prices are sampled at adjacent
/// strikes and rejected if calls rise with strike or violate local convexity.
/// A calibrated quote grid still needs a full surface-level arbitrage check.
/// Value-bearing execution must assemble the complete grid, call
/// [`certify_sabr_surface`], and consume only [`CertifiedSabrQuote`] values;
/// this isolated return is analytics/input to that workflow, not a global
/// no-arbitrage certificate.
/// Executable pricing also requires `nu²T <= 0.5`; implied-vol analytics are
/// available outside that range but are not certified for execution.
/// This safety check evaluates three prices; budget approximately 3× the raw
/// single-strike SABR+BS cost.
/// Final accepted-case SBF audit: 390,137 CU average, 639,037 P99,
/// 643,745 max.
pub fn sabr_price(
    s: u128,
    k: u128,
    r: u128,
    t: u128,
    alpha: u128,
    beta: u128,
    rho: i128,
    nu: u128,
) -> Result<(u128, u128), SolMathError> {
    let price = sabr_price_raw(s, k, r, t, alpha, beta, rho, nu)?;
    validate_sabr_local(s, k, r, t, alpha, beta, rho, nu, price)?;
    Ok(price)
}

#[allow(clippy::too_many_arguments)]
fn validate_sabr_local(
    s: u128,
    k: u128,
    r: u128,
    t: u128,
    alpha: u128,
    beta: u128,
    rho: i128,
    nu: u128,
    price: (u128, u128),
) -> Result<(), SolMathError> {
    if k == 0 || t == 0 || s == 0 {
        return Ok(());
    }
    // Keep the three strikes exactly equally spaced, including for tiny K.
    // A saturating lower point with a larger fixed step invalidates the
    // discrete convexity inequality and falsely rejects valid prices.
    let step = (k / 1_000).max(1).min(k);
    let k_lo = k - step;
    let k_hi = k.checked_add(step).ok_or(SolMathError::Overflow)?;
    let lo = sabr_price_raw(s, k_lo, r, t, alpha, beta, rho, nu)?;
    let hi = sabr_price_raw(s, k_hi, r, t, alpha, beta, rho, nu)?;
    const ROUNDING_TOLERANCE: u128 = 1_000;
    if lo.0.saturating_add(ROUNDING_TOLERANCE) < price.0
        || price.0.saturating_add(ROUNDING_TOLERANCE) < hi.0
    {
        return Err(SolMathError::NoConvergence);
    }
    let chord =
        lo.0.checked_add(hi.0)
            .and_then(|v| v.checked_add(ROUNDING_TOLERANCE))
            .ok_or(SolMathError::Overflow)?;
    let twice = price.0.checked_mul(2).ok_or(SolMathError::Overflow)?;
    if chord < twice {
        return Err(SolMathError::NoConvergence);
    }
    Ok(())
}

/// Full SABR Greeks via BS(sigma_SABR). Sticky-strike sensitivities.
///
/// Computes SABR implied vol then returns the full [`BsFull`] struct
/// (call, put, delta, gamma, vega, theta, rho) from `bs_full_hp`.
///
/// # Parameters
/// Same as [`sabr_price`].
///
/// # Returns
/// [`BsFull`] with prices and all first-order Greeks at SCALE.
///
/// The embedded price has only the isolated local guard. Value-bearing
/// execution must separately certify the complete price grid with
/// [`certify_sabr_surface`] before acting on this output.
///
/// # CU Cost
/// Final accepted-case SBF audit: 390,165 CU average, 639,065 P99,
/// 643,773 max, including neighbouring safety quotes.
pub fn sabr_greeks(
    s: u128,
    k: u128,
    r: u128,
    t: u128,
    alpha: u128,
    beta: u128,
    rho: i128,
    nu: u128,
) -> Result<BsFull, SolMathError> {
    if s > i128::MAX as u128
        || k > i128::MAX as u128
        || r > i128::MAX as u128
        || t > i128::MAX as u128
        || alpha > i128::MAX as u128
        || beta > i128::MAX as u128
        || nu > i128::MAX as u128
    {
        return Err(SolMathError::Overflow);
    }
    // Greeks are undefined at expiry — force caller to handle the degenerate case
    if t == 0 {
        return Err(SolMathError::DomainError);
    }
    if fp_mul(fp_mul(nu, nu)?, t)? > SCALE / 2 {
        return Err(SolMathError::NoConvergence);
    }
    let r_t = fp_mul_i(r as i128, t as i128)?;
    let f = fp_mul_i(s as i128, exp_fixed_i(r_t)?)? as u128;
    let sigma = sabr_implied_vol(f, k, t, alpha, beta, rho, nu)?;
    if sigma == 0 {
        let k_disc = fp_mul_i(k as i128, exp_fixed_i(-r_t)?)? as u128;
        if s == k_disc {
            return Err(SolMathError::DomainError); // kink: delta is undefined
        }
        let r_kd = fp_mul_i(r as i128, k_disc as i128)?;
        let t_kd = fp_mul_i(t as i128, k_disc as i128)?;
        if s > k_disc {
            return Ok(BsFull {
                call: s - k_disc,
                put: 0,
                call_delta: SCALE_I,
                put_delta: 0,
                gamma: 0,
                vega: 0,
                call_theta: -r_kd,
                put_theta: 0,
                call_rho: t_kd,
                put_rho: 0,
            });
        }
        return Ok(BsFull {
            call: 0,
            put: k_disc - s,
            call_delta: 0,
            put_delta: -SCALE_I,
            gamma: 0,
            vega: 0,
            call_theta: 0,
            put_theta: r_kd,
            call_rho: 0,
            put_rho: -t_kd,
        });
    }
    let greeks = bs_full_hp(s, k, r, sigma, t)?;
    validate_sabr_local(s, k, r, t, alpha, beta, rho, nu, (greeks.call, greeks.put))?;
    Ok(greeks)
}

// ============================================================
// Batch smile API — precompute F-dependent work once
// ============================================================

/// Precomputed intermediates for pricing a SABR smile (multiple strikes, fixed F).
///
/// Created by `sabr_precompute`. Passed to `sabr_vol_at` for each strike.
/// All F-dependent quantities are cached: ln(F), F^(1-β), h numerators, ATM vol.
/// Final SBF audit: precompute averaged 31,633 CU (49,817 max) and each
/// `sabr_vol_at` averaged 29,609 CU (37,520 max).
#[derive(Clone, Copy)]
pub struct SabrSmile {
    f: u128,
    t: u128,
    ln_f: i128,
    one_minus_beta: u128,
    half_omb: i128,      // (1-β)/2 at SCALE
    half_omb_ln_f: i128, // (1-β)/2 · ln(F) — for exp-based f_mid_pow
    omb2: u128,
    omb4: u128,
    alpha_i: i128,
    rho: i128,
    nu_over_alpha: i128,
    h1_num: i128, // (1-β)²·α² — h1 numerator
    h2_num: i128, // ρ·β·ν·α — h2 numerator
    h3: i128,     // (2-3ρ²)·ν²/24 — strike-independent
    atm_vol: u128,
}

/// Precompute F-dependent SABR intermediates for batch smile pricing.
///
/// Final SBF audit: 31,633 CU average / 49,817 max. Then call
/// `sabr_vol_at`, measured at 29,609 average / 37,520 max per strike.
pub fn sabr_precompute(
    f: u128,
    t: u128,
    alpha: u128,
    beta: u128,
    rho: i128,
    nu: u128,
) -> Result<SabrSmile, SolMathError> {
    if f > i128::MAX as u128
        || t > i128::MAX as u128
        || alpha > i128::MAX as u128
        || beta > i128::MAX as u128
        || nu > i128::MAX as u128
    {
        return Err(SolMathError::Overflow);
    }
    if f == 0 || t == 0 || alpha == 0 {
        return Ok(SabrSmile {
            f: 0,
            t: 0,
            ln_f: 0,
            one_minus_beta: 0,
            half_omb: 0,
            half_omb_ln_f: 0,
            omb2: 0,
            omb4: 0,
            alpha_i: 0,
            rho: 0,
            nu_over_alpha: 0,
            h1_num: 0,
            h2_num: 0,
            h3: 0,
            atm_vol: 0,
        });
    }
    if beta > SCALE {
        return Err(SolMathError::DomainError);
    }
    if rho <= -SCALE_I || rho >= SCALE_I {
        return Err(SolMathError::DomainError);
    }
    let s = SCALE;
    let si = SCALE_I;
    // s = SCALE = 1e12, beta ∈ [0, SCALE]; one_minus_beta ∈ [0, SCALE]. No underflow (u128).
    let one_minus_beta = s - beta;
    let alpha_i = alpha as i128;
    let beta_i = beta as i128;
    let nu_i = nu as i128;

    let ln_f = ln_fixed_i(f)?;

    // F^(1-β) for ATM
    let f_pow = if one_minus_beta == 0 {
        s
    } else if one_minus_beta == s {
        f
    } else {
        pow_fixed_hp(f, one_minus_beta)?
    };

    let atm_base_vol = fp_div_i(alpha_i, f_pow as i128)?;

    // h building blocks
    let omb2 = fp_mul(one_minus_beta, one_minus_beta)?;
    let omb4 = fp_mul(omb2, omb2)?;
    let alpha2 = fp_mul(alpha, alpha)?;
    let rho2 = fp_mul_i_fast(rho, rho);

    let h1_num = fp_mul_i(omb2 as i128, alpha2 as i128)?;
    let h2_num = fp_mul_i(fp_mul_i(rho, beta_i)?, fp_mul_i(nu_i, alpha_i)?)?;
    // 2 * si - 3 * rho2: si = SCALE_I = 1e12, rho2 ∈ [0, SCALE_I]; result ∈ [-1e12, 2e12]. Fits i128.
    let h3 = fp_div_i(fp_mul_i(2 * si - 3 * rho2, fp_mul_i(nu_i, nu_i)?)?, 24 * si)?;

    // ATM vol
    let atm_vol = {
        let f_pow_2 = fp_mul(f_pow, f_pow)?;
        let h1 = if f_pow_2 == 0 {
            0
        } else {
            fp_div_i(h1_num, checked_divisor(24, f_pow_2)?)?
        };
        let h2 = if f_pow == 0 {
            0
        } else {
            fp_div_i(h2_num, checked_divisor(4, f_pow)?)?
        };
        // h1, h2, h3 each ∈ [-SCALE_I, SCALE_I] by design; sum ∈ [-3·SCALE_I, 3·SCALE_I]. Fits i128.
        let h = h1
            .checked_add(h2)
            .and_then(|v| v.checked_add(h3))
            .ok_or(SolMathError::Overflow)?;
        // h ∈ [-SCALE_I, SCALE_I]; fp_mul_i(h, t) ≤ SCALE_I for t ≤ SCALE_I;
        // si + correction ≤ 2·SCALE_I. Fits i128.
        let correction = si
            .checked_add(fp_mul_i(h, t as i128)?)
            .ok_or(SolMathError::Overflow)?;
        if correction <= 0 {
            return Err(SolMathError::NoConvergence);
        }
        let sigma_i = fp_mul_i(atm_base_vol, correction)?;
        if sigma_i > 0 {
            sigma_i as u128
        } else {
            return Err(SolMathError::NoConvergence);
        }
    };

    // For exp-based f_mid_pow: f_mid^(1-β) = exp((1-β)/2 · (ln F + ln K))
    let half_omb = (one_minus_beta / 2) as i128;
    let half_omb_ln_f = fp_mul_i(half_omb, ln_f)?;

    let nu_over_alpha = if nu == 0 { 0 } else { fp_div_i(nu_i, alpha_i)? };

    Ok(SabrSmile {
        f,
        t,
        ln_f,
        one_minus_beta,
        half_omb,
        half_omb_ln_f,
        omb2,
        omb4,
        alpha_i,
        rho,
        nu_over_alpha,
        h1_num,
        h2_num,
        h3,
        atm_vol,
    })
}

/// Compute SABR implied vol for a single strike using precomputed intermediates.
///
/// Final SBF audit: 29,609 CU average, 33,215 median, 37,520 max.
/// Uses exp((1-β)/2·(lnF+lnK)) for f_mid^(1-β) instead of pow_fixed_hp.
pub fn sabr_vol_at(pre: &SabrSmile, k: u128) -> Result<u128, SolMathError> {
    if k == 0 || pre.alpha_i == 0 {
        return Ok(0);
    }

    let si = SCALE_I;

    // ATM → cached
    let is_atm = pre.f == k;
    if is_atm {
        return Ok(pre.atm_vol);
    }

    // ln(K) — needed for both log(F/K) and f_mid_pow
    let ln_k = ln_fixed_i(k)?;
    // ln_f and ln_k each ∈ [-40·SCALE_I, 40·SCALE_I]; difference fits i128 easily.
    let log_fk = pre.ln_f - ln_k;

    // f_mid^(1-β) via exp — reuses ln_k, avoids pow_fixed_hp (~6K vs ~23K CU)
    let f_mid_pow = if pre.one_minus_beta == 0 {
        SCALE
    } else if pre.one_minus_beta == SCALE {
        fp_sqrt(fp_mul(pre.f, k)?)?
    } else {
        // half_omb_ln_f and fp_mul_i(half_omb, ln_k) each ≤ 40·SCALE_I; sum ≤ 80·SCALE_I. Fits i128.
        let exponent = pre
            .half_omb_ln_f
            .checked_add(fp_mul_i(pre.half_omb, ln_k)?)
            .ok_or(SolMathError::Overflow)?;
        exp_fixed_i(exponent)? as u128
    };

    // D_log: si = 1e12; each corrective term ≤ si/24 < 1; d_log ∈ (SCALE_I, 2·SCALE_I). Fits i128.
    let log_fk_sq = fp_mul_i(log_fk, log_fk)?;
    let d2 = fp_div_i(fp_mul_i(pre.omb2 as i128, log_fk_sq)?, 24 * si)?;
    let log_fk_4 = fp_mul_i(log_fk_sq, log_fk_sq)?;
    let d4 = fp_div_i(fp_mul_i(pre.omb4 as i128, log_fk_4)?, 1920 * si)?;
    let d_log = si
        .checked_add(d2)
        .and_then(|v| v.checked_add(d4))
        .ok_or(SolMathError::Overflow)?;

    // z
    let z = fp_mul_i(pre.nu_over_alpha, fp_mul_i(f_mid_pow as i128, log_fk)?)?;

    let z_over_chi = sabr_z_over_chi(z, pre.rho)?;

    // h using precomputed numerators
    let f_mid_pow_2 = fp_mul(f_mid_pow, f_mid_pow)?;
    let h1 = if f_mid_pow_2 == 0 {
        0
    } else {
        fp_div_i(pre.h1_num, checked_divisor(24, f_mid_pow_2)?)?
    };
    let h2 = if f_mid_pow == 0 {
        0
    } else {
        fp_div_i(pre.h2_num, checked_divisor(4, f_mid_pow)?)?
    };
    // h1, h2, pre.h3 each ∈ [-SCALE_I, SCALE_I] by design; sum ∈ [-3·SCALE_I, 3·SCALE_I]. Fits i128.
    let h = h1
        .checked_add(h2)
        .and_then(|v| v.checked_add(pre.h3))
        .ok_or(SolMathError::Overflow)?;
    // h ∈ [-SCALE_I, SCALE_I]; fp_mul_i(h, t) ≤ SCALE_I for t ≤ SCALE_I;
    // si + correction ≤ 2·SCALE_I. Fits i128.
    let time_correction = si
        .checked_add(fp_mul_i(h, pre.t as i128)?)
        .ok_or(SolMathError::Overflow)?;
    if time_correction <= 0 {
        return Err(SolMathError::NoConvergence);
    }

    let denom = fp_mul_i(f_mid_pow as i128, d_log)?;
    let base_vol = fp_div_i(pre.alpha_i, denom)?;
    let sigma_i = fp_mul_i(fp_mul_i(base_vol, z_over_chi)?, time_correction)?;

    if sigma_i > 0 {
        Ok(sigma_i as u128)
    } else {
        Err(SolMathError::NoConvergence)
    }
}

// ============================================================
// Shared helpers
// ============================================================

/// Assemble the general SABR formula from f_mid_pow and log_fk.
/// Shared between sabr_implied_vol (single) paths.
#[inline]
fn sabr_assemble(
    f_mid_pow: u128,
    log_fk: i128,
    one_minus_beta: u128,
    alpha: u128,
    alpha_i: i128,
    beta_i: i128,
    rho: i128,
    nu_i: i128,
    t: u128,
) -> Result<u128, SolMathError> {
    let si = SCALE_I;

    let log_fk_sq = fp_mul_i(log_fk, log_fk)?;
    let omb2 = fp_mul(one_minus_beta, one_minus_beta)?;
    let omb4 = fp_mul(omb2, omb2)?;
    let log_fk_4 = fp_mul_i(log_fk_sq, log_fk_sq)?;

    // d_log: si = 1e12; each corrective term ≤ si/24 < 1; sum ∈ (SCALE_I, 2·SCALE_I). Fits i128.
    let d2 = fp_div_i(fp_mul_i(omb2 as i128, log_fk_sq)?, 24 * si)?;
    let d4 = fp_div_i(fp_mul_i(omb4 as i128, log_fk_4)?, 1920 * si)?;
    let d_log = si
        .checked_add(d2)
        .and_then(|v| v.checked_add(d4))
        .ok_or(SolMathError::Overflow)?;

    let z = fp_mul_i(
        fp_div_i(nu_i, alpha_i)?,
        fp_mul_i(f_mid_pow as i128, log_fk)?,
    )?;

    let z_over_chi = sabr_z_over_chi(z, rho)?;

    let h = compute_h(one_minus_beta, alpha, beta_i, rho, nu_i, alpha_i, f_mid_pow)?;
    // h ∈ [-SCALE_I, SCALE_I] by design; fp_mul_i(h, t) ≤ SCALE_I for t ≤ SCALE_I;
    // si + correction ≤ 2·SCALE_I. Fits i128.
    let time_correction = si
        .checked_add(fp_mul_i(h, t as i128)?)
        .ok_or(SolMathError::Overflow)?;
    if time_correction <= 0 {
        return Err(SolMathError::NoConvergence);
    }

    let denom = fp_mul_i(f_mid_pow as i128, d_log)?;
    let base_vol = fp_div_i(alpha_i, denom)?;
    let sigma_i = fp_mul_i(fp_mul_i(base_vol, z_over_chi)?, time_correction)?;

    if sigma_i > 0 {
        Ok(sigma_i as u128)
    } else {
        Err(SolMathError::NoConvergence)
    }
}

/// h = (1−β)²α²/(24·fpow²) + ρβνα/(4·fpow) + (2−3ρ²)ν²/24
#[inline]
fn compute_h(
    one_minus_beta: u128,
    alpha: u128,
    beta_i: i128,
    rho: i128,
    nu_i: i128,
    alpha_i: i128,
    f_pow: u128,
) -> Result<i128, SolMathError> {
    let si = SCALE_I;
    let f_pow_2 = fp_mul(f_pow, f_pow)?;
    let omb2 = fp_mul(one_minus_beta, one_minus_beta)?;
    let alpha2 = fp_mul(alpha, alpha)?;
    let rho2 = fp_mul_i_fast(rho, rho);

    let h1 = if f_pow_2 == 0 {
        0
    } else {
        fp_div_i(
            fp_mul_i(omb2 as i128, alpha2 as i128)?,
            checked_divisor(24, f_pow_2)?,
        )?
    };
    let h2 = if f_pow == 0 {
        0
    } else {
        fp_div_i(
            fp_mul_i(fp_mul_i_fast(rho, beta_i), fp_mul_i(nu_i, alpha_i)?)?,
            checked_divisor(4, f_pow)?,
        )?
    };
    // 2 * si - 3 * rho2: si = SCALE_I = 1e12, rho2 ∈ [0, SCALE_I]; result ∈ [-1e12, 2e12]. Fits i128.
    let h3 = fp_div_i(fp_mul_i(2 * si - 3 * rho2, fp_mul_i(nu_i, nu_i)?)?, 24 * si)?;

    // h1, h2, h3 each ∈ [-SCALE_I, SCALE_I] by design; sum ∈ [-3·SCALE_I, 3·SCALE_I]. Fits i128.
    h1.checked_add(h2)
        .and_then(|v| v.checked_add(h3))
        .ok_or(SolMathError::Overflow)
}

#[inline]
fn checked_divisor(m: i128, x: u128) -> Result<i128, SolMathError> {
    i128::try_from(x)
        .ok()
        .and_then(|v| v.checked_mul(m))
        .ok_or(SolMathError::Overflow)
}

/// z/χ(z) — exact via sqrt + ln. ~10K CU.
fn sabr_z_over_chi(z: i128, rho: i128) -> Result<i128, SolMathError> {
    let si = SCALE_I;

    if rho <= -si || rho >= si {
        return Err(SolMathError::DomainError);
    }

    if z.unsigned_abs() < (si / 1_000_000) as u128 {
        return Ok(si);
    }

    // si = 1e12; 2 * fp_mul_i(rho, z) ≤ 2·SCALE_I (both rho, z ≤ SCALE_I); disc ∈ (-1e12, 3e12). Fits i128.
    let two_rho_z = fp_mul_i(rho, z)?
        .checked_mul(2)
        .ok_or(SolMathError::Overflow)?;
    let z_sq = fp_mul_i(z, z)?;
    let disc = si
        .checked_sub(two_rho_z)
        .and_then(|v| v.checked_add(z_sq))
        .ok_or(SolMathError::Overflow)?;

    let sqrt_disc = if disc > 0 {
        fp_sqrt(disc as u128)? as i128
    } else {
        0
    };

    // sqrt_disc ≤ SCALE_I, z ≤ SCALE_I, rho ∈ (-SCALE_I, SCALE_I); num ≤ 3·SCALE_I. Fits i128.
    let num = sqrt_disc
        .checked_add(z)
        .and_then(|v| v.checked_sub(rho))
        .ok_or(SolMathError::Overflow)?;
    // rho ∈ (-SCALE_I, SCALE_I); den = si - rho ∈ (0, 2·SCALE_I). Fits i128.
    let den = si - rho;

    let ratio = if num.unsigned_abs() < (si / 1000) as u128 || den < si / 1000 {
        // Rationalized form avoids subtracting two nearly equal quantities:
        // (sqrt+z-rho)/(1-rho) = (1+rho)/(sqrt-z+rho).
        let alt_num = si.checked_add(rho).ok_or(SolMathError::Overflow)?;
        let alt_den = sqrt_disc
            .checked_sub(z)
            .and_then(|v| v.checked_add(rho))
            .ok_or(SolMathError::Overflow)?;
        if alt_den <= 0 {
            return Err(SolMathError::NoConvergence);
        }
        fp_div_i(alt_num, alt_den)?
    } else {
        fp_div_i(num, den)?
    };
    if ratio <= 0 {
        return Err(SolMathError::NoConvergence);
    }

    let chi = ln_fixed_i(ratio as u128)?;
    if chi.abs() < si / 1_000_000_000 {
        return Ok(si);
    }

    fp_div_i(z, chi)
}

// ============================================================
// Experimental: Padé [2/2] rational approximation for z/χ(z)
// ============================================================

/// Degree-3 Taylor approximation of z/chi(z) for SABR implied vol.
///
/// Replaces the exact sqrt + ln path (~10K CU) with a polynomial (~1K CU).
/// Falls back to exact for |z| > 0.5*SCALE where the approximation degrades.
///
/// # Parameters
/// - `z` -- SABR z-variable at SCALE (`i128`)
/// - `rho` -- Spot-vol correlation at SCALE (`i128`)
///
/// # Returns
/// z/chi(z) at SCALE.
///
/// # Accuracy
/// - |z| < 0.3: < 10 ppm (0.001%)
/// - |z| < 0.5: < 200 ppm (0.02%)
/// - |z| > 0.5: falls back to exact (sqrt + ln)
///
/// # CU Cost
/// Final mixed-path SBF audit: 10,911 CU average, 11,753 median, 12,855 max.
pub fn sabr_z_over_chi_pade(z: i128, rho: i128) -> Result<i128, SolMathError> {
    let si = SCALE_I;

    if rho <= -si || rho >= si {
        return Err(SolMathError::DomainError);
    }

    if z.unsigned_abs() < (si / 1_000_000) as u128 {
        return Ok(si);
    }

    // The polynomial's error grows sharply as |rho| approaches one; use the
    // stable exact/rationalized path throughout that region.
    if rho.unsigned_abs() > 900_000_000_000 {
        return sabr_z_over_chi(z, rho);
    }

    // Fall back to exact for |z| > 0.5
    if z.unsigned_abs() > (si / 2) as u128 {
        return sabr_z_over_chi(z, rho);
    }

    let rho2 = fp_mul_i_fast(rho, rho);

    // rho ∈ (-SCALE_I, SCALE_I); c1 = -rho/2 ∈ (-SCALE_I/2, SCALE_I/2). Fits i128.
    let c1 = -rho / 2;
    // 2 * si - 3 * rho2 ∈ [-1e12, 2e12]; divided by 12. Fits i128.
    let c2 = (2 * si - 3 * rho2) / 12;
    // 4*si ≤ 4e12; 4*rho ≤ 4e12; 12*rho2 ≤ 12e12; 9*fp_mul_i_fast(rho,rho2) ≤ 9e12; sum ≤ 29e12 ≪ i128::MAX; divided by 24.
    let c3 = (4 * si - 4 * rho - 12 * rho2 + 9 * fp_mul_i_fast(rho, rho2)) / 24;

    let z2 = fp_mul_i_fast(z, z);
    let z3 = fp_mul_i_fast(z2, z);

    // Each fp_mul_i_fast term ≤ SCALE_I (z ≤ 0.5·SCALE_I, c* ≤ SCALE_I); sum ≤ 4·SCALE_I. Fits i128.
    Ok(si + fp_mul_i_fast(c1, z) + fp_mul_i_fast(c2, z2) + fp_mul_i_fast(c3, z3))
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    const SURFACE_STRIKES: [u128; 3] = [80 * SCALE, 100 * SCALE, 120 * SCALE];
    const SURFACE_MATURITIES: [u128; 2] = [SCALE, 2 * SCALE];
    const SURFACE_CALLS: [u128; 6] = [
        25 * SCALE,
        12 * SCALE,
        5 * SCALE,
        27 * SCALE,
        15 * SCALE,
        8 * SCALE,
    ];
    const SURFACE_PUTS: [u128; 6] = [
        5 * SCALE,
        12 * SCALE,
        25 * SCALE,
        7 * SCALE,
        15 * SCALE,
        28 * SCALE,
    ];

    fn fill_zero_rate_intrinsic_surface(
        spot: u128,
        strikes: &[u128],
        maturities: &[u128],
        calls: &mut [u128],
        puts: &mut [u128],
    ) {
        assert_eq!(calls.len(), strikes.len() * maturities.len());
        assert_eq!(puts.len(), calls.len());
        let mut maturity_index = 0usize;
        while maturity_index < maturities.len() {
            let mut strike_index = 0usize;
            while strike_index < strikes.len() {
                let index = maturity_index * strikes.len() + strike_index;
                calls[index] = spot.saturating_sub(strikes[strike_index]);
                puts[index] = strikes[strike_index].saturating_sub(spot);
                strike_index += 1;
            }
            maturity_index += 1;
        }
    }

    #[test]
    fn certified_surface_returns_stored_quotes_without_repricing() {
        let certificate = certify_sabr_surface(
            100 * SCALE,
            0,
            &SURFACE_STRIKES,
            &SURFACE_MATURITIES,
            &SURFACE_CALLS,
            &SURFACE_PUTS,
        )
        .unwrap();

        assert_eq!(certificate.spot(), 100 * SCALE);
        assert_eq!(certificate.rate(), 0);
        assert_eq!(certificate.strikes(), &SURFACE_STRIKES);
        assert_eq!(certificate.maturities(), &SURFACE_MATURITIES);
        assert_eq!(certificate.quote_count(), 6);
        let quote = certificate.quote_at(1, 2).unwrap();
        assert_eq!(quote.spot(), 100 * SCALE);
        assert_eq!(quote.rate(), 0);
        assert_eq!(quote.strike(), 120 * SCALE);
        assert_eq!(quote.maturity(), 2 * SCALE);
        assert_eq!(quote.call(), 8 * SCALE);
        assert_eq!(quote.put(), 28 * SCALE);
        assert_eq!(certificate.quote_at(2, 0), Err(SolMathError::DomainError));
        assert_eq!(certificate.quote_at(0, 3), Err(SolMathError::DomainError));
    }

    #[test]
    fn certified_surface_rejects_malformed_axes_and_shapes() {
        assert_eq!(
            certify_sabr_surface(100 * SCALE, 0, &[], &SURFACE_MATURITIES, &[], &[]),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            certify_sabr_surface(
                100 * SCALE,
                0,
                &SURFACE_STRIKES[..2],
                &SURFACE_MATURITIES,
                &SURFACE_CALLS[..4],
                &SURFACE_PUTS[..4],
            ),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            certify_sabr_surface(
                100 * SCALE,
                0,
                &SURFACE_STRIKES,
                &SURFACE_MATURITIES,
                &SURFACE_CALLS[..5],
                &SURFACE_PUTS,
            ),
            Err(SolMathError::DomainError)
        );

        let duplicate_strikes = [80 * SCALE, 80 * SCALE, 120 * SCALE];
        assert_eq!(
            certify_sabr_surface(
                100 * SCALE,
                0,
                &duplicate_strikes,
                &SURFACE_MATURITIES,
                &SURFACE_CALLS,
                &SURFACE_PUTS,
            ),
            Err(SolMathError::DomainError)
        );
        let descending_maturities = [2 * SCALE, SCALE];
        assert_eq!(
            certify_sabr_surface(
                100 * SCALE,
                0,
                &SURFACE_STRIKES,
                &descending_maturities,
                &SURFACE_CALLS,
                &SURFACE_PUTS,
            ),
            Err(SolMathError::DomainError)
        );
        let zero_strike = [0, 100 * SCALE, 120 * SCALE];
        assert_eq!(
            certify_sabr_surface(
                100 * SCALE,
                0,
                &zero_strike,
                &SURFACE_MATURITIES,
                &SURFACE_CALLS,
                &SURFACE_PUTS,
            ),
            Err(SolMathError::DomainError)
        );
    }

    #[test]
    fn certified_surface_requires_exact_parity_and_bounds() {
        let mut bad_puts = SURFACE_PUTS;
        bad_puts[4] += 1;
        assert_eq!(
            certify_sabr_surface(
                100 * SCALE,
                0,
                &SURFACE_STRIKES,
                &SURFACE_MATURITIES,
                &SURFACE_CALLS,
                &bad_puts,
            ),
            Err(SolMathError::NoConvergence)
        );

        let mut calls = SURFACE_CALLS;
        let mut puts = SURFACE_PUTS;
        calls[0] = 101 * SCALE;
        puts[0] = 81 * SCALE; // preserves parity at K=80 but violates both bounds
        assert_eq!(
            certify_sabr_surface(
                100 * SCALE,
                0,
                &SURFACE_STRIKES,
                &SURFACE_MATURITIES,
                &calls,
                &puts,
            ),
            Err(SolMathError::NoConvergence)
        );
    }

    #[test]
    fn certified_surface_uses_discounted_strike_at_positive_rates() {
        let spot = 100 * SCALE;
        let rate = 50_000_000_000; // 5%

        // Keep every discounted strike below spot so the zero-volatility call
        // curve is the same linear function C(x) = S - x at both maturities.
        // A grid that straddles the intrinsic kink at x = S has a different
        // piecewise-linear interpolant after each maturity's discount shifts
        // the nominal strike axis, so it is not a valid calendar fixture.
        let strikes = [60 * SCALE, 80 * SCALE, 100 * SCALE];
        let mut calls = [0u128; 6];
        let mut puts = [0u128; 6];

        let mut maturity_index = 0usize;
        while maturity_index < SURFACE_MATURITIES.len() {
            let discount =
                sabr_surface_discount(rate, SURFACE_MATURITIES[maturity_index]).unwrap() as u128;
            let mut strike_index = 0usize;
            while strike_index < strikes.len() {
                let index = maturity_index * strikes.len() + strike_index;
                let discounted_strike = fp_mul(strikes[strike_index], discount).unwrap();
                calls[index] = spot.saturating_sub(discounted_strike);
                puts[index] = discounted_strike.saturating_sub(spot);
                strike_index += 1;
            }
            maturity_index += 1;
        }

        let first_discount = sabr_surface_discount(rate, SURFACE_MATURITIES[0]).unwrap();
        let second_discount = sabr_surface_discount(rate, SURFACE_MATURITIES[1]).unwrap();
        let mut first_axis = [0u128; MAX_SABR_SURFACE_STRIKES];
        let mut second_axis = [0u128; MAX_SABR_SURFACE_STRIKES];
        fill_discounted_strikes(&strikes, first_discount, &mut first_axis).unwrap();
        fill_discounted_strikes(&strikes, second_discount, &mut second_axis).unwrap();
        assert_eq!(
            validate_sabr_surface_row(
                spot,
                &first_axis[..strikes.len()],
                &calls[..strikes.len()],
                &puts[..strikes.len()],
            ),
            Ok(())
        );
        assert_eq!(
            validate_sabr_surface_row(
                spot,
                &second_axis[..strikes.len()],
                &calls[strikes.len()..],
                &puts[strikes.len()..],
            ),
            Ok(())
        );
        assert_eq!(
            validate_sabr_surface_calendar(
                spot,
                &strikes,
                first_discount,
                &calls[..strikes.len()],
                second_discount,
                &calls[strikes.len()..],
            ),
            Ok(())
        );

        let certificate =
            certify_sabr_surface(spot, rate, &strikes, &SURFACE_MATURITIES, &calls, &puts).unwrap();
        assert_eq!(certificate.quote_at(0, 0).unwrap().put(), 0);
        assert!(certificate.quote_at(1, 0).unwrap().call() > calls[0]);
    }

    #[test]
    fn distant_wing_individually_valid_quotes_fail_global_butterfly() {
        let strikes = [50 * SCALE, 100 * SCALE, 200 * SCALE];
        let maturities = [SCALE, 2 * SCALE];
        let calls = [55 * SCALE, 54 * SCALE, SCALE, 55 * SCALE, 54 * SCALE, SCALE];
        let puts = [
            5 * SCALE,
            54 * SCALE,
            101 * SCALE,
            5 * SCALE,
            54 * SCALE,
            101 * SCALE,
        ];

        // Every isolated quote satisfies hard bounds and exact parity. Only
        // the distant, irregularly-spaced butterfly reveals the arbitrage.
        let mut row = 0usize;
        while row < maturities.len() {
            let mut column = 0usize;
            while column < strikes.len() {
                let index = row * strikes.len() + column;
                assert_eq!(
                    validate_sabr_surface_quote(
                        100 * SCALE,
                        strikes[column],
                        calls[index],
                        puts[index],
                    ),
                    Ok(())
                );
                column += 1;
            }
            row += 1;
        }
        assert_eq!(
            certify_sabr_surface(100 * SCALE, 0, &strikes, &maturities, &calls, &puts,),
            Err(SolMathError::NoConvergence)
        );
    }

    #[test]
    fn distant_wing_local_sabr_guards_pass_but_global_surface_fails() {
        let strikes = [50 * SCALE, 100 * SCALE, 200 * SCALE];
        let maturities = [SCALE, 2 * SCALE];
        // Each strike uses a locally plausible calibration. The extreme wing
        // has much higher alpha, which a single-strike/local-neighbour guard
        // cannot compare with the rest of the assembled quote surface.
        let alphas = [100_000_000_000, 100_000_000_000, 2 * SCALE];
        let mut calls = [0u128; 6];
        let mut puts = [0u128; 6];

        let mut maturity_index = 0usize;
        while maturity_index < maturities.len() {
            let mut strike_index = 0usize;
            while strike_index < strikes.len() {
                let index = maturity_index * strikes.len() + strike_index;
                let (call, put) = sabr_price(
                    100 * SCALE,
                    strikes[strike_index],
                    0,
                    maturities[maturity_index],
                    alphas[strike_index],
                    SCALE,
                    0,
                    0,
                )
                .expect("each isolated quote must pass the local SABR guard");
                calls[index] = call;
                puts[index] = put;
                strike_index += 1;
            }
            maturity_index += 1;
        }

        assert!(
            calls[2] > calls[1],
            "the distant call wing rises with strike"
        );
        assert_eq!(
            certify_sabr_surface(100 * SCALE, 0, &strikes, &maturities, &calls, &puts,),
            Err(SolMathError::NoConvergence)
        );
    }

    #[test]
    fn certified_surface_rejects_vertical_spread_and_calendar_arbitrage() {
        let strikes = [100 * SCALE, 101 * SCALE, 102 * SCALE];
        let maturities = [SCALE, 2 * SCALE];
        let calls = [100 * SCALE, 0, 0, 100 * SCALE, 0, 0];
        let puts = [100 * SCALE, SCALE, 2 * SCALE, 100 * SCALE, SCALE, 2 * SCALE];
        assert_eq!(
            certify_sabr_surface(100 * SCALE, 0, &strikes, &maturities, &calls, &puts,),
            Err(SolMathError::NoConvergence)
        );

        let mut calendar_calls = SURFACE_CALLS;
        let mut calendar_puts = SURFACE_PUTS;
        calendar_calls[3] = 24 * SCALE;
        calendar_puts[3] = 4 * SCALE;
        assert_eq!(
            certify_sabr_surface(
                100 * SCALE,
                0,
                &SURFACE_STRIKES,
                &SURFACE_MATURITIES,
                &calendar_calls,
                &calendar_puts,
            ),
            Err(SolMathError::NoConvergence)
        );
    }

    #[test]
    fn certified_surface_unequal_spacing_uses_correct_slope_direction() {
        let strikes = [80 * SCALE, 100 * SCALE, 140 * SCALE];
        let maturities = [SCALE, 2 * SCALE];
        let calls = [
            30 * SCALE,
            20 * SCALE,
            10 * SCALE,
            30 * SCALE,
            20 * SCALE,
            10 * SCALE,
        ];
        let puts = [
            10 * SCALE,
            20 * SCALE,
            50 * SCALE,
            10 * SCALE,
            20 * SCALE,
            50 * SCALE,
        ];
        assert!(
            certify_sabr_surface(100 * SCALE, 0, &strikes, &maturities, &calls, &puts,).is_ok()
        );

        // The middle quote reverses the required slope ordering: the call
        // falls faster, and the put rises slower, in the wider right interval.
        let concave_calls = [
            30 * SCALE,
            25 * SCALE,
            10 * SCALE,
            30 * SCALE,
            25 * SCALE,
            10 * SCALE,
        ];
        let concave_puts = [
            10 * SCALE,
            25 * SCALE,
            50 * SCALE,
            10 * SCALE,
            25 * SCALE,
            50 * SCALE,
        ];
        assert_eq!(
            certify_sabr_surface(
                100 * SCALE,
                0,
                &strikes,
                &maturities,
                &concave_calls,
                &concave_puts,
            ),
            Err(SolMathError::NoConvergence)
        );
    }

    #[test]
    fn certified_surface_wide_products_are_exact_near_i128_limit() {
        let maximum = i128::MAX as u128;
        let strikes = [1, maximum / 2, maximum];
        let maturities = [SCALE, 2 * SCALE];
        let row = [maximum - 1, maximum - maximum / 2, 0];
        let calls = [row[0], row[1], row[2], row[0], row[1], row[2]];
        let puts = [0u128; 6];

        let left_drop = calls[0] - calls[1];
        let right_width = strikes[2] - strikes[1];
        assert!(left_drop.checked_mul(right_width).is_none());
        assert!(certify_sabr_surface(maximum, 0, &strikes, &maturities, &calls, &puts,).is_ok());

        assert_eq!(
            certify_sabr_surface(
                maximum + 1,
                0,
                &SURFACE_STRIKES,
                &SURFACE_MATURITIES,
                &SURFACE_CALLS,
                &SURFACE_PUTS,
            ),
            Err(SolMathError::Overflow)
        );
        let excessive_strike = [1, maximum, maximum + 1];
        assert_eq!(
            certify_sabr_surface(
                100 * SCALE,
                0,
                &excessive_strike,
                &SURFACE_MATURITIES,
                &SURFACE_CALLS,
                &SURFACE_PUTS,
            ),
            Err(SolMathError::Overflow)
        );
        let excessive_maturity = [SCALE, maximum + 1];
        assert_eq!(
            certify_sabr_surface(
                100 * SCALE,
                0,
                &SURFACE_STRIKES,
                &excessive_maturity,
                &SURFACE_CALLS,
                &SURFACE_PUTS,
            ),
            Err(SolMathError::Overflow)
        );
        assert_eq!(
            certify_sabr_surface(
                100 * SCALE,
                maximum + 1,
                &SURFACE_STRIKES,
                &SURFACE_MATURITIES,
                &SURFACE_CALLS,
                &SURFACE_PUTS,
            ),
            Err(SolMathError::Overflow)
        );
    }

    #[test]
    fn certified_surface_enforces_bounded_grid_limits() {
        let spot = 100 * SCALE;
        let strikes_at_limit: [u128; MAX_SABR_SURFACE_STRIKES] =
            core::array::from_fn(|i| (i as u128 + 1) * 5 * SCALE);
        let maturities_for_strike_limit: [u128; 8] =
            core::array::from_fn(|i| (i as u128 + 1) * SCALE);
        let mut calls_at_limit = [0u128; MAX_SABR_SURFACE_QUOTES];
        let mut puts_at_limit = [0u128; MAX_SABR_SURFACE_QUOTES];
        fill_zero_rate_intrinsic_surface(
            spot,
            &strikes_at_limit,
            &maturities_for_strike_limit,
            &mut calls_at_limit,
            &mut puts_at_limit,
        );
        assert_eq!(
            certify_sabr_surface(
                spot,
                0,
                &strikes_at_limit,
                &maturities_for_strike_limit,
                &calls_at_limit,
                &puts_at_limit,
            )
            .unwrap()
            .quote_count(),
            MAX_SABR_SURFACE_QUOTES
        );

        let strikes_for_maturity_limit: [u128; 16] =
            core::array::from_fn(|i| (i as u128 + 1) * 10 * SCALE);
        let maturities_at_limit: [u128; MAX_SABR_SURFACE_MATURITIES] =
            core::array::from_fn(|i| (i as u128 + 1) * SCALE);
        let mut maturity_limit_calls = [0u128; MAX_SABR_SURFACE_QUOTES];
        let mut maturity_limit_puts = [0u128; MAX_SABR_SURFACE_QUOTES];
        fill_zero_rate_intrinsic_surface(
            spot,
            &strikes_for_maturity_limit,
            &maturities_at_limit,
            &mut maturity_limit_calls,
            &mut maturity_limit_puts,
        );
        assert!(certify_sabr_surface(
            spot,
            0,
            &strikes_for_maturity_limit,
            &maturities_at_limit,
            &maturity_limit_calls,
            &maturity_limit_puts,
        )
        .is_ok());

        let too_many_strikes: [u128; MAX_SABR_SURFACE_STRIKES + 1] =
            core::array::from_fn(|i| (i as u128 + 1) * SCALE);
        let two_maturities = [SCALE, 2 * SCALE];
        let oversized_strike_calls = [0u128; (MAX_SABR_SURFACE_STRIKES + 1) * 2];
        assert_eq!(
            certify_sabr_surface(
                spot,
                0,
                &too_many_strikes,
                &two_maturities,
                &oversized_strike_calls,
                &oversized_strike_calls,
            ),
            Err(SolMathError::DomainError)
        );

        let three_strikes = [80 * SCALE, 100 * SCALE, 120 * SCALE];
        let too_many_maturities: [u128; MAX_SABR_SURFACE_MATURITIES + 1] =
            core::array::from_fn(|i| (i as u128 + 1) * SCALE);
        let oversized_maturity_calls = [0u128; 3 * (MAX_SABR_SURFACE_MATURITIES + 1)];
        assert_eq!(
            certify_sabr_surface(
                spot,
                0,
                &three_strikes,
                &too_many_maturities,
                &oversized_maturity_calls,
                &oversized_maturity_calls,
            ),
            Err(SolMathError::DomainError)
        );

        let quote_cap_strikes: [u128; 17] = core::array::from_fn(|i| (i as u128 + 1) * SCALE);
        let quote_cap_maturities: [u128; 16] = core::array::from_fn(|i| (i as u128 + 1) * SCALE);
        let too_many_quotes = [0u128; 17 * 16];
        assert_eq!(
            certify_sabr_surface(
                spot,
                0,
                &quote_cap_strikes,
                &quote_cap_maturities,
                &too_many_quotes,
                &too_many_quotes,
            ),
            Err(SolMathError::DomainError)
        );
    }

    #[test]
    fn test_sabr_pade_rejects_invalid_rho_before_fast_path() {
        assert_eq!(
            sabr_z_over_chi_pade(0, SCALE_I),
            Err(SolMathError::DomainError)
        );
        assert_eq!(
            sabr_z_over_chi_pade(0, i128::MIN),
            Err(SolMathError::DomainError)
        );
    }

    #[test]
    fn test_sabr_exact_path_is_continuous_near_rho_one() {
        let a = sabr_z_over_chi_pade(500_000_000_000, 999_900_000_000).unwrap();
        let b = sabr_z_over_chi_pade(500_000_000_001, 999_900_000_000).unwrap();
        assert!(a.abs_diff(b) < 10_000_000, "a={a}, b={b}");
        assert!((700_000_000_000..750_000_000_000).contains(&a));
    }

    #[test]
    fn test_sabr_rejects_non_positive_asymptotic_correction() {
        assert_eq!(
            sabr_implied_vol(
                100 * SCALE,
                100 * SCALE,
                20 * SCALE,
                200_000_000_000,
                SCALE,
                990_000_000_000,
                2 * SCALE,
            ),
            Err(SolMathError::NoConvergence)
        );
    }

    #[test]
    fn test_sabr_price_rejects_locally_increasing_call_wing() {
        assert_eq!(
            sabr_price(
                100 * SCALE,
                165 * SCALE,
                0,
                2 * SCALE,
                SCALE / 2,
                SCALE,
                900_000_000_000,
                SCALE,
            ),
            Err(SolMathError::NoConvergence)
        );
    }

    #[test]
    fn test_sabr_rejects_uncertified_global_wing_regime() {
        assert_eq!(
            sabr_price(
                100 * SCALE,
                100 * SCALE,
                0,
                2 * SCALE,
                SCALE,
                SCALE,
                0,
                5 * SCALE,
            ),
            Err(SolMathError::NoConvergence)
        );
    }

    #[test]
    fn test_sabr_deterministic_greeks_are_not_zeroed() {
        let g = sabr_greeks(
            110 * SCALE,
            100 * SCALE,
            50_000_000_000,
            SCALE,
            0,
            SCALE / 2,
            0,
            0,
        )
        .unwrap();
        assert_eq!(g.call_delta, SCALE_I);
        assert!(g.call_rho > 0);
        assert!(g.call_theta < 0);
    }

    #[test]
    fn test_sabr_pade_minimum_z_returns_error_not_panic() {
        assert_eq!(
            sabr_z_over_chi_pade(i128::MIN, 0),
            Err(SolMathError::Overflow)
        );
    }

    // --- Single-strike tests ---

    #[test]
    fn test_sabr_atm_basic() {
        let f = 100 * SCALE;
        let alpha = 200_000_000_000;
        let beta = SCALE / 2;
        let rho = -300_000_000_000i128;
        let nu = 400_000_000_000;
        let t = SCALE;

        let vol = sabr_implied_vol(f, f, t, alpha, beta, rho, nu).unwrap();
        assert!(
            vol > 10_000_000_000 && vol < 50_000_000_000,
            "ATM vol {} out of range",
            vol
        );
    }

    #[test]
    fn test_sabr_beta1_rho0_symmetric() {
        let f = 100 * SCALE;
        let alpha = 200_000_000_000;
        let beta = SCALE;
        let rho = 0i128;
        let nu = 300_000_000_000;
        let t = SCALE;

        let k_low = 90 * SCALE;
        let k_high = (f as i128 * f as i128 / k_low as i128) as u128;
        let vol_low = sabr_implied_vol(f, k_low, t, alpha, beta, rho, nu).unwrap();
        let vol_high = sabr_implied_vol(f, k_high, t, alpha, beta, rho, nu).unwrap();
        let diff = (vol_low as i128 - vol_high as i128).abs();
        assert!(
            diff < SCALE_I / 1000,
            "Symmetry broken: low={} high={} diff={}",
            vol_low,
            vol_high,
            diff
        );
    }

    #[test]
    fn test_sabr_nu_zero_is_cev() {
        let f = 100 * SCALE;
        let k = 105 * SCALE;
        let alpha = 200_000_000_000;
        let beta = SCALE / 2;
        let t = SCALE;

        let vol = sabr_implied_vol(f, k, t, alpha, beta, 0, 0).unwrap();
        let expected = 20_000_000_000u128;
        let diff = (vol as i128 - expected as i128).abs();
        assert!(
            diff < SCALE_I / 100,
            "CEV vol={} expected≈{}",
            vol,
            expected
        );
    }

    #[test]
    fn test_sabr_put_call_parity() {
        let s = 100 * SCALE;
        let k = 105 * SCALE;
        let r = 50_000_000_000u128;
        let t = SCALE;
        let alpha = 300_000_000_000;
        let beta = 700_000_000_000;
        let rho = -500_000_000_000i128;
        let nu = 400_000_000_000;

        let (call, put) = sabr_price(s, k, r, t, alpha, beta, rho, nu).unwrap();
        let disc = exp_fixed_i(-fp_mul_i(r as i128, t as i128).unwrap()).unwrap();
        let parity =
            (call as i128 - put as i128 - s as i128 + fp_mul_i(k as i128, disc).unwrap()).abs();
        assert!(parity < 1000, "Put-call parity error: {}", parity);
    }

    #[test]
    fn test_sabr_negative_rho_skew() {
        let f = 100 * SCALE;
        let alpha = 200_000_000_000;
        let beta = SCALE / 2;
        let rho = -500_000_000_000i128;
        let nu = 400_000_000_000;
        let t = SCALE;

        let vol_90 = sabr_implied_vol(f, 90 * SCALE, t, alpha, beta, rho, nu).unwrap();
        let vol_100 = sabr_implied_vol(f, f, t, alpha, beta, rho, nu).unwrap();
        let vol_110 = sabr_implied_vol(f, 110 * SCALE, t, alpha, beta, rho, nu).unwrap();

        assert!(
            vol_90 > vol_100,
            "Expected vol_90 > vol_100: {} vs {}",
            vol_90,
            vol_100
        );
        assert!(
            vol_90 > vol_110,
            "Expected vol_90 > vol_110 (skew): {} vs {}",
            vol_90,
            vol_110
        );
    }

    #[test]
    fn test_sabr_smile_curvature() {
        let f = 100 * SCALE;
        let alpha = 200_000_000_000;
        let beta = SCALE;
        let rho = 0i128;
        let nu = 500_000_000_000;
        let t = SCALE;

        let vol_90 = sabr_implied_vol(f, 90 * SCALE, t, alpha, beta, rho, nu).unwrap();
        let vol_100 = sabr_implied_vol(f, f, t, alpha, beta, rho, nu).unwrap();
        let vol_110 = sabr_implied_vol(f, 110 * SCALE, t, alpha, beta, rho, nu).unwrap();

        assert!(
            vol_90 > vol_100,
            "Left wing below ATM: {} vs {}",
            vol_90,
            vol_100
        );
        assert!(
            vol_110 > vol_100,
            "Right wing below ATM: {} vs {}",
            vol_110,
            vol_100
        );
    }

    #[test]
    fn test_sabr_hagan_rates_params() {
        let f = 48_800_000_000u128;
        let alpha = 87_300_000_000u128;
        let beta = 700_000_000_000u128;
        let rho = -480_000_000_000i128;
        let nu = 470_000_000_000u128;
        let t = 10 * SCALE;

        for &k_bp in &[100, 200, 300, 400, 500, 600, 700, 800] {
            let k = k_bp as u128 * SCALE / 10000;
            let vol = sabr_implied_vol(f, k, t, alpha, beta, rho, nu);
            assert!(vol.is_ok(), "Failed at K={}bp: {:?}", k_bp, vol);
            let v = vol.unwrap();
            assert!(
                v > SCALE / 100 && v < 2 * SCALE,
                "Vol {} out of range at K={}bp",
                v,
                k_bp
            );
        }
    }

    #[test]
    fn test_sabr_zero_inputs() {
        assert_eq!(
            sabr_implied_vol(0, SCALE, SCALE, SCALE, SCALE / 2, 0, SCALE).unwrap(),
            0
        );
        assert_eq!(
            sabr_implied_vol(SCALE, 0, SCALE, SCALE, SCALE / 2, 0, SCALE).unwrap(),
            0
        );
        assert_eq!(
            sabr_implied_vol(SCALE, SCALE, 0, SCALE, SCALE / 2, 0, SCALE).unwrap(),
            0
        );
        assert_eq!(
            sabr_implied_vol(SCALE, SCALE, SCALE, 0, SCALE / 2, 0, SCALE).unwrap(),
            0
        );
    }

    #[test]
    fn test_sabr_beta_zero() {
        let f = 100 * SCALE;
        let alpha = 2 * SCALE;
        let rho = -300_000_000_000i128;
        let nu = 400_000_000_000;
        let t = SCALE;
        let vol = sabr_implied_vol(f, 95 * SCALE, t, alpha, 0, rho, nu).unwrap();
        assert!(vol > 0, "β=0 vol should be positive: {}", vol);
    }

    #[test]
    fn test_sabr_beta_one() {
        let f = 100 * SCALE;
        let alpha = 200_000_000_000;
        let rho = -300_000_000_000i128;
        let nu = 400_000_000_000;
        let t = SCALE;
        let vol_atm = sabr_implied_vol(f, f, t, alpha, SCALE, rho, nu).unwrap();
        let vol_otm = sabr_implied_vol(f, 110 * SCALE, t, alpha, SCALE, rho, nu).unwrap();
        assert!(
            vol_atm > 150_000_000_000 && vol_atm < 300_000_000_000,
            "β=1 ATM vol {}",
            vol_atm
        );
        assert!(vol_otm > 0);
    }

    #[test]
    fn test_sabr_beta_one_limit() {
        let f = 100 * SCALE;
        let alpha = 200_000_000_000;
        let rho = -300_000_000_000i128;
        let nu = 400_000_000_000;
        let t = SCALE;
        let k = 105 * SCALE;
        let vol_exact = sabr_implied_vol(f, k, t, alpha, SCALE, rho, nu).unwrap();
        let vol_near = sabr_implied_vol(f, k, t, alpha, SCALE - 1, rho, nu).unwrap();
        let diff = (vol_exact as i128 - vol_near as i128).abs();
        assert!(
            diff < 100,
            "β=1 limit: exact={} near={} diff={}",
            vol_exact,
            vol_near,
            diff
        );
    }

    #[test]
    fn test_sabr_greeks_basic() {
        let s = 100 * SCALE;
        let k = 100 * SCALE;
        let r = 50_000_000_000;
        let t = SCALE;
        let alpha = 200_000_000_000;
        let beta = SCALE / 2;
        let rho = -300_000_000_000i128;
        let nu = 400_000_000_000;
        let greeks = sabr_greeks(s, k, r, t, alpha, beta, rho, nu).unwrap();
        assert!(greeks.call > 0 && greeks.put > 0 && greeks.vega > 0 && greeks.gamma > 0);
    }

    // --- Batch smile tests ---

    #[test]
    fn test_sabr_smile_matches_single() {
        let f = 100 * SCALE;
        let alpha = 200_000_000_000;
        let beta = SCALE / 2;
        let rho = -300_000_000_000i128;
        let nu = 400_000_000_000;
        let t = SCALE;

        let pre = sabr_precompute(f, t, alpha, beta, rho, nu).unwrap();

        // ATM should match
        let vol_single = sabr_implied_vol(f, f, t, alpha, beta, rho, nu).unwrap();
        let vol_batch = sabr_vol_at(&pre, f).unwrap();
        let atm_diff = (vol_single as i128 - vol_batch as i128).abs();
        assert!(
            atm_diff < 10,
            "ATM mismatch: single={} batch={}",
            vol_single,
            vol_batch
        );

        // OTM strikes — batch uses exp instead of pow, so allow ~5 ULP tolerance
        for &k in &[
            85 * SCALE,
            90 * SCALE,
            95 * SCALE,
            105 * SCALE,
            110 * SCALE,
            115 * SCALE,
        ] {
            let vol_s = sabr_implied_vol(f, k, t, alpha, beta, rho, nu).unwrap();
            let vol_b = sabr_vol_at(&pre, k).unwrap();
            let diff = (vol_s as i128 - vol_b as i128).abs();
            // exp-based f_mid_pow may differ from pow_fixed_hp by a few ULP
            let tol = vol_s / 10_000; // 0.01% relative tolerance
            assert!(
                diff < tol as i128 || diff < 10,
                "K={}: single={} batch={} diff={} tol={}",
                k / SCALE,
                vol_s,
                vol_b,
                diff,
                tol
            );
        }
    }

    #[test]
    fn test_sabr_smile_beta1() {
        let f = 100 * SCALE;
        let alpha = 200_000_000_000;
        let rho = -300_000_000_000i128;
        let nu = 400_000_000_000;
        let t = SCALE;

        let pre = sabr_precompute(f, t, alpha, SCALE, rho, nu).unwrap();

        for &k in &[
            90 * SCALE,
            95 * SCALE,
            100 * SCALE,
            105 * SCALE,
            110 * SCALE,
        ] {
            let vol_s = sabr_implied_vol(f, k, t, alpha, SCALE, rho, nu).unwrap();
            let vol_b = sabr_vol_at(&pre, k).unwrap();
            let diff = (vol_s as i128 - vol_b as i128).abs();
            // β=1: no pow/exp, should match closely
            assert!(
                diff < 100,
                "β=1 K={}: single={} batch={} diff={}",
                k / SCALE,
                vol_s,
                vol_b,
                diff
            );
        }
    }

    #[test]
    fn test_sabr_smile_beta0() {
        let f = 100 * SCALE;
        let alpha = 2 * SCALE;
        let rho = -300_000_000_000i128;
        let nu = 400_000_000_000;
        let t = SCALE;

        let pre = sabr_precompute(f, t, alpha, 0, rho, nu).unwrap();

        for &k in &[90 * SCALE, 95 * SCALE, 105 * SCALE, 110 * SCALE] {
            let vol_s = sabr_implied_vol(f, k, t, alpha, 0, rho, nu).unwrap();
            let vol_b = sabr_vol_at(&pre, k).unwrap();
            let diff = (vol_s as i128 - vol_b as i128).abs();
            // β=0: both use sqrt, should match exactly
            assert!(
                diff < 100,
                "β=0 K={}: single={} batch={} diff={}",
                k / SCALE,
                vol_s,
                vol_b,
                diff
            );
        }
    }

    #[test]
    fn test_sabr_smile_zero_inputs() {
        let pre = sabr_precompute(0, SCALE, SCALE, SCALE / 2, 0, SCALE).unwrap();
        assert_eq!(sabr_vol_at(&pre, SCALE).unwrap(), 0);
    }

    // --- Padé approximation tests ---

    #[test]
    fn test_pade_vs_exact_small_z() {
        // Taylor-3 tested across ρ × z grid
        let rhos = [
            -800_000_000_000i128,
            -500_000_000_000,
            -200_000_000_000,
            0,
            200_000_000_000,
            500_000_000_000,
            800_000_000_000,
        ];
        let zs = [
            -400_000_000_000i128,
            -200_000_000_000,
            -100_000_000_000,
            100_000_000_000,
            200_000_000_000,
            400_000_000_000,
        ];

        let mut max_rel_err: i128 = 0;
        for &rho in &rhos {
            for &z in &zs {
                let exact = sabr_z_over_chi(z, rho).unwrap();
                let approx = sabr_z_over_chi_pade(z, rho).unwrap();
                let err = (exact - approx).abs();
                if exact.abs() > SCALE_I / 100 {
                    let rel = err * 1_000_000 / exact.abs(); // ppm
                    if rel > max_rel_err {
                        max_rel_err = rel;
                    }
                }
            }
        }
        // Taylor-3 within 2% for |z| < 0.5, degrades for |ρ| > 0.7
        assert!(
            max_rel_err < 20_000,
            "Max relative error for |z|<0.5: {} ppm (expect <20000)",
            max_rel_err
        );
    }

    #[test]
    fn test_pade_fallback_large_z() {
        // For |z| > 0.5, Taylor poly falls back to exact
        for &z_scale in &[6, 8, 10, 15, 20] {
            let z = z_scale as i128 * SCALE_I / 10;
            let rho = -500_000_000_000i128;
            let exact = sabr_z_over_chi(z, rho).unwrap();
            let approx = sabr_z_over_chi_pade(z, rho).unwrap();
            assert_eq!(exact, approx, "Should fall back to exact for z={}", z_scale);
        }
    }

    // --- Invariant (property-based) tests ---

    #[test]
    fn test_sabr_put_call_parity_sweep() {
        let r = 50_000_000_000u128;
        let params: &[(u128, u128, u128, i128, u128)] = &[
            (
                100 * SCALE,
                200_000_000_000,
                SCALE / 2,
                -300_000_000_000,
                400_000_000_000,
            ),
            (
                100 * SCALE,
                300_000_000_000,
                SCALE,
                -700_000_000_000,
                500_000_000_000,
            ),
            (
                100 * SCALE,
                200_000_000_000,
                0,
                -300_000_000_000,
                400_000_000_000,
            ),
            (
                50 * SCALE,
                150_000_000_000,
                700_000_000_000,
                -500_000_000_000,
                300_000_000_000,
            ),
        ];
        let strikes: &[u128] = &[80, 90, 95, 100, 105, 110, 120];
        let mats: &[u128] = &[100_000_000_000, 250_000_000_000, SCALE, 2 * SCALE];

        for &(s, alpha, beta, rho, nu) in params {
            for &k_m in strikes {
                let k = k_m * SCALE;
                for &t in mats {
                    if let Ok((call, put)) = sabr_price(s, k, r, t, alpha, beta, rho, nu) {
                        let disc = exp_fixed_i(-fp_mul_i(r as i128, t as i128).unwrap()).unwrap();
                        let parity = (call as i128 - put as i128 - s as i128
                            + fp_mul_i(k as i128, disc).unwrap())
                        .abs();
                        assert!(
                            parity < 10_000,
                            "P/C parity: s={} k={} t={} err={}",
                            s / SCALE,
                            k / SCALE,
                            t,
                            parity
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_sabr_vol_positive_sweep() {
        let params: &[(u128, u128, u128, i128, u128)] = &[
            (
                100 * SCALE,
                200_000_000_000,
                SCALE / 2,
                -300_000_000_000,
                400_000_000_000,
            ),
            (
                100 * SCALE,
                200_000_000_000,
                SCALE,
                -900_000_000_000,
                800_000_000_000,
            ),
            (100 * SCALE, 200_000_000_000, 0, 0, 100_000_000_000),
        ];
        for &(f, alpha, beta, rho, nu) in params {
            for k_pct in (50..=200).step_by(10) {
                let k = f / 100 * k_pct as u128;
                for &t in &[100_000_000_000u128, SCALE, 5 * SCALE] {
                    if let Ok(vol) = sabr_implied_vol(f, k, t, alpha, beta, rho, nu) {
                        assert!(
                            vol > 0,
                            "Non-positive vol: f={} k={} t={} vol={}",
                            f,
                            k,
                            t,
                            vol
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_sabr_skew_direction() {
        let f = 100 * SCALE;
        let t = SCALE;
        let cases: &[(u128, u128, i128, u128)] = &[
            (
                200_000_000_000,
                SCALE / 2,
                -500_000_000_000,
                200_000_000_000,
            ),
            (
                200_000_000_000,
                700_000_000_000,
                -700_000_000_000,
                300_000_000_000,
            ),
        ];
        for &(alpha, beta, rho, nu) in cases {
            let vol_90 = sabr_implied_vol(f, 90 * SCALE, t, alpha, beta, rho, nu).unwrap();
            let vol_110 = sabr_implied_vol(f, 110 * SCALE, t, alpha, beta, rho, nu).unwrap();
            assert!(
                vol_90 > vol_110,
                "Skew wrong: rho={} vol_90={} vol_110={}",
                rho,
                vol_90,
                vol_110
            );
        }
    }

    #[test]
    fn test_sabr_beta1_continuity() {
        let f = 100 * SCALE;
        let t = SCALE;
        let alpha = 200_000_000_000u128;
        let rho = -300_000_000_000i128;
        let nu = 400_000_000_000u128;

        for &k in &[
            90 * SCALE,
            95 * SCALE,
            100 * SCALE,
            105 * SCALE,
            110 * SCALE,
        ] {
            let vol_exact = sabr_implied_vol(f, k, t, alpha, SCALE, rho, nu).unwrap();
            let vol_near = sabr_implied_vol(f, k, t, alpha, 999_000_000_000, rho, nu).unwrap();
            let diff = (vol_exact as i128 - vol_near as i128).abs();
            let tol = vol_exact as i128 / 100;
            assert!(
                diff < tol,
                "β continuity: k={} vol_1.0={} vol_0.999={} diff={}",
                k / SCALE,
                vol_exact,
                vol_near,
                diff
            );
        }
    }
}
