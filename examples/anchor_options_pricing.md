# Anchor Options Pricing

This is a copy-paste starting point for an Anchor instruction. Keep decimal
parsing off-chain: clients and tests can use `solmath::fp("0.05")`, while the
program receives already-validated `u128` values scaled by `SCALE = 1e12`.

`Cargo.toml`:

```toml
[dependencies]
anchor-lang = "0.31"
solmath = { version = "0.1", default-features = false, features = ["transcendental"] }
```

Instruction code:

```rust
use anchor_lang::prelude::*;
use solmath::{bs_full_hp, SolMathError};

declare_id!("11111111111111111111111111111111");

#[program]
pub mod options_pricer {
    use super::*;

    pub fn quote_option(
        ctx: Context<QuoteOption>,
        spot: u128,
        strike: u128,
        risk_free_rate: u128,
        volatility: u128,
        years_to_expiry: u128,
    ) -> Result<()> {
        let greeks = bs_full_hp(
            spot,
            strike,
            risk_free_rate,
            volatility,
            years_to_expiry,
        )
        .map_err(map_math_error)?;

        let quote = &mut ctx.accounts.quote;
        quote.call = greeks.call;
        quote.put = greeks.put;
        quote.call_delta = greeks.call_delta;
        quote.put_delta = greeks.put_delta;
        quote.gamma = greeks.gamma;
        quote.vega = greeks.vega;
        quote.call_theta = greeks.call_theta;
        quote.put_theta = greeks.put_theta;
        quote.call_rho = greeks.call_rho;
        quote.put_rho = greeks.put_rho;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct QuoteOption<'info> {
    #[account(init_if_needed, payer = payer, space = 8 + OptionQuote::LEN)]
    pub quote: Account<'info, OptionQuote>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct OptionQuote {
    pub call: u128,
    pub put: u128,
    pub call_delta: i128,
    pub put_delta: i128,
    pub gamma: i128,
    pub vega: i128,
    pub call_theta: i128,
    pub put_theta: i128,
    pub call_rho: i128,
    pub put_rho: i128,
}

impl OptionQuote {
    pub const LEN: usize = 2 * 16 + 8 * 16;
}

#[error_code]
pub enum PricingError {
    #[msg("input outside the supported mathematical domain")]
    MathDomain,
    #[msg("fixed-point arithmetic overflow")]
    MathOverflow,
    #[msg("division by zero")]
    MathDivisionByZero,
    #[msg("iterative solver did not converge")]
    MathNoConvergence,
}

fn map_math_error(err: SolMathError) -> anchor_lang::error::Error {
    match err {
        SolMathError::DomainError => PricingError::MathDomain.into(),
        SolMathError::Overflow => PricingError::MathOverflow.into(),
        SolMathError::DivisionByZero => PricingError::MathDivisionByZero.into(),
        SolMathError::NoConvergence => PricingError::MathNoConvergence.into(),
    }
}
```
