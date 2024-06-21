use super::Staff;
use crate::context::{Clef, Key, TimeSignature};

#[derive(Default)]
pub enum SystemIndicator {
  #[default]
  None,
  Brace,
  Bracket,
  Line,
  Square,
}

pub struct System {
  name: String,
  indicator: SystemIndicator,
  staves: Vec<Staff>,
}

impl System {
  pub fn new(name: &str, indicator: SystemIndicator) -> Self {
    System {
      name: String::from(name),
      indicator,
      staves: Vec::new(),
    }
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_indicator(&self) -> &SystemIndicator {
    &self.indicator
  }

  pub fn set_indicator(&mut self, indicator: SystemIndicator) -> &mut Self {
    self.indicator = indicator;
    self
  }

  pub fn add_staff(
    &mut self,
    name: &str,
    clef: Clef,
    starting_key: Key,
    starting_time_signature: TimeSignature,
  ) -> &mut Staff {
    self
      .remove_staff(name)
      .staves
      .push(Staff::new(name, clef, starting_key, starting_time_signature));
    self.staves.last_mut().unwrap()
  }

  pub fn get_staff(&mut self, name: &str) -> Option<&mut Staff> {
    self.staves.iter_mut().find(|staff| staff.get_name() == name)
  }

  pub fn get_staff_names(&self) -> Vec<String> {
    self.staves.iter().map(|staff| String::from(staff.get_name())).collect()
  }

  pub fn remove_staff(&mut self, name: &str) -> &mut Self {
    self.staves.retain(|staff| staff.get_name() != name);
    self
  }
}
