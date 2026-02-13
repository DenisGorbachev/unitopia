/// A trait for scaling to a larger unit (e.g. gram -> kilogram)
pub trait Upscale<M> {
    type Output;

    /// The implementation must contain a `debug_assert!(numerator > denominator)`, then it can be sure that certain operations will be infallible
    /// The implementation must return a remainder
    fn upscale(self, numerator: M, denominator: M) -> Self::Output;
}

impl Upscale<u64> for u64 {
    // TODO
    type Output = ();

    fn upscale(self, _numerator: u64, _denominator: u64) -> Self::Output {
        todo!()
    }
}
