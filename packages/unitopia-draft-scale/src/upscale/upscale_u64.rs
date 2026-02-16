use crate::Upscale;

impl Upscale<u64> for u64 {
    /// `(upscaled, remainder)` where:
    /// `self * denominator = upscaled * numerator + remainder`.
    ///
    /// Returning `remainder` preserves the fractional part that would
    /// otherwise be lost in integer-only upscaling.
    type Output = (u64, u64);

    fn upscale(self, numerator: u64, denominator: u64) -> Self::Output {
        assert!(numerator >= denominator);
        assert_ne!(denominator, 0);

        let scaled = u128::from(self)
            .checked_mul(u128::from(denominator))
            .expect("always succeeds because product of two u64 values always fits into u128");

        let upscaled_u128 = scaled
            .checked_div(u128::from(numerator))
            .expect("always succeeds because numerator is greater than denominator, therefore numerator is non-zero");

        let remainder_u128 = scaled
            .checked_rem(u128::from(numerator))
            .expect("always succeeds because numerator is greater than denominator, therefore numerator is non-zero");

        let upscaled = u64::try_from(upscaled_u128).expect("always succeeds because upscaling to a larger unit with positive factors cannot increase the original u64 value");

        let remainder = u64::try_from(remainder_u128).expect("always succeeds because remainder is strictly less than numerator and numerator has u64 type");

        (upscaled, remainder)
    }
}

#[cfg(test)]
mod tests {
    use crate::Upscale;
    use crate::upscale::test_helpers::{assert_panics_if_denominator_is_zero, assert_panics_if_numerator_is_less_than_denominator};

    #[test]
    fn must_upscale_u64() {
        assert_eq!(1550u64.upscale(1000, 1), (1, 550))
    }

    #[test]
    fn must_panic_if_numerator_is_less_than_denominator_u64() {
        assert_panics_if_numerator_is_less_than_denominator::<u64>();
    }

    #[test]
    fn must_panic_if_denominator_is_zero_u64() {
        assert_panics_if_denominator_is_zero::<u64>();
    }
}
