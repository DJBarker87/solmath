# SolMath — Error Bound Analysis

Error bounds for core fixed-point functions. Bounds are stated in output units at
SCALE = 10^12 (or SCALE_HP = 10^15 where noted): an error of 1 means the computed
integer result differs from the ideal real-valued result by at most 1 in the
least-significant fixed-point unit.

All source references are to files under `src/`.

**Disclaimer:** The methodology and assumptions in this document are sound to the
author's knowledge, but the proofs themselves were generated with AI assistance and
have not been independently verified. The discerning reader should reproduce any
result before relying on it. The empirical benchmarks were run by the author and are
the primary accuracy reference.

**Certificate script status:** The Lipschitz certificate scripts referenced below
(`lipschitz_certificate.py`, `trig_lipschitz_certificate.py`) are not included in
the current crate package. The polynomial certification chain for Propositions 6,
9, and 10 should be treated as non-reproducible until those scripts are restored
and re-run against the current source. The empirical benchmark results remain the
primary accuracy reference.

-----

## Definitions

**Definition 1 (Fixed-point representation).** The library represents real numbers as
integers scaled by SCALE = 10^12. An integer `v` at standard scale represents the
real number `v / SCALE`. For example, the integer 1,500,000,000,000 represents 1.5.
All arithmetic operates on these integer representatives, and all error bounds in
this document are stated in terms of the integer representation unless otherwise noted.

**Definition 2 (Unit in the last place — ULP).** One ULP is 1 in the integer
representation at the relevant scale. At standard scale, 1 ULP = 1 (representing
10^-12 in real-valued terms). At high-precision scale, 1 ULP = 1 (representing
10^-15 in real-valued terms). This is not floating-point ULP in the IEEE 754
sense — it is a fixed absolute quantity determined by the scaling factor.

**Definition 3 (High-precision scale).** SCALE_HP = 10^15. Certain functions
(prefixed `_hp`) operate at this finer resolution. Conversion between scales is
performed by multiplication or division by 1000.

**Definition 4 (Truncation and rounding).** Three rounding modes appear:

- `trunc(x)` denotes truncation toward zero (Rust integer division semantics for
  signed types). For non-negative x this is identical to the mathematical floor.
  For negative x, truncation toward zero rounds toward +∞: e.g. trunc(−2.7) = −2,
  whereas floor(−2.7) = −3.
- `floor(x)` denotes rounding toward −∞. Used only for unsigned paths (where
  it coincides with trunc) and for the dedicated floor/ceil rounding modes.
- `round(x)` denotes rounding to nearest integer. The precise tie-breaking rule
  varies by call site: `(v + S/2) / S` for unsigned values rounds ties toward
  +∞, while the sign-aware variant in `fp_mul_hp_fast` rounds ties away from zero.
  For the purposes of error bounds, all rounding-to-nearest variants satisfy
  `|round(x) - x| <= 0.5` regardless of tie-breaking rule.

**Definition 5 (Error).** For a mathematical function f, the notation f-hat(x)
denotes the library's computed result and f(x) denotes the true mathematical value.
The error of the computation is `|f-hat(x) - f(x)|`, measured in ULP at the
appropriate scale.

-----

## Rust arithmetic guarantees

The proofs rely repeatedly on the following five properties of Rust's
integer arithmetic. These are language-level guarantees documented in the Rust
Reference and standard library — they hold on all platforms and all Rust versions.

**(R1) Integer multiplication is exact.** The `*` operator on integer types produces
the exact mathematical product when the result fits in the type. No rounding occurs.
If the result does not fit, the operation panics in debug mode or wraps (two's
complement) in release mode. There is no silent truncation of a value that fits.

**(R2) Integer division truncates toward zero.** The `/` operator on signed integer
types rounds the mathematical quotient toward zero, discarding any fractional part.
For unsigned types, truncation toward zero is equivalent to floor division. This
operation panics if the divisor is zero or if the division overflows (only possible
for `i128::MIN / -1`).

**(R3) The remainder operator is exact and consistent with division.** For all
integers a, b with b ≠ 0: `(a / b) * b + (a % b) == a`. This identity holds
exactly. The sign of `a % b` matches the sign of `a` (consistent with truncating
division).

**(R4) `checked_mul` returns `None` on overflow.** The method `a.checked_mul(b)`
returns `Some(a * b)` if the product fits in the type, or `None` if it would
overflow. No wrapping, no silent truncation — the caller gets an unambiguous signal.
The same pattern applies to `checked_add`, `checked_sub`, etc.

**(R5) Overflow behaviour is defined.** In debug mode, integer overflow on `+`, `-`,
`*` panics. In release mode, it wraps (two's complement for signed types, modular
for unsigned). The `wrapping_*`, `saturating_*`, and `checked_*` method families
provide explicit control. The library uses checked arithmetic and explicit
`Result` errors at overflow-sensitive points.

These five properties are referenced throughout the proofs as **(R1)**–**(R5)**.

*Sources: The Rust Reference (Operator Expressions) for R1, R2, R5; The Rust Book
(Data Types §3.2) and `std::ops::Div` documentation for R2; C99/C11 truncating
division semantics for R3; standard library `i128::checked_mul` documentation for R4.*

-----

## Lipschitz certificate method

Several Category B results (Proposition 6, Proposition 9, Proposition 10) claim that
a polynomial approximation's error is bounded *everywhere* on an interval, not just at
the points where it was tested. This section explains how finite grid sampling can yield
a continuous guarantee.

**The core idea.** If a function e(x) can change at rate at most L per unit of x
(i.e. |e'(x)| ≤ L everywhere), then the value at any point is within L × d of the
nearest measured value, where d is the distance to that measurement. On a uniform
grid with spacing h, the furthest any point can be from the nearest grid point is
h/2 (the midpoint between consecutive grid points). Therefore:

```
max |e(x)| over the continuous interval  ≤  max |e(x)| over grid points  +  L × h/2
```

This is a direct consequence of the mean value theorem: if |e'| ≤ L everywhere,
then |e(a) - e(b)| ≤ L × |a - b| for all a, b. The quantity L is called the
**Lipschitz constant** of e.

**The problem with L.** To use this technique, you need a rigorous bound on L = sup|e'(x)|.
But bounding the maximum of e'(x) is the same type of problem — you'd want to sample
e'(x) on a grid and bound the gaps. This seems circular.

**The three-level chain breaks the circularity.** The solution is to go up to the third
derivative, where the problem becomes purely algebraic:

1. **Level 3 — bound |e'''(x)| analytically.** The error function is
   e(x) = p(x) - f(x)·SCALE, where p is a polynomial with known integer coefficients
   and f is the target function (e.g. Φ for the CDF). The third derivative p'''(x) is
   bounded exactly by summing absolute values of coefficients (triangle inequality).
   The third derivative f'''(x) has a known analytical bound (e.g. for the normal CDF,
   Φ'''(x) involves (1 - x²)φ(x) which has a computable global maximum).
   **No grid sampling at this level** — the bound M3 on |e'''(x)| is purely algebraic.
2. **Level 2 — bound |e''(x)| using M3.** Sample e''(x) on a 10K-point grid to find
   the grid maximum M2_grid. Since e'' has Lipschitz constant at most M3 (because
   |e'''| ≤ M3), apply the core idea: M2 = M2_grid + M3 × h₂/2.
3. **Level 1 — bound |e'(x)| using M2.** Sample e'(x) on a 100K-point grid to find
   L_grid. Since e' has Lipschitz constant at most M2: L = L_grid + M2 × h₁/2.
4. **Certificate — bound |e(x)| using L.** Sample e(x) on a 100K-point grid to find
   grid_max. Apply the core idea: certified max = grid_max + L × h/2.

Each level's grid-based maximum is elevated to a rigorous continuous bound by the
level above. The chain terminates at Level 3 with pure algebra, so there is no
circularity.

**Precision of grid evaluations.** At each grid point, the error function and its
derivatives are evaluated to 60-digit precision using mpmath (a Python arbitrary-
precision library). These are not finite-difference approximations — they are exact
symbolic derivatives of the polynomial, evaluated against the target function computed
to far more precision than needed. The grid evaluations are effectively exact; the
only gap is between grid points, which is what L × h/2 fills in.

**Why h/2 and not h.** On a grid with spacing h, every point in the interval is at
most h/2 from the nearest grid point (the worst case is the midpoint between two
consecutive grid points). If the grid were non-uniform, you would replace h/2 with
half the maximum gap between consecutive points.

The Lipschitz certificate results are referenced in Proposition 6, Proposition 9,
and Proposition 10, and detailed further in the Appendix (Lipschitz certificates).

-----

## Bound classification

Not all bounds in this document have the same epistemological status. Each result
falls into one of three categories:

|Category                                            |Meaning                                                                                                                                                                                                                                                                                                                                   |Results                                                                                                                                                                                                                           |
|----------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|**A — Exact integer proofs**                        |The computation is proven exact (0 error) or the only error is the unavoidable final truncation/rounding remainder, proved from the code's integer arithmetic with no approximation theory involved.                                                                                                                                      |Lemma 1, Proposition 1 Path B, Lemma 3, Lemma 2, Proposition 2, Proposition 3, Proposition 4, Proposition 5                                                                                                                       |
|**A/B — Exact with analytical convergence argument**|The computation is structurally exact (integer Newton iteration converging to floor(sqrt)), with convergence guaranteed by a standard analytical argument. No approximation theory involved, but the convergence proof is not machine-checked.                                                                                            |Proposition 1 Path A                                                                                                                                                                                                              |
|**B — Analytical conservative bounds**              |The error bound is derived by summing worst-case contributions from each rounding step in the algorithm. For polynomial approximation results, the minimax approximation error is rigorously certified via Lipschitz analysis. These are genuine analytical arguments but rely on worst-case accumulation that is unlikely to be realized simultaneously.|Proposition 6, Proposition 7, Proposition 8, Proposition 9, Proposition 10, Proposition 11 (restricted domain), Proposition 12                                                                                                    |

**How to read this document:** Category A bounds are unconditionally reliable — they
follow from integer arithmetic identities that hold by construction. Category B bounds
are reliable but conservative; they could in principle be tightened by tracking error
correlations. For polynomial approximation results (Proposition 6, Proposition 9,
Proposition 10), the minimax approximation error is rigorously certified via Lipschitz
analysis (see Appendix for the three-level rigour chain). All bounds are consistent
with (and significantly more conservative than) empirical benchmarks.

-----

## Lemmas

The following three results establish the error behaviour of the library's
fundamental building blocks and are used repeatedly by the main propositions.

-----

### Lemma 1 (fp_mul_i — truncating fixed-point multiplication)

**Source:** `arithmetic.rs:12-14`

**Statement.** For all integers a, b where `trunc((a * b) / SCALE)` fits in i128,

```
|fp_mul_i(a, b) - (a * b) / SCALE| < 1 ULP.
```

When `trunc((a * b) / SCALE)` does not fit in i128, the function returns
`Err(Overflow)`.

The true value is `f(a,b) = (a * b) / SCALE` (real-valued division).

*Proof.* Two code paths.

*Fast path.* When `a.checked_mul(b)` succeeds **(R4)**, the product `a * b` is exact
in i128 by **(R1)**. Rust integer division `(a * b) / SCALE` truncates toward zero
by **(R2)**. The truncation error satisfies

```
|trunc((a*b) / SCALE) - (a*b) / SCALE| = |(a*b) mod SCALE| / SCALE
```

Since `0 <= |(a*b) mod SCALE| < SCALE`, the error lies in [0, 1).

*Widened fallback.* When `a * b` overflows i128, the function delegates to
`checked_mul_div_i(a, b, SCALE)` (Lemma 3), which computes `trunc((a * b) / SCALE)`
exactly via U256 arithmetic. If the exact result fits in i128, the error is the same
truncation remainder (in [0, 1)). If it does not fit, `checked_mul_div_i` returns
`Err(Overflow)`. The fast path adds zero overhead (a single `imul` + overflow flag
check); the widened path fires only for extreme inputs. □

*Remark (Empirical validation).* Max observed error = 1 ULP across benchmark vectors
(production suite: fp_mul_i max ULP = 0 across 95K vectors; on-chain harness max = 1). The bound is
strict (< 1 ULP), but the benchmark reports integer ULP so any nonzero truncation
registers as 1 ULP. The actual error is always in [0, 1), never exactly 1.0 in
real-valued terms.

-----

### Lemma 2 (fp_div / fp_div_i — exact truncated fixed-point division)

**Source:** `arithmetic.rs:35-38` (unsigned), `arithmetic.rs:45-74` (signed),
both delegating to `fp_div_rem_experimental_u` at `arithmetic.rs:122-155`.

**Statement.**

- `fp_div(a, b)` computes `floor(a * SCALE / b)` exactly (unsigned).
- `fp_div_i(a, b)` computes `trunc(a * SCALE / b)` exactly (signed, toward zero).

In both cases, the only error relative to real-valued division is the unavoidable
final truncation remainder, which is strictly less than one ULP.

The true value is `f(a,b) = a * SCALE / b` (real-valued).

**Implementation.** `fp_div(a, b)` returns `Err(DivisionByZero)` when `b == 0`.
Otherwise it calls `fp_div_rem_experimental_u(a, b)` and returns the quotient, or
`Err(Overflow)` if the quotient cannot be represented. The core function
`fp_div_rem_experimental_u` computes `floor(a * SCALE / b)` and `(a * SCALE) mod b`
via three paths:

**Path 1** (thin, `arithmetic.rs:131-134`): When `a <= FP_DIV_THIN_MAX` (= u128::MAX / SCALE):

```
scaled = a * SCALE      (exact — no overflow by the guard)
return (scaled / b, scaled % b)
```

This is exact integer division. Result = `floor(a * SCALE / b)`.

**Path 2** (quotient decomposition, `arithmetic.rs:139-154`):
When `a > FP_DIV_THIN_MAX` and `q = a / b` satisfies `0 < q <= FP_DIV_THIN_MAX`:

```
q = a / b              (integer division, exact quotient)
r = a - q * b          (exact remainder, 0 <= r < b)
base = q * SCALE       (exact — q <= FP_DIV_THIN_MAX)
(frac, rem) = fp_div_fractional_tail_u(r, b)   // floor(r * SCALE / b)
return (base + frac, rem)
```

**Key identity:** `a * SCALE / b = q * SCALE + r * SCALE / b`, where `q = floor(a/b)`
and `r = a mod b`. Since `q * SCALE` is an exact integer:

```
floor(a * SCALE / b) = q * SCALE + floor(r * SCALE / b) = base + frac
```

The decomposition is algebraically exact. `fp_div_fractional_tail_u` (`arithmetic.rs:109-119`)
computes `floor(r * SCALE / b)` either directly (when `r <= FP_DIV_THIN_MAX`) or via
`checked_mul_div_rem_u` (U256 widened arithmetic). Both are exact integer truncation.

**Path 3** (wide, `arithmetic.rs:140-141`): When `q = 0` (i.e. `a < b`) and
`a > FP_DIV_THIN_MAX`: delegates to `checked_mul_div_rem_u(a, SCALE, b)`, which
uses U256 exact 256-bit arithmetic (proved exact in Lemma 3). Returns
`floor(a * SCALE / b)`.

*Proof.* In all three paths, the returned quotient is exactly `floor(a * SCALE / b)`.

**Path 1:** The product `a * SCALE` is exact by **(R1)** (the guard ensures no
overflow). Division `scaled / b` truncates toward zero by **(R2)**, which for
unsigned values is floor division. The remainder is exact by **(R3)**.

**Path 2:** The algebraic decomposition uses the identity
`floor(n + x) = n + floor(x)` for any integer n and real x. The integer quotient
`q = a / b` and remainder `r = a - q * b` are exact by **(R2)** and **(R3)**
respectively. The product `q * SCALE` is exact by **(R1)** (the guard ensures
`q <= FP_DIV_THIN_MAX`). The fractional tail uses either the thin path or U256
arithmetic, both exact.

**Path 3:** The U256 widened computation `floor(a * SCALE / b)` is exact by the
proof in Lemma 3.

Relative to the real-valued scaled quotient `a * SCALE / b`, the only error is
the truncation remainder:

```
|floor(a*SCALE/b) - a*SCALE/b| = ((a*SCALE) mod b) / b
```

Since `0 <= (a*SCALE) mod b < b`, this remainder lies in [0, 1).

**Signed variant (fp_div_i):** The function takes absolute values of both arguments,
calls the same `fp_div_rem_experimental_u`, and restores the sign (`arithmetic.rs:53-73`).
The sign logic and i128 range check are exact. Truncation is toward zero, consistent
with Rust integer division semantics by **(R2)**. The same conclusion applies: exact
`trunc(a * SCALE / b)`, with the only error being the truncation remainder. □

*Remark (Overflow).* When `fp_div_rem_experimental_u` returns `None` (quotient
overflows u128, or `q > FP_DIV_THIN_MAX` in Path 2), `fp_div` returns
`Err(Overflow)`. The < 1 ULP bound applies only to `Ok` results.

*Remark (fp_div_ceil and fp_div_floor).* `fp_div_ceil` (`arithmetic.rs:85-90`) uses
the exact remainder from `fp_div_rem_experimental_u`: if `rem > 0`, it adds 1 to the
truncated quotient, giving `ceil(a*SCALE/b)` exactly. `fp_div_floor` (`arithmetic.rs:78-80`)
is identical to `fp_div` for unsigned inputs. Both are 0 ULP vs their respective
rounding semantics. These exact-rounding results are stated formally in Proposition 5.

*Remark (Empirical validation).*
`fp_div`: max observed error = 1 ULP across 100K production vectors.
`fp_div_i`: max observed error = 0 ULP across 100K production vectors.
`checked_mul_div_i`: max observed error = 0 ULP across 100K production vectors.
The 1 ULP observations in unsigned division are from nonzero
truncation remainders, consistent with the < 1 ULP real-valued bound.

-----

### Lemma 3 (checked_mul_div_i — exact widened multiply-divide)

**Source:** `overflow.rs:99-101`, delegating to `checked_mul_div_round_i` at `overflow.rs:60-96`,
which calls `checked_mul_div_rem_u` at `overflow.rs:23-52`.

**Statement.** When `checked_mul_div_i(a, b, c)` returns `Some(result)`,

```
result = trunc((a * b) / c)    exactly (0 error).
```

**Implementation.**

1. `checked_mul_div_i(a, b, c)` calls `checked_mul_div_round_i(a, b, c, MulDivRounding::ToZero)`
   (`overflow.rs:100`).
1. `checked_mul_div_round_i` computes the sign, takes absolute values, and calls
   `checked_mul_div_rem_u(|a|, |b|, |c|)` (`overflow.rs:72`).
1. `checked_mul_div_rem_u(a, b, c)` (`overflow.rs:23-52`):

- If `a * b` fits in u128 (`a.checked_mul(b)` succeeds by **(R4)**): returns
  `(product / c, product % c)` — exact integer division by **(R2)** and **(R3)**.
- Otherwise: computes `U256::mul_u128(a, b)` (`constants.rs:125-146`),
  a 256-bit unsigned product, then divides by c via either `div_rem_u64`
  (for c <= u64::MAX) or `div_rem_u128` (for larger c).

1. The sign is restored and overflow is checked against the i128 range.

*Proof.* The argument proceeds in four steps.

*(A) The 256-bit multiplication is exact.*
Since `|a| <= 2^127` and `|b| <= 2^127` (both fit in i128), the product
`|a| * |b| <= 2^254`, which fits in U256 (256 bits). The implementation
`U256::mul_u128` (`constants.rs:125-146`) performs schoolbook multiplication
on 64-bit limbs with carry propagation. Each limb multiplication is exact
by **(R1)** and carry propagation is exact integer addition. No truncation
or rounding occurs.

*(B) The 256-bit division is exact integer division.*
`div_rem_u64` (`constants.rs:165-178`) performs long division using 128-bit
intermediate values for each 64-bit limb. `div_rem_u128` (`constants.rs:181+`)
performs Knuth's Algorithm D (normalized long division). Both operate on unsigned
(absolute) values and compute `floor(numerator / divisor)` by **(R2)** and the
exact remainder by **(R3)**. For unsigned values, floor division and truncation
toward zero are identical.

*(C) Sign correction is exact.*
Negation of an integer is exact in two's complement, with the one edge case
`i128::MIN` handled explicitly at `overflow.rs:84-85`.

*(D) The overflow check is correct.*
If the unsigned quotient exceeds `i128::MAX` (or `i128::MIN` for negative
results), `None` is returned (`overflow.rs:88-95`). When `Some` is returned,
the value is within the i128 range.

Combining (A)–(D), the returned value is exactly `trunc((a * b) / c)`.
Error = 0. □

*Remark (checked_mul_div_floor_i and checked_mul_div_ceil_i).* These variants
(`overflow.rs:104-111`) use the same `checked_mul_div_round_i` with
`MulDivRounding::Floor` or `MulDivRounding::Ceil`. The floor/ceil adjustment
(`overflow.rs:74-81`) adds 1 to the magnitude when the exact remainder is nonzero
and the rounding mode requires it. Since the remainder from `checked_mul_div_rem_u`
is exact, the floor/ceil adjustment is exact. These are also 0 ULP. The formal
statement is Proposition 4.

*Remark (Empirical validation).* Max observed error = 0 ULP across benchmark vectors
(production suite: checked_mul_div_i max ULP = 0 across 100K vectors).

-----

## Propositions

Ordered so that each result depends only on lemmas and propositions already stated.

-----

### Proposition 1 (fp_sqrt — fixed-point square root)

**Source:** `arithmetic.rs:220-263`

**Statement.** For all x in [0, u128::MAX],

```
|fp_sqrt(x) - sqrt(x * SCALE)| < 1 ULP.
```

**Classification.** Path B is Category A (exact by 256-bit bisection). Path A is
Category A/B: the algorithm is structurally exact (integer Newton iteration converging
to floor(sqrt)), with convergence guaranteed by a standard analytical argument that
is not machine-checked. Both paths are empirically validated to < 1 ULP on all tested inputs.

**Implementation.** Two code paths depending on whether `x * SCALE` fits in u128:

**Path A** (no overflow, `arithmetic.rs:225-227`): When `x.checked_mul(SCALE)` succeeds:

1. Compute `scaled = x * SCALE` (exact, fits in u128)
1. Call `sqrt_scaled_newton(scaled)` (`arithmetic.rs:172-187`)

`sqrt_scaled_newton` performs:

1. Initial guess: `g = 1 << ((bit_len + 1) / 2)` where `bit_len = 128 - scaled.leading_zeros()`
1. Up to 8 Newton-Raphson iterations: `g_new = (g + scaled / g) / 2`
1. Early termination when `g_new == g` or `g_new + 1 == g`; returns `min(g, g_new)`

**Path B** (overflow, `arithmetic.rs:229-262`): When `x * SCALE` overflows u128:

1. Reduce: repeatedly divide x by 4 and accumulate `scale_back *= 2`
   (using `sqrt(4a) = 2*sqrt(a)`) until `reduced * SCALE` fits in u128
1. Compute approximate root: `sqrt_scaled_newton(reduced * SCALE) * scale_back`
1. **Exact refinement via 256-bit bisection** (`arithmetic.rs:253-260`):
   binary-search for the largest integer `r` such that
   `wide_mul(r, r) <= wide_mul(x, SCALE)` using exact 256-bit arithmetic
   (`wide_mul_u128` at `arithmetic.rs:189-206`, `cmp_sqrt_candidate` at `arithmetic.rs:216-218`)

The true value is `f(x) = sqrt(x * SCALE)` (real-valued).

*Proof of Path B (Category A — exact).*

*(A) Initial approximation.* The reduction step computes `reduced = ceil(x / 4^j)`
for some j, with `scale_back = 2^j`. The rounding in `(reduced + 2) / 4` introduces
error, but this only affects the initial approximation for the bisection.

*(B) 256-bit bisection.* The bisection at `arithmetic.rs:253-260` finds
`low = max { r : r^2 <= x * SCALE }`, where the comparison `r^2 <= x * SCALE` is
performed in exact 256-bit arithmetic via `cmp_sqrt_candidate(mid, x)`, which
computes `wide_mul(mid, mid)` vs `wide_mul(x, SCALE)` with no overflow.

*(C) Termination.* The binary search terminates when `low + 1 == high`, which
means `low^2 <= x*SCALE < (low+1)^2`. This is exactly the condition
`low = floor(sqrt(x*SCALE))`.

*(D) Error.* Path B returns `floor(sqrt(x * SCALE))` exactly. The error satisfies
`sqrt(x * SCALE) - floor(sqrt(x * SCALE)) ∈ [0, 1)`. □ (Path B)

*Proof of Path A (Category A/B — analytical convergence).*

*(A) Scaling.* The product `scaled = x * SCALE` is exact by **(R1)**, since the
checked multiply confirms no overflow.

*(B) Overestimate.* The initial guess is `g_0 = 2^(ceil(bit_len/2))`. Since
`2^(ceil(log2(n)/2)) >= sqrt(n)` for all n > 0, the initial guess satisfies
`g_0 >= sqrt(scaled)`.

*(C) Monotone convergence.* Each iteration computes
`g_{k+1} = floor((g_k + scaled/g_k) / 2)` by **(R2)**. For any g > floor(sqrt(n)),
the integer iteration `floor((g + floor(n/g)) / 2)` is strictly less than g — this
is the key property that makes integer Newton iteration terminate. The floor operations
in integer division do not break monotone convergence; they ensure it. The sequence is
non-increasing for k >= 1 and converges to floor(sqrt(scaled)). (This is a standard
result; see Cohen, *A Course in Computational Algebraic Number Theory*, §1.7.1.)

*(D) Convergence rate.* Let `m = bit_len` so that `2^(m-1) <= scaled < 2^m`.
The true root satisfies `sqrt(scaled) >= 2^((m-1)/2)`. Therefore:

```
g_0 / sqrt(scaled) <= 2^(ceil(m/2)) / 2^((m-1)/2) = 2^(ceil(m/2) - (m-1)/2)
```

For even m: `ceil(m/2) - (m-1)/2 = 1/2`, so `g_0/sqrt(scaled) <= sqrt(2) < 2`.
For odd m: `ceil(m/2) - (m-1)/2 = 1`, so `g_0/sqrt(scaled) <= 2`.
In both cases, `g_0 / sqrt(scaled) <= 2`. The first iteration yields:

```
g_1/sqrt(scaled) - 1 <= (g_0/sqrt(scaled) - 1)^2 / (2 * g_0/sqrt(scaled)) <= 1/4
```

so `g_1` is within 25% of the root and quadratic convergence applies from k=1
onwards. After 8 iterations, the relative error is bounded by
`(0.25)^(2^7) = (0.25)^128 < 10^-77` — far below a single ULP for any 128-bit input.

*(E) Termination.* The early termination condition (`g_new == g` or `g_new + 1 == g`)
detects convergence and returns `min(g, g_new) = floor(sqrt(scaled))`.

*(F) Error.* Path A returns `floor(sqrt(x * SCALE))` exactly. The error satisfies
`sqrt(x * SCALE) - floor(sqrt(x * SCALE)) ∈ [0, 1)`. □ (Path A)

**Combined.** In both paths, `f-hat(x) = floor(sqrt(x * SCALE))` and the error
`|f-hat(x) - f(x)| = sqrt(x*SCALE) - floor(sqrt(x*SCALE))` lies in [0, 1).
Therefore **|f-hat(x) - f(x)| < 1 ULP** for all valid x. □

*Remark (Post-check).* A `debug_assert!` post-check already exists at
`arithmetic.rs:186-191`, verifying `g * g <= scaled && (g+1) * (g+1) > scaled`.
This catches convergence failures during development and testing. Promoting it to
an unconditional `assert!` would upgrade Path A to Category A (the output would be
verifiably correct regardless of convergence analysis), at the cost of a small CU
increase in production.

*Remark (Empirical validation).* Max observed error = 1 ULP across benchmark vectors
(production suite: fp_sqrt max ULP = 1 across 100K vectors). The ≤ 1 ULP
integer observation is consistent with the < 1 ULP real-valued bound: any `x` where
`x * SCALE` is not a perfect square has fractional truncation error in (0, 1), which
registers as 1 ULP in the integer-valued benchmark.

-----

### Proposition 2 (fp_mul_hp — high-precision fixed-point multiplication)

**Source:** `hp.rs:4-37`

All three variants compute fixed-point multiplication at SCALE_HP = 10^15.

**Statement.** For each variant, `|f-hat(a,b) - a*b/SCALE_HP| <= 0.5 ULP`
when `Ok(result)` is returned.

#### 2a. fp_mul_hp_u — overflow-safe unsigned

**Implementation** (`hp.rs:4-16`): Four-part schoolbook decomposition:

```
a = hi_a * S + lo_a,   b = hi_b * S + lo_b    (where S = SCALE_HP)
result = hi_a*hi_b*S + hi_a*lo_b + lo_a*hi_b + round(lo_a*lo_b / S)
```

The first three terms are exact integers (no division). The fourth term
`ll = (lo_a*lo_b + S/2) / S` rounds to nearest, with error <= 0.5 ULP.

*Proof.* The only rounding is in the `ll` term. Since `lo_a, lo_b < SCALE_HP`,
the product `lo_a * lo_b < SCALE_HP^2 = 10^30`, which fits in u128 (max ~3.4e38)
by **(R1)**. The rounding `(lo_a*lo_b + S/2) / S` by **(R2)** gives |error| <= 0.5.
All other terms are exact products and sums. □

#### 2b. fp_mul_hp_i — signed wrapper

**Implementation** (`hp.rs:19-28`): Takes absolute values, calls `fp_mul_hp_u`,
restores sign, clamps to i128 range. Sign restoration and clamping are exact.
Inherits the <= 0.5 ULP bound from Part 2a. □

#### 2c. fp_mul_hp_fast — direct rounding division

**Implementation** (`hp.rs:31-38`):

```
f-hat(a,b) = round((a * b) / SCALE_HP)
```

where rounding adds `SCALE_HP/2` before truncating (sign-aware).

*Proof.* The integer product `a * b` is exact by **(R1)** (precondition: no overflow).
Rounding division by SCALE_HP by **(R2)**: `|(a*b + S/2)/S - a*b/S| <= 0.5`. □

*Remark (Empirical validation).* `fp_mul_hp_i` max observed error = 0 ULP across
100K production vectors.

-----

### Proposition 3 (fp_div_hp_safe — high-precision fixed-point division)

**Source:** `hp.rs:41-61`

**Statement.** When `Ok(result)` is returned, `fp_div_hp_safe` computes
`trunc(a * SCALE_HP / b)` exactly. The error relative to real-valued division
is strictly less than one HP-scale ULP.

**Implementation:** Quotient-remainder decomposition at SCALE_HP:

```
q = a / b                    (exact integer quotient)
r = a % b                    (exact remainder)
q_scaled = q * SCALE_HP      (checked — returns Err on overflow)
r_scaled = (r * SCALE_HP) / b, using U256 fallback if r*SCALE_HP overflows i128
result = q_scaled + r_scaled
```

*Proof.* This is the same algebraic decomposition as Lemma 2 (Path 2), adapted for
signed inputs. Rust's `a / b` and `a % b` for signed integers truncate toward zero
by **(R2)** and **(R3)**, giving `q = trunc(a / b)` and `r = a - q*b` with
`|r| < |b|` and `sign(r) = sign(a)`.

The identity `a*S/b = q*S + r*S/b` holds exactly in real arithmetic. The only
rounding is `(r * SCALE_HP) / b` — signed integer division truncating toward zero,
with error magnitude in [0, 1). Since `a = q*b + r` by definition of truncating
division, and `q*S` is an exact integer, the truncation of the full expression
`a*S/b` decomposes as `q*S + trunc(r*S/b)`, which is exactly what the code computes. □

*Remark (Overflow on remainder path — resolved).* The remainder path originally
used `checked_mul_div_i(r, SCALE_HP, b)` (Lemma 3) but fell back to 0 on overflow
via `unwrap_or(0)` — silently returning an incorrect result. This has been fixed:
the fallback now uses `checked_mul_div_i(r, SCALE_HP, b)?`, which computes through
U256 or returns `Err(Overflow)`. The direct `r * SCALE_HP` overflow occurs when
`|r| > i128::MAX / SCALE_HP` (≈ 1.7 × 10^23), requiring `|b| > 1.7 × 10^23`
— well beyond typical HP-scale inputs.

-----

### Proposition 4 (checked_mul_div_floor/ceil — exact rounded multiply-divide)

**Source:** `overflow.rs:104-111`, both delegating to `checked_mul_div_round_i`
at `overflow.rs:60-96`.

**Statement.** When `Some(result)` is returned, the result is exact: 0 ULP.

- `checked_mul_div_floor_i(a, b, c)` returns `floor((a*b) / c)` (toward −∞).
- `checked_mul_div_ceil_i(a, b, c)` returns `ceil((a*b) / c)` (toward +∞).

*Proof.* Both call `checked_mul_div_round_i`, which computes the exact unsigned
quotient and remainder via U256 arithmetic (proved exact in Lemma 3). The rounding
adjustment at `overflow.rs:74-81`:

- **Floor** (`MulDivRounding::Floor`): when the remainder is nonzero and the
  result is negative, adds 1 to the magnitude (moving toward −∞).
- **Ceil** (`MulDivRounding::Ceil`): when the remainder is nonzero and the
  result is positive, adds 1 to the magnitude (moving toward +∞).

The adjustment uses the exact remainder from `checked_mul_div_rem_u` (Lemma 3): if
`rem != 0`, the true quotient has a fractional part, and the floor/ceil correction
is exactly +1 or 0. No approximation occurs. □

*Remark (Empirical validation).* Same U256 engine as Lemma 3 — 0 ULP.

-----

### Proposition 5 (fp_div_floor / fp_div_ceil — exact rounded division)

**Source:** `arithmetic.rs:78-90`

**Statement.** Both compute exact integer rounding of `a * SCALE / b`: 0 ULP
relative to their respective rounding semantics.

**fp_div_floor** (`arithmetic.rs:78-80`): Identical to `fp_div` (Lemma 2). For
unsigned inputs, truncation toward zero is floor division. Result: exact `floor(a*S/b)`.

**fp_div_ceil** (`arithmetic.rs:85-90`): Calls `fp_div_rem_experimental_u(a, b)`,
which returns the exact quotient `floor(a*S/b)` and the exact remainder
`(a*S) mod b`.

*Proof.* The remainder from `fp_div_rem_experimental_u` is exact (all three paths
in Lemma 2 preserve the exact remainder). When `rem > 0`, the true quotient `a*S/b`
has a fractional part, so `floor + 1 = ceil`. When `rem = 0`, the quotient is exact
and floor = ceil. □

-----

### Proposition 6 (norm_cdf_poly — standard-scale normal CDF)

**Source:** `normal.rs:80-117`

**Classification:** Category B — Lipschitz-certified approximation error plus
analytical implementation overhead.

**Statement.** For all x in [-8*SCALE, 8*SCALE],

```
|norm_cdf_poly(x) - Phi(x/SCALE) * SCALE| <= 9 ULP,
```

where Φ is the standard normal CDF.

**Certificate summary** (`lipschitz_certificate.py`, 100K grid points/piece,
mpmath 60-digit precision):

|Piece|Interval     |grid_max|L×h/2   |Certificate|
|-----|-------------|--------|--------|-----------|
|I0   |[0, 0.5]     |2.26    |0.000   |2.26       |
|I1   |[0.5, 1.5]   |1.14    |0.001   |1.14       |
|I2   |[1.5, 2.25]  |0.77    |0.001   |0.77       |
|I3   |[2.25, 3.0]  |1.76    |0.001   |1.76       |
|I4   |[3.0, 4.0]   |0.67    |0.001   |0.67       |
|I5   |[4.0, 5.0]   |0.77    |0.001   |0.77       |

Certified maximum real-valued approximation error: **2.27 ULP** (piece I0).

**Implementation overview.**

1. **Clamping** (`normal.rs:81-89`): Returns `Ok(0)` for x < -8*SCALE, `Ok(SCALE_I)`
   for x > 8*SCALE, `Ok(SCALE_I / 2)` for x = 0.
1. **Symmetry** (`normal.rs:91,112-116`): Works on `ax = x.abs()`; for x < 0, returns
   `SCALE_I - cdf_pos`.
1. **Interval selection** (`normal.rs:93-108`): Six degree-11 polynomial pieces on
   [0, 5] plus an asymptotic continued-fraction tail for (5, 8]:

   |Piece|Interval     |mid   |hw   |Coefficients  |
   |-----|-------------|------|-----|--------------|
   |I0   |[0, 0.5]     |0.25  |0.25 |`POLY_V2_I0`  |
   |I1   |[0.5, 1.5]   |1.0   |0.5  |`POLY_V2_I1`  |
   |I2   |[1.5, 2.25]  |1.875 |0.375|`POLY_V2_I2`  |
   |I3   |[2.25, 3.0]  |2.625 |0.375|`POLY_V2_I3`  |
   |I4   |[3.0, 4.0]   |3.5   |0.5  |`POLY_V2_I4`  |
   |I5   |[4.0, 5.0]   |4.5   |0.5  |`POLY_V2_I5`  |
   |Tail |(5.0, 8.0]   |—     |—    |`norm_cdf_tail`|

1. **Local variable** (`normal.rs:94`, via `poly_map_t_round` at `normal.rs:43-46`):
   `t = round((ax - mid) * SCALE_I / hw)`, mapping the interval to [-SCALE, SCALE]
   with rounding to nearest (sign-aware: adds `hw/2` for positive, subtracts for
   negative, before dividing by `hw`).
1. **Degree-11 rounding Horner** (`normal.rs:94`, via `horner_11_round` at
   `normal.rs:25-38`): evaluates
   `p(t) = c[0] + c[1]*(t/S) + c[2]*(t/S)^2 + ... + c[11]*(t/S)^11`
   using 11 calls to `fp_mul_i_round` (rounding multiply, <= 0.5 ULP per step).
1. **Asymptotic tail** (`normal.rs:107`, via `norm_cdf_tail` at `normal.rs:61-75`):
   For ax > 5*SCALE, computes `SCALE - phi(ax) * mills_ratio(ax)` where:
   - `phi(ax)` = `fp_mul_i_round(INV_SQRT_2PI, exp_fixed_i(-x^2/2))` (Prop 12 + Prop 8)
   - `mills_ratio` = 6-level continued fraction via `fp_div_i_round` (`normal.rs:51-57`)
   - Result clamped to [0, SCALE_I]
1. **Clamp and symmetry** (`normal.rs:110-116`): Clamp `cdf_pos` to [0, SCALE_I],
   then apply `SCALE_I - cdf_pos` for x < 0.

**Constants:** The polynomial coefficients (6 sets of 12, at `constants.rs:590-624`)
are degree-11 minimax (Chebyshev) approximations to Φ on each interval, with
boundary-constrained, coordinate-descent optimized coefficients.

*Proof.* The total error is the sum of the Lipschitz-certified approximation error
and the analytically bounded implementation overhead.

**Part 1: Approximation error (Lipschitz-certified).**

The Lipschitz certificate (`lipschitz_certificate.py`) evaluates the real-valued
polynomial (exact mpmath arithmetic, 60-digit precision) against `mpmath.ncdf` at
100K grid points per piece. The certificate bounds the *continuous* maximum via:
`cert = grid_max + L × h/2`, where L is a rigorous Lipschitz constant for the error
function, itself bounded via the three-level analytical chain (see §Lipschitz
Certificate Method).

The certified maximum across all 6 pieces is **2.27 ULP** (piece I0). This covers
the minimax polynomial approximation error and coefficient quantization error
(both are captured by evaluating the integer-coefficient polynomial in exact
arithmetic against the true CDF).

**Part 2: Implementation overhead (analytically bounded).**

*(A) Horner evaluation error.* `horner_11_round` (`normal.rs:25-38`) performs 11
multiply-add steps. Each `fp_mul_i_round(r, t)` rounds `(r * t) / SCALE` to nearest,
introducing error δ_k with |δ_k| <= 0.5 ULP. The addition of c[k] is exact.

Unrolling the Horner recurrence gives accumulated error:

```
|e_0| <= sum_{k=0}^{10} 0.5 * |t/S|^k <= 0.5 * 11 = 5.5 ULP.
```

**Horner evaluation error <= 5.5 ULP.**

*(B) Local variable computation error.* `poly_map_t_round` (`normal.rs:43-46`)
computes `t = round((ax - mid) * SCALE_I / hw)`. The subtraction and `checked_mul`
are exact. The rounding division introduces <= 0.5 ULP in t.

This error propagates through the polynomial with attenuation by the derivative:
`|dp/dt| <= phi(x) * hw <= 0.399 * 0.5 * SCALE = 2e11` (worst case, piece I1),
so 0.5 ULP error in t contributes `0.5 * 2e11 / SCALE = 0.1 ULP`.
**poly_map_t_round error contributes < 0.5 ULP.**

*(C) Symmetry and clamping.* Exact. **0 ULP.**

**Implementation overhead:**

|Source                           |Bound       |
|---------------------------------|------------|
|(A) Rounding Horner evaluation   |<= 5.5 ULP  |
|(B) poly_map_t_round             |< 0.5 ULP   |
|(C) Symmetry/clamping            |0 ULP       |
|**Total overhead**               |**< 6 ULP** |

**Part 3: Total bound (polynomial path).**

```
Total = certified approximation (2.27) + implementation overhead (< 6) < 8.27 ULP.
```

Rounded to a clean bound: **<= 9 ULP** for the polynomial path (pieces I0–I5).

**Part 4: Tail path (x > 5*SCALE).**

The tail computes `SCALE_I - phi(x) * R(x)` where R(x) is the Mills ratio
approximated by a 6-level continued fraction. At x = 5, the CF6 approximation
to the true Mills ratio has relative error < 1e-13. The `phi(x)` term uses
`exp_fixed_i` (Prop 8) and `INV_SQRT_2PI`, contributing the exp error bound.
For x > 5, phi(x) < 1.49e-6 * SCALE, so absolute errors in the tail product
are negligible relative to 1 ULP. The clamp to [0, SCALE_I] ensures safety.

**|f-hat(x) - Φ(x/SCALE) * SCALE| <= 9** for all x in [-8*SCALE, 8*SCALE]. □

*Remark (Empirical validation).* Max observed error = 4 ULP across 100K production
vectors (production suite: norm_cdf_poly max ULP = 4).

-----

### Proposition 7 (ln_fixed_i — fixed-point natural logarithm)

**Source:** `transcendental.rs:11-87`

**Classification:** Category B — analytical conservative bound with sub-ULP
correction chain.

**Statement.** For all x > 0 where the result fits in i128,

```
|ln_fixed_i(x) - ln(x / SCALE) * SCALE| <= 3 ULP.
```

Returns `Err(DomainError)` for x = 0.

**Implementation overview.** Table-assisted Remez-polynomial arctanh with 16-entry
split-constant lookup and combined sub-ULP correction.

1. **Domain guard** (`transcendental.rs:12-14`): Returns `Err(DomainError)` for x = 0.
1. **Primary range reduction** (`transcendental.rs:16-27`): Find integer k such that
   `m = x / 2^k` (or `x * 2^|k|`) lies in `[SCALE, 2*SCALE)`. Uses `checked_mul(2)`
   for the upward direction and `/ 2` for downward.
1. **Direct path (near x = 1)** (`transcendental.rs:34-51`): When
   `offset = m - SCALE < LN_TABLE_HALF_STEP` (= SCALE/32), avoids the table lookup
   and computes directly:
   - `t = round((m - SCALE) * SCALE / (m + SCALE))` — arctanh argument with rounding
   - `u = fp_mul_i_round(t, t)` — t squared
   - Degree-3 Remez polynomial: `p = W3*u + W2; p = p*u + W1; p = p*u + W0`
     via 3 calls to `fp_mul_i_round`
   - `series_result = fp_mul_i_round(2*t, p)` — the arctanh(t) approximation
   - Split LN2 correction: `round(k * LN2_LO / SCALE_I)` folded into result
   - Return: `series_result + k * LN2_I + ln2_correction`
1. **Table path (general)** (`transcendental.rs:53-86`): For m further from SCALE:
   - Index `j = min(offset / LN_TABLE_STEP, 15)` selects a table entry
   - Table midpoint: `m_j = SCALE + (2j + 1) * LN_TABLE_HALF_STEP`
   - Look up `ln_m_j = LN_TABLE_16[j]` (high part) and
     `ln_m_j_lo = LN_TABLE_LO_16[j]` (sub-ULP residual)
   - Compute local arctanh variable: `t = round((m - m_j) * SCALE / (m + m_j))`
   - Extract t's sub-ULP residual: `t_rem = (m - m_j)*SCALE - t*(m + m_j)` (exact),
     then `t_lo = t_rem * SCALE / (m + m_j)`
   - Same degree-3 Remez polynomial as direct path: `p(u) = W3*u^3 + W2*u^2 + W1*u + W0`
   - `series_result = fp_mul_i_round(2*t, p)`
   - **Combined sub-ULP correction**: `correction = round((ln_m_j_lo + k*LN2_LO + t_lo) / SCALE_I)`
   - Return: `series_result + ln_m_j + k * LN2_I + correction`

**Constants used** (from `constants.rs`):

- `LN2_I = 693_147_180_560` — error from exact `ln(2) * 10^12`: +0.055 ULP
- `LN2_LO = -54_690_582_768` — sub-ULP residual: `LN2_I * SCALE + LN2_LO ≈ ln(2) * SCALE^2`
- `LN_TABLE_STEP = SCALE / 16` — table step size (62.5e9)
- `LN_TABLE_HALF_STEP = SCALE / 32` — half step (31.25e9)
- `LN_TABLE_16[0..16]` — `round(ln(m_j / SCALE) * SCALE)` for 16 evenly spaced midpoints
- `LN_TABLE_LO_16[0..16]` — sub-ULP residuals for each table entry
- `LN_REMEZ_W0 = 1_000_000_000_000` (= SCALE, representing coefficient 1.0)
- `LN_REMEZ_W1 = 333_333_333_406` (≈ 1/3 × SCALE, Remez-optimized)
- `LN_REMEZ_W2 = 199_999_986_321` (≈ 1/5 × SCALE, Remez-optimized)
- `LN_REMEZ_W3 = 142_858_120_138` (≈ 1/7 × SCALE, Remez-optimized)

*Proof.* The algorithm computes `ln(m/S) = 2*arctanh((m - m_j)/(m + m_j)) + ln(m_j/S)`,
where the arctanh is approximated by `t * P(t^2)` with a degree-3 Remez polynomial.
The total error decomposes into five components.

*(A) Polynomial approximation error.* The Remez polynomial P(u) = W0 + W1*u + W2*u^2 + W3*u^3
approximates `arctanh(t)/t` on the interval `|t/SCALE| <= 1/32 ≈ 0.031` (table path)
or `|t/SCALE| <= 1/64 ≈ 0.016` (direct path). The 16-entry table ensures each
subinterval is narrow: `|t/SCALE| <= (step/2) / (2*SCALE + step/2) < 0.016`.

The approximation error of P relative to the true `arctanh(t)/t` is bounded by the
Remez exchange algorithm's guarantee. The first neglected odd term in the arctanh series
is `t^9/9`. At worst case `|t/SCALE| = 0.031`: `0.031^9 / 9 ≈ 3.2e-15`, which is
negligible (< 0.001 ULP). **Contribution: < 0.01 ULP.**

*(B) Horner evaluation rounding.* The polynomial `p = W3*u + W2; p = p*u + W1; p = p*u + W0`
involves 3 calls to `fp_mul_i_round`, each with error <= 0.5 ULP. With |u/SCALE| <= 0.001
(since u = t^2 and |t/SCALE| < 0.031), errors are strongly attenuated:

```
step 0: 0.5 ULP (in W3*u, attenuated by u/S ≈ 0.001 in subsequent steps)
step 1: 0.5 ULP (in p*u, attenuated by u/S)
step 2: 0.5 ULP (in p*u)
```

Total in p-space: < 0.5 + 0.5*0.001 + 0.5*0.001^2 ≈ **0.501 ULP**.

The final multiply `fp_mul_i_round(2*t, p)` adds <= 0.5 ULP and attenuates p-space
errors by `|2*t/SCALE| < 0.063`. So p-space error contributes `0.501 * 0.063 = 0.032`
to the output, plus the fresh 0.5 ULP from the final multiply.
**Horner rounding contributes < 1.1 ULP to series_result.**

*(C) t computation and sub-ULP residual.* The arctanh argument is computed as
`t = round((m - m_j) * SCALE / (m + m_j))`. The numerator `(m - m_j) * SCALE` is
exact by **(R1)** (product fits in i128). The rounding division introduces <= 0.5 ULP
error in t.

The sub-ULP residual `t_lo = (p_val - t * t_den) * SCALE / t_den` captures the
division remainder at extended precision. This is folded into the combined correction
term, recovering most of the division error. After the correction, the effective
t error is **< 0.1 ULP** (limited by the single rounding of the correction term).

*(D) Table and LN2 constant errors.* Each table entry `LN_TABLE_16[j]` has
rounding error <= 0.5 ULP from the true `ln(m_j/SCALE) * SCALE`. The sub-ULP
residual `LN_TABLE_LO_16[j]` captures this error at extended precision.

The combined sub-ULP correction folds three residuals into one rounding:
`correction = round((ln_m_j_lo + k * LN2_LO + t_lo) / SCALE_I)`.

This single rounding introduces <= 0.5 ULP of error. The three residual terms
collectively cancel the table, LN2, and t rounding errors to sub-ULP precision,
leaving only the final rounding of their sum.

For `LN2_I`, the per-unit error is +0.055 ULP. With |k| <= 88 (for x near u128::MAX):
`88 * 0.055 = 4.84 ULP` before correction. The `LN2_LO` correction reduces this to
**< 0.5 ULP** (the residual rounding).

**Combined D contribution: < 1.0 ULP** (table residual rounding + LN2 residual
rounding + their cross-term, all folded into one rounding operation).

*(E) u computation error.* `u = fp_mul_i_round(t, t)` introduces <= 0.5 ULP.
Since u only appears in the polynomial evaluation where it is multiplied by
coefficients ~ 0.14-1.0 and the result is further attenuated by 2t/SCALE < 0.063,
the u error contribution to the output is `0.5 * 1.0 * 0.063 < 0.04 ULP`.
**Negligible.**

**Combined bound:**

|Error source                        |Contribution to result|
|------------------------------------|----------------------|
|(A) Polynomial approximation        |< 0.01 ULP            |
|(B) Horner rounding + final multiply|< 1.1 ULP             |
|(C) t computation (after correction)|< 0.1 ULP             |
|(D) Table + LN2 (after correction)  |< 1.0 ULP             |
|(E) u computation                   |< 0.04 ULP            |
|**Total (worst case)**              |**< 2.3 ULP**         |

The analytical bound of < 2.3 ULP covers the implementation overhead per subinterval.
The polynomial approximation error (A) is negligible (< 0.01 ULP) because the
16-entry table narrows each subinterval to |t/SCALE| < 0.031, making the degree-3
Remez polynomial extremely accurate. The sub-ULP correction chain (C, D) recovers
nearly all rounding error from t computation and constant quantization. The direct
path (near x = 1) has a slightly different error structure but the same 2.3 ULP bound
applies (narrower subinterval, same polynomial).

Rounded to a clean bound: **|f-hat(x) - ln(x/SCALE) * SCALE| <= 3** for all x > 0. □

*Remark (Empirical validation).* Max observed error = 3 ULP across 100K production
vectors (production suite: ln_fixed_i max ULP = 3, median 1).

-----

### Proposition 8 (exp_fixed_i — fixed-point exponential)

**Source:** `transcendental.rs:93-141`

**Classification:** Category B — analytical conservative bound. Remez rational
approximation (FreeBSD msun style) with split LN2 correction.

**Statement.** For all x in [-40*SCALE, 40*SCALE],

```
|exp_fixed_i(x) - exp(x/SCALE)*SCALE| <= C * 2^k
```

where k = floor(x / LN2_I) (after correction), C <= 4 ULP in the pre-reconstruction
sum. The relative error is bounded independently of k:

```
|f-hat(x) - exp(x/SCALE)*SCALE| / |exp(x/SCALE)*SCALE| < 4*sqrt(2) / SCALE ≈ 5.7 * 10^-12.
```

Returns `Ok(0)` for x <= -40*SCALE, `Err(Overflow)` for x >= 40*SCALE.

**Implementation overview.** Remez rational formula with 7 rounding operations
(vs 24 in the previous Taylor series).

1. **Domain guards** (`transcendental.rs:94-98`): Returns `Ok(0)` for x <= -40*SCALE,
   `Err(Overflow)` for x >= 40*SCALE, `Ok(SCALE_I)` for x = 0.
1. **Range reduction with split LN2** (`transcendental.rs:103-117`):
   - `k = x / LN2_I` — initial octave estimate
   - `ln2_correction = round(k * LN2_LO / SCALE_I)` — sub-ULP residual correction.
     `LN2_I` overshoots true `ln(2)*SCALE` (since `LN2_LO < 0`), so `k*LN2_I` is too
     large and `r = x - k*LN2_I` is too small. The correction adds back the deficit.
   - `r = x - k * LN2_I - ln2_correction` — reduced argument
   - Boundary adjustment: if `|r| > LN2_I/2`, adjust k by ±1 and r by ∓LN2_I
   - After reduction: `|r/SCALE| <= ln(2)/2 ≈ 0.347`
1. **Remez rational formula** (`transcendental.rs:120-133`):
   - `xx = fp_mul_i_round(r, r)` — r squared (1 rounding op)
   - Degree-4 Horner in xx:
     `poly = P1 + xx*(P2 + xx*(P3 + xx*(P4 + xx*P5)))` (4 rounding ops via
     `fp_mul_i_round`)
   - `c = r - fp_mul_i_round(poly, xx)` — correction term (1 rounding op)
   - `rc = fp_mul_i_round(r, c)` — (1 rounding op)
   - `sum = SCALE_I + r + fp_div_i(rc, 2*SCALE_I - c)` — rational combination
     (1 division)
1. **Reconstruction** (`transcendental.rs:136-140`): `sum << k` for k >= 0,
   `sum >> |k|` for k < 0. Uses `checked_shl` with `Err(Overflow)` on overflow.

**Constants used** (from `constants.rs`):

- `LN2_I = 693_147_180_560` — round(ln(2) × SCALE), error +0.055 ULP
- `LN2_LO = -54_690_582_768` — sub-ULP residual of LN2_I
- `EXP_REMEZ_P1 = 166_666_666_667` (≈ 1/6 × SCALE)
- `EXP_REMEZ_P2 = -2_777_777_778` (≈ -1/360 × SCALE)
- `EXP_REMEZ_P3 = 66_137_563`
- `EXP_REMEZ_P4 = -1_653_390`
- `EXP_REMEZ_P5 = 41_381`

*Proof.* The error decomposes into four components.

*(A) Range reduction error.* The split LN2 approach computes
`r = x - k * LN2_I - round(k * LN2_LO / SCALE_I)`. The residual after correction
is bounded by the rounding of `k * LN2_LO / SCALE_I`, which is <= 0.5 ULP.
This is a factor of ~|k| better than the uncorrected approach (which would have
error `|k| * 0.055` ULP). **Contribution: <= 0.5 ULP** in r.

Since `d(exp(r))/dr = exp(r)` and the sum ≈ SCALE, a 0.5 ULP error in r propagates
as `0.5 * exp(r_real) / SCALE` ≈ 0.5 ULP (since `exp(r_real) ∈ [1/√2, √2]`).
**Range reduction contributes <= 0.7 ULP to the sum.**

*(B) Rational formula rounding.* The formula involves 7 rounding operations:

1. `xx = fp_mul_i_round(r, r)` — 0.5 ULP
2. `fp_mul_i_round(xx, P5)` — 0.5 ULP (attenuated by subsequent multiplications)
3. `fp_mul_i_round(xx, poly)` — 0.5 ULP (×3 more of these)
4-5. Two more Horner steps — 0.5 ULP each
6. `fp_mul_i_round(poly, xx)` for c — 0.5 ULP
7. `fp_mul_i_round(r, c)` for rc — 0.5 ULP

The Horner steps (2-5) compute `poly ≈ P1 ≈ 0.167 * SCALE` (the higher-order terms
are negligible for |r/SCALE| < 0.35). The polynomial error is attenuated by the
subsequent `poly * xx` multiplication: `|xx/SCALE| <= 0.347^2 = 0.120`, so Horner
errors contribute `< 2.0 * 0.120 = 0.24 ULP` to c.

The `xx` error (0.5 ULP) propagates through `c = r - poly*xx` as
`0.5 * |poly/SCALE| = 0.5 * 0.167 = 0.083 ULP`.

The `rc = r * c` and `rc / (2*SCALE - c)` steps: since `|c/SCALE| < 0.06` and
`|rc/SCALE| < 0.02`, the division error from `fp_div_i` is negligible relative to
the sum (which is ≈ SCALE).

**Total rounding in sum: < 1.5 ULP.**

*(C) Approximation error.* The Remez rational formula `1 + r + r*c/(2-c)` with
`c = r - P(xx)*xx` approximates `exp(r)` with error bounded by the Remez exchange
algorithm. For |r/SCALE| <= 0.347, the approximation error of the degree-9 rational
form is < 10^-14 relative, i.e. < 0.01 ULP at SCALE. **Negligible.**

*(D) Reconstruction amplification.* The bit shift `sum << k` multiplies both result
and error by 2^k. The relative error is invariant under this scaling.

**Combined bound:**

|Error source                |Contribution to sum|
|----------------------------|-------------------|
|(A) Range reduction (split) |<= 0.7 ULP         |
|(B) Rational formula rounding|< 1.5 ULP         |
|(C) Approximation error     |< 0.01 ULP         |
|**Total pre-reconstruction**|**< 2.3 ULP**      |

Conservative clean bound: C = 4.

**Absolute error:** <= 4 × 2^k ULP. **Relative error:** < 4√2 / SCALE ≈ 5.7 × 10^-12.

For |x| <= 20*SCALE: |k| <= 28, absolute bound <= 1.1 × 10^9.
For |x| <= 40*SCALE: |k| <= 57, absolute bound <= 5.8 × 10^17.
The relative bound (5.7 × 10^-12) holds across the full ±40×SCALE range. □

*Remark (Empirical validation).* Max observed = 473M ULP at i128 boundary (consistent
with 4 × 2^k amplification); max 1 ULP in financial domain (|x| < 20*SCALE).
Production suite: 100K vectors.

*Remark (Domain guards).* The code returns `Ok(0)` for x <= -40*SCALE (underflow
to zero) and `Err(Overflow)` for x >= 40*SCALE. These are documented boundary
behaviours; the high-side overflow is reported through `Result`.

-----

### Proposition 9 (sin_core / cos_core — fixed-point trigonometric kernels)

**Source:** `trig.rs:4-25`

**Classification:** Category B — analytical conservative bound. The minimax
approximation error is rigorously certified via two-level Lipschitz analysis
(`trig_lipschitz_certificate.py`). The Lipschitz certificate
uses the same three-level rigour chain as the CDF certificates (see Appendix):
an analytical bound on the third derivative of the error function certifies the
second derivative from a grid, which certifies the first derivative from a grid,
which certifies the error function itself.

Both evaluate minimax polynomial approximations on the reduced domain
|x| <= π/4 × SCALE (≈ 785,398,163,397).

#### 9a. sin_core

**Statement.** For |x| <= π/4 × SCALE,

```
|sin_core(x) - sin(x/SCALE)*SCALE| < 4 ULP.
```

**Implementation** (`trig.rs:4-13`):

```
t = fp_mul_i(x, x)                        // t = x^2/SCALE (truncated, Lemma 1)
r = horner_5(SIN_C11..SIN_C1, t)          // P(t) ≈ sin(x)/x * SCALE
result = fp_mul_i(r, x)                   // sin(x) * SCALE (Lemma 1)
```

The Horner chain evaluates a degree-5 polynomial in `u = x^2/SCALE`:
`P(u) = SIN_C1 + SIN_C3*u/S + SIN_C5*(u/S)^2 + ... + SIN_C11*(u/S)^5`
with 5 fp_mul_i multiplications (by t), then one final fp_mul_i (by x).

**Coefficients** (`constants.rs:336-341`): Minimax-optimized for sin(x)/x on
[-π/4, π/4], not truncated Taylor.

*Proof.* The key structural observation is that P(u) computes `sin(x)/x × SCALE`,
and the final `fp_mul_i(r, x)` produces `r × x / SCALE`. Since `fp_mul_i` computes
`P × x / SCALE`, an error ε_P in P propagates to the output as `ε_P × |x| / SCALE`.
All errors in P(u) are therefore **attenuated by |x/SCALE| <= π/4 ≈ 0.785** in the
output.

**Errors in P(u)** (the Horner output, before the final multiply):

1. **Approximation error** (Lipschitz-certified): The certificate evaluates the
   real-valued polynomial against mpmath sin(x)/x at 200,000 grid points, then
   bounds the continuous maximum via the Lipschitz constant of the error function.
   **Certified: 0.097** (grid max 0.096, L×h/2 < 0.001).
1. **Horner truncation** (integer rounding in 5 fp_mul_i steps, each < 1 ULP by
   Lemma 1): The j-th error propagates through the remaining outer multiplications,
   attenuated by `(|t|/S)^j` where `|t/S| <= (π/4)^2 = 0.617`. Analytically bounded:
   `sum_{k=0}^{4} 0.617^k = ` **2.38**.
1. **t-error propagation:** t = fp_mul_i(x, x) has truncation error |ε_t| < 1
   (Lemma 1). This propagates through P as |dP/dt| × |ε_t|. Since
   |dP/dt| ≈ 1/6, the contribution is < **0.17**.

**Total in P-space:** 0.10 + 2.38 + 0.17 = **2.65**.

**Output error:** P-space total × |x/SCALE|: 2.65 × 0.785 = **2.08**. Final
fp_mul_i(r, x) truncation (Lemma 1): < **1**. Total: < **3.08**.

**|f-hat(x) - sin(x/SCALE)*SCALE| < 4.** □

#### 9b. cos_core

**Statement.** For |x| <= π/4 × SCALE,

```
|cos_core(x) - cos(x/SCALE)*SCALE| < 4 ULP.
```

**Implementation** (`trig.rs:16-25`):

```
t = fp_mul_i(x, x)                        // t = x^2/SCALE (Lemma 1)
r = horner_5(COS_C10..COS_C0, t)          // Q(t) ≈ cos(x) * SCALE
```

Same structure as sin_core but no final multiplication by x (cosine is even).
**There is no attenuation** — errors in Q(t) pass through to the output directly.

**Coefficients** (`constants.rs:344-349`): Minimax-optimized for cos(x) on [-π/4, π/4].

*Proof.* All errors are in output units (no attenuation).

1. **Approximation error** (Lipschitz-certified):
   **Certified: 0.162** (grid max 0.162, L×h/2 < 0.001).
1. **Horner truncation:** `sum_{k=0}^{4} 0.617^k = ` **2.38** (same calculation
   as sin_core, each step < 1 ULP by Lemma 1).
1. **t-error propagation:** |dQ/dt| × 1. Since Q(t) ≈ SCALE × (1 - t/(2·SCALE) + …),
   |dQ/dt| ≈ 1/2 (largest at t = 0). Contribution: < **0.50**.

**Total:** 0.16 + 2.38 + 0.50 = **3.04**.

**|f-hat(x) - cos(x/SCALE)*SCALE| < 4.** □

**Certificate summary (both kernels):**

|Component                  |sin_core (P-space)|cos_core (output)|
|---------------------------|------------------|-----------------|
|Approximation (certified)  |0.097             |0.162            |
|Horner truncation (analyt.)|2.377             |2.377            |
|t-error propagation        |0.17              |0.50             |
|**P-space / output total** |**2.64**          |**3.04**         |
|Output attenuation (×0.785)|2.08              |—                |
|Final fp_mul_i             |< 1               |—                |
|**Final output total**     |**< 3.08**        |**< 3.04**       |

Both bounds are comfortably under 4.

*Remark (Empirical validation).* The i64 variants (`sin6` / `cos6`) report max
observed = 3 each. Direct i128-scale benchmark
data for `sin_core` / `cos_core` is not currently available, but the certified
bounds (< 4) are consistent with the i64 observations.

-----

### Proposition 10 (norm_cdf_poly_hp — high-precision normal CDF)

**Source:** `hp.rs:419-476`

**Classification:** Category B — Lipschitz-certified approximation error plus
analytical implementation overhead.

**Statement.** For all x in [-8*SCALE_HP, 8*SCALE_HP],

```
|norm_cdf_poly_hp(x) - Phi(x/SCALE_HP)*SCALE_HP| <= 12 ULP
```

where Φ is the standard normal CDF and SCALE_HP = 10^15.

**Certificate summary** (`lipschitz_certificate.py`, 100K grid points/piece,
mpmath 60-digit precision):

|Piece |Deg|Interval     |grid_max|L×h/2 |Certificate|
|------|---|-------------|--------|------|-----------|
|HP_I0 |13 |[0, 0.5]     |1.10    |0.136 |1.24       |
|HP_I1 |13 |[0.5, 1.5]   |2.53    |0.930 |3.46       |
|HP_I2A|15 |[1.5, 2.25]  |2.39    |0.335 |2.73       |
|HP_I2B|15 |[2.25, 3.0]  |1.17    |0.282 |1.45       |
|HP_I3A|17 |[3.0, 4.0]   |1.01    |0.544 |1.56       |
|HP_I3B|17 |[4.0, 5.0]   |2.49    |0.501 |2.99       |

Certified maximum real-valued approximation error: **3.46 ULP** (piece HP_I1).

**Implementation overview.** Same architecture as Proposition 6 but at SCALE_HP,
with 6 polynomial pieces (higher degrees) plus a Mills ratio tail for x > 5:

|Piece|Interval     |Degree|Horner function|Coefficients       |
|-----|-------------|------|---------------|-------------------|
|I0   |[0, 0.5]     |13    |`horner_hp_13` |`POLY_HP_V2_I0`    |
|I1   |[0.5, 1.5]   |13    |`horner_hp_13` |`POLY_HP_V2_I1`    |
|I2A  |[1.5, 2.25]  |15    |`horner_hp_15` |`POLY_HP_V2_I2A`   |
|I2B  |[2.25, 3.0]  |15    |`horner_hp_15` |`POLY_HP_V2_I2B`   |
|I3A  |[3.0, 4.0]   |17    |`horner_hp_17` |`POLY_HP_V2_I3A`   |
|I3B  |[4.0, 5.0]   |17    |`horner_hp_17` |`POLY_HP_V2_I3B`   |
|Tail |(5.0, 8.0]   |17+CF8|PDF × Mills    |`POLY_HP_I4_PDF`   |

1. **Clamping** (`hp.rs:420-428`): Returns `Ok(0)` for x < -8*SCALE_HP,
   `Ok(SCALE_HP)` for x > 8*SCALE_HP, `Ok(SCALE_HP / 2)` for x = 0.
1. **Symmetry** (`hp.rs:430,471-475`): Works on `ax = x.abs()`; for x < 0,
   returns `SCALE_HP - cdf_pos`.
1. **Interval selection** (`hp.rs:432-467`): Seven branches (6 polynomial + 1 tail).
1. **Local variable** (via `poly_map_t_hp` at `hp.rs:480-483`):
   `t = (ax - mid) * SCALE_HP / hw` — truncation division mapping to [-SCALE_HP, SCALE_HP].
1. **Horner evaluation**: Degree-13/15/17 Horner chains using `fp_mul_hp_i`
   (Proposition 2b, rounding to nearest, <= 0.5 ULP per multiplication).
1. **Tail path** (`hp.rs:463-466`): For ax > 5*SCALE_HP:
   - PDF via degree-17 Horner on mapped coordinate `t` using `POLY_HP_I4_PDF`,
     clamped to non-negative
   - Mills ratio via 8-deep continued fraction (`mills_ratio_cf8_hp` at
     `hp.rs:407-412`): 8 `fp_div_hp_safe` iterations plus 1 final division (9 total)
   - `tail = fp_mul_hp_i(pdf, mills)`, result = `SCALE_HP - tail`
1. **Clamp and symmetry** (`hp.rs:469-475`): Clamp `cdf_pos` to [0, SCALE_HP].

*Proof (polynomial pieces I0–I3B).*

**Part 1: Approximation error (Lipschitz-certified).**

The Lipschitz certificate proves the real-valued polynomial error (exact arithmetic
vs true CDF) is **<= 3.46 ULP** across all 6 pieces (see certificate table above).
This covers approximation + quantization error.

**Part 2: Implementation overhead (analytically bounded).**

*(A) Horner evaluation error.* Each `fp_mul_hp_i` introduces <= 0.5 ULP rounding
error (Proposition 2b). For degree N with |t/S| <= 1:

```
Horner error <= 0.5 * N ULP.
```

Worst case (degree 17, pieces I3A/I3B): <= 0.5 × 17 = 8.5 ULP.

*(B) poly_map_t_hp.* `(ax - mid) * SCALE_HP / hw` — the subtraction and `checked_mul`
are exact. The truncation division introduces < 1 ULP error in t. This propagates
through the polynomial attenuated by `|dp/dt| / SCALE_HP`, contributing < 0.5 ULP
to the output (same analysis as Proposition 6).

**Total polynomial path:** certified approximation (3.46) + Horner (8.5) + poly_map (0.5)
= **< 12.5 ULP**. Rounded: **<= 12 ULP**.

*Proof (tail path, |x| ∈ (5, 8]).*

*(A) PDF Horner.* Degree-17 Horner using `fp_mul_hp_i` contributes < 8.5 HP-scale
units. The `.max(0)` clamp ensures non-negative PDF.

*(B) Mills ratio CF8.* `mills_ratio_cf8_hp` (`hp.rs:407-412`) performs 8 iterations
of `fp_div_hp_safe(k * SCALE_HP, x + r)` for k = 8,7,...,1, plus a final
`fp_div_hp_safe(SCALE_HP, x + r)` — 9 total `fp_div_hp_safe` calls. Each contributes
< 1 HP-scale unit of error (Proposition 3). Since each CF step computes
`n / (x + prev)` where `x > 5*SCALE_HP` dominates, errors are attenuated by roughly
`n/x^2 < 0.04` per step. Total CF error: < 9 HP-scale units (conservative).

*(C) Final multiplication.* `fp_mul_hp_i(pdf, mills)` — <= 0.5 HP-scale units.

**Combined tail overhead:** < 8.5 + 9 + 0.5 = **18 HP-scale units** (analytical).

**Combined bound.** The polynomial path dominates: **<= 12 ULP** (certified + analytical).
The tail path is bounded analytically at < 18 ULP.

**|f-hat(x) - Φ(x/SCALE_HP) * SCALE_HP| <= 12** for polynomial pieces,
**<= 18** for tail, for all x in [-8*SCALE_HP, 8*SCALE_HP]. □

*Remark (Tail precision caveat).* The bounds above are *absolute* error bounds on
Φ(x). In the far tail (|x| > 5), Φ(x) is extremely close to 0 or 1, so the
*relative* error in the tail probability `1 - Φ(x)` can be much larger.

*Remark (Empirical validation).* Max observed error = 5 ULP across 100K production
vectors (production suite: norm_cdf_poly_hp max ULP = 5).

-----

### Proposition 11 (pow_fixed_hp — fixed-point exponentiation)

**Source:** `hp.rs:226-308`

**Classification:** Category B — analytical conservative bound via component-wise
error propagation through the ln → mul → exp chain, with a split path for large
exponents. Each component uses HP-scale (10^15) arithmetic internally, with
1e12 I/O.

**Statement.** For the general-case path (excluding special cases that are exact),
with base in `[0.01*SCALE, 100*SCALE]` and exponent in `[0, 20*SCALE]`:

- Relative error < 3.2 × 10^-13 across the full stated domain (the primary
  invariant).
- Absolute error <= 5 at standard scale output, for outputs <= 14 × SCALE
  (corollary).

**Special cases** (`hp.rs:227-247`): Returns exact results for 0^0 (DomainError),
x^0 = SCALE, 0^y = 0, x^1 = x, 1^y = SCALE, x^2 = fp_mul(x,x), x^0.5 = fp_sqrt(x).

**Implementation overview.** Two paths depending on `|product|`:

**Fast path** (`hp.rs:249-258`): When `|exponent × ln(base)| < 39 × SCALE_HP`:

```
base_hp  = upscale_std_to_hp(base)        // min(base, i128::MAX/1000) * 1000
exp_hp   = upscale_std_to_hp(exponent)    // min(exp, i128::MAX/1000) * 1000
ln_base  = ln_fixed_hp(base_hp)           // compensated Remez at HP (see below)
product  = fp_mul_hp_i(exp_hp, ln_base)   // exponent × ln(base) at HP (Prop. 2b)
result_hp = exp_fixed_hp(product)         // Remez rational exp at HP (see below)
output   = downscale_hp_to_std(result_hp) // (x + 500) / 1000
```

**Split path** (`hp.rs:260-307`): When `|product| >= 39 × SCALE_HP`, the product
exceeds `exp_fixed_hp`'s comfortable range. The exponent is decomposed into integer
and fractional parts:

```
n = exponent / SCALE                     // integer part
frac_std = exponent % SCALE              // fractional remainder

// Integer part: repeated squaring via checked_mul_div_u (exact)
int_result = base^n via squaring loop

// Fractional part: HP exp∘ln (same as fast path)
frac_product = fp_mul_hp_i(frac_hp, ln_base)
frac_result  = downscale_hp_to_std(exp_fixed_hp(frac_product))

// Combine
output = checked_mul_div_u(int_result, frac_result, SCALE)
```

**Key HP helper functions:**

- `ln_fixed_hp` (`hp.rs:112-174`): Compensated Remez HP logarithm. Uses
  `horner_compensated_hp_dw` (DoubleWord-tracked polynomial evaluation) for
  sub-ULP precision propagation through the degree-9 Remez polynomial. Extracts
  sub-ULP residuals from t, the polynomial, and the t×P product, combining them
  into a single correction before rounding. Split LN2 correction via `LN2_HP` +
  `LN2_HP_LO`. Error: <= 2 ULP at HP scale.

- `exp_fixed_hp` (`hp.rs:181-216`): Remez rational HP exponential. Same structure
  as `exp_fixed_i` (Proposition 8) but at SCALE_HP with HP-specific coefficients
  (`EXP_REMEZ_HP_P1..P5`). Uses `fp_mul_hp_fast` and `fp_div_hp_safe`. Split LN2
  correction via `LN2_HP` + `LN2_HP_LO`. Error: <= 3 ULP at HP scale.

*Proof.*

*(A) upscale_std_to_hp.* Checks `x <= i128::MAX / 1000`, then computes
`x * 1000`. Exact within the stated domain by **(R1)**; returns `Err(Overflow)`
outside it. 0 error for `Ok` results.

*(B) ln_fixed_hp.* The compensated Remez approach with DoubleWord propagation
achieves <= 2 HP-scale units. The key improvements over `ln_fixed_i` (Proposition 7):
- `horner_compensated_hp_dw` tracks sub-ULP remainders from each `fp_mul_hp_fast`
  step via the `DoubleWord` type (`hi` + `lo/SCALE_HP`)
- The t computation extracts a sub-ULP residual `t_lo`
- Three sub-ULP correction sources (polynomial remainder, t remainder, LN2 remainder)
  are combined and rounded in a single step

*(C) fp_mul_hp_i(exp_hp, ln_base).* Rounding error <= 0.5 HP-scale units
(Proposition 2b). Propagated error from ln_base: amplified by
|exp_hp / SCALE_HP| = |exponent / SCALE| <= 20. With ln_fixed_hp error <= 2:
propagated ln error <= 20 × 2 = **40 HP-scale units** in the product.

*(D) exp_fixed_hp (fast path).* The Remez rational formula (same structure as
Proposition 8 but at HP scale) contributes <= 3 HP-scale units of its own rounding
error. The argument error of 40 HP-scale units propagates through exp as
`d(exp(r))/dr × 40 / SCALE_HP = exp(r_real) × 40 / SCALE_HP`. Since the relative
error is invariant under the 2^k reconstruction:

```
relative_error = (3/exp(r_real) + 40) / SCALE_HP
```

Maximised at exp(r_real) = 1/√2:
`(3√2 + 40) / SCALE_HP = 44.2 / 10^15 ≈ 4.4 × 10^-14`.

*(D') Split path error.* The integer part uses `checked_mul_div_u` (exact via U256
widening, Lemma 3). The fractional part uses the same HP exp∘ln chain with
`|frac_product| < 39 × SCALE_HP` (guaranteed since frac < SCALE). The final
`checked_mul_div_u(int_result, frac_result, SCALE)` is exact. Total error equals
the fractional HP chain error, which is bounded by the same relative error as the
fast path.

*(E) downscale_hp_to_std.* `(x + 500) / 1000` for positive values, 0 for
non-positive. Rounding division error <= 0.5 standard units.

**Combined bound (fast path).** At standard scale output O:

```
error_std = O × 44.2 / SCALE_HP + 0.5 = O × 4.4e-14 + 0.5
```

|Output (× SCALE)|HP chain error|+ downscale|Total|<= 5?|
|----------------|--------------|-----------|-----|-----|
|1               |0.04          |0.50       |0.54 |yes  |
|5               |0.22          |0.50       |0.72 |yes  |
|10              |0.44          |0.50       |0.94 |yes  |
|100             |4.42          |0.50       |4.92 |yes  |

**Absolute bound:** For outputs <= 100 × SCALE: **total <= 5 standard units**.

**Relative bound:** Across the full domain: **relative error < 4.4 × 10^-14**
(tighter than the stated < 3.2 × 10^-13, which used the old Taylor-based ln/exp). □

*Remark (Improvement from compensated ln).* The compensated Remez `ln_fixed_hp`
(2 ULP) vs the previous arctanh series (15 ULP) reduces the propagated ln error
from 300 to 40 HP-scale units, tightening the relative bound by ~7×.

*Remark (Empirical validation).* Max observed error = 1 ULP for outputs ≤ 100×SCALE;
21.5M ULP at extreme range (consistent with relative bound). Production suite: 100K vectors.

-----

### Proposition 12 (norm_pdf — standard normal PDF)

**Source:** `normal.rs:8-21`

**Statement.** For |x| <= 8*SCALE,

```
|norm_pdf(x) - phi(x/SCALE)*SCALE| <= 14 ULP,
```

where φ is the standard normal PDF.

**Implementation** (`normal.rs:8-21`):

```
x_sq          = fp_mul_i(x, x)             // x^2 / SCALE (Lemma 1)
neg_half_x_sq = -(x_sq / 2)               // -x^2 / (2*SCALE)
if neg_half_x_sq < -40 * SCALE_I:          // guard: exp would underflow
    return 0
exp_term      = exp_fixed_i(neg_half_x_sq) // exp(-x^2/2) * SCALE (Prop 8)
                match Ok(v) => v, Err(_) => 0  // unreachable after guard
result        = fp_mul_i(INV_SQRT_2PI, exp_term)  // phi(x) * SCALE (Lemma 1)
```

*Proof.*

*(A) x_sq = fp_mul_i(x, x).* Truncation error < 1 ULP by Lemma 1.

*(B) neg_half_x_sq = -(x_sq / 2).* The division by 2 truncates by **(R2)**, with
error < 1 ULP. Combined with step (A): the argument to exp has error <= 2 ULP.

*(C) exp_fixed_i(neg_half_x_sq).* The argument is always <= 0 (since x_sq >= 0).
Therefore the range reduction gives k <= 0, and the reconstruction is a right-shift
`sum >> |k|` which **attenuates** errors (divides by 2^|k|).

- Series rounding: < 7.3 ULP in sum (Proposition 8, refined analysis). After
  right-shift: < 7.3 / 2^|k| ULP. For k = 0 (x near 0): < 7.3 ULP. For
  |k| >= 1: < 3.7 ULP.
- Propagated error from the argument (2 ULP): the derivative of exp at the
  series level is exp(r) ≈ SCALE (at k = 0, the worst case). So 2 ULP in the
  argument contributes 2 × exp(r) / SCALE ≈ 2 ULP to sum. After attenuation:
  <= 2 ULP.

Total exp error: **< 12 ULP** (at k = 0, the worst case: 7.3 + 2 + 1.5 range
reduction = 10.8, rounded up with margin).

*(D) fp_mul_i(INV_SQRT_2PI, exp_term).* Truncation error < 1 ULP by Lemma 1.
`INV_SQRT_2PI = 398_942_280_401` has constant error 0.433 ULP from the exact value,
contributing `0.433 × exp_term / SCALE < 0.5 ULP` to the result.

**Combined:** < 12 + 1 + 0.5 = **13.5 ULP**. Rounded: **<= 14 ULP.** □

*Remark (Tightness).* The worst case (k = 0, i.e. x = 0) gives
φ(0) = 1/sqrt(2π) ≈ 0.3989 × SCALE. Here the full series rounding error applies.
For |x| > 1, the attenuation from the right-shift makes the exp error negligible,
and the total drops to ~2 ULP.

*Remark (Empirical validation).* The `norm_cdf_and_pdf` harness
(`normal.rs:94` — "PDF uses norm_pdf, ~2 ULP max") reports observed max ~2 ULP,
consistent with the attenuation effect dominating in practice.

-----

## Summary

|Function                    |Cat.|Bound                                                 |Observed / note        |Domain                             |
|----------------------------|----|------------------------------------------------------|-----------------------|-----------------------------------|
|fp_mul_i (Lemma 1)          |A   |< 1 (truncation remainder)                            |1                      |returns Err on overflow            |
|fp_sqrt Path B (Prop. 1)    |A   |< 1 (exact by 256-bit bisection)                      |1                      |x*SCALE overflows u128             |
|fp_sqrt Path A (Prop. 1)    |A/B |< 1 (Newton convergence arg.)                         |1                      |x*SCALE fits in u128               |
|checked_mul_div_i (Lemma 3) |A   |0 (exact)                                             |0                      |returns Err on overflow            |
|fp_div / fp_div_i (Lemma 2) |A   |exact floor (unsigned) / trunc (signed); remainder < 1 |1                     |b != 0, result fits                |
|fp_mul_hp_u / _i / _fast (Prop. 2)|A|<= 0.5 (rounding)                                  |0                      |no overflow                        |
|fp_div_hp_safe (Prop. 3)    |A   |exact trunc; remainder < 1                            |—                      |b != 0, returns Ok                 |
|checked_mul_div_floor/ceil_i (Prop. 4)|A|0 (exact)                                       |0                      |returns Err on overflow            |
|fp_div_floor / fp_div_ceil (Prop. 5)|A|0 (exact rounding)                                 |0-1                    |b != 0, result fits                |
|norm_cdf_poly (Prop. 6)     |B   |<= 9 (cert 2.27 + overhead 6)                          |4                      |\|x\| <= 8*SCALE                   |
|ln_fixed_i (Prop. 7)        |B   |<= 3 (analytical, sub-ULP correction)                  |3                      |x > 0, result fits i128            |
|exp_fixed_i (Prop. 8)       |B   |<= 4 * 2^k; rel < 5.7e-12                             |473M (boundary), 1 (financial)|\|x\| <= 40*SCALE            |
|sin_core (Prop. 9a)         |B   |< 4 (certified approx 0.10 + analyt. overhead)        |2 (sin_fixed, 100K)    |\|x\| <= pi/4 * SCALE              |
|cos_core (Prop. 9b)         |B   |< 4 (certified approx 0.16 + analyt. overhead)        |2 (cos_fixed, 100K)    |\|x\| <= pi/4 * SCALE              |
|norm_cdf_poly_hp (Prop. 10) |B   |<= 12 (cert 3.46 + overhead 8.5); tail <= 18           |5                      |\|x\| <= 8*SCALE_HP                |
|pow_fixed_hp (Prop. 11)     |B   |<= 5 (output<=100*S); rel < 4.4e-14                   |1 (moderate), 21.5M (extreme)  |base∈[0.01,100]*S, exp∈[0,20]*S    |
|norm_pdf (Prop. 12)         |B   |<= 14                                                 |2                      |\|x\| <= 8*SCALE                   |

**Category key:** A = exact integer proof. A/B = exact with analytical convergence
argument. B = analytical conservative bound (worst-case accumulation and/or Lipschitz
certificate).

‡ `fp_div_hp_safe` (Proposition 3) uses a widened fallback when the intermediate
`|r| * SCALE_HP` overflows i128. The result is exact for `Ok` values; unrepresentable
quotients return `Err(Overflow)`.

† The code accepts inputs up to ±40×SCALE, returns `Ok(0)` for underflow below
that range, and returns `Err(Overflow)` above that range. The error analysis in
Proposition 8 covers only ±20×SCALE. The relative
error bound (< 1.7 × 10^-11) is valid for the full ±40×SCALE range; the absolute
bound formula (12 × 2^k) is valid but gives much larger values at |x| > 20×SCALE.
See Proposition 8 domain discrepancy remark.

**Notes on the table.**

Category A results are exact integer computations; any nonzero benchmark observation
is the unavoidable truncation or rounding remainder. The HP multiplication variants
achieve <= 0.5 via rounding (vs < 1 for standard-scale truncation).

Category B bounds are conservative by construction — they sum worst-case contributions
that are unlikely to coincide. The gap between proved bounds and observed maxima
(e.g. 15 vs 7 for ln_fixed_i) is expected. Polynomial approximation bounds
(Proposition 6, Proposition 9, Proposition 10) are backed by Lipschitz certificates
that rigorously bound the continuous error between dense grid points (see Appendix
for the three-level rigour chain).

The `exp_fixed_i` bound (Proposition 8) scales with 2^k — this is inherent to
exponentiation. The relative error bound (1.7e-11) is the more meaningful
characterisation. Absolute ULP observations are not informative for large outputs
(they scale with 2^k); the benchmark confirms behaviour is consistent with the
relative bound.

The `sin_core` / `cos_core` observed maxima are from i64 benchmarks; direct i128
benchmark data is not yet available. The analytical bounds (Proposition 9) stand
independently of the empirical observations.

-----

## Appendix: Lipschitz certificates

All polynomial approximation results in this document have been rigorously certified
via Lipschitz analysis. No conditional or empirically calibrated bounds remain.

### Three-level rigour chain

The Lipschitz certificates use a three-level analytical chain to rigorously bound
the continuous maximum of the error function e(x) = p_int(x) - f(x)*SCALE
(where f is the target function) between dense grid points. The chain is:

1. **Level 3 — M3 = bound on |e'''(x)|:** Purely analytical, computed via
   triangle inequality on the polynomial's third derivative coefficients plus
   a global analytical bound on the target function's third derivative (e.g.
   |(1 - x²)φ(x)| for the CDF case). **No grid sampling at this level.**
   This is the anchor that makes the entire chain non-circular.
2. **Level 2 — M2 = bound on |e''(x)|:** The second derivative e''(x) is
   sampled on a 10K-point grid to obtain M2_grid = max over grid points.
   The analytical M3 bound from Level 3 certifies the continuous maximum
   between grid points: **M2 = M2_grid + M3 × h_m2 / 2**, where h_m2 is
   the Level 2 grid spacing. This is rigorous because |e''| has Lipschitz
   constant at most M3 (by the mean value theorem applied to e''').
3. **Level 1 — L = bound on |e'(x)|:** The first derivative e'(x) is sampled
   on a 100K-point grid to obtain L_grid = max over grid points. The rigorous
   M2 from Level 2 certifies: **L = L_grid + M2 × h / 2**, where h is the
   Level 1 grid spacing.
4. **Certificate — bound on |e(x)|:** The error function itself is sampled at
   100K+ points to obtain grid_max. The rigorous L from Level 1 certifies:
   **cert = grid_max + L × h / 2**.

Each level's grid-based maximum is elevated to a rigorous continuous bound by
the level above. The chain terminates at Level 3 with a pure analytical bound,
so there is no circular dependence on grid sampling.

**Why the naive approach failed for HP pieces:** For the HP CDF polynomials
(Proposition 10), the naive triangle-inequality bound on L (bounding |p'| and |φ|
separately) was too loose because the polynomial derivative and PDF nearly cancel
on the approximation interval. The two-level approach (pushing to M2 via e''
sampling, then to L) was necessary because the cancellation occurs in the first
derivative but not in the second — at the second-derivative level, the polynomial
and target function diverge enough for the bounds to be tight.

### CDF polynomials (Proposition 6, Proposition 10)

**Script:** `lipschitz_certificate.py`.

**Method:** As described in the three-level chain above. The CDF certificates
use the analytical bound |(1 - x²)φ(x)| for the third derivative of Φ(x)
at Level 3.

**Results:** norm_cdf_poly (Proposition 6) certified at 42.83 (claimed 43).
norm_cdf_poly_hp (Proposition 10, pieces I0–I3) certified at 7.27 (claimed 8).

### Trig polynomials (Proposition 9)

**Script:** `trig_lipschitz_certificate.py`.

**Method:** Two-level certificate separating the real polynomial approximation quality
(Level 1, Lipschitz-certified via the same three-level chain) from the integer Horner
truncation accumulation (Level 2, analytically bounded). Each function uses a single
polynomial on a single interval, making the certificate simpler than the multi-piece
CDF case.

Level 1 certifies the real-valued polynomial error (exact coefficients, no integer
rounding) against mpmath sin/cos at 200,000 grid points (60-digit precision).
Level 2 bounds the Horner truncation via the geometric sum
`sum_{k=0}^{4} (π/4)^{2k} = 2.38`.

**Results:**

|Component                  |sin_core|cos_core|
|---------------------------|--------|--------|
|Level 1 (certified approx) |0.097   |0.162   |
|Level 2 (Horner truncation)|2.377   |2.377   |
|Total certified (P-space)  |2.47    |2.54    |

The first version failed on sin_core because the sinc derivative formula had a
factor-of-SCALE error from misapplying the chain rule. The cos derivative (simpler
formula) was unaffected, which is why cos_core passed immediately.

-----

## Appendix: Out-of-domain behaviour

This table documents what each function does when called with inputs outside its
stated proof domain, based on source code inspection. **ERR** means the function
returns `Err(SolMathError::...)`. **UNDERFLOWS** means it returns zero because the
mathematical value is below the fixed-point resolution. **CLAMPS** means the input
or output is clamped to the documented boundary. **SILENT** means no guard exists
and the function computes a meaningless or incorrect result without any indication
of failure.

|Function          |Domain per result                      |Out-of-domain behaviour                                                                                                                                                                                                                                                                                                                    |Safety           |
|------------------|---------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------|
|`fp_mul_i`        |result fits i128 (Lemma 1)             |**ERR** — `checked_mul` fast path + `checked_mul_div_i` widened fallback; returns `Err(Overflow)` if the exact scaled result does not fit.                                                                                                                                                                                                 |Safe             |
|`ln_fixed_i`      |x > 0 (Prop. 7)                        |**ERR** — returns `Err(DomainError)` for x = 0. x < 0 impossible: parameter is u128.                                                                                                                                                                                                                                                       |Safe             |
|`exp_fixed_i`     |\|x\| < 40×SCALE (code guard) (Prop. 8)|**UNDERFLOWS/ERR** — returns `Ok(0)` for x ≤ −40×SCALE and `Err(Overflow)` for x ≥ 40×SCALE.                                                                                                                                                                                                                                             |Safe             |
|`pow_fixed_hp`    |base ∈ [0.01, 100]×S, exp ∈ [0, 20]×S (Prop. 11)|**UNDERFLOWS/ERR** — returns exact special cases such as x^0 and 0^y, returns `Ok(0)` when the fixed-point result underflows, and returns `Err(Overflow)` when the result is too large. Negative base impossible (u128).                                                                                                                   |Safe             |
|`sin_core`        |\|x\| ≤ π/4 × SCALE (Prop. 9a)         |**PANICS** (debug) / **SILENT** (release) — `debug_assert!` added; public `sin_fixed` performs octant reduction before calling.                                                                                                                                                                                                            |Mitigated (internal)|
|`cos_core`        |\|x\| ≤ π/4 × SCALE (Prop. 9b)         |**PANICS** (debug) / **SILENT** (release) — identical to sin_core. Public `cos_fixed` performs range reduction.                                                                                                                                                                                                                            |Mitigated (internal)|
|`norm_cdf_poly`   |\|x\| ≤ 8×SCALE (Prop. 6)              |**CLAMPS** — returns 0 for x < −8×SCALE, SCALE for x > 8×SCALE. Post-clamp on polynomial output to [0, SCALE].                                                                                                                                                                                                                             |Safe             |
|`norm_cdf_poly_hp`|\|x\| ≤ 8×SCALE_HP (Prop. 10)          |**CLAMPS** — returns 0 for x < −8×SCALE_HP, SCALE_HP for x > 8×SCALE_HP. Post-clamp on polynomial output to [0, SCALE_HP].                                                                                                                                                                                                                 |Safe             |
|`norm_pdf`        |\|x\| ≤ 8×SCALE (Prop. 12)             |**UNDERFLOWS** — returns 0 in extreme tails after an explicit `-x²/2 < -40×SCALE` guard or unreachable `exp_fixed_i` overflow.                                                                                                                                                                                                             |Safe             |
|`fp_div_hp_safe`  |scaled quotient fits i128 (Prop. 3)    |**ERR** — returns `Err(DivisionByZero)` for b = 0 and `Err(Overflow)` for unrepresentable quotients; uses U256 fallback for large remainder products.                                                                                                                                                                                       |Safe             |

**Notes:**

1. `sin_core` and `cos_core` are `pub(crate)` internals. They have `debug_assert!`
   guards; in release mode,
   out-of-range inputs are still caught by the public wrappers (`sin_fixed`,
   `cos_fixed`) which perform range reduction. `fp_div_hp_safe` uses widened
   arithmetic instead of producing silently incorrect results. `fp_mul_i` handles
   overflow via a widened fallback and returns `Err(Overflow)` if the final result
   does not fit. The error bounds in this document apply to the internal kernels
   on their stated domains.
1. No production public function is expected to panic on supported release builds.
   Out-of-domain behaviour is documented as `Err`, underflow-to-zero, or clamping.

-----

## Appendix: Monotonicity analysis for CDF functions

### The problem

For a CDF implementation to be safe for pricing, the computed output must be
non-decreasing: `norm_cdf_poly(x + δ) >= norm_cdf_poly(x)` for all δ > 0. A
monotonicity violation means P(S < K₁) > P(S < K₂) for K₁ < K₂ — a negative
implied density that creates phantom arbitrage signals.

### Analysis at single-ULP input resolution

At standard scale (SCALE = 10^12), adjacent integer inputs x and x+1 represent
real values differing by 10^-12. The true CDF increment between them is:

```
Φ((x+1)/S) × S - Φ(x/S) × S ≈ φ(x/S)
```

where φ is the standard normal PDF. This increment is at most φ(0) ≈ 0.399 — less
than 0.4 output ULP even at the peak of the PDF. The output is therefore a staircase
function that stays flat across many consecutive integer inputs and occasionally
increments by 1.

With the proved error bound of ±43 ULP (Proposition 6), adjacent inputs could in
theory produce outputs differing by up to 86 ULP in the "wrong" direction — if the
error function swung from +43 to -43 across a single input step. However, the
Lipschitz analysis constrains the rate of change of the error function. For the
worst piece (I3, [3.0, 5.0]):

```
L ≈ 2700 per real unit (derived from L × h/2 = 0.027, h = 2 × 10^-5)
Error change per integer ULP = L / SCALE ≈ 2.7 × 10^-9
```

The error function changes by less than 3 × 10^-9 per integer input step —
effectively constant. **Adjacent-input monotonicity violations are theoretically
possible but constrained to at most 1 ULP in magnitude** (from the staircase
rounding, not from error swings). These are inherent to any fixed-point CDF
implementation and are not specific to this library.

### Minimum separation for guaranteed monotonicity

The meaningful question is: at what input separation δ is monotonicity guaranteed?
This requires the true CDF increment to exceed twice the maximum possible error
swing over that interval:

```
φ(x/S) × δ > 2 × L × δ / S + 2 × (staircase rounding)
```

Since the Lipschitz error change over interval δ is `L × δ / S`, and L/S ≈ 2.7 × 10^-9,
this is negligible compared to φ(x/S) for all |x| ≤ 5×SCALE (where φ ≥ 1.49 × 10^-6).
The binding constraint is the staircase rounding: the output must increment by at least
2 to guarantee that rounding cannot reverse the ordering.

**Output increments by region:**

|Input region (σ)|\|x\|/SCALE|φ(x/S)|Output increment per input ULP|Steps for +2 output increment|
|----------------|-----------|------|------------------------------|-----------------------------|
|Near 0          |0          |0.399 |0.399                         |~5                           |
|1σ              |1          |0.242 |0.242                         |~9                           |
|2σ              |2          |0.054 |0.054                         |~37                          |
|3σ              |3          |0.0044|0.0044                        |~454                         |
|4σ              |4          |0.0001|0.0001                        |~17,000                      |
|5σ              |5          |1.5e-6|1.5e-6                        |~1,340,000                   |

At 3σ, monotonicity is guaranteed over input separations of ~454 ULP (= 4.54 × 10^-10
in real terms). At 5σ, the minimum separation grows to ~1.34 × 10^6 ULP
(= 1.34 × 10^-6 in real terms).

### Practical implications

For option pricing, the relevant granularity is the strike price increment, not
single-ULP fixed-point steps. A typical strike increment of $0.01 on a $100 underlying
corresponds to a standardised price ratio change of ~10^-4, which is ~10^8 input ULP
at SCALE. This vastly exceeds the monotonicity threshold at any σ level in the table
above.

**Conclusion:** Single-ULP monotonicity violations are inherent to fixed-point CDF
computation and cannot be eliminated at this precision without increasing SCALE.
However, monotonicity at any practically relevant pricing granularity is guaranteed
by the Lipschitz smoothness of the error function combined with the dominance of the
true CDF increment over the error variation at separations above ~10^3 ULP.

**Recommendation:** If strict monotonicity at single-ULP resolution is required for
a specific application (e.g., an AMM invariant), implement a simple post-hoc
monotonicity wrapper: `max(result, prev_result)` when evaluating the CDF at
increasing x values. This adds no error (it only clips spurious decreases of ≤ 1 ULP)
and guarantees monotone output for any sorted input sequence.

### HP scale

At HP scale (SCALE_HP = 10^15), the CDF increment between adjacent integer inputs is
φ(x/S_HP) × 10^-3, three orders of magnitude smaller than at standard scale. The
HP error bound of ≤ 8 (Proposition 10) means the staircase rounding concern applies
at even coarser granularity. However, the same analysis holds: at any practically
relevant input separation, the true CDF increment dominates the error variation.

-----

## Appendix: Compute unit budget (Solana)

CU measurements from Solana localnet production benchmark runs. Most medians come
from 50,000 on-chain vectors per function (`BENCH_CONCURRENCY=32`); avg/max values
come from the current benchmark tables where available. All values are Solana
compute units.

### Individual function CU cost

|Function         |Avg CU |Median CU|Max CU |Notes                                      |
|-----------------|-------|---------|-------|-------------------------------------------|
|fp_div           |625    |655      |690    |Current NUC arithmetic rerun               |
|fp_div_i         |652    |676      |724    |Current NUC arithmetic rerun               |
|checked_mul_div_i|883    |883      |3,807  |U256 path when product overflows u128      |
|fp_sqrt          |3,598  |3,007    |9,402  |Thin path vs overflow path                 |
|fp_mul_hp_i      |103    |103      |103    |Current optimized HP multiply              |
|ln_fixed_i       |4,562  |4,362    |5,207  |Table-assisted Remez                       |
|ln_fixed_hp      |19,175 |18,889   |19,764 |Compensated HP path                        |
|sin_fixed        |4,654  |4,029    |5,170  |Includes range reduction                   |
|cos_fixed        |4,578  |4,027    |5,181  |Includes range reduction                   |
|norm_cdf_poly    |6,844  |6,186    |15,333 |Varies by piece and tail path              |
|norm_cdf_poly_hp |24,234 |19,708   |40,691 |High variance: short-circuit vs polynomial |
|pow_fixed_hp     |27,408 |27,408   |35,000 |ln -> mul -> exp chain                     |

### Composite workflow CU cost

|Workflow / function       |Avg CU |Median CU|Max CU |
|--------------------------|-------|---------|-------|
|**bs_full (standard)**    |50,191 |50,015   |68,418 |
|**bs_full_hp (HP)**       |118,202|116,628  |164,961|
|barrier_option            |262,906|261,773  |385,456|
|pow_product_hp            |16,000 |—        |20,000 |
|nig_64                    |344,273|346,648  |386,010|
|implied_vol               |156,563|148,575  |395,940|
|bvn_cdf                   |128,614|129,700  |153,090|
|Phi2Table.eval            |943    |943      |943    |

### Budget assessment

**Standard Black-Scholes (bs_full):** 68K CU worst case — fits comfortably within
the default 200K CU transaction budget. Leaves ample room for surrounding program
logic (account deserialization, state updates, CPI calls).

**HP Black-Scholes (bs_full_hp):** 165K CU worst case in the benchmark set — fits
within 200K CU but with limited headroom (~35K CU remaining). For transactions that require additional
computation beyond pricing (e.g., settlement logic, multi-leg evaluation), a CU
budget increase to 400K should be requested. This is standard practice on Solana and
has minimal gas cost impact.

**NIG model (nig_64):** 386K CU worst case — requires a CU budget increase and
may need architectural consideration (e.g., splitting across instructions) if
combined with other computation.

**Implied volatility (implied_vol):** 396K CU worst case — requires a CU budget
increase. Iterative solvers are inherently variable; fast convergence is much
cheaper while edge cases can approach the upper bound.

**Barrier options:** 385K CU worst case — require a CU budget increase.

-----

## Appendix: Empirical validation summary

Production benchmark suite results (100K stratified vectors per function) superseding
the per-result empirical remarks above. Where adversarial
vectors were run (10K targeting known weak spots), those results are also included.

### Proved bounds vs observed maxima

|Function         |Proved bound (this document)     |Observed max (production)|Observed max (adversarial)|Ratio (prod)|Status             |
|-----------------|---------------------------------|-------------------------|--------------------------|------------|-------------------|
|fp_mul (unsigned)|≤ 1 (Lemma 1)                    |1                        |—                         |—           |✓ Consistent       |
|fp_mul_i (signed)|≤ 1 (Lemma 1)                    |0                        |—                         |—           |✓ Consistent       |
|fp_div / fp_div_i|≤ 1 (Lemma 2)                    |1 / 0                    |—                         |—           |✓ Consistent       |
|checked_mul_div_i|0 (Lemma 3)                      |0                        |—                         |—           |✓ Exact            |
|fp_sqrt          |≤ 1 (Prop. 1)                    |1                        |—                         |—           |✓ Consistent       |
|fp_mul_hp_i      |≤ 0.5 (Prop. 2)                  |0                        |—                         |—           |✓ Consistent       |
|fp_div_hp_safe   |≤ 1 (Prop. 3)                    |1                        |—                         |—           |✓ Consistent       |
|ln_fixed_i       |≤ 15 (Prop. 7)                   |7                        |6                         |2.1×        |✓ Conservative     |
|ln_fixed_hp      |≤ 15 (Prop. 7 analysis)          |8                        |—                         |1.9×        |✓ Conservative     |
|exp_fixed_i      |≤ 12 × 2^k (Prop. 8)            |1.5 × 10^9 (production)  |6.0 × 10^17 (adversarial) |—           |✓ See note 1       |
|pow_fixed_hp     |≤ 5 for output ≤ 14×S (Prop. 11) |112 × 10^6               |118.5 × 10^6              |—           |✓ See note 2       |
|norm_cdf_poly    |≤ 43 (Prop. 6)                   |42                       |9 (deep tails)            |1.02×       |✓ Tight            |
|norm_cdf_poly_hp |≤ 8 / ≤ 17 (Prop. 10)            |5                        |—                         |1.6× / 3.4× |✓ Conservative     |
|sin_fixed (full) |< 4 (Prop. 9a, core bound)       |2                        |—                         |2×          |✓ Conservative     |
|cos_fixed (full) |< 4 (Prop. 9b, core bound)       |2                        |—                         |2×          |✓ Conservative     |
|norm_pdf         |≤ 14 (Prop. 12)                  |2                        |—                         |7×          |✓ Very conservative|

**Note 1 (exp_fixed_i):** The large absolute errors are expected and consistent with
the 2^k amplification structure (Proposition 8). At the adversarial range [25, 39.5]×SCALE
(within the code's ±40×SCALE guard but beyond the ±20×SCALE analysis domain), the observed
maximum of 6.0 × 10^17 is below the theoretical bound of 12 × 2^57 ≈ 1.7 × 10^18.
The significant-figures metric is more informative: median 12.0 SF (production),
confirming the relative error bound of ~1.7 × 10^-11 holds across the domain.

**Note 2 (pow_fixed_hp):** The observed maximum of 112M vastly exceeds the proved
absolute bound of ≤ 5 for moderate outputs (Proposition 11). This is not a
contradiction: the ≤ 5 bound applies only when the output is ≤ 14×SCALE. Large
outputs amplify absolute error via the 2^k reconstruction in exp. The significant-
figures metric (median 14.3, worst 10.9) confirms the relative error bound of
< 3.2 × 10^-13 holds. For the adversarial suite (near-1 cancellation + overflow),
worst-case 10.9 SF is still excellent. The standard-scale `pow_fixed` (62.7G max,
10.0 worst SF) is significantly less precise — **HP should be the default path for
any precision-sensitive computation.**

### Black-Scholes end-to-end accuracy

**HP path (bs_full_hp):** Max error 4 ULP on call/put price at HP scale (10^15).
Cross-validated against QuantLib at 14.2 median significant figures for call/put
prices — this exceeds the precision of IEEE 754 double-precision arithmetic (15.9
decimal digits, but typically ~14-15 SF after a chain of transcendental evaluations).

|Greek     |Max abs error (HP)|Median SF vs QuantLib|
|----------|------------------|---------------------|
|Call price|4                 |14.2                 |
|Put price |4                 |14.2                 |
|Delta     |1                 |12.2                 |
|Gamma     |1                 |10.1                 |
|Vega      |9                 |14.3                 |
|Theta     |3                 |13.8                 |
|Rho       |22–23             |14.0–14.5            |

The Rho error (22–23 HP ULP) is the largest among the Greeks. This is expected:
Rho involves a multiplication by time-to-expiry T, which amplifies errors from the
CDF and exp chain. At 14.0 SF it remains well within practical requirements.

**Standard path (bs_full):** Max error 37K ULP at standard scale (10^12). This is
orders of magnitude worse than HP, driven primarily by the standard-scale exp/pow
chain. Median SF of 11.1–11.2 for call/put is adequate for many applications but
leaves less headroom. The adversarial suite shows 6K max for calls, 5K for puts.

**Recommendation:** Use `bs_full_hp` for all pricing paths where accuracy matters.
The CU cost premium (118K vs 50K average) is justified by the 3+ orders of magnitude
improvement in accuracy. Reserve `bs_full` for gas-constrained paths where
approximate pricing is acceptable (e.g., indicative quotes, UI display values).

### Distribution of errors

The production benchmark provides error percentile data showing that worst-case
bounds are rarely approached:

|Function        |P50|P95|P99|Max|
|----------------|---|---|---|---|
|ln_fixed_i      |1  |3  |4  |7  |
|norm_cdf_poly   |3  |12 |18 |42 |
|norm_cdf_poly_hp|0  |2  |2  |5  |
|bs_full_hp.call |0  |1  |1  |4  |
|bs_full_hp.delta|0  |0  |0  |1  |
|bs_full_hp.gamma|0  |0  |0  |1  |

For the HP Black-Scholes path, 73.2% of call prices are computed exactly (0 error)
and 99% are within 1 ULP. This is consistent with the proved bounds being
conservative worst-case accumulations that rarely coincide in practice.

-----

## mul_div_floor / mul_div_ceil (Lemma 4)

### Correctness proof

For `mul_div_floor(a, b, c)` where `a`, `b` are `u64` and `c` is `u64`, `c > 0`:

1. Let P = a × b, computed as `u128`. Since a ≤ 2^64 − 1 and b ≤ 2^64 − 1,
   P ≤ (2^64 − 1)² = 2^128 − 2^65 + 1 < 2^128. **No u128 overflow.**

2. `floor(P / c)` = `P / c` (Rust integer division truncates toward zero).
   Since P ≥ 0 and c > 0, this is equivalent to floor division. **Exact.**

3. `ceil(P / c)` = `(P + c − 1) / c`.
   P + c − 1 ≤ (2^128 − 2^65 + 1) + (2^64 − 1) − 1 = 2^128 − 2^64 < 2^128.
   **No u128 overflow in the ceiling numerator.**

4. The result is checked against `u64::MAX` before casting. If result > `u64::MAX`,
   `Err(Overflow)` is returned. **No truncation.**

### Error bounds

- `mul_div_floor`: result ≤ true value < result + 1. Error < 1 (integer rounding).
- `mul_div_ceil`: result − 1 < true value ≤ result. Error < 1 (integer rounding).
- When a × b is exactly divisible by c: floor == ceil == exact value. **Error = 0.**

These are **exact integer computations** — there is no approximation error. The only
"error" is the deliberate floor or ceiling rounding, which is the defined behaviour.

### Validation

Cross-validated against Python arbitrary-precision arithmetic across 76,070 vectors
(see `tests/reference/mul_div_vectors.json`). 0 mismatches.

-----

## Appendix: Benchmark formal bounds sync

The benchmark suite's `CLAIMED_BOUND_STD` for `norm_cdf_poly` has been updated from
53 to 43 (matching the Lipschitz-certified value in Proposition 6). The `ln_fixed_i`
bound in this document is ≤ 15 (conservative ceiling of 14.5); the benchmark uses 14.
Both are valid ceilings — the difference is a rounding convention.
