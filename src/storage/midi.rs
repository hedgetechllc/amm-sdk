use super::Convert;
use crate::{
  Accidental, Chord, ChordModificationType, Clef, ClefType, Composition, DirectionType, Duration, DynamicMarking, Key,
  KeyMode, NoteModificationType, PedalType, Phrase, PhraseModificationType, Pitch, Section, SectionModificationType,
  Tempo, TimeSignature,
};
use alloc::{string::String, vec::Vec};
use midly::Smf;
use std::fs;

pub struct MidiConverter;

impl MidiConverter {
  fn load_from_midi(data: &[u8]) -> Result<Composition, String> {
    // Parse the MIDI representation
    let midi = Smf::parse(data).map_err(|err| err.to_string())?;

    // Generate the initial composition structure and fill in known metadata
    let mut composition = Composition::new("Error", None, None, None);
    // TODO: Fill in metadata from MIDI, set correct title, etc.

    // TODO: Implement construction of Composition from MIDI

    // Return the fully constructed composition
    Ok(composition)
  }
}

impl Convert for MidiConverter {
  fn load(path: &str) -> Result<Composition, String> {
    let data = fs::read(path).map_err(|err| err.to_string())?;
    MidiConverter::load_from_midi(&data)
  }

  fn load_data(data: &[u8]) -> Result<Composition, String> {
    MidiConverter::load_from_midi(&data)
  }

  fn save(_path: &str, _composition: &Composition) -> Result<usize, String> {
    todo!(); // TODO: Implement
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_midi_whatever() {}
}
