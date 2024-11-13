use super::chord::ChordModificationType;
use crate::context::{generate_id, Dynamic};
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize, ModOrder};

/// Represents a technique used in handbell playing.
#[derive(Copy, Clone, Debug, Eq, PartialEq, ModOrder, JsonDeserialize, JsonSerialize)]
pub enum HandbellTechnique {
  /// <span class="smufl">&#xE81F;</span>
  Belltree,
  /// <span class="smufl">&#xE81E;</span>
  Damp,
  /// <span class="smufl">&#xE81B;</span>
  Echo,
  /// <span class="smufl">&#xE81D;</span>
  Gyro,
  /// <span class="smufl">&#xE812;</span>
  HandMartellato,
  /// <span class="smufl">&#xE816;</span>
  MalletLift,
  /// <span class="smufl">&#xE815;</span>
  MalletTable,
  /// <span class="smufl">&#xE810;</span>
  Martellato,
  /// <span class="smufl">&#xE811;</span>
  MartellatoLift,
  /// <span class="smufl">&#xE813;</span>
  MutedMartellato,
  /// <span class="smufl">&#xE817;</span>
  PluckLift,
  /// <span class="smufl">&#xE81A;</span>
  Swing,
}

/// Represents a type of modification to a note.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ModOrder, JsonDeserialize, JsonSerialize)]
pub enum NoteModificationType {
  /// ![Accent](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/accent.png)
  #[default]
  Accent,
  /// ![Brass Bend](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/brass-bend.png)
  BrassBend,
  /// ![Detached Legato](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/detached-legato.png)
  DetachedLegato,
  /// ![Doit](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/doit.png)
  Doit,
  /// ![Double Tongue](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/double-tongue.png)
  DoubleTongue,
  /// ![Down Bow](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/down-bow.png)
  DownBow,
  /// Represents a dynamic change that only affects the current note.
  Dynamic { dynamic: Dynamic },
  /// ![Falloff](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/falloff.png)
  Falloff,
  /// ![Fermata](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/fermata.png)
  Fermata,
  /// ![Fingernails](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/fingernails.png)
  Fingernails,
  /// ![Flip](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/flip.png)
  Flip,
  /// ![Glissando Up](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/glissando.png)
  Glissando { from_current: bool, going_up: bool },
  /// ![Golpe](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/golpe.png)
  Golpe,
  /// ![Grace Acciaccatura](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/grace-acciaccatura.png)
  ///
  /// ![Grace Appoggiatura](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/grace-appoggiatura.png)
  Grace { acciaccatura: bool },
  /// ![Half Muted](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/half-muted.png)
  HalfMuted,
  /// Represents a [HandbellTechnique] used in handbell playing.
  Handbell { technique: HandbellTechnique },
  /// Open: <span class="smufl">&#xE5EB;</span>
  ///
  /// Half: <span class="smufl">&#xE5E9;</span>
  ///
  /// Closed: <span class="smufl">&#xE5E8;</span>
  HarmonMute { open: bool, half: bool },
  /// ![Haydn](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/haydn.png)
  Haydn,
  /// ![Heel](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/heel.png)
  Heel,
  /// Open: <span class="smufl">&#xE5F9;</span>
  ///
  /// Half: <span class="smufl">&#xE5F6;</span>
  ///
  /// Closed: <span class="smufl">&#xE5F4;</span>
  Hole { open: bool, half: bool },
  /// ![Marcato](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/accent.png)
  Marcato,
  /// ![Mordent Upper](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/inverted-mordent.png)
  ///
  /// ![Mordent Lower](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/mordent.png)
  Mordent { upper: bool },
  /// ![Open](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/open.png)
  Open,
  /// ![Pizzicato](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/snap-pizzicato.png)
  Pizzicato,
  /// ![Plop](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/plop.png)
  Plop,
  /// ![Portamento Up](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/slide.png)
  Portamento { from_current: bool, going_up: bool },
  /// ![Schleifer](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/schleifer.png)
  Schleifer,
  /// ![Scoop](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/scoop.png)
  Scoop,
  /// ![Sforzando](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/sfz.png)
  Sforzando,
  /// ![Shake](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/shake.png)
  Shake,
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
  /// ![Stopped](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/stopped.png)
  Stopped,
  /// ![Stress](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/stress.png)
  Stress,
  /// ![Tap](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/tap.png)
  Tap,
  /// ![Tenuto](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/tenuto.png)
  Tenuto,
  /// ![Thumb Position](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/thumb-position.png)
  ThumbPosition,
  /// ![Tie](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/tied.png)
  Tie,
  /// ![Toe](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/toe.png)
  Toe,
  /// ![Tremolo](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/tremolo.png)
  Tremolo { relative_speed: u8 },
  /// ![Trill](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/trill-mark.png)
  Trill { upper: bool },
  /// ![Triple Tongue](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/triple-tongue.png)
  TripleTongue,
  /// ![Turn Upper](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/turn.png)
  ///
  /// ![Turn Lower](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/inverted-turn.png)
  ///
  /// ![Vertical Turn Upper](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/vertical-turn.png)
  Turn { upper: bool, delayed: bool, vertical: bool },
  /// ![Unstress](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/unstress.png)
  Unstress,
  /// ![Up Bow](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/up-bow.png)
  UpBow,
}

/// Represents a modification to a note.
#[derive(Debug, Default, Eq, JsonDeserialize, JsonSerialize)]
pub struct NoteModification {
  /// The unique identifier for this modification.
  id: usize,
  /// The type of note modification.
  pub r#type: NoteModificationType,
}

impl NoteModification {
  /// Creates a new note modification based on the given type.
  #[must_use]
  pub fn new(r#type: NoteModificationType) -> Self {
    Self {
      id: generate_id(),
      r#type,
    }
  }

  /// Creates a new note modification based on the given
  /// chord modification type.
  ///
  /// Many chord modifications have direct corresponding
  /// note modifications, and this function provides a
  /// convenient way to convert between the two.
  ///
  /// Returns `None` if the chord modification type does
  /// not have a corresponding note modification type.
  #[must_use]
  pub fn from_chord_modification(r#type: &ChordModificationType) -> Option<Self> {
    match *r#type {
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
    .map(|r#type| Self {
      id: generate_id(),
      r#type,
    })
  }

  /// Returns the unique identifier for this modification.
  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }
}

impl Clone for NoteModification {
  fn clone(&self) -> Self {
    Self {
      id: generate_id(),
      r#type: self.r#type,
    }
  }
}

impl PartialEq for NoteModification {
  fn eq(&self, other: &Self) -> bool {
    self.r#type == other.r#type
  }
}

impl Ord for NoteModification {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.r#type.cmp(&other.r#type)
  }
}

impl PartialOrd for NoteModification {
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
  }
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
      Self::Grace { acciaccatura } => write!(
        f,
        "Grace {}",
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
      Self::Mordent { upper } => write!(f, "Mordent: {}", if *upper { "Upper" } else { "Lower" }),
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
      Self::Tremolo { relative_speed } => write!(f, "Tremolo: {relative_speed}x speed"),
      Self::Trill { upper } => write!(f, "Trill: {}", if *upper { "Upper" } else { "Lower" }),
      Self::TripleTongue => write!(f, "Triple Tongue"),
      Self::Turn {
        upper,
        delayed,
        vertical,
      } => write!(
        f,
        "Turn: {} {}{}",
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
impl core::fmt::Display for NoteModification {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.r#type)
  }
}
