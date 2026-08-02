# Project concepts

## This document

A specification for [`unitopia`](#unitopia).

Notes:

* Some list items have a format "{id}: {text}" where id is a string in CAPS (you can use the id to refer to the list item)

## `unitopia`

A Rust workspace that provides multiple patterns of implementations of quantity value types.

Requirements:

* Must use the [physical definitions set](#physical-definitions-set)
* Must keep its members in `packages` dir
* Must contain at least one member package that exports a [quantity value type](#quantity-value-type)
* Every member must use macros to avoid boilerplate code when defining units and prefixes
* Every member must define the macros in `src/macros.rs`
* Every member must have the following crate-level attributes in `src/lib.rs`:
  * `#![no_std]`
  * `#![forbid(unsafe_code)]`
* Must use US English spelling

Allowances:

* May implement banned traits but must not call their methods and must not use operators that desugar to their methods
  * Rationale:
    * Some dependents may prefer to use the banned traits, so we should provide the implementations
    * The traits are banned because their implementations may panic or silently break the underlying assumptions, so we should not call their methods

Notes:

* The adjective "unital" indicates that a corresponding noun is related to a [unit](#unit).

Design choices:

* Kind
  * Unit
  * Prefix
* Archetype
  * Value
  * Vanilla marker struct
  * Generic marker struct with only a single argument
  * Wrapper struct

Constants:

| Name      | Value                                                         | Notes                                              |
|-----------|---------------------------------------------------------------|:---------------------------------------------------|
| u128::MAX | 340282366920938463463374607431768211455                       |                                                    |
| 10^38     | 100000000000000000000000000000000000000                       | Largest power of 10 representable by u128          |
| 10^60     | 1000000000000000000000000000000000000000000000000000000000000 | Largest conversion factor in SI (quetta to quecto) |

Prefix conversion notes:

* It is possible but not yet necessary to implement an `fn conversion_succeeds<Src: Scale + Bounded, Dst: Scale + Bounded>() -> Option<bool>`:
  * Return values:
    * `Some(true)` if it will always succeed
    * `Some(false)` if it will always fail
    * `None` if it will succeed or fail depending on the actual values at runtime
  * Details:
    * Some conversions will always succeed
      * Example groups:
        * Conversions where the source inner type is smaller than the target inner type
          * Examples:
            * `Kilo<u32>` into `Kilo<u64>`
        * Conversions where the source prefix is larger than target prefix but the target inner type is large enough to hold the largest source value
          * Examples:
            * `Kilo<u32>` into `Uno<u128>` (`u32::MAX * 10 ^ (3 - 0) < u128::MAX`)
            * `Kilo<u32>` into `Deci<u128>` (`u32::MAX * 10 ^ (3 - 1) < u128::MAX`)
            * `Kilo<u32>` into `Kilo<u64>` (`u32::MAX * 10 ^ (3 - 3) < u64::MAX`) (a special case already covered by previous example group)
    * Some conversions will always fail
    * Some conversions will succeed or fail depending on the actual values at runtime
  * Notes:
    * The min and max values should have `rust_decimal::Decimal` type to avoid precision loss
    * `Bounded` is a trait from `num_traits`

Blockers:

* B001: Generic wrapper structs cannot implement `Into<T>` for their inner type without conflicting with `core`'s blanket `Into` impl (`impl<T, U> Into<U> for T where U: From<T>`), and `From<Wrapper<T>> for T` is also forbidden by orphan/coherence rules. Workarounds:
  * Define a special trait that would have a conversion method (for example, `fn into_inner(self) -> T`) and implement it for generic wrappers.
  * Keep `From<T> for Wrapper<T>` and use the blanket `Into<Wrapper<T>> for T` for the construction direction.
  * Implement `Into<Inner>` only for non-generic concrete wrapper types where coherence allows it.

## `unitopia-helpers`

A [`unitopia`](#unitopia) member package that exports various helpers.

Requirements:

* Must export the following macros:
  * `define_strict_wrapper_struct`
* Must export a `Scalar<T>` type (a [strict open wrapper struct](#strict-open-wrapper-struct) for scalars)
  * Notes:
    * Needed to implement `Mul`, `Div`, `MulAdd` in a generic way while satisfying Rust coherence rules
* Must export a `HasScale` trait:
  * Requirements:
    * Must have a `const NUM: u128; // numerator`
    * Must have a `const DEN: u128; // denominator`
* Must export the [linearity marker traits](#linearity-marker-trait)

## `unitopia-test-helpers`

A [`unitopia`](#unitopia) member package various helpers for a [quantity value test package](#quantity-value-test-package).

TODO:

* The testing approach that uses a custom language can't do compile-fail tests
  * Or can it? Can we ask the TestContext implementor package to produce a valid Rust code?

Requirements:

* Must export the following functions:
  * `parse_fractional_unit` ((1, Milli, Second))
  * `add_sub_scalar_failure` (compile-fail)
  * `add_sub_same_unit`
  * `add_same_quantity_unit`
  * `sub_same_quantity_unit`
  * `add_different_unit_failure` (compile-fail)
  * `sub_different_unit_failure` (compile-fail)
  * `mul_div_scalar`
  * `mul_div_same_unit`
  * `mul_div_different_unit`
  * `newton_eq_kg_m_s2`
    * This test must check that `1 Newton` is equal to `1 Kilogram * 1 Meter / (1 Second * 1 Second)`

## `unitopia-marker-quantities`

A [`unitopia`](#unitopia) member [quantity package](#quantity-package) that exports physical quantities implemented as [marker structs](#marker-struct).

## `unitopia-marker-units`

A [`unitopia`](#unitopia) member [unit package](#unit-package) that exports physical units implemented as [marker structs](#marker-struct).

## `unitopia-marker-prefixes`

A [`unitopia`](#unitopia) member [prefix package](#prefix-package) that exports prefixes implemented as [marker structs](#marker-struct).

## `unitopia-marker-arith-outputs`

A [`unitopia`](#unitopia) member package that exports the following [marker structs](#marker-struct):

* `Prod<A, B>`
* `Quot<A, B>`
* `Powr<A, N>`
* `Summ<A>`
* `Diff<A>`

Requirements:

* Must define all types in `src/lib.rs`

## `unitopia-open-wrapper-arith-outputs`

A [`unitopia`](#unitopia) member package that exports the following [OWS](#open-wrapper-struct)

* `Prod<A, B, T>`
* `Quot<A, B, T>`
* `Powr<A, N, T>`

Requirements:

* Every arith output type must have a `pub inner: T` field (`T` is the storage type)
* Must define all types in `src/lib.rs`

## `unitopia-strict-wrapper-prefixes`

A [`unitopia`](#unitopia) member [prefix package](#prefix-package) that exports prefixes implemented as [strict open wrapper structs](#strict-open-wrapper-struct) whose `T` represents a unit.

Requirements:

* Must implement the same traits as for units.

Notes:

* Example usage:
  * `Milli<Second<u64>>`

## `unitopia-strict-wrapper-units`

A [`unitopia`](#unitopia) member [unit package](#unit-package) that exports physical units implemented as [strict open wrapper structs](#strict-open-wrapper-struct) whose `T` represents a generic storage type.

* Must define, export, use the following macros:
  * `define_strict_wrapper_unit`
    * Must use `define_strict_wrapper_struct`
* Must use the types from `unitopia-open-wrapper-arith-outputs` to implement the [general multiplication traits](#general-multiplication-trait)

## `unitopia-measure-draft`

A [`unitopia`](#unitopia) member package that exports a `Measure` type.

## `unitopia-runtime-unit-quantity-value`

A [`unitopia`](#unitopia) member [generic quantity value package](#generic-quantity-value-package) that exports [`QuantityValue`](#quantityvalue).

Requirements:

* Must define all types in `src/lib.rs`
* Must have an optional `serde` feature:
  * If enabled: `QuantityValue` must derive `Serialize` and `Deserialize`.

## `unitopia-measurement-draft`

A [`unitopia`](#unitopia) member package that exports a `Measurement` [FGQVT](#fully-generic-quantity-value-type).

Requirements:

* Must use the `Unit` and `Quantity` traits from `unitopia-helpers`
* Must use the types from `unitopia-marker-prefixes` and `unitopia-marker-units` in the tests
* Must implement conversions from and into types exported from the foreign timekeeping crates (see [timestamp-please project doc](#timestamp-please-project-doc))
  * Must not lose data on conversions:
    * Must implement conversions into foreign timekeeping types only for those unit types and storage types that are natively supported by the foreign timekeeping types
    * Must implement conversions from foreign timekeeping types via `IntoMeasurement` trait with `into_measurement` method that may return a tuple of `Measurement` values
      * Rationale: this is necessary to correctly convert types that stores values of different units in the same type (e.g. `core::time::Duration` which stores both seconds and nanoseconds)
      * Requirements:
        * The storage types used in the implementation `Output` type must be at least as large as the storage types used in the implementor (so that conversion is lossless)

```rust
pub trait IntoMeasurement {
    type Output;

    fn into_measurement(self) -> Self::Output;
}
```

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

* Sizes must be calculated for a pair of values with the largest and smallest SI prefix

## Physical type

A newtype that represents a physical measurement outcome.

Requirements:

* Must implement [identity traits](#identity-trait).
* Must implement [addition traits](#addition-trait) for values with the same quantity (with same or different units) if this unit implements the `Linear` [linearity marker trait](#linearity-marker-trait).
  * Requirements:
    * Every addition operation that returns a value must return a quantity value whose quantity is `Summ<Q>` (where `Q` is the input quantity type).
    * `Summ` must have exactly one generic argument.
* Must implement [subtraction traits](#subtraction-trait) for values with the same unit and quantity if this unit implements the `Linear` [linearity marker trait](#linearity-marker-trait).
  * Requirements:
    * Every subtraction operation that returns a value must return a quantity value whose quantity is `Diff<Q>` (where `Q` is the input quantity type).
    * `Diff` must have exactly one generic argument.
* Must not implement [addition traits](#addition-trait) for values with different quantities (use [compile-fail tests](#compile-fail-test)).
* Must not implement [addition traits](#addition-trait) for values with scalars of any storage type (use [compile-fail tests](#compile-fail-test)).
* Must not implement [subtraction traits](#subtraction-trait) for values with different units (use [compile-fail tests](#compile-fail-test)).
* Must not implement [subtraction traits](#subtraction-trait) for values with different quantities (use [compile-fail tests](#compile-fail-test)).
* Must not implement [subtraction traits](#subtraction-trait) for values with scalars of any storage type (use [compile-fail tests](#compile-fail-test)).
* Must implement [general multiplication traits](#general-multiplication-trait) for values with the same or different units.
  * Requirements:
    * Must have a `type Output` with a distinct unit that represents a [monomial](#monomial) of input units.
* Must implement [scalar multiplication traits](#scalar-multiplication-trait) for values with scalars of the same storage type.
* Must implement `num_traits::MulAdd` for values where `A` parameter is a unit and `B` parameter is a unit that represents a multiplication of `A` and `Self` if `B` implements the `Linear` [linearity marker trait](#linearity-marker-trait).
  * Requirements:
    * Must have a `type Output` with a distinct unit that represents a multiplication of `Self` and `A` units.
* Must implement `num_traits::MulAdd` for values where `A` parameter is a scalar and `B` parameter is `Self` if `Self` implements the `Linear` [linearity marker trait](#linearity-marker-trait).
* Must implement `num_traits::MulAddAssign` for values where `A` parameter is a scalar and `B` parameter is `Self` if `Self` implements the `Linear` [linearity marker trait](#linearity-marker-trait).
* Must implement serialization/deserialization traits from the popular crates (feature-gated):
  * `serde`
  * `rkyv`
  * `bitcode`
* Must implement checked, wrapping, overflowing, saturating arithmetic operation traits from `num-traits`
* Must use methods instead of operators in trait implementations
  * Examples:
    * Use `add_assign` instead of `+=`

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

* How to represent [SI derived units](#si-derived-unit)?
  * Examples:
    * Newton
    * Square meter
  * Ideas:
    * DUS: Represent them as separate types
    * DUTNP: Represent them as tuples of nested pairs where the first element is the [SI base unit](#si-base-unit) and the second element is the power
      * Examples
        * `pub type Newton = ((Kilogram, P1), (Meter, P1), (Second, N2));`
      * Properties:
        * Semantically same are syntactically same: No
          * Counterexamples:
            * `pub type Newton2 = ((Meter, P1), (Kilogram, P1), (Second, N2));`
          * Notes:
            * This can be mitigated by convention:
              * Document a specific order of units as canonical
              * Implement arithmetic traits in a way that `type Output` has a canonical order of units
    * DUTFP: Represent them as tuples of flattened pairs where the first element is the [SI base unit](#si-base-unit) and the second element is the power
      * Examples:
        * `pub type Newton = (Kilogram, P1, Meter, P1, Second, N2);`
      * Pros:
        * Less code
      * Cons: (I don't see any, but it puts the units and powers on the same level, so maybe some cons will be discovered during implementation)
    * DUIM: Represent them as specifications of `Mul` type only (represent `Div` as `Mul` with negative power) (e.g. `type Newton = Mul<Mul<Kilogram, P1, Meter, P1>, Mul<Second, P1, Second, P1>, P1, N1>;`)
      * Superseded by DUTNP and DUTFP
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
    * Notes:
      * The examples use types from `typenum`
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
    * CMTT: Represent them as type that encodes the scale information in the type itself using `typenum` (see example: packages/unitopia-draft-scale/src/scale.rs)
* How to convert between values of the same unit but different prefix?
  * Examples
    * `Second<Milli<u32>>` and `Second<Atto<f64>>`
  * Ideas:
    * Require the user to perform this conversion

Notes:

* The ideas are not requirements. If you think that an idea is wrong, tell me about it and don't implement it.
* Stable Rust 1.85 supports arithmetic in const generics only for constant operands wrapped in braces (e.g. `Foo::<{ 2 + 2 }>::new(42)`)
* Some units are non-linear (example: pH)
  * If a package implements units as distinct types:
    * Then: it's possible to implement traits selectively, so non-linear units must not implement [scalar multiplication traits](#scalar-multiplication-trait)
    * Else: it's not possible to implement traits selectively, so non-linear units must implement [scalar multiplication traits](#scalar-multiplication-trait) as a consequence of generic implementations

## Base terms

Examples (exhaustive):

* Dimension
* Unit
* Quantity

## Physical definitions set

A set of definitions for [base terms](#base-terms) used in theoretical physics.

## Metrological definitions set

A set of definitions for [base terms](#base-terms) used in metrology.

Notes:

* This set is used by BIPM that publishes SI
* This set is used by OIML that publishes VIM

## Dimension

One of:

* [Dimension (mathematics)](#dimension-mathematics)
* [Dimension (physics)](#dimension-physics)

## Dimension (mathematics)

A number of degrees of freedom.

Examples:

* The dimension of a vector space is the number of coordinates necessary to specify any vector.

Notes:

* The use of the word "dimension" to mean a specific coordinate axis (e.g. "time dimension") is informal and mathematically incorrect.

## Dimension (physics)

A [monomial](#monomial) of base [quantity kinds](#quantity-kind).

Examples:

* `1` - a monomial where every quantity kind has a power of 0
* `M` - a monomial where mass quantity kind has a power of 1 and all other quantity kinds have a power of 0 (represents the mass itself)
* `M * L * T^-2` - a monomial where mass quantity kind has a power of 1, length quantity kind has a power of 1, time quantity kind has a power of -2, all other quantity kinds have a power of 0 (represents force)

Notes:

* The list of base quantity kinds is determined by the metric system.

## Dimensional (physics)

A [dimension (physics)](#dimension-physics) which is not reduced according to algebraic rules (remains the same expression as determined by the formula).

Examples:

* `L^+1 * L^-1` - a dimensional of radian
* `L^+2 * L^-2` - a dimensional of steradian

Notes:

* This definition is non-standard, but it allows to distinguish between truly dimensionless quantities produced by pure counting (e.g. amount of substance, count of oscillations) and the quantities that appear to be dimensionless but actually are calculated from other quantities which are not dimensionless (e.g. radian, steradian).
* Every dimension is a valid dimensional.
  * `1` is a special dimensional for pure counts (e.g. amount of substance, count of oscillations)

## Quantity kind

A kind of a property that can be quantified by measurement.

Examples:

* Time
* Length
* Mass
* Speed
* Force
* Energy
* Volume
* Heat
* Angle
* Information

Notes:

* Some quantity kinds are non-physical (e.g. information, specific currency)
* Some quantity kinds are [dimensionless](#dimension-physics) (e.g. angle, bit)
* Some quantity kinds are derived (e.g. force is `mass * length / (time * time)`)

## Quantity kind group

A group of related quantity kinds.

Examples:

* Physics: Length, Mass, Time.
* Currency: U.S. Dollar, Bitcoin, Rai.
* Other: Information.

## Quantity

A property that can be quantified by measurement.

Synonyms: Measurand.

Examples:

* Q001: Height of a human measured at under specific physical conditions (time, location, temperature) and posture (head in Frankfort horizontal plane)
* Q002: Height of a building measured at under specific physical conditions (time, location, temperature)
* Q003: Width of a human from shoulder tip to shoulder tip measured at under specific physical conditions (time, location, temperature)
* Q004: Balance of U.S. Dollar on a specific bank account

Notes:

* Some quantities are of the same quantity kind
  * Examples:
    * Q001 and Q002 are quantities of "Length" kind
    * "Width", "Height", "Depth" are quantities of "Length" kind.
* It is possible to make the quantity more precise by fixing other properties that may affect the measurement (e.g. time, location, temperature)

## Quantity value

A pair of a value and a unit.

Examples:

* 1 hour
* 3 kilograms
* 12 newtons

## Measure

A tuple of a value, a unit, a quantity.

Examples:

* 74.55 kg is the mass of a person identified by passport number 98882348 issued by Russian Federation, recorded at 2024-08-02T09:26:13Z on scales identified by inventory number 1238343.

Note:

* VIM uses the word "measure" to mean an instrument, but in common English "measure" can mean both the instrument and the output, and we need a short word because we'll use this definition often.

## SI base quantity kind

A [quantity kind](#quantity-kind) that is designated as foundational in the International System of Units (SI).

Examples (exhaustive):

* Time
* Length
* Mass
* Electric current
* Thermodynamic temperature
* Amount of substance
* Luminous intensity

## SI derived quantity kind

A [quantity kind](#quantity-kind) that is expressed as a [monomial](#monomial) of other SI quantities.

Examples:

* Speed
* Force
* Energy
* Volume

## Unit

A magnitude of a [quantity](#quantity).

Examples:

* Second is a unit of time defined as "the duration of 9,192,631,770 periods of the radiation corresponding to the transition between the two hyperfine levels of the ground state of the cesium-133 atom"
* Meter is a unit of length defined as "the length of the path traveled by light in vacuum during a time interval of 1/299792458 of a second"
* Mole is a unit of amount of substance defined as exactly 6.02214076 * 10^23.
* Bit is a unit of information (doesn't have a physical definition because the amount of information representable by a physical object is inherently subjective to a specific observer - it depends on how many states of a physical object a specific observer can distinguish).
* Radian is a unit of angle defined as the angle subtended at the center of a plane circle by an arc that is equal in length to the radius.

Non-examples:

* Year is not a unit of time because different years have different numbers of days.
* Month is not a unit of time because different months have different numbers of days (and Feb has a different number of days depending on the year).

Notes:

* Unit does not uniquely determine the quantity kind
  * Examples
    * Pascal is associated both with pressure and stress
* Unit does not uniquely determine the [dimension (physics)](#dimension-physics)
  * Examples
    * Radian and steradian are units of different quantity kinds but same dimension `1` (i.e. they are "dimensionless")
* Unit does uniquely determine the [dimension (physics)](#dimension-physics) for raw (non-reduced) monomials:
  * Examples:
    * Radian and steradian have different raw monomials
      * Radian is `L^+1 * L^-1`
      * Steradian is `L^+2 * L^-2`
* The units of the same [quantity kind](#quantity-kind) can be converted between each other.
  * Examples:
    * Meter and foot.
    * Radian and degree.
* Spelling of the metric unit for length:
  * "Meter" in the US and the Philippines
  * "Metre" in other English-speaking nations

## SI base unit

A [unit](#unit) of a [SI base quantity kind](#si-base-quantity-kind).

## SI derived unit

A [unit](#unit) of a [SI derived quantity kind](#si-derived-quantity-kind).

## Custom unit

A [unit](#unit) that is not a part of SI.

Examples:

* Galactosidase Activity Unit (GaIU) defined as "the amount of α-galactosidase that releases 1 micromole (1 µmol) of p-nitrophenol per minute from a synthetic substrate (commonly p-nitrophenyl-α-D-galactopyranoside), under specified assay conditions (temperature and pH)".
* Power of hydrogen (pH) defined as "−log10 (a_H+), where a_H+ is the activity of hydrogen (1+) ions in solution"

## Prefix

A name of a rational number acts as a coefficient for a [unit](#unit).

Examples:

* Kilo (1000 / 1)
* Giga (1000000000 / 1)
* Uno (1 / 1)
* Micro (1 / 1000000)
* Hexagesi (60 / 1)
* Tetravigesi (24 / 1)
* Quetta (10 ^ 30 / 1)
* Quecto (1 / 10 ^ 30)
* Ronto (1 / 10 ^ 27)

Notes:

* A prefix may be a part of a base unit name (e.g. kilogram)
* All SI prefixes have numerators or denominators that fit in `u128`
  * The conversion may still overflow, so use `Checked` versions of traits

## SI prefix

A [prefix](#prefix) that is a part of SI.

## Custom prefix

A [prefix](#prefix) that is not a part of SI.

Examples:

* Hexagesi (60 / 1)
* Tetravigesi (24 / 1)

## Monomial

An algebraic expression which is a multiplication of a set of variables raised to specific powers.

Examples:

* `m^2` (square meter, unit of area)
* `kg * m * s^-2` (newton, unit of force)

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

## Identity trait

A trait from the following list:

* `num_traits::ConstZero`
* `num_traits::ConstOne`
* `num_traits::Zero`
* `num_traits::One`

## Addition trait

A trait from the following list:

* `core::ops::Add`
* `core::ops::AddAssign`
* `num_traits::CheckedAdd`
* `num_traits::SaturatingAdd`
* `num_traits::WrappingAdd`
* `num_traits::OverflowingAdd`

## Subtraction trait

A trait from the following list:

* `core::ops::Sub`
* `core::ops::SubAssign`
* `num_traits::CheckedSub`
* `num_traits::SaturatingSub`
* `num_traits::WrappingSub`
* `num_traits::OverflowingSub`

## General multiplication trait

A trait from the following list:

* `core::ops::Mul`
* `core::ops::Div`
* `num_traits::CheckedMul`
* `num_traits::CheckedDiv`
* `num_traits::SaturatingMul`
* `num_traits::WrappingMul`
* `num_traits::OverflowingMul`

Notes:

* The following traits don't exist because they don't make sense for division:
  * `num_traits::SaturatingDiv`
  * `num_traits::WrappingDiv`
  * `num_traits::OverflowingDiv`
* Units must have implementations of `core::ops::Rem` and `num_traits::CheckedRem` whose `Rhs` is a scalar
* Units must not have implementations of `core::ops::Rem` and `num_traits::CheckedRem` whose `Rhs` is a unit

## Scalar multiplication trait

A trait that is either [general multiplication trait](#general-multiplication-trait) or a trait from the following list:

* `num_traits::Pow`
* `core::ops::Rem`
* `num_traits::CheckedRem`
* `core::ops::MulAssign`
* `core::ops::DivAssign`

Notes:

* `MulAssign` and `DivAssign` are scalar-only because their functions return `()`

## Quantity package

TODO

## Unit package

A Rust package that exports [unit](#unit) types.

Requirements:

* Must define all base units from SI.
* Must define at least the following non-SI units:
  * `PowerOfHydrogen`
  * `GalactosidaseActivityUnit`
* Must define at least the following derived units as type aliases (as compositions of base units and other types):
  * `Area`
  * `Newton`
  * `Volt`
* Must keep the unit definitions in src/units.rs (not separate files)

## Prefix package

A Rust package that exports [prefix](#prefix) types.

Requirements:

* Must export all [SI prefixes](#si-prefix)
* Must export all [custom prefixes](#custom-prefix) listed in examples
* Must define all prefixes in src/prefixes.rs (not separate files)
* Must contain the following tests:
  * `mul_giga_scalar_is_giga`
  * `add_giga_nano_is_nano`
    * Must construct `large` as 1 gigameter from `1u128`.
    * Must construct `small` as 1 nanometer from `1u128`.
    * Must `assert!(large > small);`
    * Must calculate `let sum = large + small;`
    * Must have a type annotation on `sum` that contains the nano unit
    * Must `assert!(sum > large);`
    * Must `assert!(sum > small);`
    * Must calculate `let diff = sum - small;`
    * Must have a type annotation on `diff` that contains the nano unit
    * Must `assert!(diff < sum);`
    * Must `assert!(diff > small);`
    * Must calculate `(diff_large, diff_large_remainder)` by converting it to a value with giga prefix
    * Must `assert!(diff_large_remainder.is_zero());`
    * Must `assert_eq!(diff_large, large);`
  * `mul_giga_nano_is_uno`

## Quantity value package

One of:

* [Generic quantity value package](#generic-quantity-value-package)
* [Macro quantity value package](#macro-quantity-value-package)

## Generic quantity value package

A package that exports generic [quantity value types](#quantity-value-type) that must be specialized by the user.

Allowances:

* May export generic quantity value types .
* May define the quantity value types using a [quantity value definition package](#macro-quantity-value-package).

## Macro quantity value package

A package that exports code items and guidelines for defining new [quantity value types](#quantity-value-type).

Allowances:

* May export macros for defining specific quantity value types.

## Quantity value test package

A package that tests a specific [quantity value package](#quantity-value-package).

Requirements:

* Must define one [quantity value type](#quantity-value-type) per quantity in `unitopia-test-helpers` using only the items exported from the specific quantity value package under test.

## Marker trait

A trait that doesn't have any functions or associated types.

Notes:

* A marker trait may have generic parameters.

## Mapping trait

A trait that doesn't have any functions but does have associated types.

Notes:

* A mapping trait may have generic parameters.
* The name "mapping" was chosen because it provides a type-level mapping from the implementing type (`Self`) to the associated type.

## Linearity marker trait

A [marker trait](#marker-trait) with one of the following names:

* `Linear`
* `Logarithmic`
* `Exponential`

## Marker struct

A struct whose every field is a `PhantomData`.

Requirements:

* Must have a `#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]` attribute

## Vanilla marker struct

A [marker struct](#marker-struct) without generic arguments.

## Generic marker struct

A [marker struct](#marker-struct) with at least one generic argument.

## Container struct

A struct with at least one generic parameter `T` and at least one field of type `T`.

## Wrapper struct

A struct with at least one generic parameter `T`, exactly one field `inner: T` and any number of fields whose outer type is `PhantomData`.

Requirements:

* Must have a `#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]` attribute
* Must have a `#[repr(transparent)]` attribute
* Must implement `Deref`, `AsRef`, `Borrow` by delegating to the corresponding impl on the `inner` field
* Must implement `From<T>`
* Must not implement `Into<T>` (see B001)
* Must have a `pub const fn new`
  * Notes:
    * `From::from` can't be used instead of `new` because it is not `const`

## Open wrapper struct

A [wrapper struct](#wrapper-struct) with the following properties:

* Must have an `inner` field that is `pub`
* Must implement `DerefMut`, `BorrowMut` by delegating to the corresponding impl on the `inner` field

Synonyms: OWS.

## Strict wrapper struct

A [wrapper struct](#wrapper-struct) with exactly one generic parameter `T`, exactly one field `inner: T`, zero fields whose outer type is `PhantomData`.

Synonyms: SWS.

## Strict open wrapper struct

A [wrapper struct](#wrapper-struct) that is both [open](#open-wrapper-struct) and [strict](#strict-wrapper-struct).

Synonyms: SOWS.

## Numeric type

A type that implements `num_traits::NumAssignRef`.

Examples:

* `i8`
* `u32`
* `usize`
* `f64`
* `rust_decimal::Decimal`
* `num::rational::Ratio`

Notes:

* `num_traits::NumAssignRef` is implemented automatically for types that implement its supertraits.

## Numeric type conditional on T

A type that is [numeric](#numeric-type) if T is [numeric](#numeric-type).

## Numeric strict open wrapper struct

A [strict open wrapper struct](#strict-open-wrapper-struct) that is [numeric type conditional on T](#numeric-type-conditional-on-t) (where `T` is the generic parameter of the wrapper).

Synonyms: NSOWS

Requirements:

* Must conditionally implement `num_traits::NumAssignRef` (this is a consequence of this type being numeric)

## Prefixed numeric strict open wrapper struct

A [NSOWS](#numeric-strict-open-wrapper-struct) whose name is a [prefix](#prefix).

Synonyms: PNSOWS

Examples:

* `Quetta<T>`
* `Ronto<T>`
* `Tetravigesi<T>`

Notes:

* The examples show only the most significant part of the type definition.
* Some values of PNSOWS are semantically equal but not syntactically equal
  * Examples
    * `Quetta::ZERO` and `Ronto::ZERO`
    * `Kilo::from(1usize)` and `Uno::from(1000usize)`
* Most conversions may result in an overflow
  * Examples:
    * `Uno::<usize>::opt_from(Kilo::from(usize::MAX))`

## Unital prefixed numeric strict open wrapper struct

A [NSOWS](#numeric-strict-open-wrapper-struct) whose name is a [unit](#unit) and whose `T` is an [PNSOWS](#prefixed-numeric-strict-open-wrapper-struct).

Synonyms: UPNSOWS

Examples:

* `Second<T>`
* `Newton<T>`

Notes:

* The examples show only the most significant part of the type definition.

## Arith output

A struct that represents the output of the arithmetic operation.

Notes:

* Arith outputs should be used as `type Output` in the arithmetic trait implementations.

## Quantity value type

A type that represents a [quantity value](#quantity-value).

Requirements:

* TODO: Move the required trait impls
* Must implement [addition traits](#addition-trait) for values of the same quantity and unit
  * Must set the quantity in the `Output` to `Summ<Q>` where `Q` is the input quantity
* Must implement [subtraction traits](#subtraction-trait) for values of the same quantity and unit
  * Must set the quantity in the `Output` to `Diff<Q>` where `Q` is the input quantity
* TODO (?): Must implement the `Quantity` trait from `unitopia_helpers`
  * (we need to express a requirement that a quantity value type must have a value, a unit, a storage)

## `QuantityValue`

A runtime-unit [quantity value type](#quantity-value-type) exported by [`unitopia-runtime-unit-quantity-value`](#unitopia-runtime-unit-quantity-value).

```rust
pub struct QuantityValue<Value, Unit> {
    pub value: Value,
    pub unit: Unit,
}
```

Requirements:

* Must have a constructor that accepts both fields.
* Its arithmetic requirements override the infallible addition and subtraction requirements of [quantity value type](#quantity-value-type).
* Must have a `Display` impl generic over `<Value: Display, Unit: Display>`:
  * Must write `"{value} {unit}"`.
* Must have methods:
  * `try_add`
    * Must perform fallible addition with another `QuantityValue` of the same Rust type.
  * `try_sub`
    * Must perform fallible subtraction with another `QuantityValue` of the same Rust type.
* `try_add` and `try_sub`:
  * Must compare the runtime units before operating on the values.
  * Must return `Err` containing both operands if the runtime units differ.
  * Must use checked value arithmetic.
  * Must return `Err` containing both operands if the value arithmetic fails.
  * Must not mutate either operand before every fallible validation and arithmetic operation succeeds.
* Must not implement `Add`, `AddAssign`, `Sub`, or `SubAssign` between two `QuantityValue` values.
  * Rationale: values with different runtime units have the same Rust type, while these traits cannot report a unit mismatch without making their output unexpectedly fallible.
* Scalar operations that preserve the unit may be infallible only when the corresponding operation on `Value` is infallible.

Notes:

* The spec for this type has been written by an LLM.
* `QuantityValue<Decimal, Currency>` can represent money whose currency is selected at runtime.
* This type contains no [quantity](#quantity) identity, so it is not a [measure](#measure).

## Quantity-generic quantity value type

A [quantity value type](#quantity-value-type) that has a generic parameter `Q` for quantity.

## Unit-generic quantity value type

A [quantity value type](#quantity-value-type) that has a generic parameter `U` for unit.

## Storage-generic quantity value type

A [quantity value type](#quantity-value-type) that has a generic parameter `S` for storage.

* Must support any [numeric type](#numeric-type) as storage.

## Fully generic quantity value type

A [quantity value type](#quantity-value-type) that:

* Is [quantity-generic](#quantity-generic-quantity-value-type)
* Is [unit-generic](#unit-generic-quantity-value-type)
* Is [storage-generic](#storage-generic-quantity-value-type)

Synonyms: FGQVT

Example pattern:

```rust
use core::marker::PhantomData;

trait Quantity { type Dimension; }          // semantic marker + associated dimension
trait Unit { type Dimension; /* scale */ }  // unit carries dimension

struct Measurement<Q, U, S>
where
    Q: Quantity,
    U: Unit<Dimension = Q::Dimension>,
{
    value: S,
    quantity: PhantomData<Q>,
    unit: PhantomData<U>,
}
```

Notes:

* If exported from our crate, this type will be classified as a "foreign type" in dependents, so the dependents won't be able to implement foreign traits for it, so we must provide all trait implementations upfront.
  * This may not be possible at all in a generic way (trait impl overlaps, so we would need to specify every type exactly without using the generics).
  * It's a lot of work to cover all potential storage types that are published on crates.io.
  * We may export such type but mention in the docs that it has a limited set of impls, and if the user wishes to implement more traits for it, they can either submit a PR or define their own `Measurement` type using a macro.

## Fully specific quantity value type

A [quantity value type](#quantity-value-type) that doesn't have any generic parameters.

Requirements:

* Must have a suffix that is equal to the unit name in plural form.
* Must document the exact conditions of the measurement in a doc comment attached to the type.

Allowances:

* May document the exact conditions of the measurement in the type name itself.

Examples of names:

* `GameDelaySeconds`
* `PersonHeightAtNoonMillimeters`

## Quantity value type definition macro

A macro that defines a quantity type.

Synonyms: QVTDM.

Requirements:

* Name must start with "quantity"

Examples:

* `quantity` - defines a generic quantity type
* `quantity_of_seconds_as_u32` - defines a fully specific quantity type

## Fully specific quantity value type definition macro

A [QVTDM](#quantity-value-type-definition-macro) that defines a [fully specific quantity type](#fully-specific-quantity-value-type).

Synonyms: FSQVTDM.

Rationale:

* If the unit type and storage type are known statically at macro expansion time (which is even earlier than compile time), then it is possible to provide specific trait implementations
  * Examples of groups of specific trait implementations:
    * Infallible conversions to wider types (e.g. `impl From<DurationSeconds> for DurationMilliseconds` if DurationSeconds uses u32 and DurationMilliseconds uses u64; note that such impl is only possible if DurationSeconds uses a narrower type than DurationMilliseconds).
    * Infallible conversions from types from foreign crates (e.g. `std`, `time`, `chrono` for quantities of time kind).

Requirements:

* Name must match the pattern: `quantity_of_{{units}}_as_{{storage}}`
  * `{{units}}` must be a unit type name in snake_case and plural form
    * Examples:
      * `seconds`
  * `{{storage}}` must be a storage type name in snake_case and singular form:
    * Requirements:
      * The following path prefixes must be omitted: `core::primitive`
      * `::` must be replaced by `_`
    * Allowances:
      * The path prefix may be omitted for commonly used types
    * Examples:
      * `core::primitive::u64` -> `u64`
      * `rust_decimal::Decimal` -> `decimal`

Examples of names:

* `quantity_of_seconds_as_u32`

## Fully specific time quantity value type definition macro

A [FSQVTDM](#fully-specific-quantity-value-type-definition-macro) that uses a time unit.

Requirements:

* Must implement conversions from and into types exported from the foreign timekeeping crates (see [timestamp-please project doc](#timestamp-please-project-doc))

Examples of names:

* `quantity_of_seconds_as_u32`
* `quantity_of_nanoseconds_as_u128`

## timestamp-please project doc

A document at <https://github.com/DenisGorbachev/timestamp-please/blob/main/.agents/project.md>
