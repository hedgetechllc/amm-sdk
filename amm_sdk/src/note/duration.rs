use crate::context::Tempo;
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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

/// Represents the type of duration of a note.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum DurationType {
  /// ![Maxima Duration](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/note-type-maxima.png)
  Maxima,
  /// ![Long Duration](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/note-type-long.png)
  Long,
  /// ![Breve Duration](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/note-type-breve.png)
  Breve,
  /// ![Whole Duration](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/note-type-whole.png)
  Whole,
  /// ![Half Duration](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/note-type-half.png)
  Half,
  /// ![Quarter Duration](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/note-type-quarter.png)
  #[default]
  Quarter,
  /// ![8th Duration](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/note-type-eighth.png)
  Eighth,
  /// ![16th Duration](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/note-type-16th.png)
  Sixteenth,
  /// ![32nd Duration](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/note-type-32nd.png)
  ThirtySecond,
  /// ![64th Duration](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/note-type-64th.png)
  SixtyFourth,
  /// ![128th Duration](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/note-type-128th.png)
  OneHundredTwentyEighth,
  /// ![256th Duration](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/note-type-256th.png)
  TwoHundredFiftySixth,
  /// ![512th Duration](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/note-type-512th.png)
  FiveHundredTwelfth,
  /// ![1024th Duration](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/note-type-1024th.png)
  OneThousandTwentyFourth,
  /// ![2048th Duration](https://hedgetechllc.github.io/amm-sdk/amm_sdk/images/note-type-2048th.png)
  TwoThousandFortyEighth,
}

impl DurationType {
  /// Returns the value of the duration type as its fractional representation.
  #[must_use]
  pub const fn value(&self) -> f64 {
    match self {
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
    }
  }
}

/// Represents the duration of a note as a combination of note type and dots.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub struct Duration {
  /// The type of duration of the note.
  pub value: DurationType,
  /// The number of dots after the note.
  ///
  /// Each dot increases the duration of a note by half of its original value, compounded.
  ///
  /// For example, a quarter note with one dot is equivalent in length to a quarter note
  /// and an eighth note. A quarter note with two dots is equivalent to a quarter
  /// note, an eighth note, and a sixteenth note.
  pub dots: u8,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Duration {
  /// Creates a new [`Duration`] with the given note type and number of dots.
  #[must_use]
  pub const fn new(value: DurationType, dots: u8) -> Self {
    Self { value, dots }
  }

  /// Returns the number of dots required to represent the remainder of a note's value.
  #[must_use]
  fn dots_from_remainder(base_value: f64, full_value: f64) -> u8 {
    let (mut current_value, mut dots) = (base_value, 0);
    while full_value - current_value >= TWO_THOUSAND_FOURTH_EIGHTH_VALUE {
      dots += 1;
      current_value += base_value / f64::powi(2.0, i32::from(dots));
    }
    dots
  }

  /// Creates a new [`Duration`] from the given beat base value and number of beats.
  ///
  /// The `beat_base_value` defines the type of note that represents a single beat.
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

  /// Creates a new set of tied [`Duration`] values from the given beat base value
  /// and number of beats.
  ///
  /// The `beat_base_value` defines the type of note that represents a single beat.
  #[must_use]
  pub fn from_beats_tied(beat_base_value: &Duration, beats: f64) -> Vec<Self> {
    let mut tied_durations: Vec<Duration> = Vec::new();
    let mut beats_remaining = beats * beat_base_value.value();
    while beats_remaining >= TWO_THOUSAND_FOURTH_EIGHTH_VALUE {
      let duration = match beats_remaining {
        v if v >= MAXIMA_VALUE => Duration::new(DurationType::Maxima, 0),
        v if v >= LONG_VALUE => Duration::new(DurationType::Long, 0),
        v if v >= BREVE_VALUE => Duration::new(DurationType::Breve, 0),
        v if v >= WHOLE_VALUE => Duration::new(DurationType::Whole, 0),
        v if v >= HALF_VALUE => Duration::new(DurationType::Half, 0),
        v if v >= QUARTER_VALUE => Duration::new(DurationType::Quarter, 0),
        v if v >= EIGHTH_VALUE => Duration::new(DurationType::Eighth, 0),
        v if v >= SIXTEENTH_VALUE => Duration::new(DurationType::Sixteenth, 0),
        v if v >= THIRTY_SECOND_VALUE => Duration::new(DurationType::ThirtySecond, 0),
        v if v >= SIXTY_FOURTH_VALUE => Duration::new(DurationType::SixtyFourth, 0),
        v if v >= ONE_HUNDRED_TWENTY_EIGHTH_VALUE => Duration::new(DurationType::OneHundredTwentyEighth, 0),
        v if v >= TWO_HUNDRED_FIFTY_SIXTH_VALUE => Duration::new(DurationType::TwoHundredFiftySixth, 0),
        v if v >= FIVE_HUNDRED_TWELFTH_VALUE => Duration::new(DurationType::FiveHundredTwelfth, 0),
        v if v >= ONE_THOUSAND_TWENTY_FOURTH_VALUE => Duration::new(DurationType::OneThousandTwentyFourth, 0),
        _ => Duration::new(DurationType::TwoThousandFortyEighth, 0),
      };
      beats_remaining -= duration.value();
      if let Some(last_duration) = tied_durations.last_mut() {
        if (0.5 * last_duration.value() - duration.value()).abs() < f64::EPSILON {
          last_duration.dots += 1;
        } else {
          tied_durations.push(duration);
        }
      } else {
        tied_durations.push(duration);
      }
    }
    tied_durations
  }

  /// Creates a new [`Duration`] from the given tempo and note duration in seconds.
  #[must_use]
  pub fn from_duration(tempo: &Tempo, duration: f64) -> Self {
    Duration::from_beats(&tempo.base_note, duration * f64::from(tempo.beats_per_minute) / 60.0)
  }

  /// Returns the minimum number and type of notes that can be used to
  /// represent the specified number of beats.
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

  /// Returns the value of the duration as its fractional representation.
  #[must_use]
  pub fn value(&self) -> f64 {
    let base_duration = self.value.value();
    (0..=self.dots)
      .map(|i| base_duration / f64::powi(2.0, i32::from(i)))
      .sum()
  }

  /// Returns the number of beats that the duration represents.
  ///
  /// The `base_beat_value` parameter defines the type of note that represents a single beat.
  #[must_use]
  pub fn beats(&self, base_beat_value: f64) -> f64 {
    self.value() / base_beat_value
  }

  /// Splits the duration into `into_notes` number of notes.
  ///
  /// **Note:** The `into_notes` parameter **must** be a power of 2.
  #[must_use]
  pub fn split(&self, mut into_notes: u8) -> Self {
    // Note: `times` must be a power of 2
    let mut duration = self.value;
    while into_notes > 1 {
      into_notes /= 2;
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

/// Converts the number of dots in the duration into a textual representation.
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
    write!(f, "{}{}", dots_to_text(self.dots), self.value)
  }
}

#[cfg(test)]
mod test {
  use super::*;

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
}
