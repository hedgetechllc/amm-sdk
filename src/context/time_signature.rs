#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimeSignature {
  pub numerator: u8,
  pub denominator: u8,
}

impl TimeSignature {
  pub fn new(numerator: u8, denominator: u8) -> Self {
    Self { numerator, denominator }
  }

  pub fn update(&mut self, numerator: u8, denominator: u8) {
    (self.numerator, self.denominator) = (numerator, denominator);
  }
}

impl Default for TimeSignature {
  fn default() -> Self {
    Self {
      numerator: 4,
      denominator: 4,
    }
  }
}

impl std::fmt::Display for TimeSignature {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}:{}", self.numerator, self.denominator)
  }
}
