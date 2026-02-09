macro_rules! define_marker_units {
    ($($name:ident),+ $(,)?) => {
        unitopia_helpers::define_marker_structs!($($name),+);
    };
}
