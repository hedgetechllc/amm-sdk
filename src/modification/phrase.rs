use crate::context::DynamicMarking;
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum PedalType {
  #[default]
  Damper,
  Sostenuto,
  Soft,
}

impl std::fmt::Display for PedalType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match *self {
        PedalType::Damper => "Damper Pedal",
        PedalType::Sostenuto => "Sostenuto Pedal",
        PedalType::Soft => "Soft Pedal",
      }
    )
  }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PhraseModificationType {
  Crescendo {
    starting_dynamic: DynamicMarking,
    ending_dynamic: DynamicMarking,
  },
  Descrescendo {
    starting_dynamic: DynamicMarking,
    ending_dynamic: DynamicMarking,
  },
  OctaveShift {
    num_octaves: i8,
  },
  Pedal {
    pedal_type: PedalType,
  },
  Slur,
  Tuplet {
    into_num_notes: u8,
  },
}

impl std::fmt::Display for PhraseModificationType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match *self {
        PhraseModificationType::Crescendo { .. } => String::from("Crescendo"),
        PhraseModificationType::Descrescendo { .. } => String::from("Decrescendo"),
        PhraseModificationType::OctaveShift { num_octaves } => format!("{} Octave Shift", num_octaves),
        PhraseModificationType::Pedal { pedal_type } => pedal_type.to_string(),
        PhraseModificationType::Slur => String::from("Slur"),
        PhraseModificationType::Tuplet { into_num_notes } => format!("X-to-{} Tuplet", into_num_notes),
      }
    )
  }
}

#[derive(Clone, PartialEq, Eq)]
pub struct PhraseModification {
  pub num_slices: usize,
  pub slice_indices: Vec<Rc<RefCell<usize>>>,
  pub modification: PhraseModificationType,
}
