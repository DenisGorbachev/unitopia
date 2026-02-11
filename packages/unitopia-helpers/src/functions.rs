use core::error::Error;
use errgonomic::{handle, handle_bool};
use thiserror::Error;

pub fn validate_lossless_conversion<A, B, E>(input: A) -> Result<(), ValidateLosslessConversionError<A, E>>
where
    B: From<A>,
    A: TryFrom<B, Error = E> + Clone + PartialEq,
    E: Error,
{
    use ValidateLosslessConversionError::*;
    let intermediate = B::from(input.clone());
    let output = handle!(A::try_from(intermediate), ReverseConversionFailed, input);
    handle_bool!(input != output, EqualityCheckFailed, input, output);
    Ok(())
}

#[derive(Error, Debug)]
pub enum ValidateLosslessConversionError<A, E> {
    #[error("reverse conversion failed")]
    ReverseConversionFailed { source: E, input: A },
    #[error("equality check failed")]
    EqualityCheckFailed { input: A, output: A },
}
