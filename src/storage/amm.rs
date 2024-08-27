use super::Convert;
use crate::Composition;
use alloc::string::String;
use std::fs;

pub struct AmmStorage;

impl Convert for AmmStorage {
  fn load(path: &str) -> Result<Composition, String> {
    let _file_contents = fs::read_to_string(path).map_err(|err| err.to_string())?;
    todo!() // TODO: Implement
  }

  fn save(_path: &str, _composition: &Composition) -> Result<usize, String> {
    todo!() // TODO: Implement
  }
}
