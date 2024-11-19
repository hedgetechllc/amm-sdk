use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Represents the symbol used to designate a clef.
///
/// Note that the same symbol can be used for different clef types.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum ClefSymbol {
  /// ![G Clef](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/clef-G.png)
  ///
  /// The middle curl of the G clef wraps around the staff line used to notate a pitch of G4.
  #[default]
  GClef,
  /// ![C Clef](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/clef-C.png)
  ///
  /// The middle of the C clef indicates the staff line used to notate a pitch of C4 (middle C).
  CClef,
  /// ![F Clef](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/clef-F.png)
  ///
  /// The two dots of the F clef surround the staff line used to notate a pitch of F3.
  FClef,
}

/// Designates the meaning of a clef.
///
/// A clef is used to determine the pitches for the notes on a staff.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum ClefType {
  /// Designates that pitch G4 is located on the second line from the bottom of the staff.
  #[default]
  Treble,
  /// Designates that pitch F3 is located on the second line from the top of the staff.
  Bass,
  /// Designates that pitch G4 is located on the bottom line of the staff.
  FrenchViolin,
  /// Designates that pitch F3 is located on the top line of the staff.
  Subbass,
  /// Designates that pitch C4 is located on the second line from the top of the staff.
  Tenor,
  /// Designates that pitch C4 is located on the middle line of the staff.
  Alto,
  /// Designates that pitch C4 is located on the bottom line of the staff.
  Soprano,
  /// Designates that pitch C4 is located on the second line from the bottom of the staff.
  MezzoSoprano,
  /// Designates that pitch C4 is located on the top line of the staff.
  Baritone,
}

/// Represents a clef which is used to determine the pitches for the notes on a staff.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub struct Clef {
  /// The symbol used to designate the clef.
  pub symbol: ClefSymbol,
  /// The meaning of the clef.
  pub clef_type: ClefType,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Clef {
  /// Creates a new clef with the given type and optional symbol.
  ///
  /// If the `symbol` parameter is `None`, the most common symbol for the
  /// given `clef_type` will be used instead.
  #[must_use]
  pub fn new(clef_type: ClefType, symbol: Option<ClefSymbol>) -> Self {
    Self {
      symbol: match clef_type {
        ClefType::Treble | ClefType::FrenchViolin => ClefSymbol::GClef,
        ClefType::Bass | ClefType::Subbass => ClefSymbol::FClef,
        ClefType::Tenor | ClefType::Alto | ClefType::Soprano | ClefType::MezzoSoprano => ClefSymbol::CClef,
        ClefType::Baritone => symbol.unwrap_or(ClefSymbol::CClef),
      },
      clef_type,
    }
  }
}

impl core::fmt::Display for ClefSymbol {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::GClef => "G-Clef",
        Self::CClef => "C-Clef",
        Self::FClef => "F-Clef",
      }
    )
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for ClefType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Treble => "Treble",
        Self::Bass => "Bass",
        Self::FrenchViolin => "French Violin",
        Self::Subbass => "Subbass",
        Self::Tenor => "Tenor",
        Self::Alto => "Alto",
        Self::Soprano => "Soprano",
        Self::MezzoSoprano => "Mezzo Soprano",
        Self::Baritone => "Baritone",
      }
    )
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Clef {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}", self.clef_type)
  }
}
