use super::{chord::Chord, multivoice::MultiVoice, timeslice::Timeslice};
use crate::context::{generate_id, Tempo};
use crate::modification::{PhraseModification, PhraseModificationType};
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
pub enum PhraseContent {
  Note(Rc<RefCell<Note>>),
  Chord(Rc<RefCell<Chord>>),
  Phrase(Rc<RefCell<Phrase>>),
  MultiVoice(Rc<RefCell<MultiVoice>>),
}

#[cfg_attr(feature = "json", derive(JsonDeserialize, JsonSerialize))]
#[derive(Clone, Debug, Default)]
pub struct Phrase {
  pub(crate) id: usize,
  pub(crate) content: Vec<PhraseContent>,
  pub(crate) modifications: Vec<Rc<RefCell<PhraseModification>>>,
}

impl Phrase {
  #[must_use]
  pub fn new() -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: generate_id(),
      content: Vec::new(),
      modifications: Vec::new(),
    }))
  }

  #[must_use]
  pub fn flatten(&self, fully: bool) -> Rc<RefCell<Self>> {
    let mut flat_phrase;
    if fully {
      flat_phrase = Self {
        id: self.id,
        content: Vec::new(),
        modifications: self.modifications.clone(),
      };
      self.content.iter().for_each(|item| match item {
        PhraseContent::Phrase(phrase) => phrase
          .borrow()
          .flatten(true)
          .borrow()
          .iter()
          .for_each(|item| flat_phrase.content.push(item.clone())),
        PhraseContent::MultiVoice(multivoice) => multivoice
          .borrow()
          .flatten()
          .borrow()
          .iter()
          .for_each(|item| flat_phrase.content.push(item.clone())),
        _ => flat_phrase.content.push(item.clone()),
      });
    } else {
      flat_phrase = Self {
        id: self.id,
        content: self
          .content
          .iter()
          .map(|item| match item {
            PhraseContent::Note(note) => PhraseContent::Note(Rc::clone(note)),
            PhraseContent::Chord(chord) => PhraseContent::Chord(Rc::clone(chord)),
            PhraseContent::Phrase(phrase) => PhraseContent::Phrase(phrase.borrow().flatten(false)),
            PhraseContent::MultiVoice(multivoice) => PhraseContent::Phrase(multivoice.borrow().flatten()),
          })
          .collect(),
        modifications: self.modifications.clone(),
      };
    }
    Rc::new(RefCell::new(flat_phrase))
  }

  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn add_note(&mut self, pitch: Pitch, duration: Duration, accidental: Option<Accidental>) -> Rc<RefCell<Note>> {
    let note = Note::new(pitch, duration, accidental);
    self.content.push(PhraseContent::Note(Rc::clone(&note)));
    note
  }

  pub fn add_chord(&mut self) -> Rc<RefCell<Chord>> {
    let chord = Chord::new();
    self.content.push(PhraseContent::Chord(Rc::clone(&chord)));
    chord
  }

  pub fn add_phrase(&mut self) -> Rc<RefCell<Phrase>> {
    let phrase = Phrase::new();
    self.content.push(PhraseContent::Phrase(Rc::clone(&phrase)));
    phrase
  }

  pub fn add_multivoice(&mut self) -> Rc<RefCell<MultiVoice>> {
    let multivoice = MultiVoice::new();
    self.content.push(PhraseContent::MultiVoice(Rc::clone(&multivoice)));
    multivoice
  }

  pub fn add_modification(&mut self, modification: PhraseModificationType) -> Rc<RefCell<PhraseModification>> {
    self
      .modifications
      .retain(|mods| *mods.borrow().get_modification() != modification);
    let modification = PhraseModification::new(modification);
    self.modifications.push(Rc::clone(&modification));
    modification
  }

  pub fn insert_note(
    &mut self,
    index: usize,
    pitch: Pitch,
    duration: Duration,
    accidental: Option<Accidental>,
  ) -> Rc<RefCell<Note>> {
    let note = Note::new(pitch, duration, accidental);
    self.content.insert(index, PhraseContent::Note(Rc::clone(&note)));
    note
  }

  pub fn insert_chord(&mut self, index: usize) -> Rc<RefCell<Chord>> {
    let chord = Chord::new();
    self.content.insert(index, PhraseContent::Chord(Rc::clone(&chord)));
    chord
  }

  pub fn insert_phrase(&mut self, index: usize) -> Rc<RefCell<Phrase>> {
    let phrase = Phrase::new();
    self.content.insert(index, PhraseContent::Phrase(Rc::clone(&phrase)));
    phrase
  }

  pub fn insert_multivoice(&mut self, index: usize) -> Rc<RefCell<MultiVoice>> {
    let multivoice = MultiVoice::new();
    self
      .content
      .insert(index, PhraseContent::MultiVoice(Rc::clone(&multivoice)));
    multivoice
  }

  #[must_use]
  pub fn get_note(&self, id: usize) -> Option<Rc<RefCell<Note>>> {
    self.content.iter().find_map(|item| match item {
      PhraseContent::Note(note) if note.borrow().get_id() == id => Some(Rc::clone(note)),
      PhraseContent::Chord(chord) => chord.borrow().get_note(id),
      PhraseContent::Phrase(phrase) => phrase.borrow().get_note(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_chord(&self, id: usize) -> Option<Rc<RefCell<Chord>>> {
    self.content.iter().find_map(|item| match item {
      PhraseContent::Chord(chord) if chord.borrow().get_id() == id => Some(Rc::clone(chord)),
      PhraseContent::Phrase(phrase) => phrase.borrow().get_chord(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_phrase(&self, id: usize) -> Option<Rc<RefCell<Phrase>>> {
    self.content.iter().find_map(|item| match item {
      PhraseContent::Phrase(phrase) if phrase.borrow().get_id() == id => Some(Rc::clone(phrase)),
      PhraseContent::Phrase(phrase) => phrase.borrow().get_phrase(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_multivoice(&self, id: usize) -> Option<Rc<RefCell<MultiVoice>>> {
    self.content.iter().find_map(|item| match item {
      PhraseContent::MultiVoice(multivoice) if multivoice.borrow().get_id() == id => Some(Rc::clone(multivoice)),
      PhraseContent::MultiVoice(multivoice) => multivoice.borrow().get_multivoice(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_modification(&self, id: usize) -> Option<Rc<RefCell<PhraseModification>>> {
    self.modifications.iter().find_map(|modification| {
      if modification.borrow().get_id() == id {
        Some(Rc::clone(modification))
      } else {
        None
      }
    })
  }

  #[must_use]
  pub fn get_index_of_item(&self, id: usize) -> Option<usize> {
    self.content.iter().position(|item| match item {
      PhraseContent::Note(note) => note.borrow().get_id() == id,
      PhraseContent::Chord(chord) => chord.borrow().get_id() == id,
      PhraseContent::Phrase(phrase) => phrase.borrow().get_id() == id,
      PhraseContent::MultiVoice(multivoice) => multivoice.borrow().get_id() == id,
    })
  }

  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration, tuplet_ratio: Option<f64>) -> f64 {
    // Determine if this phrase creates a tuplet
    let new_tuplet_ratio = self
      .modifications
      .iter()
      .find_map(|item| match item.borrow().get_modification() {
        PhraseModificationType::Tuplet { num_beats, into_beats } => {
          Some(f64::from(*into_beats) / f64::from(*num_beats))
        }
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
      .content
      .iter()
      .map(|content| match &content {
        PhraseContent::Note(note) => note.borrow().get_beats(beat_base, tuplet_ratio),
        PhraseContent::Chord(chord) => chord.borrow().get_beats(beat_base, tuplet_ratio),
        PhraseContent::Phrase(phrase) => phrase.borrow().get_beats(beat_base, tuplet_ratio),
        PhraseContent::MultiVoice(multivoice) => multivoice.borrow().get_beats(beat_base, tuplet_ratio),
      })
      .sum()
  }

  #[must_use]
  pub fn get_duration(&self, tempo: &Tempo, tuplet_ratio: Option<f64>) -> f64 {
    self.get_beats(&tempo.base_note, tuplet_ratio) * 60.0 / f64::from(tempo.beats_per_minute)
  }

  pub fn remove_item(&mut self, id: usize) -> &mut Self {
    self.content.retain(|item| match item {
      PhraseContent::Note(note) => note.borrow().get_id() != id,
      PhraseContent::Chord(chord) => chord.borrow().get_id() != id,
      PhraseContent::Phrase(phrase) => phrase.borrow().get_id() != id,
      PhraseContent::MultiVoice(multivoice) => multivoice.borrow().get_id() != id,
    });
    self.content.iter().for_each(|item| match item {
      PhraseContent::Chord(chord) => {
        chord.borrow_mut().remove_item(id);
      }
      PhraseContent::Phrase(phrase) => {
        phrase.borrow_mut().remove_item(id);
      }
      PhraseContent::MultiVoice(multivoice) => {
        multivoice.borrow_mut().remove_item(id);
      }
      PhraseContent::Note(_) => (),
    });
    self
  }

  pub fn remove_modification(&mut self, id: usize) -> &mut Self {
    self
      .modifications
      .retain(|modification| modification.borrow().get_id() != id);
    self
  }

  #[must_use]
  pub(crate) fn num_timeslices(&self) -> usize {
    self
      .content
      .iter()
      .map(|item| match item {
        PhraseContent::Note(_) | PhraseContent::Chord(_) => 1,
        PhraseContent::Phrase(phrase) => phrase.borrow().num_timeslices(),
        PhraseContent::MultiVoice(multivoice) => multivoice.borrow().num_timeslices(),
      })
      .sum()
  }

  fn update_timeslice_details(
    &self,
    timeslices: &mut Vec<Timeslice>,
    mut timeslice: Timeslice,
    index: usize,
    num_timeslices: usize,
  ) -> usize {
    if !self.modifications.is_empty() {
      for content in &mut timeslice.content {
        let details = content.add_phrase_details(index, num_timeslices);
        self.modifications.iter().for_each(|modification| {
          details.modifications.push(*modification.borrow().get_modification());
        });
        if index > 0 {
          timeslices[index - 1].content.iter_mut().for_each(|item| {
            item.phrase_details.iter_mut().for_each(|details| {
              let note = content.note.borrow();
              details.next_pitch = note.pitch;
              details.next_accidental = note.accidental;
            });
          });
        }
      }
    }
    timeslices.push(timeslice);
    index + 1
  }

  pub fn iter(&self) -> Iter<'_, PhraseContent> {
    self.content.iter()
  }

  #[must_use]
  pub fn iter_modifications(&self) -> Iter<'_, Rc<RefCell<PhraseModification>>> {
    self.modifications.iter()
  }

  #[must_use]
  pub fn iter_timeslices(&self) -> Vec<Timeslice> {
    let num_timeslices = self.num_timeslices();
    let (mut index, mut timeslices) = (0, Vec::new());
    self.content.iter().for_each(|item| match item {
      PhraseContent::Note(note) => {
        let mut timeslice = Timeslice::new();
        timeslice.add_note(note);
        index = self.update_timeslice_details(&mut timeslices, timeslice, index, num_timeslices);
      }
      PhraseContent::Chord(chord) => {
        let timeslice = chord.borrow().to_timeslice();
        index = self.update_timeslice_details(&mut timeslices, timeslice, index, num_timeslices);
      }
      PhraseContent::Phrase(phrase) => {
        phrase.borrow().iter_timeslices().into_iter().for_each(|timeslice| {
          index = self.update_timeslice_details(&mut timeslices, timeslice, index, num_timeslices);
        });
      }
      PhraseContent::MultiVoice(multivoice) => {
        multivoice.borrow().iter_timeslices().into_iter().for_each(|timeslice| {
          index = self.update_timeslice_details(&mut timeslices, timeslice, index, num_timeslices);
        });
      }
    });
    timeslices
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
  type IntoIter = Iter<'a, PhraseContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Phrase {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let mods = self
      .modifications
      .iter()
      .map(|modification| modification.borrow().to_string())
      .collect::<Vec<String>>()
      .join(", ");
    let items = self
      .content
      .iter()
      .map(|item| match item {
        PhraseContent::Note(note) => note.borrow().to_string(),
        PhraseContent::Chord(chord) => chord.borrow().to_string(),
        PhraseContent::Phrase(phrase) => phrase.borrow().to_string(),
        PhraseContent::MultiVoice(multivoice) => multivoice.borrow().to_string(),
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
  use crate::{DurationType, PitchName};

  fn create_phrase() -> Rc<RefCell<Phrase>> {
    let phrase = Phrase::new();
    let phrase1 = phrase.borrow_mut().add_phrase();
    let phrase2 = phrase1.borrow_mut().add_phrase();
    phrase2.borrow_mut().add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Quarter, 0),
      Some(Accidental::Sharp),
    );
    phrase2.borrow_mut().add_note(
      Pitch::new(PitchName::D, 4),
      Duration::new(DurationType::Quarter, 0),
      Some(Accidental::Flat),
    );
    let multivoice = phrase2.borrow_mut().add_multivoice();
    let mphrase1 = multivoice.borrow_mut().add_phrase();
    let mphrase2 = multivoice.borrow_mut().add_phrase();
    mphrase1.borrow_mut().add_note(
      Pitch::new(PitchName::E, 4),
      Duration::new(DurationType::Quarter, 0),
      Some(Accidental::Natural),
    );
    mphrase1.borrow_mut().add_note(
      Pitch::new(PitchName::F, 4),
      Duration::new(DurationType::Quarter, 0),
      Some(Accidental::Sharp),
    );
    mphrase1
      .borrow_mut()
      .add_note(Pitch::new_rest(), Duration::new(DurationType::Half, 0), None);
    mphrase2.borrow_mut().add_note(
      Pitch::new(PitchName::G, 4),
      Duration::new(DurationType::Half, 0),
      Some(Accidental::Flat),
    );
    mphrase2.borrow_mut().add_note(
      Pitch::new(PitchName::A, 4),
      Duration::new(DurationType::Half, 0),
      Some(Accidental::Natural),
    );
    let phrase3 = phrase2.borrow_mut().add_phrase();
    let phrase4 = phrase3.borrow_mut().add_phrase();
    phrase4.borrow_mut().add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Quarter, 0),
      Some(Accidental::Sharp),
    );
    let chord = phrase4.borrow_mut().add_chord();
    chord.borrow_mut().add_note(
      Pitch::new(PitchName::D, 4),
      Duration::new(DurationType::Quarter, 0),
      Some(Accidental::Flat),
    );
    chord.borrow_mut().add_note(
      Pitch::new(PitchName::E, 4),
      Duration::new(DurationType::Quarter, 0),
      Some(Accidental::Natural),
    );
    chord.borrow_mut().add_note(
      Pitch::new(PitchName::F, 4),
      Duration::new(DurationType::Quarter, 0),
      Some(Accidental::Sharp),
    );
    phrase
  }

  #[test]
  fn test_triplet() {
    let tempo = Tempo::new(Duration::new(DurationType::Quarter, 0), 120);
    let phrase = Phrase::new();
    phrase.borrow_mut().add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Quarter, 0),
      None,
    );
    phrase.borrow_mut().add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Quarter, 0),
      None,
    );
    phrase.borrow_mut().add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Quarter, 0),
      None,
    );
    assert_eq!(phrase.borrow().get_duration(&tempo, None), 1.5);
    phrase.borrow_mut().add_modification(PhraseModificationType::Tuplet {
      num_beats: 3,
      into_beats: 2,
    });
    assert_eq!(phrase.borrow().get_duration(&tempo, None), 1.0);
  }

  #[test]
  fn test_flatten_light() {
    let tempo = Tempo::new(Duration::new(DurationType::Quarter, 0), 120);
    let phrase = create_phrase().borrow().flatten(false);
    assert_eq!(phrase.borrow().get_duration(&tempo, None), 4.0);
  }

  #[test]
  fn test_flatten_full() {
    let tempo = Tempo::new(Duration::new(DurationType::Quarter, 0), 120);
    let phrase = create_phrase().borrow().flatten(true);
    assert_eq!(phrase.borrow().get_duration(&tempo, None), 4.0);
  }
}
