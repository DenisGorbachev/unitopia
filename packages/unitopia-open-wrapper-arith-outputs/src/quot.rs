use core::marker::PhantomData;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Quot<A, B, S>(PhantomData<(A, B, S)>);
