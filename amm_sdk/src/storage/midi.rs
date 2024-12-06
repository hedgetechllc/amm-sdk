use super::Load;
use crate::Composition;
use alloc::string::String;
use midly::{Smf, Track, MetaMessage};
use midly::num::{u7, u15, u24, u28};
use std::collections::VecDeque;
use std::fs;
use crate::context::{Key, KeyMode, TimeSignature};
use crate::modification::{Direction, DirectionType};
use crate::note::{Accidental, Duration, Note, Pitch, PitchName};
use crate::structure::{Staff, StaffContent};

const WHOLE_VALUE: f64 = 1.0;
const HALF_VALUE: f64 = 0.5;
const QUARTER_VALUE: f64 = 0.25;
const EIGHTH_VALUE: f64 = 0.125;
const SIXTEENTH_VALUE: f64 = 0.062_5;
const THIRTY_SECOND_VALUE: f64 = 0.031_25;
const SIXTY_FOURTH_VALUE: f64 = 0.015_625;

const POSSIBLE_NOTE_LENGTHS: [f64; 21] = [
  SIXTY_FOURTH_VALUE, SIXTY_FOURTH_VALUE * 1.5, SIXTY_FOURTH_VALUE * 1.75,
  THIRTY_SECOND_VALUE, THIRTY_SECOND_VALUE * 1.5, THIRTY_SECOND_VALUE * 1.75,
  SIXTEENTH_VALUE, SIXTEENTH_VALUE * 1.5, SIXTEENTH_VALUE * 1.75,
  EIGHTH_VALUE, EIGHTH_VALUE * 1.5, EIGHTH_VALUE * 1.75,
  QUARTER_VALUE, QUARTER_VALUE * 1.5, QUARTER_VALUE * 1.75,
  HALF_VALUE, HALF_VALUE * 1.5, HALF_VALUE * 1.75,
  WHOLE_VALUE, WHOLE_VALUE * 1.5, WHOLE_VALUE,
];

fn floor_note_length(x: f64) -> f64 {
  let mut value = 0.0;
  for duration in POSSIBLE_NOTE_LENGTHS {
    if duration <= x {
      value = duration;
    } else {
      return value;
    }
  }
  value
}

impl Duration {
  fn from_beats_with_tie(beat_base_value: &Duration, beat_length: f64) -> Vec<Self> {
    let mut value = beat_length * beat_base_value.value();
    let mut durations = Vec::new();
    while value > SIXTY_FOURTH_VALUE {
      let temp = floor_note_length(value);
      durations.push(Self::from_beats(beat_base_value, temp / beat_base_value.value()));
      value -= temp;
    }
    durations
  }
}

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

enum NoteWrapper {
  PlainNote(StaffContent),
  TiedNote(Vec<StaffContent>),
}

impl Note {
  fn from_raw_note_data(key: u8, beat_length: f64) -> NoteWrapper {
    let pitch_container = PitchContainer::new(key);
    let pitch = pitch_container.pitch;
    let duration = Duration::from_beats_with_tie(&Duration::default(), beat_length);
    let accidental = pitch_container.accidental;

    if duration.len() == 0 {
      let note = Self::new(pitch, Duration::default(), Some(accidental));
      NoteWrapper::PlainNote(StaffContent::Note(note))
    } else if duration.len() == 1 {
      let note = Self::new(pitch, duration[0], Some(accidental));
      NoteWrapper::PlainNote(StaffContent::Note(note))
    } else {
      let mut staff_content = Vec::new();
      for d in duration {
        let note = Self::new(pitch, d, Some(accidental));
        staff_content.push(StaffContent::Note(note));
      }
      NoteWrapper::TiedNote(staff_content)
    }
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

  fn get_staff_content(&mut self, message: &midly::MetaMessage) -> Option<StaffContent> {
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
        Some(StaffContent::Direction(Direction::new(direction_type)) )
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

  fn handle(&mut self, event: &midly::MidiMessage, cur_time: u32) -> Option<NoteWrapper> {
    if let midly::MidiMessage::NoteOn { key: _, vel } = event {
      self.last_note_velocity = vel.extend();
      self.last_note_on_offset = cur_time;
      let epsilon = (self.ticks_per_beat as f32 *  0.125).ceil() as u32;
      if self.last_note_on_offset - self.last_note_off_offset >= epsilon {
        let beat_length = (self.last_note_on_offset - self.last_note_off_offset) as f64 /
            self.ticks_per_beat as f64;
        return Some(Note::from_raw_note_data(255, beat_length));
      }
    } else if let midly::MidiMessage::NoteOff { key , vel: _ } = event {
      self.last_note_off_offset = cur_time;
      let beat_length = (self.last_note_off_offset - self.last_note_on_offset) as f64 /
          self.ticks_per_beat as f64;
      return Some(Note::from_raw_note_data(key.extend(), beat_length));
    }
    None
  }
}

fn parse_control_track(control_track: &Track) -> VecDeque<(StaffContent, TimeStamp)> {
  let mut content = VecDeque::new();
  let mut cur_time = 0;
  let mut meta_handler = MetaHandler::new();
  for event in control_track {
    cur_time += event.delta.extend();
    if let midly::TrackEventKind::Meta(message) = event.kind {
      let result = meta_handler.get_staff_content(&message);
      if result.is_some() {
        content.push_back((result.unwrap(), cur_time));
      }
    }
  }
  content
}

fn load_staff_content(
  staff: &mut Staff, 
  mut control_track: VecDeque<(StaffContent, TimeStamp)>, 
  track: &Track,
  ticks_per_beat: u32,
) {
  let mut cur_time = 0;
  let mut meta_handler = MetaHandler::new();
  let mut note_handler = NoteHandler::new(ticks_per_beat);

  for event in track {
    cur_time += event.delta.extend();
    if control_track.front().is_some() {
      if control_track.front().unwrap().1 >= cur_time {
        let staff_content = control_track.pop_front();
        staff.add_content(&staff_content.unwrap().0);
      }
    }

    if let midly::TrackEventKind::Meta(message) = event.kind {
      let result = meta_handler.get_staff_content(&message);
      if result.is_some() {
        staff.add_content(&result.unwrap());
      }
    }
    if let midly::TrackEventKind::Midi { channel: _, message } = event.kind {
      let result = note_handler.handle(&message, cur_time);
      if result.is_some() {
        match result.unwrap() {
          NoteWrapper::PlainNote(n) => staff.add_content(&n),
          NoteWrapper::TiedNote(v) => {
            for n in v {
              staff.add_content(&n);
            }
          }
        }
      }
    }
  }
}

pub struct MidiConverter;

impl MidiConverter {
  fn load_from_midi(data: &[u8]) -> Result<Composition, String> {
    // Parse the MIDI representation
    let _midi = Smf::parse(data).map_err(|err| err.to_string())?;
    let track_count = _midi.tracks.len();
    let ticks_per_beat = get_ticks_per_beat(&_midi.header);
    let control_track = parse_control_track(&_midi.tracks[0]);

    // Generate the initial composition structure and fill in known metadata
    let mut composition = Composition::new("Default", None, None, None);
    let part = composition.add_part("todo_part_name");
    let section = part.add_section("Default");
    for i in 1..track_count {
      let staff = section.add_staff(&format!("{}", i));
      load_staff_content(staff, control_track.clone(), &_midi.tracks[i], ticks_per_beat);
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

  #[test]
  fn tie_note_test() {
    let beat_length_1 = 2.5;
    let tied_1 = Duration::from_beats_with_tie(&Duration::default(), beat_length_1);
    assert_eq!(tied_1.len(), 2);
    assert_eq!(tied_1[0].value(), HALF_VALUE);
    assert_eq!(tied_1[1].value(), EIGHTH_VALUE);

    let beat_length_2 = 5.0;
    let tied_2 = Duration::from_beats_with_tie(&Duration::default(), beat_length_2);
    assert_eq!(tied_2.len(), 2);
    assert_eq!(tied_2[0].value(), WHOLE_VALUE);
    assert_eq!(tied_2[1].value(), QUARTER_VALUE);

    let beat_length_3 = 1.25;
    let tied_3 = Duration::from_beats_with_tie(&Duration::default(), beat_length_3);
    assert_eq!(tied_3.len(), 2);
    assert_eq!(tied_3[0].value(), QUARTER_VALUE);
    assert_eq!(tied_3[1].value(), SIXTEENTH_VALUE);

    let beat_length_4 = 5.25;
    let tied_4 = Duration::from_beats_with_tie(&Duration::default(), beat_length_4);
    assert_eq!(tied_4.len(), 3);
    assert_eq!(tied_4[0].value(), WHOLE_VALUE);
    assert_eq!(tied_4[1].value(), QUARTER_VALUE);
    assert_eq!(tied_4[2].value(), SIXTEENTH_VALUE);
  }
}
