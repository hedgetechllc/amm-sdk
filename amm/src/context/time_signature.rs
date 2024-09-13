#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "json")]
use {
  amm_internal::json_prelude::*,
  amm_macros::{JsonDeserialize, JsonSerialize},
};

#[cfg_attr(feature = "json", derive(JsonDeserialize, JsonSerialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum TimeSignatureType {
  #[default]
  CommonTime,
  CutTime,
  Explicit,
  None,
}

#[cfg_attr(feature = "json", derive(JsonDeserialize, JsonSerialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct TimeSignature {
  pub signature: TimeSignatureType,
  pub numerator: u8,
  pub denominator: u8,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl TimeSignature {
  #[must_use]
  pub fn new(signature: TimeSignatureType) -> Self {
    Self {
      signature,
      numerator: match signature {
        TimeSignatureType::CommonTime | TimeSignatureType::Explicit => 4,
        TimeSignatureType::CutTime => 2,
        TimeSignatureType::None => 0,
      },
      denominator: match signature {
        TimeSignatureType::CommonTime | TimeSignatureType::Explicit => 4,
        TimeSignatureType::CutTime => 2,
        TimeSignatureType::None => 0,
      },
    }
  }

  #[must_use]
  pub fn new_explicit(numerator: u8, denominator: u8) -> Self {
    Self {
      signature: TimeSignatureType::Explicit,
      numerator,
      denominator,
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for TimeSignatureType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::CommonTime => "Common Time",
        Self::CutTime => "Cut Time",
        Self::Explicit => "Explicit",
        Self::None => "Senza Misura",
      }
    )
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for TimeSignature {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self.signature {
        TimeSignatureType::Explicit => format!("{}/{}", self.numerator, self.denominator),
        _ => self.signature.to_string(),
      }
    )
  }
}
