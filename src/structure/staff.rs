use super::{MusicalSlice, NotationalItem, NotationalSlice, Slice};
use crate::context::{Clef, Key, TimeSignature};
use std::iter::IntoIterator;
use std::slice::Iter;

pub struct Staff {
  name: String,
  slices: Vec<Slice>,
}

impl Staff {
  pub fn new(name: &str, clef: Clef, starting_key: Key, starting_time_signature: TimeSignature) -> Self {
    let mut staff = Staff {
      name: String::from(name),
      slices: Vec::new(),
    };
    staff
      .add_notational_items()
      .add(NotationalItem::Clef(clef))
      .add(NotationalItem::KeySignature(starting_key))
      .add(NotationalItem::TimeSignature(starting_time_signature));
    staff
  }

  pub fn add_notational_items(&mut self) -> &mut NotationalSlice {
    if self.slices.is_empty() || !self.slices.last().unwrap().is_notational() {
      self.slices.push(Slice::notational());
    }
    self.slices.last_mut().unwrap().to_notational().unwrap()
  }

  pub fn add_musical_items(&mut self) -> &mut MusicalSlice {
    self.slices.push(Slice::musical());
    self.slices.last_mut().unwrap().to_musical().unwrap()
  }

  pub fn insert_notational_items(&mut self, index: usize) -> &mut NotationalSlice {
    let mut index = index;
    if index == 0 || !self.slices.get(index - 1).unwrap().is_notational() {
      self.slices.insert(index, Slice::notational());
    } else if index > 0 {
      index -= 1;
    }
    self.slices.get_mut(index).unwrap().to_notational().unwrap()
  }

  pub fn insert_musical_items(&mut self, index: usize) -> &mut MusicalSlice {
    self.slices.insert(index, Slice::musical());
    self.slices.get_mut(index).unwrap().to_musical().unwrap()
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_slice_mut(&mut self, index: usize) -> &mut Slice {
    self.slices.get_mut(index).unwrap()
  }

  pub fn get_slice(&self, index: usize) -> &Slice {
    self.slices.get(index).unwrap()
  }

  pub fn remove_slice(&mut self, index: usize) -> Slice {
    self.slices.remove(index)
  }

  pub fn iter(&self) -> Iter<'_, Slice> {
    self.slices.iter()
  }

  pub fn duration(&self) -> f64 {
    self.slices.iter().map(|slice| slice.duration()).sum()
  }
}

impl<'a> IntoIterator for &'a Staff {
  type Item = <Iter<'a, Slice> as Iterator>::Item;
  type IntoIter = Iter<'a, Slice>;
  fn into_iter(self) -> Self::IntoIter {
    self.slices.as_slice().into_iter()
  }
}
