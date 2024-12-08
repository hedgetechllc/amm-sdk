use crate::note::{Accidental, PitchName};
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const FIFTHS_A_MAJOR: i8 = 3;
const FIFTHS_A_FLAT_MAJOR: i8 = -4;
const FIFTHS_B_MAJOR: i8 = 5;
const FIFTHS_B_FLAT_MAJOR: i8 = -2;
const FIFTHS_C_MAJOR: i8 = 0;
const FIFTHS_C_SHARP_MAJOR: i8 = 7;
const FIFTHS_C_FLAT_MAJOR: i8 = -7;
const FIFTHS_D_MAJOR: i8 = 2;
const FIFTHS_D_FLAT_MAJOR: i8 = -5;
const FIFTHS_E_MAJOR: i8 = 4;
const FIFTHS_E_FLAT_MAJOR: i8 = -3;
const FIFTHS_F_MAJOR: i8 = -1;
const FIFTHS_F_SHARP_MAJOR: i8 = 6;
const FIFTHS_G_MAJOR: i8 = 1;
const FIFTHS_G_FLAT_MAJOR: i8 = -6;

const FIFTHS_F_SHARP_MINOR: i8 = 3;
const FIFTHS_F_MINOR: i8 = -4;
const FIFTHS_G_SHARP_MINOR: i8 = 5;
const FIFTHS_G_MINOR: i8 = -2;
const FIFTHS_A_MINOR: i8 = 0;
const FIFTHS_A_SHARP_MINOR: i8 = 7;
const FIFTHS_A_FLAT_MINOR: i8 = -7;
const FIFTHS_B_MINOR: i8 = 2;
const FIFTHS_B_FLAT_MINOR: i8 = -5;
const FIFTHS_C_SHARP_MINOR: i8 = 4;
const FIFTHS_C_MINOR: i8 = -3;
const FIFTHS_D_MINOR: i8 = -1;
const FIFTHS_D_SHARP_MINOR: i8 = 6;
const FIFTHS_E_MINOR: i8 = 1;
const FIFTHS_E_FLAT_MINOR: i8 = -6;

/// Represents the relative intervals between notes in a musical scale.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum KeyMode {
  /// Represents the following note intervals in semitones,
  /// starting from the root note of the corresponding key:
  ///
  /// `[2, 2, 1, 2, 2, 2, 1]`
  #[default]
  Major,
  /// Represents the following note intervals in semitones,
  /// starting from the root note of the corresponding key:
  ///
  /// `[2, 1, 2, 2, 2, 1, 2]`
  Minor,
}

/// Represents the key signature of a musical piece, not taking
/// into account its mode (i.e., major, minor, etc.).
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum KeySignature {
  /// The key of A is defined by a scale with a root note (tonic) of A.
  A,
  /// The key of A# is defined by a scale with a root note (tonic) of A#.
  ASharp,
  /// The key of A♭ is defined by a scale with a root note (tonic) of A♭.
  AFlat,
  /// The key of B is defined by a scale with a root note (tonic) of B.
  B,
  /// The key of B♭ is defined by a scale with a root note (tonic) of B♭.
  BFlat,
  /// The key of C is defined by a scale with a root note (tonic) of C.
  #[default]
  C,
  /// The key of C# is defined by a scale with a root note (tonic) of C#.
  CSharp,
  /// The key of C♭ is defined by a scale with a root note (tonic) of C♭.
  CFlat,
  /// The key of D is defined by a scale with a root note (tonic) of D.
  D,
  /// The key of D# is defined by a scale with a root note (tonic) of D#.
  DSharp,
  /// The key of D♭ is defined by a scale with a root note (tonic) of D♭.
  DFlat,
  /// The key of E is defined by a scale with a root note (tonic) of E.
  E,
  /// The key of E♭ is defined by a scale with a root note (tonic) of E♭.
  EFlat,
  /// The key of F is defined by a scale with a root note (tonic) of F.
  F,
  /// The key of F# is defined by a scale with a root note (tonic) of F#.
  FSharp,
  /// The key of G is defined by a scale with a root note (tonic) of G.
  G,
  /// The key of G# is defined by a scale with a root note (tonic) of G#.
  GSharp,
  /// The key of G♭ is defined by a scale with a root note (tonic) of G♭.
  GFlat,
}

/// Represents the key of a musical piece, including both its
/// mode (i.e., major, minor, etc.) and its signature (defining root note).
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub struct Key {
  /// The mode of the key (i.e., major, minor, etc.).
  pub mode: KeyMode,
  /// The defining root note of the key (i.e., A, A♭, B, etc).
  pub signature: KeySignature,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Key {
  /// Creates a new key with the given signature and mode.
  #[must_use]
  pub fn new(signature: KeySignature, mode: KeyMode) -> Self {
    Self { mode, signature }
  }

  /// Creates a new key from the given circle of fifths value and
  /// optional mode.
  ///
  /// The circle of fifths represents the number of flats or sharps in
  /// a traditional key signature. Negative numbers are used for flats
  /// and positive numbers for sharps. For example, a key with two flats
  /// would be represented by a `fifths` value of `-2`.
  ///
  /// If the `mode` parameter is `None`, the key will default
  /// to [`KeyMode::Major`].
  #[must_use]
  pub fn from_fifths(fifths: i8, mode: Option<KeyMode>) -> Self {
    let mode = mode.unwrap_or(KeyMode::Major);
    let signature = match (fifths, mode) {
      (FIFTHS_A_MAJOR, KeyMode::Major) | (FIFTHS_A_MINOR, KeyMode::Minor) => KeySignature::A,
      (FIFTHS_A_SHARP_MINOR, KeyMode::Minor) => KeySignature::ASharp,
      (FIFTHS_A_FLAT_MAJOR, KeyMode::Major) | (FIFTHS_A_FLAT_MINOR, KeyMode::Minor) => KeySignature::AFlat,
      (FIFTHS_B_MAJOR, KeyMode::Major) | (FIFTHS_B_MINOR, KeyMode::Minor) => KeySignature::B,
      (FIFTHS_B_FLAT_MAJOR, KeyMode::Major) | (FIFTHS_B_FLAT_MINOR, KeyMode::Minor) => KeySignature::BFlat,
      (FIFTHS_C_SHARP_MAJOR, KeyMode::Major) | (FIFTHS_C_SHARP_MINOR, KeyMode::Minor) => KeySignature::CSharp,
      (FIFTHS_C_FLAT_MAJOR, KeyMode::Major) => KeySignature::CFlat,
      (FIFTHS_D_MAJOR, KeyMode::Major) | (FIFTHS_D_MINOR, KeyMode::Minor) => KeySignature::D,
      (FIFTHS_D_SHARP_MINOR, KeyMode::Minor) => KeySignature::DSharp,
      (FIFTHS_D_FLAT_MAJOR, KeyMode::Major) => KeySignature::DFlat,
      (FIFTHS_E_MAJOR, KeyMode::Major) | (FIFTHS_E_MINOR, KeyMode::Minor) => KeySignature::E,
      (FIFTHS_E_FLAT_MAJOR, KeyMode::Major) | (FIFTHS_E_FLAT_MINOR, KeyMode::Minor) => KeySignature::EFlat,
      (FIFTHS_F_MAJOR, KeyMode::Major) | (FIFTHS_F_MINOR, KeyMode::Minor) => KeySignature::F,
      (FIFTHS_F_SHARP_MAJOR, KeyMode::Major) | (FIFTHS_F_SHARP_MINOR, KeyMode::Minor) => KeySignature::FSharp,
      (FIFTHS_G_MAJOR, KeyMode::Major) | (FIFTHS_G_MINOR, KeyMode::Minor) => KeySignature::G,
      (FIFTHS_G_FLAT_MAJOR, KeyMode::Major) => KeySignature::GFlat,
      (FIFTHS_G_SHARP_MINOR, KeyMode::Minor) => KeySignature::GSharp,
      _ => KeySignature::C,
    };
    Self { mode, signature }
  }

  /// Returns the circle of fifths value for the key.
  ///
  /// The circle of fifths represents the number of flats or sharps in
  /// a traditional key signature. Negative numbers are used for flats
  /// and positive numbers for sharps. For example, a key with two flats
  /// would be represented by a `fifths` value of `-2`.
  #[must_use]
  pub fn fifths(&self) -> i8 {
    match (self.signature, self.mode) {
      (KeySignature::A, KeyMode::Major) => FIFTHS_A_MAJOR,
      (KeySignature::A, KeyMode::Minor) => FIFTHS_A_MINOR,
      (KeySignature::ASharp, KeyMode::Minor) => FIFTHS_A_SHARP_MINOR,
      (KeySignature::AFlat, KeyMode::Major) => FIFTHS_A_FLAT_MAJOR,
      (KeySignature::AFlat, KeyMode::Minor) => FIFTHS_A_FLAT_MINOR,
      (KeySignature::B, KeyMode::Major) => FIFTHS_B_MAJOR,
      (KeySignature::B, KeyMode::Minor) => FIFTHS_B_MINOR,
      (KeySignature::BFlat, KeyMode::Major) => FIFTHS_B_FLAT_MAJOR,
      (KeySignature::BFlat, KeyMode::Minor) => FIFTHS_B_FLAT_MINOR,
      (KeySignature::C, KeyMode::Minor) => FIFTHS_C_MINOR,
      (KeySignature::CSharp, KeyMode::Major) => FIFTHS_C_SHARP_MAJOR,
      (KeySignature::CSharp, KeyMode::Minor) => FIFTHS_C_SHARP_MINOR,
      (KeySignature::CFlat, KeyMode::Major) => FIFTHS_C_FLAT_MAJOR,
      (KeySignature::D, KeyMode::Major) => FIFTHS_D_MAJOR,
      (KeySignature::D, KeyMode::Minor) => FIFTHS_D_MINOR,
      (KeySignature::DSharp, KeyMode::Minor) => FIFTHS_D_SHARP_MINOR,
      (KeySignature::DFlat, KeyMode::Major) => FIFTHS_D_FLAT_MAJOR,
      (KeySignature::E, KeyMode::Major) => FIFTHS_E_MAJOR,
      (KeySignature::E, KeyMode::Minor) => FIFTHS_E_MINOR,
      (KeySignature::EFlat, KeyMode::Major) => FIFTHS_E_FLAT_MAJOR,
      (KeySignature::EFlat, KeyMode::Minor) => FIFTHS_E_FLAT_MINOR,
      (KeySignature::F, KeyMode::Major) => FIFTHS_F_MAJOR,
      (KeySignature::F, KeyMode::Minor) => FIFTHS_F_MINOR,
      (KeySignature::FSharp, KeyMode::Major) => FIFTHS_F_SHARP_MAJOR,
      (KeySignature::FSharp, KeyMode::Minor) => FIFTHS_F_SHARP_MINOR,
      (KeySignature::G, KeyMode::Major) => FIFTHS_G_MAJOR,
      (KeySignature::G, KeyMode::Minor) => FIFTHS_G_MINOR,
      (KeySignature::GSharp, KeyMode::Minor) => FIFTHS_G_SHARP_MINOR,
      (KeySignature::GFlat, KeyMode::Major) => FIFTHS_G_FLAT_MAJOR,
      _ => FIFTHS_C_MAJOR,
    }
  }

  /// Returns whether the key contains an accidental on the given pitch.
  #[must_use]
  pub fn contains(&self, pitch: PitchName) -> bool {
    let key_accidentals = self.accidentals();
    key_accidentals[pitch.index()] != Accidental::None
  }

  /// Returns whether the key contains one or more flat accidentals.
  #[must_use]
  pub fn is_flat_key(&self) -> bool {
    self.fifths() < 0
  }

  /// Returns whether the key contains one or more sharp accidentals.
  #[must_use]
  pub fn is_sharp_key(&self) -> bool {
    self.fifths() > 0
  }

  /// Returns a new key with the same tonic (root note) as the current key,
  /// but with the opposite mode (i.e., the parallel key of C-Major
  /// would be C-Minor and vice versa).
  #[must_use]
  pub fn to_parallel(&self) -> Self {
    Self {
      mode: if self.mode == KeyMode::Major {
        KeyMode::Minor
      } else {
        KeyMode::Major
      },
      signature: self.signature,
    }
  }

  /// Returns a new key with the same accidentals as the current key,
  /// but with the opposite mode (i.e., the relative key of C-Major
  /// would be A-Minor and vice versa).
  #[must_use]
  pub fn to_relative(&self) -> Self {
    let new_mode = if self.mode == KeyMode::Major {
      KeyMode::Minor
    } else {
      KeyMode::Major
    };
    Key::from_fifths(self.fifths(), Some(new_mode))
  }

  /// Converts the current key into its parallel key.
  ///
  /// A parallel key is a key with the same tonic (root note) as the current
  /// key, but with the opposite mode (i.e., the parallel key of C-Major
  /// would be C-Minor and vice versa).
  pub fn make_parallel(&mut self) {
    self.mode = if self.mode == KeyMode::Major {
      KeyMode::Minor
    } else {
      KeyMode::Major
    };
  }

  /// Converts the current key into its relative key.
  ///
  /// A relative key is a key with the same accidentals as the current
  /// key, but with the opposite mode (i.e., the relative key of C-Major
  /// would be A-Minor and vice versa).
  pub fn make_relative(&mut self) {
    let new_mode = if self.mode == KeyMode::Major {
      KeyMode::Minor
    } else {
      KeyMode::Major
    };
    *self = Key::from_fifths(self.fifths(), Some(new_mode));
  }

  /// Returns the accidentals for each note in the key.
  ///
  /// The first element in the array represents a rest note, while the
  /// remaining elements represent the notes A through G in order.
  #[must_use]
  pub(crate) fn accidentals(self) -> [Accidental; 8] {
    let fifths = self.fifths();
    [
      Accidental::None,
      match fifths {
        x if x <= -3 => Accidental::Flat,
        x if x >= 5 => Accidental::Sharp,
        _ => Accidental::None,
      },
      match fifths {
        x if x <= -1 => Accidental::Flat,
        x if x >= 7 => Accidental::Sharp,
        _ => Accidental::None,
      },
      match fifths {
        x if x <= -6 => Accidental::Flat,
        x if x >= 2 => Accidental::Sharp,
        _ => Accidental::None,
      },
      match fifths {
        x if x <= -4 => Accidental::Flat,
        x if x >= 4 => Accidental::Sharp,
        _ => Accidental::None,
      },
      match fifths {
        x if x <= -2 => Accidental::Flat,
        x if x >= 6 => Accidental::Sharp,
        _ => Accidental::None,
      },
      match fifths {
        x if x <= -7 => Accidental::Flat,
        x if x >= 1 => Accidental::Sharp,
        _ => Accidental::None,
      },
      match fifths {
        x if x <= -5 => Accidental::Flat,
        x if x >= 3 => Accidental::Sharp,
        _ => Accidental::None,
      },
    ]
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for KeyMode {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Major => "Major",
        Self::Minor => "Minor",
      }
    )
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for KeySignature {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::A => "A",
        Self::ASharp => "A♯",
        Self::AFlat => "A♭",
        Self::B => "B",
        Self::BFlat => "B♭",
        Self::C => "C",
        Self::CSharp => "C♯",
        Self::CFlat => "C♭",
        Self::D => "D",
        Self::DSharp => "D♯",
        Self::DFlat => "D♭",
        Self::E => "E",
        Self::EFlat => "E♭",
        Self::F => "F",
        Self::FSharp => "F♯",
        Self::G => "G",
        Self::GSharp => "G♯",
        Self::GFlat => "G♭",
      }
    )
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Key {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}{}",
      self.signature,
      if self.mode == KeyMode::Major { "" } else { "m" }
    )
  }
}
