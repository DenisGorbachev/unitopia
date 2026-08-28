use crate::{ConvertStringToUcumCodeError, UcumCode};
use ConvertStringToUcumCodeError::*;
use PolyUnit::*;
use alloc::string::String;
use core::fmt::{self, Display, Formatter};
use smart_default::SmartDefault;

/// A reported unit string classified by whether it is a valid, case-sensitive UCUM code.
#[derive(SmartDefault, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "String", untagged))]
pub enum PolyUnit {
    /// A recognized UCUM code.
    Ucum(UcumCode),
    /// A source unit string that was not recognized by more specific variants.
    #[default]
    Unrecognized(String),
}

impl From<String> for PolyUnit {
    /// PRUNING: discards the UCUM validation error after preserving the complete source string in [`PolyUnit::Unrecognized`], because this conversion classifies input without failing.
    fn from(value: String) -> Self {
        match UcumCode::try_from(value) {
            Ok(code) => Ucum(code),
            Err(ValidateFailed {
                value,
                ..
            }) => Unrecognized(value),
        }
    }
}

impl From<&str> for PolyUnit {
    fn from(value: &str) -> Self {
        Self::from(String::from(value))
    }
}

impl From<UcumCode> for PolyUnit {
    fn from(code: UcumCode) -> Self {
        Ucum(code)
    }
}

impl AsRef<str> for PolyUnit {
    fn as_ref(&self) -> &str {
        match self {
            Ucum(code) => code.as_ref(),
            Unrecognized(value) => value,
        }
    }
}

impl Display for PolyUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}
