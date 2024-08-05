use super::Convert;
use crate::Composition;
use alloc::string::String;
use std::fs;

pub struct MidiConverter;

impl Convert for MidiConverter {
  fn load(path: &str) -> Result<Composition, String> {
    // TODO: Implement
    let file_contents = fs::read_to_string(path).map_err(|err| err.to_string())?;
    Ok(Composition::new("Error", None, None, None))
  }

  fn save(path: &str, composition: &Composition) -> Result<usize, String> {
    // TODO: Implement
    Ok(0)
  }
}
