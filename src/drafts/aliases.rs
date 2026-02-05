#![cfg(feature = "num")]

use crate::{Measure, Second};
use num::rational::Ratio;

pub type Seconds<Value> = Measure<Second, Value>;

pub const MILLISECOND_U64: Seconds<Ratio<u64>> = Seconds::new_const(Ratio::<u64>::new_raw(1, 1000));
