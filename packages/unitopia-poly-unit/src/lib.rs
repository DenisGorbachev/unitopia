//! Polymorphic unit strings classified by recognized representation.

#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::arithmetic_side_effects)]
#![cfg_attr(not(test), deny(unused_crate_dependencies))]

extern crate alloc;

mod types;
pub use types::*;
