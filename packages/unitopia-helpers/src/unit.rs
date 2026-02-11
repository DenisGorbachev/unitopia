use crate::{HasDimension, HasScale};

pub trait Unit: HasScale + HasDimension {}
