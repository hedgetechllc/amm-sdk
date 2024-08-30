use super::chord::ChordModificationType;
use crate::context::{generate_id, Dynamic};
use crate::storage::{Serialize, SerializedItem};
use alloc::{collections::BTreeMap, rc::Rc};
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

#[cfg(feature = "print")]
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
  Dynamic { dynamic: Dynamic },
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
  pub fn from_chord_modification(modification: &ChordModificationType) -> Option<Rc<RefCell<Self>>> {
    match *modification {
      ChordModificationType::Accent => Some(NoteModificationType::Accent),
      ChordModificationType::DetachedLegato => Some(NoteModificationType::DetachedLegato),
      ChordModificationType::DownBow => Some(NoteModificationType::DownBow),
      ChordModificationType::Dynamic { dynamic } => Some(NoteModificationType::Dynamic { dynamic }),
      ChordModificationType::Fermata => Some(NoteModificationType::Fermata),
      ChordModificationType::Fingernails => Some(NoteModificationType::Fingernails),
      ChordModificationType::HalfMuted => Some(NoteModificationType::HalfMuted),
      ChordModificationType::HarmonMute { open, half } => Some(NoteModificationType::HarmonMute { open, half }),
      ChordModificationType::Heel => Some(NoteModificationType::Heel),
      ChordModificationType::Marcato => Some(NoteModificationType::Marcato),
      ChordModificationType::Open => Some(NoteModificationType::Open),
      ChordModificationType::Pizzicato => Some(NoteModificationType::Pizzicato),
      ChordModificationType::Sforzando => Some(NoteModificationType::Sforzando),
      ChordModificationType::Smear => Some(NoteModificationType::Smear),
      ChordModificationType::SoftAccent => Some(NoteModificationType::SoftAccent),
      ChordModificationType::Spiccato => Some(NoteModificationType::Spiccato),
      ChordModificationType::Staccato => Some(NoteModificationType::Staccato),
      ChordModificationType::Staccatissimo => Some(NoteModificationType::Staccatissimo),
      ChordModificationType::Stress => Some(NoteModificationType::Stress),
      ChordModificationType::Tenuto => Some(NoteModificationType::Tenuto),
      ChordModificationType::Tie => Some(NoteModificationType::Tie),
      ChordModificationType::Toe => Some(NoteModificationType::Toe),
      ChordModificationType::Tremolo { relative_speed } => Some(NoteModificationType::Tremolo { relative_speed }),
      ChordModificationType::Unstress => Some(NoteModificationType::Unstress),
      ChordModificationType::UpBow => Some(NoteModificationType::UpBow),
      _ => None,
    }
    .map(|modification| {
      Rc::new(RefCell::new(Self {
        id: generate_id(),
        modification,
      }))
    })
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

#[cfg(feature = "print")]
impl core::fmt::Display for NoteModification {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.modification)
  }
}

#[cfg(feature = "print")]
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

#[cfg(feature = "print")]
impl Serialize for NoteModification {
  fn serialize(&self) -> SerializedItem {
    let (name, serialized) = match &self.modification {
      NoteModificationType::Dynamic { dynamic } => (String::from("Dynamic"), dynamic.serialize()),
      NoteModificationType::Glissando { from_current, going_up } => (
        String::from("Glissando"),
        SerializedItem {
          attributes: BTreeMap::from([
            (String::from("from_current"), from_current.to_string()),
            (String::from("going_up"), going_up.to_string()),
          ]),
          contents: BTreeMap::new(),
          elements: BTreeMap::new(),
        },
      ),
      NoteModificationType::Grace {
        acciaccatura,
        note_value,
      } => (
        String::from("Grace"),
        SerializedItem {
          attributes: BTreeMap::from([
            (String::from("acciaccatura"), acciaccatura.to_string()),
            (String::from("note_value"), note_value.to_string()),
          ]),
          contents: BTreeMap::new(),
          elements: BTreeMap::new(),
        },
      ),
      NoteModificationType::Handbell { technique } => (
        String::from("Handbell"),
        SerializedItem {
          attributes: BTreeMap::from([(String::from("technique"), technique.to_string())]),
          contents: BTreeMap::new(),
          elements: BTreeMap::new(),
        },
      ),
      NoteModificationType::HarmonMute { open, half } => (
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
      NoteModificationType::Hole { open, half } => (
        String::from("Hole"),
        SerializedItem {
          attributes: BTreeMap::from([
            (String::from("open"), open.to_string()),
            (String::from("half"), half.to_string()),
          ]),
          contents: BTreeMap::new(),
          elements: BTreeMap::new(),
        },
      ),
      NoteModificationType::Mordent { upper } => (
        String::from("Mordent"),
        SerializedItem {
          attributes: BTreeMap::from([(String::from("upper"), upper.to_string())]),
          contents: BTreeMap::new(),
          elements: BTreeMap::new(),
        },
      ),
      NoteModificationType::Portamento { from_current, going_up } => (
        String::from("Portamento"),
        SerializedItem {
          attributes: BTreeMap::from([
            (String::from("from_current"), from_current.to_string()),
            (String::from("going_up"), going_up.to_string()),
          ]),
          contents: BTreeMap::new(),
          elements: BTreeMap::new(),
        },
      ),
      NoteModificationType::Tremolo { relative_speed } => (
        String::from("Tremolo"),
        SerializedItem {
          attributes: BTreeMap::from([(String::from("relative_speed"), relative_speed.to_string())]),
          contents: BTreeMap::new(),
          elements: BTreeMap::new(),
        },
      ),
      NoteModificationType::Trill { upper } => (
        String::from("Trill"),
        SerializedItem {
          attributes: BTreeMap::from([(String::from("upper"), upper.to_string())]),
          contents: BTreeMap::new(),
          elements: BTreeMap::new(),
        },
      ),
      NoteModificationType::Turn {
        upper,
        delayed,
        vertical,
      } => (
        String::from("Turn"),
        SerializedItem {
          attributes: BTreeMap::from([
            (String::from("upper"), upper.to_string()),
            (String::from("delayed"), delayed.to_string()),
            (String::from("vertical"), vertical.to_string()),
          ]),
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
