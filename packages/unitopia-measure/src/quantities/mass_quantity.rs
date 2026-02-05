crate::def_quantity!(MassQuantity, Mass);

pub mod mass_f32 {
    #![allow(dead_code)]

    use crate::scales::f32::*;

    pub type Mass = crate::Mass<f32>;

    pub const MICROGRAM: Mass = Mass::new_const(MICRO);
    pub const MILLIGRAM: Mass = Mass::new_const(MILLI);
    pub const POUND: Mass = Mass::new_const(0.45359237);
}
