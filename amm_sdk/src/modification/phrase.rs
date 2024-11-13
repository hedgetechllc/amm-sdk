use crate::context::{generate_id, Dynamic};
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize, ModOrder};

/// Represents a type of pedal used in piano playing.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ModOrder, JsonDeserialize, JsonSerialize)]
pub enum PedalType {
  /// ![Sustain](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/pedal.png)
  #[default]
  Sustain,
  /// Usually denoted as `sost.` in sheet music.
  Sostenuto,
  /// Usually denoted as `una corda` in sheet music.
  Soft,
}

/// Represents a type of modification to a phrase.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ModOrder, JsonDeserialize, JsonSerialize)]
pub enum PhraseModificationType {
  /// ![Crescendo](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/crescendo.png)
  ///
  /// Represents a gradual increase in volume over a series of notes.
  ///
  /// The `final_dynamic` field is optional and represents the dynamic
  /// level that the crescendo should reach by the end of the phrase.
  Crescendo { final_dynamic: Option<Dynamic> },
  /// ![Decrescendo](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/decrescendo.png)
  ///
  /// Represents a gradual decrease in volume over a series of notes.
  ///
  /// The `final_dynamic` field is optional and represents the dynamic
  /// level that the decrescendo should reach by the end of the phrase.
  Decrescendo { final_dynamic: Option<Dynamic> },
  /// ![Glissando](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/glissando.png)
  Glissando,
  /// ![Hairpin](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/hairpin.png)
  ///
  /// Represents a gradual increase in volume followed by a gradual
  /// decrease in volume over a series of notes.
  ///
  /// The `maximum_dynamic` field is optional and represents the dynamic
  /// level that the hairpin should reach by the middle of the phrase.
  Hairpin { maximum_dynamic: Option<Dynamic> },
  /// ![Legato](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/slur.png)
  #[default]
  Legato,
  /// ![Octave Shift Up](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/octave-shift.png)
  OctaveShift { num_octaves: i8 },
  /// Represents a change in the use of a pedal for the current phrase.
  Pedal { pedal_type: PedalType },
  /// ![Portamento](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/slide.png)
  Portamento,
  /// ![Tremolo](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/tremolo.png)
  Tremolo { relative_speed: u8 },
  /// ![Triplet](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/tuplet.png)
  ///
  /// ![Quintuplet](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/quintuplet.png)
  Tuplet { num_beats: u8, into_beats: u8 },
}

/// Represents a modification to a phrase.
#[derive(Debug, Default, Eq, JsonDeserialize, JsonSerialize)]
pub struct PhraseModification {
  /// The unique identifier for this modification.
  id: usize,
  /// The type of phrase modification.
  pub r#type: PhraseModificationType,
}

impl PhraseModification {
  /// Creates a new phrase modification based on the given type.
  #[must_use]
  pub fn new(r#type: PhraseModificationType) -> Self {
    Self {
      id: generate_id(),
      r#type,
    }
  }

  /// Returns the unique identifier for this modification.
  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }
}

impl Clone for PhraseModification {
  fn clone(&self) -> Self {
    Self {
      id: generate_id(),
      r#type: self.r#type,
    }
  }
}

impl PartialEq for PhraseModification {
  fn eq(&self, other: &Self) -> bool {
    self.r#type == other.r#type
  }
}

impl Ord for PhraseModification {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.r#type.cmp(&other.r#type)
  }
}

impl PartialOrd for PhraseModification {
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for PedalType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Sustain => "Sustain",
        Self::Sostenuto => "Sostenuto",
        Self::Soft => "Soft",
      }
    )
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for PhraseModificationType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match self {
      Self::Crescendo {
        final_dynamic: Some(dynamic),
      } => write!(f, "Crescendo to {dynamic}"),
      Self::Crescendo { final_dynamic: None } => write!(f, "Crescendo"),
      Self::Decrescendo {
        final_dynamic: Some(dynamic),
      } => write!(f, "Decrescendo to {dynamic}"),
      Self::Decrescendo { final_dynamic: None } => write!(f, "Decrescendo"),
      Self::Glissando => write!(f, "Glissando"),
      Self::Hairpin {
        maximum_dynamic: Some(dynamic),
      } => write!(f, "Hairpin to {dynamic}"),
      Self::Hairpin { maximum_dynamic: None } => write!(f, "Hairpin"),
      Self::Legato => write!(f, "Legato"),
      Self::OctaveShift { num_octaves } => write!(f, "Octave Shift: by {num_octaves}"),
      Self::Pedal { pedal_type } => write!(f, "Pedal: {pedal_type}"),
      Self::Portamento => write!(f, "Portamento"),
      Self::Tremolo { relative_speed } => write!(f, "Tremolo: {relative_speed}x speed"),
      Self::Tuplet { num_beats, into_beats } => write!(f, "Tuplet: {num_beats}:{into_beats}"),
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for PhraseModification {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.r#type)
  }
}
