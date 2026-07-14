# SolMath Examples

- `options_pricing.rs` — Black-Scholes price and Greeks with `fp("...")` input helpers.
- `twap_options.rs` — partially fixed 30-minute arithmetic-Asian/TWAP settlement; run with `--features asian`.
- `asian_batch.rs` — line-oriented offline validation harness; run with `--features asian`.
- `american_kbi_batch.rs` — line-oriented KBI runtime validator; run with `--features american-kbi`.
- `nig_batch.rs` — line-oriented exponential-NIG runtime validator; run with `--features nig`.
- `weighted_pool_swap.rs` — Balancer-style weighted pool swap; run with `--features pool`.
- `safe_token_conversion.rs` — floor/ceil token conversion policy; run with `--features pool`.
- `anchor_options_pricing.md` — Anchor instruction template for option quoting.
