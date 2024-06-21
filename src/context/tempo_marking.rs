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
    match *self {
      TempoMarking::Larghissimo => "very, very slowly",
      TempoMarking::Grave => "very slowly",
      TempoMarking::Largo => "broadly",
      TempoMarking::Lento => "slowly",
      TempoMarking::Larghetto => "rather broadly",
      TempoMarking::Adagio => "slowly and stately",
      TempoMarking::Adagietto => "more slowly than andante",
      TempoMarking::Andante => "at a walking pace",
      TempoMarking::Andantino => "at a brisk walking pace",
      TempoMarking::MarciaModerato => "moderately, in the manner of a march",
      TempoMarking::AndanteModerato => "between andante and moderato",
      TempoMarking::Moderato => "moderately",
      TempoMarking::Allegretto => "moderately quickly",
      TempoMarking::AllegroModerato => "brightly and moderately quickly",
      TempoMarking::Allegro => "quickly and brightly",
      TempoMarking::Vivace => "lively and fast",
      TempoMarking::Vivacissimo => "very fast and lively",
      TempoMarking::Allegrissimo => "very fast",
      TempoMarking::AllegroVivace => "very fast",
      TempoMarking::Presto => "very, very fast",
      TempoMarking::Prestissimo => "extremely fast",
    }
  }

  pub fn bpm_range(&self) -> (u16, u16) {
    match *self {
      TempoMarking::Larghissimo => (10, 24),
      TempoMarking::Grave => (25, 45),
      TempoMarking::Largo => (40, 60),
      TempoMarking::Lento => (45, 60),
      TempoMarking::Larghetto => (60, 66),
      TempoMarking::Adagio => (66, 76),
      TempoMarking::Adagietto => (72, 76),
      TempoMarking::Andante => (76, 108),
      TempoMarking::Andantino => (80, 108),
      TempoMarking::MarciaModerato => (83, 85),
      TempoMarking::AndanteModerato => (92, 112),
      TempoMarking::Moderato => (108, 120),
      TempoMarking::Allegretto => (112, 120),
      TempoMarking::AllegroModerato => (116, 120),
      TempoMarking::Allegro => (120, 168),
      TempoMarking::Vivace => (168, 176),
      TempoMarking::Vivacissimo => (172, 176),
      TempoMarking::Allegrissimo => (172, 176),
      TempoMarking::AllegroVivace => (172, 176),
      TempoMarking::Presto => (168, 200),
      TempoMarking::Prestissimo => (200, 240),
    }
  }

  pub fn value(&self) -> u16 {
    match *self {
      TempoMarking::Larghissimo => 22,
      TempoMarking::Grave => 35,
      TempoMarking::Largo => 50,
      TempoMarking::Lento => 55,
      TempoMarking::Larghetto => 63,
      TempoMarking::Adagio => 70,
      TempoMarking::Adagietto => 74,
      TempoMarking::Andante => 86,
      TempoMarking::Andantino => 94,
      TempoMarking::MarciaModerato => 84,
      TempoMarking::AndanteModerato => 102,
      TempoMarking::Moderato => 114,
      TempoMarking::Allegretto => 116,
      TempoMarking::AllegroModerato => 118,
      TempoMarking::Allegro => 140,
      TempoMarking::Vivace => 172,
      TempoMarking::Vivacissimo => 174,
      TempoMarking::Allegrissimo => 174,
      TempoMarking::AllegroVivace => 174,
      TempoMarking::Presto => 190,
      TempoMarking::Prestissimo => 220,
    }
  }
}

impl std::fmt::Display for TempoMarking {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match *self {
        TempoMarking::Larghissimo => "Larghissimo",
        TempoMarking::Grave => "Grave",
        TempoMarking::Largo => "Largo",
        TempoMarking::Lento => "Lento",
        TempoMarking::Larghetto => "Larghetto",
        TempoMarking::Adagio => "Adagio",
        TempoMarking::Adagietto => "Adagietto",
        TempoMarking::Andante => "Andante",
        TempoMarking::Andantino => "Andantino",
        TempoMarking::MarciaModerato => "Marcia Moderato",
        TempoMarking::AndanteModerato => "Andante Moderato",
        TempoMarking::Moderato => "Moderato",
        TempoMarking::Allegretto => "Allegretto",
        TempoMarking::AllegroModerato => "Allegro Moderato",
        TempoMarking::Allegro => "Allegro",
        TempoMarking::Vivace => "Vivace",
        TempoMarking::Vivacissimo => "Vivacissimo",
        TempoMarking::Allegrissimo => "Allegrissimo",
        TempoMarking::AllegroVivace => "Allegro Vivace",
        TempoMarking::Presto => "Presto",
        TempoMarking::Prestissimo => "Prestissimo",
      }
    )
  }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TempoModification {
  Accelerando,
  Rallentando,
  Ritardando,
  Ritenuto,
  Stringendo,
}

impl std::fmt::Display for TempoModification {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match *self {
        TempoModification::Accelerando => "Accelerando",
        TempoModification::Rallentando => "Rallentando",
        TempoModification::Ritardando => "Ritardando",
        TempoModification::Ritenuto => "Ritenuto",
        TempoModification::Stringendo => "Stringendo",
      }
    )
  }
}
