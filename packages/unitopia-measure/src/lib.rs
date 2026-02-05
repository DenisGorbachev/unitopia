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
mod mul_unit;
pub use mul_unit::*;
mod div_unit;
pub use div_unit::*;
mod pow_unit;
pub use pow_unit::*;
mod exponent;
pub use exponent::*;
mod quantities;
pub use quantities::*;
mod scales;
pub use scales::*;
