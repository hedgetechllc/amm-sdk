use crate::note::{Duration, DurationType};
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Represents an explicit tempo marking in music notation.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub struct Tempo {
  /// The base note which represents a single "beat" in the tempo.
  pub base_note: Duration,
  /// The number of beats per minute.
  pub beats_per_minute: u16,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Tempo {
  /// Creates a new tempo with the given base note and beats per minute.
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
