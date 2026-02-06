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
        unitopia_helpers::impl_wincode_schema_through_inner!($name);
        impl_strict_wrapper_prefix_ops!($name);
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
        unitopia_helpers::impl_wrapper_identity_traits!($prefix);
        unitopia_helpers::impl_wrapper_add_sub_traits!($prefix);
        impl_prefix_scalar_mul_div_traits!($prefix);
        impl_prefix_mul_div_unit_traits!($prefix);
        impl_prefix_mul_add_traits!($prefix);
        impl_prefix_pow_traits!($prefix);
    };
}

macro_rules! define_strict_wrapper_prefixes {
    ($($name:ident),+ $(,)?) => {
        $(
            define_strict_wrapper_prefix!($name);
        )+
    };
}

define_strict_wrapper_prefixes!(Atto, Centi, Deca, Deci, Exa, Femto, Giga, Hecto, Hexagesi, Kilo, Mega, Micro, Milli, Nano, Peta, Pico, Quecto, Quetta, Ronna, Ronto, Tera, Tetravigesi, Yocto, Yotta, Zepto, Zetta,);
