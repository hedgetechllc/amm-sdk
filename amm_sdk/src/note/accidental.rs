use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Represents a musical pitch modification.
///
/// Common examples include sharps and flats, which raise or lower the
/// pitch of a note by a half step (semitone).
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum Accidental {
  /// Represents an explicit lack of an accidental.
  ///
  /// This means that any accidentals will be inferred from either the key
  /// signature or a previously played accidental in the same measure.
  #[default]
  None,
  /// <span class="smufl">TODO</span>
  ///
  /// Represents a natural pitch, which is neither sharp nor flat. Used primarily to
  /// negate the effect of a previous accidental or a key signature.
  Natural,
  /// <span class="smufl">TODO</span>
  ///
  /// Represents a sharp pitch, which raises the pitch of a note by a half step (semitone).
  Sharp,
  /// <span class="smufl">TODO</span>
  ///
  /// Represents a flat pitch, which lowers the pitch of a note by a half step (semitone).
  Flat,
  /// <span class="smufl">TODO</span>
  ///
  /// Represents a double-sharp pitch, which raises the pitch of a
  /// note by a whole step (2 semitones).
  DoubleSharp,
  /// <span class="smufl">TODO</span>
  ///
  /// Represents a double-flat pitch, which lowers the pitch of a
  /// note by a whole step (2 semitones).
  DoubleFlat,
}

impl Accidental {
  /// Returns the number of semitones that this accidental raises or lowers a pitch.
  #[must_use]
  pub fn value(&self) -> i8 {
    match self {
      Self::Sharp => 1,
      Self::Flat => -1,
      Self::DoubleSharp => 2,
      Self::DoubleFlat => -2,
      Self::None | Self::Natural => 0,
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Accidental {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Natural => "♮",
        Self::Sharp => "♯",
        Self::Flat => "♭",
        Self::DoubleSharp => "𝄪",
        Self::DoubleFlat => "𝄫",
        Self::None => "",
      }
    )
  }
}
