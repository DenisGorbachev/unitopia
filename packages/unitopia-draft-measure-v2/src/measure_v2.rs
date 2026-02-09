use derive_more::{AsMut, AsRef, From, Into};
use derive_new::new;
use std::marker::PhantomData;
use std::ops::{Add, Sub};

/// BLOCKED: I can't express the "same base unit but different power" if the power is a const generic within the unit (see `impl_binop_self_same_unit_diff_power` in this file)
#[derive(new, AsRef, AsMut, From, Into, Eq, PartialEq, Ord, PartialOrd, Default, Hash, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct MeasureV2<Value, Unit> {
    #[as_ref]
    #[as_mut]
    #[new(into)]
    value: Value,
    #[new(default)]
    unit: PhantomData<Unit>,
}

impl<Unit, Value> MeasureV2<Value, Unit> {
    /// This function can be called in const contexts
    pub const fn new_const(value: Value) -> Self {
        Self {
            value,
            unit: PhantomData,
        }
    }

    /// This function returns a copy of the value
    ///
    /// Use `as_ref()` to obtain a reference to the value
    pub const fn value(&self) -> Value
    where
        Value: Copy,
    {
        self.value
    }
}

macro_rules! impl_binop_self_same_unit_same_power {
    ($trait_name:ident, $method:ident) => {
        impl<LhsValue, RhsValue, OutValue, Unit> $trait_name<MeasureV2<RhsValue, Unit>> for MeasureV2<LhsValue, Unit>
        where
            LhsValue: $trait_name<RhsValue, Output = OutValue>,
        {
            type Output = MeasureV2<OutValue, Unit>;

            fn $method(self, rhs: MeasureV2<RhsValue, Unit>) -> MeasureV2<OutValue, Unit> {
                MeasureV2 {
                    value: self.value.$method(rhs.value),
                    unit: self.unit,
                }
            }
        }
    };
}

// macro_rules! impl_binop_self_same_unit_diff_power {
//     ($trait_name:ident, $method:ident) => {
//         impl<LhsValue, LhsUnit, RhsValue, RhsUnit, OutValue, OutUnit> $trait_name<MeasureV2<RhsValue, RhsUnit>> for MeasureV2<LhsValue, LhsUnit>
//         where
//             LhsValue: $trait_name<RhsValue, Output = OutValue>,
//             LhsUnit: $trait_name<RhsUnit, Output = OutUnit>,
//         {
//             type Output = MeasureV2<OutValue, OutUnit>;
//
//             fn $method(self, rhs: MeasureV2<RhsValue, RhsUnit>) -> MeasureV2<OutValue, OutUnit> {
//                 MeasureV2 {
//                     value: self.value.$method(rhs.value),
//                     unit: self.unit.$method(rhs.unit),
//                 }
//             }
//         }
//     };
// }

impl_binop_self_same_unit_same_power!(Add, add);
impl_binop_self_same_unit_same_power!(Sub, sub);
// impl_binop_self_same_unit_diff_power!(Mul, mul);
// impl_binop_self_same_unit_diff_power!(Div, div);

mod gram;
#[allow(unused_imports)]
pub use gram::*;
mod meter;
#[allow(unused_imports)]
pub use meter::*;
