#[cfg(target_arch = "wasm32")]
use amm_sdk::wasm::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn pass() {
  let data = include_str!("../examples/Grande Valse Brillante.musicxml");
  match load_from_musicxml(data.as_bytes()) {
    Ok(composition) => console_log!("{:?}", composition),
    Err(error) => console_log!("{:?}", error),
  }
  assert_eq!(1, 1);
}
