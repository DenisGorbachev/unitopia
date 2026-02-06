use unitopia_strict_wrapper_units::Meter;

#[test]
fn mul_div_same_unit() {
    let lhs = Meter::new(6u32);
    let rhs = Meter::new(2u32);
    let product = lhs * rhs;
    assert_eq!(product.inner, 12);

    let quotient = Meter::new(6u32) / Meter::new(2u32);
    assert_eq!(quotient.inner, 3);
}
