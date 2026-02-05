use core::marker::PhantomData;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct DivUnit<Lhs, Rhs> {
    marker: PhantomData<(Lhs, Rhs)>,
}
