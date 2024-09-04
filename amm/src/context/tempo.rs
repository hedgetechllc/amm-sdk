use crate::note::{Duration, DurationType};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "json")]
use {
  amm_internal::json_prelude::*,
  amm_macros::{JsonDeserialize, JsonSerialize},
};

#[cfg_attr(feature = "json", derive(JsonDeserialize, JsonSerialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempo {
  pub base_note: Duration,
  pub beats_per_minute: u16,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Tempo {
  #[must_use]
  pub fn new(base_note: Duration, beats_per_minute: u16) -> Self {
    Self {
      base_note,
      beats_per_minute,
    }
  }
}

impl Default for Tempo {
  fn default() -> Self {
    Self {
      base_note: Duration::new(DurationType::Quarter, 0),
      beats_per_minute: 120,
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Tempo {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}={} bpm", self.base_note, self.beats_per_minute)
  }
}
