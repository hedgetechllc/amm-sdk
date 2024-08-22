use super::{chord::Chord, multivoice::MultiVoice, phrase::Phrase, timeslice::Timeslice};
use crate::context::{generate_id, Clef, Key, Tempo, TimeSignature};
use crate::modification::{Direction, DirectionType};
use crate::note::{Accidental, Duration, Note, Pitch};
use alloc::{
  rc::Rc,
  string::{String, ToString},
  vec::Vec,
};
use core::{cell::RefCell, slice::Iter};

#[derive(Clone)]
pub enum StaffContent {
  Note(Rc<RefCell<Note>>),
  Chord(Rc<RefCell<Chord>>),
  Phrase(Rc<RefCell<Phrase>>),
  MultiVoice(Rc<RefCell<MultiVoice>>),
  Direction(Rc<RefCell<Direction>>),
}

#[derive(Clone)]
pub struct Staff {
  id: usize,
  name: String,
  content: Vec<StaffContent>,
}

impl Staff {
  #[must_use]
  pub fn new(
    name: &str,
    clef: Option<Clef>,
    key: Option<Key>,
    time_signature: Option<TimeSignature>,
  ) -> Rc<RefCell<Staff>> {
    let mut staff = Self {
      id: generate_id(),
      name: String::from(name),
      content: Vec::new(),
    };
    if let Some(clef) = clef {
      staff.add_direction(DirectionType::Clef { clef });
    }
    if let Some(key) = key {
      staff.add_direction(DirectionType::Key { key });
    }
    if let Some(time_signature) = time_signature {
      staff.add_direction(DirectionType::TimeSignature { time_signature });
    }
    Rc::new(RefCell::new(staff))
  }

  #[must_use]
  pub fn flatten(&self) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: self.id,
      name: self.name.clone(),
      content: self
        .content
        .iter()
        .map(|item| match item {
          StaffContent::Note(note) => StaffContent::Note(Rc::clone(note)),
          StaffContent::Chord(chord) => StaffContent::Chord(Rc::clone(chord)),
          StaffContent::Phrase(phrase) => StaffContent::Phrase(phrase.borrow().flatten(false)),
          StaffContent::MultiVoice(multivoice) => StaffContent::Phrase(multivoice.borrow().flatten()),
          StaffContent::Direction(direction) => StaffContent::Direction(Rc::clone(direction)),
        })
        .collect(),
    }))
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

  pub fn add_note(&mut self, pitch: Pitch, duration: Duration, accidental: Option<Accidental>) -> Rc<RefCell<Note>> {
    let note = Note::new(pitch, duration, accidental);
    self.content.push(StaffContent::Note(Rc::clone(&note)));
    note
  }

  pub fn add_chord(&mut self) -> Rc<RefCell<Chord>> {
    let chord = Chord::new();
    self.content.push(StaffContent::Chord(Rc::clone(&chord)));
    chord
  }

  pub fn add_phrase(&mut self) -> Rc<RefCell<Phrase>> {
    let phrase = Phrase::new();
    self.content.push(StaffContent::Phrase(Rc::clone(&phrase)));
    phrase
  }

  pub fn add_multivoice(&mut self) -> Rc<RefCell<MultiVoice>> {
    let multivoice = MultiVoice::new();
    self.content.push(StaffContent::MultiVoice(Rc::clone(&multivoice)));
    multivoice
  }

  pub fn add_direction(&mut self, direction: DirectionType) -> Rc<RefCell<Direction>> {
    let direction = Direction::new(direction);
    self.content.push(StaffContent::Direction(Rc::clone(&direction)));
    direction
  }

  pub fn insert_note(
    &mut self,
    index: usize,
    pitch: Pitch,
    duration: Duration,
    accidental: Option<Accidental>,
  ) -> Rc<RefCell<Note>> {
    let note = Note::new(pitch, duration, accidental);
    self.content.insert(index, StaffContent::Note(Rc::clone(&note)));
    note
  }

  pub fn insert_chord(&mut self, index: usize) -> Rc<RefCell<Chord>> {
    let chord = Chord::new();
    self.content.insert(index, StaffContent::Chord(Rc::clone(&chord)));
    chord
  }

  pub fn insert_phrase(&mut self, index: usize) -> Rc<RefCell<Phrase>> {
    let phrase = Phrase::new();
    self.content.insert(index, StaffContent::Phrase(Rc::clone(&phrase)));
    phrase
  }

  pub fn insert_multivoice(&mut self, index: usize) -> Rc<RefCell<MultiVoice>> {
    let multivoice = MultiVoice::new();
    self
      .content
      .insert(index, StaffContent::MultiVoice(Rc::clone(&multivoice)));
    multivoice
  }

  pub fn insert_direction(&mut self, index: usize, direction: DirectionType) -> Rc<RefCell<Direction>> {
    let direction = Direction::new(direction);
    self
      .content
      .insert(index, StaffContent::Direction(Rc::clone(&direction)));
    direction
  }

  #[must_use]
  pub fn get_note(&mut self, id: usize) -> Option<Rc<RefCell<Note>>> {
    self.content.iter().find_map(|item| match item {
      StaffContent::Note(note) if note.borrow().get_id() == id => Some(Rc::clone(note)),
      StaffContent::Chord(chord) => chord.borrow_mut().get_note(id),
      StaffContent::Phrase(phrase) => phrase.borrow_mut().get_note(id),
      StaffContent::MultiVoice(multivoice) => multivoice.borrow_mut().get_note(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_chord(&mut self, id: usize) -> Option<Rc<RefCell<Chord>>> {
    self.content.iter().find_map(|item| match item {
      StaffContent::Chord(chord) if chord.borrow().get_id() == id => Some(Rc::clone(chord)),
      StaffContent::Phrase(phrase) => phrase.borrow_mut().get_chord(id),
      StaffContent::MultiVoice(multivoice) => multivoice.borrow_mut().get_chord(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_phrase(&mut self, id: usize) -> Option<Rc<RefCell<Phrase>>> {
    self.content.iter().find_map(|item| match item {
      StaffContent::Phrase(phrase) if phrase.borrow().get_id() == id => Some(Rc::clone(phrase)),
      StaffContent::Phrase(phrase) => phrase.borrow_mut().get_phrase(id),
      StaffContent::MultiVoice(multivoice) => multivoice.borrow_mut().get_phrase(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_multivoice(&mut self, id: usize) -> Option<Rc<RefCell<MultiVoice>>> {
    self.content.iter().find_map(|item| match item {
      StaffContent::MultiVoice(multivoice) if multivoice.borrow().get_id() == id => Some(Rc::clone(multivoice)),
      StaffContent::MultiVoice(multivoice) => multivoice.borrow_mut().get_multivoice(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_direction(&mut self, id: usize) -> Option<Rc<RefCell<Direction>>> {
    self.content.iter().find_map(|item| match item {
      StaffContent::Direction(direction) if direction.borrow().get_id() == id => Some(Rc::clone(direction)),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_index_of_item(&mut self, id: usize) -> Option<usize> {
    self.content.iter().position(|item| match item {
      StaffContent::Note(note) => note.borrow().get_id() == id,
      StaffContent::Chord(chord) => chord.borrow().get_id() == id,
      StaffContent::Phrase(phrase) => phrase.borrow().get_id() == id,
      StaffContent::MultiVoice(multivoice) => multivoice.borrow().get_id() == id,
      StaffContent::Direction(direction) => direction.borrow().get_id() == id,
    })
  }

  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration) -> f64 {
    self
      .content
      .iter()
      .map(|content| match &content {
        StaffContent::Note(note) => note.borrow().get_beats(beat_base, None),
        StaffContent::Chord(chord) => chord.borrow().get_beats(beat_base, None),
        StaffContent::Phrase(phrase) => phrase.borrow().get_beats(beat_base, None),
        StaffContent::MultiVoice(multivoice) => multivoice.borrow().get_beats(beat_base, None),
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
      StaffContent::Note(note) => note.borrow().get_id() != id,
      StaffContent::Chord(chord) => chord.borrow().get_id() != id,
      StaffContent::Phrase(phrase) => phrase.borrow().get_id() != id,
      StaffContent::MultiVoice(multivoice) => multivoice.borrow().get_id() != id,
      StaffContent::Direction(direction) => direction.borrow().get_id() != id,
    });
    self.content.iter().for_each(|item| match item {
      StaffContent::Chord(chord) => {
        chord.borrow_mut().remove_item(id);
      }
      StaffContent::Phrase(phrase) => {
        phrase.borrow_mut().remove_item(id);
      }
      StaffContent::MultiVoice(multivoice) => {
        multivoice.borrow_mut().remove_item(id);
      }
      _ => (),
    });
    self
  }

  pub fn iter(&self) -> Iter<'_, StaffContent> {
    self.content.iter()
  }

  #[must_use]
  pub fn iter_timeslices(&self) -> Vec<Timeslice> {
    let mut timeslices = Vec::new();
    self.content.iter().for_each(|item| match item {
      StaffContent::Note(note) => {
        let mut timeslice = Timeslice::new();
        timeslice.add_note(note);
        timeslices.push(timeslice);
      }
      StaffContent::Chord(chord) => {
        timeslices.push(chord.borrow().to_timeslice());
      }
      StaffContent::Phrase(phrase) => {
        timeslices.append(&mut phrase.borrow().iter_timeslices());
      }
      StaffContent::MultiVoice(multivoice) => {
        timeslices.append(&mut multivoice.borrow().iter_timeslices());
      }
      StaffContent::Direction(direction) => {
        let mut timeslice = Timeslice::new();
        timeslice.add_direction(direction);
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

impl core::fmt::Display for Staff {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let items = self
      .content
      .iter()
      .map(|item| match item {
        StaffContent::Note(note) => note.borrow().to_string(),
        StaffContent::Chord(chord) => chord.borrow().to_string(),
        StaffContent::Phrase(phrase) => phrase.borrow().to_string(),
        StaffContent::MultiVoice(multi_voice) => multi_voice.borrow().to_string(),
        StaffContent::Direction(direction) => direction.borrow().get_modification().to_string(),
      })
      .collect::<Vec<_>>()
      .join(", ");
    write!(f, "Staff {}: [{items}]", self.name)
  }
}
