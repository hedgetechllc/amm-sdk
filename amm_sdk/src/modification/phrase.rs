use crate::context::{generate_id, Dynamic};
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize, ModOrder};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ModOrder, JsonDeserialize, JsonSerialize)]
pub enum PedalType {
  #[default]
  Sustain,
  Sostenuto,
  Soft,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ModOrder, JsonDeserialize, JsonSerialize)]
pub enum PhraseModificationType {
  Crescendo {
    final_dynamic: Option<Dynamic>,
  },
  Decrescendo {
    final_dynamic: Option<Dynamic>,
  },
  Glissando,
  Hairpin {
    maximum_dynamic: Option<Dynamic>,
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

#[derive(Debug, Default, Eq, JsonDeserialize, JsonSerialize)]
pub struct PhraseModification {
  id: usize,
  pub r#type: PhraseModificationType,
}

impl PhraseModification {
  #[must_use]
  pub fn new(r#type: PhraseModificationType) -> Self {
    Self {
      id: generate_id(),
      r#type,
    }
  }

  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }
}

impl Clone for PhraseModification {
  fn clone(&self) -> Self {
    Self {
      id: generate_id(),
      r#type: self.r#type,
    }
  }
}

impl PartialEq for PhraseModification {
  fn eq(&self, other: &Self) -> bool {
    self.r#type == other.r#type
  }
}

impl Ord for PhraseModification {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.r#type.cmp(&other.r#type)
  }
}

impl PartialOrd for PhraseModification {
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
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
      Self::Crescendo {
        final_dynamic: Some(dynamic),
      } => write!(f, "Crescendo to {dynamic}"),
      Self::Crescendo { final_dynamic: None } => write!(f, "Crescendo"),
      Self::Decrescendo {
        final_dynamic: Some(dynamic),
      } => write!(f, "Decrescendo to {dynamic}"),
      Self::Decrescendo { final_dynamic: None } => write!(f, "Decrescendo"),
      Self::Glissando => write!(f, "Glissando"),
      Self::Hairpin {
        maximum_dynamic: Some(dynamic),
      } => write!(f, "Hairpin to {dynamic}"),
      Self::Hairpin { maximum_dynamic: None } => write!(f, "Hairpin"),
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
    write!(f, "{}", self.r#type)
  }
}
