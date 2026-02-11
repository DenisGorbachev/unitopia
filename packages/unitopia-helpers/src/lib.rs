#![no_std]
#![deny(clippy::arithmetic_side_effects)]

mod macros;
mod scalar;

pub use scalar::*;

mod has_dimensional;

pub use has_dimensional::*;

mod has_scale;

pub use has_scale::*;

mod has_value_copy;

pub use has_value_copy::*;

mod unital;

pub use unital::*;

mod has_value_ref;

pub use has_value_ref::*;
