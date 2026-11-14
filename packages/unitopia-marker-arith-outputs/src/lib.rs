#![deny(clippy::arithmetic_side_effects)]

use core::marker::PhantomData;

macro_rules! define_marker_arith_output {
    ($name:ident<$lhs:ident, $rhs:ident>) => {
        #[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
        pub struct $name<$lhs, $rhs>(PhantomData<($lhs, $rhs)>);
    };
}

define_marker_arith_output!(Prod<A, B>);
define_marker_arith_output!(Quot<A, B>);
define_marker_arith_output!(Powr<A, N>);
