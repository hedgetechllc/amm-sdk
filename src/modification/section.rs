use crate::context::{generate_id, Tempo, TempoModification};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Eq, PartialEq)]
pub enum SectionModificationType {
  JumpAtEnd { section: String },
  OnlyPlay { iterations: Vec<u8> },
  Repeat { num_times: u8 },
  Silence { num_beats: usize },
  Tempo { tempo: Tempo },
  TempoModification { tempo_modification: TempoModification },
}

#[derive(Clone, Eq, PartialEq)]
pub struct SectionModification {
  id: usize,
  modification: SectionModificationType,
}

impl SectionModification {
  pub fn new(modification: SectionModificationType) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: generate_id(),
      modification,
    }))
  }

  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn get_modification(&self) -> &SectionModificationType {
    &self.modification
  }
}

impl std::fmt::Display for SectionModificationType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::JumpAtEnd { section } => write!(f, "Jump at end of section to {}", section),
      Self::OnlyPlay { iterations } => write!(f, "Only play {} times", iterations.len()),
      Self::Repeat { num_times } => write!(f, "Repeat {} times", num_times),
      Self::Silence { num_beats } => write!(f, "Silence for {} beats", num_beats),
      Self::Tempo { tempo } => write!(f, "Tempo: {}", tempo),
      Self::TempoModification { tempo_modification } => write!(f, "Tempo modification: {}", tempo_modification),
    }
  }
}
