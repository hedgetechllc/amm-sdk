use super::note::NoteModificationType;
use crate::context::{generate_id, Dynamic};
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize, ModOrder};

/// Represents a type of modification to a chord.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ModOrder, JsonDeserialize, JsonSerialize)]
pub enum ChordModificationType {
  /// ![Accent](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/accent.png)
  #[default]
  Accent,
  /// ![Arpeggiate](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/arpeggiate.png)
  Arpeggiate,
  /// ![Detached Legato](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/detached-legato.png)
  DetachedLegato,
  /// ![Down Bow](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/down-bow.png)
  DownBow,
  /// Represents a dynamic change that only affects the current chord.
  Dynamic { dynamic: Dynamic },
  /// ![Fermata](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/fermata.png)
  Fermata,
  /// ![Fingernails](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/fingernails.png)
  Fingernails,
  /// ![Half Muted](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/half-muted.png)
  HalfMuted,
  /// Open: <span class="smufl">&#xE5EB;</span>
  ///
  /// Half: <span class="smufl">&#xE5E9;</span>
  ///
  /// Closed: <span class="smufl">&#xE5E8;</span>
  HarmonMute { open: bool, half: bool },
  /// ![Heel](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/heel.png)
  Heel,
  /// ![Marcato](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/accent.png)
  Marcato,
  /// ![Non-Arpeggiate](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/non-arpeggiate.png)
  NonArpeggiate,
  /// ![Open](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/open.png)
  Open,
  /// ![Pizzicato](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/snap-pizzicato.png)
  Pizzicato,
  /// ![Sforzando](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/sfz.png)
  Sforzando,
  /// ![Smear](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/smear.png)
  Smear,
  /// ![Soft Accent](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/soft-accent.png)
  SoftAccent,
  /// ![Spiccato](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/spiccato.png)
  Spiccato,
  /// ![Staccato](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/staccato.png)
  Staccato,
  /// ![Staccatissimo](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/staccatissimo.png)
  Staccatissimo,
  /// ![Stress](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/stress.png)
  Stress,
  /// ![Tenuto](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/tenuto.png)
  Tenuto,
  /// ![Tie](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/tied.png)
  Tie,
  /// ![Toe](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/toe.png)
  Toe,
  /// ![Tremolo](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/tremolo.png)
  Tremolo { relative_speed: u8 },
  /// ![Unstress](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/unstress.png)
  Unstress,
  /// ![Upbow](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/up-bow.png)
  UpBow,
}

/// Represents a modification to a chord.
#[derive(Debug, Default, Eq, JsonDeserialize, JsonSerialize)]
pub struct ChordModification {
  /// The unique identifier for this modification.
  id: usize,
  /// The type of chord modification.
  pub r#type: ChordModificationType,
}

impl ChordModification {
  /// Creates a new chord modification based on the given type.
  #[must_use]
  pub fn new(r#type: ChordModificationType) -> Self {
    Self {
      id: generate_id(),
      r#type,
    }
  }

  /// Creates a new chord modification based on the given
  /// note modification type.
  ///
  /// Many note modifications have direct corresponding
  /// chord modifications, and this function provides a
  /// convenient way to convert between the two.
  ///
  /// Returns `None` if the note modification type does
  /// not have a corresponding chord modification type.
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

  /// Returns the unique identifier for this modification.
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
