use super::{Accidental, Duration, Pitch};
use crate::context::Key;
use crate::modification::NoteModification;
use alloc::{rc::Rc, string::String, vec::Vec};
use core::cell::RefCell;

const A4_FREQUENCY_HZ: f32 = 440.0;
const MIDI_NUMBER_A4: i16 = 69;

#[derive(Clone, Eq)]
pub struct Note {
  pub id: usize,
  pub pitch: Pitch,
  pub duration: Duration,
  pub accidental: Accidental,
  pub modifications: Vec<Rc<RefCell<NoteModification>>>,
}

impl Note {
  fn semitone_distance(&self, key_accidentals: &[Accidental; 8]) -> i16 {
    let (pitch_index, num_semitones) = self.pitch.value();
    num_semitones
      + if self.accidental != Accidental::None {
        self.accidental.value()
      } else {
        key_accidentals[pitch_index].value()
      }
  }

  pub fn is_same_pitch(&self, other: &Note) -> bool {
    self.pitch == other.pitch
  }

  pub fn is_rest(&self) -> bool {
    self.pitch.is_rest()
  }

  pub fn pitch_hz(&self, key_accidentals: &[Accidental; 8], a4_frequency_hz: Option<f32>) -> f32 {
    a4_frequency_hz.unwrap_or(A4_FREQUENCY_HZ) * 2.0_f32.powf(f32::from(self.semitone_distance(key_accidentals)) / 12.0)
  }

  pub fn midi_number(&self, key_accidentals: &[Accidental; 8]) -> u8 {
    (MIDI_NUMBER_A4 + self.semitone_distance(key_accidentals)) as u8
  }

  pub fn beats(&self, base_beat_value: f64) -> f64 {
    self.duration.beats(base_beat_value)
  }
}

impl PartialEq for Note {
  fn eq(&self, other: &Self) -> bool {
    let c_major_accidentals = &Key::CMajor.accidentals();
    let quarter_duration = Duration::Quarter(0).value();
    (self.semitone_distance(c_major_accidentals) == other.semitone_distance(c_major_accidentals))
      && (self.beats(quarter_duration) == other.beats(quarter_duration))
      && (self.is_rest() == other.is_rest())
  }
}

impl core::fmt::Display for Note {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let mods = self
      .modifications
      .iter()
      .map(|modification| modification.borrow_mut().to_string())
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
        format!(" ({})", mods)
      },
    )
  }
}
