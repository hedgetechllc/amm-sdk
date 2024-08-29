#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum TimeSignatureType {
  #[default]
  CommonTime,
  CutTime,
  Explicit,
  None,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct TimeSignature {
  pub r#type: TimeSignatureType,
  pub numerator: u8,
  pub denominator: u8,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl TimeSignature {
  #[must_use]
  pub fn new(r#type: TimeSignatureType) -> Self {
    Self {
      r#type,
      numerator: 4,
      denominator: 4,
    }
  }

  #[must_use]
  pub fn new_explicit(numerator: u8, denominator: u8) -> Self {
    Self {
      r#type: TimeSignatureType::Explicit,
      numerator,
      denominator,
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for TimeSignature {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match self.r#type {
      TimeSignatureType::CommonTime => write!(f, "Common Time"),
      TimeSignatureType::CutTime => write!(f, "Cut Time"),
      TimeSignatureType::Explicit => write!(f, "{}/{}", self.numerator, self.denominator),
      TimeSignatureType::None => write!(f, "Senza Misura"),
    }
  }
}
