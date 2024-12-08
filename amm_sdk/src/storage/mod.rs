//! This module provides the necessary tools to load and store
//! compositions in different formats.

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
  fn load_data(data: Vec<u8>) -> Result<Composition, String>;
}

pub(crate) trait Store {
  fn save(path: &str, composition: &Composition) -> Result<usize, String>;
}

/// Represents the various storage formats supported by the SDK.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum Storage {
  #[default]
  AMM,
  MusicXML,
  MIDI,
}

impl Storage {
  /// Loads a composition from a file at the specified `path`.
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
  pub fn load_data(&self, data: Vec<u8>) -> Result<Composition, String> {
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
      Self::MusicXML => Err(String::from("Cannot export to MusicXML")),
      Self::MIDI => Err(String::from("Cannot export to MIDI")),
    }
  }
}

impl core::fmt::Display for Storage {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::AMM => "AMM (Abstract Music Manipulation)",
        Self::MusicXML => "MusicXML (Music Extensible Markup Language)",
        Self::MIDI => "MIDI (Musical Instrument Digital Interface)",
      }
    )
  }
}
