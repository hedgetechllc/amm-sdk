mod composition;
mod context;
mod modification;
mod note;
mod storage;
mod structure;

use wasm_bindgen::prelude::*;
use web_sys::console;

pub use composition::Composition;
pub use context::{
  Clef, ClefType, DynamicMarking, Key, KeyMode, Tempo, TempoMarking, TempoModification, TimeSignature,
};
pub use modification::{NoteModification, PhraseModification, PhraseModificationType, SliceModification};
pub use note::{Accidental, Duration, Note, Pitch};
pub use note::{Beamed, DisplayOptions, Stem, Voice};
pub use storage::Storage;
pub use structure::{MusicalSlice, NotationalItem, NotationalSlice, Slice, Staff, System, SystemIndicator};

#[wasm_bindgen]
pub fn hello_world() {
  console::log_1(&"Hello world".into());
}
