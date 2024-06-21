#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum Accidental {
  #[default]
  None,
  Natural,
  Sharp,
  Flat,
  DoubleSharp,
  DoubleFlat,
  NaturalSharp,
  NaturalFlat,
}

impl Accidental {
  pub fn value(&self) -> i16 {
    match *self {
      Accidental::Sharp => 1,
      Accidental::Flat => -1,
      Accidental::DoubleSharp => 2,
      Accidental::DoubleFlat => -2,
      Accidental::NaturalSharp => 1,
      Accidental::NaturalFlat => -1,
      _ => 0,
    }
  }
}

impl std::fmt::Display for Accidental {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match *self {
        Accidental::Natural => "♮",
        Accidental::Sharp => "♯",
        Accidental::Flat => "♭",
        Accidental::DoubleSharp => "𝄪",
        Accidental::DoubleFlat => "𝄫",
        Accidental::NaturalSharp => "♮♯",
        Accidental::NaturalFlat => "♮♭",
        _ => "",
      }
    )
  }
}
