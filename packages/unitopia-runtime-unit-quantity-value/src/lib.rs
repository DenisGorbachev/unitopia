//! Quantity values whose units are selected at runtime.

#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::arithmetic_side_effects)]
#![cfg_attr(not(test), deny(unused_crate_dependencies))]

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
