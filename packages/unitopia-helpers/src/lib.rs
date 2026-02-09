#![no_std]
#![deny(clippy::arithmetic_side_effects)]

pub trait Scale {
    const NUM: u128;
    const DEN: u128;
}

mod macros;
mod scalar;

pub use scalar::*;
