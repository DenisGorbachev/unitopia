use unitopia_strict_wrapper_units::{Meter, Second, StrictWrapperUnit};

#[test]
fn mul_div_different_unit() {
    let length = Meter::from_value(8u32);
    let time = Second::from_value(2u32);
    let product = length * time;
    assert_eq!(product.inner, 16);

    let quotient = Meter::from_value(9u32) / Second::from_value(3u32);
    assert_eq!(quotient.inner, 3);
}
