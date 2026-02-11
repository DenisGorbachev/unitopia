#[macro_export]
macro_rules! define_strict_wrapper_unit {
    ($name:ident) => {
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

        impl_strict_wrapper_unit_ops!($name);
    };
}

macro_rules! impl_strict_wrapper_unit_ops {
    ($unit:ident) => {
        unitopia_helpers::impl_wrapper_identity_traits!($unit);
        unitopia_helpers::impl_wrapper_add_sub_traits!($unit);
        impl_unit_scalar_mul_div_rem_traits!($unit);
        impl_unit_mul_div_measure_traits!($unit);
        impl_unit_mul_add_traits!($unit);
        impl_unit_pow_traits!($unit);
    };
}

macro_rules! impl_unit_scalar_mul_div_rem_traits {
    ($unit:ident) => {
        impl<T> core::ops::Mul<unitopia_helpers::Scalar<T>> for $unit<T>
        where
            T: core::ops::Mul<T, Output = T>,
        {
            type Output = Self;

            fn mul(self, rhs: unitopia_helpers::Scalar<T>) -> Self {
                Self {
                    inner: core::ops::Mul::mul(self.inner, rhs.inner),
                }
            }
        }

        impl<T> core::ops::Div<unitopia_helpers::Scalar<T>> for $unit<T>
        where
            T: core::ops::Div<T, Output = T>,
        {
            type Output = Self;

            fn div(self, rhs: unitopia_helpers::Scalar<T>) -> Self {
                Self {
                    inner: core::ops::Div::div(self.inner, rhs.inner),
                }
            }
        }

        impl<T> core::ops::Rem<T> for $unit<T>
        where
            T: core::ops::Rem<T, Output = T>,
        {
            type Output = Self;

            fn rem(self, rhs: T) -> Self {
                Self {
                    inner: core::ops::Rem::rem(self.inner, rhs),
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
        impl<T, const POWER: u32> num_traits::Pow<unitopia_helpers::Exponent<POWER>> for $unit<T>
        where
            T: num_traits::Pow<u32, Output = T>,
        {
            type Output = unitopia_open_wrapper_arith_outputs::Powr<$unit<T>, unitopia_helpers::Exponent<POWER>, T>;

            fn pow(self, _rhs: unitopia_helpers::Exponent<POWER>) -> Self::Output {
                unitopia_open_wrapper_arith_outputs::Powr::from(<T as num_traits::Pow<u32>>::pow(self.inner, POWER))
            }
        }
    };
}

macro_rules! define_strict_wrapper_units {
    ($($name:ident),+ $(,)?) => {
        $(
            define_strict_wrapper_unit!($name);
        )+
    };
}
