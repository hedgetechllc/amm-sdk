use crate::modification::SliceModification;
use crate::note::{Accidental, DisplayOptions, Duration, Note, Pitch};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Default)]
pub struct MusicalSlice {
  notes: Vec<Rc<RefCell<Note>>>,
  modifications: Vec<SliceModification>,
}

impl MusicalSlice {
  pub fn duration(&self) -> f64 {
    0.0 // TODO: IMPLEMENT THIS, TAKING INTO ACCOUNT MODS AND NOTE DURATIONS
  }

  pub fn add_note(
    &mut self,
    pitch: Pitch,
    duration: Duration,
    accidental: Option<Accidental>,
    display: Option<DisplayOptions>,
  ) -> &Rc<RefCell<Note>> {
    let note = Note::new(pitch, duration, accidental, display);
    self.remove_note(&note).notes.push(Rc::new(RefCell::new(note)));
    self.notes.last_mut().unwrap()
  }

  pub fn add_modification(&mut self, modification: SliceModification) -> &mut Self {
    self.remove_modification(&modification).modifications.push(modification);
    self
  }

  pub fn remove_note(&mut self, note: &Note) -> &mut Self {
    self.notes.retain(|value| !value.borrow().is_same_pitch(note));
    self
  }

  pub fn remove_modification(&mut self, modification: &SliceModification) -> &mut Self {
    self
      .modifications
      .retain(|item| std::mem::discriminant(item) != std::mem::discriminant(modification));
    self
  }
}

impl std::fmt::Display for MusicalSlice {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Musical Slice ({} notes)", self.notes.len())
  }
}
