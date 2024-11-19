use super::{Accidental, Duration, Pitch};
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
