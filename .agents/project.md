# Project concepts

## `unitopia`

A Rust package that implements a generic newtype for unit-of-measurement + helpers.

Requirements:

* Must export a [Measure newtype](#measure-newtype)

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

## Measure newtype

A newtype that represents a physical measurement outcome.

Requirements:

* Must support a [custom unit](#custom-unit).
* Must support a generic storage type (e.g. `u32`, `u64`, `i32`, `i64` `f32`, `f64` or any other generic type that implements the necessary traits for arithmetic operations).
* Must disallow adding or subtracting measures of different physical quantities.
* Must allow multiplying or dividing measures of different physical quantities.
  * The result must have its own distinct type.
* Must support fractional units (e.g. millisecond).
* Must integrate with serialization frameworks, at least the following:
  * `serde`
  * `rkyv`
  * `bitcode`
  * `wincode`
* Must implement checked, wrapping, overflowing, saturating arithmetic operation traits from `num-traits`

Preferences:

* Should integrate with existing crates that provide similar types:
  * Examples:
    * `time` provides types that represent nanoseconds
    * `chrono` provides types that represent nanoseconds
    * `core` provides `Duration` that represents nanoseconds
  * Notes:
    * Some crates provide types that represent measurement deltas, not just measurements
      * Examples:
        * `core` provides `Duration`
    * Some types are measurements with a custom offset
      * Examples:
        * Timestamp is a measure of time with a custom offset (UNIX epoch)

Implementation ideas:

* May use `Mul` or `Div` generic types (e.g. `type Newton = Div<Mul<Kilogram, Meter>, Mul<Second, Second>>;`)
  * Notes:
    * This makes the units which are semantically equivalent syntactically different
      * Examples:
        * `Mul<Kilogram, Meter>` and `Mul<Meter, Kilogram>`
      * Solutions:
        * Provide an `invert_unit` method for measures with `Mul` unit
        * Switch to runtime check
        * Represent all units in a system with a single type whose generic parameters are unit powers
          * May use `typenum` crate
* May use `Mul` type only (represent `Div` as `Mul` with negative power) (e.g. `type Newton = Mul<Mul<Kilogram, Meter, 1, 1>, Mul<Second, Second, 1, 1>, 1, -1>;`)

Allowances:

* May not follow SI

Notes:

* The implementation ideas are just ideas. If you think this is a wrong idea, tell me about it and don't implement it.

## Custom unit

A unit that is not a part of SI.

Examples:

* Enzyme unit (e.g. FIP)
