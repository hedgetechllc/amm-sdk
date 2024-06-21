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
  pub fn is_rest(&self) -> bool {
    std::mem::discriminant(self) == std::mem::discriminant(&Pitch::Rest)
  }

  pub fn value(&self) -> (usize, i16) {
    match *self {
      Pitch::Rest => (0, 0),
      Pitch::A(octave) => (1, i16::from(0 + (12 * octave)) - 48),
      Pitch::B(octave) => (2, i16::from(2 + (12 * octave)) - 48),
      Pitch::C(octave) => (3, i16::from(3 + (12 * octave)) - 60),
      Pitch::D(octave) => (4, i16::from(5 + (12 * octave)) - 60),
      Pitch::E(octave) => (5, i16::from(7 + (12 * octave)) - 60),
      Pitch::F(octave) => (6, i16::from(8 + (12 * octave)) - 60),
      Pitch::G(octave) => (7, i16::from(10 + (12 * octave)) - 60),
    }
  }
}

impl std::fmt::Display for Pitch {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match *self {
        Pitch::Rest => format!(""),
        Pitch::A(octave) => format!("A{}", octave),
        Pitch::B(octave) => format!("B{}", octave),
        Pitch::C(octave) => format!("C{}", octave),
        Pitch::D(octave) => format!("D{}", octave),
        Pitch::E(octave) => format!("E{}", octave),
        Pitch::F(octave) => format!("F{}", octave),
        Pitch::G(octave) => format!("G{}", octave),
      }
    )
  }
}
