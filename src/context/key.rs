use crate::note::Accidental;

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum KeyMode {
  #[default]
  Major,
  Minor,
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum Key {
  AMajor,
  AMinor,
  ASharpMinor,
  AFlatMajor,
  AFlatMinor,
  BMajor,
  BMinor,
  BFlatMajor,
  BFlatMinor,
  #[default]
  CMajor,
  CMinor,
  CSharpMajor,
  CSharpMinor,
  CFlatMajor,
  DMajor,
  DMinor,
  DSharpMinor,
  DFlatMajor,
  EMajor,
  EMinor,
  EFlatMajor,
  EFlatMinor,
  FMajor,
  FMinor,
  FSharpMajor,
  FSharpMinor,
  GMajor,
  GMinor,
  GSharpMinor,
  GFlatMajor,
}

impl Key {
  pub fn from_fifths(fifths: i8, mode: Option<KeyMode>) -> Self {
    let mode = mode.unwrap_or(KeyMode::Major);
    match fifths {
      3 => {
        if mode == KeyMode::Major {
          Key::AMajor
        } else {
          Key::FSharpMinor
        }
      }
      -4 => {
        if mode == KeyMode::Major {
          Key::AFlatMajor
        } else {
          Key::FMinor
        }
      }
      5 => {
        if mode == KeyMode::Major {
          Key::BMajor
        } else {
          Key::GSharpMinor
        }
      }
      -2 => {
        if mode == KeyMode::Major {
          Key::BFlatMajor
        } else {
          Key::GMinor
        }
      }
      7 => {
        if mode == KeyMode::Major {
          Key::CSharpMajor
        } else {
          Key::ASharpMinor
        }
      }
      -7 => {
        if mode == KeyMode::Major {
          Key::CFlatMajor
        } else {
          Key::AFlatMinor
        }
      }
      2 => {
        if mode == KeyMode::Major {
          Key::DMajor
        } else {
          Key::BMinor
        }
      }
      -5 => {
        if mode == KeyMode::Major {
          Key::DFlatMajor
        } else {
          Key::BFlatMinor
        }
      }
      4 => {
        if mode == KeyMode::Major {
          Key::EMajor
        } else {
          Key::CSharpMinor
        }
      }
      -3 => {
        if mode == KeyMode::Major {
          Key::EFlatMajor
        } else {
          Key::CMinor
        }
      }
      -1 => {
        if mode == KeyMode::Major {
          Key::FMajor
        } else {
          Key::DMinor
        }
      }
      6 => {
        if mode == KeyMode::Major {
          Key::FSharpMajor
        } else {
          Key::DSharpMinor
        }
      }
      1 => {
        if mode == KeyMode::Major {
          Key::GMajor
        } else {
          Key::EMinor
        }
      }
      -6 => {
        if mode == KeyMode::Major {
          Key::GFlatMajor
        } else {
          Key::EFlatMinor
        }
      }
      _ => {
        if mode == KeyMode::Major {
          Key::CMajor
        } else {
          Key::AMinor
        }
      }
    }
  }

  pub fn fifths(&self) -> i8 {
    match *self {
      Key::AMajor => 3,
      Key::AMinor => 0,
      Key::ASharpMinor => 7,
      Key::AFlatMajor => -4,
      Key::AFlatMinor => -7,
      Key::BMajor => 5,
      Key::BMinor => 2,
      Key::BFlatMajor => -2,
      Key::BFlatMinor => -5,
      Key::CMajor => 0,
      Key::CMinor => -3,
      Key::CSharpMajor => 7,
      Key::CSharpMinor => 4,
      Key::CFlatMajor => -7,
      Key::DMajor => 2,
      Key::DMinor => -1,
      Key::DSharpMinor => 6,
      Key::DFlatMajor => -5,
      Key::EMajor => 4,
      Key::EMinor => 1,
      Key::EFlatMajor => -3,
      Key::EFlatMinor => -6,
      Key::FMajor => -1,
      Key::FMinor => -4,
      Key::FSharpMajor => 6,
      Key::FSharpMinor => 3,
      Key::GMajor => 1,
      Key::GMinor => -2,
      Key::GSharpMinor => 5,
      Key::GFlatMajor => -6,
    }
  }

  pub fn accidentals(&self) -> [Accidental; 8] {
    match *self {
      Key::AMajor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      Key::AMinor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
      ],
      Key::ASharpMinor => [
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      Key::AFlatMajor => [
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
      ],
      Key::AFlatMinor => [
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
      ],
      Key::BMajor => [
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      Key::BMinor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
      ],
      Key::BFlatMajor => [
        Accidental::None,
        Accidental::None,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
      ],
      Key::BFlatMinor => [
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::Flat,
      ],
      Key::CMajor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
      ],
      Key::CMinor => [
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
      ],
      Key::CSharpMajor => [
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      Key::CSharpMinor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      Key::CFlatMajor => [
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
      ],
      Key::DMajor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
      ],
      Key::DMinor => [
        Accidental::None,
        Accidental::None,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
      ],
      Key::DSharpMinor => [
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      Key::DFlatMajor => [
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::Flat,
      ],
      Key::EMajor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      Key::EMinor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
      ],
      Key::EFlatMajor => [
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
      ],
      Key::EFlatMinor => [
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::Flat,
      ],
      Key::FMajor => [
        Accidental::None,
        Accidental::None,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
      ],
      Key::FMinor => [
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
      ],
      Key::FSharpMajor => [
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      Key::FSharpMinor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      Key::GMajor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
      ],
      Key::GMinor => [
        Accidental::None,
        Accidental::None,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
      ],
      Key::GSharpMinor => [
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      Key::GFlatMajor => [
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::Flat,
      ],
    }
  }
}

impl std::fmt::Display for Key {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match *self {
        Key::AMajor => "A",
        Key::AMinor => "Am",
        Key::ASharpMinor => "A♯m",
        Key::AFlatMajor => "A♭",
        Key::AFlatMinor => "A♭m",
        Key::BMajor => "B",
        Key::BMinor => "Bm",
        Key::BFlatMajor => "B♭",
        Key::BFlatMinor => "B♭m",
        Key::CMajor => "C",
        Key::CMinor => "Cm",
        Key::CSharpMajor => "C♯",
        Key::CSharpMinor => "C♯m",
        Key::CFlatMajor => "C♭",
        Key::DMajor => "D",
        Key::DMinor => "Dm",
        Key::DSharpMinor => "D♯m",
        Key::DFlatMajor => "D♭",
        Key::EMajor => "E",
        Key::EMinor => "Em",
        Key::EFlatMajor => "E♭",
        Key::EFlatMinor => "E♭m",
        Key::FMajor => "F",
        Key::FMinor => "Fm",
        Key::FSharpMajor => "F♯",
        Key::FSharpMinor => "F♯m",
        Key::GMajor => "G",
        Key::GMinor => "Gm",
        Key::GSharpMinor => "G♯m",
        Key::GFlatMajor => "G♭",
      }
    )
  }
}
