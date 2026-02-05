use core::marker::PhantomData;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Prod<A, B>(PhantomData<(A, B)>);
