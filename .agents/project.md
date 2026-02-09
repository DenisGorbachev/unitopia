# Project concepts

## This document

A specification for [`unitopia`](#unitopia).

Notes:

* Some list items have a format "{id}: {text}" where id is a string in CAPS (you can use the id to refer to the list item)

## `unitopia`

A Rust workspace that provides multiple implementations of physical types.

Requirements:

* Must keep its members in `packages` dir
* Must contain at least one member package that exports a [physical type](#physical-type)
* Every member must use macros to avoid boilerplate code when defining units and prefixes
* Every member must define the macros in src/macros.rs
* Every member must have the following crate-level attributes in src/lib.rs:
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

| Name      | Value                                                         | Notes                                       |
|-----------|---------------------------------------------------------------|:--------------------------------------------|
| u128::MAX | 340282366920938463463374607431768211455                       |                                             |
| 10^38     | 100000000000000000000000000000000000000                       | Max conversion factor representable by u128 |
| 10^60     | 1000000000000000000000000000000000000000000000000000000000000 | Max conversion factor (quetta to quecto)    |

Prefix conversion notes:

* It is possible but not yet necessary to implement an `fn conversion_succeeds<Src: Scale + Min + Max, Dst: Scale + Min + Max>() -> Option<bool>`:
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
    * The min and max values should have `rust_decimal::Decimal` type

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
* Must export a `Scale` trait:
  * Requirements:
    * Must have a `const NUM: u128; // numerator`
    * Must have a `const DEN: u128; // denominator`

## `unitopia-marker-units`

A [`unitopia`](#unitopia) member [unit package](#unit-package) that exports physical units implemented as [marker structs](#marker-struct).

## `unitopia-marker-prefixes`

A [`unitopia`](#unitopia) member [prefix package](#prefix-package) that exports prefixes implemented as [marker structs](#marker-struct).

## `unitopia-marker-arith-outputs`

A [`unitopia`](#unitopia) member package that exports the following [marker structs](#marker-struct):

* `Prod<A, B>`
* `Quot<A, B>`
* `Powr<A, N>`

Requirements:

* Must define all types in src/lib.rs

## `unitopia-open-wrapper-arith-outputs`

A [`unitopia`](#unitopia) member package that exports the following [OWS](#open-wrapper-struct)

* `Prod<A, B, T>`
* `Quot<A, B, T>`
* `Powr<A, N, T>`

Requirements:

* Every arith output type must have a `pub inner: T` field (`T` is the storage type)
* Must define all types in src/lib.rs

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

## `unitopia-measure`

A [`unitopia`](#unitopia) member package that exports [Measure](#measure-newtype).

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

* Must support a generic storage type (e.g. `u32`, `u64`, `i32`, `i64`, `f32`, `f64` or any other generic type that implements the necessary traits for arithmetic operations).
* Must support fractional values (e.g. millisecond).
* Must allow to represent a specific unit
  * May embed the unit name in the type name
  * May take the unit as a generic parameter
* Must implement [identity traits](#identity-trait).
* Must implement [addition traits](#addition-trait) for values with the same unit.
* Must not implement [addition traits](#addition-trait) for values with different units (use [compile-fail tests](#compile-fail-test)).
* Must not implement [addition traits](#addition-trait) for values with scalars of any storage type (use [compile-fail tests](#compile-fail-test)).
* Must implement [general multiplication traits](#general-multiplication-trait) for values with the same or different units.
  * Requirements:
    * Must have a `type Output` with a distinct unit that represents a [monomial](#monomial) of input units.
* Must implement [scalar multiplication traits](#scalar-multiplication-trait) for values with scalars of the same storage type.
* Must implement `num_traits::MulAdd` for values where `A` parameter is a unit and `B` parameter is a unit that represents a multiplication of `A` and `Self`
  * Requirements:
    * Must have a `type Output` with a distinct unit that represents a multiplication of `Self` and `A` units.
* Must implement `num_traits::MulAdd` for values where `A` parameter is a scalar and `B` parameter is `Self`
* Must implement `num_traits::MulAddAssign` for values where `A` parameter is a scalar and `B` parameter is `Self`
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

* How to represent [derived units](#derived-unit)?
  * Examples:
    * Newton
    * Square meter
  * Ideas:
    * DUS: Represent them as separate types
    * DUTNP: Represent them as tuples of nested pairs where the first element is the [base unit](#base-unit) and the second element is the power
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
    * DUTFP: Represent them as tuples of flattened pairs where the first element is the [base unit](#base-unit) and the second element is the power
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

## Measure newtype

A [physical type](#physical-type) that takes the [unit](#unit) as a generic parameter.

Requirements:

* Must have a `#[repr(transparent)]` attribute.
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

* Second is a unit of time defined as "the duration of 9,192,631,770 periods of the radiation corresponding to the transition between the two hyperfine levels of the ground state of the cesium-133 atom"
* Meter is a unit of length defined as "the length of the path traveled by light in vacuum during a time interval of 1/299792458 of a second"
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
* `core::ops::Sub`
* `core::ops::SubAssign`
* `num_traits::CheckedAdd`
* `num_traits::CheckedSub`
* `num_traits::SaturatingAdd`
* `num_traits::SaturatingSub`
* `num_traits::WrappingAdd`
* `num_traits::WrappingSub`
* `num_traits::OverflowingAdd`
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
* Rem traits (`core::ops::Rem` and `num_traits::CheckedRem`) should not be implemented for units because it does not produce a unital value

## Scalar multiplication trait

A trait that is either [general multiplication trait](#general-multiplication-trait) or a trait from the following list:

* `num_traits::Pow`
* `core::ops::Rem`
* `num_traits::CheckedRem`
* `core::ops::MulAssign`
* `core::ops::DivAssign`

Notes:

* `MulAssign` and `DivAssign` are scalar-only because their functions return `()`__

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
* Must define all units in src/units.rs (not separate files)
* Must contain the following tests:
  * `add_sub_scalar_failure` (compile-fail)
  * `add_sub_same_unit`
  * `add_sub_different_unit_failure` (compile-fail)
  * `mul_div_scalar`
  * `mul_div_same_unit`
  * `mul_div_different_unit`
  * `newton_eq_kg_m_s2`
    * This test must check that `1 Newton` is equal to `1 Kilogram * 1 Meter / (1 Second * 1 Second)`

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
