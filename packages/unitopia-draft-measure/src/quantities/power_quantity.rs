crate::def_quantity!(PowerQuantity, Power);

pub mod power_f32 {
    #![allow(dead_code)]

    use crate::Power as QuantityPower;
    use crate::scales::f32::*;

    pub type Power = QuantityPower<f32>;

    pub const WATT: Power = Power::new_const(ONE);
    pub const MEGAWATT: Power = Power::new_const(MEGA);
}
