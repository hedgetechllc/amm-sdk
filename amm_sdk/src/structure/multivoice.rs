use super::{
  chord::{Chord, ChordContent},
  phrase::{Phrase, PhraseContent, PhraseTimesliceIter},
};
use crate::context::{generate_id, Tempo};
use crate::modification::PhraseModificationType;
use crate::note::{Duration, DurationType, Note};
use crate::temporal::Timeslice;
use alloc::collections::VecDeque;
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};

#[derive(Clone, Debug, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum MultiVoiceContent {
  Phrase(Phrase),
}

#[derive(Debug, Default, Eq, JsonDeserialize, JsonSerialize)]
pub struct MultiVoice {
  id: usize,
  content: Vec<MultiVoiceContent>,
}

impl MultiVoice {
  #[must_use]
  pub fn new() -> Self {
    Self {
      id: generate_id(),
      content: Vec::new(),
    }
  }

  #[must_use]
  pub(crate) fn simplify(&mut self) -> Option<Phrase> {
    self.content.retain_mut(|MultiVoiceContent::Phrase(phrase)| {
      phrase.simplify();
      !phrase.is_empty()
    });
    if self.content.len() == 1 {
      self.content.pop().map(|MultiVoiceContent::Phrase(phrase)| phrase)
    } else {
      None
    }
  }

  fn flatten_tuplets(timeslices: VecDeque<(f64, Vec<PhraseContent>)>, beat_base_note: Duration) -> Phrase {
    let mut phrase = Phrase::new();
    let (mut tuplet_end_time, mut tuplet_ratio) = (0.0, Some(3.0 / 2.0));
    if let Some((_, slice_content)) = timeslices.front() {
      // Pre-calculate required tuplet statistics
      let tuplet_phrases = slice_content
        .iter()
        .filter_map(|content| match content {
          PhraseContent::Phrase(phrase) => Some((phrase.get_beats(&beat_base_note, None), phrase)),
          _ => None,
        })
        .collect::<Vec<_>>();
      let (longest_duration, longest_tuplet) = unsafe {
        *tuplet_phrases
          .iter()
          .max_by(|(beats1, _), (beats2, _)| beats1.partial_cmp(beats2).unwrap_unchecked())
          .unwrap_unchecked()
      };
      let target_type = unsafe {
        longest_tuplet
          .iter_modifications()
          .find(|modification| matches!(modification.r#type, PhraseModificationType::Tuplet { .. }))
          .unwrap_unchecked()
          .r#type
      };
      tuplet_ratio = Some(
        if let PhraseModificationType::Tuplet { num_beats, into_beats } = target_type {
          f64::from(into_beats) / f64::from(num_beats)
        } else {
          1.0
        },
      );
      tuplet_end_time = longest_duration;

      // Check for various tuplet situations
      if slice_content.iter().any(|content| match content {
        PhraseContent::Phrase(phrase) => phrase.is_nested_tuplet(),
        _ => false,
      }) {
        // Nested tuplet present, simply return the longest tuplet phrase
        return longest_tuplet.clone();
      } else if tuplet_phrases.len() > 1 {
        // Combine all phrases which are tuplets of the same type, duration, and number of notes
        let target_num_notes = longest_tuplet.num_items();
        phrase.add_modification(target_type);
        let mut phrase_chords = Vec::new();
        for (_, tuplet) in tuplet_phrases.into_iter().filter(|(tuplet_duration, tuplet)| {
          (longest_duration - *tuplet_duration).abs() < 0.000_001
            && target_num_notes == tuplet.num_items()
            && tuplet
              .iter_modifications()
              .any(|modification| modification.r#type == target_type)
        }) {
          for (idx, item) in tuplet.into_iter().enumerate() {
            let chord = if let Some(chord) = phrase_chords.get_mut(idx) {
              chord
            } else {
              phrase.add_chord()
            };
            match item {
              PhraseContent::Note(note) => {
                chord.claim_note(note.clone());
              }
              PhraseContent::Chord(sub_chord) => {
                sub_chord.iter_modifications().for_each(|modification| {
                  chord.add_modification(modification.r#type);
                });
                for ChordContent::Note(note) in sub_chord {
                  chord.claim_note(note.clone());
                }
              }
              _ => unsafe { core::hint::unreachable_unchecked() },
            }
          }
        }
      } else {
        // Only one tuplet phrase to contend with
        phrase = longest_tuplet.clone();
      }
    }

    // Fill in the list of valid times for each item in the tuplet
    let mut curr_time = 0.0;
    let mut valid_times_reversed = phrase
      .iter()
      .enumerate()
      .map(|(idx, content)| match content {
        PhraseContent::Chord(chord) => {
          let start_time = curr_time;
          curr_time += chord.get_beats(&beat_base_note, tuplet_ratio);
          (idx, start_time, true)
        }
        PhraseContent::Note(note) => {
          let start_time = curr_time;
          curr_time += note.get_beats(&beat_base_note, tuplet_ratio);
          (idx, start_time, false)
        }
        _ => (0, 0.0, false),
      })
      .collect::<Vec<(usize, f64, bool)>>();
    valid_times_reversed.reverse();

    // Deal with reminder of the timeslices
    for (slice_time, slice_content) in timeslices {
      if let Some((tuplet_item_idx, tuplet_start_time, is_chord)) = valid_times_reversed
        .iter_mut()
        .find(|(_, start_time, _)| slice_time >= *start_time)
      {
        // Ensure that the tuplet item to add to is a chord
        if !*is_chord {
          let mut chord = Chord::new();
          if let Some(tuplet_content) = phrase.content.get_mut(*tuplet_item_idx) {
            if let PhraseContent::Note(note) = tuplet_content {
              chord.claim_note(note.clone());
            }
            let mut chord_content = PhraseContent::Chord(chord);
            core::mem::swap(&mut chord_content, tuplet_content);
            *is_chord = true;
          }
        }

        // Get the chord to add the content to and its current duration
        let Some(PhraseContent::Chord(tuplet_chord)) = phrase.content.get_mut(*tuplet_item_idx) else {
          unsafe { core::hint::unreachable_unchecked() }
        };
        let current_tuplet_beats = tuplet_chord.get_beats(&beat_base_note, tuplet_ratio);
        let current_tuplet_duration = unsafe {
          tuplet_chord
            .iter()
            .next()
            .map(|ChordContent::Note(note)| note.duration)
            .unwrap_unchecked()
        };

        // Add the content to the current tuplet chord item
        for content in slice_content {
          match content {
            PhraseContent::Chord(chord) => {
              for ChordContent::Note(mut note) in chord {
                while *tuplet_start_time + note.get_beats(&beat_base_note, tuplet_ratio) > tuplet_end_time {
                  note.duration = note.duration.split(2);
                }
                if note.get_beats(&beat_base_note, tuplet_ratio) < current_tuplet_beats {
                  note.duration = current_tuplet_duration;
                }
                tuplet_chord.claim_note(note);
              }
            }
            PhraseContent::Note(mut note) => {
              while *tuplet_start_time + note.get_beats(&beat_base_note, tuplet_ratio) > tuplet_end_time {
                note.duration = note.duration.split(2);
              }
              if note.get_beats(&beat_base_note, tuplet_ratio) < current_tuplet_beats {
                note.duration = current_tuplet_duration;
              }
              tuplet_chord.claim_note(note);
            }
            _ => (),
          }
        }
      }
    }
    phrase
  }

  #[must_use]
  pub fn flatten(&self) -> Phrase {
    // Note: Loses any modifications on contained phrases
    // Flatten all multi-voice content into individual phrases containing only notes, chords, and tuplets
    let beat_base_note = Duration::new(DurationType::TwoThousandFortyEighth, 0);
    let phrases: Vec<Phrase> = self
      .iter()
      .map(|MultiVoiceContent::Phrase(phrase)| phrase.flatten(true))
      .collect();

    // Place all resulting phrase content into temporally ordered timeslices
    let mut timeslices: Vec<(f64, Vec<PhraseContent>)> = Vec::new();
    for phrase in phrases {
      let (mut index, mut curr_time) = (0, 0.0);
      for item in phrase {
        let slice_duration = match &item {
          PhraseContent::Note(note) => note.get_beats(&beat_base_note, None),
          PhraseContent::Chord(chord) => chord.get_beats(&beat_base_note, None),
          PhraseContent::Phrase(phrase) => phrase.get_beats(&beat_base_note, None),
          PhraseContent::MultiVoice(_) => unsafe { core::hint::unreachable_unchecked() },
        };
        if let Some(slice_details) = timeslices.get_mut(index) {
          let (mut slice_time, mut existing_slice) = (slice_details.0, &mut slice_details.1);
          while curr_time > slice_time && curr_time - slice_time > 0.000_001 {
            index += 1;
            (slice_time, existing_slice) = if let Some((start_time, slice)) = timeslices.get_mut(index) {
              (*start_time, slice)
            } else {
              unsafe {
                timeslices.push((curr_time, Vec::new()));
                let (start_time, slice) = timeslices.last_mut().unwrap_unchecked();
                (*start_time, slice)
              }
            };
          }
          if (slice_time - curr_time).abs() < 0.000_001 {
            existing_slice.push(item);
          } else {
            timeslices.insert(index, (curr_time, vec![item]));
          }
        } else {
          timeslices.push((curr_time, vec![item]));
        }
        curr_time += slice_duration;
        index += 1;
      }
    }

    // Combine the content of each timeslice into a new phrase, taking special care of tuplets
    let mut phrase = Phrase::new();
    let (mut tuplet_slices, mut tuplet_end) = (VecDeque::new(), 0.0);
    for (slice_time, mut slice_content) in timeslices {
      if !tuplet_slices.is_empty() {
        if slice_time < tuplet_end {
          tuplet_slices.push_back((slice_time, slice_content));
          continue;
        }
        phrase.content.push(PhraseContent::Phrase(Self::flatten_tuplets(
          tuplet_slices,
          beat_base_note,
        )));
        tuplet_slices = VecDeque::new();
      }
      if slice_content.len() > 1 {
        if let Some(tuplet) = slice_content.iter().find_map(|item| match item {
          PhraseContent::Phrase(phrase) => Some(phrase),
          _ => None,
        }) {
          tuplet_end = slice_time + tuplet.get_beats(&beat_base_note, None);
          tuplet_slices.push_back((slice_time, slice_content));
        } else {
          let combined = phrase.add_chord();
          for content in slice_content {
            match content {
              PhraseContent::Note(note) => {
                let new_note = combined.add_note(note.pitch, note.duration, Some(note.accidental));
                note.iter_modifications().for_each(|modification| {
                  new_note.add_modification(modification.r#type);
                });
              }
              PhraseContent::Chord(chord) => {
                chord.iter_modifications().for_each(|modification| {
                  combined.add_modification(modification.r#type);
                });
                for ChordContent::Note(note) in chord {
                  let new_note = combined.add_note(note.pitch, note.duration, Some(note.accidental));
                  note.iter_modifications().for_each(|modification| {
                    new_note.add_modification(modification.r#type);
                  });
                }
              }
              _ => unsafe { core::hint::unreachable_unchecked() },
            }
          }
        }
      } else if let Some(content) = slice_content.pop() {
        phrase.content.push(content);
      }
    }
    phrase
  }

  #[must_use]
  pub const fn get_id(&self) -> usize {
    self.id
  }

  pub fn add_phrase(&mut self) -> &mut Phrase {
    self.content.push(MultiVoiceContent::Phrase(Phrase::new()));
    match self.content.last_mut() {
      Some(MultiVoiceContent::Phrase(phrase)) => phrase,
      None => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn claim(&mut self, item: MultiVoiceContent) -> &mut Self {
    self.content.push(item);
    self
  }

  pub fn claim_phrase(&mut self, phrase: Phrase) -> &mut Phrase {
    self.content.push(MultiVoiceContent::Phrase(phrase));
    match self.content.last_mut() {
      Some(MultiVoiceContent::Phrase(phrase)) => phrase,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  #[must_use]
  pub fn get_phrase(&self, id: usize) -> Option<&Phrase> {
    self
      .iter()
      .find_map(|MultiVoiceContent::Phrase(phrase)| phrase.get_phrase(id))
  }

  #[must_use]
  pub fn get_phrase_mut(&mut self, id: usize) -> Option<&mut Phrase> {
    self
      .iter_mut()
      .find_map(|MultiVoiceContent::Phrase(phrase)| phrase.get_phrase_mut(id))
  }

  #[must_use]
  pub fn get_multivoice(&self, id: usize) -> Option<&MultiVoice> {
    if self.id == id {
      Some(self)
    } else {
      self
        .iter()
        .find_map(|MultiVoiceContent::Phrase(phrase)| phrase.get_multivoice(id))
    }
  }

  #[must_use]
  pub fn get_multivoice_mut(&mut self, id: usize) -> Option<&mut MultiVoice> {
    if self.id == id {
      Some(self)
    } else {
      self
        .iter_mut()
        .find_map(|MultiVoiceContent::Phrase(phrase)| phrase.get_multivoice_mut(id))
    }
  }

  #[must_use]
  pub fn get_chord(&self, id: usize) -> Option<&Chord> {
    self
      .iter()
      .find_map(|MultiVoiceContent::Phrase(phrase)| phrase.get_chord(id))
  }

  #[must_use]
  pub fn get_chord_mut(&mut self, id: usize) -> Option<&mut Chord> {
    self
      .iter_mut()
      .find_map(|MultiVoiceContent::Phrase(phrase)| phrase.get_chord_mut(id))
  }

  #[must_use]
  pub fn get_note(&self, id: usize) -> Option<&Note> {
    self
      .iter()
      .find_map(|MultiVoiceContent::Phrase(phrase)| phrase.get_note(id))
  }

  #[must_use]
  pub fn get_note_mut(&mut self, id: usize) -> Option<&mut Note> {
    self
      .iter_mut()
      .find_map(|MultiVoiceContent::Phrase(phrase)| phrase.get_note_mut(id))
  }

  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration, tuplet_ratio: Option<f64>) -> f64 {
    self
      .iter()
      .map(|MultiVoiceContent::Phrase(phrase)| phrase.get_beats(beat_base, tuplet_ratio))
      .reduce(f64::max)
      .unwrap_or_default()
  }

  #[must_use]
  pub fn get_duration(&self, tempo: &Tempo, tuplet_ratio: Option<f64>) -> f64 {
    self.get_beats(&tempo.base_note, tuplet_ratio) * 60.0 / f64::from(tempo.beats_per_minute)
  }

  pub fn remove_item(&mut self, id: usize) -> &mut Self {
    self.content.retain(|item| match item {
      MultiVoiceContent::Phrase(phrase) => phrase.get_id() != id,
    });
    self.iter_mut().for_each(|MultiVoiceContent::Phrase(phrase)| {
      phrase.remove_item(id);
    });
    self
  }

  pub fn remove_modification(&mut self, id: usize) -> &mut Self {
    self.iter_mut().for_each(|MultiVoiceContent::Phrase(phrase)| {
      phrase.remove_modification(id);
    });
    self
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
    self.iter_timeslices().count()
  }

  pub fn iter(&self) -> core::slice::Iter<'_, MultiVoiceContent> {
    self.content.iter()
  }

  pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, MultiVoiceContent> {
    self.content.iter_mut()
  }

  #[must_use]
  pub fn iter_timeslices(&self) -> MultiVoiceTimesliceIter<'_> {
    MultiVoiceTimesliceIter {
      base_duration: Duration::new(DurationType::TwoThousandFortyEighth, 0),
      phrase_iterators: self
        .iter()
        .map(|MultiVoiceContent::Phrase(phrase)| {
          let mut iter = phrase.iter_timeslices();
          let next = iter.next();
          (0.0, iter, next)
        })
        .collect(),
    }
  }
}

impl IntoIterator for MultiVoice {
  type Item = MultiVoiceContent;
  type IntoIter = alloc::vec::IntoIter<Self::Item>;
  fn into_iter(self) -> Self::IntoIter {
    self.content.into_iter()
  }
}

impl<'a> IntoIterator for &'a MultiVoice {
  type Item = &'a MultiVoiceContent;
  type IntoIter = core::slice::Iter<'a, MultiVoiceContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut MultiVoice {
  type Item = &'a mut MultiVoiceContent;
  type IntoIter = core::slice::IterMut<'a, MultiVoiceContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

impl Clone for MultiVoice {
  fn clone(&self) -> Self {
    Self {
      id: generate_id(),
      content: self.content.clone(),
    }
  }
}

impl PartialEq for MultiVoice {
  fn eq(&self, other: &Self) -> bool {
    self.content == other.content
  }
}

pub struct MultiVoiceTimesliceIter<'a> {
  base_duration: Duration,
  phrase_iterators: Vec<(f64, PhraseTimesliceIter<'a>, Option<Timeslice>)>,
}

impl Iterator for MultiVoiceTimesliceIter<'_> {
  type Item = Timeslice;
  fn next(&mut self) -> Option<Self::Item> {
    let mut next_start_time = f64::MAX;
    let mut timeslice: Option<Timeslice> = None;
    self
      .phrase_iterators
      .iter_mut()
      .for_each(|(next_time, iterator, next_item)| {
        if next_time.abs() <= 0.000_001 {
          if let Some(mut slice) = next_item.take() {
            *next_item = iterator.next();
            *next_time = slice.get_beats(&self.base_duration);
            if *next_time < next_start_time && next_item.is_some() {
              next_start_time = *next_time;
            }
            if let Some(timeslice) = timeslice.as_mut() {
              timeslice.combine_with(&mut slice);
            } else {
              timeslice = Some(slice);
            }
          }
        } else if *next_time >= 0.0 && *next_time < next_start_time {
          next_start_time = *next_time;
        }
      });
    if timeslice.is_some() {
      self.phrase_iterators.iter_mut().for_each(|(next_time, _, _)| {
        *next_time -= next_start_time;
      });
    }
    timeslice
  }
}

impl core::iter::FusedIterator for MultiVoiceTimesliceIter<'_> {}

#[cfg(feature = "print")]
impl core::fmt::Display for MultiVoice {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let voices = self
      .iter()
      .map(|MultiVoiceContent::Phrase(phrase)| phrase.to_string())
      .collect::<Vec<_>>()
      .join(", ");
    write!(f, "MultiVoice: [{voices}]")
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::modification::NoteModificationType;
  use crate::note::{Accidental, Pitch, PitchName};

  #[test]
  fn test_flatten_normal_to_tuplet() {
    let mut multivoice = MultiVoice::new();
    let mut phrase = multivoice.add_phrase();
    phrase.add_modification(PhraseModificationType::Tuplet {
      num_beats: 3,
      into_beats: 2,
    });
    phrase.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    phrase.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Eighth, 0),
      Some(Accidental::Sharp),
    );
    phrase.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    phrase = multivoice.add_phrase();
    phrase.add_note(
      Pitch::new(PitchName::E, 4),
      Duration::new(DurationType::Quarter, 0),
      None,
    );
    let mut flattened = Phrase::new();
    flattened.add_modification(PhraseModificationType::Tuplet {
      num_beats: 3,
      into_beats: 2,
    });
    let mut chord = flattened.add_chord();
    chord.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    chord
      .add_note(
        Pitch::new(PitchName::E, 4),
        Duration::new(DurationType::Quarter, 0),
        None,
      )
      .add_modification(NoteModificationType::Tie);
    flattened.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Eighth, 0),
      Some(Accidental::Sharp),
    );
    chord = flattened.add_chord();
    chord.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    chord.add_note(
      Pitch::new(PitchName::E, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    assert_eq!(flattened, multivoice.flatten());
    //println!("MultiVoice Normal to Tuplet: {}", multivoice.flatten());
  }

  #[test]
  fn test_flatten_tuplet_to_normal() {
    let mut multivoice = MultiVoice::new();
    let mut phrase = multivoice.add_phrase();
    phrase.add_modification(PhraseModificationType::Tuplet {
      num_beats: 3,
      into_beats: 2,
    });
    phrase.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    phrase.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Eighth, 0),
      Some(Accidental::Sharp),
    );
    phrase.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    phrase = multivoice.add_phrase();
    phrase.add_note(
      Pitch::new(PitchName::E, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    phrase.add_note(
      Pitch::new(PitchName::E, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    let mut flattened = Phrase::new();
    let chord = flattened.add_chord();
    chord.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Quarter, 0),
      None,
    );
    chord.add_note(
      Pitch::new(PitchName::E, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    flattened.add_note(
      Pitch::new(PitchName::E, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    assert_eq!(flattened, multivoice.flatten());
    //println!("MultiVoice Tuplet to Normal: {}", multivoice.flatten());
  }

  #[test]
  fn test_flatten_tuplet_to_tuplet() {
    let mut multivoice = MultiVoice::new();
    let mut phrase = multivoice.add_phrase();
    phrase.add_modification(PhraseModificationType::Tuplet {
      num_beats: 3,
      into_beats: 2,
    });
    phrase.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    phrase.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Eighth, 0),
      Some(Accidental::Sharp),
    );
    phrase.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    phrase = multivoice.add_phrase();
    phrase.add_modification(PhraseModificationType::Tuplet {
      num_beats: 3,
      into_beats: 2,
    });
    phrase.add_note(
      Pitch::new(PitchName::E, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    phrase.add_note(
      Pitch::new(PitchName::F, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    phrase.add_note(
      Pitch::new(PitchName::E, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    let mut flattened = Phrase::new();
    flattened.add_modification(PhraseModificationType::Tuplet {
      num_beats: 3,
      into_beats: 2,
    });
    let mut chord = flattened.add_chord();
    chord.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    chord.add_note(
      Pitch::new(PitchName::E, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    chord = flattened.add_chord();
    chord.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Eighth, 0),
      Some(Accidental::Sharp),
    );
    chord.add_note(
      Pitch::new(PitchName::F, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    chord = flattened.add_chord();
    chord.add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    chord.add_note(
      Pitch::new(PitchName::E, 4),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    assert_eq!(flattened, multivoice.flatten());
    //println!("MultiVoice Tuplet to Tuplet: {}", multivoice.flatten());
  }

  #[test]
  fn test_flatten_monstrosity() {
    let mut multivoice = MultiVoice::new();
    {
      let phrase1 = multivoice.add_phrase();
      phrase1.add_modification(PhraseModificationType::Tuplet {
        num_beats: 3,
        into_beats: 2,
      });
      phrase1.add_note(
        Pitch::new(PitchName::C, 4),
        Duration::new(DurationType::Eighth, 0),
        None,
      );
      phrase1.add_note(
        Pitch::new(PitchName::C, 4),
        Duration::new(DurationType::Eighth, 0),
        Some(Accidental::Sharp),
      );
      phrase1.add_note(
        Pitch::new(PitchName::C, 4),
        Duration::new(DurationType::Eighth, 0),
        None,
      );
    }
    {
      let phrase2 = multivoice.add_phrase();
      phrase2.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Whole, 1), None);
    }
    {
      let phrase3 = multivoice.add_phrase();
      let multivoice2 = phrase3.add_multivoice();
      {
        let phrase3 = multivoice2.add_phrase();
        phrase3.add_note(
          Pitch::new(PitchName::A, 4),
          Duration::new(DurationType::Sixteenth, 0),
          None,
        );
      }
      {
        let phrase4 = multivoice2.add_phrase();
        phrase4.add_note(Pitch::new(PitchName::B, 4), Duration::new(DurationType::Half, 0), None);
        phrase4.add_note(
          Pitch::new(PitchName::B, 4),
          Duration::new(DurationType::Half, 0),
          Some(Accidental::Flat),
        );
      }
    }
    let mut flattened = Phrase::new();
    let chord = flattened.add_chord();
    chord.add_note(Pitch::new(PitchName::C, 4), Duration::new(DurationType::Half, 0), None);
    chord.add_note(Pitch::new(PitchName::E, 4), Duration::new(DurationType::Whole, 1), None);
    chord.add_note(Pitch::new(PitchName::A, 4), Duration::new(DurationType::Half, 0), None);
    chord.add_note(Pitch::new(PitchName::B, 4), Duration::new(DurationType::Half, 0), None);
    flattened.add_note(
      Pitch::new(PitchName::B, 4),
      Duration::new(DurationType::Half, 0),
      Some(Accidental::Flat),
    );
    assert_eq!(flattened, multivoice.flatten());
    //println!("MultiVoice Flatten: {}", multivoice.flatten());
  }
}
