use unitopia_strict_wrapper_units::{Meter, Second, StrictWrapperUnit};

fn main() {
    let lhs = Meter::from_value(1u32);
    let rhs = Second::from_value(1u32);
    let _ = lhs + rhs;
}
