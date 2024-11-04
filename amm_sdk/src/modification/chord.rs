use super::note::NoteModificationType;
use crate::context::{generate_id, Dynamic};
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize, ModOrder};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ModOrder, JsonDeserialize, JsonSerialize)]
pub enum ChordModificationType {
  #[default]
  Accent,
  Arpeggiate,
  DetachedLegato,
  DownBow,
  Dynamic {
    dynamic: Dynamic,
  },
  Fermata,
  Fingernails,
  HalfMuted,
  HarmonMute {
    open: bool,
    half: bool,
  },
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
  Tremolo {
    relative_speed: u8,
  },
  Unstress,
  UpBow,
}

#[derive(Debug, Default, Eq, JsonDeserialize, JsonSerialize)]
pub struct ChordModification {
  id: usize,
  pub r#type: ChordModificationType,
}

impl ChordModification {
  #[must_use]
  pub fn new(r#type: ChordModificationType) -> Self {
    Self {
      id: generate_id(),
      r#type,
    }
  }

  #[must_use]
  pub fn from_note_modification(r#type: &NoteModificationType) -> Option<Self> {
    match *r#type {
      NoteModificationType::Accent => Some(ChordModificationType::Accent),
      NoteModificationType::DetachedLegato => Some(ChordModificationType::DetachedLegato),
      NoteModificationType::DownBow => Some(ChordModificationType::DownBow),
      NoteModificationType::Dynamic { dynamic } => Some(ChordModificationType::Dynamic { dynamic }),
      NoteModificationType::Fermata => Some(ChordModificationType::Fermata),
      NoteModificationType::Fingernails => Some(ChordModificationType::Fingernails),
      NoteModificationType::HalfMuted => Some(ChordModificationType::HalfMuted),
      NoteModificationType::HarmonMute { open, half } => Some(ChordModificationType::HarmonMute { open, half }),
      NoteModificationType::Heel => Some(ChordModificationType::Heel),
      NoteModificationType::Marcato => Some(ChordModificationType::Marcato),
      NoteModificationType::Open => Some(ChordModificationType::Open),
      NoteModificationType::Pizzicato => Some(ChordModificationType::Pizzicato),
      NoteModificationType::Sforzando => Some(ChordModificationType::Sforzando),
      NoteModificationType::Smear => Some(ChordModificationType::Smear),
      NoteModificationType::SoftAccent => Some(ChordModificationType::SoftAccent),
      NoteModificationType::Spiccato => Some(ChordModificationType::Spiccato),
      NoteModificationType::Staccato => Some(ChordModificationType::Staccato),
      NoteModificationType::Staccatissimo => Some(ChordModificationType::Staccatissimo),
      NoteModificationType::Stress => Some(ChordModificationType::Stress),
      NoteModificationType::Tenuto => Some(ChordModificationType::Tenuto),
      NoteModificationType::Toe => Some(ChordModificationType::Toe),
      NoteModificationType::Tremolo { relative_speed } => Some(ChordModificationType::Tremolo { relative_speed }),
      NoteModificationType::Unstress => Some(ChordModificationType::Unstress),
      NoteModificationType::UpBow => Some(ChordModificationType::UpBow),
      _ => None,
    }
    .map(|mod_type| Self {
      id: generate_id(),
      r#type: mod_type,
    })
  }

  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }
}

impl Clone for ChordModification {
  fn clone(&self) -> Self {
    Self {
      id: generate_id(),
      r#type: self.r#type,
    }
  }
}

impl PartialEq for ChordModification {
  fn eq(&self, other: &Self) -> bool {
    self.r#type == other.r#type
  }
}

impl Ord for ChordModification {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.r#type.cmp(&other.r#type)
  }
}

impl PartialOrd for ChordModification {
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for ChordModification {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.r#type)
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
      Self::Tremolo { relative_speed } => write!(f, "Tremolo: {relative_speed}x speed"),
      Self::Unstress => write!(f, "Unstress"),
      Self::UpBow => write!(f, "Up Bow"),
    }
  }
}
