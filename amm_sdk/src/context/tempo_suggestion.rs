use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
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
  #[default]
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub struct TempoSuggestion {
  pub marking: TempoMarking,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl TempoSuggestion {
  #[must_use]
  pub fn new(marking: TempoMarking) -> Self {
    Self { marking }
  }

  #[must_use]
  pub fn description(&self) -> String {
    String::from(match self.marking {
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
      TempoMarking::AllegroVivace => "very lively and fast",
      TempoMarking::Presto => "very, very fast",
      TempoMarking::Prestissimo => "extremely fast",
    })
  }

  #[must_use]
  pub fn bpm_min(&self) -> u16 {
    match self.marking {
      TempoMarking::Larghissimo => 10,
      TempoMarking::Grave => 25,
      TempoMarking::Largo => 40,
      TempoMarking::Lento => 45,
      TempoMarking::Larghetto => 60,
      TempoMarking::Adagio => 66,
      TempoMarking::Adagietto => 72,
      TempoMarking::Andante => 76,
      TempoMarking::Andantino => 80,
      TempoMarking::MarciaModerato => 83,
      TempoMarking::AndanteModerato => 92,
      TempoMarking::Moderato => 108,
      TempoMarking::Allegretto => 112,
      TempoMarking::AllegroModerato => 116,
      TempoMarking::Allegro => 120,
      TempoMarking::Vivace | TempoMarking::Presto => 168,
      TempoMarking::Vivacissimo | TempoMarking::Allegrissimo => 172,
      TempoMarking::AllegroVivace => 174,
      TempoMarking::Prestissimo => 200,
    }
  }

  #[must_use]
  pub fn bpm_max(&self) -> u16 {
    match self.marking {
      TempoMarking::Larghissimo => 24,
      TempoMarking::Grave => 45,
      TempoMarking::Largo | TempoMarking::Lento => 60,
      TempoMarking::Larghetto => 66,
      TempoMarking::Adagio | TempoMarking::Adagietto => 76,
      TempoMarking::Andante | TempoMarking::Andantino => 108,
      TempoMarking::MarciaModerato => 85,
      TempoMarking::AndanteModerato => 112,
      TempoMarking::Moderato | TempoMarking::Allegretto | TempoMarking::AllegroModerato => 120,
      TempoMarking::Allegro => 168,
      TempoMarking::Vivace | TempoMarking::Vivacissimo => 176,
      TempoMarking::Allegrissimo | TempoMarking::AllegroVivace => 178,
      TempoMarking::Presto => 200,
      TempoMarking::Prestissimo => 240,
    }
  }

  #[must_use]
  pub fn value(&self) -> u16 {
    match self.marking {
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
      TempoMarking::Allegrissimo => 175,
      TempoMarking::AllegroVivace => 176,
      TempoMarking::Presto => 190,
      TempoMarking::Prestissimo => 220,
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for TempoMarking {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
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

#[cfg(feature = "print")]
impl core::fmt::Display for TempoSuggestion {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.marking)
  }
}
