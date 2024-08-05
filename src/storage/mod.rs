use crate::Composition;

use alloc::string::String;
use amm::AmmStorage;
use midi::MidiConverter;
use musicxml::MusicXmlConverter;

mod amm;
mod midi;
mod musicxml;

pub trait Convert {
  fn load(path: &str) -> Result<Composition, String>;
  fn save(path: &str, composition: &Composition) -> Result<usize, String>;
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum Storage {
  #[default]
  AMM,
  MusicXML,
  MIDI,
}

impl Storage {
  pub fn load(&self, path: &str) -> Result<Composition, String> {
    match self {
      Self::AMM => AmmStorage::load(path),
      Self::MusicXML => MusicXmlConverter::load(path),
      Self::MIDI => MidiConverter::load(path),
    }
  }

  pub fn save(&self, path: &str, composition: &Composition) -> Result<usize, String> {
    match self {
      Self::AMM => AmmStorage::save(path, composition),
      Self::MusicXML => MusicXmlConverter::save(path, composition),
      Self::MIDI => MidiConverter::save(path, composition),
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
