use crate::context::{generate_id, Tempo, TempoSuggestion};
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};

#[derive(Clone, Eq, Debug, Default, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum SectionModificationType {
  #[default]
  Accelerando, // Quick tempo acceleration over few notes or measures
  OnlyPlay {
    iterations: Vec<u8>,
  },
  Rallentando, // Gradual tempo reduction leading to context change
  Repeat {
    num_times: u8,
  },
  Ritardando, // Gradual tempo reduction leading to complete stop
  Ritenuto,   // Immediate tempo reduction
  Stringendo, // Gradual tempo acceleration leading to context change
  TempoExplicit {
    tempo: Tempo,
  },
  TempoImplicit {
    tempo: TempoSuggestion,
  },
}

#[derive(Debug, Default, Eq, JsonDeserialize, JsonSerialize)]
pub struct SectionModification {
  id: usize,
  pub r#type: SectionModificationType,
}

impl SectionModification {
  #[must_use]
  pub fn new(r#type: SectionModificationType) -> Self {
    Self {
      id: generate_id(),
      r#type,
    }
  }

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

#[cfg(test)]
mod test {
  use super::*;
  use crate::{Duration, DurationType};
  use amm_internal::JsonSerializer;

  #[test]
  fn test_section_mod_serialization_json() {
    let accel = SectionModification::new(SectionModificationType::Accelerando);
    let only_play = SectionModification::new(SectionModificationType::OnlyPlay {
      iterations: vec![1, 3, 4],
    });
    let tempo_explicit = SectionModification::new(SectionModificationType::TempoExplicit {
      tempo: Tempo {
        base_note: Duration {
          value: DurationType::Half,
          dots: 0,
        },
        beats_per_minute: 135,
      },
    });
    assert_eq!(
      JsonSerializer::serialize_json(&accel),
      "{\"id\":1,\"modification\":\"Accelerando\"}"
    );
    assert_eq!(
      JsonSerializer::serialize_json(&only_play),
      "{\"id\":2,\"modification\":{\"type\":\"OnlyPlay\",\"iterations\":[1,3,4]}}"
    );
    assert_eq!(
      JsonSerializer::serialize_json(&tempo_explicit),
      "{\"id\":3,\"modification\":{\"type\":\"TempoExplicit\",\"tempo\":{\"base_note\":{\"type\":\"Half\",\"dots\":0},\"beats_per_minute\":135}}}"
    );
  }
}
