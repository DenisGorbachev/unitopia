use unitopia_strict_wrapper_units::{Meter, StrictWrapperUnit};

#[test]
fn mul_div_same_unit() {
    let lhs = Meter::from_value(6u32);
    let rhs = Meter::from_value(2u32);
    let product = lhs * rhs;
    assert_eq!(product.into_value(), 12);

    let quotient = Meter::from_value(6u32) / Meter::from_value(2u32);
    assert_eq!(quotient.into_value(), 3);
}
