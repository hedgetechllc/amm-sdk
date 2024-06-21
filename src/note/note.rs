use super::{Accidental, DisplayOptions, Duration, Pitch};
use crate::context::Key;
use crate::modification::{NoteModification, PhraseModification};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, PartialEq, Eq)]
pub struct PhraseMembership {
  pub index: Rc<RefCell<usize>>,
  pub modification: Rc<RefCell<PhraseModification>>,
}

#[derive(Clone, Eq)]
pub struct Note {
  pub pitch: Pitch,
  pub duration: Duration,
  pub accidental: Accidental,
  pub display: DisplayOptions,
  pub modifications: Vec<NoteModification>,
  pub memberships: Vec<PhraseMembership>,
  pub tied_to: Option<Rc<RefCell<Note>>>,
  pub is_tied: bool,
}

impl Note {
  pub fn new(
    pitch: Pitch,
    duration: Duration,
    accidental: Option<Accidental>,
    display: Option<DisplayOptions>,
  ) -> Self {
    Note {
      pitch,
      duration,
      accidental: accidental.unwrap_or_default(),
      display: display.unwrap_or_default(),
      modifications: Vec::new(),
      memberships: Vec::new(),
      tied_to: None,
      is_tied: false,
    }
  }

  fn semitone_distance(&self, key_accidentals: &[Accidental; 8]) -> i16 {
    let (pitch_index, num_semitones) = self.pitch.value();
    num_semitones
      + if self.accidental != Accidental::None {
        self.accidental.value()
      } else {
        key_accidentals[pitch_index].value()
      }
  }

  pub fn add_note_modification(&mut self, modification: NoteModification) -> &mut Self {
    self
      .remove_note_modification(&modification)
      .modifications
      .push(modification);
    self
  }

  pub fn remove_note_modification(&mut self, modification: &NoteModification) -> &mut Self {
    self
      .modifications
      .retain(|item| std::mem::discriminant(item) != std::mem::discriminant(modification));
    self
  }

  pub fn add_phrase_modification(
    &mut self,
    modification: &Rc<RefCell<PhraseModification>>,
    note_index_in_phrase: usize,
    new_phrase_slice: bool,
  ) -> &mut Self {
    self.remove_phrase_modification(modification);
    let note_index_reference = Rc::new(RefCell::new(note_index_in_phrase));
    if new_phrase_slice {
      modification.borrow_mut().num_slices += 1;
      modification.borrow_mut().slice_indices.iter_mut().for_each(|index| {
        if *index.borrow() >= note_index_in_phrase {
          *index.borrow_mut() += 1;
        }
      });
    }
    modification
      .borrow_mut()
      .slice_indices
      .push(Rc::clone(&note_index_reference));
    self.memberships.push(PhraseMembership {
      index: Rc::clone(&note_index_reference),
      modification: Rc::clone(modification),
    });
    self
  }

  pub fn remove_phrase_modification(&mut self, modification: &Rc<RefCell<PhraseModification>>) -> &mut Self {
    if let Some(index) = self
      .memberships
      .iter()
      .position(|item| Rc::ptr_eq(&item.modification, modification))
    {
      let note_index = *self.memberships[index].index.borrow();
      if modification
        .borrow()
        .slice_indices
        .iter()
        .filter(|&index| *index.borrow() == note_index)
        .count()
        == 1
      {
        modification.borrow_mut().num_slices -= 1;
        modification.borrow_mut().slice_indices.iter_mut().for_each(|index| {
          if *index.borrow() > note_index {
            *index.borrow_mut() -= 1;
          }
        });
      }
      modification
        .borrow_mut()
        .slice_indices
        .retain(|item| Rc::ptr_eq(item, &self.memberships[index].index));
      self.memberships.remove(index);
    }
    self
  }

  pub fn tie(&mut self, note: &Rc<RefCell<Note>>) -> &mut Self {
    self.untie().tied_to = Some(Rc::clone(note));
    note.borrow_mut().is_tied = true;
    self
  }

  pub fn untie(&mut self) -> &mut Self {
    if let Some(tied_note) = &self.tied_to {
      tied_note.borrow_mut().is_tied = false;
    }
    self.tied_to = None;
    self
  }

  pub fn is_same_pitch(&self, other: &Note) -> bool {
    self.pitch == other.pitch
  }

  pub fn is_rest(&self) -> bool {
    self.pitch.is_rest()
  }

  pub fn pitch_hz(&self, key_accidentals: &[Accidental; 8], a4_frequency_hz: Option<f32>) -> f32 {
    a4_frequency_hz.unwrap_or(440.0) * 2.0_f32.powf(f32::from(self.semitone_distance(key_accidentals)) / 12.0)
  }

  pub fn midi_number(&self, key_accidentals: &[Accidental; 8]) -> u8 {
    (69 + self.semitone_distance(key_accidentals)) as u8
  }

  pub fn beats(&self, base_beat_value: f64) -> f64 {
    self.duration.beats(base_beat_value)
  }
}

impl PartialEq for Note {
  fn eq(&self, other: &Self) -> bool {
    (self.semitone_distance(&Key::CMajor.accidentals()) == other.semitone_distance(&Key::CMajor.accidentals()))
      && (self.beats(Duration::Quarter(0).value()) == other.beats(Duration::Quarter(0).value()))
      && (self.is_rest() == other.is_rest())
  }
}

impl std::fmt::Display for Note {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}{}{}{} {}",
      self.pitch,
      self.accidental,
      if self.is_rest() { "" } else { " " },
      self.duration,
      if self.is_rest() { "Rest" } else { "Note" }
    )
  }
}
