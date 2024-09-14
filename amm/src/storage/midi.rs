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
use midly::num::{u7, u15, u24, u28};
use note_preprocessing::{RawNoteData, RawNoteHandler};
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

impl_bit_extend!(u7 => u8, u15 => u16, u24 => u32, u28 => u32);

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

mod midi_header {
  use crate::storage::midi::BitExtend;

  pub fn get_ticks_per_beat(header: &midly::Header) -> u32 {
    let midly::Header { format: _, timing } = header;
    if let midly::Timing::Metrical(x) = timing {
      let ticks_per_beat = (*x).extend();
      return ticks_per_beat as u32;
    }
    panic!("Timing format not supported");
  }
}

mod note_preprocessing {
  use super::*;

  pub struct RawNoteData {
    pub key: u8,
    pub onset: u32,
    pub vel: u8,
  }

  impl RawNoteData {
    fn new(key: u8, onset: u32, vel: u8) -> Self {
      Self { key, onset, vel }
    }
  }
  pub struct RawNoteHandler {
    last_note_on_offset: u32,
    last_note_off_offset: u32,
    last_note_velocity: u8,
    ticks_per_beat: u32,
  }

  impl RawNoteHandler {
    pub fn new(ticks_per_beat: u32) -> Self {
      Self {
        last_note_on_offset: 0,
        last_note_off_offset: 0,
        last_note_velocity: 0,
        ticks_per_beat,
      }
    }

    pub fn handle(&mut self, event: &midly::MidiMessage, cur_time: u32) -> Option<RawNoteData> {
      if let midly::MidiMessage::NoteOn { key: _, vel } = event {
        self.last_note_velocity = vel.extend();
        self.last_note_on_offset = cur_time;
        let note_length_epsilon = (self.ticks_per_beat as f32 *  0.125).ceil() as u32;
        if self.last_note_on_offset - self.last_note_off_offset >= note_length_epsilon {
          Some(RawNoteData::new(255, self.last_note_off_offset, 0))
        } else {
          None
        }
      } else if let midly::MidiMessage::NoteOff { key , vel: _ } = event {
        self.last_note_off_offset = cur_time;
        Some(RawNoteData::new(key.extend(), self.last_note_on_offset, self.last_note_velocity))
      } else {
        None
      }
    }
  }
}

mod track_processing {
  use std::collections::VecDeque;
  use crate::storage::midi::control_track::ControlTrackData;
  use crate::storage::midi::{BitExtend, TimeStamp};
  use super::note_preprocessing::{RawNoteData, RawNoteHandler};

  pub enum TrackEvent {
    MidiEvent {
      delta: TimeStamp,
      event: RawNoteData,
    },
  }

  pub struct Section {
    pub content: Vec<TrackEvent>,
  }

  impl Section {
    fn new() -> Self {
      Self { content: Vec::new() }
    }

    fn push(&mut self, event: TrackEvent) {
      self.content.push(event);
    }
  }

  pub struct TrackData {
    pub content: Vec<Section>
  }

  impl TrackData {
    fn new() -> Self {
      Self { content: vec![Section::new()] }
    }

    fn push(&mut self, event: TrackEvent) {
      let last_index = self.content.len() - 1;
      self.content[last_index].push(event);
    }

    fn add_section(&mut self) {
      self.content.push(Section::new());
    }
  }

  pub fn parse_track_data(
    track_data: &[midly::TrackEvent],
    control_track_data: &ControlTrackData,
    mut ticks_per_beat: u32
  ) -> TrackData {
    let scalar = if ticks_per_beat % 12 == 0 { 1 } else { 12 };
    ticks_per_beat = if ticks_per_beat % 12 == 0 { ticks_per_beat } else { ticks_per_beat * 12 };
    let mut delta: u32 = 0;
    let mut raw_note_handler = RawNoteHandler::new(ticks_per_beat);
    let mut processed_data = TrackData::new();

    let mut time_signature_index: usize = 1;
    for event in track_data {
      delta += (event.delta.extend() * scalar);

      if time_signature_index < control_track_data.time_signature_messages.len() &&
          delta >= control_track_data.time_signature_messages[time_signature_index].1 * scalar {
        time_signature_index += 1;
        processed_data.add_section();
      }

      if let midly::TrackEventKind::Midi { channel: _, message} = event.kind {
        let raw_note_data = raw_note_handler.handle(&message, delta);
        if let Some(raw_note_data) = raw_note_data {
          processed_data.push(TrackEvent::MidiEvent {
            delta: raw_note_data.onset,
            event: raw_note_data
          });
        }
      }
    }

    while time_signature_index < control_track_data.time_signature_messages.len() {
      time_signature_index += 1;
      processed_data.add_section();
    }

    processed_data
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
  use crate::storage::midi::track_processing::{TrackData};
  use super::*;

  fn set_up_control_track_data(path: &str) -> ControlTrackData {
    let contents = fs::read(path).unwrap();
    let midi = Smf::parse(&contents).unwrap();
    let control_track = &midi.tracks[0];
    ControlTrackData::parse_control_track(control_track)
  }

  fn set_up_first_track_data(path: &str) -> TrackData {
    let contents = fs::read(path).unwrap();
    let midi = Smf::parse(&contents).unwrap();
    let control_track =  ControlTrackData::parse_control_track(&midi.tracks[0]);
    let ticks_per_beat = midi_header::get_ticks_per_beat(&midi.header);
    track_processing::parse_track_data(&midi.tracks[1], &control_track, ticks_per_beat)
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

  #[test]
  fn test_track_preprocessing() {
    let track_data_1 = set_up_first_track_data("tests/test_midi_files/test-6.mid");
    assert_eq!(track_data_1.content.len(), 3);

    let track_data_2 = set_up_first_track_data("tests/test_midi_files/test-1.mid");
    assert_eq!(track_data_2.content.len(), 1);
    let section = track_data_2.content.first().unwrap();
    assert_eq!(section.content.len(), 26);
  }
}
