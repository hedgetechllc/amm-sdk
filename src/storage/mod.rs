use crate::Composition;
use amm::AmmStorage;
use midi::MidiConverter;
use musicxml::MusicXmlConverter;
use std::fmt;

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
    match *self {
      Storage::AMM => AmmStorage::load(path),
      Storage::MusicXML => MusicXmlConverter::load(path),
      Storage::MIDI => MidiConverter::load(path),
    }
  }

  pub fn save(&self, path: &str, composition: &Composition) -> Result<usize, String> {
    match *self {
      Storage::AMM => AmmStorage::save(path, composition),
      Storage::MusicXML => MusicXmlConverter::save(path, composition),
      Storage::MIDI => MidiConverter::save(path, composition),
    }
  }
}

impl fmt::Display for Storage {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match *self {
        Storage::AMM => "Abstract Music Manipulation (AMM)",
        Storage::MusicXML => "MusicXML",
        Storage::MIDI => "MIDI",
      }
    )
  }
}
