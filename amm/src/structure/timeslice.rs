use crate::context::{Key, Tempo, TimeSignature};
use crate::modification::{Direction, PhraseModificationType};
use crate::note::{Accidental, Duration, Note, Pitch};
use alloc::{collections::BTreeMap, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct TimesliceContext {
  pub key: Key,
  pub original_tempo: Tempo,
  pub current_tempo: Tempo,
  pub time_signature: TimeSignature,
}

#[derive(Debug)]
pub struct TimeslicePhraseDetails {
  pub modifications: Vec<PhraseModificationType>,
  pub index_in_phrase: usize,
  pub phrase_length: usize,
  pub next_pitch: Pitch,
  pub next_accidental: Accidental,
}

#[cfg(feature = "print")]
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
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(", ")
    )
  }
}

#[derive(Debug)]
pub struct TimesliceContent {
  pub note: Note,
  pub phrase_details: Vec<TimeslicePhraseDetails>,
}

impl TimesliceContent {
  #[must_use]
  pub fn new(note: Note) -> Self {
    Self {
      note,
      phrase_details: Vec::new(),
    }
  }

  pub fn add_phrase_details(&mut self, index_in_phrase: usize, phrase_length: usize) -> &mut TimeslicePhraseDetails {
    self.phrase_details.push(TimeslicePhraseDetails {
      modifications: Vec::new(),
      index_in_phrase,
      phrase_length,
      next_pitch: Pitch::new_rest(),
      next_accidental: Accidental::None,
    });
    unsafe { self.phrase_details.last_mut().unwrap_unchecked() }
  }

  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration) -> f64 {
    self.note.get_beats(
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
  pub fn get_pcm_samples(&self, _context: &TimesliceContext) -> Vec<f32> {
    todo!() // TODO: Implement
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for TimesliceContent {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let phrase_context = self
      .phrase_details
      .iter()
      .map(ToString::to_string)
      .collect::<Vec<String>>()
      .join(", ");
    write!(
      f,
      "{}{}{}{}",
      self.note,
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

#[derive(Debug, Default)]
pub struct Timeslice {
  pub arpeggiated: bool,
  pub content: Vec<TimesliceContent>,
  pub directions: Vec<Direction>,
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

  pub fn add_note(&mut self, note: Note) -> &mut Self {
    self.content.push(TimesliceContent::new(note));
    self
  }

  pub fn add_direction(&mut self, direction: Direction) -> &mut Self {
    self.directions.push(direction);
    self
  }

  pub fn combine_with(&mut self, other: &mut Self) -> &mut Self {
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

#[cfg(feature = "print")]
impl core::fmt::Display for Timeslice {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let directions_string = self
      .directions
      .iter()
      .map(ToString::to_string)
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
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(", "),
      if self.content.is_empty() { "" } else { "]" },
    )
  }
}

#[derive(Debug, Default)]
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

  pub fn add_timeslice(&mut self, part_name: &str, timeslice: Timeslice) -> &mut Self {
    self.timeslices.insert(String::from(part_name), timeslice);
    self
  }

  #[must_use]
  pub fn get_timeslice_for(&self, part_name: &str) -> Option<&Timeslice> {
    self.timeslices.get(part_name)
  }
}
