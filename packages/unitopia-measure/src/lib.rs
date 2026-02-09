#![forbid(unsafe_code)]
#![deny(clippy::arithmetic_side_effects)]

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
pub use unitopia_marker_arith_outputs::Powr;
pub use unitopia_marker_arith_outputs::{Prod, Quot};

mod exponent;

pub use exponent::*;

mod quantities;

pub use quantities::*;

mod scales;

pub use scales::*;
