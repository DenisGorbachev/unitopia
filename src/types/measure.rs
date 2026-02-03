//! Currently, [`Measure`] does not implement [`Mul`] and [`Div`] because multiplying one unit by another unit does not produce the same unit. It is possible to implement [`Mul`] and [`Div`] for [`Measure`] while specifying `Output = UnitMul<UnitA, UnitB>`, but this is a work for a future release.

use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use derive_more::{AsRef, Deref, DerefMut, From, Into};
use derive_new::new;
use num_traits::Zero;

#[derive(new, Deref, DerefMut, AsRef, From, Into, Eq, PartialEq, Ord, PartialOrd, Default, Hash, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
pub struct Measure<Quantity, Value> {
    #[cfg_attr(feature = "serde", serde(skip))]
    #[new(default)]
    quantity: PhantomData<Quantity>,
    #[deref]
    #[deref_mut]
    #[as_ref]
    #[new(into)]
    value: Value,
}

impl<Quantity, Value> Measure<Quantity, Value> {
    /// This function can be called in const contexts
    pub const fn new_const(value: Value) -> Self {
        Self {
            value,
            quantity: PhantomData,
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

// impl<Quantity: Display, Value: Display> Display for Measure<Quantity, Value> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} {}", self.value, self.quantity)
//     }
// }

impl<Quantity, Value> From<Value> for Measure<Quantity, Value> {
    fn from(value: Value) -> Self {
        Self::new_const(value)
    }
}

impl<Quantity, Value> Zero for Measure<Quantity, Value>
where
    Value: Zero,
{
    fn zero() -> Self {
        Self::new_const(Value::zero())
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

// TODO: Try resolving the error when implementing this trait
// impl<Quantity, Value> From<Measure<Quantity, Value>> for Value {
//     fn from(measure: Measure<Quantity, Value>) -> Self {
//         measure.value
//     }
// }

macro_rules! impl_binop_self {
    ($trait_name:ident, $method:ident) => {
        impl<Quantity, Value> $trait_name<Self> for Measure<Quantity, Value>
        where
            Value: $trait_name<Value, Output = Value>,
        {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self {
                Self {
                    value: self.value.$method(rhs.value),
                    quantity: self.quantity,
                }
            }
        }
    };
}

macro_rules! impl_binop_ref_self {
    ($trait_name:ident, $method:ident) => {
        impl<'a, Quantity, Value> $trait_name<&'a Self> for Measure<Quantity, Value>
        where
            Value: $trait_name<&'a Value, Output = Value>,
        {
            type Output = Self;

            fn $method(self, rhs: &'a Self) -> Self {
                Self {
                    value: self.value.$method(&rhs.value),
                    quantity: self.quantity,
                }
            }
        }
    };
}

macro_rules! impl_binop_assign_self {
    ($trait_name:ident, $method:ident) => {
        impl<Quantity, Value> $trait_name<Self> for Measure<Quantity, Value>
        where
            Value: $trait_name<Value>,
        {
            fn $method(&mut self, rhs: Self) {
                self.value.$method(rhs.value);
            }
        }
    };
}

macro_rules! impl_binop_assign_ref_self {
    ($trait_name:ident, $method:ident) => {
        impl<'a, Quantity, Value> $trait_name<&'a Self> for Measure<Quantity, Value>
        where
            Value: $trait_name<&'a Value>,
        {
            fn $method(&mut self, rhs: &'a Self) {
                self.value.$method(&rhs.value);
            }
        }
    };
}

macro_rules! impl_binop_value {
    ($trait_name:ident, $method:ident) => {
        impl<Quantity, Value> $trait_name<Value> for Measure<Quantity, Value>
        where
            Value: $trait_name<Value, Output = Value>,
        {
            type Output = Self;

            fn $method(self, value: Value) -> Self {
                Self {
                    value: self.value.$method(value),
                    quantity: self.quantity,
                }
            }
        }
    };
}

macro_rules! impl_binop_ref_value {
    ($trait_name:ident, $method:ident) => {
        impl<'a, Quantity, Value> $trait_name<&'a Value> for Measure<Quantity, Value>
        where
            Value: $trait_name<&'a Value, Output = Value>,
        {
            type Output = Self;

            fn $method(self, value: &'a Value) -> Self {
                Self {
                    value: self.value.$method(value),
                    quantity: self.quantity,
                }
            }
        }
    };
}

macro_rules! impl_binop_assign_value {
    ($trait_name:ident, $method:ident) => {
        impl<Quantity, Value> $trait_name<Value> for Measure<Quantity, Value>
        where
            Value: $trait_name<Value>,
        {
            fn $method(&mut self, value: Value) {
                self.value.$method(value);
            }
        }
    };
}

// IMPORTANT: Don't implement Mul and Div for Measure, since they must return a different unit
impl_binop_self!(Add, add);
impl_binop_self!(Sub, sub);
impl_binop_ref_self!(Add, add);
impl_binop_ref_self!(Sub, sub);
impl_binop_assign_self!(AddAssign, add_assign);
impl_binop_assign_self!(SubAssign, sub_assign);
impl_binop_assign_ref_self!(AddAssign, add_assign);
impl_binop_assign_ref_self!(SubAssign, sub_assign);

// Do implement Mul and Div for values (multiplication and division of Measure by scalar values)
impl_binop_value!(Add, add);
impl_binop_value!(Sub, sub);
impl_binop_value!(Mul, mul);
impl_binop_value!(Div, div);
impl_binop_ref_value!(Add, add);
impl_binop_ref_value!(Sub, sub);
impl_binop_ref_value!(Mul, mul);
impl_binop_ref_value!(Div, div);
impl_binop_assign_value!(AddAssign, add_assign);
impl_binop_assign_value!(SubAssign, sub_assign);
impl_binop_assign_value!(MulAssign, mul_assign);
impl_binop_assign_value!(DivAssign, div_assign);
// TODO: impl_binop_assign_ref_value!

// TODO: Implement checked, wrapping, overflowing, saturating variants as `num-traits`

#[cfg(test)]
pub mod test_templates {
    use super::*;

    pub fn must_implement_std_ops_on_self<Quantity: Clone, Value: Clone + Add<Value, Output = Value> + for<'a> Add<&'a Value, Output = Value> + Mul<Value, Output = Value> + for<'a> Mul<&'a Value, Output = Value>>(a: Measure<Quantity, Value>, b: Measure<Quantity, Value>) {
        let _ = a.clone() + b.clone();
        let _ = a.clone() + &b;
    }

    pub fn must_implement_std_ops_on_value<Quantity: Clone, Value: Clone + Add<Value, Output = Value> + for<'a> Add<&'a Value, Output = Value> + Mul<Value, Output = Value> + for<'a> Mul<&'a Value, Output = Value>>(a: Measure<Quantity, Value>, value: Value) {
        let _ = a.clone() + value.clone();
        let _ = a.clone() + &value;
    }
}
