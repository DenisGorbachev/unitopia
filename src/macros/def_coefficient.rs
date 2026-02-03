#[macro_export]
macro_rules! def_coefficient {
    ($name:ident) => {
        $crate::def_struct_coefficient!($name);
        $crate::impl_display_for_coefficient!($name);
    };
}
