use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Represents the letter name corresponding to a pitch.
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
impl PitchName {
  /// Returns the index of the pitch name within the `PitchName` enum.
  #[must_use]
  pub(crate) fn index(self) -> usize {
    match self {
      PitchName::Rest => 0,
      PitchName::A => 1,
      PitchName::B => 2,
      PitchName::C => 3,
      PitchName::D => 4,
      PitchName::E => 5,
      PitchName::F => 6,
      PitchName::G => 7,
    }
  }
}

/// Represents a musical pitch, which is a combination of a pitch name and octave.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub struct Pitch {
  /// The letter name of the pitch.
  pub name: PitchName,
  /// The octave number of the pitch, where each octave contains 12 semitones.
  pub octave: u8,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Pitch {
  /// Creates a new pitch with the given name and octave.
  ///
  /// **Note:** This method should only be used for musical pitches like
  /// [`PitchName::A`] or [`PitchName::B`]. If you need to create an unvoiced
  /// pitch (a rest), use [`Pitch::new_rest`] instead.
  #[must_use]
  pub fn new(name: PitchName, octave: u8) -> Self {
    Self { name, octave }
  }

  /// Creates a new unvoiced pitch which represents silence.
  #[must_use]
  pub fn new_rest() -> Self {
    Self {
      name: PitchName::Rest,
      octave: 0,
    }
  }

  /// Returns whether the pitch is a rest (i.e., unvoiced).
  #[must_use]
  pub fn is_rest(&self) -> bool {
    self.name == PitchName::Rest
  }

  /// Returns the pitch's value as a tuple of `(pitch_index, num_semitones_from_A4)`.
  #[must_use]
  #[allow(clippy::cast_possible_wrap)]
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

impl Ord for Pitch {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.value().1.cmp(&other.value().1)
  }
}

impl PartialOrd for Pitch {
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
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
