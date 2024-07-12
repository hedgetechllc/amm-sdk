use crate::context::generate_id;
use crate::modification::{NoteModification, NoteModificationType};
use crate::note::{Accidental, Duration, Pitch};
use std::{cell::RefCell, rc::Rc};

pub use crate::note::Note;

impl Note {
  pub fn new(pitch: Pitch, duration: Duration, accidental: Option<Accidental>) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: generate_id(),
      pitch,
      duration,
      accidental: accidental.unwrap_or_default(),
      modifications: Vec::new(),
    }))
  }

  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn add_modification(&mut self, modification: NoteModificationType) -> Rc<RefCell<NoteModification>> {
    let modification = NoteModification::new(modification);
    self.modifications.push(Rc::clone(&modification));
    modification
  }

  pub fn get_modification(&mut self, id: usize) -> Option<Rc<RefCell<NoteModification>>> {
    self.modifications.iter().find_map(|modification| {
      if modification.borrow().get_id() == id {
        Some(Rc::clone(modification))
      } else {
        None
      }
    })
  }

  pub fn remove_modification(&mut self, id: usize) -> &mut Self {
    self
      .modifications
      .retain(|modification| modification.borrow().get_id() != id);
    self
  }
}
