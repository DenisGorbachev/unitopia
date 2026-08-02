macro_rules! define_try_quantity_value_arithmetic {
    ($method:ident, $checked_trait:ident, $checked_method:ident, $error:ident, $units_error:literal, $arithmetic_error:literal $(,)?) => {
        impl<Value, Unit> $crate::QuantityValue<Value, Unit>
        where
            Value: num_traits::$checked_trait,
            Unit: PartialEq,
        {
            /// Combines two quantity values after validating that their runtime units match.
            pub fn $method(self, rhs: Self) -> Result<Self, $error<Value, Unit>> {
                use $error::*;
                errgonomic::handle_bool!(self.unit != rhs.unit, UnitsDifferInvalid, lhs: self, rhs);
                let value = errgonomic::handle_opt!(
                    <Value as num_traits::$checked_trait>::$checked_method(&self.value, &rhs.value),
                    ValueArithmeticFailed,
                    lhs: self,
                    rhs
                );
                Ok(Self::new(value, self.unit))
            }
        }

        /// An error returned by the corresponding checked quantity-value arithmetic operation.
        #[derive(thiserror::Error, Debug)]
        pub enum $error<Value, Unit> {
            /// The operands use different runtime units.
            #[error($units_error)]
            UnitsDifferInvalid {
                /// The left-hand operand.
                lhs: $crate::QuantityValue<Value, Unit>,
                /// The right-hand operand.
                rhs: $crate::QuantityValue<Value, Unit>,
            },
            /// The underlying value arithmetic failed.
            #[error($arithmetic_error)]
            ValueArithmeticFailed {
                /// The left-hand operand.
                lhs: $crate::QuantityValue<Value, Unit>,
                /// The right-hand operand.
                rhs: $crate::QuantityValue<Value, Unit>,
            },
        }
    };
}
