use super::Convert;
use crate::Composition;
use alloc::string::String;
use std::fs;

pub struct AmmStorage;

impl AmmStorage {
  fn load_from_amm(_data: &[u8]) -> Result<Composition, String> {
    todo!() // TODO: Implement
  }
}

impl Convert for AmmStorage {
  fn load(path: &str) -> Result<Composition, String> {
    let data = fs::read(path).map_err(|err| err.to_string())?;
    AmmStorage::load_from_amm(&data)
  }

  fn load_data(data: &[u8]) -> Result<Composition, String> {
    AmmStorage::load_from_amm(data)
  }

  fn save(_path: &str, _composition: &Composition) -> Result<usize, String> {
    todo!() // TODO: Implement
  }
}
