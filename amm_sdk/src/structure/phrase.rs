use super::{
  chord::Chord,
  multivoice::{MultiVoice, MultiVoiceTimesliceIter},
};
use crate::context::{generate_id, Tempo};
use crate::modification::{PhraseModification, PhraseModificationType};
use crate::note::{Accidental, Duration, Note, Pitch};
use crate::temporal::Timeslice;
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};

#[derive(Clone, Debug, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum PhraseContent {
  Note(Note),
  Chord(Chord),
  Phrase(Phrase),
  MultiVoice(MultiVoice),
}

#[derive(Debug, Default, Eq, JsonDeserialize, JsonSerialize)]
pub struct Phrase {
  id: usize,
  pub(crate) content: Vec<PhraseContent>,
  modifications: BTreeSet<PhraseModification>,
}

impl Phrase {
  #[must_use]
  pub fn new() -> Self {
    Self {
      id: generate_id(),
      content: Vec::new(),
      modifications: BTreeSet::new(),
    }
  }

  pub(crate) fn simplify(&mut self) {
    let mut content_changed = true;
    while content_changed {
      let mut content_to_edit = Vec::new();
      self.iter_mut().enumerate().for_each(|(idx, item)| match item {
        PhraseContent::Phrase(phrase) => {
          phrase.simplify();
          if phrase.content.is_empty() {
            content_to_edit.push((idx, None));
          } else if phrase.modifications.is_empty() {
            content_to_edit.push((idx, Some(core::mem::take(&mut phrase.content))));
          }
        }
        PhraseContent::MultiVoice(multivoice) => {
          let single_phrase = multivoice
            .simplify()
            .map(|phrase| Vec::from([PhraseContent::Phrase(phrase)]));
          if multivoice.is_empty() {
            content_to_edit.push((idx, single_phrase));
          }
        }
        _ => (),
      });
      content_changed = !content_to_edit.is_empty();
      for (idx, content) in content_to_edit.into_iter().rev() {
        if let Some(contents) = content {
          self.content.splice(idx..=idx, contents);
        } else {
          self.content.remove(idx);
        }
      }
    }
  }

  #[must_use]
  pub fn flatten(&self, fully: bool) -> Self {
    // Removes all multivoice layers (flattens multivoices into a single phrase)
    // The "fully" parameter determines whether sub-phrases will also be flattened into a
    // single phrase containing only notes, chords, and tuplets
    let mut flat_phrase = Self {
      id: generate_id(),
      content: Vec::new(),
      modifications: self.modifications.clone(),
    };
    if fully {
      self.iter().for_each(|item| match item {
        PhraseContent::Phrase(phrase) => {
          let mut flattened_phrase = phrase.flatten(true);
          if flattened_phrase.is_tuplet() {
            flat_phrase.content.push(PhraseContent::Phrase(flattened_phrase));
          } else {
            flat_phrase.content.append(&mut flattened_phrase.content);
          }
        }
        PhraseContent::MultiVoice(multivoice) => flat_phrase.content.append(&mut multivoice.flatten().content),
        _ => flat_phrase.content.push(item.clone()),
      });
    } else {
      flat_phrase.content.extend(self.content.iter().map(|item| match item {
        PhraseContent::Phrase(phrase) => PhraseContent::Phrase(phrase.flatten(false)),
        PhraseContent::MultiVoice(multivoice) => PhraseContent::Phrase(multivoice.flatten()),
        _ => item.clone(),
      }));
    }
    flat_phrase
  }

  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn add_note(&mut self, pitch: Pitch, duration: Duration, accidental: Option<Accidental>) -> &mut Note {
    self
      .content
      .push(PhraseContent::Note(Note::new(pitch, duration, accidental)));
    match self.content.last_mut() {
      Some(PhraseContent::Note(note)) => note,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn add_chord(&mut self) -> &mut Chord {
    self.content.push(PhraseContent::Chord(Chord::new()));
    match self.content.last_mut() {
      Some(PhraseContent::Chord(chord)) => chord,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn add_phrase(&mut self) -> &mut Phrase {
    self.content.push(PhraseContent::Phrase(Phrase::new()));
    match self.content.last_mut() {
      Some(PhraseContent::Phrase(phrase)) => phrase,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn add_multivoice(&mut self) -> &mut MultiVoice {
    self.content.push(PhraseContent::MultiVoice(MultiVoice::new()));
    match self.content.last_mut() {
      Some(PhraseContent::MultiVoice(multivoice)) => multivoice,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn add_modification(&mut self, mod_type: PhraseModificationType) -> usize {
    let modification = PhraseModification::new(mod_type);
    let modification_id = modification.get_id();
    self.modifications.replace(modification);
    modification_id
  }

  pub fn claim(&mut self, item: PhraseContent) -> &mut Self {
    self.content.push(item);
    self
  }

  pub fn claim_note(&mut self, note: Note) -> &mut Note {
    self.content.push(PhraseContent::Note(note));
    match self.content.last_mut() {
      Some(PhraseContent::Note(note)) => note,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn claim_chord(&mut self, chord: Chord) -> &mut Chord {
    self.content.push(PhraseContent::Chord(chord));
    match self.content.last_mut() {
      Some(PhraseContent::Chord(chord)) => chord,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn claim_phrase(&mut self, phrase: Phrase) -> &mut Phrase {
    self.content.push(PhraseContent::Phrase(phrase));
    match self.content.last_mut() {
      Some(PhraseContent::Phrase(phrase)) => phrase,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn claim_multivoice(&mut self, multivoice: MultiVoice) -> &mut MultiVoice {
    self.content.push(PhraseContent::MultiVoice(multivoice));
    match self.content.last_mut() {
      Some(PhraseContent::MultiVoice(multivoice)) => multivoice,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn insert_note(
    &mut self,
    index: usize,
    pitch: Pitch,
    duration: Duration,
    accidental: Option<Accidental>,
  ) -> &mut Note {
    self
      .content
      .insert(index, PhraseContent::Note(Note::new(pitch, duration, accidental)));
    match self.content.get_mut(index) {
      Some(PhraseContent::Note(note)) => note,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn insert_chord(&mut self, index: usize) -> &mut Chord {
    self.content.insert(index, PhraseContent::Chord(Chord::new()));
    match self.content.get_mut(index) {
      Some(PhraseContent::Chord(chord)) => chord,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn insert_phrase(&mut self, index: usize) -> &mut Phrase {
    self.content.insert(index, PhraseContent::Phrase(Phrase::new()));
    match self.content.get_mut(index) {
      Some(PhraseContent::Phrase(phrase)) => phrase,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn insert_multivoice(&mut self, index: usize) -> &mut MultiVoice {
    self.content.insert(index, PhraseContent::MultiVoice(MultiVoice::new()));
    match self.content.get_mut(index) {
      Some(PhraseContent::MultiVoice(multivoice)) => multivoice,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  #[must_use]
  pub fn get_note(&self, id: usize) -> Option<&Note> {
    self.iter().find_map(|item| match item {
      PhraseContent::Note(note) if note.get_id() == id => Some(note),
      PhraseContent::Chord(chord) => chord.get_note(id),
      PhraseContent::Phrase(phrase) => phrase.get_note(id),
      PhraseContent::MultiVoice(multivoice) => multivoice.get_note(id),
      PhraseContent::Note(_) => None,
    })
  }

  #[must_use]
  pub fn get_note_mut(&mut self, id: usize) -> Option<&mut Note> {
    self.iter_mut().find_map(|item| match item {
      PhraseContent::Note(note) if note.get_id() == id => Some(note),
      PhraseContent::Chord(chord) => chord.get_note_mut(id),
      PhraseContent::Phrase(phrase) => phrase.get_note_mut(id),
      PhraseContent::MultiVoice(multivoice) => multivoice.get_note_mut(id),
      PhraseContent::Note(_) => None,
    })
  }

  #[must_use]
  pub fn get_chord(&self, id: usize) -> Option<&Chord> {
    self.iter().find_map(|item| match item {
      PhraseContent::Chord(chord) if chord.get_id() == id => Some(chord),
      PhraseContent::Phrase(phrase) => phrase.get_chord(id),
      PhraseContent::MultiVoice(multivoice) => multivoice.get_chord(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_chord_mut(&mut self, id: usize) -> Option<&mut Chord> {
    self.iter_mut().find_map(|item| match item {
      PhraseContent::Chord(chord) if chord.get_id() == id => Some(chord),
      PhraseContent::Phrase(phrase) => phrase.get_chord_mut(id),
      PhraseContent::MultiVoice(multivoice) => multivoice.get_chord_mut(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_phrase(&self, id: usize) -> Option<&Phrase> {
    if self.id == id {
      Some(self)
    } else {
      self.iter().find_map(|item| match item {
        PhraseContent::Phrase(phrase) => phrase.get_phrase(id),
        PhraseContent::MultiVoice(multivoice) => multivoice.get_phrase(id),
        _ => None,
      })
    }
  }

  #[must_use]
  pub fn get_phrase_mut(&mut self, id: usize) -> Option<&mut Phrase> {
    if self.id == id {
      Some(self)
    } else {
      self.iter_mut().find_map(|item| match item {
        PhraseContent::Phrase(phrase) => phrase.get_phrase_mut(id),
        PhraseContent::MultiVoice(multivoice) => multivoice.get_phrase_mut(id),
        _ => None,
      })
    }
  }

  #[must_use]
  pub fn get_multivoice(&self, id: usize) -> Option<&MultiVoice> {
    self.iter().find_map(|item| match item {
      PhraseContent::MultiVoice(multivoice) => multivoice.get_multivoice(id),
      PhraseContent::Phrase(phrase) => phrase.get_multivoice(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_multivoice_mut(&mut self, id: usize) -> Option<&mut MultiVoice> {
    self.iter_mut().find_map(|item| match item {
      PhraseContent::MultiVoice(multivoice) => multivoice.get_multivoice_mut(id),
      PhraseContent::Phrase(phrase) => phrase.get_multivoice_mut(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_modification(&self, id: usize) -> Option<&PhraseModification> {
    self
      .iter_modifications()
      .find(|modification| modification.get_id() == id)
  }

  #[must_use]
  pub fn get_index_of_item(&self, id: usize) -> Option<usize> {
    self.content.iter().position(|item| match item {
      PhraseContent::Note(note) => note.get_id() == id,
      PhraseContent::Chord(chord) => chord.get_id() == id,
      PhraseContent::Phrase(phrase) => phrase.get_id() == id,
      PhraseContent::MultiVoice(multivoice) => multivoice.get_id() == id,
    })
  }

  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration, tuplet_ratio: Option<f64>) -> f64 {
    // Determine if this phrase creates a tuplet
    let new_tuplet_ratio = self.iter_modifications().find_map(|item| match item.r#type {
      PhraseModificationType::Tuplet { num_beats, into_beats } => Some(f64::from(into_beats) / f64::from(num_beats)),
      _ => None,
    });
    let tuplet_ratio = match tuplet_ratio {
      Some(ratio) => match new_tuplet_ratio {
        Some(new_ratio) => Some(ratio * new_ratio),
        None => Some(ratio),
      },
      None => new_tuplet_ratio,
    };

    // Calculate the sum of all phrase component durations
    self
      .iter()
      .map(|content| match &content {
        PhraseContent::Note(note) => note.get_beats(beat_base, tuplet_ratio),
        PhraseContent::Chord(chord) => chord.get_beats(beat_base, tuplet_ratio),
        PhraseContent::Phrase(phrase) => phrase.get_beats(beat_base, tuplet_ratio),
        PhraseContent::MultiVoice(multivoice) => multivoice.get_beats(beat_base, tuplet_ratio),
      })
      .sum()
  }

  #[must_use]
  pub fn get_duration(&self, tempo: &Tempo, tuplet_ratio: Option<f64>) -> f64 {
    self.get_beats(&tempo.base_note, tuplet_ratio) * 60.0 / f64::from(tempo.beats_per_minute)
  }

  pub fn remove_item(&mut self, id: usize) -> &mut Self {
    self.content.retain(|item| match item {
      PhraseContent::Note(note) => note.get_id() != id,
      PhraseContent::Chord(chord) => chord.get_id() != id,
      PhraseContent::Phrase(phrase) => phrase.get_id() != id,
      PhraseContent::MultiVoice(multivoice) => multivoice.get_id() != id,
    });
    self.iter_mut().for_each(|item| match item {
      PhraseContent::Chord(chord) => {
        chord.remove_item(id);
      }
      PhraseContent::Phrase(phrase) => {
        phrase.remove_item(id);
      }
      PhraseContent::MultiVoice(multivoice) => {
        multivoice.remove_item(id);
      }
      PhraseContent::Note(_) => (),
    });
    self
  }

  pub fn remove_modification(&mut self, id: usize) -> &mut Self {
    self.modifications.retain(|modification| modification.get_id() != id);
    self.iter_mut().for_each(|item| match item {
      PhraseContent::Note(note) => {
        note.remove_modification(id);
      }
      PhraseContent::Chord(chord) => {
        chord.remove_modification(id);
      }
      PhraseContent::Phrase(phrase) => {
        phrase.remove_modification(id);
      }
      PhraseContent::MultiVoice(multivoice) => {
        multivoice.remove_modification(id);
      }
    });
    self
  }

  #[must_use]
  pub fn is_tuplet(&self) -> bool {
    self
      .iter_modifications()
      .any(|modification| matches!(modification.r#type, PhraseModificationType::Tuplet { .. }))
  }

  #[must_use]
  pub fn is_nested_tuplet(&self) -> bool {
    self.is_tuplet()
      && self.iter().any(|item| match item {
        PhraseContent::Phrase(phrase) => phrase.is_tuplet(),
        _ => false,
      })
  }

  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.content.is_empty()
  }

  #[must_use]
  pub fn num_items(&self) -> usize {
    self.content.len()
  }

  #[must_use]
  pub fn num_timeslices(&self) -> usize {
    self
      .iter()
      .map(|item| match item {
        PhraseContent::Note(_) | PhraseContent::Chord(_) => 1,
        PhraseContent::Phrase(phrase) => phrase.num_timeslices(),
        PhraseContent::MultiVoice(multivoice) => multivoice.num_timeslices(),
      })
      .sum()
  }

  pub fn iter(&self) -> core::slice::Iter<'_, PhraseContent> {
    self.content.iter()
  }

  pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, PhraseContent> {
    self.content.iter_mut()
  }

  pub fn iter_modifications(&self) -> alloc::collections::btree_set::Iter<'_, PhraseModification> {
    self.modifications.iter()
  }

  #[must_use]
  pub fn iter_timeslices(&self) -> PhraseTimesliceIter<'_> {
    PhraseTimesliceIter {
      index: 0,
      num_timeslices: self.num_timeslices(),
      pending_timeslice: None,
      content_iterator: self.iter(),
      child_phrase_iterator: None,
      child_multivoice_iterator: None,
      modifications: &self.modifications,
    }
  }
}

impl IntoIterator for Phrase {
  type Item = PhraseContent;
  type IntoIter = alloc::vec::IntoIter<Self::Item>;
  fn into_iter(self) -> Self::IntoIter {
    self.content.into_iter()
  }
}

impl<'a> IntoIterator for &'a Phrase {
  type Item = &'a PhraseContent;
  type IntoIter = core::slice::Iter<'a, PhraseContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut Phrase {
  type Item = &'a mut PhraseContent;
  type IntoIter = core::slice::IterMut<'a, PhraseContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

impl Clone for Phrase {
  fn clone(&self) -> Self {
    Self {
      id: generate_id(),
      content: self.content.clone(),
      modifications: self.modifications.clone(),
    }
  }
}

impl PartialEq for Phrase {
  fn eq(&self, other: &Self) -> bool {
    self.content == other.content && self.modifications == other.modifications
  }
}

pub struct PhraseTimesliceIter<'a> {
  index: usize,
  num_timeslices: usize,
  pending_timeslice: Option<Timeslice>,
  content_iterator: core::slice::Iter<'a, PhraseContent>,
  child_phrase_iterator: Option<Box<PhraseTimesliceIter<'a>>>,
  child_multivoice_iterator: Option<MultiVoiceTimesliceIter<'a>>,
  modifications: &'a BTreeSet<PhraseModification>,
}

fn update_timeslice_details(iterator: &mut PhraseTimesliceIter, mut timeslice: Timeslice) -> Option<Timeslice> {
  let mut pending_timeslice_updated = false;
  timeslice.content.iter_mut().for_each(|content| {
    if !iterator.modifications.is_empty() {
      let details = content.add_phrase_details(iterator.index, iterator.num_timeslices);
      iterator.modifications.iter().for_each(|modification| {
        details.modifications.push(modification.r#type);
      });
    }
    if !pending_timeslice_updated {
      if let Some(pending_timeslice) = iterator.pending_timeslice.as_mut() {
        pending_timeslice.content.iter_mut().for_each(|note| {
          note.phrase_details.iter_mut().for_each(|details| {
            details.next_pitch = content.note.pitch;
            details.next_accidental = content.note.accidental;
          });
        });
      }
      pending_timeslice_updated = true;
    }
  });
  iterator.index += 1;
  let mut return_slice = Some(timeslice);
  core::mem::swap(&mut iterator.pending_timeslice, &mut return_slice);
  return_slice
}

impl Iterator for PhraseTimesliceIter<'_> {
  type Item = Timeslice;
  fn next(&mut self) -> Option<Self::Item> {
    let (mut valid_timeslice, mut timeslice) = (false, None);
    while !valid_timeslice {
      if let Some(child_iterator) = &mut self.child_phrase_iterator {
        match child_iterator.next() {
          Some(timeslice) => return update_timeslice_details(self, timeslice),
          None => self.child_phrase_iterator = None,
        }
      }
      if let Some(child_iterator) = &mut self.child_multivoice_iterator {
        match child_iterator.next() {
          Some(timeslice) => return update_timeslice_details(self, timeslice),
          None => self.child_multivoice_iterator = None,
        }
      }
      (valid_timeslice, timeslice) = match self.content_iterator.next() {
        Some(PhraseContent::Note(note)) => match update_timeslice_details(self, note.to_timeslice()) {
          Some(timeslice) => (true, Some(timeslice)),
          None => (false, None),
        },
        Some(PhraseContent::Chord(chord)) => match update_timeslice_details(self, chord.to_timeslice()) {
          Some(timeslice) => (true, Some(timeslice)),
          None => (false, None),
        },
        Some(PhraseContent::Phrase(phrase)) => {
          let mut child_iterator = phrase.iter_timeslices();
          match child_iterator.next() {
            Some(timeslice) => {
              self.child_phrase_iterator = Some(Box::new(child_iterator));
              match update_timeslice_details(self, timeslice) {
                Some(timeslice) => (true, Some(timeslice)),
                None => (false, None),
              }
            }
            None => (false, None),
          }
        }
        Some(PhraseContent::MultiVoice(multivoice)) => {
          let mut child_iterator = multivoice.iter_timeslices();
          match child_iterator.next() {
            Some(timeslice) => {
              self.child_multivoice_iterator = Some(child_iterator);
              match update_timeslice_details(self, timeslice) {
                Some(timeslice) => (true, Some(timeslice)),
                None => (false, None),
              }
            }
            None => (false, None),
          }
        }
        None => (true, self.pending_timeslice.take()),
      };
    }
    timeslice
  }
}

impl core::iter::FusedIterator for PhraseTimesliceIter<'_> {}

#[cfg(feature = "print")]
impl core::fmt::Display for Phrase {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let mods = self
      .iter_modifications()
      .map(ToString::to_string)
      .collect::<Vec<String>>()
      .join(", ");
    let items = self
      .iter()
      .map(|item| match item {
        PhraseContent::Note(note) => note.to_string(),
        PhraseContent::Chord(chord) => chord.to_string(),
        PhraseContent::Phrase(phrase) => phrase.to_string(),
        PhraseContent::MultiVoice(multivoice) => multivoice.to_string(),
      })
      .collect::<Vec<_>>()
      .join(", ");
    write!(
      f,
      "Phrase{}: [{items}]",
      if mods.is_empty() {
        String::new()
      } else {
        format!(" ({mods})")
      }
    )
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::note::{Accidental, DurationType, PitchName};

  fn create_phrase() -> Phrase {
    let mut phrase = Phrase::new();
    let phrase1 = phrase.add_phrase();
    let phrase2 = phrase1.add_phrase();
    phrase2.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Quarter, 0),
      Some(Accidental::Sharp),
    );
    phrase2.add_note(
      Pitch::new(PitchName::D, 4),
      Duration::new(DurationType::Quarter, 0),
      Some(Accidental::Flat),
    );
    let multivoice = phrase2.add_multivoice();
    let mut mphrase = multivoice.add_phrase();
    mphrase.add_note(
      Pitch::new(PitchName::E, 4),
      Duration::new(DurationType::Quarter, 0),
      Some(Accidental::Natural),
    );
    mphrase.add_note(
      Pitch::new(PitchName::F, 4),
      Duration::new(DurationType::Quarter, 0),
      Some(Accidental::Sharp),
    );
    mphrase.add_note(Pitch::new_rest(), Duration::new(DurationType::Half, 0), None);
    mphrase = multivoice.add_phrase();
    mphrase.add_note(
      Pitch::new(PitchName::G, 4),
      Duration::new(DurationType::Half, 0),
      Some(Accidental::Flat),
    );
    mphrase.add_note(
      Pitch::new(PitchName::A, 4),
      Duration::new(DurationType::Half, 0),
      Some(Accidental::Natural),
    );
    let phrase3 = phrase2.add_phrase();
    let phrase4 = phrase3.add_phrase();
    phrase4.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Quarter, 0),
      Some(Accidental::Sharp),
    );
    let chord = phrase4.add_chord();
    chord.add_note(
      Pitch::new(PitchName::D, 4),
      Duration::new(DurationType::Quarter, 0),
      Some(Accidental::Flat),
    );
    chord.add_note(
      Pitch::new(PitchName::E, 4),
      Duration::new(DurationType::Quarter, 0),
      Some(Accidental::Natural),
    );
    chord.add_note(
      Pitch::new(PitchName::F, 4),
      Duration::new(DurationType::Quarter, 0),
      Some(Accidental::Sharp),
    );
    phrase
  }

  #[test]
  fn test_triplet() {
    let tempo = Tempo::new(Duration::new(DurationType::Quarter, 0), 120);
    let mut phrase = Phrase::new();
    phrase.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Quarter, 0),
      None,
    );
    phrase.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Quarter, 0),
      None,
    );
    phrase.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Quarter, 0),
      None,
    );
    assert_eq!(phrase.get_duration(&tempo, None), 1.5);
    phrase.add_modification(PhraseModificationType::Tuplet {
      num_beats: 3,
      into_beats: 2,
    });
    assert_eq!(phrase.get_duration(&tempo, None), 1.0);
  }

  #[test]
  fn test_flatten_light() {
    let tempo = Tempo::new(Duration::new(DurationType::Quarter, 0), 120);
    let phrase = create_phrase().flatten(false);
    assert_eq!(phrase.get_duration(&tempo, None), 4.0);
  }

  #[test]
  fn test_flatten_full() {
    let tempo = Tempo::new(Duration::new(DurationType::Quarter, 0), 120);
    let phrase = create_phrase().flatten(true);
    assert_eq!(phrase.get_duration(&tempo, None), 4.0);
  }
}
