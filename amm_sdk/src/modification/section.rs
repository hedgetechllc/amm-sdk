use crate::context::{generate_id, Tempo, TempoSuggestion};
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize, ModOrder};

/// Represents a type of modification to a section.
#[derive(Clone, Eq, Debug, Default, PartialEq, ModOrder, JsonDeserialize, JsonSerialize)]
pub enum SectionModificationType {
  /// Represents a section with a quick tempo acceleration over
  /// a few notes or measures.
  #[default]
  Accelerando,
  /// Represents a section that should only be played during
  /// certain iterations.
  OnlyPlay { iterations: Vec<u8> },
  /// Represents a section with a gradual tempo reduction
  /// leading to context change.
  Rallentando,
  /// Represents a section that should be repeated a certain
  /// number of times.
  ///
  /// Note that a `num_times` parameter of 1 means that the
  /// section will play a total of two times, while a parameter
  /// of 2 means it will play 3 times, etc.
  Repeat {
    num_times: u8, // Number of times to repeat the section (repeat 1 means play twice, repeat 2 means play 3x)
  },
  /// Represents a section with a gradual tempo reduction
  /// leading to complete stop.
  Ritardando,
  /// Represents a section with an immediate tempo reduction.
  Ritenuto,
  /// Represents a section with a gradual tempo acceleration
  /// leading to context change.
  Stringendo,
  /// Represents a section with an explicit tempo change.
  TempoExplicit { tempo: Tempo },
  /// Represents a section with a suggested tempo change.
  TempoImplicit { tempo: TempoSuggestion },
}

/// Represents a modification to a section.
#[derive(Debug, Default, Eq, JsonDeserialize, JsonSerialize)]
pub struct SectionModification {
  /// The unique identifier for this modification.
  id: usize,
  /// The type of section modification.
  pub r#type: SectionModificationType,
}

impl SectionModification {
  /// Creates a new section modification based on the given type.
  #[must_use]
  pub fn new(r#type: SectionModificationType) -> Self {
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

impl Clone for SectionModification {
  fn clone(&self) -> Self {
    Self {
      id: generate_id(),
      r#type: self.r#type.clone(),
    }
  }
}

impl PartialEq for SectionModification {
  fn eq(&self, other: &Self) -> bool {
    self.r#type == other.r#type
  }
}

impl Ord for SectionModification {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.r#type.cmp(&other.r#type)
  }
}

impl PartialOrd for SectionModification {
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for SectionModificationType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match self {
      Self::Accelerando => write!(f, "Accelerando"),
      Self::OnlyPlay { iterations } => {
        let iterations = iterations
          .iter()
          .map(ToString::to_string)
          .collect::<Vec<_>>()
          .join(", ");
        write!(f, "Only Play: [{iterations}]")
      }
      Self::Rallentando => write!(f, "Rallentando"),
      Self::Repeat { num_times } => write!(f, "Repeat: {num_times} times"),
      Self::Ritardando => write!(f, "Ritardando"),
      Self::Ritenuto => write!(f, "Ritenuto"),
      Self::Stringendo => write!(f, "Stringendo"),
      Self::TempoExplicit { tempo } => write!(f, "Explicit Tempo: {tempo}"),
      Self::TempoImplicit { tempo } => write!(f, "Implicit Tempo: {tempo}"),
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for SectionModification {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.r#type)
  }
}
