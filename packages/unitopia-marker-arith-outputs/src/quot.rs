use core::marker::PhantomData;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Quot<A, B>(PhantomData<(A, B)>);
