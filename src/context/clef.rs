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
  #[must_use]
  pub fn clef_type(&self) -> ClefType {
    match self {
      Self::Treble | Self::FrenchViolin => ClefType::GClef,
      Self::Bass | Self::Subbass => ClefType::FClef,
      Self::Tenor | Self::Alto | Self::Soprano | Self::MezzoSoprano => ClefType::CClef,
      Self::Baritone(clef_type) => *clef_type,
    }
  }
}

impl core::fmt::Display for Clef {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
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
