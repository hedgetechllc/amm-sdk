use super::Load;
use crate::context::{Key, KeyMode, Tempo, TimeSignature};
use crate::modification::{Direction, DirectionType};
use crate::note::{Duration, DurationType, Note};
use crate::structure::{Staff, StaffContent};
use crate::Composition;
use alloc::{collections::VecDeque, string::String};
use core::str;
use midly::{MetaMessage, Smf, Track};
use std::fs;

type TimeStamp = u32;

enum NoteWrapper {
  PlainNote(StaffContent),
  TiedNote(Vec<StaffContent>),
}

impl Note {
  fn from_raw_note_data(midi_number: u8, beat_length: f64, beat_base_value: Duration, key: Key) -> NoteWrapper {
    let mut note = Note::from_midi(midi_number, beat_base_value, Some(key));
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

#[derive(Clone, Debug)]
enum MetaContent {
  StaffContent(StaffContent),
  TempoChange(Tempo),
  NewSection(String),
  KeyChange(Key),
}

struct MetaHandler {
  copyright: Option<String>,
  initial_tempo: Option<Tempo>,
  initial_time_signature: Option<TimeSignature>,
  initial_key: Option<Key>,
}

impl MetaHandler {
  fn new() -> Self {
    Self {
      copyright: None,
      initial_tempo: None,
      initial_time_signature: None,
      initial_key: None,
    }
  }

  fn handle(&mut self, message: midly::MetaMessage) -> Option<MetaContent> {
    match message {
      MetaMessage::KeySignature(fifths, minor) => {
        let mode = if minor { KeyMode::Minor } else { KeyMode::Major };
        let key = Key::from_fifths(fifths, Some(mode));
        if self.initial_key.is_none() {
          self.initial_key = Some(key);
        }
        Some(MetaContent::KeyChange(key))
      }
      MetaMessage::TimeSignature(numerator, beat_type_int, _, _) => {
        let denominator = 2u8.pow(u32::from(beat_type_int));
        let time_signature = TimeSignature::new_explicit(numerator, denominator);
        let direction_type = DirectionType::TimeSignatureChange { time_signature };
        if self.initial_time_signature.is_none() {
          self.initial_time_signature = Some(time_signature);
        }
        Some(MetaContent::StaffContent(StaffContent::Direction(Direction::new(
          direction_type,
        ))))
      }
      MetaMessage::Copyright(copyright) => {
        if let Ok(copyright) = String::from_utf8(copyright.to_vec()) {
          self.copyright = Some(copyright);
        }
        None
      }
      MetaMessage::Marker(marker) => String::from_utf8(marker.to_vec()).ok().map(MetaContent::NewSection),
      MetaMessage::Tempo(us_per_quarter_note) => {
        let bpm = u16::try_from(60_000_000 / us_per_quarter_note.as_int()).unwrap_or(120);
        let tempo = Tempo::new(Duration::new(DurationType::Quarter, 0), bpm);
        if self.initial_tempo.is_none() {
          self.initial_tempo = Some(tempo);
        }
        Some(MetaContent::TempoChange(tempo))
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

  fn handle(&mut self, event: midly::MidiMessage, cur_time: u32, current_key: Key) -> Option<NoteWrapper> {
    match event {
      midly::MidiMessage::NoteOn { key: _, vel } => {
        self.last_note_on_offset = cur_time;
        self.last_note_velocity = vel.as_int();
        if self.last_note_on_offset > self.last_note_off_offset
          && f64::from(self.last_note_on_offset - self.last_note_off_offset) >= self.rest_epsilon
        {
          let beat_length = f64::from(self.last_note_on_offset - self.last_note_off_offset) / self.ticks_per_beat;
          Some(Note::from_raw_note_data(
            255,
            beat_length,
            self.base_beat_type,
            current_key,
          ))
        } else {
          None
        }
      }
      midly::MidiMessage::NoteOff { key, vel: _ } => {
        self.last_note_off_offset = cur_time;
        let beat_length = f64::from(self.last_note_off_offset - self.last_note_on_offset) / self.ticks_per_beat;
        Some(Note::from_raw_note_data(
          key.as_int(),
          beat_length,
          self.base_beat_type,
          current_key,
        ))
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

  fn get_starting_key(tracks: &[Track]) -> Key {
    for track in tracks {
      for event in track {
        if let midly::TrackEventKind::Meta(MetaMessage::KeySignature(fifths, minor)) = event.kind {
          let mode = if minor { KeyMode::Minor } else { KeyMode::Major };
          return Key::from_fifths(fifths, Some(mode));
        }
      }
    }
    Key::default()
  }

  fn get_track_name(track_index: usize, track: &Track) -> String {
    // TODO: Try to infer part name from instrument number first (if available)
    let mut track_name = String::new();
    for event in track {
      if let midly::TrackEventKind::Meta(message) = event.kind {
        if let MetaMessage::TrackName(name) = message {
          if let Ok(name) = String::from_utf8(name.to_vec()) {
            if track_name.is_empty() && !name.is_empty() {
              track_name = name;
            }
          }
        } else if let MetaMessage::InstrumentName(name) = message {
          if let Ok(mut name) = String::from_utf8(name.to_vec()) {
            if !name.is_empty() {
              name.get_mut(0..1).map(|c| {
                c.make_ascii_uppercase();
                &*c
              });
              track_name = name;
              break;
            }
          }
        }
      }
    }
    if track_name.is_empty() {
      String::from("MIDI Track ") + &track_index.to_string()
    } else {
      track_name
    }
  }

  fn parse_control_track(composition: &mut Composition, control_track: &Track) -> VecDeque<(MetaContent, TimeStamp)> {
    // Parse the control track for all metadata and context changes
    let mut cur_time = 0;
    let mut meta_handler = MetaHandler::new();
    let mut content = VecDeque::new();
    for event in control_track {
      cur_time += event.delta.as_int();
      if let midly::TrackEventKind::Meta(message) = event.kind {
        if let Some(meta_content) = meta_handler.handle(message) {
          content.push_back((meta_content, cur_time));
        }
      }
    }

    // Fill in any top-level metadata for the composition
    if let Some(tempo) = meta_handler.initial_tempo {
      composition.set_tempo(tempo);
    }
    if let Some(starting_time_signature) = meta_handler.initial_time_signature {
      composition.set_starting_time_signature(starting_time_signature);
    }
    if let Some(copyright) = meta_handler.copyright {
      composition.set_copyright(&copyright);
    }

    // Return all time-based contextual content
    content
  }

  fn handle_meta_content(staff: &mut Staff, current_key: &mut Key, meta_content: MetaContent) {
    match meta_content {
      MetaContent::StaffContent(content) => {
        staff.claim(content);
      }
      MetaContent::TempoChange(_tempo) => {
        {}; // TODO: Implement tempo change (use new section)
      }
      MetaContent::NewSection(_name) => {
        {}; // TODO: Implement new section
      }
      MetaContent::KeyChange(key) => {
        *current_key = key;
        staff.add_direction(DirectionType::KeyChange { key });
      }
    }
  }

  fn load_staff_content(
    staff: &mut Staff,
    mut context_changes: VecDeque<(MetaContent, TimeStamp)>,
    track: &Track,
    ticks_per_beat: u16,
    base_beat_type: Duration,
    mut current_key: Key,
  ) {
    // Iterate through all track events
    let mut cur_time = 0;
    let mut meta_handler = MetaHandler::new();
    let mut note_handler = NoteHandler::new(base_beat_type, ticks_per_beat);
    for event in track {
      // Check if any musical context changes are needed at the current timestamp
      cur_time += event.delta.as_int();
      if let Some(final_change_idx) = context_changes
        .iter()
        .position(|(_, change_time)| cur_time < *change_time)
      {
        for _ in 0..final_change_idx {
          if let Some((meta_content, _)) = context_changes.pop_front() {
            Self::handle_meta_content(staff, &mut current_key, meta_content);
          }
        }
      } else {
        while let Some((meta_content, _)) = context_changes.pop_front() {
          Self::handle_meta_content(staff, &mut current_key, meta_content);
        }
      }

      // Handle the next musical event in the track
      match event.kind {
        midly::TrackEventKind::Meta(message) => {
          if let Some(meta_content) = meta_handler.handle(message) {
            Self::handle_meta_content(staff, &mut current_key, meta_content);
          }
        }
        midly::TrackEventKind::Midi { channel: _, message } => {
          match note_handler.handle(message, cur_time, current_key) {
            Some(NoteWrapper::PlainNote(content)) => {
              staff.claim(content);
            }
            Some(NoteWrapper::TiedNote(contents)) => {
              for content in contents {
                staff.claim(content);
              }
            }
            None => {}
          }
        }
        _ => {}
      }
    }
  }

  fn load_from_midi(data: &[u8]) -> Result<Composition, String> {
    // Parse the MIDI representation
    let midi = Smf::parse(data).map_err(|err| err.to_string())?;
    let starting_key = Self::get_starting_key(&midi.tracks);
    let ticks_per_beat = Self::get_ticks_per_beat(midi.header);
    let base_beat_type = Duration::new(DurationType::Quarter, 0);

    // Generate the composition structure and parse the control track for metadata
    let mut composition = Composition::new("Untitled", None, Some(starting_key), None);
    let control_track = Self::parse_control_track(&mut composition, &midi.tracks[0]);

    // Parse the MIDI tracks and fill in all musical data
    for idx in 1..midi.tracks.len() {
      let part = composition.add_part(&Self::get_track_name(idx, &midi.tracks[idx]));
      // TODO: If part name already exists, retrieve existing part and add to it
      let section = part.add_section("Top-Level Section");
      let staff = section.add_staff("1");
      Self::load_staff_content(
        staff,
        control_track.clone(),
        &midi.tracks[idx],
        ticks_per_beat,
        base_beat_type,
        starting_key,
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
    let composition = Storage::MIDI.load("tests/test_midi_files/test-1.mid");
    assert!(composition.is_ok());
    /*if let Ok(composition) = composition {
      println!("{composition}");
      for part in composition {
        println!("{part}");
      }
    }*/
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

// TODO: Implement chords
// TODO: Implement tuplets
// TODO: Attempt to implement dynamics
// TODO: Attempt to implement mordents, trills, and other ornaments based on timing data
