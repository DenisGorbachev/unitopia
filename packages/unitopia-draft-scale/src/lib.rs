#![deny(clippy::arithmetic_side_effects)]

#[cfg(feature = "typenum")]
mod scale;

#[cfg(feature = "typenum")]
pub use scale::*;
