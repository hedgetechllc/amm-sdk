use super::note::Note;
use crate::context::{generate_id, Tempo};
use crate::modification::{ChordModification, ChordModificationType};
use crate::note::{Accidental, Duration, Pitch};
use std::{cell::RefCell, rc::Rc, slice::Iter};

pub enum ChordContent {
  Note(Rc<RefCell<Note>>),
}

pub struct Chord {
  id: usize,
  content: Vec<ChordContent>,
  modifications: Vec<Rc<RefCell<ChordModification>>>,
}

impl Chord {
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
    self.content.push(ChordContent::Note(Rc::clone(&note)));
    note
  }

  pub fn add_modification(&mut self, modification: ChordModificationType) -> Rc<RefCell<ChordModification>> {
    let modification = ChordModification::new(modification);
    self.modifications.push(Rc::clone(&modification));
    modification
  }

  pub fn get_note(&mut self, id: usize) -> Option<Rc<RefCell<Note>>> {
    self.content.iter().find_map(|item| match item {
      ChordContent::Note(note) if note.borrow().get_id() == id => Some(Rc::clone(note)),
      _ => None,
    })
  }

  pub fn get_modification(&mut self, id: usize) -> Option<Rc<RefCell<ChordModification>>> {
    self.modifications.iter().find_map(|modification| {
      if modification.borrow().get_id() == id {
        Some(Rc::clone(modification))
      } else {
        None
      }
    })
  }

  pub fn remove_item(&mut self, id: usize) -> &mut Self {
    self.content.retain(|item| match item {
      ChordContent::Note(note) => note.borrow().get_id() != id,
    });
    self
  }

  pub fn remove_modification(&mut self, id: usize) -> &mut Self {
    self
      .modifications
      .retain(|modification| modification.borrow().get_id() != id);
    self
  }

  pub fn get_duration(&self, tempo: &Tempo, tuplet_ratio: Option<f64>) -> f64 {
    self
      .content
      .iter()
      .map(|content| match &content {
        ChordContent::Note(note) => note.borrow().get_duration(&tempo, tuplet_ratio),
      })
      .reduce(f64::min)
      .unwrap_or_default()
  }

  pub fn iter(&self) -> Iter<'_, ChordContent> {
    self.content.iter()
  }
}

impl std::fmt::Display for Chord {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mods = self
      .modifications
      .iter()
      .map(|modification| modification.borrow_mut().to_string())
      .collect::<Vec<String>>()
      .join(", ");
    let notes = self
      .content
      .iter()
      .map(|item| match &item {
        ChordContent::Note(note) => note.borrow().to_string(),
      })
      .collect::<Vec<_>>()
      .join(", ");
    write!(
      f,
      "Chord{}: [{}]",
      if mods.is_empty() {
        String::new()
      } else {
        format!(" ({})", mods)
      },
      notes
    )
  }
}

impl<'a> IntoIterator for &'a Chord {
  type Item = <Iter<'a, ChordContent> as Iterator>::Item;
  type IntoIter = Iter<'a, ChordContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.content.as_slice().into_iter()
  }
}
