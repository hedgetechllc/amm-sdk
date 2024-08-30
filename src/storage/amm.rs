use super::Convert;
use crate::Composition;
use alloc::string::String;
use std::fs;

pub struct AmmStorage;

impl AmmStorage {
  fn load_from_amm(_data: &[u8]) -> Result<Composition, String> {
    todo!() // TODO: Implement
  }

  fn save_to_amm(_composition: &Composition) -> Result<String, String> {
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

  fn save(path: &str, composition: &Composition) -> Result<usize, String> {
    let amm = AmmStorage::save_to_amm(composition).map_err(|err| err.to_string())?;
    fs::write(path, amm.as_bytes()).map_err(|err| err.to_string())?;
    Ok(amm.as_bytes().len())
  }
}
