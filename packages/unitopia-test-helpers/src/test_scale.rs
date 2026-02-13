#[allow(unused_imports)]
use TestScale::*;
use strum::Display;

#[derive(Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum TestScale {
    Quecto,
    Quetta,
    Kilo,
    Milli,
    Uno,
    Tetravigesi,
}

impl TestScale {
    /// Returns a pair of (numerator, denominator)
    pub const fn to_numbers(self) -> (u128, u128) {
        todo!()
    }

    pub const fn num(self) -> u128 {
        self.to_numbers().0
    }

    pub const fn den(self) -> u128 {
        self.to_numbers().1
    }
}
