use crate::*;

use {wasm_bindgen::prelude::*, web_sys::console};

#[wasm_bindgen]
pub fn send_clef() -> Result<JsValue, JsValue> {
  console::log_1(&"Hello world".into());
  let clef = Clef::Baritone(ClefType::GClef);
  Ok(serde_wasm_bindgen::to_value(&clef)?)
}

#[wasm_bindgen]
pub fn load_from_musicxml(data: &[u8]) -> Result<JsValue, JsValue> {
  let composition = Storage::MusicXML.load_data(data);
  match composition {
    Ok(composition) => Ok(serde_wasm_bindgen::to_value(&composition)?),
    Err(error) => Err(JsValue::from_str(&error.to_string())),
  }
}
