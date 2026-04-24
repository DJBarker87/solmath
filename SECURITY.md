# SolMath — Security Properties

This document covers the safety guarantees of `solmath`: panic behaviour,
overflow handling, domain checking, and the distinction between public API
contracts and internal fast paths.

---

## No panics in the production public API

All public functions are either infallible or return `Result<_, SolMathError>`.
The four error variants and their triggers:

| Variant | When it fires |
|---------|---------------|
| `DomainError` | Input outside the mathematical domain (ln of zero, zero barrier level, token_decimals > 38, …) |
| `Overflow` | Intermediate or final result would exceed the representable integer range |
| `DivisionByZero` | Divisor is zero in a divide operation |
| `NoConvergence` | Iterative solver (implied_vol) ran out of iterations |

The only `panic!`/`unwrap`/`expect` calls found in the crate are in tests,
rustdoc examples, test-only helpers, and `debug_assert!` statements. The
`debug_assert!` checks are compiled out in release builds, including Solana
`.so` deployments.

---

## No unsafe code

The library is entirely safe Rust. There is no `unsafe` block anywhere in
`solmath`. This is enforced by `#![forbid(unsafe_code)]` in `lib.rs`.

---

## No heap allocation

`solmath` is `#![no_std]` with zero dependencies. It never allocates.
Every value lives on the stack or in a register. This is a hard requirement
for Solana on-chain programs.

---

## Overflow strategy

The library has three layers of overflow defence:

### 1. Checked primitive arithmetic

Throughout the codebase, `checked_mul`, `checked_add`, `checked_sub` etc. are
the default. Any failure propagates as `Err(SolMathError::Overflow)`.

```rust
// Example from fp_mul:
match a.checked_mul(b) {
    Some(p) => Ok(p / SCALE),
    None => checked_mul_div_u(a, b, SCALE).ok_or(SolMathError::Overflow),
}
```

### 2. U256 fallback for wide multiplications

When a `u128 × u128` product exceeds `u128::MAX`, the library falls through to
a software U256 path rather than returning an error immediately. This means
functions like `fp_mul`, `fp_mul_i`, and the `checked_mul_div_*` family succeed
for all inputs whose *final* result fits in u128/i128, even if the intermediate
product doesn't.

The U256 implementation (`overflow.rs`) uses 128-bit limbs and a long-division
fallback verified by `debug_assert` against the long-division reference.

### 3. Explicit bounds guards at function entry

High-level functions add domain-specific guards before entering arithmetic. For
example, `heston_price` rejects any parameter that exceeds `i128::MAX` or the
i64-scale limit used by the characteristic function:

```rust
if s > i128::MAX as u128 || k > i128::MAX as u128 || … {
    return Err(SolMathError::Overflow);
}
```

---

## Internal fast paths

Two internal functions perform unchecked multiplication:

| Function | Where used | Why it is safe |
|----------|-----------|----------------|
| `fp_mul_i_fast(a, b)` | Trig polynomial cores (`sin_core`, `cos_core`), Heston CF | Inputs are reduction-bounded; comments prove `\|a*b\| < i128::MAX` at every call site |
| `fp_mul_hp_fast(a, b)` | HP ln/exp polynomial evaluation | Callers guarantee `\|a\|, \|b\| <= ~40*SCALE_HP`; product <= 1.6e33, fits i128 (max ~1.7e38) |
| `mul_fast(a, b)` in `iv.rs` | Li rational-guess polynomial | IV solver validates inputs; comments bound each product to ~24 decimal digits < i128::MAX |

These are `pub(crate)` and cannot be called from external code. Every call site
carries a comment explaining why no overflow can occur.

---

## Angle reduction safety

`sin_fixed` and `cos_fixed` reduce arbitrary angles via `rem_euclid`, which
handles `i128::MIN` correctly (unlike the raw `%` operator, which has
implementation-defined sign for negative dividends in many languages). The
reduced value is always in `(-π, π]` before polynomial evaluation.

---

## Division-by-zero handling

- `fp_div`, `fp_div_i`, `fp_div_floor`, `fp_div_ceil`, `fp_div_round` — return
  `Err(DivisionByZero)` when the divisor is zero.
- `fp_div_hp_safe` — same.
- `weighted_pool_swap` — returns `Err(DivisionByZero)` when `weight_out == 0`.
- All `checked_mul_div_*` variants — check `c == 0` and return `None`/`Err`.

---

## Convergence and iteration bounds

`implied_vol` uses three successive methods (Li rational initial guess → Halley
→ Jaeckel rational). Each iteration loop has an explicit `max_iter` cap. If the
solver does not converge inside the cap it returns `Err(NoConvergence)` rather
than looping forever.

---

## What is not guaranteed

- **Accuracy after extreme inputs.** The library is designed for financially
  realistic inputs (spot/strike in reasonable ranges, σ ∈ (0, 5), T ∈ (0, 10)).
  Functions will *not panic* outside these ranges and will *attempt* to return a
  result, but accuracy is not validated beyond the tested parameter space.
  See `PROOFS.md` and the benchmark validation reports for tested ranges.

- **Cryptographic security.** SolMath is financial math, not a cryptographic
  library. No timing-side-channel guarantees are made.

- **Formal verification.** Error bounds in `PROOFS.md` are analytically derived
  with AI assistance but have not been independently machine-checked.

---

## Audit history

No independent third-party audit is claimed for the published crate. The
current public validation consists of reproducible reference vectors,
deterministic property-style tests, and internal review documented in
`PROOFS.md` and the generated test data. Treat financial-model use as unaudited
until your integration has its own review.
