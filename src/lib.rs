//#![no_std]

#[macro_use]
extern crate alloc;

mod composition;
mod context;
mod modification;
mod note;
mod storage;
mod structure;

use wasm_bindgen::prelude::*;
use web_sys::console;

pub use composition::Composition;
pub use context::{Clef, ClefType, DynamicMarking, Key, KeyMode, Tempo, TempoMarking, TimeSignature};
pub use modification::{
  ChordModification, ChordModificationType, Direction, DirectionType, HandbellTechnique, NoteModification,
  NoteModificationType, PedalType, PhraseModification, PhraseModificationType, SectionModification,
  SectionModificationType,
};
pub use note::{Accidental, Duration, Note, Pitch};
pub use storage::Storage;
pub use structure::{
  Chord, ChordContent, DirectionalTimeslice, MultiVoice, MultiVoiceContent, MusicalTimeslice, Part, PartContent,
  Phrase, PhraseContent, Section, SectionContent, Staff, StaffContent, Timeslice,
};

#[wasm_bindgen]
pub fn hello_world() {
  console::log_1(&"Hello world".into());
}
