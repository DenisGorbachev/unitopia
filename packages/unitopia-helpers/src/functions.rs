use core::error::Error;
use thiserror::Error;

// TODO: Fix error handling
pub fn validate_lossless_conversion<A, B, E>(input: A) -> Result<(), ValidateLosslessConversionError>
where
    B: From<A>,
    A: TryFrom<B, Error = E> + Clone + PartialEq,
    E: Error,
{
    use ValidateLosslessConversionError::*;
    let intermediate = B::from(input.clone());
    let output = A::try_from(intermediate).unwrap();
    if input != output {
        return Err(EqualityCheckFailed);
    }
    Ok(())
}

#[derive(Error, Debug)]
pub enum ValidateLosslessConversionError {
    #[error("equality check failed")]
    EqualityCheckFailed,
}
