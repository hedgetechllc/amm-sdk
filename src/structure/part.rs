use super::{
  chord::Chord, multivoice::MultiVoice, phrase::Phrase, section::Section, staff::Staff, timeslice::Timeslice,
};
use crate::context::{generate_id, Tempo};
use crate::note::{Duration, Note};
use alloc::{
  collections::{BTreeMap, BTreeSet},
  rc::Rc,
  string::{String, ToString},
  vec::Vec,
};
use core::{cell::RefCell, slice::Iter};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub enum PartContent {
  Section(Rc<RefCell<Section>>),
}

#[derive(Clone)]
pub struct Part {
  pub(crate) id: usize,
  pub(crate) name: String,
  pub(crate) content: Vec<PartContent>,
}

impl Part {
  #[must_use]
  pub fn new(name: &str) -> Self {
    Self {
      id: generate_id(),
      name: String::from(name),
      content: Vec::new(),
    }
  }

  #[must_use]
  pub fn flatten(&self) -> Self {
    Self {
      id: self.id,
      name: self.name.clone(),
      content: self
        .content
        .iter()
        .map(|item| match item {
          PartContent::Section(section) => PartContent::Section(section.borrow().flatten()),
        })
        .collect(),
    }
  }

  #[must_use]
  pub fn extract_staves_as_parts(&self) -> Vec<Self> {
    let mut staff_parts: BTreeMap<String, Self> = self
      .get_staff_names()
      .iter()
      .map(|staff_name| {
        (
          String::from(staff_name),
          Self::new((self.name.clone() + "_" + staff_name).as_str()),
        )
      })
      .collect();
    self.content.iter().for_each(|item| match item {
      PartContent::Section(section) => {
        staff_parts.iter_mut().for_each(|(staff_name, part)| {
          part
            .content
            .push(PartContent::Section(section.borrow().single_staff_clone(staff_name)))
        });
      }
    });
    staff_parts.into_iter().map(|(_, part)| part).collect()
  }

  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }

  #[must_use]
  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn rename(&mut self, name: &str) -> &mut Self {
    self.name = String::from(name);
    self
  }

  pub fn add_section(&mut self, name: &str) -> Rc<RefCell<Section>> {
    if let Some(section) = self.get_section_by_name(name) {
      section
    } else {
      unsafe {
        self.content.push(PartContent::Section(Section::new(name)));
        self.get_section_by_name(name).unwrap_unchecked()
      }
    }
  }

  pub fn add_default_section(&mut self) -> Rc<RefCell<Section>> {
    self.add_section("default")
  }

  #[must_use]
  pub fn get_section_names(&self, recurse: bool) -> Vec<String> {
    // Section names are not necessarily unique when nested, so using `recurse` might generate misleading results
    // It is recommended to directly iterate over the sections themselves instead
    let mut section_names = BTreeSet::new();
    self.content.iter().for_each(|item| match item {
      PartContent::Section(section) => {
        section_names.insert(String::from(section.borrow().get_name()));
        if recurse {
          section.borrow().get_section_names(true).into_iter().for_each(|name| {
            section_names.insert(name);
          });
        }
      }
    });
    section_names.into_iter().collect()
  }

  #[must_use]
  pub fn get_staff_names(&self) -> Vec<String> {
    self
      .content
      .iter()
      .flat_map(|item| match item {
        PartContent::Section(section) => section.borrow().get_staff_names(true),
      })
      .collect::<BTreeSet<String>>()
      .into_iter()
      .collect()
  }

  #[must_use]
  pub fn get_section(&mut self, id: usize) -> Option<Rc<RefCell<Section>>> {
    self.content.iter().find_map(|item| match item {
      PartContent::Section(section) if section.borrow().get_id() == id => Some(Rc::clone(section)),
      PartContent::Section(section) => section.borrow_mut().get_section(id),
    })
  }

  #[must_use]
  pub fn get_section_by_name(&mut self, name: &str) -> Option<Rc<RefCell<Section>>> {
    self.content.iter().find_map(|item| match item {
      PartContent::Section(section) if section.borrow().get_name() == name => Some(Rc::clone(section)),
      PartContent::Section(_) => None,
    })
  }

  #[must_use]
  pub fn get_default_section(&mut self) -> Option<Rc<RefCell<Section>>> {
    self.get_section_by_name("default")
  }

  #[must_use]
  pub fn get_chord(&mut self, id: usize) -> Option<Rc<RefCell<Chord>>> {
    self.content.iter().find_map(|item| match item {
      PartContent::Section(section) => section.borrow_mut().get_chord(id),
    })
  }

  #[must_use]
  pub fn get_multivoice(&mut self, id: usize) -> Option<Rc<RefCell<MultiVoice>>> {
    self.content.iter().find_map(|item| match item {
      PartContent::Section(section) => section.borrow_mut().get_multivoice(id),
    })
  }

  #[must_use]
  pub fn get_note(&mut self, id: usize) -> Option<Rc<RefCell<Note>>> {
    self.content.iter().find_map(|item| match item {
      PartContent::Section(section) => section.borrow_mut().get_note(id),
    })
  }

  #[must_use]
  pub fn get_phrase(&mut self, id: usize) -> Option<Rc<RefCell<Phrase>>> {
    self.content.iter().find_map(|item| match item {
      PartContent::Section(section) => section.borrow_mut().get_phrase(id),
    })
  }

  #[must_use]
  pub fn get_staff(&mut self, id: usize) -> Option<Rc<RefCell<Staff>>> {
    self.content.iter().find_map(|item| match item {
      PartContent::Section(section) => section.borrow_mut().get_staff(id),
    })
  }

  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration) -> f64 {
    self
      .content
      .iter()
      .map(|content| match &content {
        PartContent::Section(section) => section.borrow().get_beats(beat_base),
      })
      .sum()
  }

  #[must_use]
  pub fn get_duration(&self, tempo: &Tempo) -> f64 {
    self.get_beats(&tempo.base_note) * 60.0 / f64::from(tempo.beats_per_minute)
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

  pub fn remove_item(&mut self, id: usize) -> &mut Self {
    self.content.retain(|item| match item {
      PartContent::Section(section) => section.borrow().get_id() != id,
    });
    self.content.iter().for_each(|item| match item {
      PartContent::Section(section) => {
        section.borrow_mut().remove_item(id);
      }
    });
    self
  }

  pub fn iter(&self) -> Iter<'_, PartContent> {
    self.content.iter()
  }

  #[must_use]
  pub fn iter_timeslices(&self) -> impl IntoIterator<Item = Timeslice> {
    // Note: use this to return timeslices for a single part
    let mut timeslices = Vec::new();
    self.content.iter().for_each(|item| match item {
      PartContent::Section(section) => {
        timeslices.append(&mut section.borrow().iter_timeslices());
      }
    });
    timeslices
  }
}

impl IntoIterator for Part {
  type Item = PartContent;
  type IntoIter = alloc::vec::IntoIter<Self::Item>;
  fn into_iter(self) -> Self::IntoIter {
    self.content.into_iter()
  }
}

impl<'a> IntoIterator for &'a Part {
  type Item = &'a PartContent;
  type IntoIter = Iter<'a, PartContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Part {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let sections = self
      .content
      .iter()
      .map(|item| match item {
        PartContent::Section(section) => section.borrow().to_string(),
      })
      .collect::<Vec<_>>()
      .join(", ");
    write!(f, "Part {}: [{sections}]", self.name)
  }
}
