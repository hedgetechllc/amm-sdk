use super::Convert;
use crate::{
  Accidental, Chord, ChordContent, ChordModification, ChordModificationType, Clef, ClefSymbol, ClefType, Composition,
  Direction, DirectionType, Duration, DurationType, Dynamic, DynamicMarking, HandbellTechnique, Key, KeyMode,
  KeySignature, MultiVoice, Note, NoteModification, NoteModificationType, PedalType, Phrase, PhraseContent,
  PhraseModification, PhraseModificationType, Pitch, PitchName, Section, SectionModificationType, Staff, StaffContent,
  Tempo, TempoMarking, TempoSuggestion, TimeSignature, TimeSignatureType,
};
use alloc::{string::String, vec::Vec};
use control_track::ControlTrackData;
use midly::Smf;
use midly::num::{u24, u28};
use std::fs;

type TimeStamp = u32;

trait BitExtend {
  type Out;
  fn extend(&self) -> Self::Out;
}

macro_rules! impl_bit_extend {
  ($($f:ident => $t:ident), *) => {
    $(
      impl BitExtend for $f {
        type Out = $t;
        fn extend(&self) -> Self::Out {
          let temp: Self::Out = (*self).into();
          temp
        }
      }
    )*
  };
}

impl_bit_extend!(u24 => u32, u28 => u32);

mod control_track {
  use super::*;

  pub struct ControlTrackData {
    pub key_signature_messages: Vec<(Key, TimeStamp)>,
    pub tempo_messages: Vec<(u32, TimeStamp)>,
    pub time_signature_messages: Vec<(TimeSignature, TimeStamp)>,
  }

  impl ControlTrackData {
    pub fn parse_control_track(control_track: &[midly::TrackEvent]) -> Self {
      let mut control_track_data = Self::new();
      let mut cur_time = 0;
      for event in control_track {
        cur_time += event.delta.extend();
        if let midly::TrackEventKind::Meta(message) = event.kind {
          control_track_data.update_data(&message, cur_time);
        }
      }
      control_track_data
    }

    fn new() -> Self {
      Self {
        key_signature_messages: Vec::new(),
        tempo_messages: Vec::new(),
        time_signature_messages: Vec::new(),
      }
    }

    fn update_data(&mut self, message: &midly::MetaMessage, time_stamp: u32) {
      match *message {
        midly::MetaMessage::KeySignature(fifths, t) => {
          let mode = Some(if t { KeyMode::Major } else { KeyMode::Minor });
          self.key_signature_messages.push((Key::from_fifths(fifths, mode), time_stamp));
        },
        midly::MetaMessage::Tempo(mspb) => {
          self.tempo_messages.push((mspb.extend() / 1000000 * 60, time_stamp));
        },
        midly::MetaMessage::TimeSignature(numerator, beat_type_int, _, _) => {
          let denominator = 2u8.pow(beat_type_int as u32);
          let time_signature = TimeSignature::new_explicit(numerator, denominator);
          self.time_signature_messages.push((time_signature.clone(), time_stamp));
        }
        _ => {}
      }
    }
  }
}

pub struct MidiConverter;

impl MidiConverter {
  fn load_from_midi(data: &[u8]) -> Result<Composition, String> {
    // Parse the MIDI representation
    let midi = Smf::parse(data).map_err(|err| err.to_string())?;

    // Generate the initial composition structure and fill in known metadata
    let mut composition = Composition::new("Error", None, None, None);
    // TODO: Fill in metadata from MIDI, set correct title, etc.

    // TODO: Implement construction of Composition from MIDI
    let mut part = composition.add_part("todo_part_name");
    let section = part.add_default_section();
    let staff_rc = section.borrow_mut().add_staff("todo_staff_name", None, None, None);
    let mut staff = staff_rc.borrow_mut();
    staff.add_chord();

    // Return the fully constructed composition
    Ok(composition)
  }
}

impl Convert for MidiConverter {
  fn load(path: &str) -> Result<Composition, String> {
    let data = fs::read(path).map_err(|err| err.to_string())?;
    MidiConverter::load_from_midi(&data)
  }

  fn load_data(data: &[u8]) -> Result<Composition, String> {
    MidiConverter::load_from_midi(&data)
  }

  fn save(_path: &str, _composition: &Composition) -> Result<usize, String> {
    todo!(); // TODO: Implement
  }
}

#[cfg(test)]
mod test {
  use super::*;

  fn set_up_control_track_data(path: &str) -> ControlTrackData {
    let contents = fs::read(path).unwrap();
    let midi = Smf::parse(&contents).unwrap();
    let control_track = &midi.tracks[0];
    ControlTrackData::parse_control_track(control_track)
  }

  #[test]
  fn test_midi_meta_import() {
    let test_1 = set_up_control_track_data("tests/test_midi_files/test-1.mid");
    assert_eq!(test_1.tempo_messages.len(), 1);
    assert_eq!(test_1.key_signature_messages.len(), 0);
    assert_eq!(test_1.time_signature_messages.len(), 1);

    let test_2 = set_up_control_track_data("tests/test_midi_files/test-6.mid");
    assert_eq!(test_2.tempo_messages.len(), 2);
    assert_eq!(test_2.key_signature_messages.len(), 2);
    assert_eq!(test_2.time_signature_messages.len(), 3);
  }
}
