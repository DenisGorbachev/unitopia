use std::marker::PhantomData;
use typenum::{Integer, N1000, P1000, U1, U60, Unsigned, Z0};
use unitopia_marker_units::Second;

/// `Pow` must be an [`Integer`] because it must be able to represent fractional units (which have negative powers)
#[derive(Default, Clone, Debug)]
pub struct Scale<Unit, Num: Unsigned, Den: Unsigned, Pow: Integer>(PhantomData<(Unit, Num, Den, Pow)>);

pub type Minute = Scale<Second, U60, U60, Z0>;
pub type Millisecond = Scale<Second, U1, U1, N1000>;

pub type Kilo<Unit> = Scale<Unit, U1, U1, P1000>;
