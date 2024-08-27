#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum DynamicMarking {
  #[default]
  None,
  Forte(u8),
  MezzoForte,
  MezzoPiano,
  Piano(u8),
}

impl DynamicMarking {
  #[must_use]
  pub fn value(&self) -> f32 {
    match *self {
      Self::Piano(degree) => (0.5 - (0.1 * f32::from(degree))).max(0.05),
      Self::MezzoPiano => 0.45,
      Self::MezzoForte => 0.55,
      Self::Forte(degree) => (0.5 + (0.1 * f32::from(degree))).min(1.0),
      Self::None => 0.5,
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for DynamicMarking {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match *self {
      Self::Piano(degree) => write!(f, "{}", "p".repeat(usize::from(degree))),
      Self::MezzoPiano => write!(f, "mp"),
      Self::MezzoForte => write!(f, "mf"),
      Self::Forte(degree) => write!(f, "{}", "f".repeat(usize::from(degree))),
      Self::None => write!(f, ""),
    }
  }
}
