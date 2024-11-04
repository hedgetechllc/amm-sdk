use crate::context::{generate_id, Tempo};
use crate::modification::{NoteModification, NoteModificationType};
use crate::note::{Accidental, Duration, Note, Pitch};
use crate::temporal::Timeslice;
use amm_internal::amm_prelude::*;

impl Note {
  #[must_use]
  pub fn new(pitch: Pitch, duration: Duration, accidental: Option<Accidental>) -> Self {
    Self {
      id: generate_id(),
      pitch,
      duration,
      accidental: accidental.unwrap_or_default(),
      modifications: BTreeSet::new(),
    }
  }

  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn add_modification(&mut self, mod_type: NoteModificationType) -> usize {
    let modification = NoteModification::new(mod_type);
    let modification_id = modification.get_id();
    self.modifications.replace(modification);
    modification_id
  }

  #[must_use]
  pub fn get_modification(&self, id: usize) -> Option<&NoteModification> {
    self
      .iter_modifications()
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

  pub fn iter_modifications(&self) -> alloc::collections::btree_set::Iter<'_, NoteModification> {
    self.modifications.iter()
  }

  #[must_use]
  pub fn to_timeslice(&self) -> Timeslice {
    let mut timeslice = Timeslice::new();
    timeslice.add_note(self.clone());
    timeslice
  }
}
