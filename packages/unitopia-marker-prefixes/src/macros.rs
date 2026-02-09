macro_rules! define_marker_prefixes {
    ($($name:ident),+ $(,)?) => {
        unitopia_helpers::define_marker_structs!($($name),+);
    };
}
