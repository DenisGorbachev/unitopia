crate::def_quantity!(VolumeQuantity, Volume);

pub mod f32 {
    #![allow(dead_code)]

    use crate::scales::f32::*;

    pub type Volume = crate::Volume<f32>;

    pub const LITER: Volume = Volume::new_const(ONE);

    pub const MILLILITER: Volume = Volume::new_const(MILLI);
}
