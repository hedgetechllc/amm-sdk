use crate::context::{Key, Tempo, TimeSignature};
use crate::modification::{Direction, PhraseModificationType};
use crate::note::{Accidental, Duration, Pitch};
use crate::structure::note::Note;
use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Default)]
pub struct TimesliceContext {
  pub key: Key,
  pub original_tempo: Tempo,
  pub current_tempo: Tempo,
  pub time_signature: TimeSignature,
}

pub struct TimeslicePhraseDetails {
  pub modifications: Vec<PhraseModificationType>,
  pub index_in_phrase: usize,
  pub phrase_length: usize,
  pub next_pitch: Pitch,
  pub next_accidental: Accidental,
}

impl core::fmt::Display for TimeslicePhraseDetails {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "Note {} of {} in {}",
      self.index_in_phrase + 1,
      self.phrase_length,
      self
        .modifications
        .iter()
        .map(|modification| modification.to_string())
        .collect::<Vec<String>>()
        .join(", ")
    )
  }
}

pub struct TimesliceContent {
  pub note: Rc<RefCell<Note>>,
  pub phrase_details: Vec<TimeslicePhraseDetails>,
}

impl TimesliceContent {
  #[must_use]
  pub fn new(note: &Rc<RefCell<Note>>) -> Self {
    Self {
      note: Rc::clone(note),
      phrase_details: Vec::new(),
    }
  }

  pub fn add_phrase_details(&mut self, index_in_phrase: usize, phrase_length: usize) -> &mut TimeslicePhraseDetails {
    self.phrase_details.push(TimeslicePhraseDetails {
      modifications: Vec::new(),
      index_in_phrase,
      phrase_length,
      next_pitch: Pitch::Rest,
      next_accidental: Accidental::None,
    });
    unsafe { self.phrase_details.last_mut().unwrap_unchecked() }
  }

  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration) -> f64 {
    self.note.borrow().get_beats(
      beat_base,
      self.phrase_details.iter().find_map(|detail| {
        detail.modifications.iter().find_map(|modification| match modification {
          PhraseModificationType::Tuplet { num_beats, into_beats } => {
            Some(f64::from(*into_beats) / f64::from(*num_beats))
          }
          _ => None,
        })
      }),
    )
  }

  #[must_use]
  pub fn get_duration(&self, tempo: &Tempo) -> f64 {
    self.get_beats(&tempo.base_note) * 60.0 / f64::from(tempo.beats_per_minute)
  }

  #[must_use]
  pub fn get_pcm_samples(&self, context: &TimesliceContext) -> Vec<f32> {
    todo!() // TODO: Implement
  }
}

impl core::fmt::Display for TimesliceContent {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let phrase_context = self
      .phrase_details
      .iter()
      .map(|detail| detail.to_string())
      .collect::<Vec<String>>()
      .join(", ");
    write!(
      f,
      "{}{}{}{}",
      self.note.borrow().to_string(),
      if self.phrase_details.is_empty() {
        ""
      } else {
        " with Phrase Context: ["
      },
      phrase_context,
      if self.phrase_details.is_empty() { "" } else { "]" },
    )
  }
}

#[derive(Default)]
pub struct Timeslice {
  pub arpeggiated: bool,
  pub content: Vec<TimesliceContent>,
  pub directions: Vec<Rc<RefCell<Direction>>>,
}

impl Timeslice {
  #[must_use]
  pub fn new() -> Self {
    Self {
      arpeggiated: false,
      content: Vec::new(),
      directions: Vec::new(),
    }
  }

  pub fn add_note(&mut self, note: &Rc<RefCell<Note>>) -> &mut Self {
    self.content.push(TimesliceContent::new(note));
    self
  }

  pub fn add_direction(&mut self, direction: &Rc<RefCell<Direction>>) -> &mut Self {
    self.directions.push(Rc::clone(direction));
    self
  }

  pub fn combine(&mut self, other: &mut Self) -> &mut Self {
    self.arpeggiated = self.arpeggiated || other.arpeggiated;
    self.content.append(other.content.as_mut());
    self.directions.append(other.directions.as_mut());
    self
  }

  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration) -> f64 {
    self
      .content
      .iter()
      .map(|element| element.get_beats(beat_base))
      .reduce(f64::min)
      .unwrap_or_default()
  }

  #[must_use]
  pub fn get_duration(&self, tempo: &Tempo) -> f64 {
    self.get_beats(&tempo.base_note) * 60.0 / f64::from(tempo.beats_per_minute)
  }
}

impl core::fmt::Display for Timeslice {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let directions_string = self
      .directions
      .iter()
      .map(|direction| direction.borrow().to_string())
      .collect::<Vec<String>>()
      .join(", ");
    write!(
      f,
      "Timeslice: {}{}{}{}{}{}",
      if self.directions.is_empty() {
        ""
      } else {
        "Directions: ["
      },
      directions_string,
      if self.directions.is_empty() { "" } else { "], " },
      if self.content.is_empty() { "" } else { "Content: [" },
      self
        .content
        .iter()
        .map(|content| content.to_string())
        .collect::<Vec<String>>()
        .join(", "),
      if self.content.is_empty() { "" } else { "]" },
    )
  }
}

#[derive(Default)]
pub struct PartTimeslice {
  pub timeslices: BTreeMap<String, Timeslice>,
}

impl PartTimeslice {
  #[must_use]
  pub fn new() -> Self {
    Self {
      timeslices: BTreeMap::new(),
    }
  }

  #[must_use]
  pub fn from(part_name: &str, timeslice: Timeslice) -> Self {
    Self {
      timeslices: BTreeMap::from([(String::from(part_name), timeslice)]),
    }
  }

  pub fn add_timeslice(&mut self, name: &str, timeslice: Timeslice) -> &mut Self {
    self.timeslices.insert(String::from(name), timeslice);
    self
  }

  #[must_use]
  pub fn get_timeslice(&self, name: &str) -> Option<&Timeslice> {
    self.timeslices.get(name)
  }
}
