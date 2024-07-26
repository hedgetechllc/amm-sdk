use crate::context::{generate_id, Tempo, TempoMarking};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Eq, PartialEq)]
pub enum SectionModificationType {
  Accelerando,
  JumpAtEnd { section: String },
  OnlyPlay { iterations: Vec<u8> },
  Rallentando,
  Repeat { num_times: u8 },
  Ritardando,
  Ritenuto,
  Silence { num_beats: usize },
  Stringendo,
  TempoExplicit { tempo: Tempo },
  TempoImplicit { tempo: TempoMarking },
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

impl std::fmt::Display for SectionModification {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.modification)
  }
}

impl std::fmt::Display for SectionModificationType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::Accelerando => write!(f, "Accelerando"),
      Self::JumpAtEnd { section } => write!(f, "Jump at end of section to {}", section),
      Self::OnlyPlay { iterations } => write!(f, "Only play {} times", iterations.len()),
      Self::Rallentando => write!(f, "Rallentando"),
      Self::Repeat { num_times } => write!(f, "Repeat {} times", num_times),
      Self::Ritardando => write!(f, "Ritardando"),
      Self::Ritenuto => write!(f, "Ritenuto"),
      Self::Silence { num_beats } => write!(f, "Silence for {} beats", num_beats),
      Self::Stringendo => write!(f, "Stringendo"),
      Self::TempoExplicit { tempo } => write!(f, "Explicit Tempo: {}", tempo),
      Self::TempoImplicit { tempo } => write!(f, "Implicit Tempo: {}", tempo),
    }
  }
}
