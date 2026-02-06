use unitopia_strict_wrapper_units::{Meter, Second};

fn main() {
    let lhs = Meter::new(1u32);
    let rhs = Second::new(1u32);
    let _ = lhs + rhs;
}
