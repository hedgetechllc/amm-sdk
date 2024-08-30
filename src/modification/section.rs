use crate::context::{generate_id, Tempo, TempoSuggestion};
use crate::storage::{Serialize, SerializedItem};
use alloc::{collections::BTreeMap, rc::Rc, vec::Vec};
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
  TempoImplicit { tempo: TempoSuggestion },
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
impl Serialize for SectionModification {
  fn serialize(&self) -> SerializedItem {
    let (name, serialized) = match &self.modification {
      SectionModificationType::OnlyPlay { iterations } => (
        String::from("Only Play"),
        SerializedItem {
          attributes: BTreeMap::from([(
            String::from("iterations"),
            iterations
              .iter()
              .map(ToString::to_string)
              .collect::<Vec<String>>()
              .join(", "),
          )]),
          contents: BTreeMap::new(),
          elements: BTreeMap::new(),
        },
      ),
      SectionModificationType::Repeat { num_times } => (
        String::from("Repeat"),
        SerializedItem {
          attributes: BTreeMap::from([(String::from("num_times"), num_times.to_string())]),
          contents: BTreeMap::new(),
          elements: BTreeMap::new(),
        },
      ),
      SectionModificationType::TempoExplicit { tempo } => (String::from("Tempo Explicit"), tempo.serialize()),
      SectionModificationType::TempoImplicit { tempo } => (String::from("Tempo Implicit"), tempo.serialize()),
      other => (other.to_string(), SerializedItem::default()),
    };
    let contents = if serialized.attributes.is_empty() && serialized.contents.is_empty() {
      BTreeMap::new()
    } else {
      BTreeMap::from([(String::from("details"), serialized)])
    };
    SerializedItem {
      attributes: BTreeMap::from([
        (String::from("id"), self.id.to_string()),
        (String::from("type"), name.clone()),
      ]),
      contents,
      elements: BTreeMap::new(),
    }
  }
}
