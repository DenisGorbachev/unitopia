use unitopia_strict_wrapper_units::{Meter, StrictWrapperUnit};

#[test]
fn mul_div_scalar() {
    let mut value = Meter::from_value(6u32);
    value *= 3u32;
    assert_eq!(*value.value_ref(), 18);

    value /= 2u32;
    assert_eq!(*value.value_ref(), 9);
}
