use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum Accidental {
  #[default]
  None,
  Natural,
  Sharp,
  Flat,
  DoubleSharp,
  DoubleFlat,
}

impl Accidental {
  #[must_use]
  pub fn value(&self) -> i8 {
    match self {
      Self::Sharp => 1,
      Self::Flat => -1,
      Self::DoubleSharp => 2,
      Self::DoubleFlat => -2,
      Self::None | Self::Natural => 0,
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Accidental {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Natural => "♮",
        Self::Sharp => "♯",
        Self::Flat => "♭",
        Self::DoubleSharp => "𝄪",
        Self::DoubleFlat => "𝄫",
        Self::None => "",
      }
    )
  }
}
