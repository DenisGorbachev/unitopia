use unitopia_marker_arith_outputs::{Prod, Quot};

unitopia_helpers::define_marker_struct!(Ampere);
unitopia_helpers::define_marker_struct!(Candela);
unitopia_helpers::define_marker_struct!(GalactosidaseActivityUnit);
unitopia_helpers::define_marker_struct!(Kelvin);
unitopia_helpers::define_marker_struct!(Kilogram);
unitopia_helpers::define_marker_struct!(Meter);
unitopia_helpers::define_marker_struct!(Mole);
unitopia_helpers::define_marker_struct!(PowerOfHydrogen);
unitopia_helpers::define_marker_struct!(Second);

pub type Area = Prod<Meter, Meter>;
pub type Newton = Quot<Prod<Kilogram, Meter>, Prod<Second, Second>>;
pub type Volt = Quot<Prod<Newton, Meter>, Ampere>;
