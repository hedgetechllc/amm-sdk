use super::{chord::Chord, multivoice::MultiVoice, phrase::Phrase, section::Section, staff::Staff};
use crate::context::{generate_id, Tempo};
use crate::note::{Duration, Note};
use crate::temporal::Timeslice;
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};

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

  pub(crate) fn simplify(&mut self) {
    self
      .content
      .iter_mut()
      .for_each(|PartContent::Section(section)| section.simplify());
  }

  #[must_use]
  pub fn flatten(&self) -> Self {
    Self {
      id: generate_id(),
      name: self.name.clone(),
      content: self
        .iter()
        .map(|PartContent::Section(section)| PartContent::Section(section.flatten()))
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
    self.iter().for_each(|PartContent::Section(section)| {
      for (staff_name, part) in &mut staff_parts {
        part
          .content
          .push(PartContent::Section(section.clone_with_single_staff(staff_name)));
      }
    });
    staff_parts.into_values().collect()
  }

  #[must_use]
  pub const fn get_id(&self) -> usize {
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
    self.content.push(PartContent::Section(Section::new(name)));
    match self.content.last_mut() {
      Some(PartContent::Section(section)) => section,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn claim(&mut self, item: PartContent) -> &mut Self {
    self.content.push(item);
    self
  }

  pub fn claim_section(&mut self, section: Section) -> &mut Section {
    self.content.push(PartContent::Section(section));
    match self.content.last_mut() {
      Some(PartContent::Section(section)) => section,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  #[must_use]
  pub fn get_section_names(&self, recurse: bool) -> Vec<String> {
    // Section names are not necessarily unique, so this function might generate misleading results
    // It is recommended to directly iterate over the sections themselves instead
    let mut section_names = BTreeSet::new();
    self.iter().for_each(|PartContent::Section(section)| {
      section_names.insert(String::from(section.get_name()));
      if recurse {
        section_names.extend(section.get_section_names(recurse));
      }
    });
    section_names.into_iter().collect()
  }

  #[must_use]
  pub fn get_staff_names(&self) -> Vec<String> {
    self
      .iter()
      .flat_map(|PartContent::Section(section)| section.get_staff_names(true))
      .collect::<BTreeSet<String>>()
      .into_iter()
      .collect()
  }

  #[must_use]
  pub fn get_section(&self, id: usize) -> Option<&Section> {
    self
      .iter()
      .find_map(|PartContent::Section(section)| section.get_section(id))
  }

  #[must_use]
  pub fn get_section_mut(&mut self, id: usize) -> Option<&mut Section> {
    self
      .iter_mut()
      .find_map(|PartContent::Section(section)| section.get_section_mut(id))
  }

  #[must_use]
  pub fn get_chord(&self, id: usize) -> Option<&Chord> {
    self
      .iter()
      .find_map(|PartContent::Section(section)| section.get_chord(id))
  }

  #[must_use]
  pub fn get_chord_mut(&mut self, id: usize) -> Option<&mut Chord> {
    self
      .iter_mut()
      .find_map(|PartContent::Section(section)| section.get_chord_mut(id))
  }

  #[must_use]
  pub fn get_multivoice(&self, id: usize) -> Option<&MultiVoice> {
    self
      .iter()
      .find_map(|PartContent::Section(section)| section.get_multivoice(id))
  }

  #[must_use]
  pub fn get_multivoice_mut(&mut self, id: usize) -> Option<&mut MultiVoice> {
    self
      .iter_mut()
      .find_map(|PartContent::Section(section)| section.get_multivoice_mut(id))
  }

  #[must_use]
  pub fn get_note(&self, id: usize) -> Option<&Note> {
    self
      .iter()
      .find_map(|PartContent::Section(section)| section.get_note(id))
  }

  #[must_use]
  pub fn get_note_mut(&mut self, id: usize) -> Option<&mut Note> {
    self
      .iter_mut()
      .find_map(|PartContent::Section(section)| section.get_note_mut(id))
  }

  #[must_use]
  pub fn get_phrase(&self, id: usize) -> Option<&Phrase> {
    self
      .iter()
      .find_map(|PartContent::Section(section)| section.get_phrase(id))
  }

  #[must_use]
  pub fn get_phrase_mut(&mut self, id: usize) -> Option<&mut Phrase> {
    self
      .iter_mut()
      .find_map(|PartContent::Section(section)| section.get_phrase_mut(id))
  }

  #[must_use]
  pub fn get_staff(&self, id: usize) -> Option<&Staff> {
    self
      .iter()
      .find_map(|PartContent::Section(section)| section.get_staff(id))
  }

  #[must_use]
  pub fn get_staff_mut(&mut self, id: usize) -> Option<&mut Staff> {
    self
      .iter_mut()
      .find_map(|PartContent::Section(section)| section.get_staff_mut(id))
  }

  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration) -> f64 {
    self
      .iter()
      .map(|PartContent::Section(section)| section.get_beats(beat_base))
      .sum()
  }

  #[must_use]
  pub fn get_duration(&self, tempo: &Tempo) -> f64 {
    self.get_beats(&tempo.base_note) * 60.0 / f64::from(tempo.beats_per_minute)
  }

  pub fn remove_section(&mut self, id: usize) -> &mut Self {
    self
      .content
      .retain(|PartContent::Section(section)| section.get_id() != id);
    self
  }

  pub fn remove_item(&mut self, id: usize) -> &mut Self {
    self
      .content
      .retain(|PartContent::Section(section)| section.get_id() != id);
    self.iter_mut().for_each(|PartContent::Section(section)| {
      section.remove_item(id);
    });
    self
  }

  pub fn remove_modification(&mut self, id: usize) -> &mut Self {
    self.iter_mut().for_each(|PartContent::Section(section)| {
      section.remove_modification(id);
    });
    self
  }

  #[must_use]
  pub fn num_timeslices(&self) -> usize {
    self
      .iter()
      .map(|PartContent::Section(section)| section.num_timeslices())
      .sum()
  }

  pub fn iter(&self) -> core::slice::Iter<'_, PartContent> {
    self.content.iter()
  }

  pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, PartContent> {
    self.content.iter_mut()
  }

  #[must_use]
  pub fn iter_timeslices(&self) -> impl core::iter::FusedIterator<Item = Timeslice> + '_ {
    // Note: use this to return timeslices for a single part
    self
      .iter()
      .flat_map(|PartContent::Section(section)| section.iter_timeslices())
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
  type IntoIter = core::slice::Iter<'a, PartContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut Part {
  type Item = &'a mut PartContent;
  type IntoIter = core::slice::IterMut<'a, PartContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
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
      .iter()
      .map(|PartContent::Section(section)| section.to_string())
      .collect::<Vec<_>>()
      .join(", ");
    write!(f, "Part {}: [{sections}]", self.name)
  }
}
