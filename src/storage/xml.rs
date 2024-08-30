use super::Convert;
use crate::Composition;
use alloc::string::String;
use std::fs;

pub struct XmlConverter;

impl XmlConverter {
  fn load_from_xml(_data: &[u8]) -> Result<Composition, String> {
    todo!() // TODO: Implement
  }

  fn save_to_xml(_composition: &Composition) -> Result<String, String> {
    todo!() // TODO: Implement
  }
}

impl Convert for XmlConverter {
  fn load(path: &str) -> Result<Composition, String> {
    let data = fs::read(path).map_err(|err| err.to_string())?;
    XmlConverter::load_from_xml(&data)
  }

  fn load_data(data: &[u8]) -> Result<Composition, String> {
    XmlConverter::load_from_xml(data)
  }

  fn save(path: &str, composition: &Composition) -> Result<usize, String> {
    let xml = XmlConverter::save_to_xml(composition).map_err(|err| err.to_string())?;
    fs::write(path, xml.as_bytes()).map_err(|err| err.to_string())?;
    Ok(xml.as_bytes().len())
  }
}

#[cfg(test)]
mod test {
  use crate::{storage::Serialize, Storage};

  #[test]
  fn test_xml_serialization() {
    let mut composition = Storage::MusicXML.load("./examples/Grande Valse Brillante.musicxml");
    match composition {
      Ok(ref mut composition) => {
        println!("{:?}", composition.serialize());
      }
      Err(error) => println!("{}", error),
    }
  }
}
