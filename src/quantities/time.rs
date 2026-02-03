crate::def_quantity!(TimeQuantity, Time);

pub mod f32 {
    #![allow(dead_code)]

    use crate::scales::f32::*;

    pub type Time = crate::Time<f32>;

    pub const SECOND: Time = Time::new_const(ONE);
}
