#[macro_export]
macro_rules! def_quantity {
    ($quantity:ident, $measure:ident) => {
        #[derive(Eq, PartialEq, Ord, PartialOrd, Default, Hash, Clone, Debug)]
        pub struct $quantity;

        pub type $measure<Value> = $crate::Measure<$quantity, Value>;
    };
}
