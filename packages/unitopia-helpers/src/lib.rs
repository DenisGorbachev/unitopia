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
