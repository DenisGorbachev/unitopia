use core::error::Error;
use errgonomic::handle_bool;
use thiserror::Error;

pub fn validate_lossless_conversion<A, B, E>(input: A) -> Result<(), ValidateLosslessConversionError>
where
    B: From<A>,
    A: TryFrom<B, Error = E> + Clone + PartialEq,
    E: Error,
{
    use ValidateLosslessConversionError::*;
    let intermediate = B::from(input.clone());
    let output = A::try_from(intermediate).map_err(|_| ReverseConversionFailed)?;
    handle_bool!(input != output, EqualityCheckFailed);
    Ok(())
}

#[derive(Error, Debug)]
pub enum ValidateLosslessConversionError {
    #[error("reverse conversion failed")]
    ReverseConversionFailed,
    #[error("equality check failed")]
    EqualityCheckFailed,
}
