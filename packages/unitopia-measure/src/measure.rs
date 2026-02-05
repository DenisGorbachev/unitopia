use crate::{DivUnit, Exponent, MulUnit, PowUnit};
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use num_traits::ops::overflowing::{OverflowingAdd, OverflowingSub};
use num_traits::{CheckedAdd, CheckedSub, ConstZero, MulAdd, MulAddAssign, Pow, SaturatingAdd, SaturatingSub, WrappingAdd, WrappingSub, Zero};

#[derive(derive_new::new, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[repr(transparent)]
pub struct Measure<Unit, Value> {
    #[new(into)]
    value: Value,
    #[cfg_attr(feature = "serde", serde(skip))]
    #[new(default)]
    unit: PhantomData<Unit>,
}

impl<Unit, Value> Measure<Unit, Value> {
    /// Constructs a measure in const contexts.
    pub const fn new_const(value: Value) -> Self {
        Self {
            value,
            unit: PhantomData,
        }
    }

    /// Returns a copy of the stored value.
    pub const fn value(&self) -> Value
    where
        Value: Copy,
    {
        self.value
    }

    /// Returns a shared reference to the stored value.
    pub const fn value_ref(&self) -> &Value {
        &self.value
    }

    /// Returns a mutable reference to the stored value.
    pub fn value_mut(&mut self) -> &mut Value {
        &mut self.value
    }

    /// Consumes the measure and returns the stored value.
    pub fn into_value(self) -> Value {
        self.value
    }
}

impl<Unit, Value> From<Value> for Measure<Unit, Value> {
    fn from(value: Value) -> Self {
        Self::new_const(value)
    }
}

impl<Unit, Value> ConstZero for Measure<Unit, Value>
where
    Value: ConstZero,
{
    const ZERO: Self = Self::new_const(Value::ZERO);
}

impl<Unit, Value> Zero for Measure<Unit, Value>
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

macro_rules! impl_binop_self {
    ($trait_name:ident, $method:ident) => {
        impl<Unit, Value> $trait_name for Measure<Unit, Value>
        where
            Value: $trait_name<Value, Output = Value>,
        {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self {
                Self::new_const(self.value.$method(rhs.value))
            }
        }
    };
}

macro_rules! impl_binop_assign_self {
    ($trait_name:ident, $method:ident) => {
        impl<Unit, Value> $trait_name for Measure<Unit, Value>
        where
            Value: $trait_name<Value>,
        {
            fn $method(&mut self, rhs: Self) {
                self.value.$method(rhs.value);
            }
        }
    };
}

macro_rules! impl_binop_scalar {
    ($trait_name:ident, $method:ident) => {
        impl<Unit, Value> $trait_name<Value> for Measure<Unit, Value>
        where
            Value: $trait_name<Value, Output = Value>,
        {
            type Output = Self;

            fn $method(self, rhs: Value) -> Self {
                Self::new_const(self.value.$method(rhs))
            }
        }
    };
}

macro_rules! impl_binop_assign_scalar {
    ($trait_name:ident, $method:ident) => {
        impl<Unit, Value> $trait_name<Value> for Measure<Unit, Value>
        where
            Value: $trait_name<Value>,
        {
            fn $method(&mut self, rhs: Value) {
                self.value.$method(rhs);
            }
        }
    };
}

macro_rules! impl_binop_measure {
    ($trait_name:ident, $method:ident, $unit:ident) => {
        impl<LhsUnit, RhsUnit, LhsValue, RhsValue, OutValue> $trait_name<Measure<RhsUnit, RhsValue>> for Measure<LhsUnit, LhsValue>
        where
            LhsValue: $trait_name<RhsValue, Output = OutValue>,
        {
            type Output = Measure<$unit<LhsUnit, RhsUnit>, OutValue>;

            fn $method(self, rhs: Measure<RhsUnit, RhsValue>) -> Self::Output {
                Measure::<$unit<LhsUnit, RhsUnit>, OutValue>::new_const(self.value.$method(rhs.value))
            }
        }
    };
}

macro_rules! impl_checked_binop_self {
    ($trait_name:ident, $method:ident) => {
        impl<Unit, Value> $trait_name for Measure<Unit, Value>
        where
            Value: $trait_name,
        {
            fn $method(&self, rhs: &Self) -> Option<Self> {
                self.value.$method(&rhs.value).map(Self::new_const)
            }
        }
    };
}

macro_rules! impl_saturating_binop_self {
    ($trait_name:ident, $method:ident) => {
        impl<Unit, Value> $trait_name for Measure<Unit, Value>
        where
            Value: $trait_name,
        {
            fn $method(&self, rhs: &Self) -> Self {
                Self::new_const(self.value.$method(&rhs.value))
            }
        }
    };
}

macro_rules! impl_wrapping_binop_self {
    ($trait_name:ident, $method:ident) => {
        impl<Unit, Value> $trait_name for Measure<Unit, Value>
        where
            Value: $trait_name,
        {
            fn $method(&self, rhs: &Self) -> Self {
                Self::new_const(self.value.$method(&rhs.value))
            }
        }
    };
}

macro_rules! impl_overflowing_binop_self {
    ($trait_name:ident, $method:ident) => {
        impl<Unit, Value> $trait_name for Measure<Unit, Value>
        where
            Value: $trait_name,
        {
            fn $method(&self, rhs: &Self) -> (Self, bool) {
                let (value, overflowed) = self.value.$method(&rhs.value);
                (Self::new_const(value), overflowed)
            }
        }
    };
}

impl_binop_self!(Add, add);
impl_binop_self!(Sub, sub);
impl_binop_assign_self!(AddAssign, add_assign);
impl_binop_assign_self!(SubAssign, sub_assign);

impl_binop_scalar!(Mul, mul);
impl_binop_scalar!(Div, div);
impl_binop_assign_scalar!(MulAssign, mul_assign);
impl_binop_assign_scalar!(DivAssign, div_assign);

impl_binop_measure!(Mul, mul, MulUnit);
impl_binop_measure!(Div, div, DivUnit);

impl_checked_binop_self!(CheckedAdd, checked_add);
impl_checked_binop_self!(CheckedSub, checked_sub);

impl_saturating_binop_self!(SaturatingAdd, saturating_add);
impl_saturating_binop_self!(SaturatingSub, saturating_sub);

impl_wrapping_binop_self!(WrappingAdd, wrapping_add);
impl_wrapping_binop_self!(WrappingSub, wrapping_sub);

impl_overflowing_binop_self!(OverflowingAdd, overflowing_add);
impl_overflowing_binop_self!(OverflowingSub, overflowing_sub);

impl<LhsUnit, RhsUnit, Value> MulAdd<Measure<RhsUnit, Value>, Measure<MulUnit<LhsUnit, RhsUnit>, Value>> for Measure<LhsUnit, Value>
where
    Value: MulAdd<Value, Value, Output = Value>,
{
    type Output = Measure<MulUnit<LhsUnit, RhsUnit>, Value>;

    fn mul_add(self, a: Measure<RhsUnit, Value>, b: Measure<MulUnit<LhsUnit, RhsUnit>, Value>) -> Self::Output {
        let b_value = b.into_value();
        Measure::<MulUnit<LhsUnit, RhsUnit>, Value>::new_const(self.value.mul_add(a.value, b_value))
    }
}

impl<Unit, Value> MulAdd<Value, Measure<Unit, Value>> for Measure<Unit, Value>
where
    Value: MulAdd<Value, Value, Output = Value>,
{
    type Output = Self;

    fn mul_add(self, a: Value, b: Measure<Unit, Value>) -> Self::Output {
        Self::new_const(self.value.mul_add(a, b.value))
    }
}

impl<Unit, Value> MulAddAssign<Value, Measure<Unit, Value>> for Measure<Unit, Value>
where
    Value: MulAddAssign<Value, Value>,
{
    fn mul_add_assign(&mut self, a: Value, b: Measure<Unit, Value>) {
        self.value.mul_add_assign(a, b.value);
    }
}

impl<Unit, Value, const POWER: u32> Pow<Exponent<POWER>> for Measure<Unit, Value>
where
    Value: Pow<u32, Output = Value>,
{
    type Output = Measure<PowUnit<Unit, POWER>, Value>;

    fn pow(self, _rhs: Exponent<POWER>) -> Self::Output {
        Measure::<PowUnit<Unit, POWER>, Value>::new_const(self.value.pow(POWER))
    }
}

#[cfg(feature = "wincode")]
unsafe impl<Unit, Value, C> wincode::SchemaWrite<C> for Measure<Unit, Value>
where
    C: wincode::config::ConfigCore,
    Value: wincode::SchemaWrite<C, Src = Value>,
{
    type Src = Self;

    const TYPE_META: wincode::TypeMeta = <Value as wincode::SchemaWrite<C>>::TYPE_META;

    fn size_of(src: &Self::Src) -> wincode::WriteResult<usize> {
        <Value as wincode::SchemaWrite<C>>::size_of(&src.value)
    }

    fn write(writer: &mut impl wincode::io::Writer, src: &Self::Src) -> wincode::WriteResult<()> {
        <Value as wincode::SchemaWrite<C>>::write(writer, &src.value)
    }
}

#[cfg(feature = "wincode")]
unsafe impl<'de, Unit, Value, C> wincode::SchemaRead<'de, C> for Measure<Unit, Value>
where
    C: wincode::config::ConfigCore,
    Value: wincode::SchemaRead<'de, C, Dst = Value>,
{
    type Dst = Self;

    const TYPE_META: wincode::TypeMeta = <Value as wincode::SchemaRead<'de, C>>::TYPE_META;

    fn read(reader: &mut impl wincode::io::Reader<'de>, dst: &mut core::mem::MaybeUninit<Self::Dst>) -> wincode::ReadResult<()> {
        let mut value = core::mem::MaybeUninit::uninit();
        match <Value as wincode::SchemaRead<'de, C>>::read(reader, &mut value) {
            Ok(()) => {
                let value = unsafe { value.assume_init() };
                dst.write(Self::new_const(value));
                Ok(())
            }
            Err(error) => Err(error),
        }
    }
}
