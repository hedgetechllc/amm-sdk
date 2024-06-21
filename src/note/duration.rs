#[derive(Copy, Clone, Eq)]
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

impl PartialEq for Duration {
  fn eq(&self, other: &Self) -> bool {
    self.value() == other.value()
  }
}

impl Duration {
  pub fn value(&self) -> f64 {
    let (base_duration, dots) = match *self {
      Duration::Maxima(dots) => (8.0_f64, dots),
      Duration::Long(dots) => (4.0_f64, dots),
      Duration::Breve(dots) => (2.0_f64, dots),
      Duration::Whole(dots) => (1.0_f64, dots),
      Duration::Half(dots) => (0.5_f64, dots),
      Duration::Quarter(dots) => (0.25_f64, dots),
      Duration::Eighth(dots) => (0.125_f64, dots),
      Duration::Sixteenth(dots) => (0.0625_f64, dots),
      Duration::ThirtySecond(dots) => (0.03125_f64, dots),
      Duration::SixtyFourth(dots) => (0.015625_f64, dots),
      Duration::OneHundredTwentyEighth(dots) => (0.0078125_f64, dots),
      Duration::TwoHundredFiftySixth(dots) => (0.00390625_f64, dots),
      Duration::FiveHundredTwelfth(dots) => (0.001953125_f64, dots),
      Duration::OneThousandTwentyFourth(dots) => (0.0009765625_f64, dots),
      Duration::TwoThousandFortyEighth(dots) => (0.00048828125_f64, dots),
    };
    let mut duration = base_duration;
    for i in 0..dots {
      duration += base_duration / f64::from(2_u32 << i);
    }
    duration
  }

  pub fn beats(&self, base_beat_value: f64) -> f64 {
    self.value() / base_beat_value
  }
}

fn dots_to_text(dots: u8) -> String {
  if dots == 0 {
    String::from("")
  } else if dots == 1 {
    String::from("Dotted ")
  } else if dots == 2 {
    String::from("Double-Dotted ")
  } else if dots == 3 {
    String::from("Triple-Dotted ")
  } else if dots == 4 {
    String::from("Quadruple-Dotted ")
  } else {
    format!("{dots}-Dotted ")
  }
}

impl std::fmt::Display for Duration {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match *self {
        Duration::Maxima(dots) => format!("{}Maxima", dots_to_text(dots)),
        Duration::Long(dots) => format!("{}Long", dots_to_text(dots)),
        Duration::Breve(dots) => format!("{}Breve", dots_to_text(dots)),
        Duration::Whole(dots) => format!("{}Whole", dots_to_text(dots)),
        Duration::Half(dots) => format!("{}Half", dots_to_text(dots)),
        Duration::Quarter(dots) => format!("{}Quarter", dots_to_text(dots)),
        Duration::Eighth(dots) => format!("{}Eighth", dots_to_text(dots)),
        Duration::Sixteenth(dots) => format!("{}Sixteenth", dots_to_text(dots)),
        Duration::ThirtySecond(dots) => format!("{}Thirty-Second", dots_to_text(dots)),
        Duration::SixtyFourth(dots) => format!("{}Sixty-Fourth", dots_to_text(dots)),
        Duration::OneHundredTwentyEighth(dots) => format!("{}One-Hundred-Twenty-Eighth", dots_to_text(dots)),
        Duration::TwoHundredFiftySixth(dots) => format!("{}Two-Hundred-Fifty-Sixth", dots_to_text(dots)),
        Duration::FiveHundredTwelfth(dots) => format!("{}Five-Hundred-Twelfth", dots_to_text(dots)),
        Duration::OneThousandTwentyFourth(dots) => format!("{}One-Thousand-Twenty-Fourth", dots_to_text(dots)),
        Duration::TwoThousandFortyEighth(dots) => format!("{}Two-Thousand-Forty-Eighth", dots_to_text(dots)),
      }
    )
  }
}
