#![deny(clippy::arithmetic_side_effects)]
#![cfg_attr(not(test), deny(unused_crate_dependencies))]

#[macro_use]
mod macros;
mod units;
pub use units::*;
