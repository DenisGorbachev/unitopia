# Project concepts

## This document

A specification for [`unitopia`](#unitopia)

Notes:

* Some list items have a format "{id}: {text}" where id is a string in CAPS (you can use the id to refer to the list item)

## `unitopia`

A Rust package that implements a generic newtype for unit-of-measurement + helpers.

Requirements:

* Must be a workspace package
* Must contain at least one member package that exports a [physical type](#physical-type)
* Must use US English spelling

Notes:

* Design choices:
  * Kind
    * Unit
    * Prefix
  * Archetype
    * Value
    * Vanilla marker struct
    * Generic marker struct with a single argument
    * Wrapper struct

## `unitopia-marker-units`

A Rust package that exports physical units implemented as [marker structs](#marker-struct).

* Must be a member of [`unitopia`](#unitopia)

## `unitopia-measure`

* Must export [Measure](#measure-newtype)

* Must be a member of [`unitopia`](#unitopia)

## Package metric

A value assigned to a specific code package.

Examples:

* Speed of calculations
* Min amount of memory used
* Size of code that constructs values
* Size of debug representation
* Scalability of trait implementations
* Ability to define custom units
* Ability to define custom prefixes (rational coefficients) for units

Notes:

* Sizes must be calculated for a pair of values with the largest and smallest prefix

## Physical type

A newtype that represents a physical measurement outcome.

Requirements:

* Must support a generic storage type (e.g. `u32`, `u64`, `i32`, `i64`, `f32`, `f64` or any other generic type that implements the necessary traits for arithmetic operations).
* Must support fractional values (e.g. millisecond).
* Must allow to represent a specific unit
  * May embed the unit name in the type name
  * May take the unit as a generic parameter
* Must implement traits for adding or subtracting of values with the same unit.
* Must not implement traits for adding or subtracting of values with different units (use [compile-fail tests](#compile-fail-test)).
* Must not implement traits for adding or subtracting of values with scalars of any storage type (use [compile-fail tests](#compile-fail-test)).
* Must implement traits for multiplying or dividing of values with the same or different units.
  * Requirements:
    * Must have a `type Output` with a distinct unit that represents a [monomial](#monomial) of input units.
* Must implement traits for multiplying or dividing of values with scalars of the same storage type.
* Must implement serialization/deserialization traits from the popular crates (feature-gated):
  * `serde`
  * `rkyv`
  * `bitcode`
  * `wincode`
* Must implement checked, wrapping, overflowing, saturating arithmetic operation traits from `num-traits`

Preferences:

* Should derive the serialization/deserialization traits instead of implementing them manually.
* Should provide `From` or `TryFrom` implementations for similar types from other crates:
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
    * DUS: Represent them as separate types
    * DUTNP: Represent them as tuples of nested pairs where the first element is the [base unit](#base-unit) and the second element is the power
      * Examples
        * `pub type Newton = ((Kilogram, 1), (Meter, 1), (Second, -2));`
      * Properties:
        * Semantically same are syntactically same: No
          * Counterexamples:
            * `pub type Newton2 = ((Meter, 1), (Kilogram, 1), (Second, -2));`
          * Notes:
            * This can be mitigated by convention:
              * Document a specific order of units as canonical
              * Implement arithmetic traits in a way that `type Output` has a canonical order of units
    * DUTFP: Represent them as tuples of flattened pairs where the first element is the [base unit](#base-unit) and the second element is the power
      * Examples:
        * `pub type Newton = (Kilogram, 1, Meter, 1, Second, -2);`
      * Pros:
        * Less code
      * Cons: (I don't see any, but it puts the units and powers on the same level, so maybe some cons will be discovered during implementation)
    * DUIM: Represent them as specifications of `Mul` type only (represent `Div` as `Mul` with negative power) (e.g. `type Newton = Mul<Mul<Kilogram, 1, Meter, 1>, Mul<Second, 1, Second, 1>, 1, -1>;`)
      * Superseded by DUNTP and DUTFP
    * DUIMD: Represent them as specifications of `Mul` or `Div` generic types
      * Examples:
        * `pub type Newton = Div<Mul<Kilogram, Meter>, Mul<Second, Second>>;`
      * Notes:
        * Superseded by DUIM
        * This makes the units which are semantically equivalent syntactically different
          * Examples:
            * `Mul<Kilogram, Meter>` and `Mul<Meter, Kilogram>`
          * Solutions:
            * Provide an `invert_unit` method for measures with `Mul` unit
            * Switch to runtime check
            * Represent all units in a system with a single type whose generic parameters are unit powers
              * May use `typenum` crate
* How to represent alternative units that are a constant multiple of base units?
  * Examples:
    * Minute is 60 * Second
    * Millisecond is 0.001 * Second
    * Foot is 0.3048 * Meter (since an international agreement in 1959)
  * Notes:
    * Using SI prefixes is not sufficient because the alternative units may have arbitrary coefficients (see the "Foot" example)
  * Ideas:
    * CMS: Represent them as a separate type
    * CMBU: Represent them as a base unit type, but use a [rational type](#rational-type) for the value (put the scale in the value)
    * CMTT: Represent them as type that encodes the scale information in the type itself using `typenum` (see example: src/drafts/scale.rs)

Notes:

* The ideas are not requirements. If you think that an idea is wrong, tell me about it and don't implement it.
* Stable Rust 1.92 supports arithmetic in const generics only for constant operands wrapped in braces (e.g. `Foo::<{ 2 + 2 }>::new(42)`)

## Measure newtype

A [physical type](#physical-type) that takes the [unit](#unit) as a generic parameter.

Requirements:

* Must support [custom units](#custom-unit).

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
  * "Metre" in other English-speaking nations

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

## Compile-fail test

A Rust file that is expected to fail to compile.

Requirements:

* Must be in a directory that is not used by cargo by default (e.g. `tests/compile_fail`)

Notes:

* Such files are used by `trybuild` to assert that they actually fail to compile.

## Marker struct

A struct whose every field is a `PhantomData`.

## Vanilla marker struct

A [marker struct](#marker-struct) without generic arguments.

## Generic marker struct

A [marker struct](#marker-struct) with at least one generic argument.

## Wrapper struct

A struct with exactly one field of type `T` (the generic parameter).

## Container struct

A struct with at least one field of type `T` (the generic parameter).
