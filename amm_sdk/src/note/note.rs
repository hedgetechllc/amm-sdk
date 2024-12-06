use super::{Accidental, Duration, Pitch, PitchName};
use crate::context::{generate_id, Key, Tempo};
use crate::modification::{NoteModification, NoteModificationType};
use crate::temporal::Timeslice;
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const A4_FREQUENCY_HZ: f32 = 440.0;
const MIDI_NUMBER_A4: i8 = 69;

/// Represents a note in a musical composition.
#[derive(Debug, Default, Eq, JsonDeserialize, JsonSerialize)]
pub struct Note {
  /// The unique identifier of the note.
  pub id: usize,
  /// The pitch of the note.
  pub pitch: Pitch,
  /// The duration of the note.
  pub duration: Duration,
  /// An accidental modifier on the note (if any).
  pub accidental: Accidental,
  /// A list of modifications on the note.
  modifications: BTreeSet<NoteModification>,
}

impl Note {
  /// Creates a new note with the given pitch, duration, and optional accidental modifier.
  #[must_use]
  pub fn new(pitch: Pitch, duration: Duration, accidental: Option<Accidental>) -> Self {
    Self {
      id: generate_id(),
      pitch,
      duration,
      accidental: accidental.unwrap_or_default(),
      modifications: BTreeSet::new(),
    }
  }

  /// Creates a new note from the given MIDI number, duration, and optional key signature.
  #[must_use]
  pub fn from_midi(mut midi_number: u8, duration: Duration, key: Option<Key>) -> Self {
    if midi_number == 255 {
      Self::new(Pitch::new_rest(), duration, None)
    } else {
      let fifths = key.unwrap_or_default().fifths();
      let (pitch_name, accidental) = match midi_number % 12 {
        0 if fifths > -6 && fifths < 2 => (PitchName::C, Accidental::None),
        0 if fifths >= 7 => {
          midi_number -= 12;
          (PitchName::B, Accidental::None)
        }
        0 => (PitchName::C, Accidental::Natural),
        1 if fifths > -4 && fifths <= 0 => (PitchName::D, Accidental::Flat),
        1 if fifths <= -4 => (PitchName::D, Accidental::None),
        1 if fifths < 2 => (PitchName::C, Accidental::Sharp),
        1 => (PitchName::C, Accidental::None),
        2 if fifths > -4 && fifths < 4 => (PitchName::D, Accidental::None),
        2 => (PitchName::D, Accidental::Natural),
        3 if fifths > -2 && fifths <= 0 => (PitchName::E, Accidental::Flat),
        3 if fifths <= -2 => (PitchName::E, Accidental::None),
        3 if fifths < 4 => (PitchName::D, Accidental::Sharp),
        3 => (PitchName::D, Accidental::None),
        4 if fifths > -2 && fifths < 6 => (PitchName::E, Accidental::None),
        4 if fifths <= -7 => (PitchName::F, Accidental::None),
        4 => (PitchName::E, Accidental::Natural),
        5 if fifths > -7 && fifths < 1 => (PitchName::F, Accidental::None),
        5 if fifths >= 6 => (PitchName::E, Accidental::None),
        5 => (PitchName::F, Accidental::Natural),
        6 if fifths > -5 && fifths < 0 => (PitchName::G, Accidental::Flat),
        6 if fifths <= -5 => (PitchName::G, Accidental::None),
        6 if fifths < 1 => (PitchName::F, Accidental::Sharp),
        6 => (PitchName::F, Accidental::None),
        7 if fifths > -5 && fifths < 3 => (PitchName::G, Accidental::None),
        7 => (PitchName::G, Accidental::Natural),
        8 if fifths > -3 && fifths <= 0 => (PitchName::A, Accidental::Flat),
        8 if fifths <= -3 => (PitchName::A, Accidental::None),
        8 if fifths < 3 => (PitchName::G, Accidental::Sharp),
        8 => (PitchName::G, Accidental::None),
        9 if fifths > -3 && fifths < 5 => (PitchName::A, Accidental::None),
        9 => (PitchName::A, Accidental::Natural),
        10 if fifths == 0 => (PitchName::B, Accidental::Flat),
        10 if fifths < 0 => (PitchName::B, Accidental::None),
        10 if fifths < 5 => (PitchName::A, Accidental::Sharp),
        10 => (PitchName::A, Accidental::None),
        11 if fifths > -1 && fifths < 7 => (PitchName::B, Accidental::None),
        11 if fifths <= -6 => {
          midi_number += 12;
          (PitchName::C, Accidental::None)
        }
        11 => (PitchName::B, Accidental::Natural),
        _ => (PitchName::Rest, Accidental::None),
      };
      Self::new(Pitch::new(pitch_name, midi_number / 12 - 1), duration, Some(accidental))
    }
  }

  /// Returns the unique identifier of the note.
  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }

  /// Returns the number of semitones between the note and A4, taking into
  /// account the accidentals for a given key signature.
  #[must_use]
  fn semitone_distance(&self, key_accidentals: [Accidental; 8]) -> i8 {
    let (pitch_index, num_semitones) = self.pitch.value();
    num_semitones
      + if self.accidental == Accidental::None {
        key_accidentals[pitch_index].value()
      } else {
        self.accidental.value()
      }
  }

  /// Returns whether the note is the same pitch as another note.
  #[must_use]
  pub fn is_same_pitch(&self, other: &Note) -> bool {
    self.pitch == other.pitch
  }

  /// Returns whether the note is a rest (i.e., unvoiced).
  #[must_use]
  pub fn is_rest(&self) -> bool {
    self.pitch.is_rest()
  }

  /// Returns whether the note is a grace note.
  #[must_use]
  pub fn is_grace_note(&self) -> bool {
    self
      .modifications
      .iter()
      .any(|modification| matches!(modification.r#type, NoteModificationType::Grace { .. }))
  }

  /// Returns the pitch of the note in Hertz,
  /// optionally taking into account a key signature.
  #[must_use]
  pub fn pitch_hz(&self, key: Option<Key>, a4_frequency_hz: Option<f32>) -> f32 {
    let accidentals = key.unwrap_or_default().accidentals();
    a4_frequency_hz.unwrap_or(A4_FREQUENCY_HZ) * 2f32.powf(f32::from(self.semitone_distance(accidentals)) / 12.0)
  }

  /// Returns the pitch of the note in MIDI number format,
  /// optionally taking into account a key signature.
  #[must_use]
  #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
  pub fn midi_number(&self, key: Option<Key>) -> u8 {
    let accidentals = key.unwrap_or_default().accidentals();
    (MIDI_NUMBER_A4 + self.semitone_distance(accidentals)) as u8
  }

  /// Returns the duration of the note in beats,
  ///
  /// The `base_beat_value` parameter defines the type of note that represents a single beat.
  #[must_use]
  pub fn beats(&self, base_beat_value: f64) -> f64 {
    if self.is_grace_note() {
      0.0
    } else {
      self.duration.beats(base_beat_value)
    }
  }

  /// Adds a modification to the note and returns a unique identifier for the modification.
  pub fn add_modification(&mut self, mod_type: NoteModificationType) -> usize {
    let modification = NoteModification::new(mod_type);
    let modification_id = modification.get_id();
    self.modifications.replace(modification);
    modification_id
  }

  /// Returns a note modification based on the specified unique identifier.
  #[must_use]
  pub fn get_modification(&self, id: usize) -> Option<&NoteModification> {
    self
      .iter_modifications()
      .find(|modification| modification.get_id() == id)
  }

  /// Returns the number of beats for the note, taking into account
  /// a base beat value and optional tuplet ratio.
  ///
  /// The `beat_base` parameter defines the type of note that represents a single beat.
  ///
  /// The `tuplet_ratio` parameter defines the ratio of the note's target duration to
  /// its original, unmodified duration.
  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration, tuplet_ratio: Option<f64>) -> f64 {
    self.beats(beat_base.value()) * tuplet_ratio.unwrap_or(1.0)
  }

  /// Returns the duration of the note in seconds, taking into account
  /// a tempo and optional tuplet ratio.
  ///
  /// The `tuplet_ratio` parameter defines the ratio of the note's target duration to
  /// its original, unmodified duration.
  #[must_use]
  pub fn get_duration(&self, tempo: &Tempo, tuplet_ratio: Option<f64>) -> f64 {
    self.get_beats(&tempo.base_note, tuplet_ratio) * 60.0 / f64::from(tempo.beats_per_minute)
  }

  /// Removes a modification from the note based on the specified unique identifier.
  pub fn remove_modification(&mut self, id: usize) -> &mut Self {
    self.modifications.retain(|modification| modification.get_id() != id);
    self
  }

  /// Returns an iterator over the note's modifications.
  pub fn iter_modifications(&self) -> alloc::collections::btree_set::Iter<'_, NoteModification> {
    self.modifications.iter()
  }

  /// Returns a [`Timeslice`] containing only this single note.
  #[must_use]
  pub fn to_timeslice(&self) -> Timeslice {
    let mut timeslice = Timeslice::new();
    timeslice.add_note(self.clone());
    timeslice
  }
}

impl PartialEq for Note {
  fn eq(&self, other: &Self) -> bool {
    let default_duration = Duration::default().value();
    let default_accidentals = Key::default().accidentals();
    (self.semitone_distance(default_accidentals) == other.semitone_distance(default_accidentals))
      && (self.beats(default_duration) == other.beats(default_duration))
      && (self.modifications == other.modifications)
  }
}

impl Clone for Note {
  fn clone(&self) -> Self {
    Self {
      id: generate_id(),
      pitch: self.pitch,
      duration: self.duration,
      accidental: self.accidental,
      modifications: self.modifications.clone(),
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Note {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let mods = self
      .iter_modifications()
      .map(ToString::to_string)
      .collect::<Vec<String>>()
      .join(", ");
    write!(
      f,
      "{}{}{}{} {}{}",
      self.pitch,
      self.accidental,
      if self.is_rest() { "" } else { " " },
      self.duration,
      if self.is_rest() { "Rest" } else { "Note" },
      if mods.is_empty() {
        String::new()
      } else {
        format!(" ({mods})")
      },
    )
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::context::{Key, KeyMode, KeySignature};

  #[test]
  fn test_note_from_midi() {
    let key_options = [
      Key::new(KeySignature::C, KeyMode::Major),
      Key::new(KeySignature::CFlat, KeyMode::Major),
      Key::new(KeySignature::CSharp, KeyMode::Major),
      Key::new(KeySignature::D, KeyMode::Major),
      Key::new(KeySignature::DFlat, KeyMode::Major),
      Key::new(KeySignature::DSharp, KeyMode::Major),
      Key::new(KeySignature::E, KeyMode::Major),
      Key::new(KeySignature::EFlat, KeyMode::Major),
      Key::new(KeySignature::F, KeyMode::Major),
      Key::new(KeySignature::FSharp, KeyMode::Major),
      Key::new(KeySignature::G, KeyMode::Major),
      Key::new(KeySignature::GFlat, KeyMode::Major),
      Key::new(KeySignature::GSharp, KeyMode::Major),
      Key::new(KeySignature::A, KeyMode::Major),
      Key::new(KeySignature::AFlat, KeyMode::Major),
      Key::new(KeySignature::ASharp, KeyMode::Major),
      Key::new(KeySignature::B, KeyMode::Major),
      Key::new(KeySignature::BFlat, KeyMode::Major),
    ];
    for key in key_options {
      for midi_number in 84..96 {
        let note = Note::from_midi(midi_number, Duration::default(), Some(key)).semitone_distance(key.accidentals());
        let expected_note =
          Note::from_midi(midi_number, Duration::default(), None).semitone_distance(Key::default().accidentals());
        assert_eq!(note, expected_note);
      }
    }
  }
}
