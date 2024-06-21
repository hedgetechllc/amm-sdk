mod musical;
mod notational;

pub use musical::MusicalSlice;
pub use notational::{NotationalItem, NotationalSlice};

pub enum Slice {
  Musical(MusicalSlice),
  Notational(NotationalSlice),
}

impl Slice {
  pub fn musical() -> Self {
    Self::Musical(MusicalSlice::default())
  }

  pub fn notational() -> Self {
    Self::Notational(NotationalSlice::default())
  }

  pub fn is_musical(&self) -> bool {
    match *self {
      Slice::Musical(_) => true,
      _ => false,
    }
  }

  pub fn is_notational(&self) -> bool {
    match *self {
      Slice::Notational(_) => true,
      _ => false,
    }
  }

  pub fn to_musical(&mut self) -> Option<&mut MusicalSlice> {
    match *self {
      Slice::Musical(ref mut slice) => Some(slice),
      _ => None,
    }
  }

  pub fn to_notational(&mut self) -> Option<&mut NotationalSlice> {
    match *self {
      Slice::Notational(ref mut slice) => Some(slice),
      _ => None,
    }
  }

  pub fn duration(&self) -> f64 {
    match *self {
      Slice::Musical(ref slice) => slice.duration(),
      Slice::Notational(ref slice) => slice.duration(),
    }
  }
}

impl std::fmt::Display for Slice {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match *self {
        Slice::Musical(ref slice) => slice.to_string(),
        Slice::Notational(ref slice) => slice.to_string(),
      }
    )
  }
}
