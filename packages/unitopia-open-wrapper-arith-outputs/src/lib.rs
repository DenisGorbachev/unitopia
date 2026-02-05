use core::marker::PhantomData;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Prod<A, B, T> {
    pub inner: T,
    pub unit: PhantomData<(A, B)>,
}

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Quot<A, B, T> {
    pub inner: T,
    pub unit: PhantomData<(A, B)>,
}
