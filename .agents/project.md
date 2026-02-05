# Project concepts

## `unitopia`

A Rust package that implements a generic newtype for unit-of-measurement + helpers.

Requirements:

* Must export a [Measure newtype](#measure-newtype)
* Must use US English spelling

Design options:

* Add scaling
  * Tasks
    * Add `numerator: i64`, `denominator: NonZeroU64`, `power: i64`
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
* Must support a generic storage type (e.g. `u32`, `u64`, `i32`, `i64`, `f32`, `f64` or any other generic type that implements the necessary traits for arithmetic operations).
* Must disallow adding or subtracting measures of different units.
* Must allow multiplying or dividing measures of different units.
  * The result must have its own distinct unit that is a multiplication or division of input units.
* Must support fractional values (e.g. millisecond).
* Must integrate with serialization frameworks (feature-gated), at least the following:
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
* Should follow SI
  * Notes:
    * This is a preference, not a requirement

Open questions:

* How to represent [derived units](#derived-unit)?
  * Examples:
    * Newton
    * Square meter
  * Ideas:
    * Represent them as separate units
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
    * May use `Mul` type only (represent `Div` as `Mul` with negative power) (e.g. `type Newton = Mul<Mul<Kilogram, 1, Meter, 1>, Mul<Second, 1, Second, 1>, 1, -1>;`)
* How to represent units that are a constant multiple of other units?
  * Examples:
    * Minute is 60 * Second
    * Millisecond is 0.001 * Second
    * Foot is 0.3048 * Meter (since an international agreement in 1959)
  * Ideas:
    * Represent them as base unit + [rational type](#rational-type)
    * Represent them as completely different type
    * Represent them as type that encodes the scale information in the type itself using `typenum` (see example: src/drafts/scale.rs)

Notes:

* The ideas are not requirements. If you think that an idea is wrong, tell me about it and don't implement it.

## Custom unit

A [unit](#unit) that is not a part of SI.

Examples:

* Galactosidase Activity Unit (GaIU) defined as "the amount of α-galactosidase that releases 1 micromole (1 µmol) of p-nitrophenol per minute from a synthetic substrate (commonly p-nitrophenyl-α-D-galactopyranoside), under specified assay conditions (temperature and pH)".
* Power of hydrogen (pH) defined as "−log10(a_H+), where a_H+ is the activity of hydrogen(1+) ions in solution"

## Quantity

A property of a physical object that can be quantified by measurement.

Examples:

* Length ([base quantity](#base-quantity))
* Mass ([base quantity](#base-quantity))
* Time ([base quantity](#base-quantity))
* Speed ([derived quantity](#derived-quantity))
* Force ([derived quantity](#derived-quantity))
* Energy ([derived quantity](#derived-quantity))
* Volume ([derived quantity](#derived-quantity))
* Radian ([derived quantity](#derived-quantity))

## Base quantity

A [quantity](#quantity) that is not expressed as a [monomial](#monomial) of other quantities.

Examples:

* Length
* Mass
* Time

## Derived quantity

A [quantity](#quantity) that is expressed as a [monomial](#monomial) of other quantities.

Examples:

* Speed
* Force
* Energy
* Volume

## Unit

A magnitude of a [quantity](#quantity).

Examples:

* Second is a unit of time defined as "the duration of 9,192,631,770 periods of the radiation corresponding to the transition between the two hyperfine levels of the ground state of the caesium-133 atom"
* Meter is a unit of length defined as "the length of the path travelled by light in vacuum during a time interval of 1/299792458 of a second"
* Mole is a unit of amount of substance defined as exactly 6.02214076 * 10^23.

Notes:

* The units of the same [quantity](#quantity) can be converted between each other.
  * Examples:
    * Meter and foot.
    * Radian and degree.
* Spelling of the metric unit for length:
  * "Meter" in the US and the Philippines
  * "Meter" in other English-speaking nations

## Base unit

A [unit](#unit) of a [base quantity](#base-quantity).

## Derived unit

A [unit](#unit) of a [derived quantity](#derived-quantity).

## Monomial

An algebraic expression which is a multiplication of a set of variables raised to specific powers.

Examples:

* m^2 (square meter, unit of area)
* kg *m* s^-2 (newton, unit of force)

Notes:

* Some variables may have power = 0, so they may be omitted.
* Division is represented by negative powers

## Rational type

A Rust type that can represent rational numbers.

Examples:

* `f32`
* `f64`
* `rust_decimal::Decimal`
* `num::rational::Ratio`

Non-examples:

* `u32`
* `u64`

Notes:

* Some rational types are lossy (e.g. `f32`, `f64`)
