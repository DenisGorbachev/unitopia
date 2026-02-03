#[macro_export]
macro_rules! def_struct_coefficient {
    ($name:ident) => {
        #[derive(Eq, PartialEq, Ord, PartialOrd, Default, Hash, Clone, Debug)]
        pub struct $name<Unit> {
            unit: Unit,
        }
    };
}
