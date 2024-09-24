use super::{chord::Chord, multivoice::MultiVoice, phrase::Phrase, timeslice::Timeslice};
use crate::context::{generate_id, Tempo};
use crate::modification::{Direction, DirectionType};
use crate::note::{Accidental, Duration, Note, Pitch};
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};
use core::slice::{Iter, IterMut};

#[derive(Clone, Debug, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum StaffContent {
  Note(Note),
  Chord(Chord),
  Phrase(Phrase),
  MultiVoice(MultiVoice),
  Direction(Direction),
}

#[derive(Debug, Default, Eq, JsonDeserialize, JsonSerialize)]
pub struct Staff {
  id: usize,
  name: String,
  content: Vec<StaffContent>,
}

impl Staff {
  #[must_use]
  pub fn new(name: &str) -> Staff {
    Self {
      id: generate_id(),
      name: String::from(name),
      content: Vec::new(),
    }
  }

  #[must_use]
  pub fn flatten(&self) -> Self {
    Self {
      id: generate_id(),
      name: self.name.clone(),
      content: self
        .content
        .iter()
        .map(|item| match item {
          StaffContent::Note(note) => StaffContent::Note(note.clone()),
          StaffContent::Chord(chord) => StaffContent::Chord(chord.clone()),
          StaffContent::Phrase(phrase) => StaffContent::Phrase(phrase.flatten(false)),
          StaffContent::MultiVoice(multivoice) => StaffContent::Phrase(multivoice.flatten()),
          StaffContent::Direction(direction) => StaffContent::Direction(direction.clone()),
        })
        .collect(),
    }
  }

  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }

  #[must_use]
  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn rename(&mut self, name: &str) -> &mut Self {
    self.name = String::from(name);
    self
  }

  pub fn add_note(&mut self, pitch: Pitch, duration: Duration, accidental: Option<Accidental>) -> &mut Note {
    self
      .content
      .push(StaffContent::Note(Note::new(pitch, duration, accidental)));
    match self.content.last_mut() {
      Some(StaffContent::Note(note)) => note,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn add_chord(&mut self) -> &mut Chord {
    self.content.push(StaffContent::Chord(Chord::new()));
    match self.content.last_mut() {
      Some(StaffContent::Chord(chord)) => chord,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn add_phrase(&mut self) -> &mut Phrase {
    self.content.push(StaffContent::Phrase(Phrase::new()));
    match self.content.last_mut() {
      Some(StaffContent::Phrase(phrase)) => phrase,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn add_multivoice(&mut self) -> &mut MultiVoice {
    self.content.push(StaffContent::MultiVoice(MultiVoice::new()));
    match self.content.last_mut() {
      Some(StaffContent::MultiVoice(multivoice)) => multivoice,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn add_direction(&mut self, direction: DirectionType) -> &mut Direction {
    self.content.push(StaffContent::Direction(Direction::new(direction)));
    match self.content.last_mut() {
      Some(StaffContent::Direction(direction)) => direction,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn claim_note(&mut self, note: Note) -> &mut Note {
    self.content.push(StaffContent::Note(note));
    match self.content.last_mut() {
      Some(StaffContent::Note(note)) => note,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn claim_chord(&mut self, chord: Chord) -> &mut Chord {
    self.content.push(StaffContent::Chord(chord));
    match self.content.last_mut() {
      Some(StaffContent::Chord(chord)) => chord,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn claim_phrase(&mut self, phrase: Phrase) -> &mut Phrase {
    self.content.push(StaffContent::Phrase(phrase));
    match self.content.last_mut() {
      Some(StaffContent::Phrase(phrase)) => phrase,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn claim_multivoice(&mut self, multivoice: MultiVoice) -> &mut MultiVoice {
    self.content.push(StaffContent::MultiVoice(multivoice));
    match self.content.last_mut() {
      Some(StaffContent::MultiVoice(multivoice)) => multivoice,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn claim_direction(&mut self, direction: Direction) -> &mut Direction {
    self.content.push(StaffContent::Direction(direction));
    match self.content.last_mut() {
      Some(StaffContent::Direction(direction)) => direction,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn insert_note(
    &mut self,
    index: usize,
    pitch: Pitch,
    duration: Duration,
    accidental: Option<Accidental>,
  ) -> &mut Note {
    self
      .content
      .insert(index, StaffContent::Note(Note::new(pitch, duration, accidental)));
    match self.content.last_mut() {
      Some(StaffContent::Note(note)) => note,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn insert_chord(&mut self, index: usize) -> &mut Chord {
    self.content.insert(index, StaffContent::Chord(Chord::new()));
    match self.content.last_mut() {
      Some(StaffContent::Chord(chord)) => chord,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn insert_phrase(&mut self, index: usize) -> &mut Phrase {
    self.content.insert(index, StaffContent::Phrase(Phrase::new()));
    match self.content.last_mut() {
      Some(StaffContent::Phrase(phrase)) => phrase,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn insert_multivoice(&mut self, index: usize) -> &mut MultiVoice {
    self.content.insert(index, StaffContent::MultiVoice(MultiVoice::new()));
    match self.content.last_mut() {
      Some(StaffContent::MultiVoice(multivoice)) => multivoice,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn insert_direction(&mut self, index: usize, direction: DirectionType) -> &mut Direction {
    self
      .content
      .insert(index, StaffContent::Direction(Direction::new(direction)));
    match self.content.last_mut() {
      Some(StaffContent::Direction(direction)) => direction,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  #[must_use]
  pub fn get_note(&self, id: usize) -> Option<&Note> {
    self.content.iter().find_map(|item| match item {
      StaffContent::Note(note) if note.get_id() == id => Some(note),
      StaffContent::Chord(chord) => chord.get_note(id),
      StaffContent::Phrase(phrase) => phrase.get_note(id),
      StaffContent::MultiVoice(multivoice) => multivoice.get_note(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_note_mut(&mut self, id: usize) -> Option<&mut Note> {
    self.content.iter_mut().find_map(|item| match item {
      StaffContent::Note(note) if note.get_id() == id => Some(note),
      StaffContent::Chord(chord) => chord.get_note_mut(id),
      StaffContent::Phrase(phrase) => phrase.get_note_mut(id),
      StaffContent::MultiVoice(multivoice) => multivoice.get_note_mut(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_chord(&self, id: usize) -> Option<&Chord> {
    self.content.iter().find_map(|item| match item {
      StaffContent::Chord(chord) if chord.get_id() == id => Some(chord),
      StaffContent::Phrase(phrase) => phrase.get_chord(id),
      StaffContent::MultiVoice(multivoice) => multivoice.get_chord(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_chord_mut(&mut self, id: usize) -> Option<&mut Chord> {
    self.content.iter_mut().find_map(|item| match item {
      StaffContent::Chord(chord) if chord.get_id() == id => Some(chord),
      StaffContent::Phrase(phrase) => phrase.get_chord_mut(id),
      StaffContent::MultiVoice(multivoice) => multivoice.get_chord_mut(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_phrase(&self, id: usize) -> Option<&Phrase> {
    self.content.iter().find_map(|item| match item {
      StaffContent::Phrase(phrase) if phrase.get_id() == id => Some(phrase),
      StaffContent::Phrase(phrase) => phrase.get_phrase(id),
      StaffContent::MultiVoice(multivoice) => multivoice.get_phrase(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_phrase_mut(&mut self, id: usize) -> Option<&mut Phrase> {
    self.content.iter_mut().find_map(|item| match item {
      StaffContent::Phrase(phrase) => {
        if phrase.get_id() == id {
          Some(phrase)
        } else {
          phrase.get_phrase_mut(id)
        }
      }
      StaffContent::MultiVoice(multivoice) => multivoice.get_phrase_mut(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_multivoice(&self, id: usize) -> Option<&MultiVoice> {
    self.content.iter().find_map(|item| match item {
      StaffContent::MultiVoice(multivoice) if multivoice.get_id() == id => Some(multivoice),
      StaffContent::MultiVoice(multivoice) => multivoice.get_multivoice(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_multivoice_mut(&mut self, id: usize) -> Option<&mut MultiVoice> {
    self.content.iter_mut().find_map(|item| match item {
      StaffContent::MultiVoice(multivoice) => {
        if multivoice.get_id() == id {
          Some(multivoice)
        } else {
          multivoice.get_multivoice_mut(id)
        }
      }
      _ => None,
    })
  }

  #[must_use]
  pub fn get_direction(&self, id: usize) -> Option<&Direction> {
    self.content.iter().find_map(|item| match item {
      StaffContent::Direction(direction) if direction.get_id() == id => Some(direction),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_direction_mut(&mut self, id: usize) -> Option<&mut Direction> {
    self.content.iter_mut().find_map(|item| match item {
      StaffContent::Direction(direction) if direction.get_id() == id => Some(direction),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_index_of_item(&self, id: usize) -> Option<usize> {
    self.content.iter().position(|item| match item {
      StaffContent::Note(note) => note.get_id() == id,
      StaffContent::Chord(chord) => chord.get_id() == id,
      StaffContent::Phrase(phrase) => phrase.get_id() == id,
      StaffContent::MultiVoice(multivoice) => multivoice.get_id() == id,
      StaffContent::Direction(direction) => direction.get_id() == id,
    })
  }

  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration) -> f64 {
    self
      .content
      .iter()
      .map(|content| match &content {
        StaffContent::Note(note) => note.get_beats(beat_base, None),
        StaffContent::Chord(chord) => chord.get_beats(beat_base, None),
        StaffContent::Phrase(phrase) => phrase.get_beats(beat_base, None),
        StaffContent::MultiVoice(multivoice) => multivoice.get_beats(beat_base, None),
        StaffContent::Direction(_) => 0.0,
      })
      .sum()
  }

  #[must_use]
  pub fn get_duration(&self, tempo: &Tempo) -> f64 {
    self.get_beats(&tempo.base_note) * 60.0 / f64::from(tempo.beats_per_minute)
  }

  pub fn remove_item(&mut self, id: usize) -> &mut Self {
    self.content.retain(|item| match item {
      StaffContent::Note(note) => note.get_id() != id,
      StaffContent::Chord(chord) => chord.get_id() != id,
      StaffContent::Phrase(phrase) => phrase.get_id() != id,
      StaffContent::MultiVoice(multivoice) => multivoice.get_id() != id,
      StaffContent::Direction(direction) => direction.get_id() != id,
    });
    self.content.iter_mut().for_each(|item| match item {
      StaffContent::Chord(chord) => {
        chord.remove_item(id);
      }
      StaffContent::Phrase(phrase) => {
        phrase.remove_item(id);
      }
      StaffContent::MultiVoice(multivoice) => {
        multivoice.remove_item(id);
      }
      _ => (),
    });
    self
  }

  #[must_use]
  pub fn num_timeslices(&self) -> usize {
    self
      .content
      .iter()
      .map(|item| match item {
        StaffContent::Note(_) | StaffContent::Chord(_) => 1,
        StaffContent::Phrase(phrase) => phrase.num_timeslices(),
        StaffContent::MultiVoice(multivoice) => multivoice.num_timeslices(),
        StaffContent::Direction(_) => 0,
      })
      .sum()
  }

  pub fn iter(&self) -> Iter<'_, StaffContent> {
    self.content.iter()
  }

  pub fn iter_mut(&mut self) -> IterMut<'_, StaffContent> {
    self.content.iter_mut()
  }

  #[must_use]
  pub fn iter_timeslices(&self) -> Vec<Timeslice> {
    let mut timeslices = Vec::new();
    self.content.iter().for_each(|item| match item {
      StaffContent::Note(note) => {
        let mut timeslice = Timeslice::new();
        timeslice.add_note(note.clone());
        timeslices.push(timeslice);
      }
      StaffContent::Chord(chord) => {
        timeslices.push(chord.to_timeslice());
      }
      StaffContent::Phrase(phrase) => {
        timeslices.append(&mut phrase.iter_timeslices());
      }
      StaffContent::MultiVoice(multivoice) => {
        timeslices.append(&mut multivoice.iter_timeslices());
      }
      StaffContent::Direction(direction) => {
        let mut timeslice = Timeslice::new();
        timeslice.add_direction(direction.clone());
        timeslices.push(timeslice);
      }
    });
    timeslices
  }
}

impl IntoIterator for Staff {
  type Item = StaffContent;
  type IntoIter = alloc::vec::IntoIter<Self::Item>;
  fn into_iter(self) -> Self::IntoIter {
    self.content.into_iter()
  }
}

impl<'a> IntoIterator for &'a Staff {
  type Item = &'a StaffContent;
  type IntoIter = Iter<'a, StaffContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl Clone for Staff {
  fn clone(&self) -> Self {
    Self {
      id: generate_id(),
      name: self.name.clone(),
      content: self.content.clone(),
    }
  }
}

impl PartialEq for Staff {
  fn eq(&self, other: &Self) -> bool {
    self.content == other.content && self.name == other.name
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Staff {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let items = self
      .content
      .iter()
      .map(|item| match item {
        StaffContent::Note(note) => note.to_string(),
        StaffContent::Chord(chord) => chord.to_string(),
        StaffContent::Phrase(phrase) => phrase.to_string(),
        StaffContent::MultiVoice(multi_voice) => multi_voice.to_string(),
        StaffContent::Direction(direction) => direction.r#type.to_string(),
      })
      .collect::<Vec<_>>()
      .join(", ");
    write!(f, "Staff {}: [{items}]", self.name)
  }
}
