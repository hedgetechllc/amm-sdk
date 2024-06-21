pub type Beamed = bool;

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum Voice {
  #[default]
  Default,
  Lower,
  Upper,
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum Stem {
  #[default]
  Default,
  Up,
  Down,
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct DisplayOptions {
  pub voice: Voice,
  pub stem: Stem,
  pub beamed: Beamed,
}
