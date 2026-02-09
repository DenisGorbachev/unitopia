define_strict_wrapper_prefixes!(Quecto, Ronto, Yocto, Zepto, Atto, Femto, Pico, Nano, Micro, Milli, Centi, Deci, Uno, Deca, Tetravigesi, Hexagesi, Hecto, Kilo, Mega, Giga, Tera, Peta, Exa, Zetta, Yotta, Ronna, Quetta,);

// SI
impl_prefix_scales!(
    Quecto => (1, 1_000_000_000_000_000_000_000_000_000_000),
    Ronto => (1, 1_000_000_000_000_000_000_000_000_000),
    Yocto => (1, 1_000_000_000_000_000_000_000_000),
    Zepto => (1, 1_000_000_000_000_000_000_000),
    Atto => (1, 1_000_000_000_000_000_000),
    Femto => (1, 1_000_000_000_000_000),
    Pico => (1, 1_000_000_000_000),
    Nano => (1, 1_000_000_000),
    Micro => (1, 1_000_000),
    Milli => (1, 1_000),
    Centi => (1, 100),
    Deci => (1, 10),
    Uno => (1, 1),
    Deca => (10, 1),
    Hecto => (100, 1),
    Kilo => (1_000, 1),
    Mega => (1_000_000, 1),
    Giga => (1_000_000_000, 1),
    Tera => (1_000_000_000_000, 1),
    Peta => (1_000_000_000_000_000, 1),
    Exa => (1_000_000_000_000_000_000, 1),
    Zetta => (1_000_000_000_000_000_000_000, 1),
    Yotta => (1_000_000_000_000_000_000_000_000, 1),
    Ronna => (1_000_000_000_000_000_000_000_000_000, 1),
    Quetta => (1_000_000_000_000_000_000_000_000_000_000, 1),
);

// Non-SI
impl_prefix_scales!(
    Tetravigesi => (24, 1),
    Hexagesi => (60, 1),
);

impl_cross_prefix_add_sub_traits!(Quecto, Ronto, Yocto, Zepto, Atto, Femto, Pico, Nano, Micro, Milli, Centi, Deci, Uno, Deca, Tetravigesi, Hexagesi, Hecto, Kilo, Mega, Giga, Tera, Peta, Exa, Zetta, Yotta, Ronna, Quetta,);
