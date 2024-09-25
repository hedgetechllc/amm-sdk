use crate::Composition;

use alloc::string::String;
use amm::AmmStorage;
use midi::MidiConverter;
use musicxml::MusicXmlConverter;

mod amm;
mod midi;
mod musicxml;

pub(crate) trait Load {
  fn load(path: &str) -> Result<Composition, String>;
  fn load_data(data: &[u8]) -> Result<Composition, String>;
}

pub(crate) trait Store {
  fn save(path: &str, composition: &Composition) -> Result<usize, String>;
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum Storage {
  #[default]
  AMM,
  MusicXML,
  MIDI,
}

impl Storage {
  /// TODO
  ///
  /// # Errors
  /// TODO
  pub fn load(&self, path: &str) -> Result<Composition, String> {
    match self {
      Self::AMM => AmmStorage::load(path),
      Self::MusicXML => MusicXmlConverter::load(path),
      Self::MIDI => MidiConverter::load(path),
    }
  }

  /// TODO
  ///
  /// # Errors
  /// TODO
  pub fn load_data(&self, data: &[u8]) -> Result<Composition, String> {
    match self {
      Self::AMM => AmmStorage::load_data(data),
      Self::MusicXML => MusicXmlConverter::load_data(data),
      Self::MIDI => MidiConverter::load_data(data),
    }
  }

  /// TODO
  ///
  /// # Errors
  /// TODO
  pub fn save(&self, path: &str, composition: &Composition) -> Result<usize, String> {
    match self {
      Self::AMM => AmmStorage::save(path, composition),
      Self::MusicXML => Err(String::from("Cannot write to MusicXML")),
      Self::MIDI => Err(String::from("Cannot write to MIDI")),
    }
  }
}

impl core::fmt::Display for Storage {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::AMM => "Abstract Music Manipulation (AMM)",
        Self::MusicXML => "MusicXML",
        Self::MIDI => "Musical Instrument Digital Interface (MIDI)",
      }
    )
  }
}
