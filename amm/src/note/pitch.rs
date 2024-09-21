use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum PitchName {
  #[default]
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
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub struct Pitch {
  pub name: PitchName,
  pub octave: u8,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Pitch {
  #[must_use]
  pub fn new(name: PitchName, octave: u8) -> Self {
    Self { name, octave }
  }

  #[must_use]
  pub fn new_rest() -> Self {
    Self {
      name: PitchName::Rest,
      octave: 0,
    }
  }

  #[must_use]
  pub fn is_rest(&self) -> bool {
    self.name == PitchName::Rest
  }

  #[must_use]
  pub(crate) fn value(self) -> (usize, i8) {
    match self.name {
      PitchName::Rest => (0, 0),
      PitchName::A => (1, (12 * self.octave) as i8 - 48),
      PitchName::B => (2, (2 + (12 * self.octave)) as i8 - 48),
      PitchName::C => (3, (3 + (12 * self.octave)) as i8 - 60),
      PitchName::D => (4, (5 + (12 * self.octave)) as i8 - 60),
      PitchName::E => (5, (7 + (12 * self.octave)) as i8 - 60),
      PitchName::F => (6, (8 + (12 * self.octave)) as i8 - 60),
      PitchName::G => (7, (10 + (12 * self.octave)) as i8 - 60),
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for PitchName {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Rest => "Rest",
        Self::A => "A",
        Self::B => "B",
        Self::C => "C",
        Self::D => "D",
        Self::E => "E",
        Self::F => "F",
        Self::G => "G",
      }
    )
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Pitch {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      if self.name == PitchName::Rest {
        String::new()
      } else {
        format!("{}{}", self.name, self.octave)
      }
    )
  }
}
