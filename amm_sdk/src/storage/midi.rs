use super::Load;
use crate::context::{Key, KeyMode, TimeSignature};
use crate::modification::{Direction, DirectionType};
use crate::note::{Duration, DurationType, Note};
use crate::structure::{Staff, StaffContent};
use crate::Composition;
use alloc::string::String;
use midly::{MetaMessage, Smf, Track};
use std::collections::VecDeque;
use std::fs;

type TimeStamp = u32;

enum NoteWrapper {
  PlainNote(StaffContent),
  TiedNote(Vec<StaffContent>),
}

impl Note {
  fn from_raw_note_data(key: u8, beat_length: f64, beat_base_value: Duration) -> NoteWrapper {
    let mut note = Note::from_midi(key, beat_base_value, None);
    let durations = Duration::from_beats_tied(&beat_base_value, beat_length);

    if durations.is_empty() {
      NoteWrapper::PlainNote(StaffContent::Note(note))
    } else if durations.len() == 1 {
      note.duration = durations[0];
      NoteWrapper::PlainNote(StaffContent::Note(note))
    } else {
      let mut staff_content = Vec::new();
      for duration in durations {
        note.duration = duration;
        staff_content.push(StaffContent::Note(note.clone()));
      }
      NoteWrapper::TiedNote(staff_content)
    }
  }
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

  fn get_staff_content(&mut self, message: midly::MetaMessage) -> Option<StaffContent> {
    match message {
      MetaMessage::KeySignature(fifths, flag) => {
        let mode = if flag { KeyMode::Major } else { KeyMode::Minor };
        let key = Key::from_fifths(fifths, Some(mode));
        let direction_type = DirectionType::KeyChange { key };
        if self.initial_key.is_none() {
          self.initial_key = Some(key);
        }
        Some(StaffContent::Direction(Direction::new(direction_type)))
      }
      MetaMessage::TimeSignature(numerator, beat_type_int, _, _) => {
        let denominator = 2u8.pow(u32::from(beat_type_int));
        let time_signature = TimeSignature::new_explicit(numerator, denominator);
        let direction_type = DirectionType::TimeSignatureChange { time_signature };
        if self.initial_time_signature.is_none() {
          self.initial_time_signature = Some(time_signature);
        }
        Some(StaffContent::Direction(Direction::new(direction_type)))
      }
      _ => None,
    }
  }
}

struct NoteHandler {
  base_beat_type: Duration,
  last_note_on_offset: u32,
  last_note_off_offset: u32,
  last_note_velocity: u8,
  ticks_per_beat: f64,
  rest_epsilon: f64,
}

impl NoteHandler {
  fn new(base_beat_type: Duration, ticks_per_beat: u16) -> Self {
    Self {
      base_beat_type,
      last_note_on_offset: 0,
      last_note_off_offset: 0,
      last_note_velocity: 0,
      ticks_per_beat: f64::from(ticks_per_beat),
      rest_epsilon: (f64::from(ticks_per_beat) * 0.125).ceil(),
    }
  }

  fn handle(&mut self, event: midly::MidiMessage, cur_time: u32) -> Option<NoteWrapper> {
    match event {
      midly::MidiMessage::NoteOn { key: _, vel } => {
        self.last_note_on_offset = cur_time;
        self.last_note_velocity = vel.as_int();
        if self.last_note_on_offset > self.last_note_off_offset
          && f64::from(self.last_note_on_offset - self.last_note_off_offset) >= self.rest_epsilon
        {
          let beat_length = f64::from(self.last_note_on_offset - self.last_note_off_offset) / self.ticks_per_beat;
          Some(Note::from_raw_note_data(255, beat_length, self.base_beat_type))
        } else {
          None
        }
      }
      midly::MidiMessage::NoteOff { key, vel: _ } => {
        self.last_note_off_offset = cur_time;
        let beat_length = f64::from(self.last_note_off_offset - self.last_note_on_offset) / self.ticks_per_beat;
        Some(Note::from_raw_note_data(key.as_int(), beat_length, self.base_beat_type))
      }
      _ => None,
    }
  }
}

pub struct MidiConverter;

impl MidiConverter {
  fn get_ticks_per_beat(header: midly::Header) -> u16 {
    match header.timing {
      midly::Timing::Metrical(ticks_per_beat) => ticks_per_beat.as_int(),
      midly::Timing::Timecode(..) => panic!("Timing format not supported"),
    }
  }

  fn parse_control_track(control_track: &Track) -> VecDeque<(StaffContent, TimeStamp)> {
    let mut cur_time = 0;
    let mut meta_handler = MetaHandler::new();
    let mut content = VecDeque::new();
    for event in control_track {
      cur_time += event.delta.as_int();
      if let midly::TrackEventKind::Meta(message) = event.kind {
        if let Some(staff_content) = meta_handler.get_staff_content(message) {
          content.push_back((staff_content, cur_time));
        }
      }
    }
    content
  }

  fn load_staff_content(
    staff: &mut Staff,
    mut control_track: VecDeque<(StaffContent, TimeStamp)>,
    track: &Track,
    ticks_per_beat: u16,
    base_beat_type: Duration,
  ) {
    let mut cur_time = 0;
    let mut meta_handler = MetaHandler::new();
    let mut note_handler = NoteHandler::new(base_beat_type, ticks_per_beat);

    for event in track {
      cur_time += event.delta.as_int();
      if let Some((_, time)) = control_track.front() {
        if *time >= cur_time {
          if let Some((content, _)) = control_track.pop_front() {
            staff.claim(content);
          }
        }
      }

      match event.kind {
        midly::TrackEventKind::Meta(message) => {
          if let Some(staff_content) = meta_handler.get_staff_content(message) {
            staff.claim(staff_content);
          }
        }
        midly::TrackEventKind::Midi { channel: _, message } => match note_handler.handle(message, cur_time) {
          Some(NoteWrapper::PlainNote(content)) => {
            staff.claim(content);
          }
          Some(NoteWrapper::TiedNote(contents)) => {
            for content in contents {
              staff.claim(content);
            }
          }
          None => {}
        },
        _ => {}
      }
    }
  }

  fn load_from_midi(data: &[u8]) -> Result<Composition, String> {
    // Parse the MIDI representation
    let midi = Smf::parse(data).map_err(|err| err.to_string())?;
    let ticks_per_beat = Self::get_ticks_per_beat(midi.header);
    let control_track = Self::parse_control_track(&midi.tracks[0]);
    let base_beat_type = Duration::new(DurationType::Quarter, 0);

    // Generate the composition structure and fill in musical data
    let mut composition = Composition::new("Default", None, None, None);
    let part = composition.add_part("MIDI Track");
    let section = part.add_section("Top-Level Section");
    for i in 1..midi.tracks.len() {
      let staff = section.add_staff(&format!("Section {i}"));
      Self::load_staff_content(
        staff,
        control_track.clone(),
        &midi.tracks[i],
        ticks_per_beat,
        base_beat_type,
      );
    }

    // Return the fully constructed composition
    Ok(composition)
  }
}

impl Load for MidiConverter {
  fn load(path: &str) -> Result<Composition, String> {
    let data = fs::read(path).map_err(|err| err.to_string())?;
    MidiConverter::load_from_midi(data.as_slice())
  }

  fn load_data(data: Vec<u8>) -> Result<Composition, String> {
    MidiConverter::load_from_midi(data.as_slice())
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::storage::Storage;

  #[test]
  fn test_midi_parser() {
    let composition = Storage::MIDI.load("tests/test_midi_files/test-2.mid");
    assert!(composition.is_ok());
  }

  #[test]
  fn test_midi_tie_note() {
    let beat_base_value = Duration::new(DurationType::Quarter, 0);

    let beat_length = 2.5;
    let tied = Duration::from_beats_tied(&beat_base_value, beat_length);
    assert_eq!(tied.len(), 2);
    assert_eq!(tied[0].value, DurationType::Half);
    assert_eq!(tied[1].value, DurationType::Eighth);

    let beat_length = 5.0;
    let tied = Duration::from_beats_tied(&beat_base_value, beat_length);
    assert_eq!(tied.len(), 2);
    assert_eq!(tied[0].value, DurationType::Whole);
    assert_eq!(tied[1].value, DurationType::Quarter);

    let beat_length = 1.25;
    let tied = Duration::from_beats_tied(&beat_base_value, beat_length);
    assert_eq!(tied.len(), 2);
    assert_eq!(tied[0].value, DurationType::Quarter);
    assert_eq!(tied[1].value, DurationType::Sixteenth);

    let beat_length = 5.25;
    let tied = Duration::from_beats_tied(&beat_base_value, beat_length);
    assert_eq!(tied.len(), 3);
    assert_eq!(tied[0].value, DurationType::Whole);
    assert_eq!(tied[1].value, DurationType::Quarter);
    assert_eq!(tied[2].value, DurationType::Sixteenth);

    let beat_length = 3.0;
    let tied = Duration::from_beats_tied(&beat_base_value, beat_length);
    assert_eq!(tied.len(), 1);
    assert_eq!(tied[0].value, DurationType::Half);
    assert_eq!(tied[0].dots, 1);
  }
}
