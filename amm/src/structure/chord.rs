use super::timeslice::Timeslice;
use crate::context::{generate_id, Tempo};
use crate::modification::{ChordModification, ChordModificationType, NoteModification};
use crate::note::{Accidental, Duration, Note, Pitch};
use alloc::{
  rc::Rc,
  string::{String, ToString},
  vec::Vec,
};
use core::{cell::RefCell, slice::Iter};
#[cfg(feature = "json")]
use {
  amm_internal::json_prelude::*,
  amm_macros::{JsonDeserialize, JsonSerialize},
};

#[cfg_attr(feature = "json", derive(JsonDeserialize, JsonSerialize))]
#[derive(Clone, Debug)]
pub enum ChordContent {
  Note(Rc<RefCell<Note>>),
}

#[cfg_attr(feature = "json", derive(JsonDeserialize, JsonSerialize))]
#[derive(Clone, Debug, Default)]
pub struct Chord {
  pub(crate) id: usize,
  pub(crate) content: Vec<ChordContent>,
  pub(crate) modifications: Vec<Rc<RefCell<ChordModification>>>,
}

impl Chord {
  #[must_use]
  pub fn new() -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: generate_id(),
      content: Vec::new(),
      modifications: Vec::new(),
    }))
  }

  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn add_note(&mut self, pitch: Pitch, duration: Duration, accidental: Option<Accidental>) -> Rc<RefCell<Note>> {
    let note = Note::new(pitch, duration, accidental);
    self.content.push(ChordContent::Note(Rc::clone(&note)));
    note
  }

  pub fn add_modification(&mut self, modification: ChordModificationType) -> Rc<RefCell<ChordModification>> {
    self
      .modifications
      .retain(|mods| mods.borrow().r#type != modification);
    let modification = ChordModification::new(modification);
    self.modifications.push(Rc::clone(&modification));
    modification
  }

  #[must_use]
  pub fn get_note(&self, id: usize) -> Option<Rc<RefCell<Note>>> {
    self.content.iter().find_map(|item| match item {
      ChordContent::Note(note) if note.borrow().get_id() == id => Some(Rc::clone(note)),
      ChordContent::Note(_) => None,
    })
  }

  #[must_use]
  pub fn get_modification(&self, id: usize) -> Option<Rc<RefCell<ChordModification>>> {
    self.modifications.iter().find_map(|modification| {
      if modification.borrow().get_id() == id {
        Some(Rc::clone(modification))
      } else {
        None
      }
    })
  }

  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration, tuplet_ratio: Option<f64>) -> f64 {
    self
      .content
      .iter()
      .map(|content| match &content {
        ChordContent::Note(note) => note.borrow().get_beats(beat_base, tuplet_ratio),
      })
      .reduce(f64::min)
      .unwrap_or_default()
  }

  #[must_use]
  pub fn get_duration(&self, tempo: &Tempo, tuplet_ratio: Option<f64>) -> f64 {
    self.get_beats(&tempo.base_note, tuplet_ratio) * 60.0 / f64::from(tempo.beats_per_minute)
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

  pub fn iter(&self) -> Iter<'_, ChordContent> {
    self.content.iter()
  }

  #[must_use]
  pub fn iter_modifications(&self) -> Iter<'_, Rc<RefCell<ChordModification>>> {
    self.modifications.iter()
  }

  #[must_use]
  pub fn to_timeslice(&self) -> Timeslice {
    let mut timeslice = Timeslice::new();
    timeslice.arpeggiated = self.modifications.iter().any(|modification| {
      matches!(
        modification.borrow().r#type,
        ChordModificationType::Arpeggiate
      )
    });
    let transferrable_modifications = self
      .modifications
      .iter()
      .filter_map(|modification| NoteModification::from_chord_modification(&modification.borrow().r#type))
      .collect::<Vec<_>>();
    self.content.iter().for_each(|item| match item {
      ChordContent::Note(note) => {
        let mut chord_note = note.borrow().clone();
        for modification in &transferrable_modifications {
          chord_note
            .modifications
            .retain(|chord_mod| chord_mod.borrow().r#type != modification.borrow().r#type);
          chord_note.modifications.push(Rc::clone(modification));
        }
        timeslice.add_note(&Rc::new(RefCell::new(chord_note)));
      }
    });
    timeslice
  }
}

impl IntoIterator for Chord {
  type Item = ChordContent;
  type IntoIter = alloc::vec::IntoIter<Self::Item>;
  fn into_iter(self) -> Self::IntoIter {
    self.content.into_iter()
  }
}

impl<'a> IntoIterator for &'a Chord {
  type Item = &'a ChordContent;
  type IntoIter = Iter<'a, ChordContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Chord {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let mods = self
      .modifications
      .iter()
      .map(|modification| modification.borrow().to_string())
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
      "Chord{}: [{notes}]",
      if mods.is_empty() {
        String::new()
      } else {
        format!(" ({mods})")
      }
    )
  }
}
