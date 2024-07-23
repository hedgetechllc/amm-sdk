#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TempoMarking {
  Larghissimo,
  Grave,
  Largo,
  Lento,
  Larghetto,
  Adagio,
  Adagietto,
  Andante,
  Andantino,
  MarciaModerato,
  AndanteModerato,
  Moderato,
  Allegretto,
  AllegroModerato,
  Allegro,
  Vivace,
  Vivacissimo,
  Allegrissimo,
  AllegroVivace,
  Presto,
  Prestissimo,
}

impl TempoMarking {
  pub fn description(&self) -> &str {
    match self {
      Self::Larghissimo => "very, very slowly",
      Self::Grave => "very slowly",
      Self::Largo => "broadly",
      Self::Lento => "slowly",
      Self::Larghetto => "rather broadly",
      Self::Adagio => "slowly and stately",
      Self::Adagietto => "more slowly than andante",
      Self::Andante => "at a walking pace",
      Self::Andantino => "at a brisk walking pace",
      Self::MarciaModerato => "moderately, in the manner of a march",
      Self::AndanteModerato => "between andante and moderato",
      Self::Moderato => "moderately",
      Self::Allegretto => "moderately quickly",
      Self::AllegroModerato => "brightly and moderately quickly",
      Self::Allegro => "quickly and brightly",
      Self::Vivace => "lively and fast",
      Self::Vivacissimo => "very fast and lively",
      Self::Allegrissimo => "very fast",
      Self::AllegroVivace => "very fast",
      Self::Presto => "very, very fast",
      Self::Prestissimo => "extremely fast",
    }
  }

  pub fn bpm_range(&self) -> (u16, u16) {
    match self {
      Self::Larghissimo => (10, 24),
      Self::Grave => (25, 45),
      Self::Largo => (40, 60),
      Self::Lento => (45, 60),
      Self::Larghetto => (60, 66),
      Self::Adagio => (66, 76),
      Self::Adagietto => (72, 76),
      Self::Andante => (76, 108),
      Self::Andantino => (80, 108),
      Self::MarciaModerato => (83, 85),
      Self::AndanteModerato => (92, 112),
      Self::Moderato => (108, 120),
      Self::Allegretto => (112, 120),
      Self::AllegroModerato => (116, 120),
      Self::Allegro => (120, 168),
      Self::Vivace => (168, 176),
      Self::Vivacissimo => (172, 176),
      Self::Allegrissimo => (172, 176),
      Self::AllegroVivace => (172, 176),
      Self::Presto => (168, 200),
      Self::Prestissimo => (200, 240),
    }
  }

  pub fn value(&self) -> u16 {
    match self {
      Self::Larghissimo => 22,
      Self::Grave => 35,
      Self::Largo => 50,
      Self::Lento => 55,
      Self::Larghetto => 63,
      Self::Adagio => 70,
      Self::Adagietto => 74,
      Self::Andante => 86,
      Self::Andantino => 94,
      Self::MarciaModerato => 84,
      Self::AndanteModerato => 102,
      Self::Moderato => 114,
      Self::Allegretto => 116,
      Self::AllegroModerato => 118,
      Self::Allegro => 140,
      Self::Vivace => 172,
      Self::Vivacissimo => 174,
      Self::Allegrissimo => 174,
      Self::AllegroVivace => 174,
      Self::Presto => 190,
      Self::Prestissimo => 220,
    }
  }
}

impl std::fmt::Display for TempoMarking {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Larghissimo => "Larghissimo",
        Self::Grave => "Grave",
        Self::Largo => "Largo",
        Self::Lento => "Lento",
        Self::Larghetto => "Larghetto",
        Self::Adagio => "Adagio",
        Self::Adagietto => "Adagietto",
        Self::Andante => "Andante",
        Self::Andantino => "Andantino",
        Self::MarciaModerato => "Marcia Moderato",
        Self::AndanteModerato => "Andante Moderato",
        Self::Moderato => "Moderato",
        Self::Allegretto => "Allegretto",
        Self::AllegroModerato => "Allegro Moderato",
        Self::Allegro => "Allegro",
        Self::Vivace => "Vivace",
        Self::Vivacissimo => "Vivacissimo",
        Self::Allegrissimo => "Allegrissimo",
        Self::AllegroVivace => "Allegro Vivace",
        Self::Presto => "Presto",
        Self::Prestissimo => "Prestissimo",
      }
    )
  }
}
