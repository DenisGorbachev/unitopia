pub trait UnitValue {
    type Value;

    fn from_value(value: Self::Value) -> Self;
    fn value_ref(&self) -> &Self::Value;
    fn value_mut(&mut self) -> &mut Self::Value;
    fn into_value(self) -> Self::Value;
}

#[macro_export]
macro_rules! define_strict_wrapper_unit {
    ($name:ident, $unit_marker:path) => {
        unitopia_helpers::define_strict_wrapper_struct!(
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
            #[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
            $name
        );

        impl<T> $crate::UnitValue for $name<T> {
            type Value = T;

            fn from_value(value: Self::Value) -> Self {
                Self {
                    inner: value,
                }
            }

            fn value_ref(&self) -> &Self::Value {
                &self.inner
            }

            fn value_mut(&mut self) -> &mut Self::Value {
                &mut self.inner
            }

            fn into_value(self) -> Self::Value {
                self.inner
            }
        }

        #[cfg(feature = "wincode")]
        unsafe impl<T, C> wincode::SchemaWrite<C> for $name<T>
        where
            C: wincode::config::ConfigCore,
            T: wincode::SchemaWrite<C, Src = T>,
        {
            type Src = Self;

            const TYPE_META: wincode::TypeMeta = <T as wincode::SchemaWrite<C>>::TYPE_META;

            fn size_of(src: &Self::Src) -> wincode::WriteResult<usize> {
                <T as wincode::SchemaWrite<C>>::size_of(&src.inner)
            }

            fn write(writer: &mut impl wincode::io::Writer, src: &Self::Src) -> wincode::WriteResult<()> {
                <T as wincode::SchemaWrite<C>>::write(writer, &src.inner)
            }
        }

        #[cfg(feature = "wincode")]
        unsafe impl<'de, T, C> wincode::SchemaRead<'de, C> for $name<T>
        where
            C: wincode::config::ConfigCore,
            T: wincode::SchemaRead<'de, C, Dst = T>,
        {
            type Dst = Self;

            const TYPE_META: wincode::TypeMeta = <T as wincode::SchemaRead<'de, C>>::TYPE_META;

            fn read(reader: &mut impl wincode::io::Reader<'de>, dst: &mut core::mem::MaybeUninit<Self::Dst>) -> wincode::ReadResult<()> {
                let mut value = core::mem::MaybeUninit::uninit();
                match <T as wincode::SchemaRead<'de, C>>::read(reader, &mut value) {
                    Ok(()) => {
                        let value = unsafe { value.assume_init() };
                        dst.write(Self {
                            inner: value,
                        });
                        Ok(())
                    }
                    Err(error) => Err(error),
                }
            }
        }
    };
}

macro_rules! impl_unit_identity_traits {
    ($unit:ident) => {
        impl<T> num_traits::ConstZero for $unit<T>
        where
            T: num_traits::ConstZero,
        {
            const ZERO: Self = Self {
                inner: T::ZERO,
            };
        }

        impl<T> num_traits::Zero for $unit<T>
        where
            T: num_traits::Zero,
        {
            fn zero() -> Self {
                Self {
                    inner: T::zero(),
                }
            }

            fn is_zero(&self) -> bool {
                self.inner.is_zero()
            }
        }
    };
}

macro_rules! impl_unit_add_sub_traits {
    ($unit:ident) => {
        impl<T> core::ops::Add for $unit<T>
        where
            T: core::ops::Add<T, Output = T>,
        {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self {
                    inner: core::ops::Add::add(self.inner, rhs.inner),
                }
            }
        }

        impl<T> core::ops::Sub for $unit<T>
        where
            T: core::ops::Sub<T, Output = T>,
        {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self {
                    inner: core::ops::Sub::sub(self.inner, rhs.inner),
                }
            }
        }

        impl<T> core::ops::AddAssign for $unit<T>
        where
            T: core::ops::AddAssign<T>,
        {
            fn add_assign(&mut self, rhs: Self) {
                core::ops::AddAssign::add_assign(&mut self.inner, rhs.inner);
            }
        }

        impl<T> core::ops::SubAssign for $unit<T>
        where
            T: core::ops::SubAssign<T>,
        {
            fn sub_assign(&mut self, rhs: Self) {
                core::ops::SubAssign::sub_assign(&mut self.inner, rhs.inner);
            }
        }

        impl<T> num_traits::CheckedAdd for $unit<T>
        where
            T: num_traits::CheckedAdd,
        {
            fn checked_add(&self, rhs: &Self) -> Option<Self> {
                num_traits::CheckedAdd::checked_add(&self.inner, &rhs.inner).map(|value| Self {
                    inner: value,
                })
            }
        }

        impl<T> num_traits::CheckedSub for $unit<T>
        where
            T: num_traits::CheckedSub,
        {
            fn checked_sub(&self, rhs: &Self) -> Option<Self> {
                num_traits::CheckedSub::checked_sub(&self.inner, &rhs.inner).map(|value| Self {
                    inner: value,
                })
            }
        }

        impl<T> num_traits::SaturatingAdd for $unit<T>
        where
            T: num_traits::SaturatingAdd,
        {
            fn saturating_add(&self, rhs: &Self) -> Self {
                Self {
                    inner: num_traits::SaturatingAdd::saturating_add(&self.inner, &rhs.inner),
                }
            }
        }

        impl<T> num_traits::SaturatingSub for $unit<T>
        where
            T: num_traits::SaturatingSub,
        {
            fn saturating_sub(&self, rhs: &Self) -> Self {
                Self {
                    inner: num_traits::SaturatingSub::saturating_sub(&self.inner, &rhs.inner),
                }
            }
        }

        impl<T> num_traits::WrappingAdd for $unit<T>
        where
            T: num_traits::WrappingAdd,
        {
            fn wrapping_add(&self, rhs: &Self) -> Self {
                Self {
                    inner: num_traits::WrappingAdd::wrapping_add(&self.inner, &rhs.inner),
                }
            }
        }

        impl<T> num_traits::WrappingSub for $unit<T>
        where
            T: num_traits::WrappingSub,
        {
            fn wrapping_sub(&self, rhs: &Self) -> Self {
                Self {
                    inner: num_traits::WrappingSub::wrapping_sub(&self.inner, &rhs.inner),
                }
            }
        }

        impl<T> num_traits::ops::overflowing::OverflowingAdd for $unit<T>
        where
            T: num_traits::ops::overflowing::OverflowingAdd,
        {
            fn overflowing_add(&self, rhs: &Self) -> (Self, bool) {
                let (value, overflowed) = num_traits::ops::overflowing::OverflowingAdd::overflowing_add(&self.inner, &rhs.inner);
                (
                    Self {
                        inner: value,
                    },
                    overflowed,
                )
            }
        }

        impl<T> num_traits::ops::overflowing::OverflowingSub for $unit<T>
        where
            T: num_traits::ops::overflowing::OverflowingSub,
        {
            fn overflowing_sub(&self, rhs: &Self) -> (Self, bool) {
                let (value, overflowed) = num_traits::ops::overflowing::OverflowingSub::overflowing_sub(&self.inner, &rhs.inner);
                (
                    Self {
                        inner: value,
                    },
                    overflowed,
                )
            }
        }
    };
}

macro_rules! impl_unit_scalar_mul_div_traits {
    ($unit:ident) => {
        impl<T> core::ops::Mul<crate::Scalar<T>> for $unit<T>
        where
            T: core::ops::Mul<T, Output = T>,
        {
            type Output = Self;

            fn mul(self, rhs: crate::Scalar<T>) -> Self {
                Self {
                    inner: core::ops::Mul::mul(self.inner, rhs.inner),
                }
            }
        }

        impl<T> core::ops::Div<crate::Scalar<T>> for $unit<T>
        where
            T: core::ops::Div<T, Output = T>,
        {
            type Output = Self;

            fn div(self, rhs: crate::Scalar<T>) -> Self {
                Self {
                    inner: core::ops::Div::div(self.inner, rhs.inner),
                }
            }
        }

        impl<T> core::ops::MulAssign<T> for $unit<T>
        where
            T: core::ops::MulAssign<T>,
        {
            fn mul_assign(&mut self, rhs: T) {
                core::ops::MulAssign::mul_assign(&mut self.inner, rhs);
            }
        }

        impl<T> core::ops::DivAssign<T> for $unit<T>
        where
            T: core::ops::DivAssign<T>,
        {
            fn div_assign(&mut self, rhs: T) {
                core::ops::DivAssign::div_assign(&mut self.inner, rhs);
            }
        }
    };
}

macro_rules! impl_unit_mul_div_measure_traits {
    ($unit:ident) => {
        impl<LhsValue, Rhs, OutValue> core::ops::Mul<Rhs> for $unit<LhsValue>
        where
            Rhs: crate::UnitValue,
            LhsValue: core::ops::Mul<Rhs::Value, Output = OutValue>,
        {
            type Output = unitopia_open_wrapper_arith_outputs::Prod<$unit<LhsValue>, Rhs, OutValue>;

            fn mul(self, rhs: Rhs) -> Self::Output {
                let rhs_value = <Rhs as crate::UnitValue>::into_value(rhs);
                unitopia_open_wrapper_arith_outputs::Prod::from(core::ops::Mul::mul(self.inner, rhs_value))
            }
        }

        impl<LhsValue, Rhs, OutValue> core::ops::Div<Rhs> for $unit<LhsValue>
        where
            Rhs: crate::UnitValue,
            LhsValue: core::ops::Div<Rhs::Value, Output = OutValue>,
        {
            type Output = unitopia_open_wrapper_arith_outputs::Quot<$unit<LhsValue>, Rhs, OutValue>;

            fn div(self, rhs: Rhs) -> Self::Output {
                let rhs_value = <Rhs as crate::UnitValue>::into_value(rhs);
                unitopia_open_wrapper_arith_outputs::Quot::from(core::ops::Div::div(self.inner, rhs_value))
            }
        }
    };
}

macro_rules! impl_unit_mul_add_traits {
    ($unit:ident) => {
        impl<LhsValue, Rhs, OutValue> num_traits::MulAdd<Rhs, unitopia_open_wrapper_arith_outputs::Prod<$unit<LhsValue>, Rhs, OutValue>> for $unit<LhsValue>
        where
            Rhs: crate::UnitValue,
            LhsValue: num_traits::MulAdd<Rhs::Value, OutValue, Output = OutValue>,
        {
            type Output = unitopia_open_wrapper_arith_outputs::Prod<$unit<LhsValue>, Rhs, OutValue>;

            fn mul_add(self, a: Rhs, b: unitopia_open_wrapper_arith_outputs::Prod<$unit<LhsValue>, Rhs, OutValue>) -> Self::Output {
                let a_value = <Rhs as crate::UnitValue>::into_value(a);
                let b_value = b.inner;
                unitopia_open_wrapper_arith_outputs::Prod::from(num_traits::MulAdd::mul_add(self.inner, a_value, b_value))
            }
        }

        impl<T> num_traits::MulAdd<T, Self> for $unit<T>
        where
            T: num_traits::MulAdd<T, T, Output = T>,
        {
            type Output = Self;

            fn mul_add(self, a: T, b: Self) -> Self::Output {
                Self {
                    inner: num_traits::MulAdd::mul_add(self.inner, a, b.inner),
                }
            }
        }

        impl<T> num_traits::MulAddAssign<T, Self> for $unit<T>
        where
            T: num_traits::MulAddAssign<T, T>,
        {
            fn mul_add_assign(&mut self, a: T, b: Self) {
                num_traits::MulAddAssign::mul_add_assign(&mut self.inner, a, b.inner);
            }
        }
    };
}

macro_rules! impl_unit_pow_traits {
    ($unit:ident) => {
        impl<T, const POWER: u32> num_traits::Pow<unitopia_measure::Exponent<POWER>> for $unit<T>
        where
            T: num_traits::Pow<u32, Output = T>,
        {
            type Output = unitopia_open_wrapper_arith_outputs::Powr<$unit<T>, unitopia_measure::Exponent<POWER>, T>;

            fn pow(self, _rhs: unitopia_measure::Exponent<POWER>) -> Self::Output {
                unitopia_open_wrapper_arith_outputs::Powr::from(<T as num_traits::Pow<u32>>::pow(self.inner, POWER))
            }
        }
    };
}

macro_rules! impl_strict_wrapper_unit_ops {
    ($unit:ident) => {
        impl_unit_identity_traits!($unit);
        impl_unit_add_sub_traits!($unit);
        impl_unit_scalar_mul_div_traits!($unit);
        impl_unit_mul_div_measure_traits!($unit);
        impl_unit_mul_add_traits!($unit);
        impl_unit_pow_traits!($unit);
    };
}

define_strict_wrapper_unit!(Ampere, unitopia_marker_units::Ampere);
impl_strict_wrapper_unit_ops!(Ampere);
define_strict_wrapper_unit!(Candela, unitopia_marker_units::Candela);
impl_strict_wrapper_unit_ops!(Candela);
define_strict_wrapper_unit!(GalactosidaseActivityUnit, unitopia_marker_units::GalactosidaseActivityUnit);
impl_strict_wrapper_unit_ops!(GalactosidaseActivityUnit);
define_strict_wrapper_unit!(Kelvin, unitopia_marker_units::Kelvin);
impl_strict_wrapper_unit_ops!(Kelvin);
define_strict_wrapper_unit!(Kilogram, unitopia_marker_units::Kilogram);
impl_strict_wrapper_unit_ops!(Kilogram);
define_strict_wrapper_unit!(Meter, unitopia_marker_units::Meter);
impl_strict_wrapper_unit_ops!(Meter);
define_strict_wrapper_unit!(Mole, unitopia_marker_units::Mole);
impl_strict_wrapper_unit_ops!(Mole);
define_strict_wrapper_unit!(PowerOfHydrogen, unitopia_marker_units::PowerOfHydrogen);
impl_strict_wrapper_unit_ops!(PowerOfHydrogen);
define_strict_wrapper_unit!(Second, unitopia_marker_units::Second);
impl_strict_wrapper_unit_ops!(Second);

pub type Area<Value> = unitopia_open_wrapper_arith_outputs::Prod<Meter<Value>, Meter<Value>, Value>;
pub type Newton<Value> = unitopia_open_wrapper_arith_outputs::Quot<unitopia_open_wrapper_arith_outputs::Prod<Kilogram<Value>, Meter<Value>, Value>, unitopia_open_wrapper_arith_outputs::Prod<Second<Value>, Second<Value>, Value>, Value>;
pub type Volt<Value> = unitopia_open_wrapper_arith_outputs::Quot<unitopia_open_wrapper_arith_outputs::Prod<Newton<Value>, Meter<Value>, Value>, Ampere<Value>, Value>;

pub use unitopia_helpers::Scalar;
