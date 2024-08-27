use crate::note::Duration;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempo {
  pub base_note: Duration,
  pub beats_per_minute: u16,
}

impl Tempo {
  #[must_use]
  pub fn new(base_note: Duration, beats_per_minute: u16) -> Self {
    Self {
      base_note,
      beats_per_minute,
    }
  }
}

impl Default for Tempo {
  fn default() -> Self {
    Self {
      base_note: Duration::Quarter(0),
      beats_per_minute: 120,
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Tempo {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}={} bpm", self.base_note, self.beats_per_minute)
  }
}
