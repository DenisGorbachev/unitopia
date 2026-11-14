## Guidelines

### General

You are a senior Rust software architect.

- Think deeply and make detailed plans before writing code.
- Write high-quality, production-ready, generic, reusable code.

#### Principles

Write code that minimizes losses:

- [Avoid data loss](#avoid-data-loss).
- [Minimize hardcoded data](#minimize-hardcoded-data).
- Minimize memory consumption.
- Minimize execution time.

##### Avoid data loss

- Don't use panicking fns (instead, use checked fns that return a `Result`).
- Don't delete data unless spec explicitly requires it.

##### Minimize hardcoded data

- Don't hardcode values (accept args instead).
- Choose carefully between accepting a param and defining a const:
  - Defs:
    - Params are execution details (user may want to change them).
    - Consts are impl details (user would never want to change them).
  - Examples:
    - Params:
      - Cache TTL.
      - Config path.
    - Consts:
      - Table name.
      - Keyspace name.
  - Recommendations:
    - When in doubt, prefer accepting a param instead of defining a const.

#### Development workflow

- Run `mise run fix` before every commit.
- Don't maintain backward compatibility.
- Don't add comments.
- Don't edit files in `.agents`.
- Don't run `git diff` or `git status` solely to review your own work at end of a turn.
- If a later instruction overrides an earlier instruction, follow later instruction.
- If I explicitly ask to update code in a way that deviates from spec, update code, spec.
- If you notice unexpected edits:
  - Then:
    - Keep them.
    - Don't mention them.
- If you see a code item or spec requirement that is unnecessary or incorrect, tell me.
- If you have to apply a workaround:
  - Then:
    - Add a comment next to workaround explaining why it is necessary.
    - Mention workaround in your final report.
- If unexpected behavior impedes progress but is not a blocker (for example: domain is unavailable, program is unavailable, available memory or disk space is too low, cmd runs for an unexpectedly long time or consumes an unexpected amount of resources), mention it in your final msg.
- If task is technically possible but would result in low-quality code:
  - Then:
    - If there is a clearly better alternative, implement it; else: explain why requested code should not be written.
  - Examples:
    - A task to write `impl From<Foo> for Bar` where `Foo` can't actually be infallibly converted to `Bar` (conversion would require calling `unwrap`, which is bad) — in this case, you should write `impl TryFrom<Foo> for Bar` and reply with "Foo can't be infallibly converted to Bar, so I implemented a fallible conversion instead".
    - A task to write a trait impl that only returns an error — in this case, you should not write trait impl but reply with "trait X can't be implemented for Foo because ..."
- If a sentence starts with "Idea: ":
  - Then:
    - Evaluate it thoroughly.
    - If you agree, implement it; else: explain why you didn't implement it and brainstorm solutions.

#### Review workflow

- Output a full list of [findings](#finding) (not a shortlist).
- If there are no findings, start your reply with "No findings".
- For each item in an ordered reply to your review:
  - If item is "+", reconsider finding, apply best fix.
    - Else:
      - If item matches "+ {number}", apply proposed fix {number}.
        - Else:
          - If item is "-", don't apply fixes; else: respond normally (keep `ctid` in your response).
- If no actionable items remain in thread identified by a specific `ctid`, drop this `ctid` from your response.

#### Debugging workflow

- Improve error handling, so that root cause is clearly visible.

#### Subagents

- When spawning a code review subagent: use fresh ctx (not inherited).

#### Cmds

- Use `fd` and `rg` instead of `find` and `grep`.
- Set cmd-execution tool call’s timeout param and `yield_time_ms` to at least 300000 ms for cmds: `mise run fix`, `cargo build`, `git commit`.

#### Recommended crates

- `errgonomic` for error handling.
- `strum` for enum derives.
- `subtype` for newtypes.
- `tempfile` for creating temp dirs or files.

#### Files

- File name must match name of primary item in this file (for example: a file with `struct User` must be named `user.rs`).
- Trait impls must be in same file as target type (for example: put `impl TryFrom<...> for User` in same file as `struct User`, which is `user.rs`).

#### Modules

- Don't use `mod.rs`; use module files with submodules in folder with same name (for example: `user.rs` with submodules in `user` folder).
- When creating a new module, attach it with a `mod` declaration followed by a `pub use` glob declaration. The parent module must re-export all items from child modules. For example:
  ```rust
  fn foo() {}

  mod my_module_name;
  pub use my_module_name::*;
  ```
- Place `mod` and `pub use` declarations at end of file (after code items).
- When importing items that are defined in current crate, import them directly from crate root. For example:
  ```rust
  use crate::foo;
  ```
- Prefer short item paths over long item paths (use a `use` statement), unless it's necessary for disambiguation. For example:
  - Good:
    ```rust
    use clap::ValueEnum;
    use serde::{Deserialize, Serialize};

    #[derive(ValueEnum, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy, Debug)]
    pub enum Side {
        Buy,
        Sell,
    }
    ```
  - Good (`serde` and `rkyv` prefixes are necessary for disambiguation):
    ```rust
    use clap::ValueEnum;

    #[derive(ValueEnum, From, serde::Serialize, serde::Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Hash, Clone, Copy, Debug)]
    pub enum Side {
        Buy,
        Sell,
    }
    ```
  - Bad (`clap` and `serde` prefixes are not necessary for disambiguation because their trait names are unique in this module):
    ```rust
    #[derive(clap::ValueEnum, serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash, Clone, Copy, Debug)]
    pub enum Side {
        Buy,
        Sell,
    }
    ```
- If you need error and result types from `std`, prefer short paths.
  - Examples:
    - `use std::io;`: `io::Result`, `io::Error`.
    - `use std::fmt;`: `fmt::Result`, `fmt::Error`.

#### Visibility

- Items:
  - Prefer `pub` visibility over `pub(crate)` or private visibility.
- Fields:
  - If a struct is a refinement of its fields:
    - Then:
      - Fields must be private.
      - Fns that construct or deserialize struct or mutate its fields must preserve invariant.
    - Else: its fields must be `pub`.

#### Consts

- Define consts only for values used in multiple places (prefer inline values).
- Put consts in `src/constants.rs`.

#### Types

- Always use most specific types (enforce semantic difference through syntactic difference):
  - Use types from existing crates.
    - Use types from `url` crate instead of `String` for URL-related values.
    - Use types from `time` crate instead of `String` or `u64` for datetime-related values.
    - Use types from `phonenumber` crate instead of `String` for phone-related values.
    - Use types from `email_address` crate instead of `String` for email-related values.
    - Use types from `core::num` module that are prefixed with `NonZero` for values that must be non-zero.
  - Search for other existing crates if you need specific types.
  - If you can't find existing crates, define newtypes.
- Every `struct`, `enum`, `union` must be in a separate file (except for error types that implement `Error`).
  - Error types that implement `Error` must be in same files as fns that return them.
- Prefer attaching types as child modules to `src/types.rs`.

#### Fns

- Prefer weakest sufficient trait bound for inputs and associated types (`FnOnce` over `FnMut` over `Fn`, `PartialOrd` over `Ord`, `PartialEq` over `Eq`).
- Implement proper error handling using macros from `errgonomic` crate instead of `unwrap` or `expect` (in normal code and in tests).
  - Use `expect` only in exceptional cases where you can prove that it always succeeds, and provide proof as first arg to `expect` (proof must start with "always succeeds because").
- Prefer streams and iterators:
  - Guidelines for inputs:
    - If fn uses methods that are available only for a specific collection type, prefer taking a specific collection type as input; else: prefer taking an `impl Stream` or `impl IntoIterator` as input.
  - Guidelines for outputs:
    - If fn return type is naturally an iterator (for example: fn returns output of a `map` or `filter`), prefer returning an `impl Iterator` as output (there's no need to collect into `Vec`); else: prefer returning a specific collection type as output.
  - Examples:
    - Good:
      ```rust
      /// Good: fn uses only generic `Iterator` methods.
      /// Good: fn naturally returns an iterator.
      pub fn filter_non_empty_strings<'a>(inputs: impl IntoIterator<Item = &'a str>) -> impl Iterator<Item = &'a str> {
          inputs.into_iter().filter(|i| i.is_empty().not())
      }

      /// Good: `Vec`-specific `extend_from_slice` requires `Vec` input.
      fn extend_args(mut args: Vec<String>, extra_args: &[String]) -> Vec<String> {
          args.extend_from_slice(extra_args);
          args
      }
      ```
    - Bad:
    - ```rust
      /// Bad: needlessly converts a `Vec` into an iterator, then collects into a `Vec`.
      pub fn filter_non_empty_strings(inputs: Vec<&str>) -> Vec<&str> {
          inputs
              .into_iter()
              .filter(|i| i.is_empty().not())
              .collect::<Vec<_>>()
      }

      /// Bad: forces caller to collect strings into a `Vec`, restricting input types and adding allocations.
      pub fn bar(inputs: Vec<String>) -> Vec<String> {}
      ```
- Prefer implementing and using `From` or `TryFrom` for conversions between types (instead of converting in-place).
- Don't use early-return fast-path guards for empty vectors, iterators, streams (i.e.: don't use `if items.is_empty() { return ...; }`).
- Use destructuring assignment for tuple args, for example: `fn try_from((name, parent_key): (&str, GroupKey)) -> ...`.
- Use iterators instead of for loops. For example:
  - Good:
    ```rust
    use errgonomic::{handle_iter, ErrVec};
    use core::num::ParseIntError;
    use thiserror::Error;

    // Good: iterator pipeline with fallible mapping and error handling.
    pub fn parse_numbers(inputs: impl IntoIterator<Item = impl AsRef<str>>) -> Result<Vec<u64>, ParseNumbersError> {
        use ParseNumbersError::*;
        let iter = inputs.into_iter().map(|s| s.as_ref().trim().parse::<u64>());
        Ok(handle_iter!(iter, InvalidInput))
    }

    #[derive(Error, Debug)]
    pub enum ParseNumbersError {
        #[error("failed to parse {len} numbers", len = source.len())]
        InvalidInput { source: ErrVec<ParseIntError> },
    }
    ```
  - Bad:
    ```rust
    use core::num::ParseIntError;

    // Bad: manual loop and mutable accumulator.
    pub fn parse_numbers(inputs: impl IntoIterator<Item = impl AsRef<str>>) -> Result<Vec<u64>, ParseIntError> {
        let mut out = Vec::new();
        for s in inputs {
            let n = s.as_ref().trim().parse::<u64>()?;
            out.push(n);
        }
        Ok(out)
    }
    ```
- If fn has a clear receiver (`self`, `&self`, `&mut self`), implement it as an associated fn; else: implement it as a standalone free fn.
- Add a local `use` statement for enums to minimize code size. For example:
  - Good:
    ```rust
    pub fn apply(op: GroupsOp) {
        use GroupsOp::*;
        match op {
            InsertOne(_) => {}
            UpdateOne(_, _) => {}
            DeleteOne(_) => {}
        }
    }
    ```
  - Bad:
    ```rust
    pub fn apply(op: GroupsOp) {
        match op {
            GroupsOp::InsertOne(_) => {}
            GroupsOp::UpdateOne(_, _) => {}
            GroupsOp::DeleteOne(_) => {}
        }
    }
    ```
- Simplify call-site code by accepting `impl Into`. For example:
  - Good:
    ```rust
    pub fn foo(input: impl Into<String>) {
        let input = input.into();
    }
    ```
  - Bad:
    ```rust
    /// Bad: caller may have to call `.into()` when passing input arg.
    pub fn foo(input: String) {}
    ```
- Provide additional flexibility at call sites by accepting `&impl AsRef` or `&mut impl AsMut` (e.g.: both `PathBuf` and `Config` may implement `AsRef<Path>`). For example:
  - Good:
    ```rust
    pub fn bar(input: &mut impl AsMut<String>) {
        let input = input.as_mut();
    }

    pub fn baz(input: &impl AsRef<str>) {
        let input = input.as_ref();
    }
    ```
  - Bad:
    ```rust
    /// Bad: caller may have to call `.as_mut()` when passing input arg.
    pub fn bar(input: &mut String) {}

    /// Bad: caller may have to call `.as_ref()` when passing input arg.
    pub fn baz(input: &str) {}
    ```
- Prefer `.map()` instead of `match` when you need to modify value in `Option` or `Result`. For example:
  - Good:
    ```rust
    use core::str::FromStr;
    use core::num::ParseIntError;

    impl FromStr for UserId {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            s.parse::<u64>().map(Self::new)
        }
    }
    ```
  - Bad:
  ```rust
  use core::str::FromStr;
  use core::num::ParseIntError;

  impl FromStr for UserId {
      type Err = ParseIntError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
          // Bad: uses more code to express same idea.
          match s.parse::<u64>() {
              Ok(value) => Ok(Self::new(value)),
              Err(error) => Err(error),
          }
      }
  }
  ```
- Use `Self` instead of type name in `impl` items. For example:
  - Good:
  ```rust
  use core::time::Duration;

  impl From<Duration> for UnixTimestamp {
      #[inline]
      fn from(duration: Duration) -> Self {
          Self::new(duration.as_secs())
      }
  }
  ```
  - Bad:
  ```rust
  use core::time::Duration;

  impl From<Duration> for UnixTimestamp {
      #[inline]
      fn from(duration: Duration) -> Self {
          UnixTimestamp::new(duration.as_secs())
      }
  }
  ```
- Prefer short method syntax over long trait-path syntax (prefer `value.method(arg)` over `Trait::method(value, arg)`).
- Generic helper fns must be in `src/functions` (one file per fn).
- If Clippy reports `too_many_arguments`:
  - Then:
    - Don't use tuples to silence lint.
    - Consider refactoring code for better separation of concerns.

#### Struct derives

- Derive `new` from `derive_new` crate for types that need `fn new`.
- If struct derives `Getters`:
  - Then:
    - For each field:
      - If its type implements `Copy`, it must have a `#[getter(copy)]` annotation.
  - Examples:
    - Good (`username` has no `#[getter(copy)]` because `String` doesn't implement `Copy`, but `age` has it because `u64` implements `Copy`):
      ```rust
      #[derive(Getters, Into, Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
      pub struct User {
        username: String,
        #[getter(copy)]
        age: u64,
      }
      ```

#### Setters

- Use setters that take `&mut self` instead of setters that take `self` and return `Self` (because passing `foo: &mut Foo` is more efficient than passing `foo: Foo` and returning `Foo` through call stack).

#### Enums

- When writing code related to enums, bring variants into scope with a `use Enum::*;` statement at top of file or fn (prefer top of file for data enums and top of fn for error enums).

#### Arithmetic

- Don't use impls of traits `core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign, Neg, Shl, ShlAssign, Shr, ShrAssign}` or their operators unless they don't panic or silently overflow.
- Write and use arithmetic trait impls that don't panic or silently overflow.
- Prefer `checked` versions of arithmetic ops.
- Every call to an `overflowing`, `saturating`, `wrapping` version must have a single-line comment above it that starts with "SAFETY: " and describes why calling this version is safe in this specific case.
- Use items from `num` crate if necessary (for example: to implement a fn that calls arithmetic methods on a generic type).

#### Index access

- Never use operators: `[], []=`.
- Never use traits: `core::ops::{Index, IndexMut}`.
- If you are sure that `get` or `get_mut` will never panic, use `expect` with a proof msg (as described in [Fns](#fns)).

#### Test fn

A fn marked with `#[test]` or `#[tokio::test]`.

- Must return a `Result`.
- Must implement proper error handling via `errgonomic` crate.
- Should use macros from `assertables` crate.
  - Should use `assert_infix` instead of `assert_gt`, `assert_ge`, `assert_lt`, `assert_le`, `assert_eq`.

#### Macros

- In `macro_rules!` expansions:
  - Use full paths without a leading `::` for foreign types (e.g.: `std::sync::Arc`), except those in standard prelude.
  - Use `$crate` paths for types defined in this crate (e.g.: `$crate::MyType`).
- If you see similar code in different places, replace similar code with a shared macro.
- If last arg of a macro arm is variadic (e.g.: `$($item:ident),*`), add a trailing `$(,)?`; else: don't add a trailing `$(,)?`.

#### Shell

- Prefer short options for cmds used in tool calls.

#### mise.toml

- Each `env` map value:
  - If it should be overridable by the user:
    - Then: it must be a map with `default` key (e.g. `PORT = { default = "3000" }`)
    - Else: it must be a specific value (e.g. `SHUCK_CACHE_DIR = "{{config_root}}/.cache"`)

#### Cargo.toml

- Don't define pkg features with only a single optional dep (such features are already defined by Cargo automatically).
- Use `cargo add` to add deps.
- When adding a dep from crates.io:
  - If pkg is [publishable](#publishable-pkg):
    - Then:
      - Run `cargo add {dependency}@{version}`.
        - Patch component of `{version}` must be 0.
      - Try `cargo update -p {dependency} --precise {version}` to lock that exact version.
        - If dep constraints prevent locking that version:
          - Then:
            - Keep version resolved by Cargo.
            - Add a comment in `Cargo.toml` explaining constraints.
    - Else:
      - Run `cargo add {dependency}` without `{version}`.
- When adding a dep in a workspace:
  - Add it to top-level manifest first (`workspace.dependencies`).
- When adding a dep for a workspace member:
  - Run `cargo add {dependency}` without `{version}` (Cargo will set `workspace = true`).
- When adding a new workspace member: add it to `packages` dir unless specified otherwise.

#### Code style

- Don't enforce a line len limit when writing code, comments, docs.

#### Chat thread id

- Must be a string.
- Must contain at least 3 characters.
- Must contain only uppercase characters.

Examples:

- `RVC`.
- `AKE`.
- `LMY`.

Notes:

- Should match thread topic.

#### Chat thread id heading

A level-3 Markdown heading that contains only a [chat thread id](#chat-thread-id).

Examples:

- `### RVC`.
- `### AKE`.
- `### LMY`.

#### Finding

- Must be formatted as `### {ctid}\n\n[{priority}] {title}. {body} ({references}). Proposed fixes: {fixes}`.
  - `ctid` must be a [chat thread id](#chat-thread-id).
  - `priority` must be one of `P0`, `P1`, `P2`, `P3`.
  - `references` must be a comma-separated list of `reference` values.
  - `reference` must be formatted as `{path}:{line}`.
  - `path` must be a file path relative to your working dir.
  - `line` must be first line of relevant code or text block.
  - `fixes` must be one of:
    - If there is at least one proposed fix, "\n\n" and a nested Markdown list of fixes where each fix must have format `{number}. {description}` (numbers should start from 1 for each list of fixes); else: exact text "none."

#### Publishable pkg

A pkg that has a remote whose name contains `public` or `pre-public` and ends with `template`.

### Project concepts

#### This document

A specification for [`unitopia`](#unitopia).

Notes:

- Some list items have a format "{id}: {text}" where id is a string in CAPS (you can use the id to refer to the list item)

#### `unitopia`

A Rust workspace that provides multiple patterns of implementations of quantity value types.

Requirements:

- Must use the [physical definitions set](#physical-definitions-set)
- Must keep its members in `packages` dir
- Must contain at least one member package that exports a [quantity value type](#quantity-value-type)
- Every member must use macros to avoid boilerplate code when defining units and prefixes
- Every member must define the macros in `src/macros.rs`
- Every member must have the following crate-level attributes in `src/lib.rs`:
  - `#![no_std]`
  - `#![forbid(unsafe_code)]`
- Must use US English spelling

Allowances:

- May implement banned traits but must not call their methods and must not use operators that desugar to their methods
  - Rationale:
    - Some dependents may prefer to use the banned traits, so we should provide the implementations
    - The traits are banned because their implementations may panic or silently break the underlying assumptions, so we should not call their methods

Notes:

- The adjective "unital" indicates that a corresponding noun is related to a [unit](#unit).

Design choices:

- Kind
  - Unit
  - Prefix
- Archetype
  - Value
  - Vanilla marker struct
  - Generic marker struct with only a single argument
  - Wrapper struct

Constants:

| Name      | Value                                                         | Notes                                              |
|-----------|---------------------------------------------------------------|:---------------------------------------------------|
| u128::MAX | 340282366920938463463374607431768211455                       |                                                    |
| 10^38     | 100000000000000000000000000000000000000                       | Largest power of 10 representable by u128          |
| 10^60     | 1000000000000000000000000000000000000000000000000000000000000 | Largest conversion factor in SI (quetta to quecto) |

Prefix conversion notes:

- It is possible but not yet necessary to implement an `fn conversion_succeeds<Src: Scale + Bounded, Dst: Scale + Bounded>() -> Option<bool>`:
  - Return values:
    - `Some(true)` if it will always succeed
    - `Some(false)` if it will always fail
    - `None` if it will succeed or fail depending on the actual values at runtime
  - Details:
    - Some conversions will always succeed
      - Example groups:
        - Conversions where the source inner type is smaller than the target inner type
          - Examples:
            - `Kilo<u32>` into `Kilo<u64>`
        - Conversions where the source prefix is larger than target prefix but the target inner type is large enough to hold the largest source value
          - Examples:
            - `Kilo<u32>` into `Uno<u128>` (`u32::MAX * 10 ^ (3 - 0) < u128::MAX`)
            - `Kilo<u32>` into `Deci<u128>` (`u32::MAX * 10 ^ (3 - 1) < u128::MAX`)
            - `Kilo<u32>` into `Kilo<u64>` (`u32::MAX * 10 ^ (3 - 3) < u64::MAX`) (a special case already covered by previous example group)
    - Some conversions will always fail
    - Some conversions will succeed or fail depending on the actual values at runtime
  - Notes:
    - The min and max values should have `rust_decimal::Decimal` type to avoid precision loss
    - `Bounded` is a trait from `num_traits`

Blockers:

- B001: Generic wrapper structs cannot implement `Into<T>` for their inner type without conflicting with `core`'s blanket `Into` impl (`impl<T, U> Into<U> for T where U: From<T>`), and `From<Wrapper<T>> for T` is also forbidden by orphan/coherence rules. Workarounds:
  - Define a special trait that would have a conversion method (for example, `fn into_inner(self) -> T`) and implement it for generic wrappers.
  - Keep `From<T> for Wrapper<T>` and use the blanket `Into<Wrapper<T>> for T` for the construction direction.
  - Implement `Into<Inner>` only for non-generic concrete wrapper types where coherence allows it.

#### `unitopia-helpers`

A [`unitopia`](#unitopia) member package that exports various helpers.

Requirements:

- Must export the following macros:
  - `define_strict_wrapper_struct`
- Must export a `Scalar<T>` type (a [strict open wrapper struct](#strict-open-wrapper-struct) for scalars)
  - Notes:
    - Needed to implement `Mul`, `Div`, `MulAdd` in a generic way while satisfying Rust coherence rules
- Must export a `HasScale` trait:
  - Requirements:
    - Must have a `const NUM: u128; // numerator`
    - Must have a `const DEN: u128; // denominator`
- Must export the [linearity marker traits](#linearity-marker-trait)

#### `unitopia-test-helpers`

A [`unitopia`](#unitopia) member package various helpers for a [quantity value test package](#quantity-value-test-package).

TODO:

- The testing approach that uses a custom language can't do compile-fail tests
  - Or can it? Can we ask the TestContext implementor package to produce a valid Rust code?

Requirements:

- Must export the following functions:
  - `parse_fractional_unit` ((1, Milli, Second))
  - `add_sub_scalar_failure` (compile-fail)
  - `add_sub_same_unit`
  - `add_same_quantity_unit`
  - `sub_same_quantity_unit`
  - `add_different_unit_failure` (compile-fail)
  - `sub_different_unit_failure` (compile-fail)
  - `mul_div_scalar`
  - `mul_div_same_unit`
  - `mul_div_different_unit`
  - `newton_eq_kg_m_s2`
    - This test must check that `1 Newton` is equal to `1 Kilogram * 1 Meter / (1 Second * 1 Second)`

#### `unitopia-marker-quantities`

A [`unitopia`](#unitopia) member [quantity package](#quantity-package) that exports physical quantities implemented as [marker structs](#marker-struct).

#### `unitopia-marker-units`

A [`unitopia`](#unitopia) member [unit package](#unit-package) that exports physical units implemented as [marker structs](#marker-struct).

#### `unitopia-marker-prefixes`

A [`unitopia`](#unitopia) member [prefix package](#prefix-package) that exports prefixes implemented as [marker structs](#marker-struct).

#### `unitopia-marker-arith-outputs`

A [`unitopia`](#unitopia) member package that exports the following [marker structs](#marker-struct):

- `Prod<A, B>`
- `Quot<A, B>`
- `Powr<A, N>`
- `Summ<A>`
- `Diff<A>`

Requirements:

- Must define all types in `src/lib.rs`

#### `unitopia-open-wrapper-arith-outputs`

A [`unitopia`](#unitopia) member package that exports the following [OWS](#open-wrapper-struct)

- `Prod<A, B, T>`
- `Quot<A, B, T>`
- `Powr<A, N, T>`

Requirements:

- Every arith output type must have a `pub inner: T` field (`T` is the storage type)
- Must define all types in `src/lib.rs`

#### `unitopia-strict-wrapper-prefixes`

A [`unitopia`](#unitopia) member [prefix package](#prefix-package) that exports prefixes implemented as [strict open wrapper structs](#strict-open-wrapper-struct) whose `T` represents a unit.

Requirements:

- Must implement the same traits as for units.

Notes:

- Example usage:
  - `Milli<Second<u64>>`

#### `unitopia-strict-wrapper-units`

A [`unitopia`](#unitopia) member [unit package](#unit-package) that exports physical units implemented as [strict open wrapper structs](#strict-open-wrapper-struct) whose `T` represents a generic storage type.

- Must define, export, use the following macros:
  - `define_strict_wrapper_unit`
    - Must use `define_strict_wrapper_struct`
- Must use the types from `unitopia-open-wrapper-arith-outputs` to implement the [general multiplication traits](#general-multiplication-trait)

#### `unitopia-measure-draft`

A [`unitopia`](#unitopia) member package that exports a `Measure` type.

#### `unitopia-runtime-unit-quantity-value`

A [`unitopia`](#unitopia) member [generic quantity value package](#generic-quantity-value-package) that exports [`QuantityValue`](#quantityvalue).

Requirements:

- Must define all types in `src/lib.rs`
- Must have an optional `serde` feature:
  - If enabled: `QuantityValue` must derive `Serialize` and `Deserialize`.

#### `unitopia-measurement-draft`

A [`unitopia`](#unitopia) member package that exports a `Measurement` [FGQVT](#fully-generic-quantity-value-type).

Requirements:

- Must use the `Unit` and `Quantity` traits from `unitopia-helpers`
- Must use the types from `unitopia-marker-prefixes` and `unitopia-marker-units` in the tests
- Must implement conversions from and into types exported from the foreign timekeeping crates (see [timestamp-please project doc](#timestamp-please-project-doc))
  - Must not lose data on conversions:
    - Must implement conversions into foreign timekeeping types only for those unit types and storage types that are natively supported by the foreign timekeeping types
    - Must implement conversions from foreign timekeeping types via `IntoMeasurement` trait with `into_measurement` method that may return a tuple of `Measurement` values
      - Rationale: this is necessary to correctly convert types that stores values of different units in the same type (e.g. `core::time::Duration` which stores both seconds and nanoseconds)
      - Requirements:
        - The storage types used in the implementation `Output` type must be at least as large as the storage types used in the implementor (so that conversion is lossless)

```rust
pub trait IntoMeasurement {
    type Output;

    fn into_measurement(self) -> Self::Output;
}
```

#### Package metric

A value assigned to a specific code package.

Examples:

- Speed of calculations
- Min amount of memory used
- Size of code that constructs values
- Size of debug representation
- Scalability of trait implementations
- Ability to define custom units
- Ability to define custom prefixes (rational coefficients) for units

Notes:

- Sizes must be calculated for a pair of values with the largest and smallest SI prefix

#### Physical type

A newtype that represents a physical measurement outcome.

Requirements:

- Must implement [identity traits](#identity-trait).
- Must implement [addition traits](#addition-trait) for values with the same quantity (with same or different units) if this unit implements the `Linear` [linearity marker trait](#linearity-marker-trait).
  - Requirements:
    - Every addition operation that returns a value must return a quantity value whose quantity is `Summ<Q>` (where `Q` is the input quantity type).
    - `Summ` must have exactly one generic argument.
- Must implement [subtraction traits](#subtraction-trait) for values with the same unit and quantity if this unit implements the `Linear` [linearity marker trait](#linearity-marker-trait).
  - Requirements:
    - Every subtraction operation that returns a value must return a quantity value whose quantity is `Diff<Q>` (where `Q` is the input quantity type).
    - `Diff` must have exactly one generic argument.
- Must not implement [addition traits](#addition-trait) for values with different quantities (use [compile-fail tests](#compile-fail-test)).
- Must not implement [addition traits](#addition-trait) for values with scalars of any storage type (use [compile-fail tests](#compile-fail-test)).
- Must not implement [subtraction traits](#subtraction-trait) for values with different units (use [compile-fail tests](#compile-fail-test)).
- Must not implement [subtraction traits](#subtraction-trait) for values with different quantities (use [compile-fail tests](#compile-fail-test)).
- Must not implement [subtraction traits](#subtraction-trait) for values with scalars of any storage type (use [compile-fail tests](#compile-fail-test)).
- Must implement [general multiplication traits](#general-multiplication-trait) for values with the same or different units.
  - Requirements:
    - Must have a `type Output` with a distinct unit that represents a [monomial](#monomial) of input units.
- Must implement [scalar multiplication traits](#scalar-multiplication-trait) for values with scalars of the same storage type.
- Must implement `num_traits::MulAdd` for values where `A` parameter is a unit and `B` parameter is a unit that represents a multiplication of `A` and `Self` if `B` implements the `Linear` [linearity marker trait](#linearity-marker-trait).
  - Requirements:
    - Must have a `type Output` with a distinct unit that represents a multiplication of `Self` and `A` units.
- Must implement `num_traits::MulAdd` for values where `A` parameter is a scalar and `B` parameter is `Self` if `Self` implements the `Linear` [linearity marker trait](#linearity-marker-trait).
- Must implement `num_traits::MulAddAssign` for values where `A` parameter is a scalar and `B` parameter is `Self` if `Self` implements the `Linear` [linearity marker trait](#linearity-marker-trait).
- Must implement serialization/deserialization traits from the popular crates (feature-gated):
  - `serde`
  - `rkyv`
  - `bitcode`
- Must implement checked, wrapping, overflowing, saturating arithmetic operation traits from `num-traits`
- Must use methods instead of operators in trait implementations
  - Examples:
    - Use `add_assign` instead of `+=`

Preferences:

- Should derive the serialization/deserialization traits instead of implementing them manually.
- Should provide `From` or `TryFrom` implementations for similar types from other crates:
  - Examples:
    - `time` provides types that represent nanoseconds
    - `chrono` provides types that represent nanoseconds
    - `core` provides `Duration` that represents nanoseconds
  - Notes:
    - Some crates provide types that represent measurement deltas, not just measurements
      - Examples:
        - `core` provides `Duration`
    - Some types are measurements with a custom offset
      - Examples:
        - Timestamp is a measure of time with a custom offset (UNIX epoch)
- Should follow SI
  - Notes:
    - This is a preference, not a requirement

Open questions:

- How to represent [SI derived units](#si-derived-unit)?
  - Examples:
    - Newton
    - Square meter
  - Ideas:
    - DUS: Represent them as separate types
    - DUTNP: Represent them as tuples of nested pairs where the first element is the [SI base unit](#si-base-unit) and the second element is the power
      - Examples
        - `pub type Newton = ((Kilogram, P1), (Meter, P1), (Second, N2));`
      - Properties:
        - Semantically same are syntactically same: No
          - Counterexamples:
            - `pub type Newton2 = ((Meter, P1), (Kilogram, P1), (Second, N2));`
          - Notes:
            - This can be mitigated by convention:
              - Document a specific order of units as canonical
              - Implement arithmetic traits in a way that `type Output` has a canonical order of units
    - DUTFP: Represent them as tuples of flattened pairs where the first element is the [SI base unit](#si-base-unit) and the second element is the power
      - Examples:
        - `pub type Newton = (Kilogram, P1, Meter, P1, Second, N2);`
      - Pros:
        - Less code
      - Cons: (I don't see any, but it puts the units and powers on the same level, so maybe some cons will be discovered during implementation)
    - DUIM: Represent them as specifications of `Mul` type only (represent `Div` as `Mul` with negative power) (e.g. `type Newton = Mul<Mul<Kilogram, P1, Meter, P1>, Mul<Second, P1, Second, P1>, P1, N1>;`)
      - Superseded by DUTNP and DUTFP
    - DUIMD: Represent them as specifications of `Mul` or `Div` generic types
      - Examples:
        - `pub type Newton = Div<Mul<Kilogram, Meter>, Mul<Second, Second>>;`
      - Notes:
        - Superseded by DUIM
        - This makes the units which are semantically equivalent syntactically different
          - Examples:
            - `Mul<Kilogram, Meter>` and `Mul<Meter, Kilogram>`
          - Solutions:
            - Provide an `invert_unit` method for measures with `Mul` unit
            - Switch to runtime check
            - Represent all units in a system with a single type whose generic parameters are unit powers
              - May use `typenum` crate
    - Notes:
      - The examples use types from `typenum`
- How to represent alternative units that are a constant multiple of base units?
  - Examples:
    - Minute is 60 * Second
    - Millisecond is 0.001 * Second
    - Foot is 0.3048 * Meter (since an international agreement in 1959)
  - Notes:
    - Using SI prefixes is not sufficient because the alternative units may have arbitrary coefficients (see the "Foot" example)
  - Ideas:
    - CMS: Represent them as a separate type
    - CMBU: Represent them as a base unit type, but use a [rational type](#rational-type) for the value (put the scale in the value)
    - CMTT: Represent them as type that encodes the scale information in the type itself using `typenum` (see example: packages/unitopia-draft-scale/src/scale.rs)
- How to convert between values of the same unit but different prefix?
  - Examples
    - `Second<Milli<u32>>` and `Second<Atto<f64>>`
  - Ideas:
    - Require the user to perform this conversion

Notes:

- The ideas are not requirements. If you think that an idea is wrong, tell me about it and don't implement it.
- Stable Rust 1.85 supports arithmetic in const generics only for constant operands wrapped in braces (e.g. `Foo::<{ 2 + 2 }>::new(42)`)
- Some units are non-linear (example: pH)
  - If a package implements units as distinct types:
    - Then: it's possible to implement traits selectively, so non-linear units must not implement [scalar multiplication traits](#scalar-multiplication-trait)
    - Else: it's not possible to implement traits selectively, so non-linear units must implement [scalar multiplication traits](#scalar-multiplication-trait) as a consequence of generic implementations

#### Base terms

Examples (exhaustive):

- Dimension
- Unit
- Quantity

#### Physical definitions set

A set of definitions for [base terms](#base-terms) used in theoretical physics.

#### Metrological definitions set

A set of definitions for [base terms](#base-terms) used in metrology.

Notes:

- This set is used by BIPM that publishes SI
- This set is used by OIML that publishes VIM

#### Dimension

One of:

- [Dimension (mathematics)](#dimension-mathematics)
- [Dimension (physics)](#dimension-physics)

#### Dimension (mathematics)

A number of degrees of freedom.

Examples:

- The dimension of a vector space is the number of coordinates necessary to specify any vector.

Notes:

- The use of the word "dimension" to mean a specific coordinate axis (e.g. "time dimension") is informal and mathematically incorrect.

#### Dimension (physics)

A [monomial](#monomial) of base [quantity kinds](#quantity-kind).

Examples:

- `1` - a monomial where every quantity kind has a power of 0
- `M` - a monomial where mass quantity kind has a power of 1 and all other quantity kinds have a power of 0 (represents the mass itself)
- `M * L * T^-2` - a monomial where mass quantity kind has a power of 1, length quantity kind has a power of 1, time quantity kind has a power of -2, all other quantity kinds have a power of 0 (represents force)

Notes:

- The list of base quantity kinds is determined by the metric system.

#### Dimensional (physics)

A [dimension (physics)](#dimension-physics) which is not reduced according to algebraic rules (remains the same expression as determined by the formula).

Examples:

- `L^+1 * L^-1` - a dimensional of radian
- `L^+2 * L^-2` - a dimensional of steradian

Notes:

- This definition is non-standard, but it allows to distinguish between truly dimensionless quantities produced by pure counting (e.g. amount of substance, count of oscillations) and the quantities that appear to be dimensionless but actually are calculated from other quantities which are not dimensionless (e.g. radian, steradian).
- Every dimension is a valid dimensional.
  - `1` is a special dimensional for pure counts (e.g. amount of substance, count of oscillations)

#### Quantity kind

A kind of a property that can be quantified by measurement.

Examples:

- Time
- Length
- Mass
- Speed
- Force
- Energy
- Volume
- Heat
- Angle
- Information

Notes:

- Some quantity kinds are non-physical (e.g. information, specific currency)
- Some quantity kinds are [dimensionless](#dimension-physics) (e.g. angle, bit)
- Some quantity kinds are derived (e.g. force is `mass * length / (time * time)`)

#### Quantity kind group

A group of related quantity kinds.

Examples:

- Physics: Length, Mass, Time.
- Currency: U.S. Dollar, Bitcoin, Rai.
- Other: Information.

#### Quantity

A property that can be quantified by measurement.

Synonyms: Measurand.

Examples:

- Q001: Height of a human measured at under specific physical conditions (time, location, temperature) and posture (head in Frankfort horizontal plane)
- Q002: Height of a building measured at under specific physical conditions (time, location, temperature)
- Q003: Width of a human from shoulder tip to shoulder tip measured at under specific physical conditions (time, location, temperature)
- Q004: Balance of U.S. Dollar on a specific bank account

Notes:

- Some quantities are of the same quantity kind
  - Examples:
    - Q001 and Q002 are quantities of "Length" kind
    - "Width", "Height", "Depth" are quantities of "Length" kind.
- It is possible to make the quantity more precise by fixing other properties that may affect the measurement (e.g. time, location, temperature)

#### Quantity value

A pair of a value and a unit.

Examples:

- 1 hour
- 3 kilograms
- 12 newtons

#### Measure

A tuple of a value, a unit, a quantity.

Examples:

- 74.55 kg is the mass of a person identified by passport number 98882348 issued by Russian Federation, recorded at 2024-08-02T09:26:13Z on scales identified by inventory number 1238343.

Note:

- VIM uses the word "measure" to mean an instrument, but in common English "measure" can mean both the instrument and the output, and we need a short word because we'll use this definition often.

#### SI base quantity kind

A [quantity kind](#quantity-kind) that is designated as foundational in the International System of Units (SI).

Examples (exhaustive):

- Time
- Length
- Mass
- Electric current
- Thermodynamic temperature
- Amount of substance
- Luminous intensity

#### SI derived quantity kind

A [quantity kind](#quantity-kind) that is expressed as a [monomial](#monomial) of other SI quantities.

Examples:

- Speed
- Force
- Energy
- Volume

#### Unit

A magnitude of a [quantity](#quantity).

Examples:

- Second is a unit of time defined as "the duration of 9,192,631,770 periods of the radiation corresponding to the transition between the two hyperfine levels of the ground state of the cesium-133 atom"
- Meter is a unit of length defined as "the length of the path traveled by light in vacuum during a time interval of 1/299792458 of a second"
- Mole is a unit of amount of substance defined as exactly 6.02214076 * 10^23.
- Bit is a unit of information (doesn't have a physical definition because the amount of information representable by a physical object is inherently subjective to a specific observer - it depends on how many states of a physical object a specific observer can distinguish).
- Radian is a unit of angle defined as the angle subtended at the center of a plane circle by an arc that is equal in length to the radius.

Non-examples:

- Year is not a unit of time because different years have different numbers of days.
- Month is not a unit of time because different months have different numbers of days (and Feb has a different number of days depending on the year).

Notes:

- Unit does not uniquely determine the quantity kind
  - Examples
    - Pascal is associated both with pressure and stress
- Unit does not uniquely determine the [dimension (physics)](#dimension-physics)
  - Examples
    - Radian and steradian are units of different quantity kinds but same dimension `1` (i.e. they are "dimensionless")
- Unit does uniquely determine the [dimension (physics)](#dimension-physics) for raw (non-reduced) monomials:
  - Examples:
    - Radian and steradian have different raw monomials
      - Radian is `L^+1 * L^-1`
      - Steradian is `L^+2 * L^-2`
- The units of the same [quantity kind](#quantity-kind) can be converted between each other.
  - Examples:
    - Meter and foot.
    - Radian and degree.
- Spelling of the metric unit for length:
  - "Meter" in the US and the Philippines
  - "Metre" in other English-speaking nations

#### SI base unit

A [unit](#unit) of a [SI base quantity kind](#si-base-quantity-kind).

#### SI derived unit

A [unit](#unit) of a [SI derived quantity kind](#si-derived-quantity-kind).

#### Custom unit

A [unit](#unit) that is not a part of SI.

Examples:

- Galactosidase Activity Unit (GaIU) defined as "the amount of α-galactosidase that releases 1 micromole (1 µmol) of p-nitrophenol per minute from a synthetic substrate (commonly p-nitrophenyl-α-D-galactopyranoside), under specified assay conditions (temperature and pH)".
- Power of hydrogen (pH) defined as "−log10 (a_H+), where a_H+ is the activity of hydrogen (1+) ions in solution"

#### Prefix

A name of a rational number acts as a coefficient for a [unit](#unit).

Examples:

- Kilo (1000 / 1)
- Giga (1000000000 / 1)
- Uno (1 / 1)
- Micro (1 / 1000000)
- Hexagesi (60 / 1)
- Tetravigesi (24 / 1)
- Quetta (10 ^ 30 / 1)
- Quecto (1 / 10 ^ 30)
- Ronto (1 / 10 ^ 27)

Notes:

- A prefix may be a part of a base unit name (e.g. kilogram)
- All SI prefixes have numerators or denominators that fit in `u128`
  - The conversion may still overflow, so use `Checked` versions of traits

#### SI prefix

A [prefix](#prefix) that is a part of SI.

#### Custom prefix

A [prefix](#prefix) that is not a part of SI.

Examples:

- Hexagesi (60 / 1)
- Tetravigesi (24 / 1)

#### Monomial

An algebraic expression which is a multiplication of a set of variables raised to specific powers.

Examples:

- `m^2` (square meter, unit of area)
- `kg * m * s^-2` (newton, unit of force)

Notes:

- Some variables may have power = 0, so they may be omitted.
- Division is represented by negative powers

#### Rational type

A Rust type that can represent rational numbers.

Examples:

- `f32`
- `f64`
- `rust_decimal::Decimal`
- `num::rational::Ratio`

Non-examples:

- `u32`
- `u64`

Notes:

- Some rational types are lossy (e.g. `f32`, `f64`)

#### Compile-fail test

A Rust file that is expected to fail to compile.

Requirements:

- Must be in a directory that is not used by cargo by default (e.g. `tests/compile_fail`)

Notes:

- Such files are used by `trybuild` to assert that they actually fail to compile.

#### Identity trait

A trait from the following list:

- `num_traits::ConstZero`
- `num_traits::ConstOne`
- `num_traits::Zero`
- `num_traits::One`

#### Addition trait

A trait from the following list:

- `core::ops::Add`
- `core::ops::AddAssign`
- `num_traits::CheckedAdd`
- `num_traits::SaturatingAdd`
- `num_traits::WrappingAdd`
- `num_traits::OverflowingAdd`

#### Subtraction trait

A trait from the following list:

- `core::ops::Sub`
- `core::ops::SubAssign`
- `num_traits::CheckedSub`
- `num_traits::SaturatingSub`
- `num_traits::WrappingSub`
- `num_traits::OverflowingSub`

#### General multiplication trait

A trait from the following list:

- `core::ops::Mul`
- `core::ops::Div`
- `num_traits::CheckedMul`
- `num_traits::CheckedDiv`
- `num_traits::SaturatingMul`
- `num_traits::WrappingMul`
- `num_traits::OverflowingMul`

Notes:

- The following traits don't exist because they don't make sense for division:
  - `num_traits::SaturatingDiv`
  - `num_traits::WrappingDiv`
  - `num_traits::OverflowingDiv`
- Units must have implementations of `core::ops::Rem` and `num_traits::CheckedRem` whose `Rhs` is a scalar
- Units must not have implementations of `core::ops::Rem` and `num_traits::CheckedRem` whose `Rhs` is a unit

#### Scalar multiplication trait

A trait that is either [general multiplication trait](#general-multiplication-trait) or a trait from the following list:

- `num_traits::Pow`
- `core::ops::Rem`
- `num_traits::CheckedRem`
- `core::ops::MulAssign`
- `core::ops::DivAssign`

Notes:

- `MulAssign` and `DivAssign` are scalar-only because their functions return `()`

#### Quantity package

TODO

#### Unit package

A Rust package that exports [unit](#unit) types.

Requirements:

- Must define all base units from SI.
- Must define at least the following non-SI units:
  - `PowerOfHydrogen`
  - `GalactosidaseActivityUnit`
- Must define at least the following derived units as type aliases (as compositions of base units and other types):
  - `Area`
  - `Newton`
  - `Volt`
- Must keep the unit definitions in src/units.rs (not separate files)

#### Prefix package

A Rust package that exports [prefix](#prefix) types.

Requirements:

- Must export all [SI prefixes](#si-prefix)
- Must export all [custom prefixes](#custom-prefix) listed in examples
- Must define all prefixes in src/prefixes.rs (not separate files)
- Must contain the following tests:
  - `mul_giga_scalar_is_giga`
  - `add_giga_nano_is_nano`
    - Must construct `large` as 1 gigameter from `1u128`.
    - Must construct `small` as 1 nanometer from `1u128`.
    - Must `assert!(large > small);`
    - Must calculate `let sum = large + small;`
    - Must have a type annotation on `sum` that contains the nano unit
    - Must `assert!(sum > large);`
    - Must `assert!(sum > small);`
    - Must calculate `let diff = sum - small;`
    - Must have a type annotation on `diff` that contains the nano unit
    - Must `assert!(diff < sum);`
    - Must `assert!(diff > small);`
    - Must calculate `(diff_large, diff_large_remainder)` by converting it to a value with giga prefix
    - Must `assert!(diff_large_remainder.is_zero());`
    - Must `assert_eq!(diff_large, large);`
  - `mul_giga_nano_is_uno`

#### Quantity value package

One of:

- [Generic quantity value package](#generic-quantity-value-package)
- [Macro quantity value package](#macro-quantity-value-package)

#### Generic quantity value package

A package that exports generic [quantity value types](#quantity-value-type) that must be specialized by the user.

Allowances:

- May export generic quantity value types .
- May define the quantity value types using a [quantity value definition package](#macro-quantity-value-package).

#### Macro quantity value package

A package that exports code items and guidelines for defining new [quantity value types](#quantity-value-type).

Allowances:

- May export macros for defining specific quantity value types.

#### Quantity value test package

A package that tests a specific [quantity value package](#quantity-value-package).

Requirements:

- Must define one [quantity value type](#quantity-value-type) per quantity in `unitopia-test-helpers` using only the items exported from the specific quantity value package under test.

#### Marker trait

A trait that doesn't have any functions or associated types.

Notes:

- A marker trait may have generic parameters.

#### Mapping trait

A trait that doesn't have any functions but does have associated types.

Notes:

- A mapping trait may have generic parameters.
- The name "mapping" was chosen because it provides a type-level mapping from the implementing type (`Self`) to the associated type.

#### Linearity marker trait

A [marker trait](#marker-trait) with one of the following names:

- `Linear`
- `Logarithmic`
- `Exponential`

#### Marker struct

A struct whose every field is a `PhantomData`.

Requirements:

- Must have a `#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]` attribute

#### Vanilla marker struct

A [marker struct](#marker-struct) without generic arguments.

#### Generic marker struct

A [marker struct](#marker-struct) with at least one generic argument.

#### Container struct

A struct with at least one generic parameter `T` and at least one field of type `T`.

#### Wrapper struct

A struct with at least one generic parameter `T`, exactly one field `inner: T` and any number of fields whose outer type is `PhantomData`.

Requirements:

- Must have a `#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]` attribute
- Must have a `#[repr(transparent)]` attribute
- Must implement `Deref`, `AsRef`, `Borrow` by delegating to the corresponding impl on the `inner` field
- Must implement `From<T>`
- Must not implement `Into<T>` (see B001)
- Must have a `pub const fn new`
  - Notes:
    - `From::from` can't be used instead of `new` because it is not `const`

#### Open wrapper struct

A [wrapper struct](#wrapper-struct) with the following properties:

- Must have an `inner` field that is `pub`
- Must implement `DerefMut`, `BorrowMut` by delegating to the corresponding impl on the `inner` field

Synonyms: OWS.

#### Strict wrapper struct

A [wrapper struct](#wrapper-struct) with exactly one generic parameter `T`, exactly one field `inner: T`, zero fields whose outer type is `PhantomData`.

Synonyms: SWS.

#### Strict open wrapper struct

A [wrapper struct](#wrapper-struct) that is both [open](#open-wrapper-struct) and [strict](#strict-wrapper-struct).

Synonyms: SOWS.

#### Numeric type

A type that implements `num_traits::NumAssignRef`.

Examples:

- `i8`
- `u32`
- `usize`
- `f64`
- `rust_decimal::Decimal`
- `num::rational::Ratio`

Notes:

- `num_traits::NumAssignRef` is implemented automatically for types that implement its supertraits.

#### Numeric type conditional on T

A type that is [numeric](#numeric-type) if T is [numeric](#numeric-type).

#### Numeric strict open wrapper struct

A [strict open wrapper struct](#strict-open-wrapper-struct) that is [numeric type conditional on T](#numeric-type-conditional-on-t) (where `T` is the generic parameter of the wrapper).

Synonyms: NSOWS

Requirements:

- Must conditionally implement `num_traits::NumAssignRef` (this is a consequence of this type being numeric)

#### Prefixed numeric strict open wrapper struct

A [NSOWS](#numeric-strict-open-wrapper-struct) whose name is a [prefix](#prefix).

Synonyms: PNSOWS

Examples:

- `Quetta<T>`
- `Ronto<T>`
- `Tetravigesi<T>`

Notes:

- The examples show only the most significant part of the type definition.
- Some values of PNSOWS are semantically equal but not syntactically equal
  - Examples
    - `Quetta::ZERO` and `Ronto::ZERO`
    - `Kilo::from(1usize)` and `Uno::from(1000usize)`
- Most conversions may result in an overflow
  - Examples:
    - `Uno::<usize>::opt_from(Kilo::from(usize::MAX))`

#### Unital prefixed numeric strict open wrapper struct

A [NSOWS](#numeric-strict-open-wrapper-struct) whose name is a [unit](#unit) and whose `T` is an [PNSOWS](#prefixed-numeric-strict-open-wrapper-struct).

Synonyms: UPNSOWS

Examples:

- `Second<T>`
- `Newton<T>`

Notes:

- The examples show only the most significant part of the type definition.

#### Arith output

A struct that represents the output of the arithmetic operation.

Notes:

- Arith outputs should be used as `type Output` in the arithmetic trait implementations.

#### Quantity value type

A type that represents a [quantity value](#quantity-value).

Requirements:

- TODO: Move the required trait impls
- Must implement [addition traits](#addition-trait) for values of the same quantity and unit
  - Must set the quantity in the `Output` to `Summ<Q>` where `Q` is the input quantity
- Must implement [subtraction traits](#subtraction-trait) for values of the same quantity and unit
  - Must set the quantity in the `Output` to `Diff<Q>` where `Q` is the input quantity
- TODO (?): Must implement the `Quantity` trait from `unitopia_helpers`
  - (we need to express a requirement that a quantity value type must have a value, a unit, a storage)

#### `QuantityValue`

A runtime-unit [quantity value type](#quantity-value-type) exported by [`unitopia-runtime-unit-quantity-value`](#unitopia-runtime-unit-quantity-value).

```rust
pub struct QuantityValue<Value, Unit> {
    pub value: Value,
    pub unit: Unit,
}
```

Requirements:

- Must have a constructor that accepts both fields.
- Its arithmetic requirements override the infallible addition and subtraction requirements of [quantity value type](#quantity-value-type).
- Must have a `Display` impl generic over `<Value: Display, Unit: Display>`:
  - Must write `"{value} {unit}"`.
- Must have methods:
  - `try_add`
    - Must perform fallible addition with another `QuantityValue` of the same Rust type.
  - `try_sub`
    - Must perform fallible subtraction with another `QuantityValue` of the same Rust type.
- `try_add` and `try_sub`:
  - Must compare the runtime units before operating on the values.
  - Must return `Err` containing both operands if the runtime units differ.
  - Must use checked value arithmetic.
  - Must return `Err` containing both operands if the value arithmetic fails.
  - Must not mutate either operand before every fallible validation and arithmetic operation succeeds.
- Must not implement `Add`, `AddAssign`, `Sub`, or `SubAssign` between two `QuantityValue` values.
  - Rationale: values with different runtime units have the same Rust type, while these traits cannot report a unit mismatch without making their output unexpectedly fallible.
- Scalar operations that preserve the unit may be infallible only when the corresponding operation on `Value` is infallible.

Notes:

- The spec for this type has been written by an LLM.
- `QuantityValue<Decimal, Currency>` can represent money whose currency is selected at runtime.
- This type contains no [quantity](#quantity) identity, so it is not a [measure](#measure).

#### Quantity-generic quantity value type

A [quantity value type](#quantity-value-type) that has a generic parameter `Q` for quantity.

#### Unit-generic quantity value type

A [quantity value type](#quantity-value-type) that has a generic parameter `U` for unit.

#### Storage-generic quantity value type

A [quantity value type](#quantity-value-type) that has a generic parameter `S` for storage.

- Must support any [numeric type](#numeric-type) as storage.

#### Fully generic quantity value type

A [quantity value type](#quantity-value-type) that:

- Is [quantity-generic](#quantity-generic-quantity-value-type)
- Is [unit-generic](#unit-generic-quantity-value-type)
- Is [storage-generic](#storage-generic-quantity-value-type)

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

- If exported from our crate, this type will be classified as a "foreign type" in dependents, so the dependents won't be able to implement foreign traits for it, so we must provide all trait implementations upfront.
  - This may not be possible at all in a generic way (trait impl overlaps, so we would need to specify every type exactly without using the generics).
  - It's a lot of work to cover all potential storage types that are published on crates.io.
  - We may export such type but mention in the docs that it has a limited set of impls, and if the user wishes to implement more traits for it, they can either submit a PR or define their own `Measurement` type using a macro.

#### Fully specific quantity value type

A [quantity value type](#quantity-value-type) that doesn't have any generic parameters.

Requirements:

- Must have a suffix that is equal to the unit name in plural form.
- Must document the exact conditions of the measurement in a doc comment attached to the type.

Allowances:

- May document the exact conditions of the measurement in the type name itself.

Examples of names:

- `GameDelaySeconds`
- `PersonHeightAtNoonMillimeters`

#### Quantity value type definition macro

A macro that defines a quantity type.

Synonyms: QVTDM.

Requirements:

- Name must start with "quantity"

Examples:

- `quantity` - defines a generic quantity type
- `quantity_of_seconds_as_u32` - defines a fully specific quantity type

#### Fully specific quantity value type definition macro

A [QVTDM](#quantity-value-type-definition-macro) that defines a [fully specific quantity type](#fully-specific-quantity-value-type).

Synonyms: FSQVTDM.

Rationale:

- If the unit type and storage type are known statically at macro expansion time (which is even earlier than compile time), then it is possible to provide specific trait implementations
  - Examples of groups of specific trait implementations:
    - Infallible conversions to wider types (e.g. `impl From<DurationSeconds> for DurationMilliseconds` if DurationSeconds uses u32 and DurationMilliseconds uses u64; note that such impl is only possible if DurationSeconds uses a narrower type than DurationMilliseconds).
    - Infallible conversions from types from foreign crates (e.g. `std`, `time`, `chrono` for quantities of time kind).

Requirements:

- Name must match the pattern: `quantity_of_{{units}}_as_{{storage}}`
  - `{{units}}` must be a unit type name in snake_case and plural form
    - Examples:
      - `seconds`
  - `{{storage}}` must be a storage type name in snake_case and singular form:
    - Requirements:
      - The following path prefixes must be omitted: `core::primitive`
      - `::` must be replaced by `_`
    - Allowances:
      - The path prefix may be omitted for commonly used types
    - Examples:
      - `core::primitive::u64` -> `u64`
      - `rust_decimal::Decimal` -> `decimal`

Examples of names:

- `quantity_of_seconds_as_u32`

#### Fully specific time quantity value type definition macro

A [FSQVTDM](#fully-specific-quantity-value-type-definition-macro) that uses a time unit.

Requirements:

- Must implement conversions from and into types exported from the foreign timekeeping crates (see [timestamp-please project doc](#timestamp-please-project-doc))

Examples of names:

- `quantity_of_seconds_as_u32`
- `quantity_of_nanoseconds_as_u128`

#### timestamp-please project doc

A document at <https://github.com/DenisGorbachev/timestamp-please/blob/main/.agents/project.md>

### Knowledge

#### General

- Some unit name multipliers are a power of 10
  - Examples
    - Milliliter (10^-3)
- Some unit names are not a power of 10
  - Examples
    - Minute (60 seconds)
    - Day (24 hours)
- Some SI units have a unit name with a power of 10
  - Examples
    - Kilogram
- Some physical quantities have units that are not an exact multiple of each other
  - Examples
    - Time has "year" and "second" (this is why "leap seconds" exist)
  - Notes
    - This implies that we must accept a Unit instead of Quantity as a generic argument
- Some units are not even fixed-duration
  - Examples
    - Year can be 365 or 366 days

#### Technical

- The `add` and `sub` ops on two measures must type-check only if their units are equal
- The `mul` and `div` ops on two measures must always type-check, but the resulting unit must be a multiplication or division of the units of the two measures
- I'm not sure about `rem`

### Error handling

#### Principle

Every fallible function must return an error with enough data for the caller to retry the call.

#### Guidelines

- Use `handle!` instead of `?` try operator to unwrap `Result` types
- Use `handle!` instead of `Result::map_err`
- Use `handle_opt!` instead of `?` try operator to unwrap `Option` types
- Use `handle_opt!` instead of `Option::ok_or` and `Option::ok_or_else`
- Use `handle_bool!` instead of `if condition { return Err(...) }` to return an error if some condition is true
- Use `handle_iter!` or `handle_iter_of_refs!` to collect and return errors from iterators
- Use `handle_into_iter!` to handle errors in collections that implement `IntoIterator` (including `Vec` and `HashMap`)
- Calls to macros that begin with `handle` must not contain calls to `clone` (must not contain `.clone()`)
  - Rationale: there is no need to clone the variables because the macros consume them only in the error branch, and the error branch contains a `return` statement. The variables are not consumed in the success branch, so you can always use them in the subsequent code.
- Don't convert a `Result` into an `Option`, always propagate the error up the call stack
- Don't use `unwrap` or `expect`
- Don't return strings as errors
- Every fallible function must return a unique error type, even if it contains only one fallible expression
- Every call to another fallible function must be wrapped in a unique error enum variant
- Every fallible function body must begin with `use ThisFunctionError::*;`, where `ThisFunctionError` must be the name of this function's error enum (for example: `use ParseConfigError::*;`)
- Every fallible function body must use the error enum variant names without the error enum name prefix (for example: `ReadFileFailed` instead of `ParseConfigError::ReadFileFailed`)
- Every error type must be an enum
- Every error type must derive `Error` via `thiserror` v2
- Every error type must be located in the same file as the function that returns it below other non-mod items
- Every error enum variant must be a struct variant
- Every error enum variant must contain one field per owned variable that is relevant to the fallible expression that this variant wraps
  - The relevant variable is a variable whose value determines whether the fallible expression returns an `Ok` or an `Err`
- Every error enum variant must have fields only for [`data types`](#data-type), not for [`non-data types`](#non-data-type)
- Every error enum variant must have an `#[error]` attribute
  - The `#[error]` attribute must contain the error message displayed for the user
  - The `#[error]` attribute must not contain the `source` field
  - The `#[error]` attribute should contain only those fields that can be displayed on one line
  - If the `#[error]` attribute contains fields that implement `Display`, then those fields must be output using `Display` formatting (not `Debug` formatting)
    - Good:
      ```rust
      #[derive(Error, Debug)]
      pub enum QueryFailed {
          #[error("task not found for query '{query}'")]
          TaskNotFound { query: String }
      }
      ```
    - Bad:
      ```rust
      #[derive(Error, Debug)]
      pub enum QueryFailed {
          #[error("task not found for query '{query:?}'")]
          TaskNotFound { query: String }
      }
      ```
  - If the `#[error]` attribute contains fields whose values may be rendered as [hard-to-see string](#hard-to-see-string), then those fields must be wrapped in single quotes:
    - `name` can be rendered as hard-to-see string, so it must be wrapped in single quotes:
      - Good: `#[error("user '{name}' not found")]`
      - Bad: `#[error("user {name} not found")]`
    - `len` can't be rendered as hard-to-see string, so it must not be wrapped in single quotes:
      - Good: `#[error("failed to parse {len} responses", len = responses.len())]`
      - Bad: `#[error("failed to parse '{len}' responses", len = responses.len())]`
  - If the error enum variant has a field whose type is `std::process::Command` or `tokio::process::Command`, it must be rendered in the error message in backticks via `render_command` function from `errgonomic` crate (requires `process` feature)
- If the error enum variant has a `source` field, then this field must be the first field
- If each field of each variant of the error enum implements `Copy`, then the error enum must implement `Copy` too
- Every error enum variant field must have an owned type (not a reference)
- Every error enum variant field must not have a `#[from]` attribute
- Every variable that contains secret data (the one which must not be displayed or logged, e.g. password, API key, personally identifying information) must have a type that doesn't output the underlying data in the `Debug` and `Display` impls (e.g. `secrecy::SecretBox`)
- The code that calls a fallible function on each element of a collection should return an `impl Iterator<Item = Result<T, E>>` instead of short-circuiting on the first error
- If Clippy outputs a `result_large_err` warning, then the large fields of the error enum must be wrapped in a `Box`
- If an argument of callee implements `Copy`, the callee should not include it in the list of error enum variant fields (the caller must include it because of the rule to include all relevant owned variables)
- If you see a function that returns a `Result` whose last argument is `()` (e.g. `Result<(), ()>`, `Result<T, ()>`, `Result<u32, ()>`), then you must fix the error handling in this function according to the guidelines and replace `()` with a proper error type

##### Naming

- The name of the error enum must end with `Error` (for example: `ParseConfigError`)
- The name of the error enum variant should end with `Failed` or `NotFound` or `Invalid` (for example: `ReadFileFailed`, `UserNotFound`, `PasswordInvalid`)
- If the error variant name is associated with a child function call, the name of the error variant must be equal to the name of the function converted to CamelCase concatenated with `Failed` (for example: if the parent function calls `read_file`, then it should call it like this: `handle!(read_file(&path), ReadFileFailed, path)`
- The name of the error enum must include the name of the function converted to CamelCase
  - If the function is a freestanding function, the name of the error type must be exactly equal to the name of the function converted to CamelCase concatenated with `Error`
  - If the function is an associated function, the name of the error type must be exactly equal to the name of the type without generics concatenated with the name of the function in CamelCase concatenated with `Error`
  - If the error is specified as an associated type of a foreign trait with multiple functions that return this associated error type, then the name of the error type must be exactly equal to the name of the trait including generics concatenated with the name of the type for which this trait is implemented concatenated with `Error`
- Every `impl TryFrom<A> for B` must use a special form of error handling that matches on multiple variables at once and returns a single error that contains fields for all available variables. For example:
  ```rust
  #[derive(Getters, Clone, Debug)]
  pub struct Human {
      name: String,
      #[getter(copy)]
      age: u32,
  }
  
  #[derive(Getters, Clone, Debug)]
  pub struct Adult {
      name: NonEmptyString,
      #[getter(copy)]
      age: u32,
  }
  
  impl TryFrom<Human> for Adult {
      type Error = TryFromHumanForAdultError;
  
      fn try_from(input: Human) -> Result<Self, Self::Error> {
          use TryFromHumanForAdultError::*;
          let Human {
              name,
              age,
          } = input;
          let name_result = NonEmptyString::try_from(name);
          let is_adult = age > 18;
          match (name_result, is_adult) {
              (Ok(name), true) => Ok(Self {
                  name,
                  age,
              }),
              (name_result, is_adult) => Err(ConversionFailed {
                  name_result,
                  age,
                  is_adult,
              }),
          }
      }
  }
  
  #[derive(Error, Debug)]
  pub enum TryFromHumanForAdultError {
      #[error("failed to convert human to adult")]
      ConversionFailed { name_result: Result<NonEmptyString, TryFromStringForNonEmptyStringError>, age: u32, is_adult: bool },
  }
  ```

#### Definitions

##### Fallible expression

An expression that returns a `Result`.

##### Fallible expression group

A group of [fallible expressions](#fallible-expression) where each output variable does not depend on the output variables of other fallible expressions within the same group.

Aliases: FEG.

##### Data type

A type that holds the actual data.

Examples:

- `bool`
- `String`
- `PathBuf`

##### Non-data type

A type that doesn't hold the actual data.

Examples:

- `RestClient` doesn't point to the actual data, it only allows querying it.
- `DatabaseConnection` doesn't hold the actual data, it only allows querying it.

##### Hard-to-see string

A string that is empty or contains only whitespace characters.

#### errgonomic/src/macros.rs

```rust
/// [`handle!`](crate::handle) is a better alternative to [`map_err`](Result::map_err) because it doesn't capture any variables from the environment if the result is [`Ok`], only when the result is [`Err`].
/// By contrast, a closure passed to `map_err` always captures the variables from environment, regardless of whether the result is [`Ok`] or [`Err`]
/// Use [`handle!`](crate::handle) if you need to pass owned variables to an error variant (which is returned only in case when result is [`Err`])
/// In addition, this macro captures the original error in the `source` variable, and sets it as the `source` key of the error variant
///
/// Note: [`handle!`](crate::handle) assumes that your error variant is a struct variant
#[macro_export]
macro_rules! handle {
    ($result:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        match $result {
            Ok(value) => value,
            Err(source) => return Err($variant {
                source: source.into(),
                $($arg: $crate::_into!($arg$(: $value)?)),*
            }),
        }
    };
}

/// See also: [`handle_opt_take!`](crate::handle_opt_take)
#[macro_export]
macro_rules! handle_opt {
    ($option:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        match $option {
            Some(value) => value,
            None => return Err($variant {
                $($arg: $crate::_into!($arg$(: $value)?)),*
            }),
        }
    };
}

/// This macro is an opposite of [`handle_opt!`](crate::handle_opt) - it returns an error if the option contains a `Some` variant.
///
/// Note that this macro calls [`Option::take`], which will leave a `None` if the option was `Some(value)`.
/// Note that this macro has a mandatory argument `$some_value` (used in `if let Some($some_value) = $option.take()`), which will also be passed to the error enum variant.
#[macro_export]
macro_rules! handle_opt_take {
    ($option:expr, $variant:ident, $some_value:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        if let Some($some_value) = $option.take() {
            return Err($variant {
                $some_value: $some_value.into(),
                $($arg: $crate::_into!($arg$(: $value)?)),*
            })
        }
    };
}

/// Returns an error when the condition is true.
///
/// This is useful for guard checks that should fail fast with a specific error variant.
#[macro_export]
macro_rules! handle_bool {
    ($condition:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        if $condition {
            return Err($variant {
                $($arg: $crate::_into!($arg$(: $value)?)),*
            });
        };
    };
}

/// Collects results from an iterator, returning a variant that wraps all errors.
///
/// `$results` must be an `impl Iterator<Item = Result<T, E>>`.
#[macro_export]
macro_rules! handle_iter {
    ($results:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        {
            match $crate::partition_result($results) {
                Ok(oks) => oks,
                Err(errors) => {
                    return Err($variant {
                        source: errors.into(),
                        $($arg: $crate::_into!($arg$(: $value)?)),*
                    });
                }
            }
        }
    };
}

/// Collects results while keeping the corresponding input items, returning `(outputs, items)` on success.
///
/// This macro returns a tuple because the iteration consumes items that may be needed later.
/// If there are no errors, `items.len() == outputs.len()`.
/// If the results iterator terminates early, the returned `items` may be shorter than the original input.
#[macro_export]
macro_rules! handle_iter_of_refs {
    ($results:expr, $items:expr, $variant:ident $(, $arg:ident$(: $value:expr)?)*) => {
        {
            use alloc::vec::Vec;
            let (outputs, items, errors) = core::iter::zip($results, $items).fold(
                (Vec::new(), Vec::new(), Vec::new()),
                |(mut outputs, mut items, mut errors), (result, item)| {
                    match result {
                        Ok(output) => {
                            outputs.push(output);
                            items.push(item);
                        }
                        Err(source) => {
                            errors.push($crate::ItemError {
                                item,
                                source,
                            });
                        }
                    }
                    (outputs, items, errors)
                },
            );
            if errors.is_empty() {
                (outputs, items)
            } else {
                return Err($variant {
                    source: errors.into(),
                    $($arg: $crate::_into!($arg$(: $value)?)),*
                });
            }
        }
    };
}

/// Collects results from any `IntoIterator`, wrapping all errors into one variant.
#[macro_export]
macro_rules! handle_into_iter {
    ($results:expr, $variant:ident $(, $arg:ident$(: $value:expr)?)*) => {
        $crate::handle_iter!($results.into_iter(), $variant $(, $arg$(: $value)?),*)
    };
}

/// [`handle_discard`](crate::handle_discard) should only be used when you want to discard the source error. This is discouraged. Prefer other handle-family macros that preserve the source error.
#[macro_export]
macro_rules! handle_discard {
    ($result:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        match $result {
            Ok(value) => value,
            Err(_) => return Err($variant {
                $($arg: $crate::_into!($arg$(: $value)?)),*
            }),
        }
    };
}

/// [`map_err`](crate::map_err) should be used only when the error variant doesn't capture any owned variables (which is very rare), or exactly at the end of the block (in the position of returned expression).
#[macro_export]
macro_rules! map_err {
    ($result:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        $result.map_err(|source| $variant {
            source: source.into(),
            $($arg: $crate::_into!($arg$(: $value)?)),*
        })
    };
}

/// Converts [`None`] into an error variant without returning early.
///
/// [`map_none`](crate::map_none) should be used only when the error variant doesn't capture any owned variables (which is very rare), or exactly at the end of the block (in the position of returned expression).
#[macro_export]
macro_rules! map_none {
    ($option:expr, $variant:ident$(,)? $($arg:ident$(: $value:expr)?),*) => {
        match $option {
            Some(value) => Ok(value),
            None => Err($variant {
                $($arg: $crate::_into!($arg$(: $value)?)),*
            })
        }
    };
}

/// Internal
#[doc(hidden)]
#[macro_export]
macro_rules! _into {
    ($arg:ident) => {
        $arg.into()
    };
    ($arg:ident: $value:expr) => {
        $value.into()
    };
}

/// Internal
#[doc(hidden)]
#[macro_export]
macro_rules! _index_err {
    ($f:ident) => {
        |(index, item)| $f(item).map_err(|err| (index, err))
    };
}

/// Internal
#[doc(hidden)]
#[macro_export]
macro_rules! _index_err_async {
    ($f:ident) => {
        async |(index, item)| $f(item).await.map_err(|err| (index, err))
    };
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use crate::{ErrVec, ItemError, PathBufDisplay};
    use alloc::boxed::Box;
    use alloc::string::String;
    use alloc::vec::Vec;
    use futures::future::join_all;
    use serde::{Deserialize, Serialize};
    use std::io;
    use std::path::{Path, PathBuf};
    use std::println;
    use std::str::FromStr;
    use std::sync::{Arc, RwLock};
    use thiserror::Error;
    use tokio::fs::read_to_string;
    use tokio::task::JoinSet;

    #[allow(dead_code)]
    struct PrintNameCommand {
        dir: PathBuf,
        format: Format,
    }

    #[allow(dead_code)]
    impl PrintNameCommand {
        async fn run(self) -> Result<(), PrintNameCommandError> {
            use PrintNameCommandError::*;
            let Self {
                dir,
                format,
            } = self;
            let config = handle!(parse_config(&dir, format).await, ParseConfigFailed);
            println!("{}", config.name);
            Ok(())
        }
    }

    /// This function tests the [`crate::handle!`] macro
    #[allow(dead_code)]
    async fn parse_config(dir: &Path, format: Format) -> Result<Config, ParseConfigError> {
        use Format::*;
        use ParseConfigError::*;
        let path_buf = dir.join("config.json");
        let contents = handle!(read_to_string(&path_buf).await, ReadFileFailed, path: path_buf);
        match format {
            Json => {
                let config = handle!(serde_json::de::from_str(&contents), DeserializeFromJson, path: path_buf, contents);
                Ok(config)
            }
            Toml => {
                let config = handle!(toml::de::from_str(&contents), DeserializeFromToml, path: path_buf, contents);
                Ok(config)
            }
        }
    }

    /// This function tests the [`crate::handle_opt!`] and [`crate::map_none!`] macros.
    #[allow(dead_code)]
    fn first_word(lines: &[String]) -> Result<&str, FirstWordError> {
        use FirstWordError::*;
        let line = handle_opt!(lines.first(), LineNotFound);
        map_none!(line.split_whitespace().next(), WordNotFound)
    }

    /// This function tests the [`crate::handle_iter!`] macro
    #[allow(dead_code)]
    fn multiply_evens(numbers: Vec<u32>) -> Result<Vec<u32>, MultiplyEvensError> {
        use MultiplyEvensError::*;
        let results = numbers.into_iter().map(|number| {
            use CheckEvenError::*;
            if number % 2 == 0 {
                match number.checked_mul(10) {
                    Some(product) => Ok(product),
                    None => Err(NumberOverflowed {
                        number,
                    }),
                }
            } else {
                Err(NumberNotEven {
                    number,
                })
            }
        });
        Ok(handle_iter!(results, CheckEvensFailed))
    }

    /// This function tests the [`crate::handle_into_iter!`] macro
    #[allow(dead_code)]
    async fn read_files(paths: Vec<PathBuf>) -> Result<Vec<String>, ReadFilesError> {
        use ReadFilesError::*;
        let results = paths
            .into_iter()
            .map(check_file)
            .collect::<JoinSet<_>>()
            .join_all()
            .await;
        Ok(handle_into_iter!(results, CheckFilesFailed))
    }

    #[allow(dead_code)]
    async fn read_files_ref(paths: Vec<PathBuf>) -> Result<Vec<String>, ReadFilesRefError> {
        use ReadFilesRefError::*;
        let iter = paths.iter().map(check_file_ref);
        let results = join_all(iter).await;
        let items = paths.into_iter().map(PathBufDisplay::from);
        let (outputs, _items) = handle_iter_of_refs!(results.into_iter(), items, CheckFilesRefFailed);
        Ok(outputs)
    }

    // async fn check_file(path: &Path)

    /// This function exists to test error handling in async code
    #[allow(dead_code)]
    async fn process(number: u32) -> Result<u32, ProcessError> {
        Ok(number)
    }

    #[derive(Error, Debug)]
    enum PrintNameCommandError {
        #[error("failed to parse config")]
        ParseConfigFailed { source: ParseConfigError },
    }

    /// Variants don't have the `format` field because every variant already corresponds to a single specific format
    /// Some variants have the `path` field because the `contents` depends on `path`
    /// Some `source` field types are wrapped in `Box` according to suggestion from `result_large_err` lint
    #[derive(Error, Debug)]
    enum ParseConfigError {
        #[error("failed to read the file: {path}")]
        ReadFileFailed { path: PathBuf, source: io::Error },
        #[error("failed to deserialize the file contents from JSON: {path}")]
        DeserializeFromJson { path: PathBuf, contents: String, source: Box<serde_json::error::Error> },
        #[error("failed to deserialize the file contents from TOML: {path}")]
        DeserializeFromToml { path: PathBuf, contents: String, source: Box<toml::de::Error> },
    }

    #[allow(dead_code)]
    #[derive(Error, Debug)]
    enum ProcessError {}

    #[allow(dead_code)]
    #[derive(Copy, Clone, Debug)]
    enum Format {
        Json,
        Toml,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    struct Config {
        name: String,
        timeout: u64,
        parallel: bool,
    }

    #[allow(dead_code)]
    fn parse_even_number(input: &str) -> Result<u32, ParseEvenNumberError> {
        use ParseEvenNumberError::*;
        let number = handle!(input.parse::<u32>(), InputParseFailed);
        handle_bool!(number % 2 != 0, NumberNotEven, number);
        Ok(number)
    }

    #[derive(Error, Debug)]
    enum ParseEvenNumberError {
        #[error("failed to parse input")]
        InputParseFailed { source: <u32 as FromStr>::Err },
        #[error("number is not even: {number}")]
        NumberNotEven { number: u32 },
    }

    #[derive(Error, Debug)]
    enum FirstWordError {
        #[error("line not found")]
        LineNotFound {},
        #[error("word not found")]
        WordNotFound {},
    }

    #[derive(Error, Debug)]
    enum MultiplyEvensError {
        #[error("failed to check {len} numbers", len = source.len())]
        CheckEvensFailed { source: ErrVec<CheckEvenError> },
    }

    #[derive(Error, Debug)]
    enum ReadFilesError {
        #[error("failed to check {len} files", len = source.len())]
        CheckFilesFailed { source: ErrVec<CheckFileError> },
    }

    #[derive(Error, Debug)]
    enum ReadFilesRefError {
        #[error("failed to check {len} files", len = source.len())]
        CheckFilesRefFailed { source: ErrVec<ItemError<PathBufDisplay, CheckFileRefError>> },
    }

    #[derive(Error, Debug)]
    enum CheckEvenError {
        #[error("number is not even: {number}")]
        NumberNotEven { number: u32 },
        #[error("number overflowed: {number}")]
        NumberOverflowed { number: u32 },
    }

    async fn check_file(path: PathBuf) -> Result<String, CheckFileError> {
        use CheckFileError::*;
        let content = handle!(read_to_string(&path).await, ReadToStringFailed, path);
        handle_bool!(content.is_empty(), FileIsEmpty, path);
        Ok(content)
    }

    #[derive(Error, Debug)]
    enum CheckFileError {
        #[error("failed to read the file to string: {path}")]
        ReadToStringFailed { path: PathBuf, source: io::Error },
        #[error("file is empty: {path}")]
        FileIsEmpty { path: PathBuf },
    }

    async fn check_file_ref(path: &PathBuf) -> Result<String, CheckFileRefError> {
        use CheckFileRefError::*;
        let content = handle!(read_to_string(&path).await, ReadToStringFailed);
        handle_bool!(content.is_empty(), FileIsEmpty);
        Ok(content)
    }

    #[derive(Error, Debug)]
    enum CheckFileRefError {
        #[error("failed to read the file to string")]
        ReadToStringFailed { source: io::Error },
        #[error("file is empty")]
        FileIsEmpty,
    }

    #[derive(Clone, Debug)]
    struct State {
        user: User,
    }

    #[derive(Clone, Debug)]
    struct User {
        username: String,
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    struct Book {
        user_idx: usize,
        name: String,
    }

    #[allow(dead_code)]
    fn get_username(state: Arc<RwLock<State>>) -> Result<String, GetUsernameError> {
        use GetUsernameError::*;
        // `state.read()` returns `LockResult` whose Err variant is `PoisonError<RwLockReadGuard<'_, T>>`, which contains an anonymous lifetime
        // The error enum returned from this function must contain only owned fields, so it can't contain a `source` that has a lifetime
        // Therefore, we have to use handle_discard!, although it is discouraged
        let guard = handle_discard!(state.read(), AcquireReadLockFailed);
        let username = guard.user.username.clone();
        Ok(username)
    }

    #[derive(Error, Debug)]
    pub enum GetUsernameError {
        #[error("failed to acquire read lock")]
        AcquireReadLockFailed,
    }

    #[derive(Clone, Debug)]
    struct Db {
        users: Vec<User>,
        books: Vec<Book>,
    }

    impl Db {
        /// Validates only the foreign keys
        /// Assumes that the collection items have already been validated before they were inserted
        #[allow(dead_code)]
        pub fn validate(&self) -> impl Iterator<Item = DbValidateError> {
            use DbValidateError::*;

            self.books
                .iter()
                .enumerate()
                .filter_map(|(book_idx, book)| {
                    let user_idx = book.user_idx;
                    if self.users.get(user_idx).is_none() {
                        Some(UserNotFound {
                            book_idx,
                            user_idx,
                        })
                    } else {
                        None
                    }
                })
        }
    }

    #[derive(Error, Debug)]
    pub enum DbValidateError {
        #[error("book #{book_idx} has a non-existent user #{user_idx}")]
        UserNotFound { book_idx: usize, user_idx: usize },
    }

    #[allow(dead_code)]
    fn get_answer(prompt: String, get_response: &mut impl FnMut(String) -> Result<WeirdResponse, io::Error>) -> Result<String, GetAnswerError> {
        use GetAnswerError::*;
        // Since the `get_response` external API doesn't return the `prompt` in its error, we have to clone `prompt` before passing it as argument, so that we could pass it to the error enum variant
        // Cloning may be necessary with external APIs that don't return arguments in errors, but it must not be necessary in our code
        let mut response = handle!(get_response(prompt.clone()), GetResponseFailed, prompt);
        handle_opt_take!(response.error, ResponseContainsError, error);
        Ok(response.answer)
    }

    /// OpenAI Responses API returns a response with `error: Option<WeirdResponseError>` field, which is weird, but must still be handled
    #[derive(Debug)]
    pub struct WeirdResponse {
        answer: String,
        error: Option<WeirdResponseError>,
    }

    #[allow(dead_code)]
    #[derive(Error, Debug)]
    pub enum WeirdResponseError {
        #[error("prompt is empty")]
        PromptIsEmpty,
        #[error("context limit reached")]
        ContextLimitReached,
    }

    /// [`GetAnswerError::GetResponseFailed`] `error` attribute doesn't contain a reference to `{prompt}` because the prompt can be very long, so it would make the error message very long, which is undesirable
    #[derive(Error, Debug)]
    pub enum GetAnswerError {
        #[error("failed to get response")]
        GetResponseFailed { source: io::Error, prompt: String },
        #[error("response contains an error")]
        ResponseContainsError { error: WeirdResponseError },
    }
}
```

## Project files

### mise.toml

```toml
min_version = "2026.7.13"

[settings]
idiomatic_version_file_enable_tools = ["rust"]
task.output = "keep-order"

[tools]
node = "24.15.0"
deno = "1.46.1"
fnox = "1.33.1"
fd = "10.4.2"
"github:ewhauser/shuck" = "0.2.2"
"aqua:nushell/nushell" = "0.116.1"
"cargo:nu-lint" = "1.8.0"
"github:nushell/nufmt" = "0.1.6"
"aqua:rvben/rumdl" = "0.3.0"
"npm:@commitlint/config-conventional" = "19.6.0"
"npm:@commitlint/cli" = "19.6.0"
"npm:@commitlint/types" = "19.5.0"
"npm:skills" = "1.5.24"
"cargo:https://github.com/DenisGorbachev/cargo-insert-docs" = { version = "rev:9bccf15cc367a50d2652b0eaf5da7faf5929c666", crate = "cargo-insert-docs", locked = true }
"cargo:cargo-hack" = "0.6.33"
"cargo:cargo-nextest" = "0.9.145"
"cargo:cargo-expand" = "1.0.114"
"cargo:taplo-cli" = "0.10.0"
"cargo:sd" = "1.0.0"
"cargo:cargo-edit" = "0.13.13"

[hooks]
postinstall = { task = "git:install-hooks" }

[env]
SHUCK_CACHE_DIR = "{{config_root}}/.cache"

[tasks."build"]
run = "cargo build --workspace"

[tasks."check"]
depends = ["cargo:validate-config"]
run = [{ tasks = ["lint", "test"] }]

[tasks."check:ci"]
depends = ["cargo:validate-config"]
run = [{ tasks = ["lint", "test:ci"] }]

[tasks."test"]
depends = ["test:code", "test:docs"]

[tasks."lint"]
depends = ["lint:name", "lint:configs", "lint:code", "lint:code:style", "lint:shell", "lint:nu", "lint:docs", "lint:generated", "lint:reports"]

[tasks."lint:name"]
run = [{ task = "fix:name", args = ["--check"] }]

[tasks."lint:configs"]
depends = ["lint:configs:cargo", "lint:configs:fnox"]

[tasks."lint:configs:cargo"]
run = [{ task = "fix:cargo", args = ["--check"] }]

[tasks."lint:configs:fnox"]
run = [{ task = "fix:fnox" }]

[tasks."lint:code"]
run = "cargo clippy --locked --workspace --all-targets --all-features -- -D warnings"

[tasks."lint:code:style"]
run = "cargo fmt --all -- --check"

[tasks."lint:shell"]
# Shuck 0.2.2 scans Git-ignored local files, so exclude them explicitly.
run = '''shuck --config "lint.source-paths = ['$HOME']" check --extend-exclude '*.local.*' .'''

[tasks."lint:docs"]
run = "rumdl check"

[tasks."lint:generated"]
run = [{ task = "md:gen", args = ["AGENTS.ts"] }, { task = "md:gen", args = ["README.ts"] }]

[tasks."test:code"]
depends = ["test:code:regular", "test:code:integration"]

[tasks."test:code:regular"]
run = "fnox --profile test exec --replace -- cargo nextest run --locked --workspace --all-features --no-tests warn --cargo-quiet --show-progress none --no-input-handler --status-level fail --final-status-level flaky --no-fail-fast"

[tasks."test:code:integration"]
# `--test-threads 1` because integration tests must be run sequentially
run = [{ task = "test:code:regular", args = ["--ignore-default-filter", "--test-threads", "1", "integration_tests::"] }]

[tasks."test:docs"]
env = { RUSTDOCFLAGS = "-D warnings" }
run = "cargo test --locked --workspace --doc --all-features --no-fail-fast --quiet"

[tasks."test:ci"]
env = { RUSTDOCFLAGS = "-D warnings" }
run = "fnox --profile test exec --replace -- cargo hack test --locked --workspace --feature-powerset --keep-going --no-fail-fast --quiet -- --skip integration_tests::"

[tasks."pre-commit"]
alias = "pre-merge-commit"
run = [{ task = "git:validate-commit" }]

[tasks."commit-msg"]
run = 'mise run --output interleave commitlint -- --edit "$@"'

[tasks."fix"]
depends = ["cargo:validate-config"]
run = [{ task = "fix:aux" }]

[tasks."fix:aux"]
depends = ["fix:configs", "fix:docs", "fix:agents", "fix:readme"]

[tasks."fix:configs"]
depends = ["fix:cargo", "fix:fnox"]

[tasks."fix:code"]
depends = ["fix:name", "fix:code:style", "fix:shell", "fix:nu"]

[tasks."fix:nu"]
run = [{ task = "nu", args = ["nufmt", "--shebang"] }]

[tasks."fix:shell"]
run = [{ task = "lint:shell", args = ["--fix"] }]

[tasks."fix:code:warnings"]
depends = ["fix:cargo"]
# second pass is needed because "cargo clippy --fix" exits with 0 even if some warnings remain
env = { __CARGO_FIX_YOLO = 'yeah' }
run = [
    "cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged",
    { task = "lint:code" },
]

[tasks."fix:code:style"]
# Run after `fix:code:warnings` because both tasks modify the same code files.
depends = ["fix:code:warnings"]
run = "cargo fmt --all"

[tasks."fix:docs"]
depends = ["fix:agents", "fix:readme"]
# use `rumdl check --fix` instead of `rumdl fmt` because `rumdl check --fix` exits with 1 if errors remain (since v0.1.0)
run = [{ task = "lint:docs", args = ["--fix"] }]

[tasks."fix:agents"]
# "fix:agents" depends on "fix:code" because it reads the code files
depends = ["fix:name", "fix:configs", "fix:code"]
run = [{ task = "gen:agents" }]

[tasks."gen:readme"]
run = [{ task = "md:gen", args = ["README.ts", "--fix"] }]

[tasks."gen:agents"]
run = [{ task = "md:gen", args = ["AGENTS.ts", "--fix"] }]

[tasks."commitlint"]
run = "commitlint --extends \"$(mise where npm:@commitlint/config-conventional)/node_modules/@commitlint/config-conventional/lib/index.js\""

[tasks."agent:docs:list"]
run = "[ -d .agents/docs ] && find .agents/docs -type f -print || true"
output = "interleave"
quiet = true
```

### fnox.toml

```toml
#:schema https://fnox.jdx.dev/schema.json

if_missing = "error"
env = "exec"

[providers]
keychain = { type = "keychain", service = "unitopia" }
pass = { type = "password-store", prefix = "unitopia/" }
age = { type = "age", recipients = [
    "age1sf4r4amev2svqr6llwg8hgtz9n7p5qdh7hh0mavcshzfrmgfduksnq3hql",
    "age1605gsnxpe536sprwccyumq74veg0g80u55n8ggems0t8deau6qdsfnq3m3"
] }
```

### Cargo.toml

```toml
[workspace]
resolver = "3"
members = [
    "packages/unitopia-draft-measure",
    "packages/unitopia-draft-measure-v2",
    "packages/unitopia-draft-measure-v3",
    "packages/unitopia-draft-scale",
    "packages/unitopia-helpers",
    "packages/unitopia-marker-arith-outputs",
    "packages/unitopia-marker-prefixes",
    "packages/unitopia-marker-units",
    "packages/unitopia-numeric-strict-wrapper-prefixes",
    "packages/unitopia-open-wrapper-arith-outputs",
    "packages/unitopia-poly-unit",
    "packages/unitopia-runtime-unit-quantity-value",
    "packages/unitopia-strict-wrapper-prefixes",
    "packages/unitopia-strict-wrapper-units",
    "packages/unitopia-test-helpers"
]

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.93.1"
repository = "https://github.com/DenisGorbachev/unitopia"

[workspace.metadata.details]
name = "unitopia"
title = "Unitopia"
readme = { generate = false }

[workspace.dependencies]
errgonomic = "0.6.0"
serde = { version = "1.0.228", features = ["derive"] }
thiserror = "2.0.18"

[workspace.lints.cargo]
unused_dependencies = "deny"

[workspace.lints.rust]
redundant_imports = "deny"
unused_import_braces = "deny"
# unused_qualifications must not be "deny" because our code style has multiple `use Foo::*;`, and some macros (derive_more::Display, strum::Display, strum::EnumString) produce code with full qualifications
# unused_qualifications = "deny"

[workspace.lints.clippy]
# Rust 1.100 reports redundant fields inside derive macros: https://github.com/rust-lang/rust-clippy/issues/17525.
redundant_field_names = "allow"
absolute_paths = "deny"
arithmetic_side_effects = "deny"
```

### packages/unitopia-draft-measure-v2/Cargo.toml

```toml
[package]
name = "unitopia-draft-measure-v2"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
repository.workspace = true

[package.metadata.details]
title = ""

[lib]
path = "src/lib.rs"

[lints]
workspace = true

[dependencies]
derive-new = "0.7.0"
derive_more = { version = "2.1.1", features = ["full"] }
serde = { workspace = true, optional = true }

[package.metadata.cargo-machete]
ignored = ["serde"]
```

### packages/unitopia-draft-measure-v3/Cargo.toml

```toml
[package]
name = "unitopia-draft-measure-v3"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
repository.workspace = true

[package.metadata.details]
title = ""

[lib]
path = "src/lib.rs"

[lints]
workspace = true

[dependencies]
derive_more = { version = "2.1.1", features = ["full"] }
serde = { workspace = true, optional = true }

[package.metadata.cargo-machete]
ignored = ["serde"]
```

### packages/unitopia-draft-measure/Cargo.toml

```toml
[package]
name = "unitopia-draft-measure"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
repository.workspace = true

[package.metadata.details]
title = ""

[lib]
path = "src/lib.rs"

[lints]
workspace = true

[dependencies]
bitcode = { version = "0.6.9", features = ["derive"], optional = true }
derive-new = "0.7.0"
num-traits = "0.2.19"
rkyv = { version = "0.8.14", optional = true }
serde = { workspace = true, optional = true }
unitopia-marker-arith-outputs = { version = "0.1.0", path = "../unitopia-marker-arith-outputs" }
unitopia-helpers = { version = "0.1.0", path = "../unitopia-helpers" }
```

### packages/unitopia-draft-scale/Cargo.toml

```toml
[package]
name = "unitopia-draft-scale"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
repository.workspace = true

[package.metadata.details]
title = ""

[lib]
path = "src/lib.rs"

[lints]
workspace = true
```

### packages/unitopia-helpers/Cargo.toml

```toml
[package]
name = "unitopia-helpers"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
repository.workspace = true

[package.metadata.details]
title = ""

[lib]
path = "src/lib.rs"

[lints]
workspace = true

[dependencies]
thiserror = { workspace = true }
errgonomic = { workspace = true }
```

### packages/unitopia-marker-arith-outputs/Cargo.toml

```toml
[package]
name = "unitopia-marker-arith-outputs"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
repository.workspace = true

[package.metadata.details]
title = ""

[lib]
path = "src/lib.rs"

[lints]
workspace = true
```

### packages/unitopia-marker-prefixes/Cargo.toml

```toml
[package]
name = "unitopia-marker-prefixes"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
repository.workspace = true

[package.metadata.details]
title = ""

[lib]
path = "src/lib.rs"

[lints]
workspace = true

[dependencies]
unitopia-helpers = { version = "0.1.0", path = "../unitopia-helpers" }
```

### packages/unitopia-marker-units/Cargo.toml

```toml
[package]
name = "unitopia-marker-units"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
repository.workspace = true

[package.metadata.details]
title = ""

[lib]
path = "src/lib.rs"

[lints]
workspace = true

[dependencies]
unitopia-helpers = { version = "0.1.0", path = "../unitopia-helpers" }
unitopia-marker-arith-outputs = { version = "0.1.0", path = "../unitopia-marker-arith-outputs" }
```

### packages/unitopia-numeric-strict-wrapper-prefixes/Cargo.toml

```toml
[package]
name = "unitopia-numeric-strict-wrapper-prefixes"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
repository.workspace = true

[package.metadata.details]
title = ""

[lints]
workspace = true
```

### packages/unitopia-open-wrapper-arith-outputs/Cargo.toml

```toml
[package]
name = "unitopia-open-wrapper-arith-outputs"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
repository.workspace = true

[package.metadata.details]
title = ""

[lib]
path = "src/lib.rs"

[lints]
workspace = true
```

### packages/unitopia-poly-unit/Cargo.toml

```toml
[package]
name = "unitopia-poly-unit"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
repository.workspace = true

[package.metadata.details]
title = "Polymorphic units"

[lib]
path = "src/lib.rs"

[lints]
workspace = true

[dependencies]
errgonomic = { workspace = true }
serde = { workspace = true, optional = true }
smart-default = "0.7.1"
subtype = { git = "https://github.com/DenisGorbachev/subtype", version = "0.1.0", features = ["macros"], optional = true }
thiserror = { workspace = true }
ucum-units = "0.1.0"

[features]
serde = ["dep:serde", "dep:subtype"]
```

### packages/unitopia-runtime-unit-quantity-value/Cargo.toml

```toml
[package]
name = "unitopia-runtime-unit-quantity-value"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
repository.workspace = true

[package.metadata.details]
title = "Runtime-unit quantity value"

[lib]
path = "src/lib.rs"

[lints]
workspace = true

[dependencies]
derive-new = "0.7.0"
errgonomic = { workspace = true }
num-traits = "0.2.19"
schemars = { version = "1.2.1", optional = true }
serde = { workspace = true, optional = true }
thiserror = { workspace = true }

[package.metadata.cargo-machete]
ignored = ["schemars", "serde"]
```

### packages/unitopia-strict-wrapper-prefixes/Cargo.toml

```toml
[package]
name = "unitopia-strict-wrapper-prefixes"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
repository.workspace = true

[package.metadata.details]
title = ""

[lib]
path = "src/lib.rs"

[lints]
workspace = true

[dependencies]
bitcode = { version = "0.6.9", features = ["derive"], optional = true }
num-traits = "0.2.19"
rkyv = { version = "0.8.14", optional = true }
serde = { workspace = true, optional = true }
unitopia-helpers = { version = "0.1.0", path = "../unitopia-helpers" }
unitopia-strict-wrapper-units = { version = "0.1.0", path = "../unitopia-strict-wrapper-units" }
unitopia-open-wrapper-arith-outputs = { version = "0.1.0", path = "../unitopia-open-wrapper-arith-outputs" }
wincode = { version = "0.3.1", features = ["derive"], optional = true }

[features]
rkyv = ["dep:rkyv"]
bitcode = ["dep:bitcode"]
wincode = ["dep:wincode"]

[package.metadata.cargo-machete]
ignored = ["bitcode", "rkyv", "serde", "wincode"]
```

### packages/unitopia-strict-wrapper-units/Cargo.toml

```toml
[package]
name = "unitopia-strict-wrapper-units"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
repository.workspace = true

[package.metadata.details]
title = ""

[lib]
path = "src/lib.rs"

[lints]
workspace = true

[dependencies]
unitopia-helpers = { version = "0.1.0", path = "../unitopia-helpers" }
unitopia-open-wrapper-arith-outputs = { version = "0.1.0", path = "../unitopia-open-wrapper-arith-outputs" }
num-traits = "0.2.19"
serde = { workspace = true, optional = true }
rkyv = { version = "0.8.14", optional = true }
bitcode = { version = "0.6.9", features = ["derive"], optional = true }

#[dev-dependencies]
#trybuild = "1.0.115"
```

### packages/unitopia-test-helpers/Cargo.toml

```toml
[package]
name = "unitopia-test-helpers"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
repository.workspace = true

[package.metadata.details]
title = ""

[lib]
path = "src/lib.rs"

[lints]
workspace = true

[dependencies]
strum = { version = "0.27", features = ["derive"] }
rust_decimal = "1.40"
derive_more = { version = "2.1.1", features = ["from"] }
ruint = "1.17.2"
```

### packages/unitopia-draft-measure-v2/src/lib.rs

```rust
#![deny(clippy::arithmetic_side_effects)]

mod measure_v2;
pub use measure_v2::*;
```

### packages/unitopia-draft-measure-v3/src/lib.rs

```rust
#![deny(clippy::arithmetic_side_effects)]

mod measure_v3;
pub use measure_v3::*;
```

### packages/unitopia-draft-measure/src/lib.rs

```rust
//! Generic measure type and quantity helpers.

#![forbid(unsafe_code)]
#![deny(clippy::arithmetic_side_effects)]

#[macro_export]
macro_rules! def_quantity {
    ($quantity:ident, $measure:ident) => {
        #[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
        pub struct $quantity;

        pub type $measure<Value> = $crate::Measure<$quantity, Value>;
    };
}

mod measure;

pub use measure::*;

mod quantities;

pub use quantities::*;

mod scales;

pub use scales::*;
```

### packages/unitopia-draft-scale/src/lib.rs

```rust
#![deny(clippy::arithmetic_side_effects)]

mod upscale;
pub use upscale::*;
```

### packages/unitopia-helpers/src/lib.rs

```rust
#![no_std]
#![deny(clippy::arithmetic_side_effects)]

mod macros;
mod scalar;

pub use scalar::*;

mod has_dimension;

pub use has_dimension::*;

mod has_dimensional;

pub use has_dimensional::*;

mod has_scale;

pub use has_scale::*;

mod has_value_copy;

pub use has_value_copy::*;

mod unit;

pub use unit::*;

mod has_value_ref;

pub use has_value_ref::*;

mod exponent;

pub use exponent::*;

mod has_storage;

pub use has_storage::*;
mod quantity;
pub use quantity::*;
mod functions;
pub use functions::*;
```

### packages/unitopia-marker-arith-outputs/src/lib.rs

```rust
#![deny(clippy::arithmetic_side_effects)]

use core::marker::PhantomData;

macro_rules! define_marker_arith_output {
    ($name:ident<$lhs:ident, $rhs:ident>) => {
        #[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
        pub struct $name<$lhs, $rhs>(PhantomData<($lhs, $rhs)>);
    };
}

define_marker_arith_output!(Prod<A, B>);
define_marker_arith_output!(Quot<A, B>);
define_marker_arith_output!(Powr<A, N>);
```

### packages/unitopia-marker-prefixes/src/lib.rs

```rust
#![deny(clippy::arithmetic_side_effects)]

#[macro_use]
mod macros;
mod prefixes;
pub use prefixes::*;
```

### packages/unitopia-marker-units/src/lib.rs

```rust
#![deny(clippy::arithmetic_side_effects)]

#[macro_use]
mod macros;
mod units;
pub use units::*;
```

### packages/unitopia-numeric-strict-wrapper-prefixes/src/lib.rs

```rust
#![deny(clippy::arithmetic_side_effects)]
```

### packages/unitopia-open-wrapper-arith-outputs/src/lib.rs

```rust
#![deny(clippy::arithmetic_side_effects)]

use core::marker::PhantomData;
use core::ops::Div;

macro_rules! define_open_wrapper_arith_output {
    ($name:ident<$lhs:ident, $rhs:ident>) => {
        #[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
        #[repr(transparent)]
        pub struct $name<$lhs, $rhs, T> {
            pub inner: T,
            unit: PhantomData<($lhs, $rhs)>,
        }

        impl<$lhs, $rhs, T> $name<$lhs, $rhs, T> {
            pub const fn new(inner: T) -> Self {
                Self {
                    inner,
                    unit: PhantomData,
                }
            }
        }
    };
}

define_open_wrapper_arith_output!(Prod<A, B>);
define_open_wrapper_arith_output!(Quot<A, B>);
define_open_wrapper_arith_output!(Powr<A, N>);

macro_rules! impl_open_wrapper_arith_output {
    ($name:ident) => {
        impl<A, B, T> core::ops::Deref for $name<A, B, T>
        where
            T: core::ops::Deref,
        {
            type Target = <T as core::ops::Deref>::Target;

            fn deref(&self) -> &Self::Target {
                core::ops::Deref::deref(&self.inner)
            }
        }

        impl<A, B, T> core::ops::DerefMut for $name<A, B, T>
        where
            T: core::ops::DerefMut,
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                core::ops::DerefMut::deref_mut(&mut self.inner)
            }
        }

        impl<A, B, T, U> core::convert::AsRef<U> for $name<A, B, T>
        where
            T: core::convert::AsRef<U>,
        {
            fn as_ref(&self) -> &U {
                core::convert::AsRef::as_ref(&self.inner)
            }
        }

        impl<A, B, T> core::borrow::Borrow<T> for $name<A, B, T> {
            fn borrow(&self) -> &T {
                <T as core::borrow::Borrow<T>>::borrow(&self.inner)
            }
        }

        impl<A, B, T> core::borrow::BorrowMut<T> for $name<A, B, T> {
            fn borrow_mut(&mut self) -> &mut T {
                <T as core::borrow::BorrowMut<T>>::borrow_mut(&mut self.inner)
            }
        }

        impl<A, B, T> From<T> for $name<A, B, T> {
            fn from(inner: T) -> Self {
                Self::new(inner)
            }
        }
    };
}

impl_open_wrapper_arith_output!(Prod);
impl_open_wrapper_arith_output!(Quot);
impl_open_wrapper_arith_output!(Powr);

impl<A, B, C, D, LhsValue, RhsValue, OutValue> Div<Prod<C, D, RhsValue>> for Prod<A, B, LhsValue>
where
    LhsValue: Div<RhsValue, Output = OutValue>,
{
    type Output = Quot<Prod<A, B, LhsValue>, Prod<C, D, RhsValue>, OutValue>;

    fn div(self, rhs: Prod<C, D, RhsValue>) -> Self::Output {
        Quot::from(Div::div(self.inner, rhs.inner))
    }
}
```

### packages/unitopia-poly-unit/src/lib.rs

```rust
//! Polymorphic unit strings classified by recognized representation.

#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::arithmetic_side_effects)]

extern crate alloc;

mod types;
pub use types::*;
```

### packages/unitopia-runtime-unit-quantity-value/src/lib.rs

```rust
//! Quantity values whose units are selected at runtime.

#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::arithmetic_side_effects)]

use core::cmp::Ordering;
use core::fmt::{self, Display, Formatter};
use derive_new::new;

#[macro_use]
mod macros;

/// A numeric value paired with its runtime unit.
#[derive(new, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QuantityValue<Value, Unit> {
    /// The numeric value.
    pub value: Value,
    /// The runtime unit in which `value` is expressed.
    pub unit: Unit,
}

impl<Value: PartialEq, Unit> PartialEq<Value> for QuantityValue<Value, Unit> {
    fn eq(&self, other: &Value) -> bool {
        self.value.eq(other)
    }
}

impl<Value: PartialOrd, Unit> PartialOrd<Value> for QuantityValue<Value, Unit> {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl<Value: Display, Unit: Display> Display for QuantityValue<Value, Unit> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

define_try_quantity_value_arithmetic!(try_add, CheckedAdd, checked_add, QuantityValueTryAddError, "cannot add quantity values with different units", "quantity value addition failed",);

define_try_quantity_value_arithmetic!(try_sub, CheckedSub, checked_sub, QuantityValueTrySubError, "cannot subtract quantity values with different units", "quantity value subtraction failed",);
```

### packages/unitopia-strict-wrapper-prefixes/src/lib.rs

```rust
#![deny(clippy::arithmetic_side_effects)]

#[macro_use]
mod macros;

mod prefixes;

pub use prefixes::*;
```

### packages/unitopia-strict-wrapper-units/src/lib.rs

```rust
#![deny(clippy::arithmetic_side_effects)]

/// This trait is required to distinguish between units and scalars.
/// [`Mul`](core::ops::Mul) impl whose rhs is a unit must have a `type Output` whose outer type is `Prod`.
/// [`Mul`](core::ops::Mul) impl whose rhs is a scalar must have a `type Output = Self`.
/// The `Rhs` generic arg must be bound by [`UnitValue`], otherwise the impls above will conflict.
pub trait UnitValue {
    type Value;

    fn from_value(value: Self::Value) -> Self;
    fn value_ref(&self) -> &Self::Value;
    fn value_mut(&mut self) -> &mut Self::Value;
    fn into_value(self) -> Self::Value;
}

#[macro_use]
mod macros;
mod units;
pub use units::*;
```

### packages/unitopia-test-helpers/src/lib.rs

```rust
#![deny(clippy::arithmetic_side_effects)]

extern crate core;

mod test_input;

pub use test_input::*;

mod test_quantity;

pub use test_quantity::*;

mod test_unit;

pub use test_unit::*;

mod test_scale;

pub use test_scale::*;
mod test_value;
pub use test_value::*;
```
