use crate::HasDimension;

/// Note that quantity doesn't automatically correspond to a single unit (it can be measured in different units), but the dimensions of a quantity and a unit must match in a single measurement
pub trait Quantity: HasDimension {}
