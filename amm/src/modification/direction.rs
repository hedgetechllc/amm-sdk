use crate::context::{generate_id, Clef, Dynamic, Key, TimeSignature};
use alloc::{rc::Rc, string::String};
use core::cell::RefCell;
#[cfg(feature = "json")]
use {
  amm_internal::json_prelude::*,
  amm_macros::{JsonDeserialize, JsonSerialize},
};

#[cfg_attr(feature = "json", derive(JsonDeserialize, JsonSerialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
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

#[cfg_attr(feature = "json", derive(JsonDeserialize, JsonSerialize))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Direction {
  id: usize,
  pub r#type: DirectionType,
}

impl Direction {
  #[must_use]
  pub fn new(r#type: DirectionType) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: generate_id(),
      r#type,
    }))
  }

  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
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
