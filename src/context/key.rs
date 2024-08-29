use crate::note::Accidental;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum KeyMode {
  #[default]
  Major,
  Minor,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum KeySignature {
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Key {
  pub mode: KeyMode,
  pub r#type: KeySignature,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Key {
  #[must_use]
  pub fn new(r#type: KeySignature) -> Self {
    Self {
      r#type,
      mode: match r#type {
        KeySignature::AMajor
        | KeySignature::AFlatMajor
        | KeySignature::BMajor
        | KeySignature::BFlatMajor
        | KeySignature::CMajor
        | KeySignature::CSharpMajor
        | KeySignature::CFlatMajor
        | KeySignature::DMajor
        | KeySignature::DFlatMajor
        | KeySignature::EMajor
        | KeySignature::EFlatMajor
        | KeySignature::FMajor
        | KeySignature::FSharpMajor
        | KeySignature::GMajor
        | KeySignature::GFlatMajor => KeyMode::Major,
        KeySignature::AMinor
        | KeySignature::ASharpMinor
        | KeySignature::AFlatMinor
        | KeySignature::BMinor
        | KeySignature::BFlatMinor
        | KeySignature::CMinor
        | KeySignature::CSharpMinor
        | KeySignature::DMinor
        | KeySignature::DSharpMinor
        | KeySignature::EMinor
        | KeySignature::EFlatMinor
        | KeySignature::FMinor
        | KeySignature::FSharpMinor
        | KeySignature::GMinor
        | KeySignature::GSharpMinor => KeyMode::Minor,
      },
    }
  }

  #[must_use]
  pub fn from_fifths(fifths: i8, mode: Option<KeyMode>) -> Self {
    Key::new(match (fifths, mode.unwrap_or(KeyMode::Major)) {
      (FIFTHS_A_MAJOR, KeyMode::Major) => KeySignature::AMajor,
      (FIFTHS_A_FLAT_MAJOR, KeyMode::Major) => KeySignature::AFlatMajor,
      (FIFTHS_B_MAJOR, KeyMode::Major) => KeySignature::BMajor,
      (FIFTHS_B_FLAT_MAJOR, KeyMode::Major) => KeySignature::BFlatMajor,
      (FIFTHS_C_SHARP_MAJOR, KeyMode::Major) => KeySignature::CSharpMajor,
      (FIFTHS_C_FLAT_MAJOR, KeyMode::Major) => KeySignature::CFlatMajor,
      (FIFTHS_D_MAJOR, KeyMode::Major) => KeySignature::DMajor,
      (FIFTHS_D_FLAT_MAJOR, KeyMode::Major) => KeySignature::DFlatMajor,
      (FIFTHS_E_MAJOR, KeyMode::Major) => KeySignature::EMajor,
      (FIFTHS_E_FLAT_MAJOR, KeyMode::Major) => KeySignature::EFlatMajor,
      (FIFTHS_F_MAJOR, KeyMode::Major) => KeySignature::FMajor,
      (FIFTHS_F_SHARP_MAJOR, KeyMode::Major) => KeySignature::FSharpMajor,
      (FIFTHS_G_MAJOR, KeyMode::Major) => KeySignature::GMajor,
      (FIFTHS_G_FLAT_MAJOR, KeyMode::Major) => KeySignature::GFlatMajor,
      (FIFTHS_F_SHARP_MINOR, KeyMode::Minor) => KeySignature::FSharpMinor,
      (FIFTHS_F_MINOR, KeyMode::Minor) => KeySignature::FMinor,
      (FIFTHS_G_SHARP_MINOR, KeyMode::Minor) => KeySignature::GSharpMinor,
      (FIFTHS_G_MINOR, KeyMode::Minor) => KeySignature::GMinor,
      (FIFTHS_A_MINOR, KeyMode::Minor) => KeySignature::AMinor,
      (FIFTHS_A_SHARP_MINOR, KeyMode::Minor) => KeySignature::ASharpMinor,
      (FIFTHS_A_FLAT_MINOR, KeyMode::Minor) => KeySignature::AFlatMinor,
      (FIFTHS_B_MINOR, KeyMode::Minor) => KeySignature::BMinor,
      (FIFTHS_B_FLAT_MINOR, KeyMode::Minor) => KeySignature::BFlatMinor,
      (FIFTHS_C_SHARP_MINOR, KeyMode::Minor) => KeySignature::CSharpMinor,
      (FIFTHS_C_MINOR, KeyMode::Minor) => KeySignature::CMinor,
      (FIFTHS_D_MINOR, KeyMode::Minor) => KeySignature::DMinor,
      (FIFTHS_D_SHARP_MINOR, KeyMode::Minor) => KeySignature::DSharpMinor,
      (FIFTHS_E_MINOR, KeyMode::Minor) => KeySignature::EMinor,
      (FIFTHS_E_FLAT_MINOR, KeyMode::Minor) => KeySignature::EFlatMinor,
      _ => KeySignature::CMajor,
    })
  }

  #[must_use]
  pub fn fifths(&self) -> i8 {
    match self.r#type {
      KeySignature::AMajor => FIFTHS_A_MAJOR,
      KeySignature::AMinor => FIFTHS_A_MINOR,
      KeySignature::ASharpMinor => FIFTHS_A_SHARP_MINOR,
      KeySignature::AFlatMajor => FIFTHS_A_FLAT_MAJOR,
      KeySignature::AFlatMinor => FIFTHS_A_FLAT_MINOR,
      KeySignature::BMajor => FIFTHS_B_MAJOR,
      KeySignature::BMinor => FIFTHS_B_MINOR,
      KeySignature::BFlatMajor => FIFTHS_B_FLAT_MAJOR,
      KeySignature::BFlatMinor => FIFTHS_B_FLAT_MINOR,
      KeySignature::CMajor => FIFTHS_C_MAJOR,
      KeySignature::CMinor => FIFTHS_C_MINOR,
      KeySignature::CSharpMajor => FIFTHS_C_SHARP_MAJOR,
      KeySignature::CSharpMinor => FIFTHS_C_SHARP_MINOR,
      KeySignature::CFlatMajor => FIFTHS_C_FLAT_MAJOR,
      KeySignature::DMajor => FIFTHS_D_MAJOR,
      KeySignature::DMinor => FIFTHS_D_MINOR,
      KeySignature::DSharpMinor => FIFTHS_D_SHARP_MINOR,
      KeySignature::DFlatMajor => FIFTHS_D_FLAT_MAJOR,
      KeySignature::EMajor => FIFTHS_E_MAJOR,
      KeySignature::EMinor => FIFTHS_E_MINOR,
      KeySignature::EFlatMajor => FIFTHS_E_FLAT_MAJOR,
      KeySignature::EFlatMinor => FIFTHS_E_FLAT_MINOR,
      KeySignature::FMajor => FIFTHS_F_MAJOR,
      KeySignature::FMinor => FIFTHS_F_MINOR,
      KeySignature::FSharpMajor => FIFTHS_F_SHARP_MAJOR,
      KeySignature::FSharpMinor => FIFTHS_F_SHARP_MINOR,
      KeySignature::GMajor => FIFTHS_G_MAJOR,
      KeySignature::GMinor => FIFTHS_G_MINOR,
      KeySignature::GSharpMinor => FIFTHS_G_SHARP_MINOR,
      KeySignature::GFlatMajor => FIFTHS_G_FLAT_MAJOR,
    }
  }
}

impl Key {
  #[must_use]
  #[allow(clippy::too_many_lines)]
  pub(crate) fn accidentals(&self) -> [Accidental; 8] {
    match self.r#type {
      KeySignature::AMajor | KeySignature::FSharpMinor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      KeySignature::AMinor | KeySignature::CMajor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
      ],
      KeySignature::ASharpMinor | KeySignature::CSharpMajor => [
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      KeySignature::AFlatMajor | KeySignature::FMinor => [
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
      ],
      KeySignature::AFlatMinor | KeySignature::CFlatMajor => [
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::Flat,
      ],
      KeySignature::BMajor | KeySignature::GSharpMinor => [
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      KeySignature::BMinor | KeySignature::DMajor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
      ],
      KeySignature::BFlatMajor | KeySignature::GMinor => [
        Accidental::None,
        Accidental::None,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
      ],
      KeySignature::BFlatMinor | KeySignature::DFlatMajor => [
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::Flat,
      ],
      KeySignature::CMinor | KeySignature::EFlatMajor => [
        Accidental::None,
        Accidental::Flat,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
      ],
      KeySignature::CSharpMinor | KeySignature::EMajor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      KeySignature::DMinor | KeySignature::FMajor => [
        Accidental::None,
        Accidental::None,
        Accidental::Flat,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
      ],
      KeySignature::DSharpMinor | KeySignature::FSharpMajor => [
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
        Accidental::Sharp,
      ],
      KeySignature::EMinor | KeySignature::GMajor => [
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::None,
        Accidental::Sharp,
        Accidental::None,
      ],
      KeySignature::EFlatMinor | KeySignature::GFlatMajor => [
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
      match self.r#type {
        KeySignature::AMajor => "A",
        KeySignature::AMinor => "Am",
        KeySignature::ASharpMinor => "A♯m",
        KeySignature::AFlatMajor => "A♭",
        KeySignature::AFlatMinor => "A♭m",
        KeySignature::BMajor => "B",
        KeySignature::BMinor => "Bm",
        KeySignature::BFlatMajor => "B♭",
        KeySignature::BFlatMinor => "B♭m",
        KeySignature::CMajor => "C",
        KeySignature::CMinor => "Cm",
        KeySignature::CSharpMajor => "C♯",
        KeySignature::CSharpMinor => "C♯m",
        KeySignature::CFlatMajor => "C♭",
        KeySignature::DMajor => "D",
        KeySignature::DMinor => "Dm",
        KeySignature::DSharpMinor => "D♯m",
        KeySignature::DFlatMajor => "D♭",
        KeySignature::EMajor => "E",
        KeySignature::EMinor => "Em",
        KeySignature::EFlatMajor => "E♭",
        KeySignature::EFlatMinor => "E♭m",
        KeySignature::FMajor => "F",
        KeySignature::FMinor => "Fm",
        KeySignature::FSharpMajor => "F♯",
        KeySignature::FSharpMinor => "F♯m",
        KeySignature::GMajor => "G",
        KeySignature::GMinor => "Gm",
        KeySignature::GSharpMinor => "G♯m",
        KeySignature::GFlatMajor => "G♭",
      }
    )
  }
}
