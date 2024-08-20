#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum TimeSignature {
  #[default]
  CommonTime,
  CutTime,
  Explicit(u8, u8),
  None,
}

impl core::fmt::Display for TimeSignature {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match *self {
      Self::CommonTime => write!(f, "Common Time"),
      Self::CutTime => write!(f, "Cut Time"),
      Self::Explicit(numerator, denominator) => write!(f, "{numerator}/{denominator}"),
      Self::None => write!(f, "Senza Misura"),
    }
  }
}
