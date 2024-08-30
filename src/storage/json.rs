use super::{Convert, Serialize, SerializedItem};
use crate::Composition;
use alloc::string::String;
use std::fs;

pub struct JsonConverter;

impl JsonConverter {
  fn write_serialized_item(item: &SerializedItem) -> String {
    let mut string = String::from("{");
    for (idx, (key, value)) in item.attributes.iter().enumerate() {
      string += (String::from("\"") + key + "\":\"" + value + "\"").as_str();
      if !item.contents.is_empty() || !item.elements.is_empty() || idx + 1 < item.attributes.len() {
        string += ",";
      }
    }
    for (idx, (key, value)) in item.contents.iter().enumerate() {
      string += (String::from("\"") + key + "\":" + JsonConverter::write_serialized_item(value).as_str()).as_str();
      if !item.elements.is_empty() || idx + 1 < item.contents.len() {
        string += ",";
      }
    }
    for (idx, (key, value)) in item.elements.iter().enumerate() {
      string += (String::from("\"") + key + "\":[").as_str();
      for (subidx, subitem) in value.iter().enumerate() {
        string += JsonConverter::write_serialized_item(subitem).as_str();
        if subidx + 1 < value.len() {
          string += ",";
        }
      }
      string += if idx + 1 < item.elements.len() { "]," } else { "]" };
    }
    string += "}";
    string
  }

  fn write_serialized_item_pretty(item: &SerializedItem, depth: usize) -> String {
    let mut string = String::from("{\n");
    for (idx, (key, value)) in item.attributes.iter().enumerate() {
      string += (" ".repeat(depth * 2) + "\"" + key + "\": \"" + value + "\"").as_str();
      string += if !item.contents.is_empty() || !item.elements.is_empty() || idx + 1 < item.attributes.len() {
        ",\n"
      } else {
        "\n"
      };
    }
    for (idx, (key, value)) in item.contents.iter().enumerate() {
      string += (" ".repeat(depth * 2)
        + "\""
        + key
        + "\": "
        + JsonConverter::write_serialized_item_pretty(value, depth + 1).as_str())
      .as_str();
      string += if !item.elements.is_empty() || idx + 1 < item.contents.len() {
        ",\n"
      } else {
        "\n"
      };
    }
    for (idx, (key, value)) in item.elements.iter().enumerate() {
      string += (" ".repeat(depth * 2) + "\"" + key + "\": [\n").as_str();
      for (subidx, subitem) in value.iter().enumerate() {
        string += " ".repeat((depth + 1) * 2).as_str();
        string += JsonConverter::write_serialized_item_pretty(subitem, depth + 2).as_str();
        string += if subidx + 1 < value.len() { ",\n" } else { "\n" };
      }
      string += (" ".repeat(depth * 2) + if idx + 1 < item.elements.len() { "],\n" } else { "]\n" }).as_str();
    }
    string += (" ".repeat((depth - 1) * 2) + "}").as_str();
    string
  }

  fn load_from_json(_data: &[u8]) -> Result<Composition, String> {
    // TODO: Implement
    Err(String::from("Not implemented"))
  }

  fn save_to_json(composition: &Composition, pretty_print: bool) -> String {
    if pretty_print {
      JsonConverter::write_serialized_item_pretty(&composition.serialize(), 1)
    } else {
      JsonConverter::write_serialized_item(&composition.serialize())
    }
  }
}

impl Convert for JsonConverter {
  fn load(path: &str) -> Result<Composition, String> {
    let data = fs::read(path).map_err(|err| err.to_string())?;
    JsonConverter::load_from_json(&data)
  }

  fn load_data(data: &[u8]) -> Result<Composition, String> {
    JsonConverter::load_from_json(data)
  }

  fn save(path: &str, composition: &Composition) -> Result<usize, String> {
    let json = JsonConverter::save_to_json(composition, false);
    fs::write(path, json.as_bytes()).map_err(|err| err.to_string())?;
    Ok(json.as_bytes().len())
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::Storage;

  #[test]
  fn test_json_serialization() {
    let mut composition = Storage::MusicXML.load("./examples/Grande Valse Brillante.musicxml");
    match composition {
      Ok(ref mut composition) => {
        println!("{}", JsonConverter::save_to_json(composition, false));
        match Storage::JSON.save("./target/test_out.json", composition) {
          Ok(size) => {
            println!("Successfully stored JSON file containing {size} bytes");
            match Storage::JSON.load("./target/test_out.json") {
              Ok(ref mut _loaded) => {
                println!("Re-imported file from JSON representation, comparing to original...");
                // TODO: Test whether original and re-imported compositions are equal
              }
              Err(error) => println!("{}", error),
            }
          }
          Err(error) => println!("{}", error),
        }
      }
      Err(error) => println!("{}", error),
    }
  }
}
