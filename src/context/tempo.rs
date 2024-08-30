use crate::note::{Duration, DurationType};
use crate::storage::{Serialize, SerializedItem};
use alloc::collections::BTreeMap;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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

#[cfg(feature = "print")]
impl Serialize for Tempo {
  fn serialize(&self) -> SerializedItem {
    SerializedItem {
      attributes: BTreeMap::from([(String::from("beats_per_minutes"), self.beats_per_minute.to_string())]),
      contents: BTreeMap::from([(String::from("base_note"), self.base_note.serialize())]),
      elements: BTreeMap::new(),
    }
  }
}
