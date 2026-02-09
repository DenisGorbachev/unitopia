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
        impl<T, Scalar> core::ops::Rem<Scalar> for $prefix<T>
        where
            T: core::ops::Rem<Scalar, Output = T>,
        {
            type Output = Self;

            fn rem(self, rhs: Scalar) -> Self::Output {
                Self {
                    inner: core::ops::Rem::rem(self.inner, rhs),
                }
            }
        }

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

macro_rules! impl_prefix_scales {
    ($($name:ident => ($num:expr, $den:expr)),+ $(,)?) => {
        $(
            impl<T> unitopia_helpers::Scale for $name<T> {
                const NUM: u128 = $num;
                const DEN: u128 = $den;
            }
        )+
    };
}

macro_rules! impl_cross_prefix_add_sub_traits_for_pair {
    ($lhs:ident, $rhs:ident) => {
        impl<T> core::ops::Add<$rhs<T>> for $lhs<T>
        where
            T: unitopia_strict_wrapper_units::UnitValue,
            <T as unitopia_strict_wrapper_units::UnitValue>::Value: Copy,
            <T as unitopia_strict_wrapper_units::UnitValue>::Value: core::convert::TryFrom<u128>,
            <T as unitopia_strict_wrapper_units::UnitValue>::Value: core::ops::Add<<T as unitopia_strict_wrapper_units::UnitValue>::Value, Output = <T as unitopia_strict_wrapper_units::UnitValue>::Value>,
            <T as unitopia_strict_wrapper_units::UnitValue>::Value: core::ops::Mul<<T as unitopia_strict_wrapper_units::UnitValue>::Value, Output = <T as unitopia_strict_wrapper_units::UnitValue>::Value>,
            <T as unitopia_strict_wrapper_units::UnitValue>::Value: core::ops::Div<<T as unitopia_strict_wrapper_units::UnitValue>::Value, Output = <T as unitopia_strict_wrapper_units::UnitValue>::Value>,
            $rhs<T>: unitopia_strict_wrapper_units::UnitValue<Value = <T as unitopia_strict_wrapper_units::UnitValue>::Value>,
            $lhs<T>: unitopia_helpers::Scale,
            $rhs<T>: unitopia_helpers::Scale,
        {
            type Output = $rhs<T>;

            fn add(self, rhs: $rhs<T>) -> Self::Output {
                let lhs_value = <T as unitopia_strict_wrapper_units::UnitValue>::into_value(self.inner);
                let rhs_value = <T as unitopia_strict_wrapper_units::UnitValue>::into_value(rhs.inner);
                let lhs_num = <$lhs<T> as unitopia_helpers::Scale>::NUM;
                let lhs_den = <$lhs<T> as unitopia_helpers::Scale>::DEN;
                let rhs_num = <$rhs<T> as unitopia_helpers::Scale>::NUM;
                let rhs_den = <$rhs<T> as unitopia_helpers::Scale>::DEN;

                let to_value = |scale: u128| -> <T as unitopia_strict_wrapper_units::UnitValue>::Value {
                    match <T as unitopia_strict_wrapper_units::UnitValue>::Value::try_from(scale) {
                        Ok(value) => value,
                        Err(_) => panic!("prefix scale conversion to storage type failed"),
                    }
                };
                let lhs_common = core::ops::Mul::mul(core::ops::Mul::mul(lhs_value, to_value(lhs_num)), to_value(rhs_den));
                let rhs_common = core::ops::Mul::mul(core::ops::Mul::mul(rhs_value, to_value(rhs_num)), to_value(lhs_den));
                let sum_common = core::ops::Add::add(lhs_common, rhs_common);
                let out_den = match lhs_den.checked_mul(rhs_num) {
                    Some(value) => value,
                    None => panic!("prefix denominator overflow"),
                };
                let out_value = core::ops::Div::div(sum_common, to_value(out_den));
                <Self::Output as unitopia_strict_wrapper_units::UnitValue>::from_value(out_value)
            }
        }

        impl<T> core::ops::Sub<$rhs<T>> for $lhs<T>
        where
            T: unitopia_strict_wrapper_units::UnitValue,
            <T as unitopia_strict_wrapper_units::UnitValue>::Value: Copy,
            <T as unitopia_strict_wrapper_units::UnitValue>::Value: core::convert::TryFrom<u128>,
            <T as unitopia_strict_wrapper_units::UnitValue>::Value: core::ops::Sub<<T as unitopia_strict_wrapper_units::UnitValue>::Value, Output = <T as unitopia_strict_wrapper_units::UnitValue>::Value>,
            <T as unitopia_strict_wrapper_units::UnitValue>::Value: core::ops::Mul<<T as unitopia_strict_wrapper_units::UnitValue>::Value, Output = <T as unitopia_strict_wrapper_units::UnitValue>::Value>,
            <T as unitopia_strict_wrapper_units::UnitValue>::Value: core::ops::Div<<T as unitopia_strict_wrapper_units::UnitValue>::Value, Output = <T as unitopia_strict_wrapper_units::UnitValue>::Value>,
            $rhs<T>: unitopia_strict_wrapper_units::UnitValue<Value = <T as unitopia_strict_wrapper_units::UnitValue>::Value>,
            $lhs<T>: unitopia_helpers::Scale,
            $rhs<T>: unitopia_helpers::Scale,
        {
            type Output = $rhs<T>;

            fn sub(self, rhs: $rhs<T>) -> Self::Output {
                let lhs_value = <T as unitopia_strict_wrapper_units::UnitValue>::into_value(self.inner);
                let rhs_value = <T as unitopia_strict_wrapper_units::UnitValue>::into_value(rhs.inner);
                let lhs_num = <$lhs<T> as unitopia_helpers::Scale>::NUM;
                let lhs_den = <$lhs<T> as unitopia_helpers::Scale>::DEN;
                let rhs_num = <$rhs<T> as unitopia_helpers::Scale>::NUM;
                let rhs_den = <$rhs<T> as unitopia_helpers::Scale>::DEN;

                let to_value = |scale: u128| -> <T as unitopia_strict_wrapper_units::UnitValue>::Value {
                    match <T as unitopia_strict_wrapper_units::UnitValue>::Value::try_from(scale) {
                        Ok(value) => value,
                        Err(_) => panic!("prefix scale conversion to storage type failed"),
                    }
                };
                let lhs_common = core::ops::Mul::mul(core::ops::Mul::mul(lhs_value, to_value(lhs_num)), to_value(rhs_den));
                let rhs_common = core::ops::Mul::mul(core::ops::Mul::mul(rhs_value, to_value(rhs_num)), to_value(lhs_den));
                let diff_common = core::ops::Sub::sub(lhs_common, rhs_common);
                let out_den = match lhs_den.checked_mul(rhs_num) {
                    Some(value) => value,
                    None => panic!("prefix denominator overflow"),
                };
                let out_value = core::ops::Div::div(diff_common, to_value(out_den));
                <Self::Output as unitopia_strict_wrapper_units::UnitValue>::from_value(out_value)
            }
        }

        impl<T> core::cmp::PartialEq<$rhs<T>> for $lhs<T>
        where
            T: unitopia_strict_wrapper_units::UnitValue,
            <T as unitopia_strict_wrapper_units::UnitValue>::Value: Copy,
            <T as unitopia_strict_wrapper_units::UnitValue>::Value: core::convert::TryFrom<u128>,
            <T as unitopia_strict_wrapper_units::UnitValue>::Value: core::ops::Mul<<T as unitopia_strict_wrapper_units::UnitValue>::Value, Output = <T as unitopia_strict_wrapper_units::UnitValue>::Value>,
            <T as unitopia_strict_wrapper_units::UnitValue>::Value: core::cmp::PartialEq,
            $lhs<T>: unitopia_helpers::Scale,
            $rhs<T>: unitopia_helpers::Scale,
        {
            fn eq(&self, rhs: &$rhs<T>) -> bool {
                let lhs_value = *<T as unitopia_strict_wrapper_units::UnitValue>::value_ref(&self.inner);
                let rhs_value = *<T as unitopia_strict_wrapper_units::UnitValue>::value_ref(&rhs.inner);
                let lhs_num = <$lhs<T> as unitopia_helpers::Scale>::NUM;
                let lhs_den = <$lhs<T> as unitopia_helpers::Scale>::DEN;
                let rhs_num = <$rhs<T> as unitopia_helpers::Scale>::NUM;
                let rhs_den = <$rhs<T> as unitopia_helpers::Scale>::DEN;
                let to_value = |scale: u128| -> <T as unitopia_strict_wrapper_units::UnitValue>::Value {
                    match <T as unitopia_strict_wrapper_units::UnitValue>::Value::try_from(scale) {
                        Ok(value) => value,
                        Err(_) => panic!("prefix scale conversion to storage type failed"),
                    }
                };
                let lhs_common = core::ops::Mul::mul(core::ops::Mul::mul(lhs_value, to_value(lhs_num)), to_value(rhs_den));
                let rhs_common = core::ops::Mul::mul(core::ops::Mul::mul(rhs_value, to_value(rhs_num)), to_value(lhs_den));
                lhs_common == rhs_common
            }
        }

        impl<T> core::cmp::PartialOrd<$rhs<T>> for $lhs<T>
        where
            T: unitopia_strict_wrapper_units::UnitValue,
            <T as unitopia_strict_wrapper_units::UnitValue>::Value: Copy,
            <T as unitopia_strict_wrapper_units::UnitValue>::Value: core::convert::TryFrom<u128>,
            <T as unitopia_strict_wrapper_units::UnitValue>::Value: core::ops::Mul<<T as unitopia_strict_wrapper_units::UnitValue>::Value, Output = <T as unitopia_strict_wrapper_units::UnitValue>::Value>,
            <T as unitopia_strict_wrapper_units::UnitValue>::Value: core::cmp::PartialOrd,
            $lhs<T>: unitopia_helpers::Scale,
            $rhs<T>: unitopia_helpers::Scale,
        {
            fn partial_cmp(&self, rhs: &$rhs<T>) -> Option<core::cmp::Ordering> {
                let lhs_value = *<T as unitopia_strict_wrapper_units::UnitValue>::value_ref(&self.inner);
                let rhs_value = *<T as unitopia_strict_wrapper_units::UnitValue>::value_ref(&rhs.inner);
                let lhs_num = <$lhs<T> as unitopia_helpers::Scale>::NUM;
                let lhs_den = <$lhs<T> as unitopia_helpers::Scale>::DEN;
                let rhs_num = <$rhs<T> as unitopia_helpers::Scale>::NUM;
                let rhs_den = <$rhs<T> as unitopia_helpers::Scale>::DEN;
                let to_value = |scale: u128| -> <T as unitopia_strict_wrapper_units::UnitValue>::Value {
                    match <T as unitopia_strict_wrapper_units::UnitValue>::Value::try_from(scale) {
                        Ok(value) => value,
                        Err(_) => panic!("prefix scale conversion to storage type failed"),
                    }
                };
                let lhs_common = core::ops::Mul::mul(core::ops::Mul::mul(lhs_value, to_value(lhs_num)), to_value(rhs_den));
                let rhs_common = core::ops::Mul::mul(core::ops::Mul::mul(rhs_value, to_value(rhs_num)), to_value(lhs_den));
                core::cmp::PartialOrd::partial_cmp(&lhs_common, &rhs_common)
            }
        }
    };
}

macro_rules! impl_cross_prefix_add_sub_traits {
    ($head:ident, $($tail:ident),+ $(,)?) => {
        $(
            impl_cross_prefix_add_sub_traits_for_pair!($head, $tail);
            impl_cross_prefix_add_sub_traits_for_pair!($tail, $head);
        )+
        impl_cross_prefix_add_sub_traits!($($tail),+);
    };
    ($single:ident $(,)?) => {};
}
