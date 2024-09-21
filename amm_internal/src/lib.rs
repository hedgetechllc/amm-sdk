#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};

pub trait JsonSerializer {
  fn serialize_json(&self) -> String;
}

pub trait JsonDeserializer {
  /// TODO
  ///
  /// # Errors
  /// TODO
  fn deserialize_json(json: &str) -> Result<Self, String>
  where
    Self: Sized;
}

impl JsonSerializer for bool {
  fn serialize_json(&self) -> String {
    self.to_string()
  }
}

impl JsonSerializer for u8 {
  fn serialize_json(&self) -> String {
    self.to_string()
  }
}

impl JsonSerializer for u16 {
  fn serialize_json(&self) -> String {
    self.to_string()
  }
}

impl JsonSerializer for u32 {
  fn serialize_json(&self) -> String {
    self.to_string()
  }
}

impl JsonSerializer for usize {
  fn serialize_json(&self) -> String {
    self.to_string()
  }
}

impl JsonSerializer for i8 {
  fn serialize_json(&self) -> String {
    self.to_string()
  }
}

impl JsonSerializer for i16 {
  fn serialize_json(&self) -> String {
    self.to_string()
  }
}

impl JsonSerializer for i32 {
  fn serialize_json(&self) -> String {
    self.to_string()
  }
}

impl JsonSerializer for isize {
  fn serialize_json(&self) -> String {
    self.to_string()
  }
}

impl JsonSerializer for String {
  fn serialize_json(&self) -> String {
    format!("\"{self}\"")
  }
}

impl JsonDeserializer for bool {
  fn deserialize_json(json: &str) -> Result<Self, String> {
    json.parse::<bool>().map_err(|err| err.to_string())
  }
}

impl JsonDeserializer for u8 {
  fn deserialize_json(json: &str) -> Result<Self, String> {
    json.parse::<u8>().map_err(|err| err.to_string())
  }
}

impl JsonDeserializer for u16 {
  fn deserialize_json(json: &str) -> Result<Self, String> {
    json.parse::<u16>().map_err(|err| err.to_string())
  }
}

impl JsonDeserializer for u32 {
  fn deserialize_json(json: &str) -> Result<Self, String> {
    json.parse::<u32>().map_err(|err| err.to_string())
  }
}

impl JsonDeserializer for usize {
  fn deserialize_json(json: &str) -> Result<Self, String> {
    json.parse::<usize>().map_err(|err| err.to_string())
  }
}

impl JsonDeserializer for i8 {
  fn deserialize_json(json: &str) -> Result<Self, String> {
    json.parse::<i8>().map_err(|err| err.to_string())
  }
}

impl JsonDeserializer for i16 {
  fn deserialize_json(json: &str) -> Result<Self, String> {
    json.parse::<i16>().map_err(|err| err.to_string())
  }
}

impl JsonDeserializer for i32 {
  fn deserialize_json(json: &str) -> Result<Self, String> {
    json.parse::<i32>().map_err(|err| err.to_string())
  }
}

impl JsonDeserializer for isize {
  fn deserialize_json(json: &str) -> Result<Self, String> {
    json.parse::<isize>().map_err(|err| err.to_string())
  }
}

impl JsonDeserializer for String {
  fn deserialize_json(json: &str) -> Result<Self, String> {
    Ok(json.to_string())
  }
}

pub mod amm_prelude {
  pub use super::JsonDeserializer;
  pub use super::JsonSerializer;
  pub use alloc::collections::{BTreeMap, BTreeSet};
  pub use alloc::string::{String, ToString};
  pub use alloc::vec::Vec;

  #[must_use]
  pub fn json_get_type(data: &str) -> &str {
    if let Some((_, type_str)) = data.split_once("\"_type\":\"") {
      type_str.split_once('"').unwrap_or_default().0
    } else {
      ""
    }
  }

  #[must_use]
  pub fn json_next_key(data: &str) -> (&str, &str) {
    let mut key_start = 0;
    for (idx, ch) in data.chars().enumerate() {
      if ch == '"' {
        if key_start > 0 {
          return (&data[(idx + 1)..], &data[key_start..idx]);
        }
        key_start = idx + 1;
      }
    }
    ("", "")
  }

  #[must_use]
  pub fn json_next_value(data: &str) -> (&str, &str) {
    let (mut value_start, mut num_openers, mut in_value) = (0, 0, false);
    for (idx, ch) in data.chars().enumerate() {
      if in_value {
        match ch {
          ',' | '"' => {
            if num_openers == 0 {
              return (&data[(idx + 1)..], &data[value_start..idx]);
            }
          }
          '[' | '{' => {
            num_openers += 1;
          }
          ']' | '}' => {
            num_openers -= 1;
            if num_openers == 0 {
              return (&data[(idx + 1)..], &data[value_start..idx]);
            }
          }
          _ => (),
        }
      } else if ch != ' ' && ch != ':' && ch != ']' && ch != '}' && ch != ',' {
        value_start = if ch == '"' || ch == '[' || ch == '{' {
          if ch == '[' || ch == '{' {
            num_openers += 1;
          }
          idx + 1
        } else {
          idx
        };
        in_value = true;
      }
    }
    ("", &data[value_start..])
  }
}
