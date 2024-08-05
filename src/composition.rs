use crate::context::{Key, Tempo, TimeSignature};
use crate::note::Note;
use crate::structure::{Chord, MultiVoice, Part, Phrase, Section, Staff};
use alloc::{rc::Rc, string::String, vec::Vec};
use core::{cell::RefCell, slice::Iter};
use std::collections::HashMap;

pub struct Composition {
  title: String,
  copyright: Option<String>,
  publisher: Option<String>,
  composers: Vec<String>,
  lyricists: Vec<String>,
  arrangers: Vec<String>,
  metadata: HashMap<String, String>,
  parts: Vec<Part>,
  tempo: Tempo,
  starting_key: Key,
  starting_time_signature: TimeSignature,
}

impl Composition {
  pub fn new(title: &str, tempo: Option<Tempo>, key: Option<Key>, time_signature: Option<TimeSignature>) -> Self {
    Self {
      title: String::from(title),
      copyright: None,
      publisher: None,
      composers: Vec::new(),
      lyricists: Vec::new(),
      arrangers: Vec::new(),
      metadata: HashMap::new(),
      parts: Vec::new(),
      tempo: tempo.unwrap_or_default(),
      starting_key: key.unwrap_or_default(),
      starting_time_signature: time_signature.unwrap_or_default(),
    }
  }

  pub fn flatten(&self) -> Self {
    Self {
      title: self.title.clone(),
      copyright: self.copyright.clone(),
      publisher: self.publisher.clone(),
      composers: self.composers.clone(),
      lyricists: self.lyricists.clone(),
      arrangers: self.arrangers.clone(),
      metadata: self.metadata.clone(),
      parts: self.parts.iter().map(|part| part.flatten()).collect(),
      tempo: self.tempo.clone(),
      starting_key: self.starting_key.clone(),
      starting_time_signature: self.starting_time_signature.clone(),
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

  pub fn add_part(&mut self, name: &str) -> &mut Part {
    self.remove_part_by_name(name).parts.push(Part::new(name));
    self.parts.last_mut().unwrap()
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

  pub fn get_part_names(&self) -> Vec<String> {
    self.parts.iter().map(|part| String::from(part.get_name())).collect()
  }

  pub fn get_part_by_name(&mut self, name: &str) -> Option<&mut Part> {
    self.parts.iter_mut().find(|part| part.get_name() == name)
  }

  pub fn get_part(&mut self, id: usize) -> Option<&mut Part> {
    self.parts.iter_mut().find(|part| part.get_id() == id)
  }

  pub fn get_chord(&mut self, id: usize) -> Option<Rc<RefCell<Chord>>> {
    self.parts.iter_mut().find_map(|part| part.get_chord(id))
  }

  pub fn get_multivoice(&mut self, id: usize) -> Option<Rc<RefCell<MultiVoice>>> {
    self.parts.iter_mut().find_map(|part| part.get_multivoice(id))
  }

  pub fn get_note(&mut self, id: usize) -> Option<Rc<RefCell<Note>>> {
    self.parts.iter_mut().find_map(|part| part.get_note(id))
  }

  pub fn get_phrase(&mut self, id: usize) -> Option<Rc<RefCell<Phrase>>> {
    self.parts.iter_mut().find_map(|part| part.get_phrase(id))
  }

  pub fn get_section(&mut self, id: usize) -> Option<Rc<RefCell<Section>>> {
    self.parts.iter_mut().find_map(|part| part.get_section(id))
  }

  pub fn get_staff(&mut self, id: usize) -> Option<Rc<RefCell<Staff>>> {
    self.parts.iter_mut().find_map(|part| part.get_staff(id))
  }

  pub fn get_duration(&self) -> f64 {
    self
      .parts
      .iter()
      .map(|part| part.get_duration(&self.tempo))
      .reduce(f64::max)
      .unwrap_or_default()
  }

  pub fn remove_copyright(&mut self) -> &mut Self {
    self.copyright = None;
    self
  }

  pub fn remove_publisher(&mut self) -> &mut Self {
    self.publisher = None;
    self
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

  pub fn remove_part_by_name(&mut self, name: &str) -> &mut Self {
    self.parts.retain(|part| part.get_name() != name);
    self
  }

  pub fn remove_item(&mut self, id: usize) -> &mut Self {
    self.parts.retain(|part| part.get_id() != id);
    self.parts.iter_mut().for_each(|part| {
      part.remove_item(id);
    });
    self
  }

  pub fn iter(&self) -> Iter<'_, Part> {
    self.parts.iter()
  }
}

impl core::fmt::Display for Composition {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let duration = self.get_duration();
    write!(f, "Composition:\n  Title: {}\n  First Composer: {}\n  First Lyricist: {}\n  First Arranger: {}\n  Publisher: {}\n  Copyright: {}\n  Tempo: {}\n  Key: {}\n  Time Signature: {}\n  Num Parts: {}\n  Length: {:02}:{:02}",
      self.title,
      self.composers.first().unwrap_or(&String::from("Unknown")),
      self.lyricists.first().unwrap_or(&String::from("Unknown")),
      self.arrangers.first().unwrap_or(&String::from("Unknown")),
      self.publisher.as_deref().unwrap_or("Unknown"),
      self.copyright.as_deref().unwrap_or("None"),
      self.tempo,
      self.starting_key,
      self.starting_time_signature,
      self.parts.len(),
      duration as u32 / 60,
      duration as u32 % 60
    )
  }
}

impl<'a> IntoIterator for &'a Composition {
  type Item = <Iter<'a, Part> as Iterator>::Item;
  type IntoIter = Iter<'a, Part>;
  fn into_iter(self) -> Self::IntoIter {
    self.parts.as_slice().into_iter()
  }
}
