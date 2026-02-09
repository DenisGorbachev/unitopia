use unitopia_marker_arith_outputs::{Prod, Quot};

define_marker_units!(Ampere, Candela, GalactosidaseActivityUnit, Kelvin, Kilogram, Meter, Mole, PowerOfHydrogen, Second,);

pub type Area = Prod<Meter, Meter>;
pub type Newton = Quot<Prod<Kilogram, Meter>, Prod<Second, Second>>;
pub type Volt = Quot<Prod<Newton, Meter>, Ampere>;
