use super::timeslice::Timeslice;
use crate::context::{generate_id, Tempo};
use crate::modification::{ChordModification, ChordModificationType, NoteModification};
use crate::note::{Accidental, Duration, Note, Pitch};
use crate::util::{MutSet, MutSetRef, MapRef, MappedRef};
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};
use core::slice::{Iter, IterMut};

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum ChordContent {
  Note(Note),
}

#[derive(Debug, Default, Eq, JsonDeserialize, JsonSerialize)]
pub struct Chord {
  id: usize,
  content: BTreeSet<ChordContent>,
  modifications: Vec<ChordModification>,
}

impl Chord {
  #[must_use]
  pub fn new() -> Self {
    Self {
      id: generate_id(),
      content: Default::default(),
      modifications: Default::default(),
    }
  }

  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }

  fn map_ref_note(v: MutSetRef<ChordContent>) -> MappedRef<MutSetRef<ChordContent>, Note> {
    v.map_ref(
      |x| match x {
        ChordContent::Note(note) => note,
      },
      |x| match x {
        ChordContent::Note(note) => note,
      }
    )
  }

  pub fn add_note(&mut self, pitch: Pitch, duration: Duration, accidental: Option<Accidental>) -> MappedRef<MutSetRef<ChordContent>, Note> {
    let note = ChordContent::Note(Note::new(pitch, duration, accidental));
    self.content.insert(note.clone());
    Self::map_ref_note(self.content.get_mut(&note).unwrap())
  }

  pub fn add_modification(&mut self, mod_type: ChordModificationType) -> &mut ChordModification {
    self.modifications.retain(|mods| mods.r#type != mod_type);
    self.modifications.push(ChordModification::new(mod_type));
    unsafe { self.modifications.last_mut().unwrap_unchecked() }
  }

  pub fn claim_note(&mut self, note: Note) -> MappedRef<MutSetRef<ChordContent>, Note> {
    self.content.retain(|item| match item {
      ChordContent::Note(old_note) => {
        note.pitch != old_note.pitch || note.duration != old_note.duration || note.accidental != old_note.accidental
      }
    });
    let note = ChordContent::Note(note);
    self.content.insert(note.clone());
    Self::map_ref_note(self.content.get_mut(&note).unwrap())
  }

  #[must_use]
  pub fn get_note(&self, id: usize) -> Option<&Note> {
    self.content.iter().find_map(|item| match item {
      ChordContent::Note(note) if note.get_id() == id => Some(note),
      ChordContent::Note(_) => None,
    })
  }

  #[must_use]
  pub fn get_note_mut(&mut self, id: usize) -> Option<MappedRef<MutSetRef<ChordContent>, Note>> {
    let note = self.content.iter().find(|v| match v {
      ChordContent::Note(note) => note.get_id() == id,
    })?.clone();
    Some(Self::map_ref_note(self.content.get_mut(&note).unwrap()))
  }

  #[must_use]
  pub fn get_modification(&self, id: usize) -> Option<&ChordModification> {
    self
      .modifications
      .iter()
      .find(|modification| modification.get_id() == id)
  }

  #[must_use]
  pub fn get_modification_mut(&mut self, id: usize) -> Option<&mut ChordModification> {
    self
      .modifications
      .iter_mut()
      .find(|modification| modification.get_id() == id)
  }

  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration, tuplet_ratio: Option<f64>) -> f64 {
    self
      .content
      .iter()
      .map(|content| match content {
        ChordContent::Note(note) => note.get_beats(beat_base, tuplet_ratio),
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
      ChordContent::Note(note) => note.get_id() != id,
    });
    self
  }

  pub fn remove_modification(&mut self, id: usize) -> &mut Self {
    self.modifications.retain(|modification| modification.get_id() != id);
    self
  }

  pub fn iter(&self) -> alloc::collections::btree_set::Iter<ChordContent> {
    self.content.iter()
  }

  pub fn iter_modifications(&self) -> Iter<'_, ChordModification> {
    self.modifications.iter()
  }

  pub fn iter_modifications_mut(&mut self) -> IterMut<'_, ChordModification> {
    self.modifications.iter_mut()
  }

  #[must_use]
  pub fn to_timeslice(&self) -> Timeslice {
    let mut timeslice = Timeslice::new();
    timeslice.arpeggiated = self
      .modifications
      .iter()
      .any(|modification| modification.r#type == ChordModificationType::Arpeggiate);
    let transferrable_modifications = self
      .modifications
      .iter()
      .filter_map(|modification| NoteModification::from_chord_modification(&modification.r#type))
      .collect::<Vec<_>>();
    self.content.iter().for_each(|item| match item {
      ChordContent::Note(note) => {
        let mut chord_note = note.clone();
        for modification in &transferrable_modifications {
          chord_note.add_modification(modification.r#type);
        }
        timeslice.add_note(chord_note);
      }
    });
    timeslice
  }
}

impl IntoIterator for Chord {
  type Item = ChordContent;
  type IntoIter = alloc::collections::btree_set::IntoIter<Self::Item>;
  fn into_iter(self) -> Self::IntoIter {
    self.content.into_iter()
  }
}

impl<'a> IntoIterator for &'a Chord {
  type Item = &'a ChordContent;
  type IntoIter = alloc::collections::btree_set::Iter<'a, ChordContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl Clone for Chord {
  fn clone(&self) -> Self {
    Self {
      id: generate_id(),
      content: self.content.clone(),
      modifications: self.modifications.clone(),
    }
  }
}

impl PartialEq for Chord {
  fn eq(&self, other: &Self) -> bool {
    self.content == other.content && self.modifications == other.modifications
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Chord {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let mods = self
      .modifications
      .iter()
      .map(ToString::to_string)
      .collect::<Vec<String>>()
      .join(", ");
    let notes = self
      .content
      .iter()
      .map(|item| match &item {
        ChordContent::Note(note) => note.to_string(),
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
