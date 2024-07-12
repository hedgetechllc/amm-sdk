use crate::context::{generate_id, Clef, DynamicMarking, Key, TimeSignature};
use std::{cell::RefCell, rc::Rc};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DirectionType {
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
  pub fn new(modification: DirectionType) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: generate_id(),
      modification,
    }))
  }

  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn get_modification(&self) -> &DirectionType {
    &self.modification
  }
}

impl std::fmt::Display for DirectionType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::BreathMark => write!(f, "Breath mark"),
      Self::Caesura => write!(f, "Caesura"),
      Self::Clef { clef } => write!(f, "Clef: {}", clef),
      Self::Dynamic { dynamic } => write!(f, "Dynamic: {}", dynamic),
      Self::Key { key } => write!(f, "Key: {}", key),
      Self::StringMute { on } => write!(f, "String mute: {}", if *on { "on" } else { "off" }),
      Self::TimeSignature { time_signature } => write!(f, "Time signature: {}", time_signature),
    }
  }
}
