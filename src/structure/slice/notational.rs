use crate::context::{Clef, Key, Tempo, TempoMarking, TimeSignature};

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum Barline {
  #[default]
  Single,
  Double,
  Final,
  Repeat,
  RepeatStart,
  RepeatEnd,
  RepeatEndStart,
}

impl std::fmt::Display for Barline {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match *self {
        Barline::Single => "|",
        Barline::Double => "||",
        Barline::Final => "|.",
        Barline::Repeat => "|:",
        Barline::RepeatStart => ":|",
        Barline::RepeatEnd => ":|:",
        Barline::RepeatEndStart => ":||:",
      }
    )
  }
}

#[derive(Clone, Eq, PartialEq)]
pub enum NotationalItem {
  Barline(Barline),
  BreathMark,
  Caesura,
  Clef(Clef),
  KeySignature(Key),
  TimeSignature(TimeSignature),
  TempoMarking(TempoMarking),
  TempoBpm(Tempo),
  WavyLine,
}

impl std::fmt::Display for NotationalItem {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        NotationalItem::Barline(line_type) => format!("Barline: {}", line_type),
        NotationalItem::BreathMark => format!("Breath Mark"),
        NotationalItem::Caesura => format!("Caesura"),
        NotationalItem::Clef(clef) => format!("Clef: {}", clef),
        NotationalItem::KeySignature(key) => format!("Key Signature: {}", key),
        NotationalItem::TimeSignature(time_sig) => format!("Time Signature: {}", time_sig),
        NotationalItem::TempoMarking(marking) => format!("Tempo: {}", marking),
        NotationalItem::TempoBpm(tempo_bpm) => format!("BPM: {}", tempo_bpm),
        NotationalItem::WavyLine => format!("Wavy Line"),
      }
    )
  }
}

#[derive(Clone, Default)]
pub struct NotationalSlice {
  items: Vec<NotationalItem>,
}

impl NotationalSlice {
  pub fn duration(&self) -> f64 {
    0.0
  }

  pub fn add(&mut self, item: NotationalItem) -> &mut Self {
    self.remove(&item).items.push(item);
    self
  }

  pub fn remove(&mut self, item: &NotationalItem) -> &mut Self {
    self
      .items
      .retain(|value| std::mem::discriminant(item) != std::mem::discriminant(value));
    self
  }
}

impl std::fmt::Display for NotationalSlice {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Notational Slice ({} items)", self.items.len())
  }
}
