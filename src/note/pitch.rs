use alloc::string::String;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Pitch {
  Rest,
  A(u8),
  B(u8),
  C(u8),
  D(u8),
  E(u8),
  F(u8),
  G(u8),
}

impl Pitch {
  #[must_use]
  pub fn is_rest(&self) -> bool {
    core::mem::discriminant(self) == core::mem::discriminant(&Self::Rest)
  }

  #[must_use]
  pub fn value(&self) -> (usize, i16) {
    match self {
      Self::Rest => (0, 0),
      Self::A(octave) => (1, i16::from(12 * octave) - 48),
      Self::B(octave) => (2, i16::from(2 + (12 * octave)) - 48),
      Self::C(octave) => (3, i16::from(3 + (12 * octave)) - 60),
      Self::D(octave) => (4, i16::from(5 + (12 * octave)) - 60),
      Self::E(octave) => (5, i16::from(7 + (12 * octave)) - 60),
      Self::F(octave) => (6, i16::from(8 + (12 * octave)) - 60),
      Self::G(octave) => (7, i16::from(10 + (12 * octave)) - 60),
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Pitch {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Rest => String::new(),
        Self::A(octave) => format!("A{octave}"),
        Self::B(octave) => format!("B{octave}"),
        Self::C(octave) => format!("C{octave}"),
        Self::D(octave) => format!("D{octave}"),
        Self::E(octave) => format!("E{octave}"),
        Self::F(octave) => format!("F{octave}"),
        Self::G(octave) => format!("G{octave}"),
      }
    )
  }
}
