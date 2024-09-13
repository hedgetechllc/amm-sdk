use crate::context::Tempo;
use alloc::string::String;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "json")]
use {
  amm_internal::json_prelude::*,
  amm_macros::{JsonDeserialize, JsonSerialize},
};

const MAXIMA_VALUE: f64 = 8.0;
const LONG_VALUE: f64 = 4.0;
const BREVE_VALUE: f64 = 2.0;
const WHOLE_VALUE: f64 = 1.0;
const HALF_VALUE: f64 = 0.5;
const QUARTER_VALUE: f64 = 0.25;
const EIGHTH_VALUE: f64 = 0.125;
const SIXTEENTH_VALUE: f64 = 0.062_5;
const THIRTY_SECOND_VALUE: f64 = 0.031_25;
const SIXTY_FOURTH_VALUE: f64 = 0.015_625;
const ONE_HUNDRED_TWENTY_EIGHTH_VALUE: f64 = 0.007_812_5;
const TWO_HUNDRED_FIFTY_SIXTH_VALUE: f64 = 0.003_906_25;
const FIVE_HUNDRED_TWELFTH_VALUE: f64 = 0.001_953_125;
const ONE_THOUSAND_TWENTY_FOURTH_VALUE: f64 = 0.000_976_562_5;
const TWO_THOUSAND_FOURTH_EIGHTH_VALUE: f64 = 0.000_488_281_25;

#[cfg_attr(feature = "json", derive(JsonDeserialize, JsonSerialize))]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
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
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
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
    while full_value - current_value >= TWO_THOUSAND_FOURTH_EIGHTH_VALUE {
      dots += 1;
      current_value += base_value / f64::powi(2.0, i32::from(dots));
    }
    dots
  }

  #[must_use]
  pub fn from_beats(beat_base_value: &Duration, beats: f64) -> Self {
    let value = beats * beat_base_value.value();
    match value {
      v if v >= MAXIMA_VALUE => Duration::new(DurationType::Maxima, Self::dots_from_remainder(MAXIMA_VALUE, v)),
      v if v >= LONG_VALUE => Duration::new(DurationType::Long, Self::dots_from_remainder(LONG_VALUE, v)),
      v if v >= BREVE_VALUE => Duration::new(DurationType::Breve, Self::dots_from_remainder(BREVE_VALUE, v)),
      v if v >= WHOLE_VALUE => Duration::new(DurationType::Whole, Self::dots_from_remainder(WHOLE_VALUE, v)),
      v if v >= HALF_VALUE => Duration::new(DurationType::Half, Self::dots_from_remainder(HALF_VALUE, v)),
      v if v >= QUARTER_VALUE => Duration::new(DurationType::Quarter, Self::dots_from_remainder(QUARTER_VALUE, v)),
      v if v >= EIGHTH_VALUE => Duration::new(DurationType::Eighth, Self::dots_from_remainder(EIGHTH_VALUE, v)),
      v if v >= SIXTEENTH_VALUE => {
        Duration::new(DurationType::Sixteenth, Self::dots_from_remainder(SIXTEENTH_VALUE, v))
      }
      v if v >= THIRTY_SECOND_VALUE => Duration::new(
        DurationType::ThirtySecond,
        Self::dots_from_remainder(THIRTY_SECOND_VALUE, v),
      ),
      v if v >= SIXTY_FOURTH_VALUE => Duration::new(
        DurationType::SixtyFourth,
        Self::dots_from_remainder(SIXTY_FOURTH_VALUE, v),
      ),
      v if v >= ONE_HUNDRED_TWENTY_EIGHTH_VALUE => Duration::new(
        DurationType::OneHundredTwentyEighth,
        Self::dots_from_remainder(ONE_HUNDRED_TWENTY_EIGHTH_VALUE, v),
      ),
      v if v >= TWO_HUNDRED_FIFTY_SIXTH_VALUE => Duration::new(
        DurationType::TwoHundredFiftySixth,
        Self::dots_from_remainder(TWO_HUNDRED_FIFTY_SIXTH_VALUE, v),
      ),
      v if v >= FIVE_HUNDRED_TWELFTH_VALUE => Duration::new(
        DurationType::FiveHundredTwelfth,
        Self::dots_from_remainder(FIVE_HUNDRED_TWELFTH_VALUE, v),
      ),
      v if v >= ONE_THOUSAND_TWENTY_FOURTH_VALUE => Duration::new(
        DurationType::OneThousandTwentyFourth,
        Self::dots_from_remainder(ONE_THOUSAND_TWENTY_FOURTH_VALUE, v),
      ),
      v => Duration::new(
        DurationType::TwoThousandFortyEighth,
        Self::dots_from_remainder(TWO_THOUSAND_FOURTH_EIGHTH_VALUE, v),
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
      beats if beats.fract() < ONE_THOUSAND_TWENTY_FOURTH_VALUE => (DurationType::Whole, beats as u32),
      beats if beats.fract() >= HALF_VALUE => (DurationType::Half, (2.0 * beats) as u32),
      beats if beats.fract() >= QUARTER_VALUE => (DurationType::Quarter, (4.0 * beats) as u32),
      beats if beats.fract() >= EIGHTH_VALUE => (DurationType::Eighth, (8.0 * beats) as u32),
      beats if beats.fract() >= SIXTEENTH_VALUE => (DurationType::Sixteenth, (16.0 * beats) as u32),
      beats if beats.fract() >= THIRTY_SECOND_VALUE => (DurationType::ThirtySecond, (32.0 * beats) as u32),
      beats if beats.fract() >= SIXTY_FOURTH_VALUE => (DurationType::SixtyFourth, (64.0 * beats) as u32),
      beats if beats.fract() >= ONE_HUNDRED_TWENTY_EIGHTH_VALUE => {
        (DurationType::OneHundredTwentyEighth, (128.0 * beats) as u32)
      }
      beats if beats.fract() >= TWO_HUNDRED_FIFTY_SIXTH_VALUE => {
        (DurationType::TwoHundredFiftySixth, (256.0 * beats) as u32)
      }
      beats if beats.fract() >= FIVE_HUNDRED_TWELFTH_VALUE => {
        (DurationType::FiveHundredTwelfth, (512.0 * beats) as u32)
      }
      beats if beats.fract() >= ONE_THOUSAND_TWENTY_FOURTH_VALUE => {
        (DurationType::OneThousandTwentyFourth, (1024.0 * beats) as u32)
      }
      _ => (DurationType::TwoThousandFortyEighth, (2048.0 * beats) as u32),
    }
  }

  #[must_use]
  pub fn value(&self) -> f64 {
    let base_duration = match self.value {
      DurationType::Maxima => MAXIMA_VALUE,
      DurationType::Long => LONG_VALUE,
      DurationType::Breve => BREVE_VALUE,
      DurationType::Whole => WHOLE_VALUE,
      DurationType::Half => HALF_VALUE,
      DurationType::Quarter => QUARTER_VALUE,
      DurationType::Eighth => EIGHTH_VALUE,
      DurationType::Sixteenth => SIXTEENTH_VALUE,
      DurationType::ThirtySecond => THIRTY_SECOND_VALUE,
      DurationType::SixtyFourth => SIXTY_FOURTH_VALUE,
      DurationType::OneHundredTwentyEighth => ONE_HUNDRED_TWENTY_EIGHTH_VALUE,
      DurationType::TwoHundredFiftySixth => TWO_HUNDRED_FIFTY_SIXTH_VALUE,
      DurationType::FiveHundredTwelfth => FIVE_HUNDRED_TWELFTH_VALUE,
      DurationType::OneThousandTwentyFourth => ONE_THOUSAND_TWENTY_FOURTH_VALUE,
      DurationType::TwoThousandFortyEighth => TWO_THOUSAND_FOURTH_EIGHTH_VALUE,
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
