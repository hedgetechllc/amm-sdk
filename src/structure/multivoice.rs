use super::{chord::Chord, note::Note, phrase::Phrase};
use crate::context::{generate_id, Tempo};
use std::{cell::RefCell, rc::Rc, slice::Iter};

pub enum MultiVoiceContent {
  Phrase(Rc<RefCell<Phrase>>),
  MultiVoice(Rc<RefCell<MultiVoice>>),
}

pub struct MultiVoice {
  id: usize,
  content: Vec<MultiVoiceContent>,
}

impl MultiVoice {
  pub fn new() -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      id: generate_id(),
      content: Vec::new(),
    }))
  }

  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn add_phrase(&mut self) -> Rc<RefCell<Phrase>> {
    let phrase = Phrase::new();
    self.content.push(MultiVoiceContent::Phrase(Rc::clone(&phrase)));
    phrase
  }

  pub fn add_multivoice(&mut self) -> Rc<RefCell<MultiVoice>> {
    let multivoice = MultiVoice::new();
    self.content.push(MultiVoiceContent::MultiVoice(Rc::clone(&multivoice)));
    multivoice
  }

  pub fn get_phrase(&mut self, id: usize) -> Option<Rc<RefCell<Phrase>>> {
    self.content.iter().find_map(|item| match item {
      MultiVoiceContent::Phrase(phrase) if phrase.borrow().get_id() == id => Some(Rc::clone(phrase)),
      MultiVoiceContent::Phrase(phrase) => phrase.borrow_mut().get_phrase(id),
      _ => None,
    })
  }

  pub fn get_multivoice(&mut self, id: usize) -> Option<Rc<RefCell<MultiVoice>>> {
    self.content.iter().find_map(|item| match item {
      MultiVoiceContent::MultiVoice(multivoice) if multivoice.borrow().get_id() == id => Some(Rc::clone(multivoice)),
      MultiVoiceContent::MultiVoice(multivoice) => multivoice.borrow_mut().get_multivoice(id),
      _ => None,
    })
  }

  pub fn get_chord(&mut self, id: usize) -> Option<Rc<RefCell<Chord>>> {
    self.content.iter().find_map(|item| match item {
      MultiVoiceContent::Phrase(phrase) => phrase.borrow_mut().get_chord(id),
      MultiVoiceContent::MultiVoice(multivoice) => multivoice.borrow_mut().get_chord(id),
    })
  }

  pub fn get_note(&mut self, id: usize) -> Option<Rc<RefCell<Note>>> {
    self.content.iter().find_map(|item| match item {
      MultiVoiceContent::Phrase(phrase) => phrase.borrow_mut().get_note(id),
      MultiVoiceContent::MultiVoice(multivoice) => multivoice.borrow_mut().get_note(id),
    })
  }

  pub fn remove_item(&mut self, id: usize) -> &mut Self {
    self.content.retain(|item| match item {
      MultiVoiceContent::Phrase(phrase) => phrase.borrow().get_id() != id,
      MultiVoiceContent::MultiVoice(multivoice) => multivoice.borrow().get_id() != id,
    });
    self.content.iter().for_each(|item| match item {
      MultiVoiceContent::Phrase(phrase) => {
        phrase.borrow_mut().remove_item(id);
      }
      MultiVoiceContent::MultiVoice(multivoice) => {
        multivoice.borrow_mut().remove_item(id);
      }
    });
    self
  }

  pub fn get_duration(&self, tempo: &Tempo) -> f64 {
    self
      .content
      .iter()
      .map(|content| match &content {
        MultiVoiceContent::Phrase(phrase) => phrase.borrow().get_duration(&tempo, None),
        MultiVoiceContent::MultiVoice(multivoice) => multivoice.borrow().get_duration(&tempo),
      })
      .reduce(f64::max)
      .unwrap_or_default()
  }

  pub fn iter(&self) -> Iter<'_, MultiVoiceContent> {
    self.content.iter()
  }
}

impl std::fmt::Display for MultiVoice {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let voices = self
      .content
      .iter()
      .map(|item| match item {
        MultiVoiceContent::Phrase(phrase) => phrase.borrow().to_string(),
        MultiVoiceContent::MultiVoice(multi_voice) => multi_voice.borrow().to_string(),
      })
      .collect::<Vec<_>>()
      .join(", ");
    write!(f, "MultiVoice: [{}]", voices)
  }
}

impl<'a> IntoIterator for &'a MultiVoice {
  type Item = <Iter<'a, MultiVoiceContent> as Iterator>::Item;
  type IntoIter = Iter<'a, MultiVoiceContent>;
  fn into_iter(self) -> Self::IntoIter {
    self.content.as_slice().into_iter()
  }
}
