use super::{
  chord::{Chord, ChordContent},
  phrase::{Phrase, PhraseContent},
  place_and_merge_timeslice,
  timeslice::Timeslice,
};
use crate::context::{generate_id, Tempo};
use crate::modification::{ChordModificationType, NoteModificationType, PhraseModification, PhraseModificationType};
use crate::note::{Duration, DurationType, Note};
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};
use core::slice::Iter;

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
  #[allow(clippy::too_many_lines)]
  pub fn flatten(&self) -> Phrase {
    // Note: Loses any modifications on contained phrases
    // Flatten all multi-voice content into individual phrases containing only notes and chords
    let beat_base_note = Duration::new(DurationType::SixtyFourth, 0);
    let phrases: Vec<Phrase> = self
      .content
      .iter()
      .map(|item| match item {
        MultiVoiceContent::Phrase(phrase) => phrase.flatten(true),
      })
      .collect();

    // Determine which phrases contain tuplets
    let tuplet_phrases: Vec<(&Phrase, &PhraseModification, (u8, u8))> = phrases
      .iter()
      .filter_map(|phrase| {
        phrase
          .iter_modifications()
          .find_map(|modification| match modification.r#type {
            PhraseModificationType::Tuplet { num_beats, into_beats } => {
              Some((phrase, modification, (num_beats, into_beats)))
            }
            _ => None,
          })
      })
      .collect();

    // Merge according to the tuplet content of all phrases
    let to_combine = if tuplet_phrases.is_empty() {
      // No tuplets, just use all phrases in their current form
      phrases
    } else if tuplet_phrases.len() == 1 {
      // Single tuplet, check if all other phrases are single-note or single-chord phrases with durations equal to or less than the tuplet
      let (tuplet_phrase, tuplet_modification) = (tuplet_phrases[0].0, tuplet_phrases[0].1);
      let tuplet_duration = tuplet_phrase.get_beats(&beat_base_note, None);
      let (num_beats, into_beats) = tuplet_phrases[0].2;
      if phrases.iter().all(|phrase| {
        (tuplet_phrase.get_id() == phrase.get_id())
          || (phrase.content.len() == 1 && phrase.get_beats(&beat_base_note, None) <= tuplet_duration)
      }) {
        // Combine all phrases into a single tuplet phrase by converting single-note durations into their tuplet equivalents
        phrases
          .iter()
          .map(|phrase| {
            let mut updated_phrase = phrase.clone();
            if phrase.get_id() != tuplet_phrase.get_id() {
              let num_additional_notes = num_beats - into_beats;
              updated_phrase.add_modification(tuplet_modification.r#type);
              unsafe {
                match &updated_phrase.content.pop().unwrap_unchecked() {
                  PhraseContent::Note(note) => {
                    let updated_duration = note.duration.split(into_beats);
                    let mut new_note = updated_phrase.add_note(note.pitch, note.duration, Some(note.accidental));
                    new_note.add_modification(NoteModificationType::Tie);
                    note.iter_modifications().for_each(|modification| {
                      new_note.add_modification(modification.r#type);
                    });
                    for index in 0..num_additional_notes {
                      new_note = updated_phrase.add_note(note.pitch, updated_duration, Some(note.accidental));
                      if index + 1 < num_additional_notes {
                        new_note.add_modification(NoteModificationType::Tie);
                      }
                    }
                  }
                  PhraseContent::Chord(chord) => {
                    let updated_duration = chord
                      .iter()
                      .map(|item| match item {
                        ChordContent::Note(note) => note.duration.split(into_beats),
                      })
                      .reduce(|min, duration| if min.value() < duration.value() { min } else { duration })
                      .unwrap_or(Duration::new(DurationType::Eighth, 0));
                    let chord_notes = chord
                      .iter()
                      .map(|item| match item {
                        ChordContent::Note(note) => note,
                      })
                      .collect::<Vec<_>>();
                    let mut new_chord = updated_phrase.add_chord();
                    new_chord.add_modification(ChordModificationType::Tie);
                    chord.iter_modifications().for_each(|modification| {
                      new_chord.add_modification(modification.r#type);
                    });
                    for index in 0..num_additional_notes {
                      new_chord = updated_phrase.add_chord();
                      for note in &chord_notes {
                        new_chord.add_note(note.pitch, updated_duration, Some(note.accidental));
                      }
                      if index + 1 < num_additional_notes {
                        new_chord.add_modification(ChordModificationType::Tie);
                      }
                    }
                  }
                  _ => core::hint::unreachable_unchecked(),
                }
              }
            }
            updated_phrase
          })
          .collect()
      } else {
        // Combine all phrases into a single phrase by converting the tuplet into a series of notes with the target composite duration
        phrases
          .iter()
          .map(|phrase| {
            // Expand tuplet by only taking the first note and holding it for the full tuplet duration
            unsafe {
              let mut updated_phrase = phrase.clone();
              if phrase.get_id() == tuplet_phrase.get_id() {
                let target_duration =
                  Duration::from_beats(&beat_base_note, updated_phrase.get_beats(&beat_base_note, None));
                match updated_phrase.content.first_mut().unwrap_unchecked() {
                  PhraseContent::Note(note) => {
                    note.duration = target_duration;
                  }
                  PhraseContent::Chord(chord) => {
                    chord.content.iter_mut().for_each(|item| match item {
                      ChordContent::Note(note) => {
                        note.duration = target_duration;
                      }
                    });
                  }
                  _ => core::hint::unreachable_unchecked(),
                };
                let mod_id = updated_phrase
                  .iter_modifications()
                  .find_map(|modification| {
                    if matches!(modification.r#type, PhraseModificationType::Tuplet { .. }) {
                      Some(modification.get_id())
                    } else {
                      None
                    }
                  })
                  .unwrap_or_default();
                updated_phrase.remove_modification(mod_id);
                updated_phrase.content.drain(1..);
              }
              updated_phrase
            }
          })
          .collect()
      }
    } else {
      // Multiple tuplets, check if all other phrases are tuplets with the same number of notes and target beat durations
      let (target_num_beats, target_into_beats) = tuplet_phrases[0].2;
      let target_duration = tuplet_phrases[0].0.get_beats(&beat_base_note, None);
      if tuplet_phrases.len() == phrases.len()
        && phrases.iter().all(|phrase| {
          (phrase.get_beats(&beat_base_note, None) - target_duration).abs() < f64::EPSILON
            && match phrase
              .iter_modifications()
              .find_map(|modification| match modification.r#type {
                PhraseModificationType::Tuplet { num_beats, into_beats } => Some((num_beats, into_beats)),
                _ => None,
              }) {
              Some((num_beats, into_beats)) => num_beats == target_num_beats && into_beats == target_into_beats,
              None => false,
            }
        })
      {
        phrases
      } else {
        // Combine all phrases into a single phrase by converting each tuplet into a series of notes with the target composite duration
        phrases
          .iter()
          .map(|phrase| {
            let mut updated_phrase = phrase.clone();
            phrase
              .iter_modifications()
              .for_each(|modification| match modification.r#type {
                PhraseModificationType::Tuplet {
                  num_beats: _,
                  into_beats: _,
                } => {
                  // Expand tuplet by only taking the first note and holding it for the full tuplet duration
                  unsafe {
                    let target_duration =
                      Duration::from_beats(&beat_base_note, updated_phrase.get_beats(&beat_base_note, None));
                    match updated_phrase.content.first_mut().unwrap_unchecked() {
                      PhraseContent::Note(note) => {
                        note.duration = target_duration;
                      }
                      PhraseContent::Chord(chord) => {
                        chord.content.iter_mut().for_each(|item| match item {
                          ChordContent::Note(note) => {
                            note.duration = target_duration;
                          }
                        });
                      }
                      _ => core::hint::unreachable_unchecked(),
                    };
                    updated_phrase.remove_modification(modification.get_id());
                    updated_phrase.content.drain(1..);
                  }
                }
                _ => (),
              });
            updated_phrase
          })
          .collect()
      }
    };

    // Create an ordered list of timeslices containing all notes and chords across all voices
    let mut phrase = unsafe { to_combine.first().unwrap_unchecked().clone() };
    let mut timeslices: Vec<(f64, Vec<PhraseContent>)> = Vec::new();
    for phrase in to_combine {
      let (mut index, mut curr_time) = (0, 0.0);
      let tuplet_ratio = phrase
        .iter_modifications()
        .find_map(|modification| match modification.r#type {
          PhraseModificationType::Tuplet { num_beats, into_beats } => {
            Some(f64::from(into_beats) / f64::from(num_beats))
          }
          _ => None,
        });
      for item in phrase {
        let slice_duration = match &item {
          PhraseContent::Note(note) => note.get_beats(&beat_base_note, tuplet_ratio),
          PhraseContent::Chord(chord) => chord.get_beats(&beat_base_note, tuplet_ratio),
          _ => unsafe { core::hint::unreachable_unchecked() },
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

    // Create a new single phrase containing all notes and chords
    phrase.content.clear();
    for (idx, (start_time, content)) in timeslices.iter().enumerate() {
      if content.len() > 1 {
        let target_beats = if idx + 1 < timeslices.len() {
          timeslices[idx + 1].0 - start_time
        } else {
          content
            .iter()
            .map(|item| match item {
              PhraseContent::Note(note) => note.get_beats(&beat_base_note, None),
              PhraseContent::Chord(chord) => chord.get_beats(&beat_base_note, None),
              _ => unsafe { core::hint::unreachable_unchecked() },
            })
            .reduce(f64::max)
            .unwrap_or_default()
        };
        let target_duration = Duration::from_beats(&beat_base_note, target_beats);
        let combined = phrase.add_chord();
        content.iter().for_each(|item| match item {
          PhraseContent::Note(note) => {
            let duration = if note.duration.value() < target_duration.value() {
              target_duration
            } else {
              note.duration
            };
            let new_note = combined.add_note(note.pitch, duration, Some(note.accidental));
            note.iter_modifications().for_each(|modification| {
              new_note.add_modification(modification.r#type);
            });
          }
          PhraseContent::Chord(chord) => {
            chord.iter_modifications().for_each(|modification| {
              combined.add_modification(modification.r#type);
            });
            chord.iter().for_each(|item| match item {
              ChordContent::Note(note) => {
                let duration = if note.duration.value() < target_duration.value() {
                  target_duration
                } else {
                  note.duration
                };
                let new_note = combined.add_note(note.pitch, duration, Some(note.accidental));
                note.iter_modifications().for_each(|modification| {
                  new_note.add_modification(modification.r#type);
                });
              }
            });
          }
          _ => unsafe { core::hint::unreachable_unchecked() },
        });
      } else if let Some(content) = content.first() {
        match content {
          PhraseContent::Note(note) => {
            phrase.content.push(PhraseContent::Note(note.clone()));
          }
          PhraseContent::Chord(chord) => {
            phrase.content.push(PhraseContent::Chord(chord.clone()));
          }
          _ => unsafe { core::hint::unreachable_unchecked() },
        }
      }
    }
    phrase
  }

  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn add_phrase(&mut self) -> &mut Phrase {
    self.content.push(MultiVoiceContent::Phrase(Phrase::new()));
    match self.content.last_mut() {
      Some(MultiVoiceContent::Phrase(phrase)) => phrase,
      None => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  #[must_use]
  pub fn get_phrase(&self, id: usize) -> Option<&Phrase> {
    self.content.iter().find_map(|item| match item {
      MultiVoiceContent::Phrase(phrase) if phrase.get_id() == id => Some(phrase),
      MultiVoiceContent::Phrase(phrase) => phrase.get_phrase(id),
    })
  }

  #[must_use]
  pub fn get_phrase_mut(&mut self, id: usize) -> Option<&mut Phrase> {
    self.content.iter_mut().find_map(|item| match item {
      MultiVoiceContent::Phrase(phrase) => {
        if phrase.get_id() == id {
          Some(phrase)
        } else {
          phrase.get_phrase_mut(id)
        }
      }
    })
  }

  #[must_use]
  pub fn get_multivoice(&self, id: usize) -> Option<&MultiVoice> {
    self.content.iter().find_map(|item| match item {
      MultiVoiceContent::Phrase(phrase) => phrase.get_multivoice(id),
    })
  }

  #[must_use]
  pub fn get_multivoice_mut(&mut self, id: usize) -> Option<&mut MultiVoice> {
    self.content.iter_mut().find_map(|item| match item {
      MultiVoiceContent::Phrase(phrase) => phrase.get_multivoice_mut(id),
    })
  }

  #[must_use]
  pub fn get_chord(&self, id: usize) -> Option<&Chord> {
    self.content.iter().find_map(|item| match item {
      MultiVoiceContent::Phrase(phrase) => phrase.get_chord(id),
    })
  }

  #[must_use]
  pub fn get_chord_mut(&mut self, id: usize) -> Option<&mut Chord> {
    self.content.iter_mut().find_map(|item| match item {
      MultiVoiceContent::Phrase(phrase) => phrase.get_chord_mut(id),
    })
  }

  #[must_use]
  pub fn get_note(&self, id: usize) -> Option<&Note> {
    self.content.iter().find_map(|item| match item {
      MultiVoiceContent::Phrase(phrase) => phrase.get_note(id),
    })
  }

  #[must_use]
  pub fn get_note_mut(&mut self, id: usize) -> Option<&mut Note> {
    self.content.iter_mut().find_map(|item| match item {
      MultiVoiceContent::Phrase(phrase) => phrase.get_note_mut(id),
    })
  }

  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration, tuplet_ratio: Option<f64>) -> f64 {
    self
      .content
      .iter()
      .map(|content| match &content {
        MultiVoiceContent::Phrase(phrase) => phrase.get_beats(beat_base, tuplet_ratio),
      })
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
    self.content.iter_mut().for_each(|item| match item {
      MultiVoiceContent::Phrase(phrase) => {
        phrase.remove_item(id);
      }
    });
    self
  }

  #[must_use]
  pub fn num_timeslices(&self) -> usize {
    self.iter_timeslices().len()
  }

  #[must_use]
  pub fn iter(&self) -> Iter<'_, MultiVoiceContent> {
    self.content.iter()
  }

  #[must_use]
  pub fn iter_timeslices(&self) -> Vec<Timeslice> {
    let mut timeslices: Vec<(f64, Timeslice)> = Vec::new();
    self.content.iter().for_each(|item| {
      let (mut index, mut curr_time) = (0, 0.0);
      for slice in match item {
        MultiVoiceContent::Phrase(phrase) => phrase.iter_timeslices(),
      } {
        (index, curr_time) = place_and_merge_timeslice(&mut timeslices, slice, index, curr_time);
      }
    });
    timeslices.into_iter().map(|(_, slice)| slice).collect()
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
  type IntoIter = Iter<'a, MultiVoiceContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
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

#[cfg(feature = "print")]
impl core::fmt::Display for MultiVoice {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let voices = self
      .content
      .iter()
      .map(|item| match item {
        MultiVoiceContent::Phrase(phrase) => phrase.to_string(),
      })
      .collect::<Vec<_>>()
      .join(", ");
    write!(f, "MultiVoice: [{voices}]")
  }
}

#[cfg(test)]
mod test {
  use super::*;
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
