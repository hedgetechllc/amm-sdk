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

impl Convert for MidiConverter {
  fn load(path: &str) -> Result<Composition, String> {
    // Parse the MIDI representation
    let file_contents = fs::read(path).map_err(|err| err.to_string())?;
    let midi = Smf::parse(&file_contents).map_err(|err| err.to_string())?;

    // Generate the initial composition structure and fill in known metadata
    let mut composition = Composition::new("Error", None, None, None);
    // TODO: Fill in metadata from MIDI, set correct title, etc.

    // TODO: Implement construction of Composition from MIDI

    // Return the fully constructed composition
    Ok(composition)
  }

  fn save(_path: &str, _composition: &Composition) -> Result<usize, String> {
    // TODO: Implement
    Ok(0)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_midi_whatever() {}
}
