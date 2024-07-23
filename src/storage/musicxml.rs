use super::Convert;
use crate::{
  Accidental, ChordModificationType, Clef, ClefType, Composition, DirectionType, Duration, DynamicMarking,
  HandbellTechnique, Key, KeyMode, Note, NoteModificationType, PedalType, PhraseModificationType, Pitch,
  SectionModificationType, Tempo, TempoMarking, TimeSignature,
};
use musicxml;
use std::collections::HashMap;

pub struct MusicXmlConverter;

#[derive(Clone)]
struct PhraseModDetails {
  pub modification: PhraseModificationType,
  pub is_start: bool,
  pub number: Option<u8>,
}

impl std::fmt::Display for PhraseModDetails {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{} {} (ID: {})",
      self.modification,
      if self.is_start { "Start" } else { "End" },
      self.number.unwrap_or(0)
    )
  }
}

#[derive(Clone)]
enum TimeSliceContents {
  Direction(DirectionType),
  ChordModification(ChordModificationType),
  PhraseModification(PhraseModDetails),
  JumpTo(String),
  SectionStart(String),
  Ending {
    start: bool,
    numbers: Vec<u8>,
  },
  Repeat {
    start: bool,
    times: u32,
  },
  Note {
    pitch: Pitch,
    duration: Duration,
    accidental: Accidental,
    tied: bool,
    voice: Option<String>,
    tuplet: Option<(u32, u32)>,
    arpeggiated: bool,
    non_arpeggiated: bool,
    note_modifications: Vec<NoteModificationType>,
    phrase_modifications: Vec<PhraseModDetails>,
  },
}

impl std::fmt::Display for TimeSliceContents {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match *self {
        TimeSliceContents::Direction(ref direction) => format!("{}", direction),
        TimeSliceContents::ChordModification(ref chord_mod) => format!("Chord Modification: {}", chord_mod),
        TimeSliceContents::PhraseModification(ref phrase_mod) => format!("Phrase Modification: {}", phrase_mod),
        TimeSliceContents::JumpTo(ref jump_to) => format!("Jump To: {}", jump_to),
        TimeSliceContents::SectionStart(ref section_start) => format!("Section Start: {}", section_start),
        TimeSliceContents::Ending { start, ref numbers } => format!(
          "Ending: Start={} Iterations=[{}]",
          start,
          numbers
            .iter()
            .map(|number| number.to_string())
            .collect::<Vec<String>>()
            .join(", ")
        ),
        TimeSliceContents::Repeat { start, times } => format!("{} Repeat {} Times", if start { "Start" } else { "End" }, times),
        TimeSliceContents::Note {
          ref pitch,
          ref duration,
          ref accidental,
          tied,
          ref voice,
          ref tuplet,
          arpeggiated,
          non_arpeggiated,
          ref note_modifications,
          ref phrase_modifications,
        } => {
          let note_mods = note_modifications
            .iter()
            .map(|note_mod| format!("{}", note_mod))
            .collect::<Vec<String>>()
            .join(", ");
          let phrase_mods = phrase_modifications
            .iter()
            .map(|phrase_mod| format!("{}", phrase_mod))
            .collect::<Vec<String>>()
            .join(", ");
          format!(
            "{}: {}{}{}{} ({}{}{}{}{}{}{} )",
            format!("{}", if pitch.is_rest() { "Rest" } else { "Note" }),
            pitch,
            accidental,
            if pitch.is_rest() { "" } else { " " },
            duration,
            if tied { " Tied" } else { "" },
            format!(
              "{}{}",
              if voice.is_some() { " Voice=" } else { "" },
              if voice.is_some() {
                voice.clone().unwrap()
              } else {
                String::from("")
              }
            ),
            format!(
              "{}{}{}{}{}",
              if tuplet.is_some() { " Tuplet=(" } else { "" },
              if tuplet.is_some() {
                tuplet.clone().unwrap().0.to_string()
              } else {
                String::from("")
              },
              if tuplet.is_some() { ", " } else { "" },
              if tuplet.is_some() {
                tuplet.clone().unwrap().1.to_string()
              } else {
                String::from("")
              },
              if tuplet.is_some() { ")" } else { "" }
            ),
            if arpeggiated { " Arpeggiated" } else { "" },
            if non_arpeggiated { " NonArpeggiated" } else { "" },
            format!(
              "{}{}{}",
              if note_mods.is_empty() { "" } else { " Mods=[" },
              if note_mods.is_empty() { "" } else { note_mods.as_str() },
              if note_mods.is_empty() { "" } else { "]" },
            ),
            format!(
              "{}{}{}",
              if phrase_mods.is_empty() { "" } else { " PhraseMods=[" },
              if phrase_mods.is_empty() {
                ""
              } else {
                phrase_mods.as_str()
              },
              if phrase_mods.is_empty() { "" } else { "]" },
            ),
          )
        }
      }
    )
  }
}

struct TemporalPartData {
  pub data: HashMap<String, HashMap<String, Vec<Vec<TimeSliceContents>>>>,
}

impl std::fmt::Display for TemporalPartData {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    for (part_name, staves) in &self.data {
      write!(f, "\nPart: {}", part_name)?;
      for (staff_name, time_slices) in staves {
        writeln!(f, "\n  Staff: {}", staff_name)?;
        for (time, time_slice) in time_slices.iter().enumerate() {
          if !time_slice.is_empty() {
            write!(f, "    Time: {}\n      Items: [ ", time)?;
            for item in time_slice {
              write!(f, "\"{}\" ", item)?;
            }
            write!(f, "]\n")?;
          }
        }
      }
    }
    Ok(())
  }
}

impl MusicXmlConverter {
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
                return TimeSignature::Explicit(
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

  fn parse_tempo_from_metronome(metronome: &musicxml::elements::Metronome) -> Option<Tempo> {
    if let musicxml::elements::MetronomeContents::BeatBased(beat_data) = &metronome.content {
      let num_dots: u8 = beat_data.beat_unit_dot.len() as u8;
      let base_note = match beat_data.beat_unit.content {
        musicxml::datatypes::NoteTypeValue::Maxima => Duration::Maxima(num_dots),
        musicxml::datatypes::NoteTypeValue::Long => Duration::Long(num_dots),
        musicxml::datatypes::NoteTypeValue::Breve => Duration::Breve(num_dots),
        musicxml::datatypes::NoteTypeValue::Whole => Duration::Whole(num_dots),
        musicxml::datatypes::NoteTypeValue::Half => Duration::Half(num_dots),
        musicxml::datatypes::NoteTypeValue::Quarter => Duration::Quarter(num_dots),
        musicxml::datatypes::NoteTypeValue::Eighth => Duration::Eighth(num_dots),
        musicxml::datatypes::NoteTypeValue::Sixteenth => Duration::Sixteenth(num_dots),
        musicxml::datatypes::NoteTypeValue::ThirtySecond => Duration::ThirtySecond(num_dots),
        musicxml::datatypes::NoteTypeValue::SixtyFourth => Duration::SixtyFourth(num_dots),
        musicxml::datatypes::NoteTypeValue::OneHundredTwentyEighth => Duration::OneHundredTwentyEighth(num_dots),
        musicxml::datatypes::NoteTypeValue::TwoHundredFiftySixth => Duration::TwoHundredFiftySixth(num_dots),
        musicxml::datatypes::NoteTypeValue::FiveHundredTwelfth => Duration::FiveHundredTwelfth(num_dots),
        musicxml::datatypes::NoteTypeValue::OneThousandTwentyFourth => Duration::OneThousandTwentyFourth(num_dots),
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
          base_note: Duration::Quarter(0),
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
            "publisher" => composition.set_publisher(&creator.content.as_str()),
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

  fn find_parts(parts_list: &Vec<musicxml::elements::PartListElement>) -> HashMap<String, String> {
    let mut parts_map: HashMap<String, String> = HashMap::new();
    for parts_list_element in parts_list {
      match parts_list_element {
        musicxml::elements::PartListElement::ScorePart(score_part) => {
          parts_map.insert(
            (*score_part.attributes.id).clone(),
            score_part.content.part_name.content.clone(),
          );
        }
        _ => (),
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

  fn find_num_measures(part_elements: &Vec<musicxml::elements::PartElement>) -> usize {
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
    time_slices: &mut HashMap<String, Vec<Vec<TimeSliceContents>>>,
    cursor: usize,
  ) -> isize {
    element.clef.iter().for_each(|item| {
      let staff_name = if let Some(number) = &item.attributes.number {
        number.to_string()
      } else {
        String::from("1")
      };
      let item = TimeSliceContents::Direction(DirectionType::Clef {
        clef: match item.content.sign.content {
          musicxml::datatypes::ClefSign::G => match &item.content.line {
            Some(line) => match *line.content {
              1 => Clef::FrenchViolin,
              _ => Clef::Treble,
            },
            None => Clef::Treble,
          },
          musicxml::datatypes::ClefSign::F => match &item.content.line {
            Some(line) => match *line.content {
              3 => Clef::Baritone(ClefType::FClef),
              5 => Clef::Subbass,
              _ => Clef::Bass,
            },
            None => Clef::Bass,
          },
          musicxml::datatypes::ClefSign::C => match &item.content.line {
            Some(line) => match *line.content {
              1 => Clef::Soprano,
              2 => Clef::MezzoSoprano,
              4 => Clef::Tenor,
              5 => Clef::Baritone(ClefType::CClef),
              _ => Clef::Alto,
            },
            None => Clef::Alto,
          },
          _ => Clef::Treble,
        },
      });
      time_slices.get_mut(&staff_name).unwrap()[cursor].push(item);
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
        let item = TimeSliceContents::Direction(DirectionType::Key {
          key: Key::from_fifths(*key.fifths.content, Some(mode)),
        });
        time_slices.get_mut(&staff_name).unwrap()[cursor].push(item);
      }
    });
    element.time.iter().for_each(|item| {
      let staff_name = if let Some(number) = &item.attributes.number {
        number.to_string()
      } else {
        String::from("1")
      };
      let item = if item.content.senza_misura.is_some() {
        TimeSliceContents::Direction(DirectionType::TimeSignature {
          time_signature: TimeSignature::None,
        })
      } else {
        let beat_element = &item.content.beats[0];
        TimeSliceContents::Direction(DirectionType::TimeSignature {
          time_signature: TimeSignature::Explicit(
            (*beat_element.beats.content).parse().unwrap(),
            (*beat_element.beat_type.content).parse().unwrap(),
          ),
        })
      };
      time_slices.get_mut(&staff_name).unwrap()[cursor].push(item);
    });
    0
  }

  fn parse_backup_element(element: &musicxml::elements::BackupContents) -> isize {
    return -(*element.duration.content as isize);
  }

  fn parse_forward_element(element: &musicxml::elements::ForwardContents) -> isize {
    return *element.duration.content as isize;
  }

  fn parse_direction_element(
    element: &musicxml::elements::Direction,
    time_slice: &mut HashMap<String, Vec<Vec<TimeSliceContents>>>,
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
          let item = TimeSliceContents::SectionStart(rehearsal[0].content.clone());
          time_slice.get_mut(&staff_name).unwrap()[cursor].push(item);
        }
        musicxml::elements::DirectionTypeContents::Segno(_segno) => {
          let item = TimeSliceContents::SectionStart(String::from("Segno"));
          time_slice.get_mut(&staff_name).unwrap()[cursor].push(item);
        }
        musicxml::elements::DirectionTypeContents::Coda(_coda) => {
          let item = TimeSliceContents::SectionStart(String::from("Coda"));
          time_slice.get_mut(&staff_name).unwrap()[cursor].push(item);
        }
        musicxml::elements::DirectionTypeContents::Wedge(wedge) => {
          if wedge.attributes.r#type != musicxml::datatypes::WedgeType::Continue {
            let item = TimeSliceContents::PhraseModification(PhraseModDetails {
              modification: match wedge.attributes.r#type {
                musicxml::datatypes::WedgeType::Diminuendo => PhraseModificationType::Decrescendo {
                  final_dynamic: DynamicMarking::None,
                },
                _ => PhraseModificationType::Crescendo {
                  final_dynamic: DynamicMarking::None,
                },
              },
              is_start: wedge.attributes.r#type != musicxml::datatypes::WedgeType::Stop,
              number: wedge.attributes.number.as_ref().map(|number| **number),
            });
            time_slice.get_mut(&staff_name).unwrap()[cursor].push(item);
          }
        }
        musicxml::elements::DirectionTypeContents::Dynamics(dynamics) => {
          let dynamic_type = match &dynamics[0].content[0] {
            musicxml::elements::DynamicsType::P(_) => Some(DynamicMarking::Piano(1)),
            musicxml::elements::DynamicsType::Pp(_) => Some(DynamicMarking::Piano(2)),
            musicxml::elements::DynamicsType::Ppp(_) => Some(DynamicMarking::Piano(3)),
            musicxml::elements::DynamicsType::Pppp(_) => Some(DynamicMarking::Piano(4)),
            musicxml::elements::DynamicsType::Ppppp(_) => Some(DynamicMarking::Piano(5)),
            musicxml::elements::DynamicsType::Pppppp(_) => Some(DynamicMarking::Piano(6)),
            musicxml::elements::DynamicsType::F(_) => Some(DynamicMarking::Forte(1)),
            musicxml::elements::DynamicsType::Ff(_) => Some(DynamicMarking::Forte(2)),
            musicxml::elements::DynamicsType::Fff(_) => Some(DynamicMarking::Forte(3)),
            musicxml::elements::DynamicsType::Ffff(_) => Some(DynamicMarking::Forte(4)),
            musicxml::elements::DynamicsType::Fffff(_) => Some(DynamicMarking::Forte(5)),
            musicxml::elements::DynamicsType::Ffffff(_) => Some(DynamicMarking::Forte(6)),
            musicxml::elements::DynamicsType::Mp(_) => Some(DynamicMarking::MezzoPiano),
            musicxml::elements::DynamicsType::Mf(_) => Some(DynamicMarking::MezzoForte),
            musicxml::elements::DynamicsType::N(_) | musicxml::elements::DynamicsType::OtherDynamics(_) => None,
            _ => Some(DynamicMarking::None),
          };
          if let Some(dynamic_type) = dynamic_type {
            let item = if dynamic_type == DynamicMarking::None {
              TimeSliceContents::ChordModification(ChordModificationType::Accent)
            } else {
              TimeSliceContents::Direction(DirectionType::Dynamic { dynamic: dynamic_type })
            };
            time_slice.get_mut(&staff_name).unwrap()[cursor].push(item);
          }
        }
        musicxml::elements::DirectionTypeContents::Pedal(pedal) => match &pedal.attributes.r#type {
          musicxml::datatypes::PedalType::Start => {
            let item = TimeSliceContents::PhraseModification(PhraseModDetails {
              modification: PhraseModificationType::Pedal {
                r#type: PedalType::Sustain,
              },
              is_start: true,
              number: pedal.attributes.number.as_ref().map(|number| **number),
            });
            time_slice.get_mut(&staff_name).unwrap()[cursor].push(item);
          }
          musicxml::datatypes::PedalType::Stop => {
            let item = TimeSliceContents::PhraseModification(PhraseModDetails {
              modification: PhraseModificationType::Pedal {
                r#type: PedalType::Sustain,
              },
              is_start: false,
              number: pedal.attributes.number.as_ref().map(|number| **number),
            });
            time_slice.get_mut(&staff_name).unwrap()[cursor].push(item);
          }
          musicxml::datatypes::PedalType::Sostenuto => {
            let item = TimeSliceContents::PhraseModification(PhraseModDetails {
              modification: PhraseModificationType::Pedal {
                r#type: PedalType::Sostenuto,
              },
              is_start: true,
              number: pedal.attributes.number.as_ref().map(|number| **number),
            });
            time_slice.get_mut(&staff_name).unwrap()[cursor].push(item);
          }
          musicxml::datatypes::PedalType::Change => {
            let item1 = TimeSliceContents::PhraseModification(PhraseModDetails {
              modification: PhraseModificationType::Pedal {
                r#type: PedalType::Sustain,
              },
              is_start: false,
              number: pedal.attributes.number.as_ref().map(|number| **number),
            });
            let item2 = TimeSliceContents::PhraseModification(PhraseModDetails {
              modification: PhraseModificationType::Pedal {
                r#type: PedalType::Sustain,
              },
              is_start: true,
              number: pedal.attributes.number.as_ref().map(|number| **number),
            });
            time_slice.get_mut(&staff_name).unwrap()[cursor].push(item1);
            time_slice.get_mut(&staff_name).unwrap()[cursor].push(item2);
          }
          _ => (),
        },
        musicxml::elements::DirectionTypeContents::OctaveShift(octave_shift) => {
          if octave_shift.attributes.r#type != musicxml::datatypes::UpDownStopContinue::Continue {
            let item = TimeSliceContents::PhraseModification(PhraseModDetails {
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
            });
            time_slice.get_mut(&staff_name).unwrap()[cursor].push(item);
          }
        }
        musicxml::elements::DirectionTypeContents::Metronome(metronome) => {
          if let Some(tempo) = Self::parse_tempo_from_metronome(metronome) {
            let item = TimeSliceContents::Direction(DirectionType::TempoChange { tempo });
            time_slice.get_mut(&staff_name).unwrap()[cursor].push(item);
          }
        }
        musicxml::elements::DirectionTypeContents::AccordionRegistration(registration) => {
          let item = TimeSliceContents::Direction(DirectionType::AccordionRegistration {
            high: registration.content.accordion_high.is_some(),
            middle: registration
              .content
              .accordion_middle
              .as_ref()
              .map(|middle| *middle.content)
              .unwrap_or(0),
            low: registration.content.accordion_low.is_some(),
          });
          time_slice.get_mut(&staff_name).unwrap()[cursor].push(item);
        }
        musicxml::elements::DirectionTypeContents::StringMute(string_mute) => {
          let item = TimeSliceContents::Direction(DirectionType::StringMute {
            on: string_mute.attributes.r#type == musicxml::datatypes::OnOff::On,
          });
          time_slice.get_mut(&staff_name).unwrap()[cursor].push(item);
        }
        _ => (),
      });
    0
  }

  fn parse_barline_element(
    element: &musicxml::elements::Barline,
    time_slice: &mut HashMap<String, Vec<Vec<TimeSliceContents>>>,
    cursor: usize,
  ) -> isize {
    if let Some(ending) = &element.content.ending {
      let item = TimeSliceContents::Ending {
        start: ending.attributes.r#type == musicxml::datatypes::StartStopDiscontinue::Start,
        numbers: ending
          .attributes
          .number
          .split(&[',', ' '][..])
          .map(|item| item.parse().unwrap())
          .collect(),
      };
      for slice in time_slice.values_mut() {
        slice[cursor].push(item.clone());
      }
    }
    if let Some(repeat) = &element.content.repeat {
      let item = TimeSliceContents::Repeat {
        start: repeat.attributes.direction == musicxml::datatypes::BackwardForward::Forward,
        times: repeat.attributes.times.as_ref().map(|item| **item).unwrap_or(1),
      };
      for slice in time_slice.values_mut() {
        slice[cursor].push(item.clone());
      }
    }
    if let Some(_) = &element.content.coda {
      let item = TimeSliceContents::SectionStart(String::from("Coda"));
      for slice in time_slice.values_mut() {
        slice[cursor].push(item.clone());
      }
    }
    if let Some(_) = &element.content.segno {
      let item = TimeSliceContents::SectionStart(String::from("Segno"));
      for slice in time_slice.values_mut() {
        slice[cursor].push(item.clone());
      }
    }
    if let Some(_) = &element.attributes.coda {
      let item = TimeSliceContents::JumpTo(String::from("Coda"));
      for slice in time_slice.values_mut() {
        slice[cursor].push(item.clone());
      }
    }
    if let Some(_) = &element.attributes.segno {
      let item = TimeSliceContents::JumpTo(String::from("Segno"));
      for slice in time_slice.values_mut() {
        slice[cursor].push(item.clone());
      }
    }
    0
  }

  fn parse_note_element(
    note: &musicxml::elements::Note,
    time_slices: &mut HashMap<String, Vec<Vec<TimeSliceContents>>>,
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
      musicxml::elements::NoteType::Cue(cue) => {
        (*cue.duration.content as usize, false, cue.chord.is_some(), Pitch::Rest)
      }
      musicxml::elements::NoteType::Grace(grace) => match &grace.info {
        musicxml::elements::GraceType::Cue(cue) => (0, false, cue.chord.is_some(), Pitch::Rest),
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
                musicxml::datatypes::Step::A => Pitch::A(octave),
                musicxml::datatypes::Step::B => Pitch::B(octave),
                musicxml::datatypes::Step::C => Pitch::C(octave),
                musicxml::datatypes::Step::D => Pitch::D(octave),
                musicxml::datatypes::Step::E => Pitch::E(octave),
                musicxml::datatypes::Step::F => Pitch::F(octave),
                musicxml::datatypes::Step::G => Pitch::G(octave),
              }
            }
            _ => Pitch::Rest,
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
              musicxml::datatypes::Step::A => Pitch::A(octave),
              musicxml::datatypes::Step::B => Pitch::B(octave),
              musicxml::datatypes::Step::C => Pitch::C(octave),
              musicxml::datatypes::Step::D => Pitch::D(octave),
              musicxml::datatypes::Step::E => Pitch::E(octave),
              musicxml::datatypes::Step::F => Pitch::F(octave),
              musicxml::datatypes::Step::G => Pitch::G(octave),
            }
          }
          _ => Pitch::Rest,
        },
      ),
    };
    let duration = if let Some(note_type) = &note.content.r#type {
      match &note_type.content {
        musicxml::datatypes::NoteTypeValue::Maxima => Duration::Maxima(num_dots),
        musicxml::datatypes::NoteTypeValue::Long => Duration::Long(num_dots),
        musicxml::datatypes::NoteTypeValue::Breve => Duration::Breve(num_dots),
        musicxml::datatypes::NoteTypeValue::Whole => Duration::Whole(num_dots),
        musicxml::datatypes::NoteTypeValue::Half => Duration::Half(num_dots),
        musicxml::datatypes::NoteTypeValue::Eighth => Duration::Eighth(num_dots),
        musicxml::datatypes::NoteTypeValue::Sixteenth => Duration::Sixteenth(num_dots),
        musicxml::datatypes::NoteTypeValue::ThirtySecond => Duration::ThirtySecond(num_dots),
        musicxml::datatypes::NoteTypeValue::SixtyFourth => Duration::SixtyFourth(num_dots),
        musicxml::datatypes::NoteTypeValue::OneHundredTwentyEighth => Duration::OneHundredTwentyEighth(num_dots),
        musicxml::datatypes::NoteTypeValue::TwoHundredFiftySixth => Duration::TwoHundredFiftySixth(num_dots),
        musicxml::datatypes::NoteTypeValue::FiveHundredTwelfth => Duration::FiveHundredTwelfth(num_dots),
        musicxml::datatypes::NoteTypeValue::OneThousandTwentyFourth => Duration::OneThousandTwentyFourth(num_dots),
        _ => Duration::Quarter(num_dots),
      }
    } else {
      match divisions {
        _ if divisions / divisions_per_quarter_note >= 32 => Duration::Maxima(MusicXmlConverter::calculate_num_dots(
          32 * divisions_per_quarter_note,
          divisions,
        )),
        _ if divisions / divisions_per_quarter_note >= 16 => Duration::Long(MusicXmlConverter::calculate_num_dots(
          16 * divisions_per_quarter_note,
          divisions,
        )),
        _ if divisions / divisions_per_quarter_note >= 8 => Duration::Breve(MusicXmlConverter::calculate_num_dots(
          8 * divisions_per_quarter_note,
          divisions,
        )),
        _ if divisions / divisions_per_quarter_note >= 4 => Duration::Whole(MusicXmlConverter::calculate_num_dots(
          4 * divisions_per_quarter_note,
          divisions,
        )),
        _ if divisions / divisions_per_quarter_note >= 2 => Duration::Half(MusicXmlConverter::calculate_num_dots(
          2 * divisions_per_quarter_note,
          divisions,
        )),
        _ if divisions / divisions_per_quarter_note >= 1 => Duration::Quarter(MusicXmlConverter::calculate_num_dots(
          1 * divisions_per_quarter_note,
          divisions,
        )),
        _ if divisions_per_quarter_note / divisions <= 2 => Duration::Eighth(MusicXmlConverter::calculate_num_dots(
          divisions_per_quarter_note / 2,
          divisions,
        )),
        _ if divisions_per_quarter_note / divisions <= 4 => Duration::Sixteenth(MusicXmlConverter::calculate_num_dots(
          divisions_per_quarter_note / 4,
          divisions,
        )),
        _ if divisions_per_quarter_note / divisions <= 8 => Duration::ThirtySecond(
          MusicXmlConverter::calculate_num_dots(divisions_per_quarter_note / 8, divisions),
        ),
        _ if divisions_per_quarter_note / divisions <= 16 => Duration::SixtyFourth(
          MusicXmlConverter::calculate_num_dots(divisions_per_quarter_note / 16, divisions),
        ),
        _ if divisions_per_quarter_note / divisions <= 32 => Duration::OneHundredTwentyEighth(
          MusicXmlConverter::calculate_num_dots(divisions_per_quarter_note / 32, divisions),
        ),
        _ if divisions_per_quarter_note / divisions <= 64 => Duration::TwoHundredFiftySixth(
          MusicXmlConverter::calculate_num_dots(divisions_per_quarter_note / 64, divisions),
        ),
        _ if divisions_per_quarter_note / divisions <= 128 => Duration::FiveHundredTwelfth(
          MusicXmlConverter::calculate_num_dots(divisions_per_quarter_note / 128, divisions),
        ),
        _ if divisions_per_quarter_note / divisions <= 256 => Duration::OneThousandTwentyFourth(
          MusicXmlConverter::calculate_num_dots(divisions_per_quarter_note / 256, divisions),
        ),
        _ => Duration::TwoThousandFortyEighth(num_dots),
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
    let tuplet = note.content.time_modification.as_ref().map(|time_modification| {
      let actual_notes = *time_modification.content.actual_notes.content;
      let normal_notes = *time_modification.content.normal_notes.content;
      (actual_notes, normal_notes)
    });
    let (mut arpeggiate, mut non_arpeggiate) = (false, false);
    let mut note_modifications: Vec<NoteModificationType> = Vec::new();
    let mut phrase_modifications: Vec<PhraseModDetails> = Vec::new();
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
              phrase_modifications.push(PhraseModDetails {
                modification: PhraseModificationType::Legato,
                is_start: slur.attributes.r#type == musicxml::datatypes::StartStopContinue::Start,
                number: match &slur.attributes.number {
                  Some(number) => Some(**number),
                  None => None,
                },
              })
            }
          }
          musicxml::elements::NotationContentTypes::Tuplet(_tuplet) => {} // Ignore in favor of <time-modification>
          musicxml::elements::NotationContentTypes::Glissando(glissando) => {
            phrase_modifications.push(PhraseModDetails {
              modification: PhraseModificationType::Glissando,
              is_start: glissando.attributes.r#type == musicxml::datatypes::StartStop::Start,
              number: match &glissando.attributes.number {
                Some(number) => Some(**number),
                None => None,
              },
            })
          }
          musicxml::elements::NotationContentTypes::Slide(slide) => phrase_modifications.push(PhraseModDetails {
            modification: PhraseModificationType::Portamento,
            is_start: slide.attributes.r#type == musicxml::datatypes::StartStop::Start,
            number: match &slide.attributes.number {
              Some(number) => Some(**number),
              None => None,
            },
          }),
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
                          phrase_modifications.push(PhraseModDetails {
                            modification: PhraseModificationType::Tremolo { relative_speed },
                            is_start: true,
                            number: None,
                          });
                          None
                        }
                        musicxml::datatypes::TremoloType::Stop => {
                          phrase_modifications.push(PhraseModDetails {
                            modification: PhraseModificationType::Tremolo { relative_speed },
                            is_start: false,
                            number: None,
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
                      .push(TimeSliceContents::Direction(DirectionType::BreathMark));
                    None
                  }
                  musicxml::elements::ArticulationsType::Caesura(_caesura) => {
                    time_slices.get_mut(&staff_name).unwrap()[cursor]
                      .push(TimeSliceContents::Direction(DirectionType::Caesura));
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
              dynamic: DynamicMarking::Piano(1),
            }),
            musicxml::elements::DynamicsType::Pp(_pp) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: DynamicMarking::Piano(2),
            }),
            musicxml::elements::DynamicsType::Ppp(_ppp) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: DynamicMarking::Piano(3),
            }),
            musicxml::elements::DynamicsType::Pppp(_pppp) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: DynamicMarking::Piano(4),
            }),
            musicxml::elements::DynamicsType::Ppppp(_pppp_it) => {
              note_modifications.push(NoteModificationType::Dynamic {
                dynamic: DynamicMarking::Piano(5),
              })
            }
            musicxml::elements::DynamicsType::Pppppp(_ppp_it) => {
              note_modifications.push(NoteModificationType::Dynamic {
                dynamic: DynamicMarking::Piano(6),
              })
            }
            musicxml::elements::DynamicsType::F(_f) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: DynamicMarking::Forte(1),
            }),
            musicxml::elements::DynamicsType::Ff(_ff) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: DynamicMarking::Forte(2),
            }),
            musicxml::elements::DynamicsType::Fff(_fff) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: DynamicMarking::Forte(3),
            }),
            musicxml::elements::DynamicsType::Ffff(_ffff) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: DynamicMarking::Forte(4),
            }),
            musicxml::elements::DynamicsType::Fffff(_ffff_it) => {
              note_modifications.push(NoteModificationType::Dynamic {
                dynamic: DynamicMarking::Forte(5),
              })
            }
            musicxml::elements::DynamicsType::Ffffff(_fff_it) => {
              note_modifications.push(NoteModificationType::Dynamic {
                dynamic: DynamicMarking::Forte(6),
              })
            }
            musicxml::elements::DynamicsType::Mp(_mp) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: DynamicMarking::MezzoPiano,
            }),
            musicxml::elements::DynamicsType::Mf(_mf) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: DynamicMarking::MezzoForte,
            }),
            musicxml::elements::DynamicsType::N(_) | musicxml::elements::DynamicsType::OtherDynamics(_) => (),
            _ => note_modifications.push(NoteModificationType::Accent),
          },
          musicxml::elements::NotationContentTypes::Fermata(_fermata) => {
            note_modifications.push(NoteModificationType::Fermata { relative_duration: 2 })
          }
          musicxml::elements::NotationContentTypes::Arpeggiate(_arpeggiate) => arpeggiate = true,
          musicxml::elements::NotationContentTypes::NonArpeggiate(_non_arpeggiate) => non_arpeggiate = true,
          _ => {}
        });
    });
    if let Some(pizzicato) = &note.attributes.pizzicato {
      if *pizzicato == musicxml::datatypes::YesNo::Yes
        && !note_modifications.iter().any(|modification| match modification {
          NoteModificationType::Pizzicato => true,
          _ => false,
        })
      {
        note_modifications.push(NoteModificationType::Pizzicato);
      }
    }
    let item = TimeSliceContents::Note {
      pitch,
      duration,
      accidental: accidental.unwrap_or(Accidental::None),
      tied,
      voice,
      tuplet,
      arpeggiated: arpeggiate,
      non_arpeggiated: non_arpeggiate,
      note_modifications,
      phrase_modifications,
    };
    if chord {
      time_slices.get_mut(&staff_name).unwrap()[previous_cursor].push(item);
      0
    } else {
      time_slices.get_mut(&staff_name).unwrap()[cursor].push(item);
      divisions as isize
    }
  }
}

impl Convert for MusicXmlConverter {
  fn load(path: &str) -> Result<Composition, String> {
    // Parse the MusicXML score representation
    let score = musicxml::read_score_partwise(path)?;

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
    parts_map.values().for_each(|name| {
      composition.add_part(name);
    });
    if score.content.part.is_empty() {
      return Err(String::from("No parts found in the MusicXML score"));
    } else if score.content.part[0].content.is_empty() {
      return Err(String::from(
        "No measures found in the first part of the MusicXML score",
      ));
    }

    // Parse the initial musical attributes of the score
    composition.set_starting_key(MusicXmlConverter::find_starting_key(&score.content.part));
    composition.set_starting_time_signature(MusicXmlConverter::find_starting_time_signature(&score.content.part));
    composition.set_tempo(MusicXmlConverter::find_tempo(&score.content.part));

    // Create a data structure to hold all temporally parsed musical data
    let mut part_data = TemporalPartData { data: HashMap::new() };
    for part in &score.content.part {
      let part_name = parts_map
        .get(&*part.attributes.id)
        .expect("Unknown Part ID encountered");
      let max_divisions = MusicXmlConverter::find_divisions_per_quarter_note(&part.content)
        * MusicXmlConverter::find_max_num_quarter_notes_per_measure(&part.content)
        * MusicXmlConverter::find_num_measures(&part.content);
      part_data.data.insert(part_name.clone(), HashMap::new());
      let part_staves = part_data.data.get_mut(part_name).unwrap();
      MusicXmlConverter::find_staves(&part.content).iter().for_each(|staff| {
        part_staves.insert(staff.clone(), vec![Vec::new(); max_divisions]);
      });
    }

    // Parse the actual musical contents of the score into discrete time slices
    for part in &score.content.part {
      let (mut cursor, mut previous_cursor): (usize, usize) = (0, 0);
      let divisions_per_quarter_note = MusicXmlConverter::find_divisions_per_quarter_note(&part.content);
      let time_slices = part_data.data.get_mut(parts_map.get(&*part.attributes.id).unwrap()).unwrap();
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

    // Use the temporally ordered time slices to construct a final composition structure
    print!("{}", part_data);
    /*for part in &score.content.part {
    let part_name = parts_map
      .get(&*part.attributes.id)
      .expect("Unknown Part ID encountered");
    let composition_part = composition.get_part(&part_name).expect("Unknown part encountered");*/

    // Return the composition
    Ok(composition)
  }

  fn save(path: &str, composition: &Composition) -> Result<usize, String> {
    //fs::write(path, contents)?;
    Ok(0)
  }
}
