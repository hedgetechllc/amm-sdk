use super::Load;
use crate::Composition;
use alloc::string::String;
use midly::{Smf, Track, MetaMessage};
use midly::num::{u7, u15, u24, u28};
use std::fs;
use crate::context::{Key, KeyMode, TimeSignature};
use crate::modification::{Direction, DirectionType};
use crate::note::{Accidental, Duration, Note, Pitch, PitchName};
use crate::structure::{Staff, StaffContent};

struct PitchContainer {
  pitch: Pitch,
  accidental: Accidental,
}

impl PitchContainer {
  fn new(mut key: u8) -> Self {
    if key == 255 {
      return Self::new_rest()
    }

    let (pitch_name, accidental) = match key % 12 {
      0 => (PitchName::C, Accidental::default()),
      1 => (PitchName::D, Accidental::Flat),
      2 => (PitchName::D, Accidental::default()),
      3 => (PitchName::E, Accidental::default()),
      4 => (PitchName::E, Accidental::default()),
      5 => (PitchName::F, Accidental::default()),
      6 => (PitchName::F, Accidental::Sharp),
      7 => (PitchName::G, Accidental::default()),
      8 => (PitchName::A, Accidental::Flat),
      9 => (PitchName::A, Accidental::default()),
      10 => (PitchName::B, Accidental::Flat),
      11 => (PitchName::B, Accidental::default()),
      _ => (PitchName::Rest, Accidental::default()),
    };

    let mut octave = 0;
    while key >= 12 {
      octave += 1;
      key -= 12;
    }
    octave -= 1;

    Self {
      pitch: Pitch::new(pitch_name, octave),
      accidental,
    }
  }

  fn new_rest() -> Self {
    Self {
      pitch: Pitch::new_rest(),
      accidental: Default::default(),
    }
  }
}

impl Note {
  fn from_raw_note_data(key: u8, beat_length: f64) -> Self {
    let pitch_container = PitchContainer::new(key);
    let pitch = pitch_container.pitch;
    let duration = Duration::from_beats(&Duration::default(), beat_length);
    let accidental = pitch_container.accidental;
    Self::new(pitch, duration, Some(accidental))
  }
}

impl Staff {
  fn add_content(&mut self, content: &StaffContent) {
    self.content.push(content.clone());
  }
}

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

fn get_ticks_per_beat(header: &midly::Header) -> u32 {
  let midly::Header { format: _, timing } = header;
  if let midly::Timing::Metrical(x) = timing {
    let ticks_per_beat = (*x).extend();
    return ticks_per_beat as u32;
  }
  panic!("Timing format not supported");
}

struct MetaHandler {
  initial_time_signature: Option<TimeSignature>,
  initial_key: Option<Key>,
}

impl MetaHandler {
  fn new() -> Self {
    Self {
      initial_time_signature: None,
      initial_key: None,
    }
  }

  fn handle(&mut self, message: &midly::MetaMessage) -> Option<StaffContent> {
    match *message {
      MetaMessage::KeySignature(fifths, flag) => {
        let mode = Some(if flag { KeyMode::Major } else { KeyMode::Minor });
        let key = Key::from_fifths(fifths, mode);
        let direction_type = DirectionType::KeyChange { key };
        if self.initial_key.is_none() {
          self.initial_key = Some(key);
        }
        Some(StaffContent::Direction(Direction::new(direction_type)))
      },
      MetaMessage::TimeSignature(numerator, beat_type_int, _, _) => {
        let denominator = 2u8.pow(beat_type_int as u32);
        let time_signature = TimeSignature::new_explicit(numerator, denominator);
        let direction_type = DirectionType::TimeSignatureChange { time_signature };
        if self.initial_time_signature.is_none() {
          self.initial_time_signature = Some(time_signature);
        }
        Some(StaffContent::Direction(Direction::new(direction_type)))
      },
      _ => None,
    }
  }
}

struct NoteHandler {
  last_note_on_offset: u32,
  last_note_off_offset: u32,
  last_note_velocity: u8,
  ticks_per_beat: u32,
}

impl NoteHandler {
  fn new(ticks_per_beat: u32) -> Self {
    Self {
      last_note_on_offset: 0,
      last_note_off_offset: 0,
      last_note_velocity: 0,
      ticks_per_beat,
    }
  }

  fn handle(&mut self, event: &midly::MidiMessage, cur_time: u32) -> Option<StaffContent> {
    if let midly::MidiMessage::NoteOn { key: _, vel } = event {
      self.last_note_velocity = vel.extend();
      self.last_note_on_offset = cur_time;
      let epsilon = (self.ticks_per_beat as f32 *  0.125).ceil() as u32;
      if self.last_note_on_offset - self.last_note_off_offset >= epsilon {
        let beat_length = (self.last_note_on_offset - self.last_note_off_offset) as f64 /
            self.ticks_per_beat as f64;
        return Some(StaffContent::Note(Note::from_raw_note_data(255, beat_length)));
      }
    } else if let midly::MidiMessage::NoteOff { key , vel: _ } = event {
      self.last_note_off_offset = cur_time;
      let beat_length = (self.last_note_off_offset - self.last_note_on_offset) as f64 /
          self.ticks_per_beat as f64;
      return Some(StaffContent::Note(Note::from_raw_note_data(key.extend(), beat_length)));
    }
    None
  }
}

struct TrackContent {
  pub content: Vec<(StaffContent, TimeStamp)>,
}

impl TrackContent {
  fn new(track: &Track, ticks_per_beat: u32) -> Self {
    let mut content = Vec::new();
    let mut cur_time = 0;
    let mut meta_handler = MetaHandler::new();
    let mut note_handler = NoteHandler::new(ticks_per_beat);

    for event in track {
      cur_time += event.delta.extend();
      if let midly::TrackEventKind::Meta(message) = event.kind {
        let result = meta_handler.handle(&message);
        if result.is_some() {
          content.push((result.unwrap(), cur_time));
        }
      }
      if let midly::TrackEventKind::Midi { channel: _, message } = event.kind {
        let result = note_handler.handle(&message, cur_time);
        if result.is_some() {
          content.push((result.unwrap(), cur_time));
        }
      }
    }

    Self {
      content
    }
  }

  fn from_list(tracks: &[Track], ticks_per_beat: u32) -> Vec<Self> {
    let mut temp = Vec::new();
    for track in tracks {
      temp.push(Self::new(track, ticks_per_beat));
    }
    temp
  }

  fn merge_control_track(&mut self, control_track: &Self) {
    let mut temp = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while j < self.content.len() && i < control_track.content.len() {
      if control_track.content[i].1 < self.content[j].1 {
        temp.push(control_track.content[i].clone());
        i += 1;
      } else {
        temp.push(self.content[j].clone());
        j += 1;
      }
    }
    while j < self.content.len() {
      temp.push(self.content[j].clone());
      j += 1;
    }
    while i < control_track.content.len() {
      temp.push(control_track.content[i].clone());
      i += 1;
    }
    self.content = temp;
  }
}

pub struct MidiConverter;

impl MidiConverter {
  fn load_from_midi(data: &[u8]) -> Result<Composition, String> {
    // Parse the MIDI representation
    let _midi = Smf::parse(data).map_err(|err| err.to_string())?;
    let track_count = _midi.tracks.len();
    let ticks_per_beat = get_ticks_per_beat(&_midi.header);
    let mut tracks = TrackContent::from_list(&_midi.tracks[1..track_count], ticks_per_beat);
    let control_track = TrackContent::new(&_midi.tracks[0], ticks_per_beat);
    for track in tracks.iter_mut() {
      track.merge_control_track(&control_track);
    }

    // Generate the initial composition structure and fill in known metadata
    let mut composition = Composition::new("Error", None, None, None);
    let part = composition.add_part("todo_part_name");
    let section = part.add_section("Default");
    for track in tracks.into_iter() {
      let staff = section.add_staff("todo_staff_name");
      for event in track.content {
        staff.add_content(&event.0);
      }
      println!("{}", staff);
    }

    // Return the fully constructed composition
    Ok(composition)
  }
}

impl Load for MidiConverter {
  fn load(path: &str) -> Result<Composition, String> {
    let data = fs::read(path).map_err(|err| err.to_string())?;
    MidiConverter::load_from_midi(&data)
  }

  fn load_data(data: &[u8]) -> Result<Composition, String> {
    MidiConverter::load_from_midi(data)
  }
}

mod tests {
  use super::*;

  #[test]
  fn parser_test() {
    let composition = MidiConverter::load("tests/test_midi_files/test-2.mid");
    assert!(composition.is_ok());
  }
}