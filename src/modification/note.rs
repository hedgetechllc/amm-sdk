use super::chord::ChordModificationType;
use crate::context::{generate_id, DynamicMarking};
use alloc::rc::Rc;
use core::cell::RefCell;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum HandbellTechnique {
  Belltree,
  Damp,
  Echo,
  Gyro,
  HandMartellato,
  MalletLift,
  MalletTable,
  Martellato,
  MartellatoLift,
  MutedMartellato,
  PluckLift,
  Swing,
}

impl core::fmt::Display for HandbellTechnique {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Belltree => "Belltree",
        Self::Damp => "Damp",
        Self::Echo => "Echo",
        Self::Gyro => "Gyro",
        Self::HandMartellato => "Hand Martellato",
        Self::MalletLift => "Mallet Lift",
        Self::MalletTable => "Mallet Table",
        Self::Martellato => "Martellato",
        Self::MartellatoLift => "Martellato Lift",
        Self::MutedMartellato => "Muted Martellato",
        Self::PluckLift => "Pluck Lift",
        Self::Swing => "Swing",
      }
    )
  }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum NoteModificationType {
  Accent,
  BrassBend,
  DetachedLegato,
  Doit,
  DoubleTongue,
  DownBow,
  Dynamic { dynamic: DynamicMarking },
  Falloff,
  Fermata,
  Fingernails,
  Flip,
  Glissando { from_current: bool, going_up: bool },
  Golpe,
  Grace { acciaccatura: bool, note_value: u8 },
  HalfMuted,
  Handbell { technique: HandbellTechnique },
  HarmonMute { open: bool, half: bool },
  Haydn,
  Heel,
  Hole { open: bool, half: bool },
  Marcato,
  Mordent { upper: bool },
  Open,
  Pizzicato,
  Plop,
  Portamento { from_current: bool, going_up: bool },
  Schleifer,
  Scoop,
  Sforzando,
  Shake,
  Smear,
  SoftAccent,
  Spiccato,
  Staccato,
  Staccatissimo,
  Stopped,
  Stress,
  Tap,
  Tenuto,
  ThumbPosition,
  Tie,
  Toe,
  Tremolo { relative_speed: u8 },
  Trill { upper: bool },
  TripleTongue,
  Turn { upper: bool, delayed: bool, vertical: bool },
  Unstress,
  UpBow,
}

#[derive(Clone, Eq, PartialEq)]
pub struct NoteModification {
  id: usize,
  modification: NoteModificationType,
}

impl NoteModification {
  #[must_use]
  pub fn new(modification: NoteModificationType) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: generate_id(),
      modification,
    }))
  }

  #[must_use]
  pub fn from_chord_modification(modification: &ChordModificationType) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: 0,
      modification: match *modification {
        ChordModificationType::Accent => NoteModificationType::Accent,
        ChordModificationType::DetachedLegato => NoteModificationType::DetachedLegato,
        ChordModificationType::DownBow => NoteModificationType::DownBow,
        ChordModificationType::Dynamic { dynamic } => NoteModificationType::Dynamic { dynamic },
        ChordModificationType::Fermata => NoteModificationType::Fermata,
        ChordModificationType::Fingernails => NoteModificationType::Fingernails,
        ChordModificationType::HalfMuted => NoteModificationType::HalfMuted,
        ChordModificationType::HarmonMute { open, half } => NoteModificationType::HarmonMute { open, half },
        ChordModificationType::Heel => NoteModificationType::Heel,
        ChordModificationType::Marcato => NoteModificationType::Marcato,
        ChordModificationType::Open => NoteModificationType::Open,
        ChordModificationType::Pizzicato => NoteModificationType::Pizzicato,
        ChordModificationType::Sforzando => NoteModificationType::Sforzando,
        ChordModificationType::Smear => NoteModificationType::Smear,
        ChordModificationType::SoftAccent => NoteModificationType::SoftAccent,
        ChordModificationType::Spiccato => NoteModificationType::Spiccato,
        ChordModificationType::Staccato => NoteModificationType::Staccato,
        ChordModificationType::Staccatissimo => NoteModificationType::Staccatissimo,
        ChordModificationType::Stress => NoteModificationType::Stress,
        ChordModificationType::Tenuto => NoteModificationType::Tenuto,
        ChordModificationType::Tie => NoteModificationType::Tie,
        ChordModificationType::Toe => NoteModificationType::Toe,
        ChordModificationType::Tremolo { relative_speed } => NoteModificationType::Tremolo { relative_speed },
        ChordModificationType::Unstress => NoteModificationType::Unstress,
        ChordModificationType::UpBow => NoteModificationType::UpBow,
        _ => unsafe { core::hint::unreachable_unchecked() },
      },
    }))
  }

  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }

  #[must_use]
  pub fn get_modification(&self) -> &NoteModificationType {
    &self.modification
  }
}

impl core::fmt::Display for NoteModification {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.modification)
  }
}

impl core::fmt::Display for NoteModificationType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match self {
      Self::Accent => write!(f, "Accent"),
      Self::BrassBend => write!(f, "Brass Bend"),
      Self::DetachedLegato => write!(f, "Detached Legato"),
      Self::Doit => write!(f, "Doit"),
      Self::DoubleTongue => write!(f, "Double Tongue"),
      Self::DownBow => write!(f, "Down Bow"),
      Self::Dynamic { dynamic } => write!(f, "Dynamic: {dynamic}"),
      Self::Falloff => write!(f, "Falloff"),
      Self::Fermata => write!(f, "Fermata"),
      Self::Fingernails => write!(f, "Fingernails"),
      Self::Flip => write!(f, "Flip"),
      Self::Glissando { from_current, going_up } => write!(
        f,
        "Glissando: {} {}",
        if *going_up { "Going Up" } else { "Going Down" },
        if *from_current { "From Note" } else { "To Note" },
      ),
      Self::Golpe => write!(f, "Golpe"),
      Self::Grace {
        acciaccatura,
        note_value,
      } => write!(
        f,
        "{} Grace Note: {note_value}",
        if *acciaccatura { "Acciaccatura" } else { "Appoggiatura" },
      ),
      Self::HalfMuted => write!(f, "Half Muted"),
      Self::Handbell { technique } => write!(f, "Handbell: {technique}"),
      Self::HarmonMute { open, half } => write!(
        f,
        "Harmon Mute: {}{}",
        if *half { "Half-" } else { "Fully " },
        if *open { "Open" } else { "Closed" },
      ),
      Self::Haydn => write!(f, "Haydn"),
      Self::Heel => write!(f, "Heel"),
      Self::Hole { open, half } => write!(
        f,
        "Hole: {} {}",
        if *half { "Half-" } else { "Fully " },
        if *open { "Open" } else { "Closed" },
      ),
      Self::Marcato => write!(f, "Marcato"),
      Self::Mordent { upper } => write!(f, "{} Mordent", if *upper { "Upper" } else { "Lower" }),
      Self::Open => write!(f, "Open"),
      Self::Pizzicato => write!(f, "Pizzicato"),
      Self::Plop => write!(f, "Plop"),
      Self::Portamento { from_current, going_up } => write!(
        f,
        "Portamento: {} {}",
        if *going_up { "Going Up" } else { "Going Down" },
        if *from_current { "From Note" } else { "To Note" },
      ),
      Self::Schleifer => write!(f, "Schleifer"),
      Self::Scoop => write!(f, "Scoop"),
      Self::Sforzando => write!(f, "Sforzando"),
      Self::Shake => write!(f, "Shake"),
      Self::Smear => write!(f, "Smear"),
      Self::SoftAccent => write!(f, "Soft Accent"),
      Self::Spiccato => write!(f, "Spiccato"),
      Self::Staccato => write!(f, "Staccato"),
      Self::Staccatissimo => write!(f, "Staccatissimo"),
      Self::Stopped => write!(f, "Stopped"),
      Self::Stress => write!(f, "Stress"),
      Self::Tap => write!(f, "Tap"),
      Self::Tenuto => write!(f, "Tenuto"),
      Self::ThumbPosition => write!(f, "Thumb Position"),
      Self::Tie => write!(f, "Tied"),
      Self::Toe => write!(f, "Toe"),
      Self::Tremolo { relative_speed } => write!(f, "Tremolo at {relative_speed}x speed"),
      Self::Trill { upper } => write!(f, "{} Trill", if *upper { "Upper" } else { "Lower" }),
      Self::TripleTongue => write!(f, "Triple Tongue"),
      Self::Turn {
        upper,
        delayed,
        vertical,
      } => write!(
        f,
        "{} {}{}Turn",
        if *upper { "Upper" } else { "Lower" },
        if *delayed { "Delayed " } else { "" },
        if *vertical { "Vertical " } else { "" }
      ),
      Self::Unstress => write!(f, "Unstress"),
      Self::UpBow => write!(f, "Up Bow"),
    }
  }
}
