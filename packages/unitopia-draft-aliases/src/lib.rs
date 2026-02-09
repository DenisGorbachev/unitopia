#![cfg(feature = "num")]
#![deny(clippy::arithmetic_side_effects)]

use num::rational::Ratio;
use unitopia_marker_units::Second;
use unitopia_measure::Measure;

pub type Seconds<Value> = Measure<Second, Value>;

pub const MILLISECOND_U64: Seconds<Ratio<u64>> = Seconds::new_const(Ratio::<u64>::new_raw(1, 1000));
