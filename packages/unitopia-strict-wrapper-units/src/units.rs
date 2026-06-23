use unitopia_open_wrapper_arith_outputs::{Prod, Quot};

define_strict_wrapper_units!(Ampere, Candela, GalactosidaseActivityUnit, Kelvin, Kilogram, Meter, Mole, PowerOfHydrogen, Second);

pub type Area<Value> = Prod<Meter<Value>, Meter<Value>, Value>;
pub type Newton<Value> = Quot<Prod<Kilogram<Value>, Meter<Value>, Value>, Prod<Second<Value>, Second<Value>, Value>, Value>;
pub type Volt<Value> = Quot<Prod<Newton<Value>, Meter<Value>, Value>, Ampere<Value>, Value>;
