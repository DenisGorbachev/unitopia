use crate::Upscale;

fn wide_ge((lhs_high, lhs_low): (u8, u128), (rhs_high, rhs_low): (u8, u128)) -> bool {
    if lhs_high > rhs_high {
        true
    } else if lhs_high < rhs_high {
        false
    } else {
        lhs_low >= rhs_low
    }
}

fn wide_sub_u128((high, low): (u8, u128), rhs: u128) -> (u8, u128) {
    if low >= rhs {
        (
            high,
            low.checked_sub(rhs)
                .expect("always succeeds because low is greater than or equal to rhs"),
        )
    } else {
        let decremented_high = high
            .checked_sub(1)
            .expect("always succeeds because caller ensures this subtraction does not underflow");
        let rhs_complement = u128::MAX
            .checked_sub(rhs)
            .expect("always succeeds because rhs has u128 type");
        let rhs_twos_complement = rhs_complement
            .checked_add(1)
            .expect("always succeeds because rhs is non-zero");
        let borrowed_low = low
            .checked_add(rhs_twos_complement)
            .expect("always succeeds because low is strictly less than rhs");
        (decremented_high, borrowed_low)
    }
}

fn mul_div_rem_u128(value: u128, numerator: u128, denominator: u128) -> (u128, u128) {
    assert!(numerator >= denominator);
    assert_ne!(denominator, 0);

    if numerator == denominator {
        return (value, 0);
    }

    let (double_numerator_low, double_numerator_carry) = numerator.overflowing_add(numerator);
    let double_numerator_wide = (u8::from(double_numerator_carry), double_numerator_low);

    (0u32..128u32)
        .rev()
        .fold((0u128, 0u128), |(quotient, remainder), bit_index| {
            let shifted = value
                .checked_shr(bit_index)
                .expect("always succeeds because bit_index is in 0..128");
            let bit = shifted & 1;
            let addend = if bit == 0 { 0 } else { denominator };

            let (double_remainder_low, double_remainder_carry) = remainder.overflowing_add(remainder);
            let (t_low, t_add_carry) = double_remainder_low.overflowing_add(addend);
            let t_high = u8::from(double_remainder_carry)
                .checked_add(u8::from(t_add_carry))
                .expect("always succeeds because only two carry bits may be produced");
            let t = (t_high, t_low);

            let t_ge_numerator = wide_ge(t, (0, numerator));
            let t_ge_double_numerator = wide_ge(t, double_numerator_wide);
            let carry = if t_ge_double_numerator {
                2u8
            } else if t_ge_numerator {
                1u8
            } else {
                0u8
            };

            let reduced = match carry {
                0 => t,
                1 => wide_sub_u128(t, numerator),
                2 => wide_sub_u128(wide_sub_u128(t, numerator), numerator),
                _ => unreachable!("carry is derived from a bounded comparison and must be 0, 1, or 2"),
            };
            assert_eq!(reduced.0, 0, "always succeeds because reduced remainder is strictly less than numerator");

            let doubled_quotient = quotient
                .checked_add(quotient)
                .expect("always succeeds because quotient is bounded by the processed input prefix");
            let quotient_next = doubled_quotient
                .checked_add(u128::from(carry))
                .expect("always succeeds because carry has value 0, 1, or 2");
            (quotient_next, reduced.1)
        })
}

impl Upscale<u128> for u128 {
    /// `(upscaled, remainder)` where:
    /// `self * denominator = upscaled * numerator + remainder`.
    ///
    /// Returning `remainder` preserves the fractional part that would
    /// otherwise be lost in integer-only upscaling.
    type Output = (u128, u128);

    fn upscale(self, numerator: u128, denominator: u128) -> Self::Output {
        mul_div_rem_u128(self, numerator, denominator)
    }
}

#[cfg(test)]
mod tests {
    use crate::Upscale;
    use crate::upscale::test_helpers::{assert_panics_if_denominator_is_zero, assert_panics_if_numerator_is_less_than_denominator};

    #[test]
    fn must_upscale_u128() {
        assert_eq!(1550u128.upscale(1000, 1), (1, 550))
    }

    #[test]
    fn must_upscale_u128_without_multiplication_overflow() {
        let numerator = u128::MAX;
        let denominator = numerator
            .checked_sub(1)
            .expect("always succeeds because numerator is greater than one");
        assert_eq!(u128::MAX.upscale(numerator, denominator), (denominator, 0));
    }

    #[test]
    fn must_panic_if_numerator_is_less_than_denominator_u128() {
        assert_panics_if_numerator_is_less_than_denominator::<u128>();
    }

    #[test]
    fn must_panic_if_denominator_is_zero_u128() {
        assert_panics_if_denominator_is_zero::<u128>();
    }
}
