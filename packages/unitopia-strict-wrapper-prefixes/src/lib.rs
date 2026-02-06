macro_rules! define_strict_wrapper_prefix {
    ($name:ident) => {
        unitopia_helpers::define_strict_wrapper_struct!(
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
            #[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
            $name
        );
        impl<T> unitopia_strict_wrapper_units::UnitValue for $name<T>
        where
            T: unitopia_strict_wrapper_units::UnitValue,
        {
            type Value = <T as unitopia_strict_wrapper_units::UnitValue>::Value;

            fn from_value(value: Self::Value) -> Self {
                Self {
                    inner: <T as unitopia_strict_wrapper_units::UnitValue>::from_value(value),
                }
            }

            fn value_ref(&self) -> &Self::Value {
                <T as unitopia_strict_wrapper_units::UnitValue>::value_ref(&self.inner)
            }

            fn value_mut(&mut self) -> &mut Self::Value {
                <T as unitopia_strict_wrapper_units::UnitValue>::value_mut(&mut self.inner)
            }

            fn into_value(self) -> Self::Value {
                <T as unitopia_strict_wrapper_units::UnitValue>::into_value(self.inner)
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
        impl_strict_wrapper_prefix_ops!($name);
    };
}

macro_rules! impl_prefix_identity_traits {
    ($prefix:ident) => {
        impl<T> num_traits::ConstZero for $prefix<T>
        where
            T: num_traits::ConstZero,
        {
            const ZERO: Self = Self {
                inner: T::ZERO,
            };
        }

        impl<T> num_traits::Zero for $prefix<T>
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

macro_rules! impl_prefix_add_sub_traits {
    ($prefix:ident) => {
        impl<T> core::ops::Add for $prefix<T>
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

        impl<T> core::ops::Sub for $prefix<T>
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

        impl<T> core::ops::AddAssign for $prefix<T>
        where
            T: core::ops::AddAssign<T>,
        {
            fn add_assign(&mut self, rhs: Self) {
                core::ops::AddAssign::add_assign(&mut self.inner, rhs.inner);
            }
        }

        impl<T> core::ops::SubAssign for $prefix<T>
        where
            T: core::ops::SubAssign<T>,
        {
            fn sub_assign(&mut self, rhs: Self) {
                core::ops::SubAssign::sub_assign(&mut self.inner, rhs.inner);
            }
        }

        impl<T> num_traits::CheckedAdd for $prefix<T>
        where
            T: num_traits::CheckedAdd,
        {
            fn checked_add(&self, rhs: &Self) -> Option<Self> {
                num_traits::CheckedAdd::checked_add(&self.inner, &rhs.inner).map(|value| Self {
                    inner: value,
                })
            }
        }

        impl<T> num_traits::CheckedSub for $prefix<T>
        where
            T: num_traits::CheckedSub,
        {
            fn checked_sub(&self, rhs: &Self) -> Option<Self> {
                num_traits::CheckedSub::checked_sub(&self.inner, &rhs.inner).map(|value| Self {
                    inner: value,
                })
            }
        }

        impl<T> num_traits::SaturatingAdd for $prefix<T>
        where
            T: num_traits::SaturatingAdd,
        {
            fn saturating_add(&self, rhs: &Self) -> Self {
                Self {
                    inner: num_traits::SaturatingAdd::saturating_add(&self.inner, &rhs.inner),
                }
            }
        }

        impl<T> num_traits::SaturatingSub for $prefix<T>
        where
            T: num_traits::SaturatingSub,
        {
            fn saturating_sub(&self, rhs: &Self) -> Self {
                Self {
                    inner: num_traits::SaturatingSub::saturating_sub(&self.inner, &rhs.inner),
                }
            }
        }

        impl<T> num_traits::WrappingAdd for $prefix<T>
        where
            T: num_traits::WrappingAdd,
        {
            fn wrapping_add(&self, rhs: &Self) -> Self {
                Self {
                    inner: num_traits::WrappingAdd::wrapping_add(&self.inner, &rhs.inner),
                }
            }
        }

        impl<T> num_traits::WrappingSub for $prefix<T>
        where
            T: num_traits::WrappingSub,
        {
            fn wrapping_sub(&self, rhs: &Self) -> Self {
                Self {
                    inner: num_traits::WrappingSub::wrapping_sub(&self.inner, &rhs.inner),
                }
            }
        }

        impl<T> num_traits::ops::overflowing::OverflowingAdd for $prefix<T>
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

        impl<T> num_traits::ops::overflowing::OverflowingSub for $prefix<T>
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

macro_rules! impl_prefix_scalar_mul_div_traits {
    ($prefix:ident) => {
        impl<T, Scalar> core::ops::MulAssign<Scalar> for $prefix<T>
        where
            T: core::ops::MulAssign<Scalar>,
        {
            fn mul_assign(&mut self, rhs: Scalar) {
                core::ops::MulAssign::mul_assign(&mut self.inner, rhs);
            }
        }

        impl<T, Scalar> core::ops::DivAssign<Scalar> for $prefix<T>
        where
            T: core::ops::DivAssign<Scalar>,
        {
            fn div_assign(&mut self, rhs: Scalar) {
                core::ops::DivAssign::div_assign(&mut self.inner, rhs);
            }
        }
    };
}

macro_rules! impl_prefix_mul_div_unit_traits {
    ($prefix:ident) => {
        impl<T, Rhs> core::ops::Mul<Rhs> for $prefix<T>
        where
            Rhs: unitopia_strict_wrapper_units::UnitValue,
            T: core::ops::Mul<Rhs>,
        {
            type Output = <T as core::ops::Mul<Rhs>>::Output;

            fn mul(self, rhs: Rhs) -> Self::Output {
                core::ops::Mul::mul(self.inner, rhs)
            }
        }

        impl<T, Rhs> core::ops::Div<Rhs> for $prefix<T>
        where
            Rhs: unitopia_strict_wrapper_units::UnitValue,
            T: core::ops::Div<Rhs>,
        {
            type Output = <T as core::ops::Div<Rhs>>::Output;

            fn div(self, rhs: Rhs) -> Self::Output {
                core::ops::Div::div(self.inner, rhs)
            }
        }
    };
}

macro_rules! impl_prefix_mul_add_traits {
    ($prefix:ident) => {
        impl<T, Rhs, OutValue> num_traits::MulAdd<Rhs, unitopia_open_wrapper_arith_outputs::Prod<T, Rhs, OutValue>> for $prefix<T>
        where
            T: unitopia_strict_wrapper_units::UnitValue,
            T: num_traits::MulAdd<Rhs, unitopia_open_wrapper_arith_outputs::Prod<T, Rhs, OutValue>, Output = unitopia_open_wrapper_arith_outputs::Prod<T, Rhs, OutValue>>,
        {
            type Output = unitopia_open_wrapper_arith_outputs::Prod<T, Rhs, OutValue>;

            fn mul_add(self, a: Rhs, b: unitopia_open_wrapper_arith_outputs::Prod<T, Rhs, OutValue>) -> Self::Output {
                num_traits::MulAdd::mul_add(self.inner, a, b)
            }
        }

        impl<T, Scalar> num_traits::MulAdd<Scalar, Self> for $prefix<T>
        where
            T: unitopia_strict_wrapper_units::UnitValue,
            T: num_traits::MulAdd<Scalar, T, Output = T>,
        {
            type Output = Self;

            fn mul_add(self, a: Scalar, b: Self) -> Self::Output {
                Self {
                    inner: num_traits::MulAdd::mul_add(self.inner, a, b.inner),
                }
            }
        }

        impl<T, Scalar> num_traits::MulAddAssign<Scalar, Self> for $prefix<T>
        where
            T: unitopia_strict_wrapper_units::UnitValue,
            T: num_traits::MulAddAssign<Scalar, T>,
        {
            fn mul_add_assign(&mut self, a: Scalar, b: Self) {
                num_traits::MulAddAssign::mul_add_assign(&mut self.inner, a, b.inner);
            }
        }
    };
}

macro_rules! impl_prefix_pow_traits {
    ($prefix:ident) => {
        impl<T, const POWER: u32> num_traits::Pow<unitopia_measure::Exponent<POWER>> for $prefix<T>
        where
            T: num_traits::Pow<unitopia_measure::Exponent<POWER>>,
        {
            type Output = <T as num_traits::Pow<unitopia_measure::Exponent<POWER>>>::Output;

            fn pow(self, rhs: unitopia_measure::Exponent<POWER>) -> Self::Output {
                num_traits::Pow::pow(self.inner, rhs)
            }
        }
    };
}

macro_rules! impl_strict_wrapper_prefix_ops {
    ($prefix:ident) => {
        impl_prefix_identity_traits!($prefix);
        impl_prefix_add_sub_traits!($prefix);
        impl_prefix_scalar_mul_div_traits!($prefix);
        impl_prefix_mul_div_unit_traits!($prefix);
        impl_prefix_mul_add_traits!($prefix);
        impl_prefix_pow_traits!($prefix);
    };
}

define_strict_wrapper_prefix!(Atto);
define_strict_wrapper_prefix!(Centi);
define_strict_wrapper_prefix!(Deca);
define_strict_wrapper_prefix!(Deci);
define_strict_wrapper_prefix!(Exa);
define_strict_wrapper_prefix!(Femto);
define_strict_wrapper_prefix!(Giga);
define_strict_wrapper_prefix!(Hecto);
define_strict_wrapper_prefix!(Hexagesi);
define_strict_wrapper_prefix!(Kilo);
define_strict_wrapper_prefix!(Mega);
define_strict_wrapper_prefix!(Micro);
define_strict_wrapper_prefix!(Milli);
define_strict_wrapper_prefix!(Nano);
define_strict_wrapper_prefix!(Peta);
define_strict_wrapper_prefix!(Pico);
define_strict_wrapper_prefix!(Quecto);
define_strict_wrapper_prefix!(Quetta);
define_strict_wrapper_prefix!(Ronna);
define_strict_wrapper_prefix!(Ronto);
define_strict_wrapper_prefix!(Tera);
define_strict_wrapper_prefix!(Tetravigesi);
define_strict_wrapper_prefix!(Yocto);
define_strict_wrapper_prefix!(Yotta);
define_strict_wrapper_prefix!(Zepto);
define_strict_wrapper_prefix!(Zetta);
