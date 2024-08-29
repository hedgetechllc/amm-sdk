use crate::context::Tempo;
use alloc::string::String;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DurationType {
  Maxima,
  Long,
  Breve,
  Whole,
  Half,
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Duration {
  pub r#type: DurationType,
  pub dots: u8,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Duration {
  #[must_use]
  pub fn new(r#type: DurationType, dots: u8) -> Self {
    Self { r#type, dots }
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
    let base_duration = match self.r#type {
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
    let mut duration = self.r#type;
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

#[cfg(feature = "print")]
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
impl core::fmt::Display for Duration {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self.r#type {
        DurationType::Maxima => format!("{}Maxima", dots_to_text(self.dots)),
        DurationType::Long => format!("{}Long", dots_to_text(self.dots)),
        DurationType::Breve => format!("{}Breve", dots_to_text(self.dots)),
        DurationType::Whole => format!("{}Whole", dots_to_text(self.dots)),
        DurationType::Half => format!("{}Half", dots_to_text(self.dots)),
        DurationType::Quarter => format!("{}Quarter", dots_to_text(self.dots)),
        DurationType::Eighth => format!("{}Eighth", dots_to_text(self.dots)),
        DurationType::Sixteenth => format!("{}Sixteenth", dots_to_text(self.dots)),
        DurationType::ThirtySecond => format!("{}Thirty-Second", dots_to_text(self.dots)),
        DurationType::SixtyFourth => format!("{}Sixty-Fourth", dots_to_text(self.dots)),
        DurationType::OneHundredTwentyEighth => format!("{}One-Hundred-Twenty-Eighth", dots_to_text(self.dots)),
        DurationType::TwoHundredFiftySixth => format!("{}Two-Hundred-Fifty-Sixth", dots_to_text(self.dots)),
        DurationType::FiveHundredTwelfth => format!("{}Five-Hundred-Twelfth", dots_to_text(self.dots)),
        DurationType::OneThousandTwentyFourth => format!("{}One-Thousand-Twenty-Fourth", dots_to_text(self.dots)),
        DurationType::TwoThousandFortyEighth => format!("{}Two-Thousand-Forty-Eighth", dots_to_text(self.dots)),
      }
    )
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
