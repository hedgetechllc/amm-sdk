use super::Convert;
use crate::{
  Accidental, Chord, ChordContent, ChordModification, ChordModificationType, Clef, ClefSymbol, ClefType, Composition,
  Direction, DirectionType, Duration, DurationType, Dynamic, DynamicMarking, HandbellTechnique, Key, KeyMode,
  KeySignature, MultiVoice, Note, NoteModification, NoteModificationType, PedalType, Phrase, PhraseContent,
  PhraseModification, PhraseModificationType, Pitch, PitchName, Section, SectionModificationType, Staff, StaffContent,
  Tempo, TempoMarking, TempoSuggestion, TimeSignature, TimeSignatureType,
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
    let mut part = composition.add_part("todo_part_name");
    let section = part.add_default_section();
    let staff_rc = section.borrow_mut().add_staff("todo_staff_name", None, None, None);
    let mut staff = staff_rc.borrow_mut();
    staff.add_chord();

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
