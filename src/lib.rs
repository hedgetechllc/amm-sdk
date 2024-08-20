#[cfg_attr(not(feature = "std"), no_std)]
#[macro_use]
extern crate alloc;

mod composition;
mod context;
mod modification;
mod note;
mod storage;
mod structure;

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
  Chord, ChordContent, MultiVoice, MultiVoiceContent, Part, PartContent, PartTimeslice, Phrase, PhraseContent, Section,
  SectionContent, Staff, StaffContent, Timeslice, TimesliceContent, TimesliceContext, TimeslicePhraseDetails,
};

#[cfg(target_arch = "wasm32")]
use {wasm_bindgen::prelude::*, web_sys::console};

#[cfg(target_arch = "wasm32")]
fn custom_panic() -> ! {
  core::arch::wasm32::unreachable()
}

#[cfg(not(target_arch = "wasm32"))]
fn custom_panic() -> ! {
  unsafe { core::hint::unreachable_unchecked() }
}

pub trait UncheckedResultUnwrap<T, E> {
  fn unwrap_ok_unchecked(self) -> T;
  fn unwrap_err_unchecked(self) -> E;
}

impl<T, E> UncheckedResultUnwrap<T, E> for Result<T, E> {
  fn unwrap_ok_unchecked(self) -> T {
    match self {
      Ok(value) => value,
      Err(_) => custom_panic(),
    }
  }

  fn unwrap_err_unchecked(self) -> E {
    match self {
      Ok(_) => custom_panic(),
      Err(error) => error,
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn hello_world() {
  console::log_1(&"Hello world".into());
}
