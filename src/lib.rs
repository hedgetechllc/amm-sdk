#[cfg_attr(not(feature = "std"), no_std)]
#[macro_use]
extern crate alloc;

mod composition;
mod context;
mod modification;
mod note;
mod storage;
mod structure;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub use composition::Composition;
pub use context::{Clef, ClefType, ClefSymbol, Dynamic, DynamicMarking, Key, KeyMode, KeySignature, Tempo, TempoMarking, TempoSuggestion, TimeSignature, TimeSignatureType};
pub use modification::{
  ChordModification, ChordModificationType, Direction, DirectionType, HandbellTechnique, NoteModification,
  NoteModificationType, PedalType, PhraseModification, PhraseModificationType, SectionModification,
  SectionModificationType,
};
pub use note::{Accidental, Duration, DurationType, Note, Pitch};
pub use storage::Storage;
pub use structure::{
  Chord, ChordContent, MultiVoice, MultiVoiceContent, Part, PartContent, PartTimeslice, Phrase, PhraseContent, Section,
  SectionContent, Staff, StaffContent, Timeslice, TimesliceContent, TimesliceContext, TimeslicePhraseDetails,
};
