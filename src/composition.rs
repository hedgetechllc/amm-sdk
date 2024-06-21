use crate::context::{Clef, Key, Tempo, TimeSignature};
use crate::structure::{Staff, System, SystemIndicator};
use std::collections::HashMap;

pub struct Composition {
  title: String,
  copyright: Option<String>,
  publisher: Option<String>,
  composers: Vec<String>,
  lyricists: Vec<String>,
  arrangers: Vec<String>,
  metadata: HashMap<String, String>,
  systems: Vec<System>,
  tempo: Tempo,
  starting_key: Key,
  starting_time_signature: TimeSignature,
}

impl Composition {
  pub fn new(title: &str, tempo: Option<Tempo>, key: Option<Key>, time_signature: Option<TimeSignature>) -> Self {
    Composition {
      title: String::from(title),
      copyright: None,
      publisher: None,
      composers: Vec::new(),
      lyricists: Vec::new(),
      arrangers: Vec::new(),
      metadata: HashMap::new(),
      systems: Vec::new(),
      tempo: tempo.unwrap_or_default(),
      starting_key: key.unwrap_or_default(),
      starting_time_signature: time_signature.unwrap_or_default(),
    }
  }

  pub fn set_title(&mut self, title: &str) -> &mut Self {
    self.title = String::from(title);
    self
  }

  pub fn set_copyright(&mut self, copyright: &str) -> &mut Self {
    self.copyright = Some(String::from(copyright));
    self
  }

  pub fn set_publisher(&mut self, publisher: &str) -> &mut Self {
    self.publisher = Some(String::from(publisher));
    self
  }

  pub fn set_tempo(&mut self, tempo: Tempo) -> &mut Self {
    self.tempo = tempo;
    self
  }

  pub fn set_starting_key(&mut self, key: Key) -> &mut Self {
    self.starting_key = key;
    self
  }

  pub fn set_starting_time_signature(&mut self, time_signature: TimeSignature) -> &mut Self {
    self.starting_time_signature = time_signature;
    self
  }

  pub fn add_composer(&mut self, name: &str) -> &mut Self {
    self.composers.push(String::from(name));
    self
  }

  pub fn add_lyricist(&mut self, name: &str) -> &mut Self {
    self.lyricists.push(String::from(name));
    self
  }

  pub fn add_arranger(&mut self, name: &str) -> &mut Self {
    self.arrangers.push(String::from(name));
    self
  }

  pub fn add_metadata(&mut self, key: &str, value: &str) -> &mut Self {
    self.metadata.insert(String::from(key), String::from(value));
    self
  }

  pub fn add_system(&mut self, name: &str, system_grouping_indicator: Option<SystemIndicator>) -> &mut System {
    self
      .remove_system(name)
      .systems
      .push(System::new(name, system_grouping_indicator.unwrap_or_default()));
    self.systems.last_mut().unwrap()
  }

  pub fn set_system_grouping_indicator(
    &mut self,
    system_name: &str,
    system_grouping_indicator: SystemIndicator,
  ) -> &mut Self {
    if let Some(system) = self.systems.iter_mut().find(|system| system.get_name() == system_name) {
      system.set_indicator(system_grouping_indicator);
    }
    self
  }

  pub fn add_staff_to_system(&mut self, system_name: &str, staff_name: &str, clef: Option<Clef>) -> Option<&mut Staff> {
    match self.systems.iter_mut().find(|system| system.get_name() == system_name) {
      Some(system) => Some(system.add_staff(
        staff_name,
        clef.unwrap_or_default(),
        self.starting_key.clone(),
        self.starting_time_signature.clone(),
      )),
      None => None,
    }
  }

  pub fn get_title(&self) -> &str {
    &self.title
  }

  pub fn get_copyright(&self) -> &Option<String> {
    &self.copyright
  }

  pub fn get_publisher(&self) -> &Option<String> {
    &self.publisher
  }

  pub fn get_tempo(&self) -> &Tempo {
    &self.tempo
  }

  pub fn get_starting_key(&self) -> &Key {
    &self.starting_key
  }

  pub fn get_starting_time_signature(&self) -> &TimeSignature {
    &self.starting_time_signature
  }

  pub fn get_composers(&self) -> &Vec<String> {
    &self.composers
  }

  pub fn get_lyricists(&self) -> &Vec<String> {
    &self.lyricists
  }

  pub fn get_arrangers(&self) -> &Vec<String> {
    &self.arrangers
  }

  pub fn get_metadata(&self) -> &HashMap<String, String> {
    &self.metadata
  }

  pub fn get_system(&mut self, name: &str) -> Option<&mut System> {
    self.systems.iter_mut().find(|system| system.get_name() == name)
  }

  pub fn get_staff_from_system(&mut self, system_name: &str, staff_name: &str) -> Option<&mut Staff> {
    match self.systems.iter_mut().find(|system| system.get_name() == system_name) {
      Some(system) => system.get_staff(staff_name),
      None => None,
    }
  }

  pub fn remove_composer(&mut self, name: &str) -> &mut Self {
    self.composers.retain(|composer| composer != name);
    self
  }

  pub fn remove_lyricist(&mut self, name: &str) -> &mut Self {
    self.lyricists.retain(|lyricist| lyricist != name);
    self
  }

  pub fn remove_arranger(&mut self, name: &str) -> &mut Self {
    self.arrangers.retain(|arranger| arranger != name);
    self
  }

  pub fn remove_metadata(&mut self, key: &str) -> &mut Self {
    self.metadata.remove(key);
    self
  }

  pub fn remove_system(&mut self, name: &str) -> &mut Self {
    self.systems.retain(|system| system.get_name() != name);
    self
  }
}

impl std::fmt::Display for Composition {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Composition:\n  Title: {}\n  First Composer: {}\n  First Lyricist: {}\n  First Arranger: {}\n  Publisher: {}\n  Copyright: {}\n  Tempo: {}\n  Key: {}\n  Time Signature: {}\n  Num Systems: {}\n  Num Staffs: {}",
      self.title,
      self.composers.first().unwrap_or(&String::from("Unknown")),
      self.lyricists.first().unwrap_or(&String::from("Unknown")),
      self.arrangers.first().unwrap_or(&String::from("Unknown")),
      self.publisher.as_deref().unwrap_or("Unknown"),
      self.copyright.as_deref().unwrap_or("None"),
      self.tempo,
      self.starting_key,
      self.starting_time_signature,
      self.systems.len(),
      self.systems.iter().map(|system| system.get_staff_names().len()).sum::<usize>(),
    )
  }
}
