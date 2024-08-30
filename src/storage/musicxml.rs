use super::Convert;
use crate::{
  Accidental, Chord, ChordContent, ChordModification, ChordModificationType, Clef, ClefSymbol, ClefType, Composition,
  Direction, DirectionType, Duration, DurationType, Dynamic, DynamicMarking, HandbellTechnique, Key, KeyMode,
  KeySignature, MultiVoice, Note, NoteModification, NoteModificationType, PedalType, Phrase, PhraseContent,
  PhraseModification, PhraseModificationType, Pitch, PitchName, Section, SectionModificationType, Staff, StaffContent,
  Tempo, TempoMarking, TempoSuggestion, TimeSignature, TimeSignatureType,
};
use alloc::{
  collections::BTreeMap,
  rc::Rc,
  string::{String, ToString},
  vec::Vec,
};
use core::{cell::RefCell, str};
use musicxml::{self, elements::ScorePartwise};

pub struct MusicXmlConverter;

#[derive(Clone)]
struct PhraseModDetails {
  pub modification: PhraseModificationType,
  pub is_start: bool,
  pub number: Option<u8>,
  pub for_voice: Option<String>,
}

#[cfg(feature = "print")]
impl core::fmt::Display for PhraseModDetails {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{} {} (ID: {}{})",
      if self.is_start { "Start" } else { "End" },
      self.modification,
      self.number.unwrap_or(0),
      match self.for_voice {
        Some(ref voice) => format!(", Voice: {voice}"),
        None => String::new(),
      }
    )
  }
}

#[derive(Clone)]
struct NoteDetails {
  pub pitch: Pitch,
  pub duration: Duration,
  pub accidental: Accidental,
  pub voice: Option<String>,
  pub arpeggiated: bool,
  pub non_arpeggiated: bool,
  pub note_modifications: Vec<NoteModificationType>,
  pub phrase_modifications_start: Vec<PhraseModDetails>,
  pub phrase_modifications_end: Vec<PhraseModDetails>,
}

#[cfg(feature = "print")]
impl core::fmt::Display for NoteDetails {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let note_mods = self
      .note_modifications
      .iter()
      .map(|note_mod| format!("{note_mod}"))
      .collect::<Vec<String>>()
      .join(", ");
    let mut phrase_modifications = self
      .phrase_modifications_start
      .iter()
      .map(|phrase_mod| format!("{phrase_mod}"))
      .collect::<Vec<String>>();
    phrase_modifications.extend(
      self
        .phrase_modifications_end
        .iter()
        .map(|phrase_mod| format!("{phrase_mod}")),
    );
    let phrase_mods = phrase_modifications.join(", ");
    write!(
      f,
      "{}: {}{}{}{} ({}{}{}{}{}{}{}{}{}{} )",
      if self.pitch.is_rest() { "Rest" } else { "Note" },
      self.pitch,
      self.accidental,
      if self.pitch.is_rest() { "" } else { " " },
      self.duration,
      if self.voice.is_some() { " Voice=" } else { "" },
      if self.voice.is_some() {
        self.voice.clone().unwrap()
      } else {
        String::new()
      },
      if self.arpeggiated { " Arpeggiated" } else { "" },
      if self.non_arpeggiated { " NonArpeggiated" } else { "" },
      if note_mods.is_empty() { "" } else { " Mods=[" },
      if note_mods.is_empty() { "" } else { note_mods.as_str() },
      if note_mods.is_empty() { "" } else { "]" },
      if phrase_mods.is_empty() { "" } else { " PhraseMods=[" },
      if phrase_mods.is_empty() {
        ""
      } else {
        phrase_mods.as_str()
      },
      if phrase_mods.is_empty() { "" } else { "]" },
    )
  }
}

#[derive(Default, Clone)]
struct TimeSliceContainer {
  pub direction: Vec<DirectionType>,
  pub chord_modification: Vec<ChordModificationType>,
  pub phrase_modification_start: Vec<PhraseModDetails>,
  pub phrase_modification_end: Vec<PhraseModDetails>,
  pub jump_to: Vec<String>,
  pub section_start: Vec<String>,
  pub ending: Vec<(bool, Vec<u8>)>,
  pub repeat: Vec<(bool, u32)>,
  pub tempo_change_explicit: Vec<Tempo>,
  pub tempo_change_implicit: Vec<TempoSuggestion>,
  pub notes: Vec<NoteDetails>,
}

impl TimeSliceContainer {
  pub fn is_empty(&self) -> bool {
    self.direction.is_empty()
      && self.chord_modification.is_empty()
      && self.phrase_modification_start.is_empty()
      && self.phrase_modification_end.is_empty()
      && self.jump_to.is_empty()
      && self.section_start.is_empty()
      && self.ending.is_empty()
      && self.repeat.is_empty()
      && self.tempo_change_explicit.is_empty()
      && self.tempo_change_implicit.is_empty()
      && self.notes.is_empty()
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for TimeSliceContainer {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let mut description = self
      .direction
      .iter()
      .map(|item| format!("\"{item}\""))
      .collect::<Vec<String>>();
    description.extend(
      self
        .chord_modification
        .iter()
        .map(|item| format!("\"Chord Modification: {item}\"")),
    );
    description.extend(
      self
        .phrase_modification_start
        .iter()
        .map(|item| format!("\"Phrase Modification: {item}\"")),
    );
    description.extend(
      self
        .phrase_modification_end
        .iter()
        .map(|item| format!("\"Phrase Modification: {item}\"")),
    );
    description.extend(self.jump_to.iter().map(|item| format!("\"Jump To: {item}\"")));
    description.extend(
      self
        .section_start
        .iter()
        .map(|item| format!("\"Section Start: {item}\"")),
    );
    description.extend(self.ending.iter().map(|(start, numbers)| {
      format!(
        "\"Ending: Start={start} Iterations=[{}]\"",
        numbers
          .iter()
          .map(ToString::to_string)
          .collect::<Vec<String>>()
          .join(", ")
      )
    }));
    description.extend(
      self
        .repeat
        .iter()
        .map(|(start, times)| format!("\"{} Repeat {times} Times\"", if *start { "Start" } else { "End" })),
    );
    description.extend(
      self
        .tempo_change_explicit
        .iter()
        .map(|item| format!("\"Tempo Change: {item}\"")),
    );
    description.extend(
      self
        .tempo_change_implicit
        .iter()
        .map(|item| format!("\"Tempo Change: {item}\"")),
    );
    description.extend(self.notes.iter().map(|item| format!("\"{item}\"")));
    let desc = description.join(", ");
    write!(f, "{desc}")
  }
}

struct TemporalPartData {
  pub data: BTreeMap<String, BTreeMap<String, Vec<TimeSliceContainer>>>,
}

#[cfg(feature = "print")]
impl core::fmt::Display for TemporalPartData {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    for (part_name, staves) in &self.data {
      write!(f, "\nPart: {part_name}")?;
      for (staff_name, time_slices) in staves {
        writeln!(f, "\n  Staff: {staff_name}")?;
        for (time, time_slice) in time_slices.iter().enumerate() {
          if !time_slice.is_empty() {
            write!(f, "    Time: {time}\n      Items: [ {time_slice} ]\n")?;
          }
        }
      }
    }
    Ok(())
  }
}

impl MusicXmlConverter {
  #[allow(clippy::cast_possible_truncation)]
  fn calculate_num_dots(base_divisions: usize, total_divisions: usize) -> u8 {
    let (mut num_dots, mut remaining_divisions) = (0, total_divisions - base_divisions);
    while remaining_divisions > 0 {
      num_dots += 1;
      remaining_divisions -= base_divisions / (2_usize.pow(num_dots));
    }
    num_dots as u8
  }

  fn find_starting_key(parts: &Vec<musicxml::elements::Part>) -> Key {
    for part in parts {
      if let musicxml::elements::PartElement::Measure(measure) = &part.content[0] {
        for measure_element in &measure.content {
          if let musicxml::elements::MeasureElement::Attributes(attributes) = measure_element {
            for key_element in &attributes.content.key {
              if let musicxml::elements::KeyContents::Explicit(key) = &key_element.content {
                let mode = match &key.mode {
                  Some(mode) => match mode.content {
                    musicxml::datatypes::Mode::Minor => KeyMode::Minor,
                    _ => KeyMode::Major,
                  },
                  None => KeyMode::Major,
                };
                return Key::from_fifths(*key.fifths.content, Some(mode));
              }
            }
          }
        }
      }
    }
    Key::default()
  }

  fn find_starting_time_signature(parts: &Vec<musicxml::elements::Part>) -> TimeSignature {
    for part in parts {
      if let musicxml::elements::PartElement::Measure(measure) = &part.content[0] {
        for measure_element in &measure.content {
          if let musicxml::elements::MeasureElement::Attributes(attributes) = measure_element {
            for time_element in &attributes.content.time {
              for beat_element in &time_element.content.beats {
                return TimeSignature::new_explicit(
                  (*beat_element.beats.content).parse().unwrap(),
                  (*beat_element.beat_type.content).parse().unwrap(),
                );
              }
            }
          }
        }
      }
    }
    TimeSignature::default()
  }

  #[allow(clippy::cast_possible_truncation)]
  fn parse_tempo_from_metronome(metronome: &musicxml::elements::Metronome) -> Option<Tempo> {
    if let musicxml::elements::MetronomeContents::BeatBased(beat_data) = &metronome.content {
      let num_dots: u8 = beat_data.beat_unit_dot.len() as u8;
      let base_note = match beat_data.beat_unit.content {
        musicxml::datatypes::NoteTypeValue::Maxima => Duration::new(DurationType::Maxima, num_dots),
        musicxml::datatypes::NoteTypeValue::Long => Duration::new(DurationType::Long, num_dots),
        musicxml::datatypes::NoteTypeValue::Breve => Duration::new(DurationType::Breve, num_dots),
        musicxml::datatypes::NoteTypeValue::Whole => Duration::new(DurationType::Whole, num_dots),
        musicxml::datatypes::NoteTypeValue::Half => Duration::new(DurationType::Half, num_dots),
        musicxml::datatypes::NoteTypeValue::Quarter => Duration::new(DurationType::Quarter, num_dots),
        musicxml::datatypes::NoteTypeValue::Eighth => Duration::new(DurationType::Eighth, num_dots),
        musicxml::datatypes::NoteTypeValue::Sixteenth => Duration::new(DurationType::Sixteenth, num_dots),
        musicxml::datatypes::NoteTypeValue::ThirtySecond => Duration::new(DurationType::ThirtySecond, num_dots),
        musicxml::datatypes::NoteTypeValue::SixtyFourth => Duration::new(DurationType::SixtyFourth, num_dots),
        musicxml::datatypes::NoteTypeValue::OneHundredTwentyEighth => {
          Duration::new(DurationType::OneHundredTwentyEighth, num_dots)
        }
        musicxml::datatypes::NoteTypeValue::TwoHundredFiftySixth => {
          Duration::new(DurationType::TwoHundredFiftySixth, num_dots)
        }
        musicxml::datatypes::NoteTypeValue::FiveHundredTwelfth => {
          Duration::new(DurationType::FiveHundredTwelfth, num_dots)
        }
        musicxml::datatypes::NoteTypeValue::OneThousandTwentyFourth => {
          Duration::new(DurationType::OneThousandTwentyFourth, num_dots)
        }
      };
      if let musicxml::elements::BeatEquation::BPM(per_minute) = &beat_data.equals {
        return Some(Tempo {
          base_note,
          beats_per_minute: per_minute.content.parse().unwrap(),
        });
      };
    }
    None
  }

  fn parse_tempo_from_sound(sound: &musicxml::elements::Sound) -> Option<Tempo> {
    if let Some(tempo) = &sound.attributes.tempo {
      let bpm = **tempo as u16;
      if bpm > 0 {
        return Some(Tempo {
          base_note: Duration::new(DurationType::Quarter, 0),
          beats_per_minute: bpm,
        });
      }
    }
    None
  }

  fn find_tempo(parts: &Vec<musicxml::elements::Part>) -> Tempo {
    for part in parts {
      if let musicxml::elements::PartElement::Measure(measure) = &part.content[0] {
        for measure_element in &measure.content {
          if let musicxml::elements::MeasureElement::Direction(direction) = measure_element {
            for direction_type in &direction.content.direction_type {
              // Attempt to find a metronome marking
              if let musicxml::elements::DirectionTypeContents::Metronome(metronome) = &direction_type.content {
                if let Some(result) = Self::parse_tempo_from_metronome(metronome) {
                  return result;
                }
              }
            }

            // If no metronome marking was found, attempt to find a sound direction
            if let Some(sound) = &direction.content.sound {
              if let Some(result) = Self::parse_tempo_from_sound(sound) {
                return result;
              }
            }
          }
        }
      }
    }
    Tempo::default()
  }

  fn find_metadata(composition: &mut Composition, metadata_contents: &musicxml::elements::ScorePartwiseContents) {
    if let Some(work) = &metadata_contents.work {
      if let Some(work_number) = &work.content.work_number {
        composition.add_metadata("opus_number", work_number.content.as_str());
      }
    }
    if let Some(movement_number) = &metadata_contents.movement_number {
      composition.add_metadata("movement_number", movement_number.content.as_str());
    }
    if let Some(movement_title) = &metadata_contents.movement_title {
      composition.add_metadata("movement_title", movement_title.content.as_str());
    }
    if let Some(identification) = &metadata_contents.identification {
      for creator in &identification.content.creator {
        if let Some(creator_type) = &creator.attributes.r#type {
          match (**creator_type).as_str() {
            "composer" => composition.add_composer(creator.content.as_str()),
            "lyricist" => composition.add_lyricist(creator.content.as_str()),
            "arranger" => composition.add_arranger(creator.content.as_str()),
            "publisher" => composition.set_publisher(creator.content.as_str()),
            other => composition.add_metadata(other, creator.content.as_str()),
          };
        } else {
          composition.add_metadata("creator", creator.content.as_str());
        }
      }
      for rights in &identification.content.rights {
        if let Some(rights_type) = &rights.attributes.r#type {
          match (**rights_type).as_str() {
            "copyright" => composition.set_copyright(rights.content.as_str()),
            other => composition.add_metadata(other, rights.content.as_str()),
          };
        } else {
          composition.set_copyright(rights.content.as_str());
        }
      }
    }
  }

  fn find_parts(parts_list: &Vec<musicxml::elements::PartListElement>) -> BTreeMap<String, String> {
    let mut parts_map: BTreeMap<String, String> = BTreeMap::new();
    for parts_list_element in parts_list {
      if let musicxml::elements::PartListElement::ScorePart(score_part) = parts_list_element {
        parts_map.insert(
          (*score_part.attributes.id).clone(),
          score_part.content.part_name.content.clone(),
        );
      }
    }
    parts_map
  }

  fn find_staves(part_elements: &Vec<musicxml::elements::PartElement>) -> Vec<String> {
    for element in part_elements {
      if let musicxml::elements::PartElement::Measure(measure) = element {
        for measure_element in &measure.content {
          if let musicxml::elements::MeasureElement::Attributes(attributes) = measure_element {
            if let Some(staves) = &attributes.content.staves {
              return (1..=*staves.content).map(|staff| staff.to_string()).collect();
            }
          }
        }
      }
    }
    vec![String::from("1")]
  }

  fn find_num_measures(part_elements: &[musicxml::elements::PartElement]) -> usize {
    part_elements.len()
  }

  fn find_divisions_per_quarter_note(part_elements: &Vec<musicxml::elements::PartElement>) -> usize {
    for element in part_elements {
      if let musicxml::elements::PartElement::Measure(measure) = element {
        for measure_element in &measure.content {
          if let musicxml::elements::MeasureElement::Attributes(attributes) = measure_element {
            if let Some(divisions_elements) = &attributes.content.divisions {
              return *divisions_elements.content as usize;
            }
          }
        }
      }
    }
    4
  }

  fn find_max_num_quarter_notes_per_measure(part_elements: &Vec<musicxml::elements::PartElement>) -> usize {
    let mut max_quarter_notes: u32 = 1;
    for element in part_elements {
      if let musicxml::elements::PartElement::Measure(measure) = element {
        for measure_element in &measure.content {
          if let musicxml::elements::MeasureElement::Attributes(attributes) = measure_element {
            for time_element in &attributes.content.time {
              for beat_element in &time_element.content.beats {
                let num_quarter_notes = (((*beat_element.beats.content).parse::<f32>().unwrap() * 4.0f32
                  / (*beat_element.beat_type.content).parse::<f32>().unwrap())
                  + 0.5f32) as u32;
                max_quarter_notes = max_quarter_notes.max(num_quarter_notes);
              }
            }
          }
        }
      }
    }
    max_quarter_notes as usize
  }

  fn parse_attributes_element(
    element: &musicxml::elements::AttributesContents,
    time_slices: &mut BTreeMap<String, Vec<TimeSliceContainer>>,
    cursor: usize,
  ) -> isize {
    element.clef.iter().for_each(|item| {
      let staff_name = if let Some(number) = &item.attributes.number {
        number.to_string()
      } else {
        String::from("1")
      };
      let item = DirectionType::Clef {
        clef: match &item.content.sign.content {
          musicxml::datatypes::ClefSign::G => match &item.content.line {
            Some(line) => match *line.content {
              1 => Clef::new(ClefType::FrenchViolin),
              _ => Clef::new(ClefType::Treble),
            },
            None => Clef::new(ClefType::Treble),
          },
          musicxml::datatypes::ClefSign::F => match &item.content.line {
            Some(line) => match *line.content {
              3 => Clef::new(ClefType::BaritoneF),
              5 => Clef::new(ClefType::Subbass),
              _ => Clef::new(ClefType::Bass),
            },
            None => Clef::new(ClefType::Bass),
          },
          musicxml::datatypes::ClefSign::C => match &item.content.line {
            Some(line) => match *line.content {
              1 => Clef::new(ClefType::Soprano),
              2 => Clef::new(ClefType::MezzoSoprano),
              4 => Clef::new(ClefType::Tenor),
              5 => Clef::new(ClefType::BaritoneC),
              _ => Clef::new(ClefType::Alto),
            },
            None => Clef::new(ClefType::Alto),
          },
          _ => Clef::new(ClefType::Treble),
        },
      };
      time_slices.get_mut(&staff_name).unwrap()[cursor].direction.push(item);
    });
    element.key.iter().for_each(|item| {
      if let musicxml::elements::KeyContents::Explicit(key) = &item.content {
        let staff_name = if let Some(number) = &item.attributes.number {
          number.to_string()
        } else {
          String::from("1")
        };
        let mode = match &key.mode {
          Some(mode) => match mode.content {
            musicxml::datatypes::Mode::Minor => KeyMode::Minor,
            _ => KeyMode::Major,
          },
          None => KeyMode::Major,
        };
        let item = DirectionType::Key {
          key: Key::from_fifths(*key.fifths.content, Some(mode)),
        };
        time_slices.get_mut(&staff_name).unwrap()[cursor].direction.push(item);
      }
    });
    element.time.iter().for_each(|item| {
      let staff_name = if let Some(number) = &item.attributes.number {
        number.to_string()
      } else {
        String::from("1")
      };
      let item = if item.content.senza_misura.is_some() {
        DirectionType::TimeSignature {
          time_signature: TimeSignature::new(TimeSignatureType::None),
        }
      } else {
        let beat_element = &item.content.beats[0];
        DirectionType::TimeSignature {
          time_signature: TimeSignature::new_explicit(
            (*beat_element.beats.content).parse().unwrap(),
            (*beat_element.beat_type.content).parse().unwrap(),
          ),
        }
      };
      time_slices.get_mut(&staff_name).unwrap()[cursor].direction.push(item);
    });
    0
  }

  fn parse_backup_element(element: &musicxml::elements::BackupContents) -> isize {
    -(*element.duration.content as isize)
  }

  fn parse_forward_element(element: &musicxml::elements::ForwardContents) -> isize {
    *element.duration.content as isize
  }

  fn parse_direction_element(
    element: &musicxml::elements::Direction,
    time_slice: &mut BTreeMap<String, Vec<TimeSliceContainer>>,
    cursor: usize,
  ) -> isize {
    let staff_name = if let Some(staff) = &element.content.staff {
      staff.content.to_string()
    } else {
      String::from("1")
    };
    element
      .content
      .direction_type
      .iter()
      .for_each(|item| match &item.content {
        musicxml::elements::DirectionTypeContents::Rehearsal(rehearsal) => {
          time_slice.get_mut(&staff_name).unwrap()[cursor]
            .section_start
            .push(rehearsal[0].content.clone());
        }
        musicxml::elements::DirectionTypeContents::Segno(_segno) => {
          time_slice.get_mut(&staff_name).unwrap()[cursor]
            .section_start
            .push(String::from("Segno"));
        }
        musicxml::elements::DirectionTypeContents::Coda(_coda) => {
          time_slice.get_mut(&staff_name).unwrap()[cursor]
            .section_start
            .push(String::from("Coda"));
        }
        musicxml::elements::DirectionTypeContents::Wedge(wedge) => {
          if wedge.attributes.r#type != musicxml::datatypes::WedgeType::Continue {
            let item = PhraseModDetails {
              modification: match wedge.attributes.r#type {
                musicxml::datatypes::WedgeType::Diminuendo => PhraseModificationType::Decrescendo {
                  final_dynamic: Dynamic::new(DynamicMarking::None, 0),
                },
                _ => PhraseModificationType::Crescendo {
                  final_dynamic: Dynamic::new(DynamicMarking::None, 0),
                },
              },
              is_start: wedge.attributes.r#type != musicxml::datatypes::WedgeType::Stop,
              number: wedge.attributes.number.as_ref().map(|number| **number),
              for_voice: None,
            };
            if item.is_start {
              time_slice.get_mut(&staff_name).unwrap()[cursor]
                .phrase_modification_start
                .push(item);
            } else {
              time_slice.get_mut(&staff_name).unwrap()[cursor]
                .phrase_modification_end
                .push(item);
            }
          }
        }
        musicxml::elements::DirectionTypeContents::Dynamics(dynamics) => {
          let dynamic_type = match &dynamics[0].content[0] {
            musicxml::elements::DynamicsType::P(_) => Some(Dynamic::new(DynamicMarking::Piano, 1)),
            musicxml::elements::DynamicsType::Pp(_) => Some(Dynamic::new(DynamicMarking::Piano, 2)),
            musicxml::elements::DynamicsType::Ppp(_) => Some(Dynamic::new(DynamicMarking::Piano, 3)),
            musicxml::elements::DynamicsType::Pppp(_) => Some(Dynamic::new(DynamicMarking::Piano, 4)),
            musicxml::elements::DynamicsType::Ppppp(_) => Some(Dynamic::new(DynamicMarking::Piano, 5)),
            musicxml::elements::DynamicsType::Pppppp(_) => Some(Dynamic::new(DynamicMarking::Piano, 6)),
            musicxml::elements::DynamicsType::F(_) => Some(Dynamic::new(DynamicMarking::Forte, 1)),
            musicxml::elements::DynamicsType::Ff(_) => Some(Dynamic::new(DynamicMarking::Forte, 2)),
            musicxml::elements::DynamicsType::Fff(_) => Some(Dynamic::new(DynamicMarking::Forte, 3)),
            musicxml::elements::DynamicsType::Ffff(_) => Some(Dynamic::new(DynamicMarking::Forte, 4)),
            musicxml::elements::DynamicsType::Fffff(_) => Some(Dynamic::new(DynamicMarking::Forte, 5)),
            musicxml::elements::DynamicsType::Ffffff(_) => Some(Dynamic::new(DynamicMarking::Forte, 6)),
            musicxml::elements::DynamicsType::Mp(_) => Some(Dynamic::new(DynamicMarking::MezzoPiano, 0)),
            musicxml::elements::DynamicsType::Mf(_) => Some(Dynamic::new(DynamicMarking::MezzoForte, 0)),
            musicxml::elements::DynamicsType::N(_) | musicxml::elements::DynamicsType::OtherDynamics(_) => None,
            _ => Some(Dynamic::new(DynamicMarking::None, 0)),
          };
          if let Some(dynamic_type) = dynamic_type {
            if dynamic_type == Dynamic::new(DynamicMarking::None, 0) {
              time_slice.get_mut(&staff_name).unwrap()[cursor]
                .chord_modification
                .push(ChordModificationType::Accent);
            } else {
              time_slice.get_mut(&staff_name).unwrap()[cursor]
                .direction
                .push(DirectionType::Dynamic { dynamic: dynamic_type });
            }
          }
        }
        musicxml::elements::DirectionTypeContents::Pedal(pedal) => match &pedal.attributes.r#type {
          musicxml::datatypes::PedalType::Start => {
            let item = PhraseModDetails {
              modification: PhraseModificationType::Pedal {
                r#type: PedalType::Sustain,
              },
              is_start: true,
              number: pedal.attributes.number.as_ref().map(|number| **number),
              for_voice: None,
            };
            time_slice.get_mut(&staff_name).unwrap()[cursor]
              .phrase_modification_start
              .push(item);
          }
          musicxml::datatypes::PedalType::Stop => {
            let item = PhraseModDetails {
              modification: PhraseModificationType::Pedal {
                r#type: PedalType::Sustain,
              },
              is_start: false,
              number: pedal.attributes.number.as_ref().map(|number| **number),
              for_voice: None,
            };
            time_slice.get_mut(&staff_name).unwrap()[cursor]
              .phrase_modification_end
              .push(item);
          }
          musicxml::datatypes::PedalType::Sostenuto => {
            let item = PhraseModDetails {
              modification: PhraseModificationType::Pedal {
                r#type: PedalType::Sostenuto,
              },
              is_start: true,
              number: pedal.attributes.number.as_ref().map(|number| **number),
              for_voice: None,
            };
            time_slice.get_mut(&staff_name).unwrap()[cursor]
              .phrase_modification_start
              .push(item);
          }
          musicxml::datatypes::PedalType::Change => {
            let item1 = PhraseModDetails {
              modification: PhraseModificationType::Pedal {
                r#type: PedalType::Sustain,
              },
              is_start: false,
              number: pedal.attributes.number.as_ref().map(|number| **number),
              for_voice: None,
            };
            let item2 = PhraseModDetails {
              modification: PhraseModificationType::Pedal {
                r#type: PedalType::Sustain,
              },
              is_start: true,
              number: pedal.attributes.number.as_ref().map(|number| **number),
              for_voice: None,
            };
            time_slice.get_mut(&staff_name).unwrap()[cursor]
              .phrase_modification_end
              .push(item1);
            time_slice.get_mut(&staff_name).unwrap()[cursor]
              .phrase_modification_start
              .push(item2);
          }
          _ => (),
        },
        musicxml::elements::DirectionTypeContents::OctaveShift(octave_shift) => {
          if octave_shift.attributes.r#type != musicxml::datatypes::UpDownStopContinue::Continue {
            let item = PhraseModDetails {
              modification: PhraseModificationType::OctaveShift {
                num_octaves: match &octave_shift.attributes.size {
                  Some(musicxml::datatypes::PositiveInteger(15)) => 2,
                  Some(musicxml::datatypes::PositiveInteger(22)) => 3,
                  _ => 1,
                } * if octave_shift.attributes.r#type == musicxml::datatypes::UpDownStopContinue::Down {
                  -1
                } else {
                  1
                },
              },
              is_start: octave_shift.attributes.r#type != musicxml::datatypes::UpDownStopContinue::Stop,
              number: octave_shift.attributes.number.as_ref().map(|number| **number),
              for_voice: None,
            };
            if item.is_start {
              time_slice.get_mut(&staff_name).unwrap()[cursor]
                .phrase_modification_start
                .push(item);
            } else {
              time_slice.get_mut(&staff_name).unwrap()[cursor]
                .phrase_modification_end
                .push(item);
            }
          }
        }
        musicxml::elements::DirectionTypeContents::Metronome(metronome) => {
          if let Some(tempo) = Self::parse_tempo_from_metronome(metronome) {
            time_slice.get_mut(&staff_name).unwrap()[cursor]
              .tempo_change_explicit
              .push(tempo);
          }
        }
        musicxml::elements::DirectionTypeContents::AccordionRegistration(registration) => {
          let item = DirectionType::AccordionRegistration {
            high: registration.content.accordion_high.is_some(),
            middle: registration
              .content
              .accordion_middle
              .as_ref()
              .map_or(0, |middle| *middle.content),
            low: registration.content.accordion_low.is_some(),
          };
          time_slice.get_mut(&staff_name).unwrap()[cursor].direction.push(item);
        }
        musicxml::elements::DirectionTypeContents::StringMute(string_mute) => {
          let item = DirectionType::StringMute {
            on: string_mute.attributes.r#type == musicxml::datatypes::OnOff::On,
          };
          time_slice.get_mut(&staff_name).unwrap()[cursor].direction.push(item);
        }
        _ => (),
      });
    0
  }

  fn parse_barline_element(
    element: &musicxml::elements::Barline,
    time_slice: &mut BTreeMap<String, Vec<TimeSliceContainer>>,
    cursor: usize,
  ) -> isize {
    if let Some(ending) = &element.content.ending {
      let item = (
        ending.attributes.r#type == musicxml::datatypes::StartStopDiscontinue::Start,
        ending
          .attributes
          .number
          .split(&[',', ' '][..])
          .map(|item| item.parse().unwrap())
          .collect(),
      );
      for slice in time_slice.values_mut() {
        slice[cursor].ending.push(item.clone());
      }
    }
    if let Some(repeat) = &element.content.repeat {
      let item = (
        repeat.attributes.direction == musicxml::datatypes::BackwardForward::Forward,
        repeat.attributes.times.as_ref().map_or(1, |item| **item),
      );
      for slice in time_slice.values_mut() {
        slice[cursor].repeat.push(item.clone());
      }
    }
    if element.content.coda.is_some() {
      for slice in time_slice.values_mut() {
        slice[cursor].section_start.push(String::from("Coda"));
      }
    }
    if element.content.segno.is_some() {
      for slice in time_slice.values_mut() {
        slice[cursor].section_start.push(String::from("Segno"));
      }
    }
    if element.attributes.coda.is_some() {
      for slice in time_slice.values_mut() {
        slice[cursor].jump_to.push(String::from("Coda"));
      }
    }
    if element.attributes.segno.is_some() {
      for slice in time_slice.values_mut() {
        slice[cursor].jump_to.push(String::from("Segno"));
      }
    }
    0
  }

  fn parse_note_element(
    note: &musicxml::elements::Note,
    time_slices: &mut BTreeMap<String, Vec<TimeSliceContainer>>,
    divisions_per_quarter_note: usize,
    previous_cursor: usize,
    cursor: usize,
  ) -> isize {
    let staff_name = if let Some(staff) = &note.content.staff {
      staff.content.to_string()
    } else {
      String::from("1")
    };
    let num_dots = note.content.dot.len() as u8;
    let (divisions, mut tied, chord, pitch) = match &note.content.info {
      musicxml::elements::NoteType::Cue(cue) => (
        *cue.duration.content as usize,
        false,
        cue.chord.is_some(),
        Pitch::new_rest(),
      ),
      musicxml::elements::NoteType::Grace(grace) => match &grace.info {
        musicxml::elements::GraceType::Cue(cue) => (0, false, cue.chord.is_some(), Pitch::new_rest()),
        musicxml::elements::GraceType::Normal(normal) => (
          0,
          normal
            .tie
            .iter()
            .any(|tie| tie.attributes.r#type == musicxml::datatypes::StartStop::Start),
          normal.chord.is_some(),
          match &normal.audible {
            musicxml::elements::AudibleType::Pitch(pitch) => {
              let octave = *pitch.content.octave.content;
              match &pitch.content.step.content {
                musicxml::datatypes::Step::A => Pitch::new(PitchName::A, octave),
                musicxml::datatypes::Step::B => Pitch::new(PitchName::B, octave),
                musicxml::datatypes::Step::C => Pitch::new(PitchName::C, octave),
                musicxml::datatypes::Step::D => Pitch::new(PitchName::D, octave),
                musicxml::datatypes::Step::E => Pitch::new(PitchName::E, octave),
                musicxml::datatypes::Step::F => Pitch::new(PitchName::F, octave),
                musicxml::datatypes::Step::G => Pitch::new(PitchName::G, octave),
              }
            }
            _ => Pitch::new_rest(),
          },
        ),
      },
      musicxml::elements::NoteType::Normal(normal) => (
        *normal.duration.content as usize,
        normal
          .tie
          .iter()
          .any(|tie| tie.attributes.r#type == musicxml::datatypes::StartStop::Start),
        normal.chord.is_some(),
        match &normal.audible {
          musicxml::elements::AudibleType::Pitch(pitch) => {
            let octave = *pitch.content.octave.content;
            match &pitch.content.step.content {
              musicxml::datatypes::Step::A => Pitch::new(PitchName::A, octave),
              musicxml::datatypes::Step::B => Pitch::new(PitchName::B, octave),
              musicxml::datatypes::Step::C => Pitch::new(PitchName::C, octave),
              musicxml::datatypes::Step::D => Pitch::new(PitchName::D, octave),
              musicxml::datatypes::Step::E => Pitch::new(PitchName::E, octave),
              musicxml::datatypes::Step::F => Pitch::new(PitchName::F, octave),
              musicxml::datatypes::Step::G => Pitch::new(PitchName::G, octave),
            }
          }
          _ => Pitch::new_rest(),
        },
      ),
    };
    let duration = if let Some(note_type) = &note.content.r#type {
      match &note_type.content {
        musicxml::datatypes::NoteTypeValue::Maxima => Duration::new(DurationType::Maxima, num_dots),
        musicxml::datatypes::NoteTypeValue::Long => Duration::new(DurationType::Long, num_dots),
        musicxml::datatypes::NoteTypeValue::Breve => Duration::new(DurationType::Breve, num_dots),
        musicxml::datatypes::NoteTypeValue::Whole => Duration::new(DurationType::Whole, num_dots),
        musicxml::datatypes::NoteTypeValue::Half => Duration::new(DurationType::Half, num_dots),
        musicxml::datatypes::NoteTypeValue::Eighth => Duration::new(DurationType::Eighth, num_dots),
        musicxml::datatypes::NoteTypeValue::Sixteenth => Duration::new(DurationType::Sixteenth, num_dots),
        musicxml::datatypes::NoteTypeValue::ThirtySecond => Duration::new(DurationType::ThirtySecond, num_dots),
        musicxml::datatypes::NoteTypeValue::SixtyFourth => Duration::new(DurationType::SixtyFourth, num_dots),
        musicxml::datatypes::NoteTypeValue::OneHundredTwentyEighth => {
          Duration::new(DurationType::OneHundredTwentyEighth, num_dots)
        }
        musicxml::datatypes::NoteTypeValue::TwoHundredFiftySixth => {
          Duration::new(DurationType::TwoHundredFiftySixth, num_dots)
        }
        musicxml::datatypes::NoteTypeValue::FiveHundredTwelfth => {
          Duration::new(DurationType::FiveHundredTwelfth, num_dots)
        }
        musicxml::datatypes::NoteTypeValue::OneThousandTwentyFourth => {
          Duration::new(DurationType::OneThousandTwentyFourth, num_dots)
        }
        _ => Duration::new(DurationType::Quarter, num_dots),
      }
    } else {
      match divisions {
        _ if divisions / divisions_per_quarter_note >= 32 => Duration::new(
          DurationType::Maxima,
          MusicXmlConverter::calculate_num_dots(32 * divisions_per_quarter_note, divisions),
        ),
        _ if divisions / divisions_per_quarter_note >= 16 => Duration::new(
          DurationType::Long,
          MusicXmlConverter::calculate_num_dots(16 * divisions_per_quarter_note, divisions),
        ),
        _ if divisions / divisions_per_quarter_note >= 8 => Duration::new(
          DurationType::Breve,
          MusicXmlConverter::calculate_num_dots(8 * divisions_per_quarter_note, divisions),
        ),
        _ if divisions / divisions_per_quarter_note >= 4 => Duration::new(
          DurationType::Whole,
          MusicXmlConverter::calculate_num_dots(4 * divisions_per_quarter_note, divisions),
        ),
        _ if divisions / divisions_per_quarter_note >= 2 => Duration::new(
          DurationType::Half,
          MusicXmlConverter::calculate_num_dots(2 * divisions_per_quarter_note, divisions),
        ),
        _ if divisions / divisions_per_quarter_note >= 1 => Duration::new(
          DurationType::Quarter,
          MusicXmlConverter::calculate_num_dots(1 * divisions_per_quarter_note, divisions),
        ),
        _ if divisions_per_quarter_note / divisions <= 2 => Duration::new(
          DurationType::Eighth,
          MusicXmlConverter::calculate_num_dots(divisions_per_quarter_note / 2, divisions),
        ),
        _ if divisions_per_quarter_note / divisions <= 4 => Duration::new(
          DurationType::Sixteenth,
          MusicXmlConverter::calculate_num_dots(divisions_per_quarter_note / 4, divisions),
        ),
        _ if divisions_per_quarter_note / divisions <= 8 => Duration::new(
          DurationType::ThirtySecond,
          MusicXmlConverter::calculate_num_dots(divisions_per_quarter_note / 8, divisions),
        ),
        _ if divisions_per_quarter_note / divisions <= 16 => Duration::new(
          DurationType::SixtyFourth,
          MusicXmlConverter::calculate_num_dots(divisions_per_quarter_note / 16, divisions),
        ),
        _ if divisions_per_quarter_note / divisions <= 32 => Duration::new(
          DurationType::OneHundredTwentyEighth,
          MusicXmlConverter::calculate_num_dots(divisions_per_quarter_note / 32, divisions),
        ),
        _ if divisions_per_quarter_note / divisions <= 64 => Duration::new(
          DurationType::TwoHundredFiftySixth,
          MusicXmlConverter::calculate_num_dots(divisions_per_quarter_note / 64, divisions),
        ),
        _ if divisions_per_quarter_note / divisions <= 128 => Duration::new(
          DurationType::FiveHundredTwelfth,
          MusicXmlConverter::calculate_num_dots(divisions_per_quarter_note / 128, divisions),
        ),
        _ if divisions_per_quarter_note / divisions <= 256 => Duration::new(
          DurationType::OneThousandTwentyFourth,
          MusicXmlConverter::calculate_num_dots(divisions_per_quarter_note / 256, divisions),
        ),
        _ => Duration::new(DurationType::TwoThousandFortyEighth, num_dots),
      }
    };
    let voice = note.content.voice.as_ref().map(|voice| voice.content.clone());
    let accidental = note
      .content
      .accidental
      .as_ref()
      .map(|accidental| match accidental.content {
        musicxml::datatypes::AccidentalValue::Sharp | musicxml::datatypes::AccidentalValue::NaturalSharp => {
          Accidental::Sharp
        }
        musicxml::datatypes::AccidentalValue::Flat | musicxml::datatypes::AccidentalValue::NaturalFlat => {
          Accidental::Flat
        }
        musicxml::datatypes::AccidentalValue::Natural => Accidental::Natural,
        musicxml::datatypes::AccidentalValue::DoubleSharp | musicxml::datatypes::AccidentalValue::SharpSharp => {
          Accidental::DoubleSharp
        }
        musicxml::datatypes::AccidentalValue::FlatFlat => Accidental::DoubleFlat,
        _ => Accidental::None,
      });
    let tuplet_details =
      note
        .content
        .time_modification
        .as_ref()
        .map(|time_modification| PhraseModificationType::Tuplet {
          num_beats: *time_modification.content.actual_notes.content as u8,
          into_beats: *time_modification.content.normal_notes.content as u8,
        });
    let (mut arpeggiate, mut non_arpeggiate) = (false, false);
    let mut note_modifications: Vec<NoteModificationType> = Vec::new();
    let (mut phrase_modifications_start, mut phrase_modifications_end) = (Vec::new(), Vec::new());
    note.content.notations.iter().for_each(|notation| {
      notation
        .content
        .notations
        .iter()
        .for_each(|notation_type| match notation_type {
          musicxml::elements::NotationContentTypes::Tied(tie) => {
            tied = (tie.attributes.r#type == musicxml::datatypes::StartStopContinue::Start)
              || (tie.attributes.r#type == musicxml::datatypes::StartStopContinue::Continue);
          }
          musicxml::elements::NotationContentTypes::Slur(slur) => {
            if slur.attributes.r#type != musicxml::datatypes::StartStopContinue::Continue {
              let item = PhraseModDetails {
                modification: PhraseModificationType::Legato,
                is_start: slur.attributes.r#type == musicxml::datatypes::StartStopContinue::Start,
                number: slur.attributes.number.as_ref().map(|number| **number),
                for_voice: None,
              };
              if item.is_start {
                phrase_modifications_start.push(item);
              } else {
                phrase_modifications_end.push(item);
              }
            }
          }
          musicxml::elements::NotationContentTypes::Tuplet(tuplet) => {
            let item = PhraseModDetails {
              modification: tuplet_details.unwrap(),
              is_start: tuplet.attributes.r#type == musicxml::datatypes::StartStop::Start,
              number: tuplet.attributes.number.as_ref().map(|number| **number),
              for_voice: voice.clone(),
            };
            if item.is_start {
              phrase_modifications_start.push(item);
            } else {
              phrase_modifications_end.push(item);
            }
          }
          musicxml::elements::NotationContentTypes::Glissando(glissando) => {
            let item = PhraseModDetails {
              modification: PhraseModificationType::Glissando,
              is_start: glissando.attributes.r#type == musicxml::datatypes::StartStop::Start,
              number: glissando.attributes.number.as_ref().map(|number| **number),
              for_voice: voice.clone(),
            };
            if item.is_start {
              phrase_modifications_start.push(item);
            } else {
              phrase_modifications_end.push(item);
            }
          }
          musicxml::elements::NotationContentTypes::Slide(slide) => {
            let item = PhraseModDetails {
              modification: PhraseModificationType::Portamento,
              is_start: slide.attributes.r#type == musicxml::datatypes::StartStop::Start,
              number: slide.attributes.number.as_ref().map(|number| **number),
              for_voice: voice.clone(),
            };
            if item.is_start {
              phrase_modifications_start.push(item);
            } else {
              phrase_modifications_end.push(item);
            }
          }
          musicxml::elements::NotationContentTypes::Ornaments(ornaments) => {
            note_modifications.extend(
              ornaments
                .content
                .ornaments
                .iter()
                .filter_map(|ornament| match ornament {
                  musicxml::elements::OrnamentType::TrillMark(_trill_mark) => {
                    Some(NoteModificationType::Trill { upper: true })
                  }
                  musicxml::elements::OrnamentType::Turn(_turn) => Some(NoteModificationType::Turn {
                    upper: true,
                    delayed: false,
                    vertical: false,
                  }),
                  musicxml::elements::OrnamentType::DelayedTurn(_delayed_turn) => Some(NoteModificationType::Turn {
                    upper: true,
                    delayed: true,
                    vertical: false,
                  }),
                  musicxml::elements::OrnamentType::InvertedTurn(_inverted_turn) => Some(NoteModificationType::Turn {
                    upper: false,
                    delayed: false,
                    vertical: false,
                  }),
                  musicxml::elements::OrnamentType::DelayedInvertedTurn(_delayed_inverted_turn) => {
                    Some(NoteModificationType::Turn {
                      upper: false,
                      delayed: true,
                      vertical: false,
                    })
                  }
                  musicxml::elements::OrnamentType::VerticalTurn(_vertical_turn) => Some(NoteModificationType::Turn {
                    upper: true,
                    delayed: false,
                    vertical: true,
                  }),
                  musicxml::elements::OrnamentType::InvertedVerticalTurn(_inverted_vertical_turn) => {
                    Some(NoteModificationType::Turn {
                      upper: false,
                      delayed: false,
                      vertical: true,
                    })
                  }
                  musicxml::elements::OrnamentType::Shake(_shake) => Some(NoteModificationType::Shake),
                  musicxml::elements::OrnamentType::WavyLine(_wavy_line) => {
                    Some(NoteModificationType::Trill { upper: true })
                  }
                  musicxml::elements::OrnamentType::Mordent(_mordent) => {
                    Some(NoteModificationType::Mordent { upper: true })
                  }
                  musicxml::elements::OrnamentType::InvertedMordent(_inverted_mordent) => {
                    Some(NoteModificationType::Mordent { upper: false })
                  }
                  musicxml::elements::OrnamentType::Schleifer(_schleifer) => Some(NoteModificationType::Schleifer),
                  musicxml::elements::OrnamentType::Tremolo(tremolo) => {
                    let relative_speed = *tremolo.content;
                    if let Some(tremolo_type) = &tremolo.attributes.r#type {
                      match tremolo_type {
                        musicxml::datatypes::TremoloType::Start => {
                          phrase_modifications_start.push(PhraseModDetails {
                            modification: PhraseModificationType::Tremolo { relative_speed },
                            is_start: true,
                            number: None,
                            for_voice: voice.clone(),
                          });
                          None
                        }
                        musicxml::datatypes::TremoloType::Stop => {
                          phrase_modifications_end.push(PhraseModDetails {
                            modification: PhraseModificationType::Tremolo { relative_speed },
                            is_start: false,
                            number: None,
                            for_voice: voice.clone(),
                          });
                          None
                        }
                        musicxml::datatypes::TremoloType::Single => {
                          Some(NoteModificationType::Tremolo { relative_speed })
                        }
                        _ => None,
                      }
                    } else {
                      Some(NoteModificationType::Tremolo { relative_speed })
                    }
                  }
                  musicxml::elements::OrnamentType::Haydn(_haydn) => Some(NoteModificationType::Haydn),
                  musicxml::elements::OrnamentType::OtherOrnament(_) => None,
                }),
            );
          }
          musicxml::elements::NotationContentTypes::Technical(technicals) => {
            note_modifications.extend(technicals.content.iter().filter_map(|technical| match technical {
              musicxml::elements::TechnicalContents::UpBow(_up_bow) => Some(NoteModificationType::UpBow),
              musicxml::elements::TechnicalContents::DownBow(_down_bow) => Some(NoteModificationType::DownBow),
              musicxml::elements::TechnicalContents::Harmonic(_harmonic) => None,
              musicxml::elements::TechnicalContents::OpenString(_open_string) => Some(NoteModificationType::Open),
              musicxml::elements::TechnicalContents::ThumbPosition(_thumb_position) => {
                Some(NoteModificationType::ThumbPosition)
              }
              musicxml::elements::TechnicalContents::Fingering(_fingering) => None,
              musicxml::elements::TechnicalContents::Pluck(_pluck) => None,
              musicxml::elements::TechnicalContents::DoubleTongue(_double_tongue) => {
                Some(NoteModificationType::DoubleTongue)
              }
              musicxml::elements::TechnicalContents::TripleTongue(_triple_tongue) => {
                Some(NoteModificationType::TripleTongue)
              }
              musicxml::elements::TechnicalContents::Stopped(_stopped) => Some(NoteModificationType::Stopped),
              musicxml::elements::TechnicalContents::SnapPizzicato(_snap_pizzicato) => {
                Some(NoteModificationType::Pizzicato)
              }
              musicxml::elements::TechnicalContents::Fret(_fret) => None,
              musicxml::elements::TechnicalContents::StringNumber(_string) => None,
              musicxml::elements::TechnicalContents::HammerOn(_hammer_on) => None,
              musicxml::elements::TechnicalContents::PullOff(_pull_off) => None,
              musicxml::elements::TechnicalContents::Bend(_bend) => None,
              musicxml::elements::TechnicalContents::Tap(_tap) => Some(NoteModificationType::Tap),
              musicxml::elements::TechnicalContents::Heel(_heel) => Some(NoteModificationType::Heel),
              musicxml::elements::TechnicalContents::Toe(_toe) => Some(NoteModificationType::Toe),
              musicxml::elements::TechnicalContents::Fingernails(_fingernails) => {
                Some(NoteModificationType::Fingernails)
              }
              musicxml::elements::TechnicalContents::Hole(hole) => Some(match hole.content.hole_closed.content {
                musicxml::datatypes::HoleClosedValue::No => NoteModificationType::Hole {
                  open: true,
                  half: false,
                },
                musicxml::datatypes::HoleClosedValue::Half => NoteModificationType::Hole { open: true, half: true },
                musicxml::datatypes::HoleClosedValue::Yes => NoteModificationType::Hole {
                  open: false,
                  half: false,
                },
              }),
              musicxml::elements::TechnicalContents::Arrow(_arrow) => None,
              musicxml::elements::TechnicalContents::Handbell(handbell) => Some(NoteModificationType::Handbell {
                technique: match &handbell.content {
                  musicxml::datatypes::HandbellValue::Belltree => HandbellTechnique::Belltree,
                  musicxml::datatypes::HandbellValue::Damp => HandbellTechnique::Damp,
                  musicxml::datatypes::HandbellValue::Echo => HandbellTechnique::Echo,
                  musicxml::datatypes::HandbellValue::Gyro => HandbellTechnique::Gyro,
                  musicxml::datatypes::HandbellValue::HandMartellato => HandbellTechnique::HandMartellato,
                  musicxml::datatypes::HandbellValue::MalletLift => HandbellTechnique::MalletLift,
                  musicxml::datatypes::HandbellValue::MalletTable => HandbellTechnique::MalletTable,
                  musicxml::datatypes::HandbellValue::Martellato => HandbellTechnique::Martellato,
                  musicxml::datatypes::HandbellValue::MartellatoLift => HandbellTechnique::MartellatoLift,
                  musicxml::datatypes::HandbellValue::MutedMartellato => HandbellTechnique::MutedMartellato,
                  musicxml::datatypes::HandbellValue::PluckLift => HandbellTechnique::PluckLift,
                  musicxml::datatypes::HandbellValue::Swing => HandbellTechnique::Swing,
                },
              }),
              musicxml::elements::TechnicalContents::BrassBend(_brass_bend) => Some(NoteModificationType::BrassBend),
              musicxml::elements::TechnicalContents::Flip(_flip) => Some(NoteModificationType::Flip),
              musicxml::elements::TechnicalContents::Smear(_smear) => Some(NoteModificationType::Smear),
              musicxml::elements::TechnicalContents::Open(_open) => Some(NoteModificationType::Open),
              musicxml::elements::TechnicalContents::HalfMuted(_half_muted) => Some(NoteModificationType::HalfMuted),
              musicxml::elements::TechnicalContents::HarmonMute(harmon_mute) => {
                Some(match harmon_mute.content.harmon_closed.content {
                  musicxml::datatypes::HarmonClosedValue::No => NoteModificationType::HarmonMute {
                    open: true,
                    half: false,
                  },
                  musicxml::datatypes::HarmonClosedValue::Half => {
                    NoteModificationType::HarmonMute { open: true, half: true }
                  }
                  musicxml::datatypes::HarmonClosedValue::Yes => NoteModificationType::HarmonMute {
                    open: false,
                    half: false,
                  },
                })
              }
              musicxml::elements::TechnicalContents::Golpe(_golpe) => Some(NoteModificationType::Golpe),
              musicxml::elements::TechnicalContents::OtherTechnical(_) => None,
            }));
          }
          musicxml::elements::NotationContentTypes::Articulations(articulations) => {
            note_modifications.extend(
              articulations
                .content
                .iter()
                .filter_map(|articulation| match articulation {
                  musicxml::elements::ArticulationsType::Accent(_accent) => Some(NoteModificationType::Accent),
                  musicxml::elements::ArticulationsType::StrongAccent(_strong_accent) => {
                    Some(NoteModificationType::Marcato)
                  }
                  musicxml::elements::ArticulationsType::Staccato(_staccato) => Some(NoteModificationType::Staccato),
                  musicxml::elements::ArticulationsType::Tenuto(_tenuto) => Some(NoteModificationType::Tenuto),
                  musicxml::elements::ArticulationsType::DetachedLegato(_detached_legato) => {
                    Some(NoteModificationType::DetachedLegato)
                  }
                  musicxml::elements::ArticulationsType::Staccatissimo(_staccatissimo) => {
                    Some(NoteModificationType::Staccatissimo)
                  }
                  musicxml::elements::ArticulationsType::Spiccato(_spiccato) => Some(NoteModificationType::Spiccato),
                  musicxml::elements::ArticulationsType::Scoop(_scoop) => Some(NoteModificationType::Scoop),
                  musicxml::elements::ArticulationsType::Plop(_plop) => Some(NoteModificationType::Plop),
                  musicxml::elements::ArticulationsType::Doit(_doit) => Some(NoteModificationType::Doit),
                  musicxml::elements::ArticulationsType::Falloff(_falloff) => Some(NoteModificationType::Falloff),
                  musicxml::elements::ArticulationsType::BreathMark(_breath_mark) => {
                    time_slices.get_mut(&staff_name).unwrap()[cursor]
                      .direction
                      .push(DirectionType::BreathMark);
                    None
                  }
                  musicxml::elements::ArticulationsType::Caesura(_caesura) => {
                    time_slices.get_mut(&staff_name).unwrap()[cursor]
                      .direction
                      .push(DirectionType::Caesura);
                    None
                  }
                  musicxml::elements::ArticulationsType::Stress(_stress) => Some(NoteModificationType::Stress),
                  musicxml::elements::ArticulationsType::Unstress(_unstress) => Some(NoteModificationType::Unstress),
                  musicxml::elements::ArticulationsType::SoftAccent(_soft_accent) => {
                    Some(NoteModificationType::SoftAccent)
                  }
                  musicxml::elements::ArticulationsType::OtherArticulation(_) => None,
                }),
            );
          }
          musicxml::elements::NotationContentTypes::Dynamics(dynamics) => match &dynamics.content[0] {
            musicxml::elements::DynamicsType::P(_p) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: Dynamic::new(DynamicMarking::Piano, 1),
            }),
            musicxml::elements::DynamicsType::Pp(_pp) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: Dynamic::new(DynamicMarking::Piano, 2),
            }),
            musicxml::elements::DynamicsType::Ppp(_ppp) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: Dynamic::new(DynamicMarking::Piano, 3),
            }),
            musicxml::elements::DynamicsType::Pppp(_pppp) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: Dynamic::new(DynamicMarking::Piano, 4),
            }),
            musicxml::elements::DynamicsType::Ppppp(_pppp_it) => {
              note_modifications.push(NoteModificationType::Dynamic {
                dynamic: Dynamic::new(DynamicMarking::Piano, 5),
              });
            }
            musicxml::elements::DynamicsType::Pppppp(_ppp_it) => {
              note_modifications.push(NoteModificationType::Dynamic {
                dynamic: Dynamic::new(DynamicMarking::Piano, 6),
              });
            }
            musicxml::elements::DynamicsType::F(_f) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: Dynamic::new(DynamicMarking::Forte, 1),
            }),
            musicxml::elements::DynamicsType::Ff(_ff) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: Dynamic::new(DynamicMarking::Forte, 2),
            }),
            musicxml::elements::DynamicsType::Fff(_fff) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: Dynamic::new(DynamicMarking::Forte, 3),
            }),
            musicxml::elements::DynamicsType::Ffff(_ffff) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: Dynamic::new(DynamicMarking::Forte, 4),
            }),
            musicxml::elements::DynamicsType::Fffff(_ffff_it) => {
              note_modifications.push(NoteModificationType::Dynamic {
                dynamic: Dynamic::new(DynamicMarking::Forte, 5),
              });
            }
            musicxml::elements::DynamicsType::Ffffff(_fff_it) => {
              note_modifications.push(NoteModificationType::Dynamic {
                dynamic: Dynamic::new(DynamicMarking::Forte, 6),
              });
            }
            musicxml::elements::DynamicsType::Mp(_mp) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: Dynamic::new(DynamicMarking::MezzoPiano, 0),
            }),
            musicxml::elements::DynamicsType::Mf(_mf) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: Dynamic::new(DynamicMarking::MezzoForte, 0),
            }),
            musicxml::elements::DynamicsType::N(_) | musicxml::elements::DynamicsType::OtherDynamics(_) => (),
            _ => note_modifications.push(NoteModificationType::Accent),
          },
          musicxml::elements::NotationContentTypes::Fermata(_fermata) => {
            note_modifications.push(NoteModificationType::Fermata);
          }
          musicxml::elements::NotationContentTypes::Arpeggiate(_arpeggiate) => arpeggiate = true,
          musicxml::elements::NotationContentTypes::NonArpeggiate(_non_arpeggiate) => non_arpeggiate = true,
          _ => {}
        });
    });
    if let Some(pizzicato) = &note.attributes.pizzicato {
      if *pizzicato == musicxml::datatypes::YesNo::Yes
        && !note_modifications
          .iter()
          .any(|modification| matches!(modification, NoteModificationType::Pizzicato))
      {
        note_modifications.push(NoteModificationType::Pizzicato);
      }
    }
    if tied {
      note_modifications.push(NoteModificationType::Tie);
    }
    let item = NoteDetails {
      pitch,
      duration,
      accidental: accidental.unwrap_or(Accidental::None),
      voice,
      arpeggiated: arpeggiate,
      non_arpeggiated: non_arpeggiate,
      note_modifications,
      phrase_modifications_start,
      phrase_modifications_end,
    };
    if chord {
      time_slices.get_mut(&staff_name).unwrap()[previous_cursor]
        .notes
        .push(item);
      0
    } else {
      time_slices.get_mut(&staff_name).unwrap()[cursor].notes.push(item);
      divisions as isize
    }
  }

  fn start_phrase(
    phrases: &mut Vec<Rc<RefCell<Phrase>>>,
    staff: &Rc<RefCell<Staff>>,
    modification: PhraseModificationType,
    phrase_ids: &mut BTreeMap<u8, usize>,
    mod_id: Option<u8>,
  ) {
    let new_phrase = if let Some(phrase) = phrases.last() {
      phrase.borrow_mut().add_phrase()
    } else {
      staff.borrow_mut().add_phrase()
    };
    new_phrase.borrow_mut().add_modification(modification);
    if let Some(number) = mod_id {
      phrase_ids.insert(number, new_phrase.borrow().get_id());
    }
    phrases.push(new_phrase);
  }

  fn end_phrase(phrases: &mut Vec<Rc<RefCell<Phrase>>>, phrase_ids: &mut BTreeMap<u8, usize>, mod_id: &Option<u8>) {
    if let Some(number) = mod_id {
      match phrase_ids.remove(number) {
        Some(phrase_id) => {
          let index = phrases
            .iter()
            .position(|phrase| phrase.borrow().get_id() == phrase_id)
            .unwrap_or(phrases.len());
          (index..phrases.len()).for_each(|_| match phrases.pop() {
            Some(phrase) => {
              let id = phrase.borrow().get_id();
              if let Some(number) = phrase_ids
                .iter()
                .find_map(|(key, phrase_id)| if *phrase_id == id { Some(*key) } else { None })
              {
                phrase_ids.remove(&number);
              }
            }
            None => unsafe { core::hint::unreachable_unchecked() },
          });
        }
        None => (),
      }
    } else {
      phrases.pop();
    }
  }

  fn load_from_musicxml(score: ScorePartwise) -> Result<Composition, String> {
    // Generate the initial composition structure and search for known metadata
    let mut composition = Composition::new(
      match &score.content.work {
        Some(work) => match &work.content.work_title {
          Some(work_title) => work_title.content.as_str(),
          None => "Untitled",
        },
        None => "Untitled",
      },
      None,
      None,
      None,
    );
    MusicXmlConverter::find_metadata(&mut composition, &score.content);

    // Find and validate all musical parts in the score
    let parts_map = MusicXmlConverter::find_parts(&score.content.part_list.content.content);
    if parts_map.is_empty() || score.content.part.is_empty() {
      return Err(String::from("No parts found in the MusicXML score"));
    } else if score.content.part.iter().all(|part| part.content.is_empty()) {
      return Err(String::from("All parts in the MusicXML score are empty"));
    }
    parts_map.values().for_each(|name| {
      composition.add_part(name);
    });

    // Parse the initial musical attributes of the score
    composition.set_starting_key(MusicXmlConverter::find_starting_key(&score.content.part));
    composition.set_starting_time_signature(MusicXmlConverter::find_starting_time_signature(&score.content.part));
    composition.set_tempo(MusicXmlConverter::find_tempo(&score.content.part));

    // Create a data structure to hold all temporally parsed musical data
    let mut part_data = TemporalPartData { data: BTreeMap::new() };
    for part in &score.content.part {
      if !part.content.is_empty() {
        let part_name = parts_map
          .get(&*part.attributes.id)
          .expect("Unknown Part ID encountered");
        let max_divisions = MusicXmlConverter::find_divisions_per_quarter_note(&part.content)
          * MusicXmlConverter::find_max_num_quarter_notes_per_measure(&part.content)
          * MusicXmlConverter::find_num_measures(&part.content);
        part_data.data.insert(part_name.clone(), BTreeMap::new());
        let part_staves = part_data.data.get_mut(part_name).unwrap();
        MusicXmlConverter::find_staves(&part.content).iter().for_each(|staff| {
          part_staves.insert(staff.clone(), vec![TimeSliceContainer::default(); max_divisions]);
        });
      }
    }

    // Parse the actual musical contents of the score into discrete time slices
    for part in &score.content.part {
      if part.content.is_empty() {
        composition.remove_part_by_name(parts_map.get(&*part.attributes.id).unwrap());
      } else {
        let (mut cursor, mut previous_cursor): (usize, usize) = (0, 0);
        let divisions_per_quarter_note = MusicXmlConverter::find_divisions_per_quarter_note(&part.content);
        let time_slices = part_data
          .data
          .get_mut(parts_map.get(&*part.attributes.id).unwrap())
          .unwrap();
        for element in &part.content {
          if let musicxml::elements::PartElement::Measure(measure) = element {
            for measure_element in &measure.content {
              let cursor_change = match measure_element {
                musicxml::elements::MeasureElement::Attributes(attributes) => {
                  MusicXmlConverter::parse_attributes_element(&attributes.content, time_slices, cursor)
                }
                musicxml::elements::MeasureElement::Note(note) => MusicXmlConverter::parse_note_element(
                  &note,
                  time_slices,
                  divisions_per_quarter_note,
                  previous_cursor,
                  cursor,
                ),
                musicxml::elements::MeasureElement::Backup(backup) => {
                  MusicXmlConverter::parse_backup_element(&backup.content)
                }
                musicxml::elements::MeasureElement::Forward(forward) => {
                  MusicXmlConverter::parse_forward_element(&forward.content)
                }
                musicxml::elements::MeasureElement::Direction(direction) => {
                  MusicXmlConverter::parse_direction_element(&direction, time_slices, cursor)
                }
                musicxml::elements::MeasureElement::Barline(barline) => {
                  MusicXmlConverter::parse_barline_element(&barline, time_slices, cursor)
                }
                _ => 0,
              };
              if cursor_change != 0 {
                previous_cursor = cursor;
                cursor = cursor.saturating_add_signed(cursor_change);
              }
            }
          }
        }
      }
    }

    // Use the temporally ordered time slices to construct a final composition structure
    // TODO: DELETE THIS: println!("{}", part_data);
    for (part_name, staves) in part_data.data {
      let part = composition
        .get_part_by_name(&part_name)
        .expect("Unknown part name encountered");
      let section = part.add_default_section();
      for (staff_name, time_slices) in staves {
        let staff = section.borrow_mut().add_staff(&staff_name, None, None, None);
        let mut phrase_ids = BTreeMap::new();
        let mut multivoice_phrases: BTreeMap<String, Rc<RefCell<Phrase>>> = BTreeMap::new();
        let (mut local_phrases, mut global_phrases) = (BTreeMap::<String, Vec<Rc<RefCell<Phrase>>>>::new(), Vec::new());
        let (mut delayed_phrase_starts, mut delayed_phrase_ends) = (Vec::new(), Vec::new());
        for time_slice in time_slices {
          // Parse notes and separate them by voice
          let (mut voicewide_mods, mut notes_by_voice) = (
            BTreeMap::new(),
            BTreeMap::<String, Vec<(Rc<RefCell<Note>>, NoteDetails)>>::new(),
          );
          for item in time_slice.notes {
            let voice = String::from(if let Some(voice) = &item.voice { voice } else { "-1" });
            let voice_mods = if voicewide_mods.contains_key(&voice) {
              voicewide_mods.get_mut(&voice).unwrap()
            } else {
              voicewide_mods.insert(voice.clone(), Vec::new());
              voicewide_mods.get_mut(&voice).unwrap()
            };
            let note = Note::new(item.pitch, item.duration, Some(item.accidental));
            for modification in &item.note_modifications {
              match modification {
                NoteModificationType::Accent
                | NoteModificationType::DownBow
                | NoteModificationType::Dynamic { dynamic: _ }
                | NoteModificationType::Fermata
                | NoteModificationType::HalfMuted
                | NoteModificationType::Marcato
                | NoteModificationType::Open
                | NoteModificationType::Pizzicato
                | NoteModificationType::Sforzando
                | NoteModificationType::SoftAccent
                | NoteModificationType::Spiccato
                | NoteModificationType::Stress
                | NoteModificationType::Tenuto
                | NoteModificationType::Unstress
                | NoteModificationType::UpBow => voice_mods.push(
                  *ChordModification::from_note_modification(modification)
                    .borrow()
                    .get_modification(),
                ),
                _ => {
                  note.borrow_mut().add_modification(modification.clone());
                }
              }
            }
            if item.arpeggiated {
              voice_mods.push(ChordModificationType::Arpeggiate);
            } else if item.non_arpeggiated {
              voice_mods.push(ChordModificationType::NonArpeggiate);
            }
            if let Some(notes) = notes_by_voice.get_mut(&voice) {
              notes.push((note, item));
            } else {
              notes_by_voice.insert(String::from(voice), vec![(note, item)]);
            }
          }

          // Merge multiple notes into chords and apply voice-specific modifications
          let mut items_by_voice = BTreeMap::new();
          for (voice, notes) in notes_by_voice {
            let (mut mods_start, mut mods_end, mut legato_start) =
              (Vec::<PhraseModDetails>::new(), Vec::<PhraseModDetails>::new(), false);
            if notes.len() <= 1 {
              for (note, details) in notes {
                mods_start = Vec::new();
                if let Some(mods) = voicewide_mods.get_mut(voice.as_str()) {
                  mods.iter().for_each(|modification| {
                    if let Some(note_mod) = NoteModification::from_chord_modification(modification) {
                      note
                        .borrow_mut()
                        .add_modification(*note_mod.borrow().get_modification());
                    }
                  });
                }
                for modification in &time_slice.chord_modification {
                  if let Some(note_mod) = NoteModification::from_chord_modification(modification) {
                    note
                      .borrow_mut()
                      .add_modification(*note_mod.borrow().get_modification());
                  }
                }
                for modification in details.phrase_modifications_start {
                  match modification.modification {
                    PhraseModificationType::Legato => legato_start = true,
                    _ => {
                      if mods_start
                        .iter()
                        .find(|&item| item.modification == modification.modification)
                        .is_none()
                      {
                        mods_start.push(modification);
                      }
                    }
                  }
                }
                items_by_voice.insert(
                  voice.clone(),
                  (
                    Some(note),
                    None,
                    mods_start,
                    details.phrase_modifications_end,
                    legato_start,
                  ),
                );
              }
            } else {
              let chord = Chord::new();
              if let Some(mods) = voicewide_mods.get_mut(voice.as_str()) {
                mods.iter().for_each(|modification| {
                  chord.borrow_mut().add_modification(*modification);
                });
              }
              for modification in &time_slice.chord_modification {
                chord.borrow_mut().add_modification(*modification);
              }
              for (note, details) in notes {
                for modification in details.phrase_modifications_start {
                  match modification.modification {
                    PhraseModificationType::Legato => legato_start = true,
                    _ => {
                      if mods_start
                        .iter()
                        .find(|&item| item.modification == modification.modification)
                        .is_none()
                      {
                        mods_start.push(modification);
                      }
                    }
                  }
                }
                for modification in details.phrase_modifications_end {
                  if mods_end
                    .iter()
                    .find(|&item| item.modification == modification.modification)
                    .is_none()
                  {
                    mods_end.push(modification);
                  }
                }
                chord.borrow_mut().content.push(ChordContent::Note(note));
              }
              items_by_voice.insert(voice.clone(), (None, Some(chord), mods_start, mods_end, legato_start));
            }
          }

          // Handle global phrase modification endings
          for item in time_slice.phrase_modification_end {
            if local_phrases.is_empty() {
              multivoice_phrases.clear();
              MusicXmlConverter::end_phrase(&mut global_phrases, &mut phrase_ids, &item.number);
            } else {
              delayed_phrase_ends.push((item.number, item.modification));
            }
          }

          // Handle staff-wide directions
          if !time_slice.direction.is_empty() {
            local_phrases.clear();
            multivoice_phrases.clear();
            while !global_phrases.is_empty() {
              MusicXmlConverter::end_phrase(&mut global_phrases, &mut phrase_ids, &None);
            }
            phrase_ids.clear();
            for item in time_slice.direction {
              staff.borrow_mut().add_direction(item);
            }
          }

          // Handle new global phrase modifications
          for item in time_slice.phrase_modification_start {
            if local_phrases.is_empty() {
              multivoice_phrases.clear();
              MusicXmlConverter::start_phrase(
                &mut global_phrases,
                &staff,
                item.modification,
                &mut phrase_ids,
                item.number,
              );
            } else {
              delayed_phrase_starts.push((item.number, item.modification));
            }
          }

          // Handle adding notes, chords, multivoices, and local phrase modifications
          let mut phrase_ends = BTreeMap::new();
          let mut pending_legato_phrases = Vec::new();
          if multivoice_phrases.is_empty() {
            if items_by_voice.len() <= 1 {
              for (voice, voice_items) in items_by_voice {
                phrase_ends.insert(voice.clone(), voice_items.3);
                if voice_items.4 {
                  if local_phrases.is_empty() {
                    let new_phrase = Phrase::new();
                    new_phrase.borrow_mut().add_modification(PhraseModificationType::Legato);
                    if let Some(note) = voice_items.0 {
                      new_phrase.borrow_mut().content.push(PhraseContent::Note(note));
                    } else if let Some(chord) = voice_items.1 {
                      new_phrase.borrow_mut().content.push(PhraseContent::Chord(chord));
                    }
                    local_phrases.insert(voice.clone(), vec![Rc::clone(&new_phrase)]);
                    if let Some(phrase) = global_phrases.last() {
                      phrase.borrow_mut().content.push(PhraseContent::Phrase(new_phrase));
                    } else {
                      staff.borrow_mut().content.push(StaffContent::Phrase(new_phrase));
                    }
                  } else {
                    pending_legato_phrases.push(voice.clone());
                  }
                } else {
                  if let Some(local_phrase) = local_phrases.get_mut(&voice) {
                    if let Some(note) = voice_items.0 {
                      local_phrase
                        .last()
                        .unwrap()
                        .borrow_mut()
                        .content
                        .push(PhraseContent::Note(note));
                    } else if let Some(chord) = voice_items.1 {
                      local_phrase
                        .last()
                        .unwrap()
                        .borrow_mut()
                        .content
                        .push(PhraseContent::Chord(chord));
                    }
                  } else if let Some(phrase) = global_phrases.last() {
                    if let Some(note) = voice_items.0 {
                      phrase.borrow_mut().content.push(PhraseContent::Note(note));
                    } else if let Some(chord) = voice_items.1 {
                      phrase.borrow_mut().content.push(PhraseContent::Chord(chord));
                    }
                  } else {
                    if let Some(note) = voice_items.0 {
                      staff.borrow_mut().content.push(StaffContent::Note(note));
                    } else if let Some(chord) = voice_items.1 {
                      staff.borrow_mut().content.push(StaffContent::Chord(chord));
                    }
                  }
                }
              }
            } else {
              local_phrases.clear();
              let multivoice = if let Some(phrase) = global_phrases.last() {
                phrase.borrow_mut().add_multivoice()
              } else {
                staff.borrow_mut().add_multivoice()
              };
              for (voice, voice_items) in items_by_voice {
                phrase_ends.insert(voice.clone(), voice_items.3);
                let new_voice = multivoice.borrow_mut().add_phrase();
                if voice_items.4 {
                  let new_phrase = Phrase::new();
                  new_phrase.borrow_mut().add_modification(PhraseModificationType::Legato);
                  if let Some(note) = voice_items.0 {
                    new_phrase.borrow_mut().content.push(PhraseContent::Note(note));
                  } else if let Some(chord) = voice_items.1 {
                    new_phrase.borrow_mut().content.push(PhraseContent::Chord(chord));
                  }
                  local_phrases.insert(voice.clone(), vec![Rc::clone(&new_phrase)]);
                  new_voice.borrow_mut().content.push(PhraseContent::Phrase(new_phrase));
                } else {
                  if let Some(note) = voice_items.0 {
                    new_voice.borrow_mut().content.push(PhraseContent::Note(note));
                  } else if let Some(chord) = voice_items.1 {
                    new_voice.borrow_mut().content.push(PhraseContent::Chord(chord));
                  }
                }
                multivoice_phrases.insert(voice.clone(), new_voice);
              }
            }
          } else {
            if items_by_voice
              .iter()
              .all(|(voice, _)| multivoice_phrases.contains_key(voice))
            {
              for (voice, voice_items) in items_by_voice {
                phrase_ends.insert(voice.clone(), voice_items.3);
                if let Some(phrase) = multivoice_phrases.get_mut(&voice) {
                  if voice_items.4 {
                    if !local_phrases.contains_key(&voice) {
                      let new_phrase = Phrase::new();
                      new_phrase.borrow_mut().add_modification(PhraseModificationType::Legato);
                      if let Some(note) = voice_items.0 {
                        new_phrase.borrow_mut().content.push(PhraseContent::Note(note));
                      } else if let Some(chord) = voice_items.1 {
                        new_phrase.borrow_mut().content.push(PhraseContent::Chord(chord));
                      }
                      local_phrases.insert(voice.clone(), vec![Rc::clone(&new_phrase)]);
                      phrase.borrow_mut().content.push(PhraseContent::Phrase(new_phrase));
                    } else {
                      pending_legato_phrases.push(voice.clone());
                    }
                  } else {
                    if let Some(note) = voice_items.0 {
                      phrase.borrow_mut().content.push(PhraseContent::Note(note));
                    } else if let Some(chord) = voice_items.1 {
                      phrase.borrow_mut().content.push(PhraseContent::Chord(chord));
                    }
                  }
                }
              }
            } else {
              local_phrases.clear();
              multivoice_phrases.clear();
              if items_by_voice.len() <= 1 {
                for (voice, voice_items) in items_by_voice {
                  phrase_ends.insert(voice.clone(), voice_items.3);
                  if voice_items.4 {
                    let new_phrase = Phrase::new();
                    new_phrase.borrow_mut().add_modification(PhraseModificationType::Legato);
                    if let Some(note) = voice_items.0 {
                      new_phrase.borrow_mut().content.push(PhraseContent::Note(note));
                    } else if let Some(chord) = voice_items.1 {
                      new_phrase.borrow_mut().content.push(PhraseContent::Chord(chord));
                    }
                    local_phrases.insert(voice.clone(), vec![Rc::clone(&new_phrase)]);
                    if let Some(phrase) = global_phrases.last() {
                      phrase.borrow_mut().content.push(PhraseContent::Phrase(new_phrase));
                    } else {
                      staff.borrow_mut().content.push(StaffContent::Phrase(new_phrase));
                    }
                  } else {
                    if let Some(phrase) = global_phrases.last() {
                      if let Some(note) = voice_items.0 {
                        phrase.borrow_mut().content.push(PhraseContent::Note(note));
                      } else if let Some(chord) = voice_items.1 {
                        phrase.borrow_mut().content.push(PhraseContent::Chord(chord));
                      }
                    } else {
                      if let Some(note) = voice_items.0 {
                        staff.borrow_mut().content.push(StaffContent::Note(note));
                      } else if let Some(chord) = voice_items.1 {
                        staff.borrow_mut().content.push(StaffContent::Chord(chord));
                      }
                    }
                  }
                }
              } else {
                let multivoice = if let Some(phrase) = global_phrases.last() {
                  phrase.borrow_mut().add_multivoice()
                } else {
                  staff.borrow_mut().add_multivoice()
                };
                for (voice, voice_items) in items_by_voice {
                  phrase_ends.insert(voice.clone(), voice_items.3);
                  let new_voice = multivoice.borrow_mut().add_phrase();
                  if voice_items.4 {
                    let new_phrase = Phrase::new();
                    new_phrase.borrow_mut().add_modification(PhraseModificationType::Legato);
                    if let Some(note) = voice_items.0 {
                      new_phrase.borrow_mut().content.push(PhraseContent::Note(note));
                    } else if let Some(chord) = voice_items.1 {
                      new_phrase.borrow_mut().content.push(PhraseContent::Chord(chord));
                    }
                    local_phrases.insert(voice.clone(), vec![Rc::clone(&new_phrase)]);
                    new_voice.borrow_mut().content.push(PhraseContent::Phrase(new_phrase));
                  } else {
                    if let Some(note) = voice_items.0 {
                      new_voice.borrow_mut().content.push(PhraseContent::Note(note));
                    } else if let Some(chord) = voice_items.1 {
                      new_voice.borrow_mut().content.push(PhraseContent::Chord(chord));
                    }
                  }
                  multivoice_phrases.insert(voice.clone(), new_voice);
                }
              }
            }
          }

          // Handle ending local phrase modifications
          for (voice, items) in phrase_ends {
            let mut to_remove = Vec::new();
            if let Some(phrase) = local_phrases.get_mut(&voice) {
              for item in items {
                phrase.pop(); //last().unwrap().borrow_mut().add_modification(item);
                if phrase.is_empty() {
                  to_remove.push(voice.clone());
                }
              }
            }
            for item in to_remove {
              local_phrases.remove(&item);
            }
            // TODO: Delayed global phrase ends
            // TODO: Delayed global phrase starts
            // TODO: Delayed legatos
          }

          // TODO:
          // Handle jump to
          // Handle section start
          // Handle ending
          // Handle repeat
          // Handle tempo change explicit
          // Handle tempo change implicit
        }
      }
    }
    // TODO: If any phrases contain other phrases of exactly the same length, combine them

    Ok(composition)
  }

  fn save_to_musicxml(_composition: &Composition) -> Result<String, String> {
    todo!() // TODO: Implement
  }
}

impl Convert for MusicXmlConverter {
  fn load(path: &str) -> Result<Composition, String> {
    let score = musicxml::read_score_partwise(path)?;
    MusicXmlConverter::load_from_musicxml(score)
  }

  fn load_data(data: &[u8]) -> Result<Composition, String> {
    let data = str::from_utf8(data).map_err(|err| err.to_string())?;
    let score = musicxml::parser::parse_from_xml_str(data).map_err(|err| err.to_string())?;
    MusicXmlConverter::load_from_musicxml(score)
    // TODO: "Update MusicXML parser library to parse from raw data so we can do the partwise conversion if necessary"
  }

  fn save(path: &str, composition: &Composition) -> Result<usize, String> {
    let musicxml = MusicXmlConverter::save_to_musicxml(composition).map_err(|err| err.to_string())?;
    std::fs::write(path, musicxml.as_bytes()).map_err(|err| err.to_string())?;
    Ok(musicxml.as_bytes().len())
  }
}
