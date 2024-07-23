use super::chord::Chord;
use super::note::Note;
use crate::context::generate_id;
use crate::modification::{PhraseModification, PhraseModificationType};
use crate::note::{Accidental, Duration, Pitch};
use std::{cell::RefCell, rc::Rc, slice::Iter};

pub enum PhraseContent {
  Note(Rc<RefCell<Note>>),
  Chord(Rc<RefCell<Chord>>),
  Phrase(Rc<RefCell<Phrase>>),
}

pub struct Phrase {
  id: usize,
  content: Vec<PhraseContent>,
  modifications: Vec<Rc<RefCell<PhraseModification>>>,
}

impl Phrase {
  pub fn new() -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: generate_id(),
      content: Vec::new(),
      modifications: Vec::new(),
    }))
  }

  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn add_note(&mut self, pitch: Pitch, duration: Duration, accidental: Option<Accidental>) -> Rc<RefCell<Note>> {
    let note = Note::new(pitch, duration, accidental);
    self.content.push(PhraseContent::Note(Rc::clone(&note)));
    note
  }

  pub fn add_chord(&mut self) -> Rc<RefCell<Chord>> {
    let chord = Chord::new();
    self.content.push(PhraseContent::Chord(Rc::clone(&chord)));
    chord
  }

  pub fn add_phrase(&mut self) -> Rc<RefCell<Phrase>> {
    let phrase = Phrase::new();
    self.content.push(PhraseContent::Phrase(Rc::clone(&phrase)));
    phrase
  }

  pub fn add_modification(&mut self, modification: PhraseModificationType) -> Rc<RefCell<PhraseModification>> {
    let modification = PhraseModification::new(modification);
    self.modifications.push(Rc::clone(&modification));
    modification
  }

  pub fn insert_note(
    &mut self,
    index: usize,
    pitch: Pitch,
    duration: Duration,
    accidental: Option<Accidental>,
  ) -> Rc<RefCell<Note>> {
    let note = Note::new(pitch, duration, accidental);
    self.content.insert(index, PhraseContent::Note(Rc::clone(&note)));
    note
  }

  pub fn insert_chord(&mut self, index: usize) -> Rc<RefCell<Chord>> {
    let chord = Chord::new();
    self.content.insert(index, PhraseContent::Chord(Rc::clone(&chord)));
    chord
  }

  pub fn insert_phrase(&mut self, index: usize) -> Rc<RefCell<Phrase>> {
    let phrase = Phrase::new();
    self.content.insert(index, PhraseContent::Phrase(Rc::clone(&phrase)));
    phrase
  }

  pub fn get_note(&mut self, id: usize) -> Option<Rc<RefCell<Note>>> {
    self.content.iter().find_map(|item| match item {
      PhraseContent::Note(note) if note.borrow().get_id() == id => Some(Rc::clone(note)),
      _ => None,
    })
  }

  pub fn get_chord(&mut self, id: usize) -> Option<Rc<RefCell<Chord>>> {
    self.content.iter().find_map(|item| match item {
      PhraseContent::Chord(chord) if chord.borrow().get_id() == id => Some(Rc::clone(chord)),
      _ => None,
    })
  }

  pub fn get_phrase(&mut self, id: usize) -> Option<Rc<RefCell<Phrase>>> {
    self.content.iter().find_map(|item| match item {
      PhraseContent::Phrase(phrase) if phrase.borrow().get_id() == id => Some(Rc::clone(phrase)),
      _ => None,
    })
  }

  pub fn get_modification(&mut self, id: usize) -> Option<Rc<RefCell<PhraseModification>>> {
    self.modifications.iter().find_map(|modification| {
      if modification.borrow().get_id() == id {
        Some(Rc::clone(modification))
      } else {
        None
      }
    })
  }

  pub fn get_index_of_item(&mut self, id: usize) -> Option<usize> {
    self.content.iter().position(|item| match item {
      PhraseContent::Note(note) => note.borrow().get_id() == id,
      PhraseContent::Chord(chord) => chord.borrow().get_id() == id,
      PhraseContent::Phrase(phrase) => phrase.borrow().get_id() == id,
    })
  }

  pub fn remove_item(&mut self, id: usize) -> &mut Self {
    self.content.retain(|item| match item {
      PhraseContent::Note(note) => note.borrow().get_id() != id,
      PhraseContent::Chord(chord) => chord.borrow().get_id() != id,
      PhraseContent::Phrase(phrase) => phrase.borrow().get_id() != id,
    });
    self
  }

  pub fn remove_modification(&mut self, id: usize) -> &mut Self {
    self
      .modifications
      .retain(|modification| modification.borrow().get_id() != id);
    self
  }

  pub fn iter(&self) -> Iter<'_, PhraseContent> {
    self.content.iter()
  }
}

impl std::fmt::Display for Phrase {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let items = self
      .content
      .iter()
      .map(|item| match item {
        PhraseContent::Note(note) => note.borrow().to_string(),
        PhraseContent::Chord(chord) => chord.borrow().to_string(),
        PhraseContent::Phrase(phrase) => phrase.borrow().to_string(),
      })
      .collect::<Vec<_>>()
      .join(", ");
    write!(f, "Phrase: [{}]", items)
  }
}

impl<'a> IntoIterator for &'a Phrase {
  type Item = <Iter<'a, PhraseContent> as Iterator>::Item;
  type IntoIter = Iter<'a, PhraseContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.content.as_slice().into_iter()
  }
}
