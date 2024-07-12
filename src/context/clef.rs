#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum ClefType {
  #[default]
  GClef,
  CClef,
  FClef,
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum Clef {
  #[default]
  Treble,
  Bass,
  FrenchViolin,
  Subbass,
  Tenor,
  Alto,
  Soprano,
  MezzoSoprano,
  Baritone(ClefType),
}

impl Clef {
  pub fn clef_type(&self) -> ClefType {
    match self {
      Self::Treble => ClefType::GClef,
      Self::Bass => ClefType::FClef,
      Self::FrenchViolin => ClefType::GClef,
      Self::Subbass => ClefType::FClef,
      Self::Tenor => ClefType::CClef,
      Self::Alto => ClefType::CClef,
      Self::Soprano => ClefType::CClef,
      Self::MezzoSoprano => ClefType::CClef,
      Self::Baritone(clef_type) => *clef_type,
    }
  }
}

impl std::fmt::Display for Clef {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Treble => "Treble",
        Self::Bass => "Bass",
        Self::FrenchViolin => "French Violin",
        Self::Subbass => "Subbass",
        Self::Tenor => "Tenor",
        Self::Alto => "Alto",
        Self::Soprano => "Soprano",
        Self::MezzoSoprano => "Mezzo Soprano",
        Self::Baritone(clef_type) => match clef_type {
          ClefType::GClef => "Baritone (G-Clef)",
          ClefType::CClef => "Baritone (C-Clef)",
          ClefType::FClef => "Baritone (F-Clef)",
        },
      }
    )
  }
}
