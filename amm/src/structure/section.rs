use super::place_and_merge_timeslice;
use super::{chord::Chord, multivoice::MultiVoice, phrase::Phrase, staff::Staff, timeslice::Timeslice};
use crate::context::{generate_id, Tempo};
use crate::modification::{SectionModification, SectionModificationType};
use crate::note::{Duration, DurationType, Note, Pitch};
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};
use core::slice::{Iter, IterMut};

#[derive(Clone, Debug, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum SectionContent {
  Staff(Staff),
  Section(Section),
}

#[derive(Debug, Default, Eq, JsonDeserialize, JsonSerialize)]
pub struct Section {
  id: usize,
  name: String,
  content: Vec<SectionContent>,
  modifications: Vec<SectionModification>,
}

impl Section {
  #[must_use]
  pub fn new(name: &str) -> Self {
    Self {
      id: generate_id(),
      name: String::from(name),
      content: Vec::new(),
      modifications: Vec::new(),
    }
  }

  #[must_use]
  pub(crate) fn clone_with_single_staff(&self, retained_staff: &str) -> Self {
    // Create an implicit section for all naked staff groupings
    let mut sections = Vec::new();
    let mut implicit_section: Option<&mut (Section, f64, bool)> = None;
    let beat_base_note = Duration::new(DurationType::Whole, 0);
    for item in &self.content {
      match item {
        SectionContent::Staff(staff) => match implicit_section {
          Some((ref mut section, _, _)) => {
            if staff.get_name() == retained_staff {
              section.content.push(SectionContent::Staff(staff.clone()));
            }
          }
          None => {
            let mut section = Section::new("Implicit");
            if staff.get_name() == retained_staff {
              section.content.push(SectionContent::Staff(staff.clone()));
            }
            sections.push((section, staff.get_beats(&beat_base_note), true));
            implicit_section = sections.last_mut();
          }
        },
        SectionContent::Section(section) => {
          sections.push((section.clone_with_single_staff(retained_staff), 0.0, false));
          implicit_section = None;
        }
      }
    }

    // Create a clone of this section with a new ID
    Self {
      id: generate_id(),
      name: self.name.clone(),
      content:
        // Ensure that all implicit sections contain at least one staff
        sections.into_iter().map(|(mut section, beats, implicit)| {
          if implicit {
            match section.content.pop() {
              Some(content) => content,
              None => {
                let mut implicit_staff = Staff::new(retained_staff);
                let (note_type, num_notes) = Duration::get_minimum_divisible_notes(beats);
                for _ in 0..num_notes {
                  implicit_staff.add_note(Pitch::new_rest(), Duration::new(note_type, 0), None);
                }
                SectionContent::Staff(implicit_staff)
              }
            }
          } else {
            SectionContent::Section(section)
          }
        }).collect(),
      modifications: self.modifications.clone(),
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
          SectionContent::Staff(staff) => SectionContent::Staff(staff.flatten()),
          SectionContent::Section(section) => SectionContent::Section(section.flatten()),
        })
        .collect(),
      modifications: self.modifications.clone(),
    }
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

  pub fn add_staff(&mut self, name: &str) -> &mut Staff {
    self.content.retain(|item| match item {
      SectionContent::Staff(staff) => staff.get_name() != name,
      SectionContent::Section(_) => true,
    });
    self.content.push(SectionContent::Staff(Staff::new(name)));
    match self.content.last_mut() {
      Some(SectionContent::Staff(staff)) => staff,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn add_section(&mut self, name: &str) -> &mut Section {
    self.content.push(SectionContent::Section(Section::new(name)));
    match self.content.last_mut() {
      Some(SectionContent::Section(section)) => section,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn add_modification(&mut self, mod_type: SectionModificationType) -> &mut SectionModification {
    self.modifications.retain(|mods| mods.r#type != mod_type);
    self.modifications.push(SectionModification::new(mod_type));
    unsafe { self.modifications.last_mut().unwrap_unchecked() }
  }

  pub fn claim_staff(&mut self, staff: Staff) -> &mut Staff {
    self.content.retain(|item| match item {
      SectionContent::Staff(old_staff) => staff.get_name() != old_staff.get_name(),
      SectionContent::Section(_) => true,
    });
    self.content.push(SectionContent::Staff(staff));
    match self.content.last_mut() {
      Some(SectionContent::Staff(staff)) => staff,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn claim_section(&mut self, section: Section) -> &mut Section {
    self.content.push(SectionContent::Section(section));
    match self.content.last_mut() {
      Some(SectionContent::Section(section)) => section,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn insert_staff(&mut self, index: usize, name: &str) -> &mut Staff {
    self.content.insert(index, SectionContent::Staff(Staff::new(name)));
    match self.content.last_mut() {
      Some(SectionContent::Staff(staff)) => staff,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  pub fn insert_section(&mut self, index: usize, name: &str) -> &mut Section {
    self.content.insert(index, SectionContent::Section(Section::new(name)));
    match self.content.last_mut() {
      Some(SectionContent::Section(section)) => section,
      _ => unsafe { core::hint::unreachable_unchecked() },
    }
  }

  #[must_use]
  pub fn get_staff_names(&self, recurse: bool) -> Vec<String> {
    self
      .content
      .iter()
      .flat_map(|item| match item {
        SectionContent::Staff(staff) => Vec::from([String::from(staff.get_name())]),
        SectionContent::Section(section) => {
          if recurse {
            section.get_staff_names(recurse)
          } else {
            Vec::new()
          }
        }
      })
      .collect::<BTreeSet<String>>()
      .into_iter()
      .collect()
  }

  #[must_use]
  pub fn get_section_names(&self, recurse: bool) -> Vec<String> {
    // Section names are not necessarily unique when nested, so using `recurse` might generate misleading results
    // It is recommended to directly iterate over the sections themselves instead
    let mut section_names = BTreeSet::new();
    self.content.iter().for_each(|item| match item {
      SectionContent::Section(section) => {
        section_names.insert(String::from(section.get_name()));
        if recurse {
          section_names.extend(section.get_section_names(recurse));
        }
      }
      SectionContent::Staff(_) => (),
    });
    section_names.into_iter().collect()
  }

  #[must_use]
  pub fn get_staff(&self, id: usize) -> Option<&Staff> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) if staff.get_id() == id => Some(staff),
      SectionContent::Section(section) => section.get_staff(id),
      SectionContent::Staff(_) => None,
    })
  }

  #[must_use]
  pub fn get_staff_mut(&mut self, id: usize) -> Option<&mut Staff> {
    self.content.iter_mut().find_map(|item| match item {
      SectionContent::Staff(staff) if staff.get_id() == id => Some(staff),
      SectionContent::Section(section) => section.get_staff_mut(id),
      SectionContent::Staff(_) => None,
    })
  }

  #[must_use]
  pub fn get_staff_by_name(&self, name: &str) -> Option<&Staff> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) if staff.get_name() == name => Some(staff),
      SectionContent::Section(section) => section.get_staff_by_name(name),
      SectionContent::Staff(_) => None,
    })
  }

  #[must_use]
  pub fn get_staff_mut_by_name(&mut self, name: &str) -> Option<&mut Staff> {
    self.content.iter_mut().find_map(|item| match item {
      SectionContent::Staff(staff) if staff.get_name() == name => Some(staff),
      SectionContent::Section(section) => section.get_staff_mut_by_name(name),
      SectionContent::Staff(_) => None,
    })
  }

  #[must_use]
  pub fn get_section(&self, id: usize) -> Option<&Section> {
    if self.id == id {
      Some(self)
    } else {
      self.content.iter().find_map(|item| match item {
        SectionContent::Section(section) => section.get_section(id),
        SectionContent::Staff(_) => None,
      })
    }
  }

  #[must_use]
  pub fn get_section_mut(&mut self, id: usize) -> Option<&mut Section> {
    if self.id == id {
      Some(self)
    } else {
      self.content.iter_mut().find_map(|item| match item {
        SectionContent::Section(section) => section.get_section_mut(id),
        SectionContent::Staff(_) => None,
      })
    }
  }

  #[must_use]
  pub fn get_chord(&self, id: usize) -> Option<&Chord> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) => staff.get_chord(id),
      SectionContent::Section(section) => section.get_chord(id),
    })
  }

  #[must_use]
  pub fn get_chord_mut(&mut self, id: usize) -> Option<&mut Chord> {
    self.content.iter_mut().find_map(|item| match item {
      SectionContent::Staff(staff) => staff.get_chord_mut(id),
      SectionContent::Section(section) => section.get_chord_mut(id),
    })
  }

  #[must_use]
  pub fn get_multivoice(&self, id: usize) -> Option<&MultiVoice> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) => staff.get_multivoice(id),
      SectionContent::Section(section) => section.get_multivoice(id),
    })
  }

  #[must_use]
  pub fn get_multivoice_mut(&mut self, id: usize) -> Option<&mut MultiVoice> {
    self.content.iter_mut().find_map(|item| match item {
      SectionContent::Staff(staff) => staff.get_multivoice_mut(id),
      SectionContent::Section(section) => section.get_multivoice_mut(id),
    })
  }

  #[must_use]
  pub fn get_note(&self, id: usize) -> Option<&Note> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) => staff.get_note(id),
      SectionContent::Section(section) => section.get_note(id),
    })
  }

  #[must_use]
  pub fn get_note_mut(&mut self, id: usize) -> Option<&mut Note> {
    self.content.iter_mut().find_map(|item| match item {
      SectionContent::Staff(staff) => staff.get_note_mut(id),
      SectionContent::Section(section) => section.get_note_mut(id),
    })
  }

  #[must_use]
  pub fn get_phrase(&self, id: usize) -> Option<&Phrase> {
    self.content.iter().find_map(|item| match item {
      SectionContent::Staff(staff) => staff.get_phrase(id),
      SectionContent::Section(section) => section.get_phrase(id),
    })
  }

  #[must_use]
  pub fn get_phrase_mut(&mut self, id: usize) -> Option<&mut Phrase> {
    self.content.iter_mut().find_map(|item| match item {
      SectionContent::Staff(staff) => staff.get_phrase_mut(id),
      SectionContent::Section(section) => section.get_phrase_mut(id),
    })
  }

  #[must_use]
  pub fn get_modification(&self, id: usize) -> Option<&SectionModification> {
    self
      .modifications
      .iter()
      .find(|modification| modification.get_id() == id)
  }

  #[must_use]
  pub fn get_modification_mut(&mut self, id: usize) -> Option<&mut SectionModification> {
    self
      .modifications
      .iter_mut()
      .find(|modification| modification.get_id() == id)
  }

  #[must_use]
  pub fn get_total_iterations(&self) -> u8 {
    self
      .modifications
      .iter()
      .find_map(|item| match item.r#type {
        SectionModificationType::Repeat { num_times } => Some(num_times),
        _ => None,
      })
      .unwrap_or(1)
  }

  #[must_use]
  pub fn get_playable_iterations(&self) -> Vec<u8> {
    self
      .modifications
      .iter()
      .find_map(|item| match &item.r#type {
        SectionModificationType::OnlyPlay { iterations } => Some(iterations.clone()),
        _ => None,
      })
      .unwrap_or_default()
  }

  #[must_use]
  pub fn get_section_tempo(&self) -> Option<Tempo> {
    self.modifications.iter().find_map(|item| match item.r#type {
      SectionModificationType::TempoExplicit { tempo } => Some(tempo),
      SectionModificationType::TempoImplicit { tempo } => {
        Some(Tempo::new(Duration::new(DurationType::Quarter, 0), tempo.value()))
      }
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
            beats += staff.get_beats(&section_beat_base) * num_repeats;
            staff_found = true;
          }
        }
        SectionContent::Section(section) => {
          let num_iterations = match section.get_playable_iterations().len() {
            0 => num_repeats,
            count => count as f64,
          };
          beats += section.get_beats(&section_beat_base) * num_iterations;
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
      SectionContent::Staff(staff) => staff.get_id() != id,
      SectionContent::Section(section) => section.get_id() != id,
    });
    self.content.iter_mut().for_each(|item| match item {
      SectionContent::Staff(staff) => {
        staff.remove_item(id);
      }
      SectionContent::Section(section) => {
        section.remove_item(id);
      }
    });
    self
  }

  pub fn remove_modification(&mut self, id: usize) -> &mut Self {
    self.modifications.retain(|modification| modification.get_id() != id);
    self
  }

  #[must_use]
  pub fn num_timeslices(&self) -> usize {
    self.iter_timeslices().len()
  }

  pub fn iter(&self) -> Iter<'_, SectionContent> {
    self.content.iter()
  }

  pub fn iter_mut(&mut self) -> IterMut<'_, SectionContent> {
    self.content.iter_mut()
  }

  pub fn iter_modifications(&self) -> Iter<'_, SectionModification> {
    self.modifications.iter()
  }

  pub fn iter_modifications_mut(&mut self) -> IterMut<'_, SectionModification> {
    self.modifications.iter_mut()
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
      let mut sections = Vec::new();
      let mut timeslices = Vec::new();
      let mut implicit_section: Option<&mut Section> = None;
      for item in &self.content {
        match item {
          SectionContent::Staff(staff) => match implicit_section {
            Some(ref mut section) => {
              section.content.push(SectionContent::Staff(staff.clone()));
            }
            None => {
              sections.push(Section::new("Implicit"));
              implicit_section = sections.last_mut();
            }
          },
          SectionContent::Section(section) => {
            sections.push(section.clone());
            implicit_section = None;
          }
        }
      }

      // Iterate through all sub-sections the correct number of times
      for iteration in 0..self.get_total_iterations() {
        for section in &sections {
          if section.get_playable_iterations().is_empty() || section.get_playable_iterations().contains(&iteration) {
            timeslices.extend(section.iter_timeslices());
          }
        }
      }
      timeslices
    } else {
      // Iterate over all staves the correct number of times
      let mut timeslices: Vec<(f64, Timeslice)> = Vec::new();
      for _ in 0..self.get_total_iterations() {
        for item in &self.content {
          match item {
            SectionContent::Staff(staff) => {
              let (mut index, mut curr_time) = (0, 0.0);
              for slice in staff.iter_timeslices() {
                (index, curr_time) = place_and_merge_timeslice(&mut timeslices, slice, index, curr_time);
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

impl Clone for Section {
  fn clone(&self) -> Self {
    Self {
      id: generate_id(),
      name: self.name.clone(),
      content: self.content.clone(),
      modifications: self.modifications.clone(),
    }
  }
}

impl PartialEq for Section {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name && self.content == other.content && self.modifications == other.modifications
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Section {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let mods = self
      .modifications
      .iter()
      .map(ToString::to_string)
      .collect::<Vec<String>>()
      .join(", ");
    let items = self
      .content
      .iter()
      .map(|item| match item {
        SectionContent::Staff(staff) => staff.to_string(),
        SectionContent::Section(section) => section.to_string(),
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
