use core::marker::PhantomData;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct PowUnit<Unit, const POWER: u32> {
    marker: PhantomData<Unit>,
}
