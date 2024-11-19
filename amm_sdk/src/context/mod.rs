//! This module contains all of the data types used to represent
//! the contextual content and information in a music score.

mod clef;
mod dynamic;
mod id;
mod key;
mod tempo;
mod tempo_suggestion;
mod time_signature;

pub(crate) use id::generate_id;

pub use clef::{Clef, ClefSymbol, ClefType};
pub use dynamic::Dynamic;
pub use key::{Key, KeyMode, KeySignature};
pub use tempo::Tempo;
pub use tempo_suggestion::{TempoMarking, TempoSuggestion};
pub use time_signature::{TimeSignature, TimeSignatureType};
