use crate::context::{generate_id, DynamicMarking};
use alloc::rc::Rc;
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
  Crescendo { final_dynamic: DynamicMarking },
  Decrescendo { final_dynamic: DynamicMarking },
  Glissando,
  Hairpin { maximum_dynamic: DynamicMarking },
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
        if final_dynamic == &DynamicMarking::None {
          ""
        } else {
          " to "
        },
        final_dynamic
      ),
      Self::Decrescendo { final_dynamic } => write!(
        f,
        "Decrescendo{}{}",
        if final_dynamic == &DynamicMarking::None {
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
        if maximum_dynamic == &DynamicMarking::None {
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
