#[macro_export]
macro_rules! impl_display_for_coefficient {
    ($name:ident) => {
        impl<Unit: std::fmt::Display> std::fmt::Display for $name<Unit> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{name}<{unit}>", name = stringify!($name), unit = self.unit)
            }
        }
    };
}
