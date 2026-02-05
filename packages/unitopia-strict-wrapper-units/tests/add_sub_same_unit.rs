use unitopia_strict_wrapper_units::{Meter, StrictWrapperUnit};

#[test]
fn add_sub_same_unit() {
    let lhs = Meter::from_value(10u32);
    let rhs = Meter::from_value(4u32);
    let sum = lhs + rhs;
    assert_eq!(*sum.value_ref(), 14);

    let diff = sum - Meter::from_value(6u32);
    assert_eq!(*diff.value_ref(), 8);
}
