use alloc::string::String;
use core::borrow::Borrow;
use core::fmt::{self, Display, Formatter};
use core::ops::Deref;
use core::str::FromStr;
use errgonomic::handle;
use thiserror::Error;

/// A validated, case-sensitive UCUM unit code.
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(subtype::SerializeTransparent, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
#[repr(transparent)]
pub struct UcumCode(String);

impl TryFrom<String> for UcumCode {
    type Error = ConvertStringToUcumCodeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        use ConvertStringToUcumCodeError::*;
        handle!(ucum::validate(&value), ValidateFailed, value);
        Ok(Self(value))
    }
}

impl FromStr for UcumCode {
    type Err = UcumCodeFromStrError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use UcumCodeFromStrError::*;
        let value = String::from(value);
        let code = handle!(Self::try_from(value), TryFromFailed);
        Ok(code)
    }
}

impl AsRef<str> for UcumCode {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Borrow<str> for UcumCode {
    fn borrow(&self) -> &str {
        self.0.borrow()
    }
}

impl Deref for UcumCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl Display for UcumCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<UcumCode> for String {
    fn from(value: UcumCode) -> Self {
        value.0
    }
}

#[derive(Error, Debug)]
pub enum ConvertStringToUcumCodeError {
    #[error("failed to validate UCUM code '{value}'")]
    ValidateFailed { source: ucum::UcumError, value: String },
}

#[derive(Error, Debug)]
pub enum UcumCodeFromStrError {
    #[error("failed to parse a UCUM code")]
    TryFromFailed { source: ConvertStringToUcumCodeError },
}
