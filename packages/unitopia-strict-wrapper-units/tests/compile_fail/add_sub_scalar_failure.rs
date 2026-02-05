use unitopia_strict_wrapper_units::{Meter, StrictWrapperUnit};

fn main() {
    let value = Meter::from_value(1u32);
    let _ = value + 1u32;
}
