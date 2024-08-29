use alloc::string::String;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PitchName {
  Rest,
  A,
  B,
  C,
  D,
  E,
  F,
  G,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pitch {
  pub r#type: PitchName,
  pub octave: u8,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Pitch {
  #[must_use]
  pub fn new(r#type: PitchName, octave: u8) -> Self {
    Self { r#type, octave }
  }

  #[must_use]
  pub fn new_rest() -> Self {
    Self {
      r#type: PitchName::Rest,
      octave: 0,
    }
  }

  #[must_use]
  pub fn is_rest(&self) -> bool {
    self.r#type == PitchName::Rest
  }

  #[must_use]
  pub(crate) fn value(&self) -> (usize, i16) {
    match self.r#type {
      PitchName::Rest => (0, 0),
      PitchName::A => (1, i16::from(12 * self.octave) - 48),
      PitchName::B => (2, i16::from(2 + (12 * self.octave)) - 48),
      PitchName::C => (3, i16::from(3 + (12 * self.octave)) - 60),
      PitchName::D => (4, i16::from(5 + (12 * self.octave)) - 60),
      PitchName::E => (5, i16::from(7 + (12 * self.octave)) - 60),
      PitchName::F => (6, i16::from(8 + (12 * self.octave)) - 60),
      PitchName::G => (7, i16::from(10 + (12 * self.octave)) - 60),
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Pitch {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self.r#type {
        PitchName::Rest => String::new(),
        PitchName::A => format!("A{}", self.octave),
        PitchName::B => format!("B{}", self.octave),
        PitchName::C => format!("C{}", self.octave),
        PitchName::D => format!("D{}", self.octave),
        PitchName::E => format!("E{}", self.octave),
        PitchName::F => format!("F{}", self.octave),
        PitchName::G => format!("G{}", self.octave),
      }
    )
  }
}
