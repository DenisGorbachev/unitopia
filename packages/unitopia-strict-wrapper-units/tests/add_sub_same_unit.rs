use unitopia_strict_wrapper_units::Meter;

#[test]
fn add_sub_same_unit() {
    let lhs = Meter::new(10u32);
    let rhs = Meter::new(4u32);
    let sum = lhs + rhs;
    assert_eq!(sum.inner, 14);

    let diff = sum - Meter::new(6u32);
    assert_eq!(diff.inner, 8);
}
