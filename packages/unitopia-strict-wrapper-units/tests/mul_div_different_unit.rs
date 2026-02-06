use unitopia_strict_wrapper_units::{Meter, Second};

#[test]
fn mul_div_different_unit() {
    let length = Meter::new(8u32);
    let time = Second::new(2u32);
    let product = length * time;
    assert_eq!(product.inner, 16);

    let quotient = Meter::new(9u32) / Second::new(3u32);
    assert_eq!(quotient.inner, 3);
}
