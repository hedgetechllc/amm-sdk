use crate::context::{generate_id, Dynamic, DynamicMarking};
use crate::storage::{Serialize, SerializedItem};
use alloc::{collections::BTreeMap, rc::Rc};
use core::cell::RefCell;

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum PedalType {
  #[default]
  Sustain,
  Sostenuto,
  Soft,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PhraseModificationType {
  Crescendo { final_dynamic: Dynamic },
  Decrescendo { final_dynamic: Dynamic },
  Glissando,
  Hairpin { maximum_dynamic: Dynamic },
  Legato,
  OctaveShift { num_octaves: i8 },
  Pedal { r#type: PedalType },
  Portamento,
  Tremolo { relative_speed: u8 },
  Tuplet { num_beats: u8, into_beats: u8 },
}

#[derive(Clone, Eq, PartialEq)]
pub struct PhraseModification {
  id: usize,
  modification: PhraseModificationType,
}

impl PhraseModification {
  #[must_use]
  pub fn new(modification: PhraseModificationType) -> Rc<RefCell<Self>> {
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
  pub fn get_modification(&self) -> &PhraseModificationType {
    &self.modification
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for PedalType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Sustain => "Sustain",
        Self::Sostenuto => "Sostenuto",
        Self::Soft => "Soft",
      }
    )
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for PhraseModification {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.modification)
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for PhraseModificationType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match self {
      Self::Crescendo { final_dynamic } => write!(
        f,
        "Crescendo{}{}",
        if final_dynamic.r#type == DynamicMarking::None {
          ""
        } else {
          " to "
        },
        final_dynamic
      ),
      Self::Decrescendo { final_dynamic } => write!(
        f,
        "Decrescendo{}{}",
        if final_dynamic.r#type == DynamicMarking::None {
          ""
        } else {
          " to "
        },
        final_dynamic
      ),
      Self::Glissando => write!(f, "Glissando"),
      Self::Hairpin { maximum_dynamic } => write!(
        f,
        "Hairpin{}{}",
        if maximum_dynamic.r#type == DynamicMarking::None {
          ""
        } else {
          " to "
        },
        maximum_dynamic
      ),
      Self::Legato => write!(f, "Legato"),
      Self::OctaveShift { num_octaves } => write!(f, "Shift by {num_octaves} octaves"),
      Self::Pedal { r#type } => write!(f, "{type} Pedal"),
      Self::Portamento => write!(f, "Portamento"),
      Self::Tremolo { relative_speed } => write!(f, "Tremolo at {relative_speed}x speed"),
      Self::Tuplet { num_beats, into_beats } => write!(f, "{num_beats}:{into_beats} Tuplet"),
    }
  }
}

#[cfg(feature = "print")]
impl Serialize for PhraseModification {
  fn serialize(&self) -> SerializedItem {
    let (name, serialized) = match &self.modification {
      PhraseModificationType::Crescendo { final_dynamic } => (String::from("Crescendo"), final_dynamic.serialize()),
      PhraseModificationType::Decrescendo { final_dynamic } => (String::from("Decrescendo"), final_dynamic.serialize()),
      PhraseModificationType::Hairpin { maximum_dynamic } => (String::from("Hairpin"), maximum_dynamic.serialize()),
      PhraseModificationType::OctaveShift { num_octaves } => (
        String::from("Octave Shift"),
        SerializedItem {
          attributes: BTreeMap::from([(String::from("num_octaves"), num_octaves.to_string())]),
          contents: BTreeMap::new(),
          elements: BTreeMap::new(),
        },
      ),
      PhraseModificationType::Pedal { r#type } => (
        String::from("Pedal"),
        SerializedItem {
          attributes: BTreeMap::from([(String::from("type"), r#type.to_string())]),
          contents: BTreeMap::new(),
          elements: BTreeMap::new(),
        },
      ),
      PhraseModificationType::Tremolo { relative_speed } => (
        String::from("Tremolo"),
        SerializedItem {
          attributes: BTreeMap::from([(String::from("relative_speed"), relative_speed.to_string())]),
          contents: BTreeMap::new(),
          elements: BTreeMap::new(),
        },
      ),
      PhraseModificationType::Tuplet { num_beats, into_beats } => (
        String::from("Tuplet"),
        SerializedItem {
          attributes: BTreeMap::from([
            (String::from("num_beats"), num_beats.to_string()),
            (String::from("into_beats"), into_beats.to_string()),
          ]),
          contents: BTreeMap::new(),
          elements: BTreeMap::new(),
        },
      ),
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
