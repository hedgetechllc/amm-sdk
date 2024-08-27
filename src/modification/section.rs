use crate::context::{generate_id, Tempo, TempoMarking};
use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;

#[derive(Clone, Eq, PartialEq)]
pub enum SectionModificationType {
  Accelerando, // Quick tempo acceleration over few notes or measures
  OnlyPlay { iterations: Vec<u8> },
  Rallentando, // Gradual tempo reduction leading to context change
  Repeat { num_times: u8 },
  Ritardando, // Gradual tempo reduction leading to complete stop
  Ritenuto,   // Immediate tempo reduction
  Stringendo, // Gradual tempo acceleration leading to context change
  TempoExplicit { tempo: Tempo },
  TempoImplicit { tempo: TempoMarking },
}

#[derive(Clone, Eq, PartialEq)]
pub struct SectionModification {
  id: usize,
  modification: SectionModificationType,
}

impl SectionModification {
  #[must_use]
  pub fn new(modification: SectionModificationType) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: generate_id(),
      modification,
    }))
  }

  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }

  #[must_use]
  pub fn get_modification(&self) -> &SectionModificationType {
    &self.modification
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for SectionModification {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.modification)
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for SectionModificationType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match self {
      Self::Accelerando => write!(f, "Accelerando"),
      Self::OnlyPlay { iterations } => write!(f, "Only play {} times", iterations.len()),
      Self::Rallentando => write!(f, "Rallentando"),
      Self::Repeat { num_times } => write!(f, "Repeat {num_times} times"),
      Self::Ritardando => write!(f, "Ritardando"),
      Self::Ritenuto => write!(f, "Ritenuto"),
      Self::Stringendo => write!(f, "Stringendo"),
      Self::TempoExplicit { tempo } => write!(f, "Explicit Tempo: {tempo}"),
      Self::TempoImplicit { tempo } => write!(f, "Implicit Tempo: {tempo}"),
    }
  }
}
