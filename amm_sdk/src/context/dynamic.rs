use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};

/// Represents a dynamic marking in music notation.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum Dynamic {
  /// ![Forte](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/f.png)
  ///
  /// A `forte` dynamic marking indicates that corresponding music should be played
  /// relatively loudly.
  /// 
  /// The magnitude of the notated forte is specified by the `u8` value.
  /// For example, `Forte(3)` represents a dynamic marking of `fff`.
  Forte(u8),
  #[default]
  /// ![Mezzo Forte](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/mf.png)
  /// 
  /// A `mezzo-forte` dynamic marking indicates that corresponding music should be played
  /// only slightly louder than average.
  MezzoForte,
  /// ![Mezzo Piano](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/mp.png)
  /// 
  /// A `mezzo-piano` dynamic marking indicates that corresponding music should be played
  /// only slightly softer than average.
  MezzoPiano,
  /// ![Piano](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/p.png)
  ///
  /// A `piano` dynamic marking indicates that corresponding music should be played
  /// relatively softly.
  /// 
  /// The magnitude of the notated piano is specified by the `u8` value.
  /// For example, `Piano(3)` represents a dynamic marking of `ppp`.
  Piano(u8),
}

impl Dynamic {
  /// Returns the relative loudness of the dynamic marking in the range `[0.0, 1.0]`.
  #[must_use]
  pub fn value(&self) -> f32 {
    match *self {
      Self::Piano(magnitude) => (0.5 - (0.1 * f32::from(magnitude))).max(0.05),
      Self::MezzoPiano => 0.45,
      Self::MezzoForte => 0.55,
      Self::Forte(magnitude) => (0.5 + (0.1 * f32::from(magnitude))).min(1.0),
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Dynamic {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match *self {
      Self::Piano(magnitude) => write!(f, "{}", "p".repeat(usize::from(magnitude))),
      Self::MezzoPiano => write!(f, "mp"),
      Self::MezzoForte => write!(f, "mf"),
      Self::Forte(magnitude) => write!(f, "{}", "f".repeat(usize::from(magnitude))),
    }
  }
}
