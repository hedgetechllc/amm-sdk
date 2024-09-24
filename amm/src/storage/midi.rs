use super::Convert;
use crate::Composition;
use alloc::string::String;
use midly::Smf;
use std::fs;

pub struct MidiConverter;

impl MidiConverter {
  fn load_from_midi(data: &[u8]) -> Result<Composition, String> {
    // Parse the MIDI representation
    let _midi = Smf::parse(data).map_err(|err| err.to_string())?;

    // Generate the initial composition structure and fill in known metadata
    let mut composition = Composition::new("Error", None, None, None);
    // TODO: Fill in metadata from MIDI, set correct title, etc.

    // TODO: Implement construction of Composition from MIDI
    let part = composition.add_part("todo_part_name");
    let section = part.add_section("Default");
    let staff = section.add_staff("todo_staff_name");
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
    MidiConverter::load_from_midi(data)
  }

  fn save(_path: &str, _composition: &Composition) -> Result<usize, String> {
    todo!(); // TODO: Implement
  }
}

#[cfg(test)]
mod test {
  //use super::*;

  #[test]
  fn test_midi_whatever() {}
}
