use super::{
  chord::{Chord, ChordContent},
  phrase::{Phrase, PhraseContent},
  timeslice::Timeslice,
};
use crate::context::{generate_id, Tempo};
use crate::modification::{ChordModificationType, NoteModificationType, PhraseModification, PhraseModificationType};
use crate::note::{Duration, DurationType, Note};
use alloc::{rc::Rc, string::ToString, vec::Vec};
use core::{cell::RefCell, slice::Iter};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub enum MultiVoiceContent {
  Phrase(Rc<RefCell<Phrase>>),
}

#[derive(Clone)]
pub struct MultiVoice {
  pub(crate) id: usize,
  pub(crate) content: Vec<MultiVoiceContent>,
}

impl MultiVoice {
  #[must_use]
  pub fn new() -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: generate_id(),
      content: Vec::new(),
    }))
  }

  #[must_use]
  #[allow(clippy::too_many_lines)]
  pub fn flatten(&self) -> Rc<RefCell<Phrase>> {
    // Note: Loses any modifications on contained phrases
    // Flatten all multi-voice content into individual phrases containing only notes and chords
    let beat_base_note = Duration::new(DurationType::SixtyFourth, 0);
    let phrases: Vec<Rc<RefCell<Phrase>>> = self
      .content
      .iter()
      .map(|item| match item {
        MultiVoiceContent::Phrase(phrase) => phrase.borrow().flatten(true),
      })
      .collect();

    // Determine which phrases contain tuplets
    #[allow(clippy::type_complexity)]
    let tuplet_phrases: Vec<(Rc<RefCell<Phrase>>, Rc<RefCell<PhraseModification>>, usize, (u8, u8))> = phrases
      .iter()
      .filter_map(|phrase| {
        phrase
          .borrow()
          .modifications
          .iter()
          .find_map(|modification| match modification.borrow().get_modification() {
            PhraseModificationType::Tuplet { num_beats, into_beats } => Some((
              Rc::clone(phrase),
              Rc::clone(modification),
              modification.borrow().get_id(),
              (*num_beats, *into_beats),
            )),
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
      let (tuplet_phrase, tuplet_modification) = (&tuplet_phrases[0].0, &tuplet_phrases[0].1);
      let tuplet_duration = tuplet_phrase.borrow().get_beats(&beat_base_note, None);
      let (num_beats, into_beats) = tuplet_phrases[0].3;
      if phrases.iter().all(|phrase| {
        Rc::ptr_eq(tuplet_phrase, phrase)
          || (phrase.borrow().content.len() == 1 && phrase.borrow().get_beats(&beat_base_note, None) <= tuplet_duration)
      }) {
        // Combine all phrases into a single tuplet phrase by converting single-note durations into their tuplet equivalents
        phrases
          .iter()
          .map(|phrase| {
            let updated_phrase = Rc::new(RefCell::new(phrase.borrow().clone()));
            if !Rc::ptr_eq(phrase, tuplet_phrase) {
              let num_additional_notes = num_beats - into_beats;
              let mut updated_phrase = updated_phrase.borrow_mut();
              updated_phrase.modifications.push(Rc::clone(tuplet_modification));
              unsafe {
                match updated_phrase.content.pop().unwrap_unchecked() {
                  PhraseContent::Note(note) => {
                    let mut updated_note = Rc::new(RefCell::new(note.borrow().clone()));
                    let updated_duration = updated_note.borrow().duration.split(into_beats);
                    updated_note.borrow_mut().add_modification(NoteModificationType::Tie);
                    updated_phrase
                      .content
                      .push(PhraseContent::Note(Rc::clone(&updated_note)));
                    for index in 0..num_additional_notes {
                      let new_note = updated_phrase.add_note(updated_note.borrow().pitch, updated_duration, None);
                      updated_note = new_note;
                      if index + 1 < num_additional_notes {
                        updated_note.borrow_mut().add_modification(NoteModificationType::Tie);
                      }
                    }
                  }
                  PhraseContent::Chord(chord) => {
                    let mut updated_chord = Rc::new(RefCell::new(chord.borrow().clone()));
                    let updated_duration = updated_chord
                      .borrow()
                      .content
                      .iter()
                      .map(|item| match item {
                        ChordContent::Note(note) => note.borrow().duration.split(into_beats),
                      })
                      .reduce(|min, duration| if min.value() < duration.value() { min } else { duration })
                      .unwrap_or(Duration::new(DurationType::Eighth, 0));
                    updated_chord.borrow_mut().add_modification(ChordModificationType::Tie);
                    updated_phrase
                      .content
                      .push(PhraseContent::Chord(Rc::clone(&updated_chord)));
                    let chord_notes = updated_chord
                      .borrow()
                      .iter()
                      .map(|item| match item {
                        ChordContent::Note(note) => Rc::clone(note),
                      })
                      .collect::<Vec<_>>();
                    for index in 0..num_additional_notes {
                      updated_chord = updated_phrase.add_chord();
                      for note in &chord_notes {
                        updated_chord
                          .borrow_mut()
                          .add_note(note.borrow().pitch, updated_duration, None);
                      }
                      if index + 1 < num_additional_notes {
                        updated_chord.borrow_mut().add_modification(ChordModificationType::Tie);
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
            let updated_phrase = Rc::new(RefCell::new(phrase.borrow().clone()));
            if Rc::ptr_eq(phrase, tuplet_phrase) {
              let target_duration = Duration::from_beats(
                &beat_base_note,
                updated_phrase.borrow().get_beats(&beat_base_note, None),
              );
              let target_element = match &updated_phrase.borrow().content[0] {
                PhraseContent::Note(note) => {
                  let mut updated_note = note.borrow().clone();
                  updated_note.duration = target_duration;
                  PhraseContent::Note(Rc::new(RefCell::new(updated_note)))
                }
                PhraseContent::Chord(chord) => {
                  let mut updated_chord = chord.borrow().clone();
                  updated_chord.content.clear();
                  chord.borrow().content.iter().for_each(|item| match item {
                    ChordContent::Note(note) => {
                      let mut updated_note = note.borrow().clone();
                      updated_note.duration = target_duration;
                      updated_chord
                        .content
                        .push(ChordContent::Note(Rc::new(RefCell::new(updated_note))));
                    }
                  });
                  PhraseContent::Chord(Rc::new(RefCell::new(updated_chord)))
                }
                _ => unsafe { core::hint::unreachable_unchecked() },
              };
              updated_phrase.borrow_mut().remove_modification(tuplet_phrases[0].2);
              updated_phrase.borrow_mut().content.clear();
              updated_phrase.borrow_mut().content.push(target_element);
            }
            updated_phrase
          })
          .collect()
      }
    } else {
      // Multiple tuplets, check if all other phrases are tuplets with the same number of notes and target beat durations
      let (target_num_beats, target_into_beats) = tuplet_phrases[0].3;
      let target_duration = tuplet_phrases[0].0.borrow().get_beats(&beat_base_note, None);
      if tuplet_phrases.len() == phrases.len()
        && phrases.iter().all(|phrase| {
          (phrase.borrow().get_beats(&beat_base_note, None) - target_duration).abs() < f64::EPSILON
            && match phrase.borrow().modifications.iter().find_map(|modification| {
              match modification.borrow().get_modification() {
                PhraseModificationType::Tuplet { num_beats, into_beats } => Some((*num_beats, *into_beats)),
                _ => None,
              }
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
            let updated_phrase = Rc::new(RefCell::new(phrase.borrow().clone()));
            let mod_id = phrase.borrow().modifications.iter().find_map(|modification| {
              match modification.borrow().get_modification() {
                PhraseModificationType::Tuplet {
                  num_beats: _,
                  into_beats: _,
                } => Some(modification.borrow().get_id()),
                _ => None,
              }
            });
            if let Some(mod_id) = mod_id {
              // Expand tuplet by only taking the first note and holding it for the full tuplet duration
              let target_duration = Duration::from_beats(
                &beat_base_note,
                updated_phrase.borrow().get_beats(&beat_base_note, None),
              );
              let target_element = match &updated_phrase.borrow().content[0] {
                PhraseContent::Note(note) => {
                  let mut updated_note = note.borrow().clone();
                  updated_note.duration = target_duration;
                  PhraseContent::Note(Rc::new(RefCell::new(updated_note)))
                }
                PhraseContent::Chord(chord) => {
                  let mut updated_chord = chord.borrow().clone();
                  updated_chord.content.clear();
                  chord.borrow().content.iter().for_each(|item| match item {
                    ChordContent::Note(note) => {
                      let mut updated_note = note.borrow().clone();
                      updated_note.duration = target_duration;
                      updated_chord
                        .content
                        .push(ChordContent::Note(Rc::new(RefCell::new(updated_note))));
                    }
                  });
                  PhraseContent::Chord(Rc::new(RefCell::new(updated_chord)))
                }
                _ => unsafe { core::hint::unreachable_unchecked() },
              };
              updated_phrase.borrow_mut().remove_modification(mod_id);
              updated_phrase.borrow_mut().content.clear();
              updated_phrase.borrow_mut().content.push(target_element);
            }
            updated_phrase
          })
          .collect()
      }
    };

    // Create an ordered list of timeslices containing all notes and chords across all voices
    let mut timeslices: Vec<(f64, Vec<PhraseContent>)> = Vec::new();
    for phrase in &to_combine {
      let (mut index, mut curr_time) = (0, 0.0);
      let tuplet_ratio =
        phrase
          .borrow()
          .modifications
          .iter()
          .find_map(|modification| match modification.borrow().get_modification() {
            PhraseModificationType::Tuplet { num_beats, into_beats } => {
              Some(f64::from(*into_beats) / f64::from(*num_beats))
            }
            _ => None,
          });
      for item in phrase.borrow().iter() {
        let (slice_duration, item) = match item {
          PhraseContent::Note(note) => (
            note.borrow().get_beats(&beat_base_note, tuplet_ratio),
            PhraseContent::Note(Rc::clone(note)),
          ),
          PhraseContent::Chord(chord) => (
            chord.borrow().get_beats(&beat_base_note, tuplet_ratio),
            PhraseContent::Chord(Rc::clone(chord)),
          ),
          _ => unsafe { core::hint::unreachable_unchecked() },
        };
        if let Some((mut slice_time, existing_slice)) = timeslices.get_mut(index) {
          let mut existing_slice = existing_slice;
          while (slice_time - curr_time).abs() > 0.000_001 && curr_time > slice_time {
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
    let phrase = Rc::new(RefCell::new(to_combine[0].borrow().clone()));
    phrase.borrow_mut().content.clear();
    for (idx, (start_time, content)) in timeslices.iter().enumerate() {
      if content.len() > 1 {
        let target_beats = if idx + 1 < timeslices.len() {
          timeslices[idx + 1].0 - start_time
        } else {
          content
            .iter()
            .map(|item| match item {
              PhraseContent::Note(note) => note.borrow().get_beats(&beat_base_note, None),
              PhraseContent::Chord(chord) => chord.borrow().get_beats(&beat_base_note, None),
              _ => unsafe { core::hint::unreachable_unchecked() },
            })
            .reduce(f64::max)
            .unwrap_or_default()
        };
        let target_duration = Duration::from_beats(&beat_base_note, target_beats);
        let combined = phrase.borrow_mut().add_chord();
        content.iter().for_each(|item| match item {
          PhraseContent::Note(note) => {
            let note = Rc::new(RefCell::new(note.borrow().clone()));
            if note.borrow().duration.value() < target_duration.value() {
              note.borrow_mut().duration = target_duration;
            }
            combined.borrow_mut().content.push(ChordContent::Note(note));
          }
          PhraseContent::Chord(chord) => {
            chord.borrow().modifications.iter().for_each(|modification| {
              combined
                .borrow_mut()
                .add_modification(*modification.borrow().get_modification());
            });
            chord.borrow().iter().for_each(|item| match item {
              ChordContent::Note(note) => {
                let note = Rc::new(RefCell::new(note.borrow().clone()));
                if note.borrow().duration.value() < target_duration.value() {
                  note.borrow_mut().duration = target_duration;
                }
                combined.borrow_mut().content.push(ChordContent::Note(note));
              }
            });
          }
          _ => unsafe { core::hint::unreachable_unchecked() },
        });
      } else if let Some(content) = content.first() {
        match content {
          PhraseContent::Note(note) => {
            phrase.borrow_mut().content.push(PhraseContent::Note(Rc::clone(note)));
          }
          PhraseContent::Chord(chord) => {
            phrase.borrow_mut().content.push(PhraseContent::Chord(Rc::clone(chord)));
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

  pub fn add_phrase(&mut self) -> Rc<RefCell<Phrase>> {
    let phrase = Phrase::new();
    self.content.push(MultiVoiceContent::Phrase(Rc::clone(&phrase)));
    phrase
  }

  #[must_use]
  pub fn get_phrase(&mut self, id: usize) -> Option<Rc<RefCell<Phrase>>> {
    self.content.iter().find_map(|item| match item {
      MultiVoiceContent::Phrase(phrase) if phrase.borrow().get_id() == id => Some(Rc::clone(phrase)),
      MultiVoiceContent::Phrase(phrase) => phrase.borrow_mut().get_phrase(id),
    })
  }

  #[must_use]
  pub fn get_multivoice(&mut self, id: usize) -> Option<Rc<RefCell<MultiVoice>>> {
    self.content.iter().find_map(|item| match item {
      MultiVoiceContent::Phrase(phrase) => phrase.borrow_mut().get_multivoice(id),
    })
  }

  #[must_use]
  pub fn get_chord(&mut self, id: usize) -> Option<Rc<RefCell<Chord>>> {
    self.content.iter().find_map(|item| match item {
      MultiVoiceContent::Phrase(phrase) => phrase.borrow_mut().get_chord(id),
    })
  }

  #[must_use]
  pub fn get_note(&mut self, id: usize) -> Option<Rc<RefCell<Note>>> {
    self.content.iter().find_map(|item| match item {
      MultiVoiceContent::Phrase(phrase) => phrase.borrow_mut().get_note(id),
    })
  }

  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration, tuplet_ratio: Option<f64>) -> f64 {
    self
      .content
      .iter()
      .map(|content| match &content {
        MultiVoiceContent::Phrase(phrase) => phrase.borrow().get_beats(beat_base, tuplet_ratio),
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
      MultiVoiceContent::Phrase(phrase) => phrase.borrow().get_id() != id,
    });
    self.content.iter().for_each(|item| match item {
      MultiVoiceContent::Phrase(phrase) => {
        phrase.borrow_mut().remove_item(id);
      }
    });
    self
  }

  #[must_use]
  pub(crate) fn num_timeslices(&self) -> usize {
    self.iter_timeslices().len()
  }

  pub fn iter(&self) -> Iter<'_, MultiVoiceContent> {
    self.content.iter()
  }

  #[must_use]
  pub fn iter_timeslices(&self) -> Vec<Timeslice> {
    let beat_base_note = Duration::new(DurationType::SixtyFourth, 0);
    let mut timeslices: Vec<(f64, Timeslice)> = Vec::new();
    self.content.iter().for_each(|item| {
      let (mut index, mut curr_time) = (0, 0.0);
      for mut slice in match item {
        MultiVoiceContent::Phrase(phrase) => phrase.borrow().iter_timeslices(),
      } {
        let slice_duration = slice.get_beats(&beat_base_note);
        if let Some((mut slice_time, existing_slice)) = timeslices.get_mut(index) {
          let mut existing_slice = existing_slice;
          while (slice_time - curr_time).abs() > 0.000_001 && curr_time > slice_time {
            index += 1;
            (slice_time, existing_slice) = if let Some((start_time, slice)) = timeslices.get_mut(index) {
              (*start_time, slice)
            } else {
              unsafe {
                timeslices.push((curr_time, Timeslice::new()));
                let (start_time, slice) = timeslices.last_mut().unwrap_unchecked();
                (*start_time, slice)
              }
            };
          }
          if (slice_time - curr_time).abs() < 0.000_001 {
            existing_slice.combine_with(&mut slice);
          } else {
            timeslices.insert(index, (curr_time, slice));
          }
        } else {
          timeslices.push((curr_time, slice));
        }
        curr_time += slice_duration;
        index += 1;
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

#[cfg(feature = "print")]
impl core::fmt::Display for MultiVoice {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let voices = self
      .content
      .iter()
      .map(|item| match item {
        MultiVoiceContent::Phrase(phrase) => phrase.borrow().to_string(),
      })
      .collect::<Vec<_>>()
      .join(", ");
    write!(f, "MultiVoice: [{voices}]")
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::note::{Accidental, Pitch};

  #[test]
  fn test_flatten_normal_to_tuplet() {
    let multivoice = MultiVoice::new();
    let phrase1 = multivoice.borrow_mut().add_phrase();
    let phrase2 = multivoice.borrow_mut().add_phrase();
    phrase1.borrow_mut().add_modification(PhraseModificationType::Tuplet {
      num_beats: 3,
      into_beats: 2,
    });
    phrase1.borrow_mut().add_note(Pitch::C(4), Duration::new(DurationType::Eighth, 0), None);
    phrase1
      .borrow_mut()
      .add_note(Pitch::C(4), Duration::new(DurationType::Eighth, 0), Some(Accidental::Sharp));
    phrase1.borrow_mut().add_note(Pitch::C(4), Duration::new(DurationType::Eighth, 0), None);
    phrase2.borrow_mut().add_note(Pitch::E(4), Duration::new(DurationType::Quarter, 0), None);
    /*println!(
      "MultiVoice Normal to Tuplet: {}",
      multivoice.borrow().flatten().borrow()
    );*/
  }

  #[test]
  fn test_flatten_tuplet_to_normal() {
    let multivoice = MultiVoice::new();
    let phrase1 = multivoice.borrow_mut().add_phrase();
    let phrase2 = multivoice.borrow_mut().add_phrase();
    phrase1.borrow_mut().add_modification(PhraseModificationType::Tuplet {
      num_beats: 3,
      into_beats: 2,
    });
    phrase1.borrow_mut().add_note(Pitch::C(4), Duration::new(DurationType::Eighth, 0), None);
    phrase1
      .borrow_mut()
      .add_note(Pitch::C(4), Duration::new(DurationType::Eighth, 0), Some(Accidental::Sharp));
    phrase1.borrow_mut().add_note(Pitch::C(4), Duration::new(DurationType::Eighth, 0), None);
    phrase2.borrow_mut().add_note(Pitch::E(4), Duration::new(DurationType::Eighth, 0), None);
    phrase2.borrow_mut().add_note(Pitch::E(4), Duration::new(DurationType::Eighth, 0), None);
    /*println!(
      "MultiVoice Tuplet to Normal: {}",
      multivoice.borrow().flatten().borrow()
    );*/
  }

  #[test]
  fn test_flatten_tuplet_to_tuplet() {
    let multivoice = MultiVoice::new();
    let phrase1 = multivoice.borrow_mut().add_phrase();
    let phrase2 = multivoice.borrow_mut().add_phrase();
    phrase1.borrow_mut().add_modification(PhraseModificationType::Tuplet {
      num_beats: 3,
      into_beats: 2,
    });
    phrase1.borrow_mut().add_note(Pitch::C(4), Duration::new(DurationType::Eighth, 0), None);
    phrase1
      .borrow_mut()
      .add_note(Pitch::C(4), Duration::new(DurationType::Eighth, 0), Some(Accidental::Sharp));
    phrase1.borrow_mut().add_note(Pitch::C(4), Duration::new(DurationType::Eighth, 0), None);
    phrase2.borrow_mut().add_modification(PhraseModificationType::Tuplet {
      num_beats: 3,
      into_beats: 2,
    });
    phrase2.borrow_mut().add_note(Pitch::E(4), Duration::new(DurationType::Eighth, 0), None);
    phrase2.borrow_mut().add_note(Pitch::F(4), Duration::new(DurationType::Eighth, 0), None);
    phrase2.borrow_mut().add_note(Pitch::E(4), Duration::new(DurationType::Eighth, 0), None);
    /*println!(
      "MultiVoice Tuplet to Tuplet: {}",
      multivoice.borrow().flatten().borrow()
    );*/
  }

  #[test]
  fn test_flatten_monstrosity() {
    let multivoice = MultiVoice::new();
    let phrase1 = multivoice.borrow_mut().add_phrase();
    let phrase2 = multivoice.borrow_mut().add_phrase();
    let phrase3 = multivoice.borrow_mut().add_phrase();
    let multivoice2 = phrase3.borrow_mut().add_multivoice();
    let phrase3 = multivoice2.borrow_mut().add_phrase();
    let phrase4 = multivoice2.borrow_mut().add_phrase();
    phrase1.borrow_mut().add_modification(PhraseModificationType::Tuplet {
      num_beats: 3,
      into_beats: 2,
    });
    phrase1.borrow_mut().add_note(Pitch::C(4), Duration::new(DurationType::Eighth, 0), None);
    phrase1
      .borrow_mut()
      .add_note(Pitch::C(4), Duration::new(DurationType::Eighth, 0), Some(Accidental::Sharp));
    phrase1.borrow_mut().add_note(Pitch::C(4), Duration::new(DurationType::Eighth, 0), None);
    phrase2.borrow_mut().add_note(Pitch::E(4), Duration::new(DurationType::Whole, 1), None);
    phrase3.borrow_mut().add_note(Pitch::A(4), Duration::new(DurationType::Sixteenth, 0), None);
    phrase4.borrow_mut().add_note(Pitch::B(4), Duration::new(DurationType::Half, 0), None);
    phrase4
      .borrow_mut()
      .add_note(Pitch::B(4), Duration::new(DurationType::Half, 0), Some(Accidental::Flat));
    //println!("MultiVoice Flatten: {}", multivoice.borrow().flatten().borrow());
  }
}
