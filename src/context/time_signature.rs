use crate::storage::{Serialize, SerializedItem};
use alloc::collections::BTreeMap;
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
      match self.r#type {
        TimeSignatureType::Explicit => format!(": {}/{}", self.numerator, self.denominator),
        _ => self.r#type.to_string(),
      }
    )
  }
}

#[cfg(feature = "print")]
impl Serialize for TimeSignature {
  fn serialize(&self) -> SerializedItem {
    SerializedItem {
      attributes: BTreeMap::from([
        (String::from("type"), self.r#type.to_string()),
        (String::from("numerator"), self.numerator.to_string()),
        (String::from("denominator"), self.denominator.to_string()),
      ]),
      contents: BTreeMap::new(),
      elements: BTreeMap::new(),
    }
  }
}
