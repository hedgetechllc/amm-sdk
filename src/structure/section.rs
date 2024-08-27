use super::{chord::Chord, multivoice::MultiVoice, phrase::Phrase, staff::Staff, timeslice::Timeslice};
use crate::context::{generate_id, Clef, Key, Tempo, TimeSignature};
use crate::modification::{SectionModification, SectionModificationType};
use crate::note::{Duration, Note, Pitch};
use alloc::{collections::BTreeSet, rc::Rc, string::String, vec::Vec};
use core::{cell::RefCell, slice::Iter};
#[cfg(target_arch = "wasm32")]
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", derive(Deserialize, Serialize))]
#[derive(Clone)]
pub enum SectionContent {
  Staff(Rc<RefCell<Staff>>),
  Section(Rc<RefCell<Section>>),
}

#[cfg_attr(target_arch = "wasm32", derive(Deserialize, Serialize))]
#[derive(Clone)]
pub struct Section {
  pub(crate) id: usize,
  pub(crate) name: String,
  pub(crate) content: Vec<SectionContent>,
  pub(crate) modifications: Vec<Rc<RefCell<SectionModification>>>,
}

impl Section {
  #[must_use]
  pub fn new(name: &str) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: generate_id(),
      name: String::from(name),
      content: Vec::new(),
      modifications: Vec::new(),
    }))
  }

  #[must_use]
  pub(crate) fn single_staff_clone(&self, retained_staff: &str) -> Rc<RefCell<Self>> {
    // Create a clone of this section with a new ID
    let mut clone = Self {
      id: generate_id(),
      name: self.name.clone(),
      content: Vec::new(),
      modifications: self.modifications.clone(),
    };

    // Create an implicit section for all naked staff groupings
    let mut sections: Vec<Rc<RefCell<Section>>> = Vec::new();
    let mut implicit_section = None;
    let (mut section_beats, beat_base_note) = (Vec::new(), Duration::Whole(0));
    for item in &self.content {
      unsafe {
        match item {
          SectionContent::Staff(staff) => {
            if implicit_section.is_none() {
              sections.push(Section::new("implicit"));
              implicit_section = Some(sections.last().unwrap_unchecked());
              section_beats.push(staff.borrow().get_beats(&beat_base_note));
            }
            if staff.borrow().get_name() == retained_staff {
              implicit_section
                .unwrap_unchecked()
                .borrow_mut()
                .content
                .push(SectionContent::Staff(Rc::clone(staff)));
            }
          }
          SectionContent::Section(section) => {
            section_beats.push(0.0);
            sections.push(section.borrow().single_staff_clone(retained_staff));
            implicit_section = None;
          }
        }
      }
    }

    // Ensure that all implicit sections contain at least one staff
    for (idx, section) in sections.into_iter().enumerate() {
      if section.borrow().name == "implicit" {
        if section.borrow().content.is_empty() {
          let implicit_staff = section.borrow_mut().add_staff(retained_staff, None, None, None);
          let (note_type, num_notes) = Duration::get_minimum_divisible_notes(section_beats[idx]);
          for _ in 0..num_notes {
            implicit_staff.borrow_mut().add_note(Pitch::Rest, note_type, None);
          }
        }
        clone
          .content
          .push(unsafe { section.borrow_mut().content.pop().unwrap_unchecked() });
      } else {
        clone.content.push(SectionContent::Section(section));
      }
    }
    Rc::new(RefCell::new(clone))
  }

  #[must_use]
  pub fn flatten(&self) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: self.id,
      name: self.name.clone(),
      content: self
        .content
        .iter()
        .map(|item| match item {
          SectionContent::Staff(staff) => SectionContent::Staff(staff.borrow().flatten()),
          SectionContent::Section(section) => SectionContent::Section(section.borrow().flatten()),
        })
        .collect(),
      modifications: self.modifications.clone(),
    }))
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
    self
      .modifications
      .retain(|mods| *mods.borrow().get_modification() != modification);
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

  #[must_use]
  pub fn get_staff_names(&self, recurse: bool) -> Vec<String> {
    let mut staff_names = BTreeSet::new();
    self.content.iter().for_each(|item| match item {
      SectionContent::Staff(staff) => {
        staff_names.insert(String::from(staff.borrow().get_name()));
      }
      SectionContent::Section(section) => {
        if recurse {
          section.borrow().get_staff_names(true).into_iter().for_each(|name| {
            staff_names.insert(name);
          });
        }
      }
    });
    staff_names.into_iter().collect()
  }

  #[must_use]
  pub fn get_section_names(&self, recurse: bool) -> Vec<String> {
    // Section names are not necessarily unique when nested, so using `recurse` might generate misleading results
    // It is recommended to directly iterate over the sections themselves instead
    let mut section_names = BTreeSet::new();
    self.content.iter().for_each(|item| match item {
      SectionContent::Section(section) => {
        section_names.insert(String::from(section.borrow().get_name()));
        if recurse {
          section.borrow().get_section_names(true).into_iter().for_each(|name| {
            section_names.insert(name);
          });
        }
      }
      SectionContent::Staff(_) => (),
    });
    section_names.into_iter().collect()
  }

  #[must_use]
  pub fn get_staff(&mut self, id: usize) -> Option<Rc<RefCell<Staff>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) if staff.borrow().get_id() == id => Some(Rc::clone(staff)),
      SectionContent::Section(section) => section.borrow_mut().get_staff(id),
      SectionContent::Staff(_) => None,
    })
  }

  #[must_use]
  pub fn get_staff_by_name(&mut self, name: &str) -> Option<Rc<RefCell<Staff>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) if staff.borrow().get_name() == name => Some(Rc::clone(staff)),
      SectionContent::Section(section) => section.borrow_mut().get_staff_by_name(name),
      SectionContent::Staff(_) => None,
    })
  }

  #[must_use]
  pub fn get_section(&mut self, id: usize) -> Option<Rc<RefCell<Section>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Section(section) if section.borrow().get_id() == id => Some(Rc::clone(section)),
      SectionContent::Section(section) => section.borrow_mut().get_section(id),
      SectionContent::Staff(_) => None,
    })
  }

  #[must_use]
  pub fn get_section_by_name(&mut self, name: &str) -> Option<Rc<RefCell<Section>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Section(section) if section.borrow().get_name() == name => Some(Rc::clone(section)),
      SectionContent::Section(section) => section.borrow_mut().get_section_by_name(name),
      SectionContent::Staff(_) => None,
    })
  }

  #[must_use]
  pub fn get_chord(&mut self, id: usize) -> Option<Rc<RefCell<Chord>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) => staff.borrow_mut().get_chord(id),
      SectionContent::Section(section) => section.borrow_mut().get_chord(id),
    })
  }

  #[must_use]
  pub fn get_multivoice(&mut self, id: usize) -> Option<Rc<RefCell<MultiVoice>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) => staff.borrow_mut().get_multivoice(id),
      SectionContent::Section(section) => section.borrow_mut().get_multivoice(id),
    })
  }

  #[must_use]
  pub fn get_note(&mut self, id: usize) -> Option<Rc<RefCell<Note>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) => staff.borrow_mut().get_note(id),
      SectionContent::Section(section) => section.borrow_mut().get_note(id),
    })
  }

  #[must_use]
  pub fn get_phrase(&mut self, id: usize) -> Option<Rc<RefCell<Phrase>>> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) => staff.borrow_mut().get_phrase(id),
      SectionContent::Section(section) => section.borrow_mut().get_phrase(id),
    })
  }

  #[must_use]
  pub fn get_modification(&mut self, id: usize) -> Option<Rc<RefCell<SectionModification>>> {
    self.modifications.iter().find_map(|modification| {
      if modification.borrow().get_id() == id {
        Some(Rc::clone(modification))
      } else {
        None
      }
    })
  }

  #[must_use]
  pub fn get_total_iterations(&self) -> u8 {
    self
      .modifications
      .iter()
      .find_map(|item| match item.borrow().get_modification() {
        SectionModificationType::Repeat { num_times } => Some(*num_times),
        _ => None,
      })
      .unwrap_or(1)
  }

  #[must_use]
  pub fn get_playable_iterations(&self) -> Vec<u8> {
    self
      .modifications
      .iter()
      .find_map(|item| match item.borrow().get_modification() {
        SectionModificationType::OnlyPlay { iterations } => Some(iterations.clone()),
        _ => None,
      })
      .unwrap_or_else(Vec::new)
  }

  #[must_use]
  pub fn get_section_tempo(&self) -> Option<Tempo> {
    self
      .modifications
      .iter()
      .find_map(|item| match item.borrow().get_modification() {
        SectionModificationType::TempoExplicit { tempo } => Some(*tempo),
        SectionModificationType::TempoImplicit { tempo } => Some(Tempo::new(Duration::Quarter(0), tempo.value())),
        _ => None,
      })
  }

  #[must_use]
  #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
  pub fn get_beats(&self, beat_base: &Duration) -> f64 {
    let section_beat_base = if let Some(tempo) = self.get_section_tempo() {
      tempo.base_note
    } else {
      *beat_base
    };
    let num_repeats = f64::from(self.get_total_iterations());
    let (mut beats, mut staff_found) = (0.0, false);
    for item in &self.content {
      match item {
        SectionContent::Staff(staff) => {
          // Staves should all have the same duration, so just return the first one
          if !staff_found {
            beats += staff.borrow().get_beats(&section_beat_base) * num_repeats;
            staff_found = true;
          }
        }
        SectionContent::Section(section) => {
          let num_iterations = match section.borrow().get_playable_iterations().len() {
            0 => num_repeats,
            count => count as f64,
          };
          beats += section.borrow().get_beats(&section_beat_base) * num_iterations;
          staff_found = false;
        }
      }
    }
    beats
  }

  #[must_use]
  #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
  pub fn get_duration(&self, tempo: &Tempo) -> f64 {
    let section_bpm = f64::from(if let Some(section_tempo) = self.get_section_tempo() {
      section_tempo.beats_per_minute
    } else {
      tempo.beats_per_minute
    });
    self.get_beats(&tempo.base_note) * 60.0 / section_bpm
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

  pub fn iter(&self) -> Iter<'_, SectionContent> {
    self.content.iter()
  }

  #[must_use]
  pub fn iter_timeslices(&self) -> Vec<Timeslice> {
    // Determine if this section contains sub-sections
    if self
      .content
      .iter()
      .any(|item| matches!(item, SectionContent::Section(_)))
    {
      // Create an implicit section for all naked staff groupings
      let mut timeslices = Vec::new();
      let mut sections: Vec<Rc<RefCell<Section>>> = Vec::new();
      let mut implicit_section = None;
      for item in &self.content {
        unsafe {
          match item {
            SectionContent::Staff(staff) => {
              if implicit_section.is_none() {
                sections.push(Section::new("implicit"));
                implicit_section = Some(sections.last().unwrap_unchecked());
              }
              implicit_section
                .unwrap_unchecked()
                .borrow_mut()
                .content
                .push(SectionContent::Staff(Rc::clone(staff)));
            }
            SectionContent::Section(section) => {
              sections.push(Rc::clone(section));
              implicit_section = None;
            }
          }
        }
      }

      // Iterate in order through all sub-sections the correct number of times
      for iteration in 1..=self.get_total_iterations() {
        for section in &sections {
          let section = section.borrow();
          if section.get_playable_iterations().is_empty() || section.get_playable_iterations().contains(&iteration) {
            timeslices.extend(section.iter_timeslices());
          }
        }
      }
      timeslices
    } else {
      // Iterate over all staves the correct number of times
      let beat_base_note = Duration::SixtyFourth(0);
      let mut timeslices: Vec<(f64, Timeslice)> = Vec::new();
      for _ in 1..=self.get_total_iterations() {
        for item in &self.content {
          match item {
            SectionContent::Staff(staff) => {
              let (mut index, mut curr_time) = (0, 0.0);
              for mut slice in staff.borrow().iter_timeslices() {
                let slice_duration = slice.get_beats(&beat_base_note);
                if let Some((mut slice_time, existing_slice)) = timeslices.get_mut(index) {
                  let mut existing_slice = existing_slice;
                  while (slice_time - curr_time).abs() > 0.000_001 && curr_time > slice_time {
                    index += 1;
                    (slice_time, existing_slice) = if let Some((start_time, slice)) = timeslices.get_mut(index) {
                      (*start_time, slice)
                    } else {
                      unsafe {
                        timeslices.push((curr_time, Timeslice::new()));
                        let (start_time, slice) = timeslices.last_mut().unwrap_unchecked();
                        (*start_time, slice)
                      }
                    };
                  }
                  if (slice_time - curr_time).abs() < 0.000_001 {
                    existing_slice.combine_with(&mut slice);
                  } else {
                    timeslices.insert(index, (curr_time, slice));
                  }
                } else {
                  timeslices.push((curr_time, slice));
                }
                curr_time += slice_duration;
                index += 1;
              }
            }
            SectionContent::Section(_) => unsafe { core::hint::unreachable_unchecked() },
          }
        }
      }
      timeslices.into_iter().map(|(_, slice)| slice).collect()
    }
  }
}

impl IntoIterator for Section {
  type Item = SectionContent;
  type IntoIter = alloc::vec::IntoIter<Self::Item>;
  fn into_iter(self) -> Self::IntoIter {
    self.content.into_iter()
  }
}

impl<'a> IntoIterator for &'a Section {
  type Item = &'a SectionContent;
  type IntoIter = Iter<'a, SectionContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Section {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let mods = self
      .modifications
      .iter()
      .map(|modification| modification.borrow().to_string())
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
      "Section{}: [{items}]",
      if mods.is_empty() {
        String::new()
      } else {
        format!(" ({mods})")
      }
    )
  }
}
