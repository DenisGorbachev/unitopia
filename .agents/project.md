# Project concepts

## `unitopia`

A Rust package that implements a generic newtype for unit-of-measurement + helpers.

Design options:

* Add scaling
  * Tasks
    * Add `numerator: u64`, `denominator: u64`, `power: u64`
  * Evaluation
    * Notes:
      * Measure is currently #[repr(transparent)] + serde(transparent) and Deref to Value; adding fields breaks that ABI/serde model and the ergonomic Deref behavior.
      * Derived Eq/Ord/Hash would become structurally wrong unless you normalize; 1/2 *10^1 and 5/10* 10^1 should compare equal but won’t without canonicalization.
      * You’ll need a sign (negative values are real in many quantities), a non‑zero denominator (should be NonZeroU64), and a policy for
        normalization + overflow.
      * power: u64 only allows non‑negative exponents; very small values then require huge denominators (overflow risk). A signed exponent (i32/i64)
        or big integer coefficient is more robust.
      * Arithmetic becomes more complex: add/sub require aligning scale/denominator, mul/div grows numerator/denominator and can overflow unless you
        reduce or use big integers.
    * Advice:
      A less disruptive path: keep Measure<Quantity, Value> generic and add a dedicated value type (e.g., ScaledRational or Decimal) that internally
      uses (numerator, denominator, power) and implements Add/Sub/Mul/Div/Zero. Then Measure<Quantity, ScaledRational> gives you integer arithmetic,
      while existing users can still use f32/f64 or i64 as Value. If you want extreme range/precision, consider an existing crate like rust_decimal,
      or num_rational::Ratio with num_bigint::BigInt.
