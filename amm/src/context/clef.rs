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
pub enum ClefSymbol {
  #[default]
  GClef,
  CClef,
  FClef,
}

#[cfg_attr(feature = "json", derive(JsonDeserialize, JsonSerialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
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
  Baritone,
}

#[cfg_attr(feature = "json", derive(JsonDeserialize, JsonSerialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Clef {
  pub symbol: ClefSymbol,
  pub clef_type: ClefType,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Clef {
  #[must_use]
  pub fn new(clef_type: ClefType, symbol: Option<ClefSymbol>) -> Self {
    Self {
      symbol: match clef_type {
        ClefType::Treble | ClefType::FrenchViolin => ClefSymbol::GClef,
        ClefType::Bass | ClefType::Subbass => ClefSymbol::FClef,
        ClefType::Tenor | ClefType::Alto | ClefType::Soprano | ClefType::MezzoSoprano => ClefSymbol::CClef,
        ClefType::Baritone => symbol.unwrap_or(ClefSymbol::CClef),
      },
      clef_type,
    }
  }
}

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
        Self::Baritone => "Baritone",
      }
    )
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Clef {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.clef_type,)
  }
}
