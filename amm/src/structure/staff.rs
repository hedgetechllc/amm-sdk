use super::{
  chord::Chord,
  multivoice::{MultiVoice, MultiVoiceTimesliceIter},
  phrase::{Phrase, PhraseContent, PhraseTimesliceIter},
};
use crate::context::{generate_id, Tempo};
use crate::modification::{Direction, DirectionType};
use crate::note::{Accidental, Duration, Note, Pitch};
use crate::temporal::Timeslice;
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};

#[derive(Clone, Debug, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum StaffContent {
  Note(Note),
  Chord(Chord),
  Phrase(Phrase),
  MultiVoice(MultiVoice),
  Direction(Direction),
}

#[derive(Debug, Default, Eq, JsonDeserialize, JsonSerialize)]
pub struct Staff {
  id: usize,
  name: String,
  content: Vec<StaffContent>,
}

impl Staff {
  #[must_use]
  pub fn new(name: &str) -> Staff {
    Self {
      id: generate_id(),
      name: String::from(name),
      content: Vec::new(),
    }
  }

  pub(crate) fn simplify(&mut self) {
    let mut content_changed = true;
    while content_changed {
      let mut content_to_edit = Vec::new();
      self.content.iter_mut().enumerate().for_each(|(idx, item)| match item {
        StaffContent::Phrase(phrase) => {
          phrase.simplify();
          if phrase.is_empty() {
            content_to_edit.push((idx, None));
          } else if phrase.iter_modifications().len() == 0 {
            let phrase = core::mem::take(phrase);
            content_to_edit.push((
              idx,
              Some(
                phrase
                  .into_iter()
                  .map(|item| match item {
                    PhraseContent::Note(note) => StaffContent::Note(note),
                    PhraseContent::Chord(chord) => StaffContent::Chord(chord),
                    PhraseContent::Phrase(phrase) => StaffContent::Phrase(phrase),
                    PhraseContent::MultiVoice(multivoice) => StaffContent::MultiVoice(multivoice),
                  })
                  .collect(),
              ),
            ));
          }
        }
        StaffContent::MultiVoice(multivoice) => {
          let single_phrase = multivoice
            .simplify()
            .map(|phrase| Vec::from([StaffContent::Phrase(phrase)]));
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
  pub fn flatten(&self) -> Self {
    Self {
      id: generate_id(),
      name: self.name.clone(),
      content: self
        .iter()
        .map(|item| match item {
          StaffContent::Phrase(phrase) => StaffContent::Phrase(phrase.flatten(false)),
          StaffContent::MultiVoice(multivoice) => StaffContent::Phrase(multivoice.flatten()),
          _ => item.clone(),
        })
        .collect(),
    }
  }

  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }

  #[must_use]
  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn rename(&mut self, name: &str) -> &mut Self {
    self.name = String::from(name);
    self
  }

  pub fn add_note(&mut self, pitch: Pitch, duration: Duration, accidental: Option<Accidental>) -> &mut Note {
    self
      .content
      .push(StaffContent::Note(Note::new(pitch, duration, accidental)));
    match self.content.last_mut() {
      Some(StaffContent::Note(note)) => note,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn add_chord(&mut self) -> &mut Chord {
    self.content.push(StaffContent::Chord(Chord::new()));
    match self.content.last_mut() {
      Some(StaffContent::Chord(chord)) => chord,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn add_phrase(&mut self) -> &mut Phrase {
    self.content.push(StaffContent::Phrase(Phrase::new()));
    match self.content.last_mut() {
      Some(StaffContent::Phrase(phrase)) => phrase,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn add_multivoice(&mut self) -> &mut MultiVoice {
    self.content.push(StaffContent::MultiVoice(MultiVoice::new()));
    match self.content.last_mut() {
      Some(StaffContent::MultiVoice(multivoice)) => multivoice,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn add_direction(&mut self, direction: DirectionType) -> &mut Direction {
    self.content.push(StaffContent::Direction(Direction::new(direction)));
    match self.content.last_mut() {
      Some(StaffContent::Direction(direction)) => direction,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn claim_note(&mut self, note: Note) -> &mut Note {
    self.content.push(StaffContent::Note(note));
    match self.content.last_mut() {
      Some(StaffContent::Note(note)) => note,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn claim_chord(&mut self, chord: Chord) -> &mut Chord {
    self.content.push(StaffContent::Chord(chord));
    match self.content.last_mut() {
      Some(StaffContent::Chord(chord)) => chord,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn claim_phrase(&mut self, phrase: Phrase) -> &mut Phrase {
    self.content.push(StaffContent::Phrase(phrase));
    match self.content.last_mut() {
      Some(StaffContent::Phrase(phrase)) => phrase,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn claim_multivoice(&mut self, multivoice: MultiVoice) -> &mut MultiVoice {
    self.content.push(StaffContent::MultiVoice(multivoice));
    match self.content.last_mut() {
      Some(StaffContent::MultiVoice(multivoice)) => multivoice,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn claim_direction(&mut self, direction: Direction) -> &mut Direction {
    self.content.push(StaffContent::Direction(direction));
    match self.content.last_mut() {
      Some(StaffContent::Direction(direction)) => direction,
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
      .insert(index, StaffContent::Note(Note::new(pitch, duration, accidental)));
    match self.content.get_mut(index) {
      Some(StaffContent::Note(note)) => note,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn insert_chord(&mut self, index: usize) -> &mut Chord {
    self.content.insert(index, StaffContent::Chord(Chord::new()));
    match self.content.get_mut(index) {
      Some(StaffContent::Chord(chord)) => chord,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn insert_phrase(&mut self, index: usize) -> &mut Phrase {
    self.content.insert(index, StaffContent::Phrase(Phrase::new()));
    match self.content.get_mut(index) {
      Some(StaffContent::Phrase(phrase)) => phrase,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn insert_multivoice(&mut self, index: usize) -> &mut MultiVoice {
    self.content.insert(index, StaffContent::MultiVoice(MultiVoice::new()));
    match self.content.get_mut(index) {
      Some(StaffContent::MultiVoice(multivoice)) => multivoice,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn insert_direction(&mut self, index: usize, direction: DirectionType) -> &mut Direction {
    self
      .content
      .insert(index, StaffContent::Direction(Direction::new(direction)));
    match self.content.get_mut(index) {
      Some(StaffContent::Direction(direction)) => direction,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  #[must_use]
  pub fn get_note(&self, id: usize) -> Option<&Note> {
    self.iter().find_map(|item| match item {
      StaffContent::Note(note) if note.get_id() == id => Some(note),
      StaffContent::Chord(chord) => chord.get_note(id),
      StaffContent::Phrase(phrase) => phrase.get_note(id),
      StaffContent::MultiVoice(multivoice) => multivoice.get_note(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_note_mut(&mut self, id: usize) -> Option<&mut Note> {
    self.iter_mut().find_map(|item| match item {
      StaffContent::Note(note) if note.get_id() == id => Some(note),
      StaffContent::Chord(chord) => chord.get_note_mut(id),
      StaffContent::Phrase(phrase) => phrase.get_note_mut(id),
      StaffContent::MultiVoice(multivoice) => multivoice.get_note_mut(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_chord(&self, id: usize) -> Option<&Chord> {
    self.iter().find_map(|item| match item {
      StaffContent::Chord(chord) if chord.get_id() == id => Some(chord),
      StaffContent::Phrase(phrase) => phrase.get_chord(id),
      StaffContent::MultiVoice(multivoice) => multivoice.get_chord(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_chord_mut(&mut self, id: usize) -> Option<&mut Chord> {
    self.iter_mut().find_map(|item| match item {
      StaffContent::Chord(chord) if chord.get_id() == id => Some(chord),
      StaffContent::Phrase(phrase) => phrase.get_chord_mut(id),
      StaffContent::MultiVoice(multivoice) => multivoice.get_chord_mut(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_phrase(&self, id: usize) -> Option<&Phrase> {
    self.iter().find_map(|item| match item {
      StaffContent::Phrase(phrase) => phrase.get_phrase(id),
      StaffContent::MultiVoice(multivoice) => multivoice.get_phrase(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_phrase_mut(&mut self, id: usize) -> Option<&mut Phrase> {
    self.iter_mut().find_map(|item| match item {
      StaffContent::Phrase(phrase) => phrase.get_phrase_mut(id),
      StaffContent::MultiVoice(multivoice) => multivoice.get_phrase_mut(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_multivoice(&self, id: usize) -> Option<&MultiVoice> {
    self.iter().find_map(|item| match item {
      StaffContent::Phrase(phrase) => phrase.get_multivoice(id),
      StaffContent::MultiVoice(multivoice) => multivoice.get_multivoice(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_multivoice_mut(&mut self, id: usize) -> Option<&mut MultiVoice> {
    self.iter_mut().find_map(|item| match item {
      StaffContent::Phrase(phrase) => phrase.get_multivoice_mut(id),
      StaffContent::MultiVoice(multivoice) => multivoice.get_multivoice_mut(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_direction(&self, id: usize) -> Option<&Direction> {
    self.iter().find_map(|item| match item {
      StaffContent::Direction(direction) if direction.get_id() == id => Some(direction),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_direction_mut(&mut self, id: usize) -> Option<&mut Direction> {
    self.iter_mut().find_map(|item| match item {
      StaffContent::Direction(direction) if direction.get_id() == id => Some(direction),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_index_of_item(&self, id: usize) -> Option<usize> {
    self.iter().position(|item| match item {
      StaffContent::Note(note) => note.get_id() == id,
      StaffContent::Chord(chord) => chord.get_id() == id,
      StaffContent::Phrase(phrase) => phrase.get_id() == id,
      StaffContent::MultiVoice(multivoice) => multivoice.get_id() == id,
      StaffContent::Direction(direction) => direction.get_id() == id,
    })
  }

  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration) -> f64 {
    self
      .iter()
      .map(|content| match &content {
        StaffContent::Note(note) => note.get_beats(beat_base, None),
        StaffContent::Chord(chord) => chord.get_beats(beat_base, None),
        StaffContent::Phrase(phrase) => phrase.get_beats(beat_base, None),
        StaffContent::MultiVoice(multivoice) => multivoice.get_beats(beat_base, None),
        StaffContent::Direction(_) => 0.0,
      })
      .sum()
  }

  #[must_use]
  pub fn get_duration(&self, tempo: &Tempo) -> f64 {
    self.get_beats(&tempo.base_note) * 60.0 / f64::from(tempo.beats_per_minute)
  }

  pub fn remove_item(&mut self, id: usize) -> &mut Self {
    self.content.retain(|item| match item {
      StaffContent::Note(note) => note.get_id() != id,
      StaffContent::Chord(chord) => chord.get_id() != id,
      StaffContent::Phrase(phrase) => phrase.get_id() != id,
      StaffContent::MultiVoice(multivoice) => multivoice.get_id() != id,
      StaffContent::Direction(direction) => direction.get_id() != id,
    });
    self.iter_mut().for_each(|item| match item {
      StaffContent::Chord(chord) => {
        chord.remove_item(id);
      }
      StaffContent::Phrase(phrase) => {
        phrase.remove_item(id);
      }
      StaffContent::MultiVoice(multivoice) => {
        multivoice.remove_item(id);
      }
      _ => (),
    });
    self
  }

  pub fn remove_modification(&mut self, id: usize) -> &mut Self {
    self.iter_mut().for_each(|item| match item {
      StaffContent::Note(note) => {
        note.remove_modification(id);
      }
      StaffContent::Chord(chord) => {
        chord.remove_modification(id);
      }
      StaffContent::Phrase(phrase) => {
        phrase.remove_modification(id);
      }
      StaffContent::MultiVoice(multivoice) => {
        multivoice.remove_modification(id);
      }
      StaffContent::Direction(_) => (),
    });
    self
  }

  #[must_use]
  pub fn num_timeslices(&self) -> usize {
    self
      .iter()
      .map(|item| match item {
        StaffContent::Phrase(phrase) => phrase.num_timeslices(),
        StaffContent::MultiVoice(multivoice) => multivoice.num_timeslices(),
        _ => 1,
      })
      .sum()
  }

  pub fn iter(&self) -> core::slice::Iter<'_, StaffContent> {
    self.content.iter()
  }

  pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, StaffContent> {
    self.content.iter_mut()
  }

  #[must_use]
  pub fn iter_timeslices(&self) -> StaffTimesliceIter<'_> {
    StaffTimesliceIter {
      content_iterator: self.iter(),
      child_phrase: None,
      child_multivoice: None,
    }
  }
}

impl IntoIterator for Staff {
  type Item = StaffContent;
  type IntoIter = alloc::vec::IntoIter<Self::Item>;
  fn into_iter(self) -> Self::IntoIter {
    self.content.into_iter()
  }
}

impl<'a> IntoIterator for &'a Staff {
  type Item = &'a StaffContent;
  type IntoIter = core::slice::Iter<'a, StaffContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut Staff {
  type Item = &'a mut StaffContent;
  type IntoIter = core::slice::IterMut<'a, StaffContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

impl Clone for Staff {
  fn clone(&self) -> Self {
    Self {
      id: generate_id(),
      name: self.name.clone(),
      content: self.content.clone(),
    }
  }
}

impl PartialEq for Staff {
  fn eq(&self, other: &Self) -> bool {
    self.content == other.content && self.name == other.name
  }
}

pub struct StaffTimesliceIter<'a> {
  content_iterator: core::slice::Iter<'a, StaffContent>,
  child_phrase: Option<PhraseTimesliceIter<'a>>,
  child_multivoice: Option<MultiVoiceTimesliceIter<'a>>,
}

impl Iterator for StaffTimesliceIter<'_> {
  type Item = Timeslice;
  fn next(&mut self) -> Option<Self::Item> {
    if let Some(child_iterator) = &mut self.child_phrase {
      match child_iterator.next() {
        Some(timeslice) => return Some(timeslice),
        None => self.child_phrase = None,
      }
    }
    if let Some(child_iterator) = &mut self.child_multivoice {
      match child_iterator.next() {
        Some(timeslice) => return Some(timeslice),
        None => self.child_multivoice = None,
      }
    }
    let (mut valid_timeslice, mut timeslice) = (false, None);
    while !valid_timeslice {
      (valid_timeslice, timeslice) = match self.content_iterator.next() {
        Some(StaffContent::Direction(direction)) => (true, Some(direction.to_timeslice())),
        Some(StaffContent::Note(note)) => (true, Some(note.to_timeslice())),
        Some(StaffContent::Chord(chord)) => (true, Some(chord.to_timeslice())),
        Some(StaffContent::Phrase(phrase)) => {
          let mut child_iterator = phrase.iter_timeslices();
          match child_iterator.next() {
            Some(timeslice) => {
              self.child_phrase = Some(child_iterator);
              (true, Some(timeslice))
            }
            None => (false, None),
          }
        }
        Some(StaffContent::MultiVoice(multivoice)) => {
          let mut child_iterator = multivoice.iter_timeslices();
          match child_iterator.next() {
            Some(timeslice) => {
              self.child_multivoice = Some(child_iterator);
              (true, Some(timeslice))
            }
            None => (false, None),
          }
        }
        None => (true, None),
      };
    }
    timeslice
  }
}

impl core::iter::FusedIterator for StaffTimesliceIter<'_> {}

#[cfg(feature = "print")]
impl core::fmt::Display for Staff {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let items = self
      .iter()
      .map(|item| match item {
        StaffContent::Note(note) => note.to_string(),
        StaffContent::Chord(chord) => chord.to_string(),
        StaffContent::Phrase(phrase) => phrase.to_string(),
        StaffContent::MultiVoice(multi_voice) => multi_voice.to_string(),
        StaffContent::Direction(direction) => direction.r#type.to_string(),
      })
      .collect::<Vec<_>>()
      .join(", ");
    write!(f, "Staff {}: [{items}]", self.name)
  }
}
