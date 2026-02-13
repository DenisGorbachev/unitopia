#[allow(unused_imports)]
use TestUnit::*;
use strum::Display;

#[derive(Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum TestUnit {
    Second,
    Meter,
    Kilogram,
    Newton,
}

impl TestUnit {}
