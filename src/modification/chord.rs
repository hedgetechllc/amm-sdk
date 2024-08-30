use super::note::NoteModificationType;
use crate::context::{generate_id, Dynamic};
use crate::storage::{Serialize, SerializedItem};
use alloc::{collections::BTreeMap, rc::Rc};
use core::cell::RefCell;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ChordModificationType {
  Accent,
  Arpeggiate,
  DetachedLegato,
  DownBow,
  Dynamic { dynamic: Dynamic },
  Fermata,
  Fingernails,
  HalfMuted,
  HarmonMute { open: bool, half: bool },
  Heel,
  Marcato,
  NonArpeggiate,
  Open,
  Pizzicato,
  Sforzando,
  Smear,
  SoftAccent,
  Spiccato,
  Staccato,
  Staccatissimo,
  Stress,
  Tenuto,
  Tie,
  Toe,
  Tremolo { relative_speed: u8 },
  Unstress,
  UpBow,
}

#[derive(Clone, Eq, PartialEq)]
pub struct ChordModification {
  id: usize,
  modification: ChordModificationType,
}

impl ChordModification {
  #[must_use]
  pub fn new(modification: ChordModificationType) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: generate_id(),
      modification,
    }))
  }

  #[must_use]
  pub fn from_note_modification(modification: &NoteModificationType) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: generate_id(),
      modification: match *modification {
        NoteModificationType::Accent => ChordModificationType::Accent,
        NoteModificationType::DetachedLegato => ChordModificationType::DetachedLegato,
        NoteModificationType::DownBow => ChordModificationType::DownBow,
        NoteModificationType::Dynamic { dynamic } => ChordModificationType::Dynamic { dynamic },
        NoteModificationType::Fermata => ChordModificationType::Fermata,
        NoteModificationType::Fingernails => ChordModificationType::Fingernails,
        NoteModificationType::HalfMuted => ChordModificationType::HalfMuted,
        NoteModificationType::HarmonMute { open, half } => ChordModificationType::HarmonMute { open, half },
        NoteModificationType::Heel => ChordModificationType::Heel,
        NoteModificationType::Marcato => ChordModificationType::Marcato,
        NoteModificationType::Open => ChordModificationType::Open,
        NoteModificationType::Pizzicato => ChordModificationType::Pizzicato,
        NoteModificationType::Sforzando => ChordModificationType::Sforzando,
        NoteModificationType::Smear => ChordModificationType::Smear,
        NoteModificationType::SoftAccent => ChordModificationType::SoftAccent,
        NoteModificationType::Spiccato => ChordModificationType::Spiccato,
        NoteModificationType::Staccato => ChordModificationType::Staccato,
        NoteModificationType::Staccatissimo => ChordModificationType::Staccatissimo,
        NoteModificationType::Stress => ChordModificationType::Stress,
        NoteModificationType::Tenuto => ChordModificationType::Tenuto,
        NoteModificationType::Tie => ChordModificationType::Tie,
        NoteModificationType::Toe => ChordModificationType::Toe,
        NoteModificationType::Tremolo { relative_speed } => ChordModificationType::Tremolo { relative_speed },
        NoteModificationType::Unstress => ChordModificationType::Unstress,
        NoteModificationType::UpBow => ChordModificationType::UpBow,
        _ => unsafe { core::hint::unreachable_unchecked() },
      },
    }))
  }

  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }

  #[must_use]
  pub fn get_modification(&self) -> &ChordModificationType {
    &self.modification
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for ChordModification {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.modification)
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for ChordModificationType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match self {
      Self::Accent => write!(f, "Accent"),
      Self::Arpeggiate => write!(f, "Arpeggiate"),
      Self::DetachedLegato => write!(f, "Detached Legato"),
      Self::DownBow => write!(f, "Down Bow"),
      Self::Dynamic { dynamic } => write!(f, "Dynamic: {dynamic}"),
      Self::Fermata => write!(f, "Fermata"),
      Self::Fingernails => write!(f, "Fingernails"),
      Self::HalfMuted => write!(f, "Half Muted"),
      Self::HarmonMute { open, half } => write!(
        f,
        "Harmon Mute: {}{}",
        if *half { "Half-" } else { "Fully " },
        if *open { "Open" } else { "Closed" },
      ),
      Self::Heel => write!(f, "Heel"),
      Self::Marcato => write!(f, "Marcato"),
      Self::NonArpeggiate => write!(f, "Non-Arpeggiate"),
      Self::Open => write!(f, "Open"),
      Self::Pizzicato => write!(f, "Pizzicato"),
      Self::Sforzando => write!(f, "Sforzando"),
      Self::Smear => write!(f, "Smear"),
      Self::SoftAccent => write!(f, "Soft Accent"),
      Self::Spiccato => write!(f, "Spiccato"),
      Self::Staccato => write!(f, "Staccato"),
      Self::Staccatissimo => write!(f, "Staccatissimo"),
      Self::Stress => write!(f, "Stress"),
      Self::Tenuto => write!(f, "Tenuto"),
      Self::Tie => write!(f, "Tied"),
      Self::Toe => write!(f, "Toe"),
      Self::Tremolo { relative_speed } => write!(f, "Tremolo at {relative_speed}x speed"),
      Self::Unstress => write!(f, "Unstress"),
      Self::UpBow => write!(f, "Up Bow"),
    }
  }
}

#[cfg(feature = "print")]
impl Serialize for ChordModification {
  fn serialize(&self) -> SerializedItem {
    let (name, serialized) = match &self.modification {
      ChordModificationType::Dynamic { dynamic } => (String::from("Dynamic"), dynamic.serialize()),
      ChordModificationType::HarmonMute { open, half } => (
        String::from("Harmon Mute"),
        SerializedItem {
          attributes: BTreeMap::from([
            (String::from("open"), open.to_string()),
            (String::from("half"), half.to_string()),
          ]),
          contents: BTreeMap::new(),
          elements: BTreeMap::new(),
        },
      ),
      ChordModificationType::Tremolo { relative_speed } => (
        String::from("Tremolo"),
        SerializedItem {
          attributes: BTreeMap::from([(String::from("relative_speed"), relative_speed.to_string())]),
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
