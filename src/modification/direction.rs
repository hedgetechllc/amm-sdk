use crate::context::{generate_id, Clef, DynamicMarking, Key, TimeSignature};
use alloc::{rc::Rc, string::String};
use core::cell::RefCell;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DirectionType {
  AccordionRegistration { high: bool, middle: u8, low: bool },
  BreathMark,
  Caesura,
  Clef { clef: Clef },
  Dynamic { dynamic: DynamicMarking },
  Key { key: Key },
  StringMute { on: bool },
  TimeSignature { time_signature: TimeSignature },
}

#[derive(Clone, Eq, PartialEq)]
pub struct Direction {
  id: usize,
  modification: DirectionType,
}

impl Direction {
  #[must_use]
  pub fn new(modification: DirectionType) -> Rc<RefCell<Self>> {
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
  pub fn get_modification(&self) -> &DirectionType {
    &self.modification
  }
}

impl core::fmt::Display for DirectionType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match self {
      Self::AccordionRegistration { high, middle, low } => write!(
        f,
        "Accordion registration: {}{}{}",
        if *high { "HIGH, " } else { "" },
        if *middle > 0 {
          format!("MIDDLE={}", *middle)
        } else {
          String::new()
        },
        if *low { ", LOW" } else { "" }
      ),
      Self::BreathMark => write!(f, "Breath mark"),
      Self::Caesura => write!(f, "Caesura"),
      Self::Clef { clef } => write!(f, "Clef: {clef}"),
      Self::Dynamic { dynamic } => write!(f, "Dynamic: {dynamic}"),
      Self::Key { key } => write!(f, "Key: {key}"),
      Self::StringMute { on } => write!(f, "String mute: {}", if *on { "on" } else { "off" }),
      Self::TimeSignature { time_signature } => write!(f, "Time signature: {time_signature}"),
    }
  }
}

impl core::fmt::Display for Direction {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.modification)
  }
}
