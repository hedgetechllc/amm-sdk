#[derive(Clone, Copy, Eq, PartialEq)]
pub enum SliceModification {
  Arpeggiate,
}

impl std::fmt::Display for SliceModification {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match *self {
        SliceModification::Arpeggiate => "Arpeggiate",
      }
    )
  }
}
