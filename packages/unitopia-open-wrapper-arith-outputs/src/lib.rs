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

impl<A, B, T> From<T> for Prod<A, B, T> {
    fn from(inner: T) -> Self {
        Self {
            inner,
            unit: PhantomData,
        }
    }
}

impl<A, B, T> From<T> for Quot<A, B, T> {
    fn from(inner: T) -> Self {
        Self {
            inner,
            unit: PhantomData,
        }
    }
}
