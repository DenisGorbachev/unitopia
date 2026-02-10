#![no_std]
#![deny(clippy::arithmetic_side_effects)]

mod macros;
mod scalar;

pub use scalar::*;

mod unit;

pub use unit::*;

mod scale;

pub use scale::*;
