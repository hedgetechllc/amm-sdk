use crate::context::{generate_id, Clef, Dynamic, Key, TimeSignature};
use crate::storage::{Serialize, SerializedItem};
use alloc::{collections::BTreeMap, rc::Rc, string::String};
use core::cell::RefCell;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DirectionType {
  AccordionRegistration { high: bool, middle: u8, low: bool },
  BreathMark,
  Caesura,
  Clef { clef: Clef },
  Dynamic { dynamic: Dynamic },
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
      Self::Clef { clef } => write!(f, "Clef: {clef}"),
      Self::Dynamic { dynamic } => write!(f, "Dynamic: {dynamic}"),
      Self::Key { key } => write!(f, "Key: {key}"),
      Self::StringMute { on } => write!(f, "String Mute: {}", if *on { "on" } else { "off" }),
      Self::TimeSignature { time_signature } => write!(f, "Time Signature: {time_signature}"),
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Direction {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.modification)
  }
}

#[cfg(feature = "print")]
impl Serialize for Direction {
  fn serialize(&self) -> SerializedItem {
    let (name, serialized) = match &self.modification {
      DirectionType::AccordionRegistration { high, middle, low } => (
        "Accordion Registration",
        SerializedItem {
          attributes: BTreeMap::from([
            (String::from("high"), high.to_string()),
            (String::from("middle"), middle.to_string()),
            (String::from("low"), low.to_string()),
          ]),
          contents: BTreeMap::new(),
          elements: BTreeMap::new(),
        },
      ),
      DirectionType::BreathMark => ("Breath Mark", SerializedItem::default()),
      DirectionType::Caesura => ("Caesura", SerializedItem::default()),
      DirectionType::Clef { clef } => ("Clef", clef.serialize()),
      DirectionType::Dynamic { dynamic } => ("Dynamic", dynamic.serialize()),
      DirectionType::Key { key } => ("Key", key.serialize()),
      DirectionType::StringMute { on } => (
        "String Mute",
        SerializedItem {
          attributes: BTreeMap::from([(String::from("on"), on.to_string())]),
          contents: BTreeMap::new(),
          elements: BTreeMap::new(),
        },
      ),
      DirectionType::TimeSignature { time_signature } => ("Time Signature", time_signature.serialize()),
    };
    let contents = if serialized.attributes.is_empty() && serialized.contents.is_empty() {
      BTreeMap::new()
    } else {
      BTreeMap::from([(String::from("details"), serialized)])
    };
    SerializedItem {
      attributes: BTreeMap::from([
        (String::from("id"), self.id.to_string()),
        (String::from("type"), String::from(name)),
      ]),
      contents,
      elements: BTreeMap::new(),
    }
  }
}
