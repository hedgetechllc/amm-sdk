use super::staff::Staff;
use crate::context::{generate_id, Clef, Key, TimeSignature};
use crate::modification::{SectionModification, SectionModificationType};
use std::{cell::RefCell, rc::Rc, slice::Iter};

pub enum SectionContent {
  Staff(Rc<RefCell<Staff>>),
  Section(Rc<RefCell<Section>>),
}

pub struct Section {
  id: usize,
  name: String,
  content: Vec<SectionContent>,
  modifications: Vec<Rc<RefCell<SectionModification>>>,
}

impl Section {
  pub fn new(name: &str) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: generate_id(),
      name: String::from(name),
      content: Vec::new(),
      modifications: Vec::new(),
    }))
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

  pub fn add_staff(
    &mut self,
    name: &str,
    clef: Option<Clef>,
    key_signature: Option<Key>,
    time_signature: Option<TimeSignature>,
  ) -> Rc<RefCell<Staff>> {
    let staff = Staff::new(name, clef, key_signature, time_signature);
    self.content.push(SectionContent::Staff(Rc::clone(&staff)));
    staff
  }

  pub fn add_section(&mut self, name: &str) -> Rc<RefCell<Section>> {
    let section = Section::new(name);
    self.content.push(SectionContent::Section(Rc::clone(&section)));
    section
  }

  pub fn add_modification(&mut self, modification: SectionModificationType) -> Rc<RefCell<SectionModification>> {
    let modification = SectionModification::new(modification);
    self.modifications.push(Rc::clone(&modification));
    modification
  }

  pub fn insert_staff(
    &mut self,
    index: usize,
    name: &str,
    clef: Option<Clef>,
    key_signature: Option<Key>,
    time_signature: Option<TimeSignature>,
  ) -> Rc<RefCell<Staff>> {
    let staff = Staff::new(name, clef, key_signature, time_signature);
    self.content.insert(index, SectionContent::Staff(Rc::clone(&staff)));
    staff
  }

  pub fn insert_section(&mut self, index: usize, name: &str) -> Rc<RefCell<Section>> {
    let section = Section::new(name);
    self.content.insert(index, SectionContent::Section(Rc::clone(&section)));
    section
  }

  pub fn get_staff_names(&self) -> Vec<String> {
    self
      .content
      .iter()
      .filter_map(|item| match item {
        SectionContent::Staff(staff) => Some(String::from(staff.borrow().get_name())),
        _ => None,
      })
      .collect()
  }

  pub fn get_section_names(&self) -> Vec<String> {
    self
      .content
      .iter()
      .filter_map(|item| match item {
        SectionContent::Section(section) => Some(String::from(section.borrow().get_name())),
        _ => None,
      })
      .collect()
  }

  pub fn get_staff_by_id(&mut self, id: usize) -> Option<Rc<RefCell<Staff>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) if staff.borrow().get_id() == id => Some(Rc::clone(staff)),
      _ => None,
    })
  }

  pub fn get_staff_by_name(&mut self, name: &str) -> Option<Rc<RefCell<Staff>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) if staff.borrow().get_name() == name => Some(Rc::clone(staff)),
      _ => None,
    })
  }

  pub fn get_section_by_id(&mut self, id: usize) -> Option<Rc<RefCell<Section>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Section(section) if section.borrow().get_id() == id => Some(Rc::clone(section)),
      _ => None,
    })
  }

  pub fn get_section_by_name(&mut self, name: &str) -> Option<Rc<RefCell<Section>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Section(section) if section.borrow().get_name() == name => Some(Rc::clone(section)),
      _ => None,
    })
  }

  pub fn get_modification(&mut self, id: usize) -> Option<Rc<RefCell<SectionModification>>> {
    self.modifications.iter().find_map(|modification| {
      if modification.borrow().get_id() == id {
        Some(Rc::clone(modification))
      } else {
        None
      }
    })
  }

  pub fn remove_item_by_id(&mut self, id: usize) -> &mut Self {
    self.content.retain(|item| match item {
      SectionContent::Staff(staff) => staff.borrow().get_id() != id,
      SectionContent::Section(section) => section.borrow().get_id() != id,
    });
    self
  }

  pub fn remove_item_by_name(&mut self, name: &str) -> &mut Self {
    self.content.retain(|item| match item {
      SectionContent::Staff(staff) => staff.borrow().get_name() != name,
      SectionContent::Section(section) => section.borrow().get_name() != name,
    });
    self
  }

  pub fn remove_modification(&mut self, id: usize) -> &mut Self {
    self
      .modifications
      .retain(|modification| modification.borrow().get_id() != id);
    self
  }

  pub fn iter(&self) -> Iter<'_, SectionContent> {
    self.content.iter()
  }
}

impl std::fmt::Display for Section {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let items = self
      .content
      .iter()
      .map(|item| match item {
        SectionContent::Staff(staff) => staff.borrow().to_string(),
        SectionContent::Section(section) => section.borrow().to_string(),
      })
      .collect::<Vec<_>>()
      .join(", ");
    write!(f, "Section: [{}]", items)
  }
}

impl<'a> IntoIterator for &'a Section {
  type Item = <Iter<'a, SectionContent> as Iterator>::Item;
  type IntoIter = Iter<'a, SectionContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.content.as_slice().into_iter()
  }
}
