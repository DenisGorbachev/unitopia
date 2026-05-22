#![forbid(unsafe_code)]
#![deny(clippy::arithmetic_side_effects)]
#![cfg_attr(not(test), deny(unused_crate_dependencies))]

//! Generic measure type and quantity helpers.

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
