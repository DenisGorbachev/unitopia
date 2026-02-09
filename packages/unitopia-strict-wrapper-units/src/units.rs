define_strict_wrapper_units!(Ampere, Candela, GalactosidaseActivityUnit, Kelvin, Kilogram, Meter, Mole, PowerOfHydrogen, Second);

pub type Area<Value> = unitopia_open_wrapper_arith_outputs::Prod<Meter<Value>, Meter<Value>, Value>;
pub type Newton<Value> = unitopia_open_wrapper_arith_outputs::Quot<unitopia_open_wrapper_arith_outputs::Prod<Kilogram<Value>, Meter<Value>, Value>, unitopia_open_wrapper_arith_outputs::Prod<Second<Value>, Second<Value>, Value>, Value>;
pub type Volt<Value> = unitopia_open_wrapper_arith_outputs::Quot<unitopia_open_wrapper_arith_outputs::Prod<Newton<Value>, Meter<Value>, Value>, Ampere<Value>, Value>;
