#[cfg(feature = "json")]
use {
  amm_internal::json_prelude::*,
  amm_macros::{JsonDeserialize, JsonSerialize},
};

#[cfg_attr(feature = "json", derive(JsonDeserialize, JsonSerialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum Dynamic {
  Forte(u8),
  #[default]
  MezzoForte,
  MezzoPiano,
  Piano(u8),
}

impl Dynamic {
  #[must_use]
  pub fn value(&self) -> f32 {
    match *self {
      Self::Piano(magnitude) => (0.5 - (0.1 * f32::from(magnitude))).max(0.05),
      Self::MezzoPiano => 0.45,
      Self::MezzoForte => 0.55,
      Self::Forte(magnitude) => (0.5 + (0.1 * f32::from(magnitude))).min(1.0),
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Dynamic {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match *self {
      Self::Piano(magnitude) => write!(f, "{}", "p".repeat(usize::from(magnitude))),
      Self::MezzoPiano => write!(f, "mp"),
      Self::MezzoForte => write!(f, "mf"),
      Self::Forte(magnitude) => write!(f, "{}", "f".repeat(usize::from(magnitude))),
    }
  }
}
