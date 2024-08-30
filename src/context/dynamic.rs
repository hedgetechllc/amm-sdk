use crate::storage::{Serialize, SerializedItem};
use alloc::collections::BTreeMap;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum DynamicMarking {
  #[default]
  None,
  Forte,
  MezzoForte,
  MezzoPiano,
  Piano,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Dynamic {
  pub r#type: DynamicMarking,
  pub repetitions: u8,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Dynamic {
  #[must_use]
  pub fn new(r#type: DynamicMarking, repetitions: u8) -> Self {
    Self { r#type, repetitions }
  }

  #[must_use]
  pub fn value(&self) -> f32 {
    match self.r#type {
      DynamicMarking::Piano => (0.5 - (0.1 * f32::from(self.repetitions))).max(0.05),
      DynamicMarking::MezzoPiano => 0.45,
      DynamicMarking::MezzoForte => 0.55,
      DynamicMarking::Forte => (0.5 + (0.1 * f32::from(self.repetitions))).min(1.0),
      DynamicMarking::None => 0.5,
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for DynamicMarking {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Piano => "p",
        Self::MezzoPiano => "mp",
        Self::MezzoForte => "mf",
        Self::Forte => "f",
        Self::None => "",
      }
    )
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Dynamic {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match self.r#type {
      DynamicMarking::Piano => write!(f, "{}", "p".repeat(usize::from(self.repetitions))),
      DynamicMarking::MezzoPiano => write!(f, "mp"),
      DynamicMarking::MezzoForte => write!(f, "mf"),
      DynamicMarking::Forte => write!(f, "{}", "f".repeat(usize::from(self.repetitions))),
      DynamicMarking::None => write!(f, ""),
    }
  }
}

#[cfg(feature = "print")]
impl Serialize for Dynamic {
  fn serialize(&self) -> SerializedItem {
    SerializedItem {
      attributes: match self.r#type {
        DynamicMarking::None => BTreeMap::new(),
        _ => BTreeMap::from([(String::from("marking"), self.r#type.to_string())]),
      },
      contents: BTreeMap::new(),
      elements: BTreeMap::new(),
    }
  }
}
