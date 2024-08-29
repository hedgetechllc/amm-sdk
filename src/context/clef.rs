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
        ClefType::Bass | ClefType::Subbass => ClefSymbol::FClef,
        ClefType::Tenor | ClefType::Alto | ClefType::Soprano | ClefType::MezzoSoprano => ClefSymbol::CClef,
        ClefType::BaritoneC => ClefSymbol::CClef,
        ClefType::BaritoneF => ClefSymbol::FClef,
      },
      r#type,
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Clef {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self.r#type {
        ClefType::Treble => "Treble",
        ClefType::Bass => "Bass",
        ClefType::FrenchViolin => "French Violin",
        ClefType::Subbass => "Subbass",
        ClefType::Tenor => "Tenor",
        ClefType::Alto => "Alto",
        ClefType::Soprano => "Soprano",
        ClefType::MezzoSoprano => "Mezzo Soprano",
        ClefType::BaritoneC => "Baritone (C-Clef)",
        ClefType::BaritoneF => "Baritone (F-Clef)",
      }
    )
  }
}
