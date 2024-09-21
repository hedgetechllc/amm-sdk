use super::{
  chord::Chord, multivoice::MultiVoice, phrase::Phrase, section::Section, staff::Staff, timeslice::Timeslice,
};
use crate::context::{generate_id, Tempo};
use crate::note::{Duration, Note};
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};
use core::slice::Iter;

#[derive(Clone, Debug, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum PartContent {
  Section(Section),
}

#[derive(Debug, Default, Eq, JsonDeserialize, JsonSerialize)]
pub struct Part {
  id: usize,
  name: String,
  content: Vec<PartContent>,
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
      id: generate_id(),
      name: self.name.clone(),
      content: self
        .content
        .iter()
        .map(|item| match item {
          PartContent::Section(section) => PartContent::Section(section.flatten()),
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
        for (staff_name, part) in &mut staff_parts {
          part
            .content
            .push(PartContent::Section(section.clone_with_single_staff(staff_name)));
        }
      }
    });
    staff_parts.into_values().collect()
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

  pub fn add_section(&mut self, name: &str) -> &mut Section {
    if !self.content.iter().any(|item| match item {
      PartContent::Section(section) => section.get_name() == name,
    }) {
      self.content.push(PartContent::Section(Section::new(name)));
    }
    unsafe { self.get_section_mut_by_name(name).unwrap_unchecked() }
  }

  pub fn add_default_section(&mut self) -> &mut Section {
    self.add_section("default")
  }

  #[must_use]
  pub fn get_section_names(&self, recurse: bool) -> Vec<String> {
    // Section names are not necessarily unique when nested, so using `recurse` might generate misleading results
    // It is recommended to directly iterate over the sections themselves instead
    let mut section_names = BTreeSet::new();
    self.content.iter().for_each(|item| match item {
      PartContent::Section(section) => {
        section_names.insert(String::from(section.get_name()));
        if recurse {
          section_names.extend(section.get_section_names(recurse));
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
        PartContent::Section(section) => section.get_staff_names(true),
      })
      .collect::<BTreeSet<String>>()
      .into_iter()
      .collect()
  }

  #[must_use]
  pub fn get_section(&self, id: usize) -> Option<&Section> {
    self.content.iter().find_map(|item| match item {
      PartContent::Section(section) if section.get_id() == id => Some(section),
      PartContent::Section(section) => section.get_section(id),
    })
  }

  #[must_use]
  pub fn get_section_mut(&mut self, id: usize) -> Option<&mut Section> {
    self.content.iter_mut().find_map(|item| match item {
      PartContent::Section(section) => {
        if section.get_id() == id {
          Some(section)
        } else {
          section.get_section_mut(id)
        }
      }
    })
  }

  #[must_use]
  pub fn get_section_by_name(&self, name: &str) -> Option<&Section> {
    self.content.iter().find_map(|item| match item {
      PartContent::Section(section) if section.get_name() == name => Some(section),
      PartContent::Section(_) => None,
    })
  }

  #[must_use]
  pub fn get_section_mut_by_name(&mut self, name: &str) -> Option<&mut Section> {
    self.content.iter_mut().find_map(|item| match item {
      PartContent::Section(section) if section.get_name() == name => Some(section),
      PartContent::Section(_) => None,
    })
  }

  #[must_use]
  pub fn get_default_section(&self) -> Option<&Section> {
    self.get_section_by_name("default")
  }

  #[must_use]
  pub fn get_default_section_mut(&mut self) -> Option<&mut Section> {
    self.get_section_mut_by_name("default")
  }

  #[must_use]
  pub fn get_chord(&self, id: usize) -> Option<&Chord> {
    self.content.iter().find_map(|item| match item {
      PartContent::Section(section) => section.get_chord(id),
    })
  }

  #[must_use]
  pub fn get_chord_mut(&mut self, id: usize) -> Option<&mut Chord> {
    self.content.iter_mut().find_map(|item| match item {
      PartContent::Section(section) => section.get_chord_mut(id),
    })
  }

  #[must_use]
  pub fn get_multivoice(&self, id: usize) -> Option<&MultiVoice> {
    self.content.iter().find_map(|item| match item {
      PartContent::Section(section) => section.get_multivoice(id),
    })
  }

  #[must_use]
  pub fn get_multivoice_mut(&mut self, id: usize) -> Option<&mut MultiVoice> {
    self.content.iter_mut().find_map(|item| match item {
      PartContent::Section(section) => section.get_multivoice_mut(id),
    })
  }

  #[must_use]
  pub fn get_note(&self, id: usize) -> Option<&Note> {
    self.content.iter().find_map(|item| match item {
      PartContent::Section(section) => section.get_note(id),
    })
  }

  #[must_use]
  pub fn get_note_mut(&mut self, id: usize) -> Option<&mut Note> {
    self.content.iter_mut().find_map(|item| match item {
      PartContent::Section(section) => section.get_note_mut(id),
    })
  }

  #[must_use]
  pub fn get_phrase(&self, id: usize) -> Option<&Phrase> {
    self.content.iter().find_map(|item| match item {
      PartContent::Section(section) => section.get_phrase(id),
    })
  }

  #[must_use]
  pub fn get_phrase_mut(&mut self, id: usize) -> Option<&mut Phrase> {
    self.content.iter_mut().find_map(|item| match item {
      PartContent::Section(section) => section.get_phrase_mut(id),
    })
  }

  #[must_use]
  pub fn get_staff(&self, id: usize) -> Option<&Staff> {
    self.content.iter().find_map(|item| match item {
      PartContent::Section(section) => section.get_staff(id),
    })
  }

  #[must_use]
  pub fn get_staff_mut(&mut self, id: usize) -> Option<&mut Staff> {
    self.content.iter_mut().find_map(|item| match item {
      PartContent::Section(section) => section.get_staff_mut(id),
    })
  }

  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration) -> f64 {
    self
      .content
      .iter()
      .map(|content| match &content {
        PartContent::Section(section) => section.get_beats(beat_base),
      })
      .sum()
  }

  #[must_use]
  pub fn get_duration(&self, tempo: &Tempo) -> f64 {
    self.get_beats(&tempo.base_note) * 60.0 / f64::from(tempo.beats_per_minute)
  }

  pub fn remove_section_by_id(&mut self, id: usize) -> &mut Self {
    self.content.retain(|item| match item {
      PartContent::Section(section) => section.get_id() != id,
    });
    self
  }

  pub fn remove_section_by_name(&mut self, name: &str) -> &mut Self {
    self.content.retain(|item| match item {
      PartContent::Section(section) => section.get_name() != name,
    });
    self
  }

  pub fn remove_default_section(&mut self) -> &mut Self {
    self.remove_section_by_name("default")
  }

  pub fn remove_item(&mut self, id: usize) -> &mut Self {
    self.content.retain(|item| match item {
      PartContent::Section(section) => section.get_id() != id,
    });
    self.content.iter_mut().for_each(|item| match item {
      PartContent::Section(section) => {
        section.remove_item(id);
      }
    });
    self
  }

  #[must_use]
  pub fn num_timeslices(&self) -> usize {
    self
      .content
      .iter()
      .map(|item| match item {
        PartContent::Section(section) => section.num_timeslices(),
      })
      .sum()
  }

  pub fn iter(&self) -> Iter<'_, PartContent> {
    self.content.iter()
  }

  #[must_use]
  pub fn iter_timeslices(&self) -> Vec<Timeslice> {
    // Note: use this to return timeslices for a single part
    self
      .content
      .iter()
      .flat_map(|item| match item {
        PartContent::Section(section) => section.iter_timeslices(),
      })
      .collect()
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

impl Clone for Part {
  fn clone(&self) -> Self {
    Self {
      id: generate_id(),
      name: self.name.clone(),
      content: self.content.clone(),
    }
  }
}

impl PartialEq for Part {
  fn eq(&self, other: &Self) -> bool {
    self.content == other.content && self.name == other.name
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Part {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let sections = self
      .content
      .iter()
      .map(|item| match item {
        PartContent::Section(section) => section.to_string(),
      })
      .collect::<Vec<_>>()
      .join(", ");
    write!(f, "Part {}: [{sections}]", self.name)
  }
}
