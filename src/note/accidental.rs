#[cfg(target_arch = "wasm32")]
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
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
  pub fn value(&self) -> i16 {
    match self {
      Self::Sharp => 1,
      Self::Flat => -1,
      Self::DoubleSharp => 2,
      Self::DoubleFlat => -2,
      _ => 0,
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
