use crate::Composition;

use alloc::string::String;
use amm::AmmStorage;
use json::JsonConverter;
use midi::MidiConverter;
use musicxml::MusicXmlConverter;

mod amm;
mod json;
mod midi;
mod musicxml;

pub(crate) trait Convert {
  fn load(path: &str) -> Result<Composition, String>;
  fn load_data(data: &[u8]) -> Result<Composition, String>;
  fn save(path: &str, composition: &Composition) -> Result<usize, String>;
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum Storage {
  #[default]
  AMM,
  MusicXML,
  MIDI,
  JSON,
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
      Self::JSON => JsonConverter::load(path),
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
      Self::JSON => JsonConverter::load_data(data),
    }
  }

  /// TODO
  /// 
  /// # Errors
  /// TODO
  pub fn save(&self, path: &str, composition: &Composition) -> Result<usize, String> {
    match self {
      Self::AMM => AmmStorage::save(path, composition),
      Self::MusicXML => MusicXmlConverter::save(path, composition),
      Self::MIDI => MidiConverter::save(path, composition),
      Self::JSON => JsonConverter::save(path, composition),
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
        Self::JSON => "JavaScript Object Notation (JSON)",
      }
    )
  }
}
