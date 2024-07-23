use crate::context::{generate_id, DynamicMarking};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum PedalType {
  #[default]
  Sustain,
  Sostenuto,
  Soft,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PhraseModificationType {
  Accelerando,
  Crescendo { final_dynamic: DynamicMarking },
  Decrescendo { final_dynamic: DynamicMarking },
  Glissando,
  Hairpin { maximum_dynamic: DynamicMarking },
  Legato,
  OctaveShift { num_octaves: i8 },
  Pedal { r#type: PedalType },
  Portamento,
  Rallentando,
  Ritardando,
  Ritenuto,
  Stringendo,
  Tied,
  Tremolo { relative_speed: u8 },
  Tuplet { into_beats: u8 },
}

#[derive(Clone, Eq, PartialEq)]
pub struct PhraseModification {
  id: usize,
  modification: PhraseModificationType,
}

impl PhraseModification {
  pub fn new(modification: PhraseModificationType) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: generate_id(),
      modification,
    }))
  }

  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn get_modification(&self) -> &PhraseModificationType {
    &self.modification
  }
}

impl std::fmt::Display for PedalType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

impl std::fmt::Display for PhraseModificationType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::Accelerando => write!(f, "Accelerando"),
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
      Self::OctaveShift { num_octaves } => write!(f, "Shift by {} octaves", num_octaves),
      Self::Pedal { r#type } => write!(f, "{} Pedal", r#type),
      Self::Portamento => write!(f, "Portamento"),
      Self::Rallentando => write!(f, "Rallentando"),
      Self::Ritardando => write!(f, "Ritardando"),
      Self::Ritenuto => write!(f, "Ritenuto"),
      Self::Stringendo => write!(f, "Stringendo"),
      Self::Tied => write!(f, "Tied"),
      Self::Tremolo { relative_speed } => write!(f, "Tremolo at {}x speed", relative_speed),
      Self::Tuplet { into_beats } => write!(f, "Tuplet into {} beats", into_beats),
    }
  }
}
