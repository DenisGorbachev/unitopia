crate::def_quantity!(PowerQuantity, Power);

pub mod power_f32 {
    #![allow(dead_code)]

    use crate::scales::f32::*;

    pub type Power = crate::Power<f32>;

    pub const WATT: Power = Power::new_const(ONE);
    pub const MEGAWATT: Power = Power::new_const(MEGA);
}
