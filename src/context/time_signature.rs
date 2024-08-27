#[cfg(target_arch = "wasm32")]
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum TimeSignature {
  #[default]
  CommonTime,
  CutTime,
  Explicit(u8, u8),
  None,
}

#[cfg(feature = "print")]
impl core::fmt::Display for TimeSignature {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match *self {
      Self::CommonTime => write!(f, "Common Time"),
      Self::CutTime => write!(f, "Cut Time"),
      Self::Explicit(numerator, denominator) => write!(f, "{numerator}/{denominator}"),
      Self::None => write!(f, "Senza Misura"),
    }
  }
}
