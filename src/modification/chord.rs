use crate::context::{generate_id, DynamicMarking};
use alloc::rc::Rc;
use core::cell::RefCell;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ChordModificationType {
  Accent,
  Arpeggiate,
  DetachedLegato,
  DownBow,
  Dynamic { dynamic: DynamicMarking },
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
  pub fn new(modification: ChordModificationType) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: generate_id(),
      modification,
    }))
  }

  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn get_modification(&self) -> &ChordModificationType {
    &self.modification
  }
}

impl core::fmt::Display for ChordModification {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.modification)
  }
}

impl core::fmt::Display for ChordModificationType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match self {
      Self::Accent => write!(f, "Accent"),
      Self::Arpeggiate => write!(f, "Arpeggiate"),
      Self::DetachedLegato => write!(f, "Detached Legato"),
      Self::DownBow => write!(f, "Down Bow"),
      Self::Dynamic { dynamic } => write!(f, "Dynamic: {}", dynamic),
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
      Self::Toe => write!(f, "Toe"),
      Self::Tremolo { relative_speed } => write!(f, "Tremolo at {}x speed", relative_speed),
      Self::Unstress => write!(f, "Unstress"),
      Self::UpBow => write!(f, "Up Bow"),
    }
  }
}
