use crate::context::{generate_id, Dynamic, DynamicMarking};
use alloc::rc::Rc;
use core::cell::RefCell;
#[cfg(feature = "json")]
use {
  amm_internal::json_prelude::*,
  amm_macros::{JsonDeserialize, JsonSerialize},
};

#[cfg_attr(feature = "json", derive(JsonDeserialize, JsonSerialize))]
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum PedalType {
  #[default]
  Sustain,
  Sostenuto,
  Soft,
}

#[cfg_attr(feature = "json", derive(JsonDeserialize, JsonSerialize))]
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub enum PhraseModificationType {
  Crescendo {
    final_dynamic: Dynamic,
  },
  Decrescendo {
    final_dynamic: Dynamic,
  },
  Glissando,
  Hairpin {
    maximum_dynamic: Dynamic,
  },
  #[default]
  Legato,
  OctaveShift {
    num_octaves: i8,
  },
  Pedal {
    pedal_type: PedalType,
  },
  Portamento,
  Tremolo {
    relative_speed: u8,
  },
  Tuplet {
    num_beats: u8,
    into_beats: u8,
  },
}

#[cfg_attr(feature = "json", derive(JsonDeserialize, JsonSerialize))]
#[derive(Clone, Default, Eq, PartialEq)]
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
impl core::fmt::Display for PhraseModificationType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match self {
      Self::Crescendo { final_dynamic } => write!(
        f,
        "Crescendo{}{}",
        if final_dynamic.marking == DynamicMarking::None {
          ""
        } else {
          ": to "
        },
        final_dynamic
      ),
      Self::Decrescendo { final_dynamic } => write!(
        f,
        "Decrescendo{}{}",
        if final_dynamic.marking == DynamicMarking::None {
          ""
        } else {
          ": to "
        },
        final_dynamic
      ),
      Self::Glissando => write!(f, "Glissando"),
      Self::Hairpin { maximum_dynamic } => write!(
        f,
        "Hairpin{}{}",
        if maximum_dynamic.marking == DynamicMarking::None {
          ""
        } else {
          ": to "
        },
        maximum_dynamic
      ),
      Self::Legato => write!(f, "Legato"),
      Self::OctaveShift { num_octaves } => write!(f, "Octave Shift: by {num_octaves}"),
      Self::Pedal { pedal_type } => write!(f, "Pedal: {pedal_type}"),
      Self::Portamento => write!(f, "Portamento"),
      Self::Tremolo { relative_speed } => write!(f, "Tremolo: {relative_speed}x speed"),
      Self::Tuplet { num_beats, into_beats } => write!(f, "Tuplet: {num_beats}:{into_beats}"),
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for PhraseModification {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.modification)
  }
}
