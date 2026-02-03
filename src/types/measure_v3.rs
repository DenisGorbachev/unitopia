use derive_more::{AsMut, AsRef, From, Into};
use std::marker::PhantomData;
use std::ops::{Add, Mul, Sub};

/// BLOCKED: Rust doesn't support arithmetic in const generics (e.g. `LHS_POWER + RHS_POWER`)
#[derive(AsRef, AsMut, From, Into, Eq, PartialEq, Ord, PartialOrd, Default, Hash, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MeasureV3<Value, Unit, const POWER: i64> {
    #[as_ref]
    #[as_mut]
    value: Value,
    unit: PhantomData<Unit>,
}

macro_rules! impl_binop_self_same_unit_same_power {
    ($trait_name:ident, $method:ident) => {
        impl<LhsValue, RhsValue, OutValue, Unit, const POWER: i64> $trait_name<MeasureV3<RhsValue, Unit, POWER>> for MeasureV3<LhsValue, Unit, POWER>
        where
            LhsValue: $trait_name<RhsValue, Output = OutValue>,
        {
            type Output = MeasureV3<OutValue, Unit, POWER>;

            fn $method(self, rhs: MeasureV3<RhsValue, Unit, POWER>) -> MeasureV3<OutValue, Unit, POWER> {
                MeasureV3 {
                    value: self.value.$method(rhs.value),
                    unit: self.unit,
                }
            }
        }
    };
}

/// `-1` in the third argument of `MeasureV3` is temporary
macro_rules! impl_binop_self_same_unit_diff_power {
    ($trait_name:ident, $method:ident) => {
        impl<LhsValue, RhsValue, OutValue, Unit, const LHS_POWER: i64, const RHS_POWER: i64> $trait_name<MeasureV3<RhsValue, Unit, RHS_POWER>> for MeasureV3<LhsValue, Unit, LHS_POWER>
        where
            LhsValue: $trait_name<RhsValue, Output = OutValue>,
        {
            type Output = MeasureV3<OutValue, Unit, /* { LHS_POWER + RHS_POWER } */ -1>;

            fn $method(self, rhs: MeasureV3<RhsValue, Unit, RHS_POWER>) -> MeasureV3<OutValue, Unit, /* { LHS_POWER + RHS_POWER } */ -1> {
                MeasureV3 {
                    value: self.value.$method(rhs.value),
                    unit: self.unit,
                }
            }
        }
    };
}

impl_binop_self_same_unit_same_power!(Add, add);
impl_binop_self_same_unit_same_power!(Sub, sub);

// TODO: Div requires subtracting the POWER
impl_binop_self_same_unit_diff_power!(Mul, mul);
// pub struct Foo<const POWER: i32> {
//     value: bool,
// }
//
// fn bar(a: Foo<{ 3 + 2 }>, b: Foo<5>) {}
mod gram;
#[allow(unused_imports)]
pub use gram::*;
mod meter;
#[allow(unused_imports)]
pub use meter::*;
