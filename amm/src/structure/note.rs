use crate::context::{generate_id, Tempo};
use crate::modification::{NoteModification, NoteModificationType};
use crate::note::{Accidental, Duration, Note, Pitch};
use alloc::vec::Vec;
use core::slice::{Iter, IterMut};

impl Note {
  #[must_use]
  pub fn new(pitch: Pitch, duration: Duration, accidental: Option<Accidental>) -> Self {
    Self {
      id: generate_id(),
      pitch,
      duration,
      accidental: accidental.unwrap_or_default(),
      modifications: Vec::new(),
    }
  }

  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn add_modification(&mut self, mod_type: NoteModificationType) -> &mut NoteModification {
    self.modifications.retain(|mods| mods.r#type != mod_type);
    self.modifications.push(NoteModification::new(mod_type));
    unsafe { self.modifications.last_mut().unwrap_unchecked() }
  }

  #[must_use]
  pub fn get_modification(&self, id: usize) -> Option<&NoteModification> {
    self
      .modifications
      .iter()
      .find(|modification| modification.get_id() == id)
  }

  #[must_use]
  pub fn get_modification_mut(&mut self, id: usize) -> Option<&mut NoteModification> {
    self
      .modifications
      .iter_mut()
      .find(|modification| modification.get_id() == id)
  }

  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration, tuplet_ratio: Option<f64>) -> f64 {
    self.beats(beat_base.value()) * tuplet_ratio.unwrap_or(1.0)
  }

  #[must_use]
  pub fn get_duration(&self, tempo: &Tempo, tuplet_ratio: Option<f64>) -> f64 {
    self.get_beats(&tempo.base_note, tuplet_ratio) * 60.0 / f64::from(tempo.beats_per_minute)
  }

  pub fn remove_modification(&mut self, id: usize) -> &mut Self {
    self.modifications.retain(|modification| modification.get_id() != id);
    self
  }

  pub fn iter_modifications(&self) -> Iter<'_, NoteModification> {
    self.modifications.iter()
  }

  pub fn iter_modifications_mut(&mut self) -> IterMut<'_, NoteModification> {
    self.modifications.iter_mut()
  }
}
