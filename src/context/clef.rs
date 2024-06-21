#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ClefType {
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
    match *self {
      Clef::Treble => ClefType::GClef,
      Clef::Bass => ClefType::FClef,
      Clef::FrenchViolin => ClefType::GClef,
      Clef::Subbass => ClefType::FClef,
      Clef::Tenor => ClefType::CClef,
      Clef::Alto => ClefType::CClef,
      Clef::Soprano => ClefType::CClef,
      Clef::MezzoSoprano => ClefType::CClef,
      Clef::Baritone(clef_type) => clef_type,
    }
  }
}

impl std::fmt::Display for Clef {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match *self {
        Clef::Treble => format!("Treble"),
        Clef::Bass => format!("Bass"),
        Clef::FrenchViolin => format!("French Violin"),
        Clef::Subbass => format!("Subbass"),
        Clef::Tenor => format!("Tenor"),
        Clef::Alto => format!("Alto"),
        Clef::Soprano => format!("Soprano"),
        Clef::MezzoSoprano => format!("Mezzo Soprano"),
        Clef::Baritone(clef_type) => format!(
          "Baritone ({})",
          if clef_type == ClefType::CClef {
            "C-Clef"
          } else {
            "F-Clef"
          }
        ),
      }
    )
  }
}
