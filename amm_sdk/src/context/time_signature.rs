use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Represents a type of time signature marking, whether explicit or implicit.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum TimeSignatureType {
  /// ![Common Time](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/time-symbol-common.png)
  ///
  /// Equivalent to a [`TimeSignatureType::Explicit`] time signature of 4/4.
  #[default]
  CommonTime,
  /// ![Cut Time](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/time-symbol-cut.png)
  ///
  /// Equivalent to a [`TimeSignatureType::Explicit`] time signature of 2/2.
  CutTime,
  /// ![Explicit Time](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/time-symbol-normal.png)
  ///
  /// Represents the number of beats in each measure (numerator) and the
  /// note value which represents a beat (denominator). For example, `3/4`
  /// represents 3 beats per measure, with a quarter note receiving one beat.
  Explicit,
  /// Represents an explicit lack of time signature,
  /// also called "senza misura".
  None,
}

/// Represents a time signature in music notation.
///
/// Some `signature` types are implicit (e.g., `CommonTime` = `4/4`,
/// `CutTime` = `2/2`), while others require an explicit `numerator` and `denominator`.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub struct TimeSignature {
  /// The type of time signature marking, whether explicit or implicit.
  pub signature: TimeSignatureType,
  /// The number of beats in each measure.
  pub numerator: u8,
  /// The note value which represents a beat in the measure (e.g.,
  /// `4` = quarter note, `8` = eighth note, etc.).
  pub denominator: u8,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl TimeSignature {
  /// Creates a new time signature with the given *implicit* type.
  ///
  /// **Note:** This method should only be used for implicit time
  /// signatures like [`TimeSignatureType::CommonTime`] and
  /// [`TimeSignatureType::CutTime`]. If you need to create an explicit
  /// time signature, use [`TimeSignature::new_explicit`] instead.
  #[must_use]
  pub const fn new(signature: TimeSignatureType) -> Self {
    Self {
      signature,
      numerator: match signature {
        TimeSignatureType::CommonTime | TimeSignatureType::Explicit => 4,
        TimeSignatureType::CutTime => 2,
        TimeSignatureType::None => 0,
      },
      denominator: match signature {
        TimeSignatureType::CommonTime | TimeSignatureType::Explicit => 4,
        TimeSignatureType::CutTime => 2,
        TimeSignatureType::None => 0,
      },
    }
  }

  /// Creates a new time signature with the given *explicit* value.
  ///
  /// The `numerator` indicates the number of beats in each measure,
  /// and the `denominator` designates the note value that represents
  /// a beat in the measure (e.g., `4` = quarter note, `8` = eighth note,
  /// etc.).
  #[must_use]
  pub const fn new_explicit(numerator: u8, denominator: u8) -> Self {
    Self {
      signature: TimeSignatureType::Explicit,
      numerator,
      denominator,
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for TimeSignatureType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::CommonTime => "Common Time",
        Self::CutTime => "Cut Time",
        Self::Explicit => "Explicit",
        Self::None => "Senza Misura",
      }
    )
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for TimeSignature {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self.signature {
        TimeSignatureType::Explicit => format!("{}/{}", self.numerator, self.denominator),
        _ => self.signature.to_string(),
      }
    )
  }
}
