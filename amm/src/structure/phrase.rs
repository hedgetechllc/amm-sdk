use super::{chord::Chord, multivoice::MultiVoice, timeslice::Timeslice};
use crate::context::{generate_id, Tempo};
use crate::modification::{PhraseModification, PhraseModificationType};
use crate::note::{Accidental, Duration, Note, Pitch};
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};
use core::slice::{Iter, IterMut};

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
  modifications: Vec<PhraseModification>,
}

impl Phrase {
  #[must_use]
  pub fn new() -> Self {
    Self {
      id: generate_id(),
      content: Vec::new(),
      modifications: Vec::new(),
    }
  }

  #[must_use]
  pub fn flatten(&self, fully: bool) -> Self {
    // Removes all multivoice layers (flattens multivoices into a single phrase)
    // The "fully" parameter determines whether sub-phrases will also be flattened into a single phrase of notes and chords
    let mut flat_phrase;
    if fully {
      flat_phrase = Self {
        id: generate_id(),
        content: Vec::new(),
        modifications: self.modifications.clone(),
      };
      self.content.iter().for_each(|item| match item {
        PhraseContent::Phrase(phrase) => phrase
          .flatten(true)
          .iter()
          .for_each(|item| flat_phrase.content.push(item.clone())),
        PhraseContent::MultiVoice(multivoice) => multivoice
          .flatten()
          .iter()
          .for_each(|item| flat_phrase.content.push(item.clone())),
        _ => flat_phrase.content.push(item.clone()),
      });
    } else {
      flat_phrase = Self {
        id: generate_id(),
        content: self
          .content
          .iter()
          .map(|item| match item {
            PhraseContent::Note(note) => PhraseContent::Note(note.clone()),
            PhraseContent::Chord(chord) => PhraseContent::Chord(chord.clone()),
            PhraseContent::Phrase(phrase) => PhraseContent::Phrase(phrase.flatten(false)),
            PhraseContent::MultiVoice(multivoice) => PhraseContent::Phrase(multivoice.flatten()),
          })
          .collect(),
        modifications: self.modifications.clone(),
      };
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

  pub fn add_modification(&mut self, mod_type: PhraseModificationType) -> &mut PhraseModification {
    self.modifications.retain(|mods| mods.r#type != mod_type);
    self.modifications.push(PhraseModification::new(mod_type));
    unsafe { self.modifications.last_mut().unwrap_unchecked() }
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
    match self.content.last_mut() {
      Some(PhraseContent::Note(note)) => note,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn insert_chord(&mut self, index: usize) -> &mut Chord {
    self.content.insert(index, PhraseContent::Chord(Chord::new()));
    match self.content.last_mut() {
      Some(PhraseContent::Chord(chord)) => chord,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn insert_phrase(&mut self, index: usize) -> &mut Phrase {
    self.content.insert(index, PhraseContent::Phrase(Phrase::new()));
    match self.content.last_mut() {
      Some(PhraseContent::Phrase(phrase)) => phrase,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn insert_multivoice(&mut self, index: usize) -> &mut MultiVoice {
    self.content.insert(index, PhraseContent::MultiVoice(MultiVoice::new()));
    match self.content.last_mut() {
      Some(PhraseContent::MultiVoice(multivoice)) => multivoice,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  #[must_use]
  pub fn get_note(&self, id: usize) -> Option<&Note> {
    self.content.iter().find_map(|item| match item {
      PhraseContent::Note(note) if note.get_id() == id => Some(note),
      PhraseContent::Chord(chord) => chord.get_note(id),
      PhraseContent::Phrase(phrase) => phrase.get_note(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_note_mut(&mut self, id: usize) -> Option<&mut Note> {
    self.content.iter_mut().find_map(|item| match item {
      PhraseContent::Note(note) if note.get_id() == id => Some(note),
      PhraseContent::Chord(chord) => chord.get_note_mut(id),
      PhraseContent::Phrase(phrase) => phrase.get_note_mut(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_chord(&self, id: usize) -> Option<&Chord> {
    self.content.iter().find_map(|item| match item {
      PhraseContent::Chord(chord) if chord.get_id() == id => Some(chord),
      PhraseContent::Phrase(phrase) => phrase.get_chord(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_chord_mut(&mut self, id: usize) -> Option<&mut Chord> {
    self.content.iter_mut().find_map(|item| match item {
      PhraseContent::Chord(chord) if chord.get_id() == id => Some(chord),
      PhraseContent::Phrase(phrase) => phrase.get_chord_mut(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_phrase(&self, id: usize) -> Option<&Phrase> {
    if self.id == id {
      Some(self)
    } else {
      self.content.iter().find_map(|item| match item {
        PhraseContent::Phrase(phrase) => phrase.get_phrase(id),
        _ => None,
      })
    }
  }

  #[must_use]
  pub fn get_phrase_mut(&mut self, id: usize) -> Option<&mut Phrase> {
    if self.id == id {
      Some(self)
    } else {
      self.content.iter_mut().find_map(|item| match item {
        PhraseContent::Phrase(phrase) => phrase.get_phrase_mut(id),
        _ => None,
      })
    }
  }

  #[must_use]
  pub fn get_multivoice(&self, id: usize) -> Option<&MultiVoice> {
    self.content.iter().find_map(|item| match item {
      PhraseContent::MultiVoice(multivoice) if multivoice.get_id() == id => Some(multivoice),
      PhraseContent::MultiVoice(multivoice) => multivoice.get_multivoice(id),
      _ => None,
    })
  }

  #[must_use]
  pub fn get_multivoice_mut(&mut self, id: usize) -> Option<&mut MultiVoice> {
    self.content.iter_mut().find_map(|item| match item {
      PhraseContent::MultiVoice(multivoice) => {
        if multivoice.get_id() == id {
          Some(multivoice)
        } else {
          multivoice.get_multivoice_mut(id)
        }
      }
      _ => None,
    })
  }

  #[must_use]
  pub fn get_modification(&self, id: usize) -> Option<&PhraseModification> {
    self
      .modifications
      .iter()
      .find(|modification| modification.get_id() == id)
  }

  #[must_use]
  pub fn get_modification_mut(&mut self, id: usize) -> Option<&mut PhraseModification> {
    self
      .modifications
      .iter_mut()
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
    let new_tuplet_ratio = self.modifications.iter().find_map(|item| match item.r#type {
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
      .content
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
    self.content.iter_mut().for_each(|item| match item {
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
    self
  }

  #[must_use]
  pub fn num_items(&self) -> usize {
    self.content.len()
  }

  #[must_use]
  pub fn num_timeslices(&self) -> usize {
    self
      .content
      .iter()
      .map(|item| match item {
        PhraseContent::Note(_) | PhraseContent::Chord(_) => 1,
        PhraseContent::Phrase(phrase) => phrase.num_timeslices(),
        PhraseContent::MultiVoice(multivoice) => multivoice.num_timeslices(),
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
          details.modifications.push(modification.r#type);
        });
        if let Some(previous_timeslice) = timeslices.last_mut() {
          previous_timeslice.content.iter_mut().for_each(|note| {
            note.phrase_details.iter_mut().for_each(|details| {
              details.next_pitch = content.note.pitch;
              details.next_accidental = content.note.accidental;
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

  pub fn iter_mut(&mut self) -> IterMut<'_, PhraseContent> {
    self.content.iter_mut()
  }

  pub fn iter_modifications(&self) -> Iter<'_, PhraseModification> {
    self.modifications.iter()
  }

  pub fn iter_modifications_mut(&mut self) -> IterMut<'_, PhraseModification> {
    self.modifications.iter_mut()
  }

  #[must_use]
  pub fn iter_timeslices(&self) -> Vec<Timeslice> {
    let mut index = 0;
    let mut timeslices = Vec::new();
    let num_timeslices = self.num_timeslices();
    self.content.iter().for_each(|item| match item {
      PhraseContent::Note(note) => {
        let mut timeslice = Timeslice::new();
        timeslice.add_note(note.clone());
        index = self.update_timeslice_details(&mut timeslices, timeslice, index, num_timeslices);
      }
      PhraseContent::Chord(chord) => {
        let timeslice = chord.to_timeslice();
        index = self.update_timeslice_details(&mut timeslices, timeslice, index, num_timeslices);
      }
      PhraseContent::Phrase(phrase) => {
        phrase.iter_timeslices().into_iter().for_each(|timeslice| {
          index = self.update_timeslice_details(&mut timeslices, timeslice, index, num_timeslices);
        });
      }
      PhraseContent::MultiVoice(multivoice) => {
        multivoice.iter_timeslices().into_iter().for_each(|timeslice| {
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

#[cfg(feature = "print")]
impl core::fmt::Display for Phrase {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let mods = self
      .modifications
      .iter()
      .map(ToString::to_string)
      .collect::<Vec<String>>()
      .join(", ");
    let items = self
      .content
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
  use crate::{DurationType, PitchName};

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
