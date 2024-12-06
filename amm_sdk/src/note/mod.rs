//! This module contains all musical note-related structs and enums.

mod accidental;
mod duration;
mod note;
mod pitch;

pub use accidental::Accidental;
pub use duration::{Duration, DurationType};
pub use note::Note;
pub use pitch::{Pitch, PitchName};
