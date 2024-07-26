use super::section::Section;
use crate::context::{generate_id, Tempo};
use std::{cell::RefCell, rc::Rc, slice::Iter};

pub enum PartContent {
  Section(Rc<RefCell<Section>>),
}

pub struct Part {
  id: usize,
  name: String,
  content: Vec<PartContent>,
}

impl Part {
  pub fn new(name: &str) -> Self {
    Self {
      id: generate_id(),
      name: String::from(name),
      content: Vec::new(),
    }
  }

  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn rename(&mut self, name: &str) -> &mut Self {
    self.name = String::from(name);
    self
  }

  pub fn add_section(&mut self, name: &str) -> Rc<RefCell<Section>> {
    match self.get_section_by_name(name) {
      Some(section) => section,
      None => {
        self.content.push(PartContent::Section(Section::new(name)));
        self.get_section_by_name(name).unwrap()
      }
    }
  }

  pub fn add_default_section(&mut self) -> Rc<RefCell<Section>> {
    self.add_section("default")
  }

  pub fn get_section_names(&self) -> Vec<String> {
    self
      .content
      .iter()
      .filter_map(|item| match item {
        PartContent::Section(section) => Some(String::from(section.borrow().get_name())),
      })
      .collect()
  }

  pub fn get_section_by_id(&mut self, id: usize) -> Option<Rc<RefCell<Section>>> {
    self.content.iter().find_map(|item| match item {
      PartContent::Section(section) if section.borrow().get_id() == id => Some(Rc::clone(section)),
      _ => None,
    })
  }

  pub fn get_section_by_name(&mut self, name: &str) -> Option<Rc<RefCell<Section>>> {
    self.content.iter().find_map(|item| match item {
      PartContent::Section(section) if section.borrow().get_name() == name => Some(Rc::clone(section)),
      _ => None,
    })
  }

  pub fn get_default_section(&mut self) -> Option<Rc<RefCell<Section>>> {
    self.get_section_by_name("default")
  }

  pub fn remove_section_by_id(&mut self, id: usize) -> &mut Self {
    self.content.retain(|item| match item {
      PartContent::Section(section) => section.borrow().get_id() != id,
    });
    self
  }

  pub fn remove_section_by_name(&mut self, name: &str) -> &mut Self {
    self.content.retain(|item| match item {
      PartContent::Section(section) => section.borrow().get_name() != name,
    });
    self
  }

  pub fn remove_default_section(&mut self) -> &mut Self {
    self.remove_section_by_name("default")
  }

  pub fn get_duration(&self, tempo: &Tempo) -> f64 {
    self
      .content
      .iter()
      .map(|content| match &content {
        PartContent::Section(section) => section.borrow().get_duration(tempo),
      })
      .sum()
  }

  pub fn iter(&self) -> Iter<'_, PartContent> {
    self.content.iter()
  }
}

impl std::fmt::Display for Part {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let sections = self
      .content
      .iter()
      .map(|item| match item {
        PartContent::Section(section) => section.borrow().to_string(),
      })
      .collect::<Vec<_>>()
      .join(", ");
    write!(f, "Part {}: [{}]", self.name, sections)
  }
}

impl<'a> IntoIterator for &'a Part {
  type Item = <Iter<'a, PartContent> as Iterator>::Item;
  type IntoIter = Iter<'a, PartContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.content.as_slice().into_iter()
  }
}
