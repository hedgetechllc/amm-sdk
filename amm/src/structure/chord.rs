use crate::context::{generate_id, Tempo};
use crate::modification::{ChordModification, ChordModificationType, NoteModification};
use crate::note::{Accidental, Duration, Note, Pitch};
use crate::temporal::Timeslice;
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};

#[derive(Clone, Debug, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum ChordContent {
  Note(Note),
}

#[derive(Debug, Default, Eq, JsonDeserialize, JsonSerialize)]
pub struct Chord {
  id: usize,
  content: Vec<ChordContent>,
  modifications: BTreeSet<ChordModification>,
}

impl Chord {
  #[must_use]
  pub fn new() -> Self {
    Self {
      id: generate_id(),
      content: Vec::new(),
      modifications: BTreeSet::new(),
    }
  }

  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn add_note(&mut self, pitch: Pitch, duration: Duration, accidental: Option<Accidental>) -> &mut Note {
    self.content.retain(|ChordContent::Note(note)| {
      note.pitch != pitch || note.duration != duration || note.accidental != accidental.unwrap_or_default()
    });
    self
      .content
      .push(ChordContent::Note(Note::new(pitch, duration, accidental)));
    match self.content.last_mut() {
      Some(ChordContent::Note(note)) => note,
      None => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn add_modification(&mut self, mod_type: ChordModificationType) -> usize {
    let modification = ChordModification::new(mod_type);
    let modification_id = modification.get_id();
    self.modifications.replace(modification);
    modification_id
  }

  pub fn claim_note(&mut self, note: Note) -> &mut Note {
    self.content.retain(|ChordContent::Note(old_note)| {
      note.pitch != old_note.pitch || note.duration != old_note.duration || note.accidental != old_note.accidental
    });
    self.content.push(ChordContent::Note(note));
    match self.content.last_mut() {
      Some(ChordContent::Note(note)) => note,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  #[must_use]
  pub fn get_note(&self, id: usize) -> Option<&Note> {
    self
      .iter()
      .find_map(|ChordContent::Note(note)| if note.get_id() == id { Some(note) } else { None })
  }

  #[must_use]
  pub fn get_note_mut(&mut self, id: usize) -> Option<&mut Note> {
    self
      .iter_mut()
      .find_map(|ChordContent::Note(note)| if note.get_id() == id { Some(note) } else { None })
  }

  #[must_use]
  pub fn get_modification(&self, id: usize) -> Option<&ChordModification> {
    self
      .iter_modifications()
      .find(|modification| modification.get_id() == id)
  }

  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration, tuplet_ratio: Option<f64>) -> f64 {
    self
      .iter()
      .map(|ChordContent::Note(note)| note.get_beats(beat_base, tuplet_ratio))
      .reduce(f64::min)
      .unwrap_or_default()
  }

  #[must_use]
  pub fn get_duration(&self, tempo: &Tempo, tuplet_ratio: Option<f64>) -> f64 {
    self.get_beats(&tempo.base_note, tuplet_ratio) * 60.0 / f64::from(tempo.beats_per_minute)
  }

  pub fn remove_item(&mut self, id: usize) -> &mut Self {
    self.content.retain(|ChordContent::Note(note)| note.get_id() != id);
    self
  }

  pub fn remove_modification(&mut self, id: usize) -> &mut Self {
    self.modifications.retain(|modification| modification.get_id() != id);
    self.iter_mut().for_each(|ChordContent::Note(note)| {
      note.remove_modification(id);
    });
    self
  }

  pub fn iter(&self) -> core::slice::Iter<'_, ChordContent> {
    self.content.iter()
  }

  pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, ChordContent> {
    self.content.iter_mut()
  }

  pub fn iter_modifications(&self) -> alloc::collections::btree_set::Iter<'_, ChordModification> {
    self.modifications.iter()
  }

  #[must_use]
  pub fn to_timeslice(&self) -> Timeslice {
    let mut timeslice = Timeslice::new();
    timeslice.arpeggiated = self
      .iter_modifications()
      .any(|modification| modification.r#type == ChordModificationType::Arpeggiate);
    let transferrable_modifications = self
      .iter_modifications()
      .filter_map(|modification| NoteModification::from_chord_modification(&modification.r#type))
      .collect::<Vec<_>>();
    self.iter().for_each(|ChordContent::Note(note)| {
      let mut chord_note = note.clone();
      for modification in &transferrable_modifications {
        chord_note.add_modification(modification.r#type);
      }
      timeslice.add_note(chord_note);
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
  type IntoIter = core::slice::Iter<'a, ChordContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut Chord {
  type Item = &'a mut ChordContent;
  type IntoIter = core::slice::IterMut<'a, ChordContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
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
      .iter_modifications()
      .map(ToString::to_string)
      .collect::<Vec<String>>()
      .join(", ");
    let notes = self
      .iter()
      .map(|ChordContent::Note(note)| note.to_string())
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
