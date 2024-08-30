use crate::context::{generate_id, Tempo};
use crate::modification::{NoteModification, NoteModificationType};
use crate::note::{Accidental, Duration, Pitch};
use crate::storage::{Serialize, SerializedItem};
use alloc::{collections::BTreeMap, rc::Rc, string::String, vec::Vec};
use core::cell::RefCell;

pub use crate::note::Note;

impl Note {
  #[must_use]
  pub fn new(pitch: Pitch, duration: Duration, accidental: Option<Accidental>) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: generate_id(),
      pitch,
      duration,
      accidental: accidental.unwrap_or_default(),
      modifications: Vec::new(),
    }))
  }

  #[must_use]
  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn add_modification(&mut self, modification: NoteModificationType) -> Rc<RefCell<NoteModification>> {
    self
      .modifications
      .retain(|mods| *mods.borrow().get_modification() != modification);
    let modification = NoteModification::new(modification);
    self.modifications.push(Rc::clone(&modification));
    modification
  }

  #[must_use]
  pub fn get_modification(&mut self, id: usize) -> Option<Rc<RefCell<NoteModification>>> {
    self.modifications.iter().find_map(|modification| {
      if modification.borrow().get_id() == id {
        Some(Rc::clone(modification))
      } else {
        None
      }
    })
  }

  #[must_use]
  pub fn get_beats(&self, beat_base: &Duration, tuplet_ratio: Option<f64>) -> f64 {
    self.beats(beat_base.value()) * tuplet_ratio.unwrap_or(1.0)
  }

  #[must_use]
  pub fn get_duration(&self, tempo: &Tempo, tuplet_ratio: Option<f64>) -> f64 {
    self.get_beats(&tempo.base_note, tuplet_ratio) * 60.0 / f64::from(tempo.beats_per_minute)
  }

  pub fn remove_modification(&mut self, id: usize) -> &mut Self {
    self
      .modifications
      .retain(|modification| modification.borrow().get_id() != id);
    self
  }
}

#[cfg(feature = "print")]
impl Serialize for Note {
  fn serialize(&self) -> SerializedItem {
    let mut contents = BTreeMap::from([
      (String::from("pitch"), self.pitch.serialize()),
      (String::from("duration"), self.duration.serialize()),
    ]);
    if Accidental::None != self.accidental {
      contents.insert(String::from("accidental"), self.accidental.serialize());
    }
    SerializedItem {
      attributes: BTreeMap::from([
        (String::from("id"), self.id.to_string()),
        (String::from("type"), String::from("Note")),
      ]),
      contents,
      elements: if self.modifications.is_empty() {
        BTreeMap::new()
      } else {
        BTreeMap::from([(
          String::from("modifications"),
          self
            .modifications
            .iter()
            .map(|modification| modification.borrow().serialize())
            .collect(),
        )])
      },
    }
  }
}
