use super::Convert;
use crate::Composition;
use alloc::string::String;
use std::fs;

pub struct AmmStorage;

impl Convert for AmmStorage {
  fn load(path: &str) -> Result<Composition, String> {
    // TODO: Implement
    let _file_contents = fs::read_to_string(path).map_err(|err| err.to_string())?;
    Ok(Composition::new("Error", None, None, None))
  }

  fn save(_path: &str, _composition: &Composition) -> Result<usize, String> {
    // TODO: Implement
    Ok(0)
  }
}
