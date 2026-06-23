use derive_more::{AsMut, AsRef, From, Into};
use std::marker::PhantomData;
use std::ops::{Add, Mul, Sub};

/// BLOCKED: Rust does not currently support combining powers in const-generic expressions for this use case.
#[derive(AsRef, AsMut, From, Into, Eq, PartialEq, Ord, PartialOrd, Default, Hash, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct MeasureV3<Value, Unit, const POWER: i64> {
    #[as_ref]
    #[as_mut]
    value: Value,
    unit: PhantomData<Unit>,
}

/// Temporary placeholder for unsupported type-level power composition.
const BLOCKED_POWER: i64 = i64::from_ne_bytes([u8::MAX; size_of::<i64>()]);

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

/// `BLOCKED_POWER` in the third argument of `MeasureV3` is temporary.
macro_rules! impl_binop_self_same_unit_diff_power {
    ($trait_name:ident, $method:ident) => {
        impl<LhsValue, RhsValue, OutValue, Unit, const LHS_POWER: i64, const RHS_POWER: i64> $trait_name<MeasureV3<RhsValue, Unit, RHS_POWER>> for MeasureV3<LhsValue, Unit, LHS_POWER>
        where
            LhsValue: $trait_name<RhsValue, Output = OutValue>,
        {
            type Output = MeasureV3<OutValue, Unit, BLOCKED_POWER>;

            fn $method(self, rhs: MeasureV3<RhsValue, Unit, RHS_POWER>) -> MeasureV3<OutValue, Unit, BLOCKED_POWER> {
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
mod gram;
#[allow(unused_imports)]
pub use gram::*;
mod meter;
#[allow(unused_imports)]
pub use meter::*;
