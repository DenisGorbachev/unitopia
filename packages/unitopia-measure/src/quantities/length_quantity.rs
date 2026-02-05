crate::def_quantity!(LengthQuantity, Length);

pub mod length_f32 {
    #![allow(dead_code)]

    use crate::scales::f32::*;

    pub type Length = crate::Length<f32>;

    pub const MILLIMETER: Length = Length::new_const(MILLI);
    pub const METER: Length = Length::new_const(ONE);
    pub const KILOMETER: Length = Length::new_const(KILO);
}
