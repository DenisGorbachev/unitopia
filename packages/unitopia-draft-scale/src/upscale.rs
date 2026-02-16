/// A trait for scaling to a larger unit (e.g. gram -> kilogram)
pub trait Upscale<M> {
    type Output;

    /// The implementation must contain an `assert!(numerator >= denominator)` to ensure that certain operations will be infallible
    /// The implementation must contain an `assert_ne!(denominator, 0);`
    /// The implementation must return a remainder
    fn upscale(self, numerator: M, denominator: M) -> Self::Output;
}

#[cfg(test)]
mod test_helpers {
    use crate::Upscale;
    use core::fmt::Debug;
    use std::panic::{UnwindSafe, catch_unwind};

    pub fn assert_panics_if_numerator_is_less_than_denominator<T>()
    where
        T: Upscale<T>,
        T: From<u8>,
        T: Copy,
        T: Debug,
        T: UnwindSafe,
    {
        let value = T::from(10);
        let numerator = T::from(1);
        let denominator = T::from(2);
        let result = catch_unwind(move || {
            let _ = value.upscale(numerator, denominator);
        });
        assert!(result.is_err());
    }

    pub fn assert_panics_if_denominator_is_zero<T>()
    where
        T: Upscale<T>,
        T: From<u8>,
        T: Copy,
        T: Debug,
        T: UnwindSafe,
    {
        let value = T::from(10);
        let numerator = T::from(1);
        let denominator = T::from(0);
        let result = catch_unwind(move || {
            let _ = value.upscale(numerator, denominator);
        });
        assert!(result.is_err());
    }
}

mod upscale_u64;

mod upscale_u128;
