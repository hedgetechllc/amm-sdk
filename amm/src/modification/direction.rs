use crate::context::{generate_id, Clef, Dynamic, Key, TimeSignature};
use crate::structure::Timeslice;
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize, ModOrder};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, ModOrder, JsonDeserialize, JsonSerialize)]
pub enum DirectionType {
  AccordionRegistration {
    high: bool,
    middle: u8,
    low: bool,
  },
  #[default]
  BreathMark,
  Caesura,
  ClefChange {
    clef: Clef,
  },
  Dynamic {
    dynamic: Dynamic,
  },
  KeyChange {
    key: Key,
  },
  StringMute {
    on: bool,
  },
  TimeSignatureChange {
    time_signature: TimeSignature,
  },
}

#[derive(Debug, Default, Eq, JsonDeserialize, JsonSerialize)]
pub struct Direction {
  id: usize,
  pub r#type: DirectionType,
}

impl Direction {
  #[must_use]
  pub fn new(r#type: DirectionType) -> Self {
    Self {
      id: generate_id(),
      r#type,
    }
  }

  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }

  #[must_use]
  pub fn to_timeslice(&self) -> Timeslice {
    let mut timeslice = Timeslice::new();
    timeslice.add_direction(self.clone());
    timeslice
  }
}

impl Clone for Direction {
  fn clone(&self) -> Self {
    Self {
      id: generate_id(),
      r#type: self.r#type,
    }
  }
}

impl PartialEq for Direction {
  fn eq(&self, other: &Self) -> bool {
    self.r#type == other.r#type
  }
}

impl Ord for Direction {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.r#type.cmp(&other.r#type)
  }
}

impl PartialOrd for Direction {
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for DirectionType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match self {
      Self::AccordionRegistration { high, middle, low } => write!(
        f,
        "Accordion Registration: {}{}{}",
        if *high { "HIGH, " } else { "" },
        if *middle > 0 {
          format!("MIDDLE={}", *middle)
        } else {
          String::new()
        },
        if *low { ", LOW" } else { "" }
      ),
      Self::BreathMark => write!(f, "Breath Mark"),
      Self::Caesura => write!(f, "Caesura"),
      Self::ClefChange { clef } => write!(f, "Clef: {clef}"),
      Self::Dynamic { dynamic } => write!(f, "Dynamic: {dynamic}"),
      Self::KeyChange { key } => write!(f, "Key: {key}"),
      Self::StringMute { on } => write!(f, "String Mute: {}", if *on { "on" } else { "off" }),
      Self::TimeSignatureChange { time_signature } => write!(f, "Time Signature: {time_signature}"),
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Direction {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.r#type)
  }
}
