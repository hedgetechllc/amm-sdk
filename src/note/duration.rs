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
  pub fn value(&self) -> f64 {
    let (base, dots) = match self {
      Self::Maxima(dots) => (3, dots),
      Self::Long(dots) => (2, dots),
      Self::Breve(dots) => (1, dots),
      Self::Whole(dots) => (0, dots),
      Self::Half(dots) => (-1, dots),
      Self::Quarter(dots) => (-2, dots),
      Self::Eighth(dots) => (-3, dots),
      Self::Sixteenth(dots) => (-4, dots),
      Self::ThirtySecond(dots) => (-5, dots),
      Self::SixtyFourth(dots) => (-6, dots),
      Self::OneHundredTwentyEighth(dots) => (-7, dots),
      Self::TwoHundredFiftySixth(dots) => (-8, dots),
      Self::FiveHundredTwelfth(dots) => (-9, dots),
      Self::OneThousandTwentyFourth(dots) => (-10, dots),
      Self::TwoThousandFortyEighth(dots) => (-11, dots),
    };
    let base_duration = f64::powi(2.0, base);
    (0..=*dots).map(|i| base_duration / f64::powi(2.0, i as i32)).sum()
  }

  pub fn beats(&self, base_beat_value: f64) -> f64 {
    self.value() / base_beat_value
  }
}

fn dots_to_text(dots: u8) -> String {
  match dots {
    0 => String::new(),
    1 => String::from("Dotted "),
    2 => String::from("Double-Dotted "),
    _ => format!("{dots}-Dotted "),
  }
}

impl std::fmt::Display for Duration {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
