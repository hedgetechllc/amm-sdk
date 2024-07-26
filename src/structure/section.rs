use super::{chord::Chord, multivoice::MultiVoice, note::Note, phrase::Phrase, staff::Staff};
use crate::context::{generate_id, Clef, Key, Tempo, TimeSignature};
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

  pub fn get_staff(&mut self, id: usize) -> Option<Rc<RefCell<Staff>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) if staff.borrow().get_id() == id => Some(Rc::clone(staff)),
      SectionContent::Section(section) => section.borrow_mut().get_staff(id),
      _ => None,
    })
  }

  pub fn get_staff_by_name(&mut self, name: &str) -> Option<Rc<RefCell<Staff>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) if staff.borrow().get_name() == name => Some(Rc::clone(staff)),
      _ => None,
    })
  }

  pub fn get_section(&mut self, id: usize) -> Option<Rc<RefCell<Section>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Section(section) if section.borrow().get_id() == id => Some(Rc::clone(section)),
      SectionContent::Section(section) => section.borrow_mut().get_section(id),
      _ => None,
    })
  }

  pub fn get_section_by_name(&mut self, name: &str) -> Option<Rc<RefCell<Section>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Section(section) if section.borrow().get_name() == name => Some(Rc::clone(section)),
      _ => None,
    })
  }

  pub fn get_chord(&mut self, id: usize) -> Option<Rc<RefCell<Chord>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) => staff.borrow_mut().get_chord(id),
      SectionContent::Section(section) => section.borrow_mut().get_chord(id),
    })
  }

  pub fn get_multivoice(&mut self, id: usize) -> Option<Rc<RefCell<MultiVoice>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) => staff.borrow_mut().get_multivoice(id),
      SectionContent::Section(section) => section.borrow_mut().get_multivoice(id),
    })
  }

  pub fn get_note(&mut self, id: usize) -> Option<Rc<RefCell<Note>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) => staff.borrow_mut().get_note(id),
      SectionContent::Section(section) => section.borrow_mut().get_note(id),
    })
  }

  pub fn get_phrase(&mut self, id: usize) -> Option<Rc<RefCell<Phrase>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) => staff.borrow_mut().get_phrase(id),
      SectionContent::Section(section) => section.borrow_mut().get_phrase(id),
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

  pub fn remove_item(&mut self, id: usize) -> &mut Self {
    self.content.retain(|item| match item {
      SectionContent::Staff(staff) => staff.borrow().get_id() != id,
      SectionContent::Section(section) => section.borrow().get_id() != id,
    });
    self.content.iter().for_each(|item| match item {
      SectionContent::Staff(staff) => {
        staff.borrow_mut().remove_item(id);
      }
      SectionContent::Section(section) => {
        section.borrow_mut().remove_item(id);
      }
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

  pub fn get_duration(&self, initial_tempo: &Tempo) -> f64 {
    // Determine if this section specifies a tempo modification
    let tempo = self
      .modifications
      .iter()
      .find_map(|item| match item.borrow().get_modification() {
        SectionModificationType::TempoExplicit { tempo } => Some(tempo.clone()),
        SectionModificationType::TempoImplicit { tempo } => Some(Tempo {
          base_note: initial_tempo.base_note,
          beats_per_minute: tempo.value(),
        }),
        _ => None,
      })
      .unwrap_or_else(|| initial_tempo.clone());

    // Calculate the duration of the section
    let (mut duration, mut max_staff_duration) = (0.0, 0.0);
    for content in &self.content {
      match &content {
        SectionContent::Section(section) => {
          duration += section.borrow().get_duration(&tempo);
          duration += max_staff_duration;
          max_staff_duration = 0.0;
        }
        SectionContent::Staff(staff) => {
          max_staff_duration = max_staff_duration.max(staff.borrow().get_duration(&tempo));
        }
      }
    }
    duration + max_staff_duration
  }

  pub fn iter(&self) -> Iter<'_, SectionContent> {
    self.content.iter()
  }
}

impl std::fmt::Display for Section {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mods = self
      .modifications
      .iter()
      .map(|modification| modification.borrow_mut().to_string())
      .collect::<Vec<String>>()
      .join(", ");
    let items = self
      .content
      .iter()
      .map(|item| match item {
        SectionContent::Staff(staff) => staff.borrow().to_string(),
        SectionContent::Section(section) => section.borrow().to_string(),
      })
      .collect::<Vec<_>>()
      .join(", ");
    write!(
      f,
      "Section{}: [{}]",
      if mods.is_empty() {
        String::new()
      } else {
        format!(" ({})", mods)
      },
      items
    )
  }
}

impl<'a> IntoIterator for &'a Section {
  type Item = <Iter<'a, SectionContent> as Iterator>::Item;
  type IntoIter = Iter<'a, SectionContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.content.as_slice().into_iter()
  }
}
