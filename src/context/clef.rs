use crate::storage::{Serialize, SerializedItem};
use alloc::collections::BTreeMap;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum ClefSymbol {
  #[default]
  GClef,
  CClef,
  FClef,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum ClefType {
  #[default]
  Treble,
  Bass,
  FrenchViolin,
  Subbass,
  Tenor,
  Alto,
  Soprano,
  MezzoSoprano,
  BaritoneC,
  BaritoneF,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Clef {
  pub symbol: ClefSymbol,
  pub r#type: ClefType,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Clef {
  #[must_use]
  pub fn new(r#type: ClefType) -> Self {
    Self {
      symbol: match r#type {
        ClefType::Treble | ClefType::FrenchViolin => ClefSymbol::GClef,
        ClefType::Bass | ClefType::Subbass | ClefType::BaritoneF => ClefSymbol::FClef,
        ClefType::Tenor | ClefType::Alto | ClefType::Soprano | ClefType::MezzoSoprano | ClefType::BaritoneC => {
          ClefSymbol::CClef
        }
      },
      r#type,
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for ClefSymbol {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::GClef => "G-Clef",
        Self::CClef => "C-Clef",
        Self::FClef => "F-Clef",
      }
    )
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for ClefType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Treble => "Treble",
        Self::Bass => "Bass",
        Self::FrenchViolin => "French Violin",
        Self::Subbass => "Subbass",
        Self::Tenor => "Tenor",
        Self::Alto => "Alto",
        Self::Soprano => "Soprano",
        Self::MezzoSoprano => "Mezzo Soprano",
        Self::BaritoneC | Self::BaritoneF => "Baritone",
      }
    )
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Clef {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self.r#type {
        ClefType::BaritoneC => String::from("Baritone (C-Clef)"),
        ClefType::BaritoneF => String::from("Baritone (F-Clef)"),
        _ => format!("{}", self.r#type),
      }
    )
  }
}

#[cfg(feature = "print")]
impl Serialize for Clef {
  fn serialize(&self) -> SerializedItem {
    SerializedItem {
      attributes: BTreeMap::from([
        (String::from("type"), self.r#type.to_string()),
        (String::from("symbol"), self.symbol.to_string()),
      ]),
      contents: BTreeMap::new(),
      elements: BTreeMap::new(),
    }
  }
}
