use crate::context::{generate_id, Clef, Dynamic, Key, TimeSignature};
use crate::temporal::Timeslice;
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize, ModOrder};

/// Represents a type of contextual direction which changes the global
/// state of the music being played starting at the point that the
/// direction is encountered.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, ModOrder, JsonDeserialize, JsonSerialize)]
pub enum DirectionType {
  /// ![Accordion Registration High](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/accordion-high.png)
  ///
  /// ![Accordion Registration Middle](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/accordion-middle.png)
  ///
  /// ![Accordion Registration Low](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/accordion-low.png)
  AccordionRegistration { high: bool, middle: u8, low: bool },
  /// ![Breath Mark](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/breath-mark.png)
  #[default]
  BreathMark,
  /// ![Caesura](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/caesura.png)
  Caesura,
  /// Represents a change in clef for the current staff.
  ClefChange { clef: Clef },
  /// Represents a change in dynamic level for the current staff.
  Dynamic { dynamic: Dynamic },
  /// Represents a change in key for the current staff.
  KeyChange { key: Key },
  /// ![String Mute](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/string-mute.png)
  StringMute { on: bool },
  /// Represents a change in time signature for the current staff.
  TimeSignatureChange { time_signature: TimeSignature },
}

/// Represents a contextual direction which changes the global state of
/// the music being played starting at the point that the direction is
/// encountered.
#[derive(Debug, Default, Eq, JsonDeserialize, JsonSerialize)]
pub struct Direction {
  /// The unique identifier for this direction.
  id: usize,
  /// The type of direction.
  pub r#type: DirectionType,
}

impl Direction {
  /// Creates a new direction based on the given type.
  #[must_use]
  pub fn new(r#type: DirectionType) -> Self {
    Self {
      id: generate_id(),
      r#type,
    }
  }

  /// Returns the unique identifier for this direction.
  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }

  /// Converts the direction into a [Timeslice].
  #[must_use]
  pub fn to_timeslice(&self) -> Timeslice {
    let mut timeslice = Timeslice::new();
    timeslice.add_direction(self.clone());
    timeslice
  }
}

impl Clone for Direction {
  fn clone(&self) -> Self {
    Self {
      id: generate_id(),
      r#type: self.r#type,
    }
  }
}

impl PartialEq for Direction {
  fn eq(&self, other: &Self) -> bool {
    self.r#type == other.r#type
  }
}

impl Ord for Direction {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.r#type.cmp(&other.r#type)
  }
}

impl PartialOrd for Direction {
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
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
      Self::ClefChange { clef } => write!(f, "Clef: {clef}"),
      Self::Dynamic { dynamic } => write!(f, "Dynamic: {dynamic}"),
      Self::KeyChange { key } => write!(f, "Key: {key}"),
      Self::StringMute { on } => write!(f, "String Mute: {}", if *on { "on" } else { "off" }),
      Self::TimeSignatureChange { time_signature } => write!(f, "Time Signature: {time_signature}"),
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Direction {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.r#type)
  }
}
