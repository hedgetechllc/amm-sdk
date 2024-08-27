use crate::context::Tempo;
use alloc::string::String;
#[cfg(target_arch = "wasm32")]
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Duration {
  Maxima(u8),
  Long(u8),
  Breve(u8),
  Whole(u8),
  Half(u8),
  Quarter(u8),
  Eighth(u8),
  Sixteenth(u8),
  ThirtySecond(u8),
  SixtyFourth(u8),
  OneHundredTwentyEighth(u8),
  TwoHundredFiftySixth(u8),
  FiveHundredTwelfth(u8),
  OneThousandTwentyFourth(u8),
  TwoThousandFortyEighth(u8),
}

impl Duration {
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
      v if v >= 8.0 => Duration::Maxima(Self::dots_from_remainder(8.0, v)),
      v if v >= 4.0 => Duration::Long(Self::dots_from_remainder(4.0, v)),
      v if v >= 2.0 => Duration::Breve(Self::dots_from_remainder(2.0, v)),
      v if v >= 1.0 => Duration::Whole(Self::dots_from_remainder(1.0, v)),
      v if v >= 0.5 => Duration::Half(Self::dots_from_remainder(0.5, v)),
      v if v >= 0.25 => Duration::Quarter(Self::dots_from_remainder(0.25, v)),
      v if v >= 0.125 => Duration::Eighth(Self::dots_from_remainder(0.125, v)),
      v if v >= 0.062_5 => Duration::Sixteenth(Self::dots_from_remainder(0.062_5, v)),
      v if v >= 0.031_25 => Duration::ThirtySecond(Self::dots_from_remainder(0.031_25, v)),
      v if v >= 0.015_625 => Duration::SixtyFourth(Self::dots_from_remainder(0.015_625, v)),
      v if v >= 0.007_812_5 => Duration::OneHundredTwentyEighth(Self::dots_from_remainder(0.007_812_5, v)),
      v if v >= 0.003_906_25 => Duration::TwoHundredFiftySixth(Self::dots_from_remainder(0.003_906_25, v)),
      v if v >= 0.001_953_125 => Duration::FiveHundredTwelfth(Self::dots_from_remainder(0.001_953_125, v)),
      v if v >= 0.000_976_562_5 => Duration::OneThousandTwentyFourth(Self::dots_from_remainder(0.000_976_562_5, v)),
      v => Duration::TwoThousandFortyEighth(Self::dots_from_remainder(0.000_488_281_25, v)),
    }
  }

  #[must_use]
  pub fn from_duration(tempo: &Tempo, duration: f64) -> Self {
    Duration::from_beats(&tempo.base_note, duration * f64::from(tempo.beats_per_minute) / 60.0)
  }

  #[must_use]
  pub fn get_minimum_divisible_notes(beats: f64) -> (Self, u32) {
    match beats {
      beats if beats.fract() < 0.000_976_562_5 => (Duration::Whole(0), beats as u32),
      beats if beats.fract() >= 0.5 => (Duration::Half(0), (2.0 * beats) as u32),
      beats if beats.fract() >= 0.25 => (Duration::Quarter(0), (4.0 * beats) as u32),
      beats if beats.fract() >= 0.125 => (Duration::Eighth(0), (8.0 * beats) as u32),
      beats if beats.fract() >= 0.062_5 => (Duration::Sixteenth(0), (16.0 * beats) as u32),
      beats if beats.fract() >= 0.031_25 => (Duration::ThirtySecond(0), (32.0 * beats) as u32),
      beats if beats.fract() >= 0.015_625 => (Duration::SixtyFourth(0), (64.0 * beats) as u32),
      beats if beats.fract() >= 0.007_812_5 => (Duration::OneHundredTwentyEighth(0), (128.0 * beats) as u32),
      beats if beats.fract() >= 0.003_906_25 => (Duration::TwoHundredFiftySixth(0), (256.0 * beats) as u32),
      beats if beats.fract() >= 0.001_953_125 => (Duration::FiveHundredTwelfth(0), (512.0 * beats) as u32),
      beats if beats.fract() >= 0.000_976_562_5 => (Duration::OneThousandTwentyFourth(0), (1024.0 * beats) as u32),
      _ => (Duration::TwoThousandFortyEighth(0), (2048.0 * beats) as u32),
    }
  }

  #[must_use]
  pub fn value(&self) -> f64 {
    let (base_duration, dots) = match self {
      Self::Maxima(dots) => (8.0, dots),
      Self::Long(dots) => (4.0, dots),
      Self::Breve(dots) => (2.0, dots),
      Self::Whole(dots) => (1.0, dots),
      Self::Half(dots) => (0.5, dots),
      Self::Quarter(dots) => (0.25, dots),
      Self::Eighth(dots) => (0.125, dots),
      Self::Sixteenth(dots) => (0.062_5, dots),
      Self::ThirtySecond(dots) => (0.031_25, dots),
      Self::SixtyFourth(dots) => (0.015_625, dots),
      Self::OneHundredTwentyEighth(dots) => (0.007_812_5, dots),
      Self::TwoHundredFiftySixth(dots) => (0.003_906_25, dots),
      Self::FiveHundredTwelfth(dots) => (0.001_953_125, dots),
      Self::OneThousandTwentyFourth(dots) => (0.000_976_562_5, dots),
      Self::TwoThousandFortyEighth(dots) => (0.000_488_281_25, dots),
    };
    (0..=*dots).map(|i| base_duration / f64::powi(2.0, i32::from(i))).sum()
  }

  #[must_use]
  pub fn beats(&self, base_beat_value: f64) -> f64 {
    self.value() / base_beat_value
  }

  #[must_use]
  pub fn split(&self, mut times: u8) -> Self {
    // Note: `times` must be a power of 2
    let mut duration = *self;
    while times > 1 {
      times /= 2;
      duration = match &duration {
        Self::Maxima(dots) => Self::Long(*dots),
        Self::Long(dots) => Self::Breve(*dots),
        Self::Breve(dots) => Self::Whole(*dots),
        Self::Whole(dots) => Self::Half(*dots),
        Self::Half(dots) => Self::Quarter(*dots),
        Self::Quarter(dots) => Self::Eighth(*dots),
        Self::Eighth(dots) => Self::Sixteenth(*dots),
        Self::Sixteenth(dots) => Self::ThirtySecond(*dots),
        Self::ThirtySecond(dots) => Self::SixtyFourth(*dots),
        Self::SixtyFourth(dots) => Self::OneHundredTwentyEighth(*dots),
        Self::OneHundredTwentyEighth(dots) => Self::TwoHundredFiftySixth(*dots),
        Self::TwoHundredFiftySixth(dots) => Self::FiveHundredTwelfth(*dots),
        Self::FiveHundredTwelfth(dots) => Self::OneThousandTwentyFourth(*dots),
        Self::OneThousandTwentyFourth(dots) | Self::TwoThousandFortyEighth(dots) => Self::TwoThousandFortyEighth(*dots),
      };
    }
    duration
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
      match *self {
        Self::Maxima(dots) => format!("{}Maxima", dots_to_text(dots)),
        Self::Long(dots) => format!("{}Long", dots_to_text(dots)),
        Self::Breve(dots) => format!("{}Breve", dots_to_text(dots)),
        Self::Whole(dots) => format!("{}Whole", dots_to_text(dots)),
        Self::Half(dots) => format!("{}Half", dots_to_text(dots)),
        Self::Quarter(dots) => format!("{}Quarter", dots_to_text(dots)),
        Self::Eighth(dots) => format!("{}Eighth", dots_to_text(dots)),
        Self::Sixteenth(dots) => format!("{}Sixteenth", dots_to_text(dots)),
        Self::ThirtySecond(dots) => format!("{}Thirty-Second", dots_to_text(dots)),
        Self::SixtyFourth(dots) => format!("{}Sixty-Fourth", dots_to_text(dots)),
        Self::OneHundredTwentyEighth(dots) => format!("{}One-Hundred-Twenty-Eighth", dots_to_text(dots)),
        Self::TwoHundredFiftySixth(dots) => format!("{}Two-Hundred-Fifty-Sixth", dots_to_text(dots)),
        Self::FiveHundredTwelfth(dots) => format!("{}Five-Hundred-Twelfth", dots_to_text(dots)),
        Self::OneThousandTwentyFourth(dots) => format!("{}One-Thousand-Twenty-Fourth", dots_to_text(dots)),
        Self::TwoThousandFortyEighth(dots) => format!("{}Two-Thousand-Forty-Eighth", dots_to_text(dots)),
      }
    )
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_value() {
    assert_eq!(Duration::Whole(3).value(), 1.875);
    assert_eq!(Duration::Quarter(4).value(), 0.484_375);
  }

  #[test]
  fn test_from_duration() {
    let tempo = Tempo::new(Duration::Quarter(0), 120);
    assert!(Duration::from_duration(&tempo, 0.5) == Duration::Quarter(0));
    assert!(Duration::from_duration(&tempo, 0.875) == Duration::Quarter(2));
    assert!(Duration::from_duration(&tempo, 1.0) == Duration::Half(0));
  }
}
