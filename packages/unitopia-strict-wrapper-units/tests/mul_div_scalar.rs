use unitopia_strict_wrapper_units::Meter;

#[test]
fn mul_div_scalar() {
    let mut value = Meter::new(6u32);
    value *= 3u32;
    assert_eq!(value.inner, 18);

    value /= 2u32;
    assert_eq!(value.inner, 9);
}
