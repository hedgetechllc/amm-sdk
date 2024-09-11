use crate::context::Tempo;
use alloc::string::String;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "json")]
use {
  amm_internal::json_prelude::*,
  amm_macros::{JsonDeserialize, JsonSerialize},
};

#[cfg_attr(feature = "json", derive(JsonDeserialize, JsonSerialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum DurationType {
  Maxima,
  Long,
  Breve,
  Whole,
  Half,
  #[default]
  Quarter,
  Eighth,
  Sixteenth,
  ThirtySecond,
  SixtyFourth,
  OneHundredTwentyEighth,
  TwoHundredFiftySixth,
  FiveHundredTwelfth,
  OneThousandTwentyFourth,
  TwoThousandFortyEighth,
}

#[cfg_attr(feature = "json", derive(JsonDeserialize, JsonSerialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Duration {
  pub value: DurationType,
  pub dots: u8,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Duration {
  #[must_use]
  pub fn new(value: DurationType, dots: u8) -> Self {
    Self { value, dots }
  }

  #[must_use]
  fn dots_from_remainder(base_value: f64, full_value: f64) -> u8 {
    let (mut current_value, mut dots) = (base_value, 0);
    while full_value - current_value >= 0.000_488_281_25 {
      dots += 1;
      current_value += base_value / f64::powi(2.0, i32::from(dots));
    }
    dots
  }

  #[must_use]
  pub fn from_beats(beat_base_value: &Duration, beats: f64) -> Self {
    let value = beats * beat_base_value.value();
    match value {
      v if v >= 8.0 => Duration::new(DurationType::Maxima, Self::dots_from_remainder(8.0, v)),
      v if v >= 4.0 => Duration::new(DurationType::Long, Self::dots_from_remainder(4.0, v)),
      v if v >= 2.0 => Duration::new(DurationType::Breve, Self::dots_from_remainder(2.0, v)),
      v if v >= 1.0 => Duration::new(DurationType::Whole, Self::dots_from_remainder(1.0, v)),
      v if v >= 0.5 => Duration::new(DurationType::Half, Self::dots_from_remainder(0.5, v)),
      v if v >= 0.25 => Duration::new(DurationType::Quarter, Self::dots_from_remainder(0.25, v)),
      v if v >= 0.125 => Duration::new(DurationType::Eighth, Self::dots_from_remainder(0.125, v)),
      v if v >= 0.062_5 => Duration::new(DurationType::Sixteenth, Self::dots_from_remainder(0.062_5, v)),
      v if v >= 0.031_25 => Duration::new(DurationType::ThirtySecond, Self::dots_from_remainder(0.031_25, v)),
      v if v >= 0.015_625 => Duration::new(DurationType::SixtyFourth, Self::dots_from_remainder(0.015_625, v)),
      v if v >= 0.007_812_5 => Duration::new(
        DurationType::OneHundredTwentyEighth,
        Self::dots_from_remainder(0.007_812_5, v),
      ),
      v if v >= 0.003_906_25 => Duration::new(
        DurationType::TwoHundredFiftySixth,
        Self::dots_from_remainder(0.003_906_25, v),
      ),
      v if v >= 0.001_953_125 => Duration::new(
        DurationType::FiveHundredTwelfth,
        Self::dots_from_remainder(0.001_953_125, v),
      ),
      v if v >= 0.000_976_562_5 => Duration::new(
        DurationType::OneThousandTwentyFourth,
        Self::dots_from_remainder(0.000_976_562_5, v),
      ),
      v => Duration::new(
        DurationType::TwoThousandFortyEighth,
        Self::dots_from_remainder(0.000_488_281_25, v),
      ),
    }
  }

  #[must_use]
  pub fn from_duration(tempo: &Tempo, duration: f64) -> Self {
    Duration::from_beats(&tempo.base_note, duration * f64::from(tempo.beats_per_minute) / 60.0)
  }

  #[must_use]
  #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
  pub(crate) fn get_minimum_divisible_notes(beats: f64) -> (DurationType, u32) {
    match beats {
      beats if beats.fract() < 0.000_976_562_5 => (DurationType::Whole, beats as u32),
      beats if beats.fract() >= 0.5 => (DurationType::Half, (2.0 * beats) as u32),
      beats if beats.fract() >= 0.25 => (DurationType::Quarter, (4.0 * beats) as u32),
      beats if beats.fract() >= 0.125 => (DurationType::Eighth, (8.0 * beats) as u32),
      beats if beats.fract() >= 0.062_5 => (DurationType::Sixteenth, (16.0 * beats) as u32),
      beats if beats.fract() >= 0.031_25 => (DurationType::ThirtySecond, (32.0 * beats) as u32),
      beats if beats.fract() >= 0.015_625 => (DurationType::SixtyFourth, (64.0 * beats) as u32),
      beats if beats.fract() >= 0.007_812_5 => (DurationType::OneHundredTwentyEighth, (128.0 * beats) as u32),
      beats if beats.fract() >= 0.003_906_25 => (DurationType::TwoHundredFiftySixth, (256.0 * beats) as u32),
      beats if beats.fract() >= 0.001_953_125 => (DurationType::FiveHundredTwelfth, (512.0 * beats) as u32),
      beats if beats.fract() >= 0.000_976_562_5 => (DurationType::OneThousandTwentyFourth, (1024.0 * beats) as u32),
      _ => (DurationType::TwoThousandFortyEighth, (2048.0 * beats) as u32),
    }
  }

  #[must_use]
  pub fn value(&self) -> f64 {
    let base_duration = match self.value {
      DurationType::Maxima => 8.0,
      DurationType::Long => 4.0,
      DurationType::Breve => 2.0,
      DurationType::Whole => 1.0,
      DurationType::Half => 0.5,
      DurationType::Quarter => 0.25,
      DurationType::Eighth => 0.125,
      DurationType::Sixteenth => 0.062_5,
      DurationType::ThirtySecond => 0.031_25,
      DurationType::SixtyFourth => 0.015_625,
      DurationType::OneHundredTwentyEighth => 0.007_812_5,
      DurationType::TwoHundredFiftySixth => 0.003_906_25,
      DurationType::FiveHundredTwelfth => 0.001_953_125,
      DurationType::OneThousandTwentyFourth => 0.000_976_562_5,
      DurationType::TwoThousandFortyEighth => 0.000_488_281_25,
    };
    (0..=self.dots)
      .map(|i| base_duration / f64::powi(2.0, i32::from(i)))
      .sum()
  }

  #[must_use]
  pub fn beats(&self, base_beat_value: f64) -> f64 {
    self.value() / base_beat_value
  }

  #[must_use]
  pub fn split(&self, mut times: u8) -> Self {
    // Note: `times` must be a power of 2
    let mut duration = self.value;
    while times > 1 {
      times /= 2;
      duration = match duration {
        DurationType::Maxima => DurationType::Long,
        DurationType::Long => DurationType::Breve,
        DurationType::Breve => DurationType::Whole,
        DurationType::Whole => DurationType::Half,
        DurationType::Half => DurationType::Quarter,
        DurationType::Quarter => DurationType::Eighth,
        DurationType::Eighth => DurationType::Sixteenth,
        DurationType::Sixteenth => DurationType::ThirtySecond,
        DurationType::ThirtySecond => DurationType::SixtyFourth,
        DurationType::SixtyFourth => DurationType::OneHundredTwentyEighth,
        DurationType::OneHundredTwentyEighth => DurationType::TwoHundredFiftySixth,
        DurationType::TwoHundredFiftySixth => DurationType::FiveHundredTwelfth,
        DurationType::FiveHundredTwelfth => DurationType::OneThousandTwentyFourth,
        DurationType::OneThousandTwentyFourth | DurationType::TwoThousandFortyEighth => {
          DurationType::TwoThousandFortyEighth
        }
      };
    }
    Self::new(duration, self.dots)
  }
}

#[must_use]
fn dots_to_text(dots: u8) -> String {
  match dots {
    0 => String::new(),
    1 => String::from("Dotted "),
    2 => String::from("Double-Dotted "),
    _ => format!("{dots}-Dotted "),
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for DurationType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        DurationType::Maxima => "Maxima",
        DurationType::Long => "Long",
        DurationType::Breve => "Breve",
        DurationType::Whole => "Whole",
        DurationType::Half => "Half",
        DurationType::Quarter => "Quarter",
        DurationType::Eighth => "Eighth",
        DurationType::Sixteenth => "Sixteenth",
        DurationType::ThirtySecond => "Thirty-Second",
        DurationType::SixtyFourth => "Sixty-Fourth",
        DurationType::OneHundredTwentyEighth => "One-Hundred-Twenty-Eighth",
        DurationType::TwoHundredFiftySixth => "Two-Hundred-Fifty-Sixth",
        DurationType::FiveHundredTwelfth => "Five-Hundred-Twelfth",
        DurationType::OneThousandTwentyFourth => "One-Thousand-Twenty-Fourth",
        DurationType::TwoThousandFortyEighth => "Two-Thousand-Forty-Eighth",
      }
    )
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Duration {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{}{}", self.value, dots_to_text(self.dots))
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use amm_internal::JsonSerializer;

  #[test]
  fn test_value() {
    assert_eq!(Duration::new(DurationType::Whole, 3).value(), 1.875);
    assert_eq!(Duration::new(DurationType::Quarter, 4).value(), 0.484_375);
  }

  #[test]
  fn test_from_duration() {
    let tempo = Tempo::new(Duration::new(DurationType::Quarter, 0), 120);
    assert!(Duration::from_duration(&tempo, 0.5) == Duration::new(DurationType::Quarter, 0));
    assert!(Duration::from_duration(&tempo, 0.875) == Duration::new(DurationType::Quarter, 2));
    assert!(Duration::from_duration(&tempo, 1.0) == Duration::new(DurationType::Half, 0));
  }

  #[test]
  fn test_duration_serialization_json() {
    let tempo = Tempo::new(Duration::new(DurationType::Quarter, 1), 130);
    assert_eq!(
      JsonSerializer::serialize_json(&tempo),
      "{\"base_note\":{\"type\":\"Quarter\",\"dots\":1},\"beats_per_minute\":130}"
    );
  }
}
