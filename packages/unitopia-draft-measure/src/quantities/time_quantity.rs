crate::def_quantity!(TimeQuantity, Time);

pub mod time_f32 {
    #![allow(dead_code)]

    use crate::Time as QuantityTime;
    use crate::scales::f32::*;

    pub type Time = QuantityTime<f32>;

    pub const SECOND: Time = Time::new_const(ONE);
}
