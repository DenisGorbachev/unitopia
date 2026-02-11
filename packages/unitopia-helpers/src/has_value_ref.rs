/// [`HasValueRef`] should only be used for types that don't implement copy. Prefer [`HasValueCopy`](crate::HasValueCopy) instead.
pub trait HasValueRef {
    type Output;

    fn value(&self) -> &Self::Output;
}
