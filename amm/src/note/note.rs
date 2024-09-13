use super::{Accidental, Duration, DurationType, Pitch};
use crate::context::{Key, KeyMode, KeySignature};
use crate::modification::NoteModification;
use alloc::{
  rc::Rc,
  string::{String, ToString},
  vec::Vec,
};
use core::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "json")]
use {
  amm_internal::json_prelude::*,
  amm_macros::{JsonDeserialize, JsonSerialize},
};

const A4_FREQUENCY_HZ: f32 = 440.0;
const MIDI_NUMBER_A4: i16 = 69;

#[cfg_attr(feature = "json", derive(JsonDeserialize, JsonSerialize))]
#[derive(Clone, Debug, Default, Eq)]
pub struct Note {
  pub id: usize,
  pub pitch: Pitch,
  pub duration: Duration,
  pub accidental: Accidental,
  pub modifications: Vec<Rc<RefCell<NoteModification>>>,
}

impl Note {
  #[must_use]
  fn semitone_distance(&self, key_accidentals: [Accidental; 8]) -> i16 {
    let (pitch_index, num_semitones) = self.pitch.value();
    num_semitones
      + if self.accidental == Accidental::None {
        key_accidentals[pitch_index].value()
      } else {
        self.accidental.value()
      }
  }

  #[must_use]
  pub fn is_same_pitch(&self, other: &Note) -> bool {
    self.pitch == other.pitch
  }

  #[must_use]
  pub fn is_rest(&self) -> bool {
    self.pitch.is_rest()
  }

  #[must_use]
  pub fn pitch_hz(&self, key: Option<Key>, a4_frequency_hz: Option<f32>) -> f32 {
    let accidentals = key.unwrap_or(Key::new(KeySignature::C, KeyMode::Major)).accidentals();
    a4_frequency_hz.unwrap_or(A4_FREQUENCY_HZ) * 2f32.powf(f32::from(self.semitone_distance(accidentals)) / 12.0)
  }

  #[must_use]
  #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
  pub fn midi_number(&self, key: Option<Key>) -> u8 {
    let accidentals = key.unwrap_or(Key::new(KeySignature::C, KeyMode::Major)).accidentals();
    (MIDI_NUMBER_A4 + self.semitone_distance(accidentals)) as u8
  }

  #[must_use]
  pub fn beats(&self, base_beat_value: f64) -> f64 {
    self.duration.beats(base_beat_value)
  }
}

impl PartialEq for Note {
  fn eq(&self, other: &Self) -> bool {
    let c_major_accidentals = Key::new(KeySignature::C, KeyMode::Major).accidentals();
    let quarter_duration = Duration::new(DurationType::Quarter, 0).value();
    (self.semitone_distance(c_major_accidentals) == other.semitone_distance(c_major_accidentals))
      && (self.beats(quarter_duration) == other.beats(quarter_duration))
      && (self.is_rest() == other.is_rest())
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Note {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let mods = self
      .modifications
      .iter()
      .map(|modification| modification.borrow().to_string())
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
