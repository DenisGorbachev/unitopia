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
