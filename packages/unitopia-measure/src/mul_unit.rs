use core::marker::PhantomData;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct MulUnit<Lhs, Rhs> {
    marker: PhantomData<(Lhs, Rhs)>,
}
