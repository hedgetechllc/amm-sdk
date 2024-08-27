use crate::note::Accidental;
#[cfg(target_arch = "wasm32")]
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum KeyMode {
  #[default]
  Major,
  Minor,
}

#[cfg_attr(target_arch = "wasm32", derive(Deserialize, Serialize))]
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
  const FIFTHS_A_MAJOR: i8 = 3;
  const FIFTHS_A_FLAT_MAJOR: i8 = -4;
  const FIFTHS_B_MAJOR: i8 = 5;
  const FIFTHS_B_FLAT_MAJOR: i8 = -2;
  const FIFTHS_C_MAJOR: i8 = 0;
  const FIFTHS_C_SHARP_MAJOR: i8 = 7;
  const FIFTHS_C_FLAT_MAJOR: i8 = -7;
  const FIFTHS_D_MAJOR: i8 = 2;
  const FIFTHS_D_FLAT_MAJOR: i8 = -5;
  const FIFTHS_E_MAJOR: i8 = 4;
  const FIFTHS_E_FLAT_MAJOR: i8 = -3;
  const FIFTHS_F_MAJOR: i8 = -1;
  const FIFTHS_F_SHARP_MAJOR: i8 = 6;
  const FIFTHS_G_MAJOR: i8 = 1;
  const FIFTHS_G_FLAT_MAJOR: i8 = -6;

  const FIFTHS_F_SHARP_MINOR: i8 = 3;
  const FIFTHS_F_MINOR: i8 = -4;
  const FIFTHS_G_SHARP_MINOR: i8 = 5;
  const FIFTHS_G_MINOR: i8 = -2;
  const FIFTHS_A_MINOR: i8 = 0;
  const FIFTHS_A_SHARP_MINOR: i8 = 7;
  const FIFTHS_A_FLAT_MINOR: i8 = -7;
  const FIFTHS_B_MINOR: i8 = 2;
  const FIFTHS_B_FLAT_MINOR: i8 = -5;
  const FIFTHS_C_SHARP_MINOR: i8 = 4;
  const FIFTHS_C_MINOR: i8 = -3;
  const FIFTHS_D_MINOR: i8 = -1;
  const FIFTHS_D_SHARP_MINOR: i8 = 6;
  const FIFTHS_E_MINOR: i8 = 1;
  const FIFTHS_E_FLAT_MINOR: i8 = -6;

  #[must_use]
  pub fn from_fifths(fifths: i8, mode: Option<KeyMode>) -> Self {
    match (fifths, mode.unwrap_or(KeyMode::Major)) {
      (Self::FIFTHS_A_MAJOR, KeyMode::Major) => Self::AMajor,
      (Self::FIFTHS_A_FLAT_MAJOR, KeyMode::Major) => Self::AFlatMajor,
      (Self::FIFTHS_B_MAJOR, KeyMode::Major) => Self::BMajor,
      (Self::FIFTHS_B_FLAT_MAJOR, KeyMode::Major) => Self::BFlatMajor,
      (Self::FIFTHS_C_SHARP_MAJOR, KeyMode::Major) => Self::CSharpMajor,
      (Self::FIFTHS_C_FLAT_MAJOR, KeyMode::Major) => Self::CFlatMajor,
      (Self::FIFTHS_D_MAJOR, KeyMode::Major) => Self::DMajor,
      (Self::FIFTHS_D_FLAT_MAJOR, KeyMode::Major) => Self::DFlatMajor,
      (Self::FIFTHS_E_MAJOR, KeyMode::Major) => Self::EMajor,
      (Self::FIFTHS_E_FLAT_MAJOR, KeyMode::Major) => Self::EFlatMajor,
      (Self::FIFTHS_F_MAJOR, KeyMode::Major) => Self::FMajor,
      (Self::FIFTHS_F_SHARP_MAJOR, KeyMode::Major) => Self::FSharpMajor,
      (Self::FIFTHS_G_MAJOR, KeyMode::Major) => Self::GMajor,
      (Self::FIFTHS_G_FLAT_MAJOR, KeyMode::Major) => Self::GFlatMajor,
      (Self::FIFTHS_F_SHARP_MINOR, KeyMode::Minor) => Self::FSharpMinor,
      (Self::FIFTHS_F_MINOR, KeyMode::Minor) => Self::FMinor,
      (Self::FIFTHS_G_SHARP_MINOR, KeyMode::Minor) => Self::GSharpMinor,
      (Self::FIFTHS_G_MINOR, KeyMode::Minor) => Self::GMinor,
      (Self::FIFTHS_A_MINOR, KeyMode::Minor) => Self::AMinor,
      (Self::FIFTHS_A_SHARP_MINOR, KeyMode::Minor) => Self::ASharpMinor,
      (Self::FIFTHS_A_FLAT_MINOR, KeyMode::Minor) => Self::AFlatMinor,
      (Self::FIFTHS_B_MINOR, KeyMode::Minor) => Self::BMinor,
      (Self::FIFTHS_B_FLAT_MINOR, KeyMode::Minor) => Self::BFlatMinor,
      (Self::FIFTHS_C_SHARP_MINOR, KeyMode::Minor) => Self::CSharpMinor,
      (Self::FIFTHS_C_MINOR, KeyMode::Minor) => Self::CMinor,
      (Self::FIFTHS_D_MINOR, KeyMode::Minor) => Self::DMinor,
      (Self::FIFTHS_D_SHARP_MINOR, KeyMode::Minor) => Self::DSharpMinor,
      (Self::FIFTHS_E_MINOR, KeyMode::Minor) => Self::EMinor,
      (Self::FIFTHS_E_FLAT_MINOR, KeyMode::Minor) => Self::EFlatMinor,
      _ => Self::CMajor,
    }
  }

  #[must_use]
  pub fn fifths(&self) -> i8 {
    match self {
      Self::AMajor => Self::FIFTHS_A_MAJOR,
      Self::AMinor => Self::FIFTHS_A_MINOR,
      Self::ASharpMinor => Self::FIFTHS_A_SHARP_MINOR,
      Self::AFlatMajor => Self::FIFTHS_A_FLAT_MAJOR,
      Self::AFlatMinor => Self::FIFTHS_A_FLAT_MINOR,
      Self::BMajor => Self::FIFTHS_B_MAJOR,
      Self::BMinor => Self::FIFTHS_B_MINOR,
      Self::BFlatMajor => Self::FIFTHS_B_FLAT_MAJOR,
      Self::BFlatMinor => Self::FIFTHS_B_FLAT_MINOR,
      Self::CMajor => Self::FIFTHS_C_MAJOR,
      Self::CMinor => Self::FIFTHS_C_MINOR,
      Self::CSharpMajor => Self::FIFTHS_C_SHARP_MAJOR,
      Self::CSharpMinor => Self::FIFTHS_C_SHARP_MINOR,
      Self::CFlatMajor => Self::FIFTHS_C_FLAT_MAJOR,
      Self::DMajor => Self::FIFTHS_D_MAJOR,
      Self::DMinor => Self::FIFTHS_D_MINOR,
      Self::DSharpMinor => Self::FIFTHS_D_SHARP_MINOR,
      Self::DFlatMajor => Self::FIFTHS_D_FLAT_MAJOR,
      Self::EMajor => Self::FIFTHS_E_MAJOR,
      Self::EMinor => Self::FIFTHS_E_MINOR,
      Self::EFlatMajor => Self::FIFTHS_E_FLAT_MAJOR,
      Self::EFlatMinor => Self::FIFTHS_E_FLAT_MINOR,
      Self::FMajor => Self::FIFTHS_F_MAJOR,
      Self::FMinor => Self::FIFTHS_F_MINOR,
      Self::FSharpMajor => Self::FIFTHS_F_SHARP_MAJOR,
      Self::FSharpMinor => Self::FIFTHS_F_SHARP_MINOR,
      Self::GMajor => Self::FIFTHS_G_MAJOR,
      Self::GMinor => Self::FIFTHS_G_MINOR,
      Self::GSharpMinor => Self::FIFTHS_G_SHARP_MINOR,
      Self::GFlatMajor => Self::FIFTHS_G_FLAT_MAJOR,
    }
  }

  #[must_use]
  #[allow(clippy::too_many_lines)]
  pub fn accidentals(&self) -> [Accidental; 8] {
    match self {
      Self::AMajor | Self::FSharpMinor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      Self::AMinor | Self::CMajor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
      ],
      Self::ASharpMinor | Self::CSharpMajor => [
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      Self::AFlatMajor | Self::FMinor => [
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
      ],
      Self::AFlatMinor | Self::CFlatMajor => [
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
      ],
      Self::BMajor | Self::GSharpMinor => [
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      Self::BMinor | Self::DMajor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
      ],
      Self::BFlatMajor | Self::GMinor => [
        Accidental::None,
        Accidental::None,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
      ],
      Self::BFlatMinor | Self::DFlatMajor => [
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::Flat,
      ],
      Self::CMinor | Self::EFlatMajor => [
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
      ],
      Self::CSharpMinor | Self::EMajor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      Self::DMinor | Self::FMajor => [
        Accidental::None,
        Accidental::None,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
      ],
      Self::DSharpMinor | Self::FSharpMajor => [
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      Self::EMinor | Self::GMajor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
      ],
      Self::EFlatMinor | Self::GFlatMajor => [
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

#[cfg(feature = "print")]
impl core::fmt::Display for Key {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
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
