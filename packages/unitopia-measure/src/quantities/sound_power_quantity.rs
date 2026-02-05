crate::def_quantity!(SoundPowerQuantity, SoundPower);

pub mod sound_power_f32 {
    #![allow(dead_code)]

    use crate::scales::f32::*;

    pub type SoundPower = crate::SoundPower<f32>;

    pub const BEL: SoundPower = SoundPower::new_const(ONE);
    pub const DECIBEL: SoundPower = SoundPower::new_const(DECI);
}
