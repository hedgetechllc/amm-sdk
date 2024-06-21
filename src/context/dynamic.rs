#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum DynamicMarking {
  #[default]
  None,
  Forte(u8),
  MezzoForte,
  MezzoPiano,
  Piano(u8),
}

impl DynamicMarking {
  pub fn value(&self) -> f32 {
    match *self {
      DynamicMarking::Piano(degree) => 0.05_f32.max(0.5_f32 - (0.1_f32 * f32::from(degree))),
      DynamicMarking::MezzoPiano => 0.45,
      DynamicMarking::MezzoForte => 0.55,
      DynamicMarking::Forte(degree) => 1.0_f32.min(0.5_f32 + (0.1_f32 * f32::from(degree))),
      _ => 0.5,
    }
  }
}

impl std::fmt::Display for DynamicMarking {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match *self {
        DynamicMarking::Piano(degree) => format!("{:p<1$}", "", usize::from(degree)),
        DynamicMarking::MezzoPiano => format!("mp"),
        DynamicMarking::MezzoForte => format!("mf"),
        DynamicMarking::Forte(degree) => format!("{:f<1$}", "", usize::from(degree)),
        _ => format!(""),
      }
    )
  }
}
