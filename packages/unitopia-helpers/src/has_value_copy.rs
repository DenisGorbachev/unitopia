/// Provides access to the underlying value by copy
pub trait HasValueCopy {
    type Output;

    fn value(self) -> Self::Output;
}
