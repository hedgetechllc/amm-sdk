use super::Load;
#[allow(clippy::wildcard_imports)]
use crate::{context::*, modification::*, note::*, structure::*, Composition};
use alloc::{
  collections::BTreeMap,
  string::{String, ToString},
  vec::Vec,
};
use core::{
  str,
  sync::atomic::{AtomicUsize, Ordering},
};
use musicxml::{self, elements::ScorePartwise};

pub struct MusicXmlConverter;

#[derive(Clone, Debug)]
struct PhraseModDetails {
  pub modification: PhraseModificationType,
  pub is_start: bool,
  pub number: Option<u8>,
  pub for_voice: Option<String>,
  pub combine_with_next: bool,
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
      match self.for_voice.as_ref() {
        Some(voice) => format!(", Voice: {voice}"),
        None => String::new(),
      }
    )
  }
}

#[derive(Clone, Debug)]
struct NoteDetails {
  pub pitch: Pitch,
  pub duration: Duration,
  pub accidental: Accidental,
  pub divisions: usize,
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
      if let Some(voice) = &self.voice {
        voice.clone()
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

fn section_generate_id() -> usize {
  static COUNTER: AtomicUsize = AtomicUsize::new(1);
  COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone, Debug, Default)]
struct SectionDetails {
  pub starting_sections: BTreeMap<usize, Section>,
  pub ending_sections: Vec<usize>,
}

impl SectionDetails {
  pub fn new_section(&mut self, name: &str) -> (usize, &mut Section) {
    let section_id = section_generate_id();
    self.starting_sections.insert(section_id, Section::new(name));
    unsafe {
      (
        section_id,
        self.starting_sections.get_mut(&section_id).unwrap_unchecked(),
      )
    }
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for SectionDetails {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let ending_sections = self
      .ending_sections
      .iter()
      .map(ToString::to_string)
      .collect::<Vec<String>>()
      .join(", ");
    let starting_sections = self
      .starting_sections
      .iter()
      .map(|(key, val)| key.to_string() + ": " + val.to_string().as_str())
      .collect::<Vec<String>>()
      .join(", ");
    write!(
      f,
      "   Ending Sections: [{ending_sections}],\n   Starting Sections: [{starting_sections}]",
    )
  }
}

#[derive(Debug, Default, Clone)]
struct TimeSliceContainer {
  pub direction: Vec<DirectionType>,
  pub chord_modification: Vec<ChordModificationType>,
  pub phrase_modification_start: Vec<PhraseModDetails>,
  pub phrase_modification_end: Vec<PhraseModDetails>,
  pub jump_to: Option<String>,
  pub section_start: Option<String>,
  pub ending: Vec<(bool, Vec<u8>)>,
  pub repeat: Vec<(bool, u8)>,
  pub tempo_change_explicit: Option<Tempo>,
  pub tempo_change_implicit: Option<TempoSuggestion>,
  pub notes: Vec<NoteDetails>,
}

impl TimeSliceContainer {
  pub fn is_empty(&self) -> bool {
    self.direction.is_empty()
      && self.chord_modification.is_empty()
      && self.phrase_modification_start.is_empty()
      && self.phrase_modification_end.is_empty()
      && self.jump_to.is_none()
      && self.section_start.is_none()
      && self.ending.is_empty()
      && self.repeat.is_empty()
      && self.tempo_change_explicit.is_none()
      && self.tempo_change_implicit.is_none()
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

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
struct MusicalItem {
  divisions: usize,
  note: Option<Note>,
  chord: Option<Chord>,
  new_phrase_modifications: Vec<PhraseModDetails>,
  ending_phrase_modifications: Vec<PhraseModDetails>,
}

impl MusicXmlConverter {
  #[allow(clippy::cast_possible_truncation)]
  fn calculate_num_dots(base_divisions: usize, total_divisions: usize) -> u8 {
    let (mut num_dots, mut remaining_divisions) = (0, total_divisions - base_divisions);
    while remaining_divisions > 0 && num_dots < 8 {
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
              if let Some(beat_element) = time_element.content.beats.first() {
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

  #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
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

  #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
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

  fn convert_divisions_to_duration(divisions: usize, divisions_per_quarter_note: usize, num_dots: u8) -> Duration {
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
        MusicXmlConverter::calculate_num_dots(divisions_per_quarter_note, divisions),
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
      let item = DirectionType::ClefChange {
        clef: match &item.content.sign.content {
          musicxml::datatypes::ClefSign::G => match &item.content.line {
            Some(line) => match *line.content {
              1 => Clef::new(ClefType::FrenchViolin, None),
              _ => Clef::new(ClefType::Treble, None),
            },
            None => Clef::new(ClefType::Treble, None),
          },
          musicxml::datatypes::ClefSign::F => match &item.content.line {
            Some(line) => match *line.content {
              3 => Clef::new(ClefType::Baritone, Some(ClefSymbol::FClef)),
              5 => Clef::new(ClefType::Subbass, None),
              _ => Clef::new(ClefType::Bass, None),
            },
            None => Clef::new(ClefType::Bass, None),
          },
          musicxml::datatypes::ClefSign::C => match &item.content.line {
            Some(line) => match *line.content {
              1 => Clef::new(ClefType::Soprano, None),
              2 => Clef::new(ClefType::MezzoSoprano, None),
              4 => Clef::new(ClefType::Tenor, None),
              5 => Clef::new(ClefType::Baritone, Some(ClefSymbol::CClef)),
              _ => Clef::new(ClefType::Alto, None),
            },
            None => Clef::new(ClefType::Alto, None),
          },
          _ => Clef::new(ClefType::Treble, None),
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
        let item = DirectionType::KeyChange {
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
        DirectionType::TimeSignatureChange {
          time_signature: TimeSignature::new(TimeSignatureType::None),
        }
      } else {
        let beat_element = &item.content.beats[0];
        DirectionType::TimeSignatureChange {
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

  #[allow(clippy::cast_possible_wrap)]
  fn parse_backup_element(element: &musicxml::elements::BackupContents) -> isize {
    -(*element.duration.content as isize)
  }

  #[allow(clippy::cast_possible_wrap)]
  fn parse_forward_element(element: &musicxml::elements::ForwardContents) -> isize {
    *element.duration.content as isize
  }

  fn parse_direction_element(
    element: &musicxml::elements::Direction,
    time_slice: &mut BTreeMap<String, Vec<TimeSliceContainer>>,
    open_wedges: &mut BTreeMap<Option<u8>, Vec<musicxml::datatypes::WedgeType>>,
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
          time_slice.get_mut(&staff_name).unwrap()[cursor].section_start = Some(rehearsal[0].content.clone());
        }
        musicxml::elements::DirectionTypeContents::Segno(_segno) => {
          time_slice.get_mut(&staff_name).unwrap()[cursor].section_start = Some(String::from("Segno"));
        }
        musicxml::elements::DirectionTypeContents::Coda(_coda) => {
          time_slice.get_mut(&staff_name).unwrap()[cursor].section_start = Some(String::from("Coda"));
        }
        musicxml::elements::DirectionTypeContents::Wedge(wedge) => {
          if wedge.attributes.r#type != musicxml::datatypes::WedgeType::Continue {
            let wedge_number = wedge.attributes.number.as_ref().map(|number| **number);
            let item = PhraseModDetails {
              modification: match wedge.attributes.r#type {
                musicxml::datatypes::WedgeType::Diminuendo => {
                  if let Some(open_wedge) = open_wedges.get_mut(&wedge_number) {
                    open_wedge.push(musicxml::datatypes::WedgeType::Diminuendo);
                  } else {
                    open_wedges.insert(wedge_number, Vec::from([musicxml::datatypes::WedgeType::Diminuendo]));
                  }
                  PhraseModificationType::Decrescendo { final_dynamic: None }
                }
                musicxml::datatypes::WedgeType::Stop => {
                  if let Some(open_wedge) = open_wedges.get_mut(&wedge_number) {
                    let wedge_type = open_wedge.pop();
                    if open_wedge.is_empty() {
                      open_wedges.remove(&wedge_number);
                    }
                    match wedge_type {
                      Some(musicxml::datatypes::WedgeType::Crescendo) => {
                        PhraseModificationType::Crescendo { final_dynamic: None }
                      }
                      _ => PhraseModificationType::Decrescendo { final_dynamic: None },
                    }
                  } else {
                    PhraseModificationType::Crescendo { final_dynamic: None }
                  }
                }
                _ => {
                  if let Some(open_wedge) = open_wedges.get_mut(&wedge_number) {
                    open_wedge.push(musicxml::datatypes::WedgeType::Crescendo);
                  } else {
                    open_wedges.insert(wedge_number, Vec::from([musicxml::datatypes::WedgeType::Crescendo]));
                  }
                  PhraseModificationType::Crescendo { final_dynamic: None }
                }
              },
              is_start: wedge.attributes.r#type != musicxml::datatypes::WedgeType::Stop,
              number: wedge.attributes.number.as_ref().map(|number| **number),
              for_voice: None,
              combine_with_next: false,
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
          let mut chord_mod = false;
          let dynamic_type = match &dynamics[0].content[0] {
            musicxml::elements::DynamicsType::P(_) => Some(Dynamic::Piano(1)),
            musicxml::elements::DynamicsType::Pp(_) => Some(Dynamic::Piano(2)),
            musicxml::elements::DynamicsType::Ppp(_) => Some(Dynamic::Piano(3)),
            musicxml::elements::DynamicsType::Pppp(_) => Some(Dynamic::Piano(4)),
            musicxml::elements::DynamicsType::Ppppp(_) => Some(Dynamic::Piano(5)),
            musicxml::elements::DynamicsType::Pppppp(_) => Some(Dynamic::Piano(6)),
            musicxml::elements::DynamicsType::F(_) => Some(Dynamic::Forte(1)),
            musicxml::elements::DynamicsType::Ff(_) => Some(Dynamic::Forte(2)),
            musicxml::elements::DynamicsType::Fff(_) => Some(Dynamic::Forte(3)),
            musicxml::elements::DynamicsType::Ffff(_) => Some(Dynamic::Forte(4)),
            musicxml::elements::DynamicsType::Fffff(_) => Some(Dynamic::Forte(5)),
            musicxml::elements::DynamicsType::Ffffff(_) => Some(Dynamic::Forte(6)),
            musicxml::elements::DynamicsType::Mp(_) => Some(Dynamic::MezzoPiano),
            musicxml::elements::DynamicsType::Mf(_) => Some(Dynamic::MezzoForte),
            musicxml::elements::DynamicsType::N(_) | musicxml::elements::DynamicsType::OtherDynamics(_) => None,
            _ => {
              chord_mod = true;
              None
            }
          };
          if let Some(dynamic_type) = dynamic_type {
            time_slice.get_mut(&staff_name).unwrap()[cursor]
              .direction
              .push(DirectionType::Dynamic { dynamic: dynamic_type });
          } else if chord_mod {
            time_slice.get_mut(&staff_name).unwrap()[cursor]
              .chord_modification
              .push(ChordModificationType::Accent);
          }
        }
        musicxml::elements::DirectionTypeContents::Pedal(pedal) => match &pedal.attributes.r#type {
          musicxml::datatypes::PedalType::Start => {
            let item = PhraseModDetails {
              modification: PhraseModificationType::Pedal {
                pedal_type: PedalType::Sustain,
              },
              is_start: true,
              number: pedal.attributes.number.as_ref().map(|number| **number),
              for_voice: None,
              combine_with_next: false,
            };
            time_slice.get_mut(&staff_name).unwrap()[cursor]
              .phrase_modification_start
              .push(item);
          }
          musicxml::datatypes::PedalType::Stop => {
            let item = PhraseModDetails {
              modification: PhraseModificationType::Pedal {
                pedal_type: PedalType::Sustain,
              },
              is_start: false,
              number: pedal.attributes.number.as_ref().map(|number| **number),
              for_voice: None,
              combine_with_next: false,
            };
            time_slice.get_mut(&staff_name).unwrap()[cursor]
              .phrase_modification_end
              .push(item);
          }
          musicxml::datatypes::PedalType::Sostenuto => {
            let item = PhraseModDetails {
              modification: PhraseModificationType::Pedal {
                pedal_type: PedalType::Sostenuto,
              },
              is_start: true,
              number: pedal.attributes.number.as_ref().map(|number| **number),
              for_voice: None,
              combine_with_next: false,
            };
            time_slice.get_mut(&staff_name).unwrap()[cursor]
              .phrase_modification_start
              .push(item);
          }
          musicxml::datatypes::PedalType::Change => {
            let item1 = PhraseModDetails {
              modification: PhraseModificationType::Pedal {
                pedal_type: PedalType::Sustain,
              },
              is_start: false,
              number: pedal.attributes.number.as_ref().map(|number| **number),
              for_voice: None,
              combine_with_next: false,
            };
            let item2 = PhraseModDetails {
              modification: PhraseModificationType::Pedal {
                pedal_type: PedalType::Sustain,
              },
              is_start: true,
              number: pedal.attributes.number.as_ref().map(|number| **number),
              for_voice: None,
              combine_with_next: false,
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
                } * if octave_shift.attributes.r#type == musicxml::datatypes::UpDownStopContinue::Up {
                  -1
                } else {
                  1
                },
              },
              is_start: octave_shift.attributes.r#type != musicxml::datatypes::UpDownStopContinue::Stop,
              number: octave_shift.attributes.number.as_ref().map(|number| **number),
              for_voice: None,
              combine_with_next: false,
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
            time_slice.get_mut(&staff_name).unwrap()[cursor].tempo_change_explicit = Some(tempo);
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
          .map(|item| item.parse::<u8>().unwrap() - 1)
          .collect(),
      );
      for slice in time_slice.values_mut() {
        slice[cursor].ending.push(item.clone());
      }
    }
    if let Some(repeat) = &element.content.repeat {
      let item = (
        repeat.attributes.direction == musicxml::datatypes::BackwardForward::Forward,
        #[allow(clippy::cast_possible_truncation)]
        repeat.attributes.times.as_ref().map_or(1, |item| (**item as u8) - 1),
      );
      for slice in time_slice.values_mut() {
        slice[cursor].repeat.push(item);
      }
    }
    if element.content.coda.is_some() {
      for slice in time_slice.values_mut() {
        slice[cursor].section_start = Some(String::from("Coda"));
      }
    }
    if element.content.segno.is_some() {
      for slice in time_slice.values_mut() {
        slice[cursor].section_start = Some(String::from("Segno"));
      }
    }
    if element.attributes.coda.is_some() {
      for slice in time_slice.values_mut() {
        slice[cursor].jump_to = Some(String::from("Coda"));
      }
    }
    if element.attributes.segno.is_some() {
      for slice in time_slice.values_mut() {
        slice[cursor].jump_to = Some(String::from("Segno"));
      }
    }
    0
  }

  #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
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
    let (divisions, mut tied, acciaccatura, appoggiatura, chord, pitch) = match &note.content.info {
      musicxml::elements::NoteType::Cue(cue) => (
        *cue.duration.content as usize,
        false,
        false,
        false,
        cue.chord.is_some(),
        Pitch::new_rest(),
      ),
      musicxml::elements::NoteType::Grace(grace) => match &grace.info {
        musicxml::elements::GraceType::Cue(cue) => (0, false, false, true, cue.chord.is_some(), Pitch::new_rest()),
        musicxml::elements::GraceType::Normal(normal) => (
          0,
          normal
            .tie
            .iter()
            .any(|tie| tie.attributes.r#type == musicxml::datatypes::StartStop::Start),
          grace.grace.attributes.slash == Some(musicxml::datatypes::YesNo::Yes)
            || grace.grace.attributes.steal_time_previous.is_some(),
          grace.grace.attributes.slash != Some(musicxml::datatypes::YesNo::Yes)
            && grace.grace.attributes.steal_time_previous.is_none(),
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
        false,
        false,
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
        musicxml::datatypes::NoteTypeValue::Quarter => Duration::new(DurationType::Quarter, num_dots),
      }
    } else {
      Self::convert_divisions_to_duration(divisions, divisions_per_quarter_note, num_dots)
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
                combine_with_next: false,
              };
              if item.is_start {
                time_slices.get_mut(&staff_name).unwrap()[cursor]
                  .phrase_modification_start
                  .push(item);
              } else {
                time_slices.get_mut(&staff_name).unwrap()[cursor + divisions]
                  .phrase_modification_end
                  .push(item);
              }
            }
          }
          musicxml::elements::NotationContentTypes::Tuplet(tuplet) => {
            let item = PhraseModDetails {
              modification: tuplet_details.unwrap(),
              is_start: tuplet.attributes.r#type == musicxml::datatypes::StartStop::Start,
              number: tuplet.attributes.number.as_ref().map(|number| **number),
              for_voice: voice.clone(),
              combine_with_next: false,
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
              combine_with_next: false,
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
              combine_with_next: false,
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
                            combine_with_next: false,
                          });
                          None
                        }
                        musicxml::datatypes::TremoloType::Stop => {
                          phrase_modifications_end.push(PhraseModDetails {
                            modification: PhraseModificationType::Tremolo { relative_speed },
                            is_start: false,
                            number: None,
                            for_voice: voice.clone(),
                            combine_with_next: false,
                          });
                          None
                        }
                        musicxml::datatypes::TremoloType::Single => {
                          Some(NoteModificationType::Tremolo { relative_speed })
                        }
                        musicxml::datatypes::TremoloType::Unmeasured => None,
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
              dynamic: Dynamic::Piano(1),
            }),
            musicxml::elements::DynamicsType::Pp(_pp) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: Dynamic::Piano(2),
            }),
            musicxml::elements::DynamicsType::Ppp(_ppp) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: Dynamic::Piano(3),
            }),
            musicxml::elements::DynamicsType::Pppp(_pppp) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: Dynamic::Piano(4),
            }),
            musicxml::elements::DynamicsType::Ppppp(_pppp_it) => {
              note_modifications.push(NoteModificationType::Dynamic {
                dynamic: Dynamic::Piano(5),
              });
            }
            musicxml::elements::DynamicsType::Pppppp(_ppp_it) => {
              note_modifications.push(NoteModificationType::Dynamic {
                dynamic: Dynamic::Piano(6),
              });
            }
            musicxml::elements::DynamicsType::F(_f) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: Dynamic::Forte(1),
            }),
            musicxml::elements::DynamicsType::Ff(_ff) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: Dynamic::Forte(2),
            }),
            musicxml::elements::DynamicsType::Fff(_fff) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: Dynamic::Forte(3),
            }),
            musicxml::elements::DynamicsType::Ffff(_ffff) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: Dynamic::Forte(4),
            }),
            musicxml::elements::DynamicsType::Fffff(_ffff_it) => {
              note_modifications.push(NoteModificationType::Dynamic {
                dynamic: Dynamic::Forte(5),
              });
            }
            musicxml::elements::DynamicsType::Ffffff(_fff_it) => {
              note_modifications.push(NoteModificationType::Dynamic {
                dynamic: Dynamic::Forte(6),
              });
            }
            musicxml::elements::DynamicsType::Mp(_mp) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: Dynamic::MezzoPiano,
            }),
            musicxml::elements::DynamicsType::Mf(_mf) => note_modifications.push(NoteModificationType::Dynamic {
              dynamic: Dynamic::MezzoForte,
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
    if acciaccatura {
      note_modifications.push(NoteModificationType::Grace { acciaccatura: true });
    } else if appoggiatura {
      note_modifications.push(NoteModificationType::Grace { acciaccatura: false });
    }
    let item = NoteDetails {
      pitch,
      duration,
      accidental: accidental.unwrap_or_default(),
      divisions,
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

  fn gather_section_structure_details(time_slices: &[TimeSliceContainer]) -> BTreeMap<usize, SectionDetails> {
    let mut section_details = BTreeMap::new();
    let (mut open_endings, mut open_repeats) = (Vec::new(), Vec::new());
    let (mut open_sections, mut open_tempos) = (Vec::new(), Vec::new());
    for (time_slice_idx, time_slice) in time_slices.iter().enumerate().filter(|(_, slice)| {
      slice.jump_to.is_some()
        || slice.section_start.is_some()
        || !slice.ending.is_empty()
        || !slice.repeat.is_empty()
        || slice.tempo_change_explicit.is_some()
        || slice.tempo_change_implicit.is_some()
    }) {
      // Priority: Ending end, Repeat end, Tempo Change, Section start, Repeat start, Ending start
      // TODO: If multiple sections start at the same time and one ends before the others (out of the order we placed them), then the one that ended needs to become the leaf-most section (i.e., everything in current section should move to that section), and all other sections should shift up by one
      // If a parent section ends before a child section, but the parent and child did not start at the same time, then the child should become the parent, and the parent should be split into multiple sections with same characteristics (how does this work with repeats, etc)
      section_details.insert(time_slice_idx, SectionDetails::default());
      let details = unsafe { section_details.get_mut(&time_slice_idx).unwrap_unchecked() };
      for (ending_start, _) in &time_slice.ending {
        if !*ending_start {
          details.ending_sections.push(open_endings.pop().unwrap_or_default());
        }
      }
      for (repeat_start, _) in &time_slice.repeat {
        if !*repeat_start {
          details.ending_sections.push(open_repeats.pop().unwrap_or_default());
        }
      }
      if let Some(section_name) = &time_slice.section_start {
        for section in open_endings.drain(..) {
          details.ending_sections.push(section);
        }
        for section in open_repeats.drain(..) {
          details.ending_sections.push(section);
        }
        for section in open_tempos.drain(..) {
          details.ending_sections.push(section);
        }
        for section in open_sections.drain(..) {
          details.ending_sections.push(section);
        }
        let new_section_id = details.new_section(section_name).0;
        open_sections.push(new_section_id);
      }
      if let Some(tempo) = &time_slice.tempo_change_explicit {
        for section in open_endings.drain(..) {
          details.ending_sections.push(section);
        }
        for section in open_repeats.drain(..) {
          details.ending_sections.push(section);
        }
        for section in open_tempos.drain(..) {
          details.ending_sections.push(section);
        }
        let (new_section_id, new_section) = details.new_section("Explicit Tempo Section");
        new_section.add_modification(SectionModificationType::TempoExplicit { tempo: *tempo });
        open_tempos.push(new_section_id);
      }
      if let Some(tempo) = &time_slice.tempo_change_implicit {
        for section in open_endings.drain(..) {
          details.ending_sections.push(section);
        }
        for section in open_repeats.drain(..) {
          details.ending_sections.push(section);
        }
        for section in open_tempos.drain(..) {
          details.ending_sections.push(section);
        }
        let (new_section_id, new_section) = details.new_section("Implicit Tempo Section");
        new_section.add_modification(SectionModificationType::TempoImplicit { tempo: *tempo });
        open_tempos.push(new_section_id);
      }
      for (repeat_start, times) in &time_slice.repeat {
        if *repeat_start {
          let (new_section_id, new_section) = details.new_section("Repeated Section");
          new_section.add_modification(SectionModificationType::Repeat { num_times: *times });
          open_repeats.push(new_section_id);
        }
      }
      for (ending_start, iterations) in &time_slice.ending {
        if *ending_start {
          let (new_section_id, new_section) = details.new_section("Ending Section");
          new_section.add_modification(SectionModificationType::OnlyPlay {
            iterations: iterations.clone(),
          });
          open_endings.push(new_section_id);
        }
      }
      // TODO: How handle "Jump To"?? time_slice.jump_to.iter().for_each(|item| println!("[{time_slice_idx}]: JumpTo: {item}"));
    }
    section_details
  }

  fn generate_section_structure(
    top_level_section: &mut Section,
    section_details: &BTreeMap<usize, SectionDetails>,
  ) -> BTreeMap<usize, usize> {
    let mut last_closed_index = 0;
    let mut parent_sections = Vec::new();
    let mut current_section_id = top_level_section.get_id();
    let (mut current_section_repeats, mut current_section_number) = (false, 0);
    let mut section_structure = BTreeMap::from([(0, top_level_section.get_id())]);
    unsafe {
      for (&index, details) in section_details {
        for &ending_section_number in &details.ending_sections {
          if ending_section_number == current_section_number {
            if let Some((parent_section_number, parent_section_id, parent_section_repeats)) = parent_sections.pop() {
              last_closed_index = index;
              current_section_id = parent_section_id;
              current_section_number = parent_section_number;
              current_section_repeats = parent_section_repeats;
              section_structure.insert(index, current_section_id);
            }
          }
        }
        for (&new_section_number, new_section) in &details.starting_sections {
          let is_ending_section = new_section
            .iter_modifications()
            .any(|modification| matches!(modification.r#type, SectionModificationType::OnlyPlay { .. }));
          if last_closed_index != index {
            section_structure.insert(
              last_closed_index,
              top_level_section
                .get_section_mut(current_section_id)
                .unwrap_unchecked()
                .add_section("Implicit Section")
                .get_id(),
            );
            last_closed_index = index;
          }
          if !is_ending_section || current_section_repeats {
            parent_sections.push((current_section_number, current_section_id, current_section_repeats));
            current_section_number = new_section_number;
            current_section_repeats = new_section
              .iter_modifications()
              .any(|modification| matches!(modification.r#type, SectionModificationType::Repeat { .. }));
            current_section_id = top_level_section
              .get_section_mut(current_section_id)
              .unwrap_unchecked()
              .claim_section(new_section.clone())
              .get_id();
            section_structure.insert(index, current_section_id);
          }
        }
      }
    }
    section_structure
  }

  fn ensure_valid_mod_overlaps(time_slices: &mut [TimeSliceContainer], section_structure: &BTreeMap<usize, usize>) {
    // Determine the start and end indices for each modification
    let mut last_valid_timeslice = 0;
    let mut valid_timeslices = BTreeMap::new();
    let mut mod_ranges: Vec<(PhraseModificationType, usize, usize)> = Vec::new();
    let mut open_mods: BTreeMap<PhraseModificationType, (usize, usize)> = BTreeMap::new();
    for (time_slice_idx, time_slice) in time_slices.iter_mut().enumerate() {
      if !time_slice.is_empty() {
        valid_timeslices.insert(time_slice_idx, 0);
        if let Some(last_timeslice) = valid_timeslices.get_mut(&last_valid_timeslice) {
          *last_timeslice = time_slice_idx;
        }
        last_valid_timeslice = time_slice_idx;
      }
      if section_structure.get(&time_slice_idx).is_some() || !time_slice.direction.is_empty() {
        for (_, _, mod_end) in &mut mod_ranges {
          if *mod_end == 0 {
            *mod_end = time_slice_idx;
          }
        }
        open_mods.clear();
      }
      for mod_end in &time_slice.phrase_modification_end {
        if let Some((mod_count, mod_idx)) = open_mods.get_mut(&mod_end.modification) {
          *mod_count -= 1;
          if *mod_count == 0 {
            mod_ranges[*mod_idx].2 = time_slice_idx;
            open_mods.remove(&mod_end.modification);
          }
        }
      }
      for mod_start in &time_slice.phrase_modification_start {
        if let Some((mod_count, _)) = open_mods.get_mut(&mod_start.modification) {
          *mod_count += 1;
        } else {
          open_mods.insert(mod_start.modification, (1, mod_ranges.len()));
          mod_ranges.push((mod_start.modification, time_slice_idx, 0));
        }
      }
      time_slice.phrase_modification_start.clear();
      time_slice.phrase_modification_end.clear();
    }

    // Repeat until no more swaps are needed
    let mut swapped = true;
    while swapped {
      // Ensure that modifications that can fully contain other modifications are ordered first
      while swapped {
        swapped = false;
        for idx in 1..mod_ranges.len() {
          if mod_ranges[idx - 1].1 == mod_ranges[idx].1 && mod_ranges[idx - 1].2 < mod_ranges[idx].2 {
            mod_ranges.swap(idx - 1, idx);
            swapped = true;
          }
        }
      }

      // Handle inappropriately overlapping crescendos and decrescendos
      let mut open_mods = Vec::new();
      for idx in 0..mod_ranges.len() {
        // Close any open modifications that have ended
        let (mod_type, mod_start, mod_end) = mod_ranges[idx];
        while let Some((_, _, _, open_mod_end)) = open_mods.last() {
          if mod_start >= *open_mod_end {
            open_mods.pop();
          } else {
            break;
          }
        }

        // Determine how to handle modifications that only partially overlap
        if open_mods.is_empty() {
          open_mods.push((idx, mod_type, mod_start, mod_end));
        } else if let Some(&(open_mod_idx, open_mod_type, open_mod_start, open_mod_end)) = open_mods.last() {
          if open_mod_end < mod_end {
            let (expand_first_amount, shrink_first_amount) = if matches!(
              open_mod_type,
              PhraseModificationType::Crescendo { .. } | PhraseModificationType::Decrescendo { .. }
            ) {
              (
                mod_end - open_mod_end,
                if idx - 1 == open_mod_idx {
                  mod_start - open_mod_start
                } else {
                  usize::MAX
                },
              )
            } else {
              (usize::MAX, usize::MAX)
            };
            let (expand_second_amount, shrink_second_amount) = if matches!(
              mod_type,
              PhraseModificationType::Crescendo { .. } | PhraseModificationType::Decrescendo { .. }
            ) {
              (
                if idx - 1 == open_mod_idx {
                  mod_start - open_mod_start
                } else {
                  usize::MAX
                },
                mod_end - open_mod_end,
              )
            } else {
              (usize::MAX, usize::MAX)
            };
            let min_disruption_amount = expand_first_amount
              .min(shrink_first_amount)
              .min(expand_second_amount)
              .min(shrink_second_amount);
            if expand_first_amount == min_disruption_amount {
              mod_ranges[open_mod_idx].2 = mod_end;
              open_mods.push((idx, mod_type, mod_start, mod_end));
            } else if expand_second_amount == min_disruption_amount {
              mod_ranges[idx].1 = open_mod_start;
              mod_ranges.swap(open_mod_idx, idx);
              let open_mods_len = open_mods.len();
              open_mods.last_mut().unwrap().0 = idx;
              open_mods.push((open_mod_idx, mod_type, open_mod_start, mod_end));
              open_mods.swap(open_mods_len, open_mods_len - 1);
              swapped = true;
            } else if shrink_first_amount == min_disruption_amount {
              mod_ranges[open_mod_idx].1 = mod_start;
              mod_ranges.swap(open_mod_idx, idx);
              let open_mods_len = open_mods.len();
              open_mods.last_mut().unwrap().0 = idx;
              open_mods.last_mut().unwrap().2 = mod_start;
              open_mods.push((open_mod_idx, mod_type, mod_start, mod_end));
              open_mods.swap(open_mods_len, open_mods_len - 1);
              swapped = true;
            } else if shrink_second_amount == min_disruption_amount {
              mod_ranges[idx].2 = open_mod_end;
              open_mods.push((idx, mod_type, mod_start, mod_end));
            }
          } else {
            open_mods.push((idx, mod_type, mod_start, mod_end));
          }
        }
      }
    }

    // Repeat until no more swaps are needed
    swapped = true;
    while swapped {
      // Ensure that modifications that can fully contain other modifications are ordered first
      while swapped {
        swapped = false;
        for idx in 1..mod_ranges.len() {
          if mod_ranges[idx - 1].1 == mod_ranges[idx].1 && mod_ranges[idx - 1].2 < mod_ranges[idx].2 {
            mod_ranges.swap(idx - 1, idx);
            swapped = true;
          }
        }
      }

      // Handle inappropriately overlapping legatos
      let mut open_mods = Vec::new();
      for idx in 0..mod_ranges.len() {
        // Close any open modifications that have ended
        let (mod_type, mod_start, mod_end) = mod_ranges[idx];
        while let Some((_, _, _, open_mod_end)) = open_mods.last() {
          if mod_start >= *open_mod_end {
            open_mods.pop();
          } else {
            break;
          }
        }

        // Determine how to handle modifications that only partially overlap
        if open_mods.is_empty() {
          open_mods.push((idx, mod_type, mod_start, mod_end));
        } else if let Some(&(open_mod_idx, open_mod_type, open_mod_start, open_mod_end)) = open_mods.last() {
          if open_mod_end < mod_end {
            let (expand_first_amount, shrink_first_amount) = if matches!(open_mod_type, PhraseModificationType::Legato)
            {
              (
                mod_end - open_mod_end,
                if idx - 1 == open_mod_idx {
                  mod_start - open_mod_start
                } else {
                  usize::MAX
                },
              )
            } else {
              (usize::MAX, usize::MAX)
            };
            let (expand_second_amount, shrink_second_amount) = if matches!(mod_type, PhraseModificationType::Legato) {
              (
                if idx - 1 == open_mod_idx {
                  mod_start - open_mod_start
                } else {
                  usize::MAX
                },
                mod_end - open_mod_end,
              )
            } else {
              (usize::MAX, usize::MAX)
            };
            let min_disruption_amount = expand_first_amount
              .min(shrink_first_amount)
              .min(expand_second_amount)
              .min(shrink_second_amount);
            if expand_first_amount == min_disruption_amount {
              mod_ranges[open_mod_idx].2 = mod_end;
              open_mods.push((idx, mod_type, mod_start, mod_end));
            } else if expand_second_amount == min_disruption_amount {
              mod_ranges[idx].1 = open_mod_start;
              mod_ranges.swap(open_mod_idx, idx);
              let open_mods_len = open_mods.len();
              open_mods.last_mut().unwrap().0 = idx;
              open_mods.push((open_mod_idx, mod_type, open_mod_start, mod_end));
              open_mods.swap(open_mods_len, open_mods_len - 1);
              swapped = true;
            } else if shrink_first_amount == min_disruption_amount {
              mod_ranges[open_mod_idx].1 = mod_start;
              mod_ranges.swap(open_mod_idx, idx);
              let open_mods_len = open_mods.len();
              open_mods.last_mut().unwrap().0 = idx;
              open_mods.last_mut().unwrap().2 = mod_start;
              open_mods.push((open_mod_idx, mod_type, mod_start, mod_end));
              open_mods.swap(open_mods_len, open_mods_len - 1);
              swapped = true;
            } else if shrink_second_amount == min_disruption_amount {
              mod_ranges[idx].2 = open_mod_end;
              open_mods.push((idx, mod_type, mod_start, mod_end));
            }
          } else {
            open_mods.push((idx, mod_type, mod_start, mod_end));
          }
        }
      }
    }

    // Remove any meaningless single-note modification phrases
    mod_ranges.retain(|(mod_type, start, end)| {
      !matches!(
        mod_type,
        PhraseModificationType::Legato
          | PhraseModificationType::Crescendo { .. }
          | PhraseModificationType::Decrescendo { .. }
      ) || if let Some(next_idx) = valid_timeslices.get(start) {
        *next_idx != *end
      } else {
        true
      }
    });

    // Update the timeslices using the new modification ranges
    for (idx, &(mod_type, mod_start_idx, mod_end_idx)) in mod_ranges.iter().enumerate() {
      let combine_with_next =
        idx + 1 < mod_ranges.len() && mod_ranges[idx + 1].1 == mod_start_idx && mod_ranges[idx + 1].2 == mod_end_idx;
      time_slices[mod_start_idx]
        .phrase_modification_start
        .push(PhraseModDetails {
          modification: mod_type,
          is_start: true,
          number: None,
          for_voice: None,
          combine_with_next,
        });
      time_slices[mod_end_idx].phrase_modification_end.push(PhraseModDetails {
        modification: mod_type,
        is_start: false,
        number: None,
        for_voice: None,
        combine_with_next: false,
      });
    }
  }

  fn get_musical_elements_by_voice(
    note_details: Vec<NoteDetails>,
    chord_mods: &[ChordModificationType],
  ) -> BTreeMap<String, MusicalItem> {
    // Parse notes and separate them by voice
    let mut note_accidentals = BTreeMap::new();
    let mut voicewide_mods = BTreeMap::new();
    let mut notes_by_voice = BTreeMap::<String, Vec<(Note, NoteDetails)>>::new();
    for item in note_details {
      let voice = String::from(if let Some(voice) = &item.voice { voice } else { "-1" });
      let voice_mods = unsafe {
        if voicewide_mods.contains_key(&voice) {
          voicewide_mods.get_mut(&voice).unwrap_unchecked()
        } else {
          voicewide_mods.insert(voice.clone(), Vec::new());
          voicewide_mods.get_mut(&voice).unwrap_unchecked()
        }
      };
      if item.accidental != Accidental::None {
        note_accidentals.insert(item.pitch, item.accidental);
      };
      let mut note = Note::new(item.pitch, item.duration, Some(item.accidental));
      for modification in &item.note_modifications {
        if let Some(chord_mod) = ChordModification::from_note_modification(modification) {
          voice_mods.push(chord_mod.r#type);
        } else {
          note.add_modification(*modification);
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
        notes_by_voice.insert(voice, Vec::from([(note, item)]));
      }
    }

    // Merge multiple notes into chords and apply voice-specific modifications
    let mut items_by_voice = BTreeMap::new();
    for (voice, notes) in notes_by_voice {
      if notes.len() <= 1 {
        for (mut note, details) in notes {
          let mut musical_item = MusicalItem::default();
          if let Some(accidental) = note_accidentals.get(&note.pitch) {
            note.accidental = *accidental;
          }
          if let Some(mods) = voicewide_mods.get_mut(voice.as_str()) {
            for modification in mods {
              if let Some(note_mod) = NoteModification::from_chord_modification(modification) {
                note.add_modification(note_mod.r#type);
              }
            }
          }
          for modification in chord_mods {
            if let Some(note_mod) = NoteModification::from_chord_modification(modification) {
              note.add_modification(note_mod.r#type);
            }
          }
          for modification in details.phrase_modifications_start {
            if !musical_item
              .new_phrase_modifications
              .iter()
              .any(|item| item.modification == modification.modification)
            {
              musical_item.new_phrase_modifications.push(modification);
            }
          }
          for modification in details.phrase_modifications_end {
            musical_item.ending_phrase_modifications.push(modification);
          }
          musical_item.note = Some(note);
          musical_item.divisions = details.divisions;
          items_by_voice.insert(voice.clone(), musical_item);
        }
      } else {
        let mut chord = Chord::new();
        let mut musical_item = MusicalItem::default();
        if let Some(mods) = voicewide_mods.get_mut(voice.as_str()) {
          for modification in mods {
            chord.add_modification(*modification);
          }
        }
        for modification in chord_mods {
          chord.add_modification(*modification);
        }
        for (mut note, details) in notes {
          if let Some(accidental) = note_accidentals.get(&note.pitch) {
            note.accidental = *accidental;
          }
          for modification in details.phrase_modifications_start {
            if !musical_item
              .new_phrase_modifications
              .iter()
              .any(|item| item.modification == modification.modification)
            {
              musical_item.new_phrase_modifications.push(modification);
            }
          }
          for modification in details.phrase_modifications_end {
            musical_item.ending_phrase_modifications.push(modification);
          }
          chord.claim_note(note);
          if musical_item.divisions == 0 || musical_item.divisions > details.divisions {
            musical_item.divisions = details.divisions;
          }
        }
        musical_item.chord = Some(chord);
        items_by_voice.insert(voice.clone(), musical_item);
      }
    }
    items_by_voice
  }

  fn load_from_musicxml(score: &ScorePartwise) -> Result<Composition, String> {
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
    for name in parts_map.values() {
      composition.add_part(name);
    }

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
        let part_staves = unsafe { part_data.data.get_mut(part_name).unwrap_unchecked() };
        for staff in MusicXmlConverter::find_staves(&part.content) {
          part_staves.insert(staff, vec![TimeSliceContainer::default(); max_divisions]);
        }
      }
    }

    // Parse the actual musical contents of the score into discrete time slices
    let mut part_divisions_per_quarter_note = BTreeMap::new();
    for part in &score.content.part {
      if part.content.is_empty() {
        composition.remove_part_by_name(unsafe { parts_map.get(&*part.attributes.id).unwrap_unchecked() });
      } else {
        let mut open_wedges = BTreeMap::new();
        let (mut cursor, mut previous_cursor): (usize, usize) = (0, 0);
        let divisions_per_quarter_note = MusicXmlConverter::find_divisions_per_quarter_note(&part.content);
        part_divisions_per_quarter_note.insert(
          unsafe { parts_map.get(&*part.attributes.id).unwrap_unchecked() },
          divisions_per_quarter_note,
        );
        let time_slices = unsafe {
          part_data
            .data
            .get_mut(parts_map.get(&*part.attributes.id).unwrap_unchecked())
            .unwrap_unchecked()
        };
        for element in &part.content {
          if let musicxml::elements::PartElement::Measure(measure) = element {
            for measure_element in &measure.content {
              let cursor_change = match measure_element {
                musicxml::elements::MeasureElement::Attributes(attributes) => {
                  MusicXmlConverter::parse_attributes_element(&attributes.content, time_slices, cursor)
                }
                musicxml::elements::MeasureElement::Note(note) => MusicXmlConverter::parse_note_element(
                  note,
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
                  MusicXmlConverter::parse_direction_element(direction, time_slices, &mut open_wedges, cursor)
                }
                musicxml::elements::MeasureElement::Barline(barline) => {
                  MusicXmlConverter::parse_barline_element(barline, time_slices, cursor)
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

    // Use the temporally ordered time slices from the first available part and staff to parse section structure details
    let section_details = if let Some(Some(time_slices)) = part_data
      .data
      .first_key_value()
      .map(|(_, staves)| staves.values().next())
    {
      Self::gather_section_structure_details(time_slices)
    } else {
      BTreeMap::new()
    };

    // Use the temporally ordered time slices for each part to construct a final composition structure
    for (part_name, staves) in part_data.data {
      let part = composition
        .get_part_mut_by_name(&part_name)
        .expect("Unknown part name encountered");
      let divisions_per_quarter_note = *part_divisions_per_quarter_note.get(&part_name).unwrap();

      // Handle creation of the section structure for this part
      let master_section = part.add_section("Top-Level Section");
      let section_structure = Self::generate_section_structure(master_section, &section_details);

      // Iterate through all staves in the part
      for (staff_name, mut time_slices) in staves {
        let mut current_section_idx = 0;
        let mut pending_directions = Vec::new();
        let mut pending_staff_phrase_starts = Vec::new();
        let mut pending_staff_phrase_ends = Vec::new();
        let mut staff_phrases = Vec::new();
        let mut voice_phrases: BTreeMap<PhraseModificationType, Vec<usize>> = BTreeMap::new();
        let mut multivoices: BTreeMap<String, (usize, [usize; 1], Vec<usize>)> = BTreeMap::new();
        Self::ensure_valid_mod_overlaps(&mut time_slices, &section_structure);

        // Ensure that all time slices have a duration that matches their annotated divisions
        let mut last_valid_idx = usize::MAX;
        for idx in 0..time_slices.len() {
          if !time_slices[idx].is_empty() {
            if last_valid_idx != usize::MAX
              && idx - last_valid_idx
                != time_slices[last_valid_idx]
                  .notes
                  .iter()
                  .map(|item| item.divisions)
                  .min()
                  .unwrap_or(usize::MAX)
            {
              if let Some(details) = time_slices[last_valid_idx].notes.first_mut() {
                details.divisions = idx - last_valid_idx;
                details.duration =
                  Self::convert_divisions_to_duration(details.divisions, divisions_per_quarter_note, 0);
              }
            }
            last_valid_idx = idx;
          }
        }

        for (time_slice_idx, mut time_slice) in time_slices.into_iter().enumerate() {
          // Handle section delineations
          if let Some(&new_section_idx) = section_structure.get(&time_slice_idx) {
            // New sections require that all existing phrases be closed
            multivoices.clear();
            staff_phrases.clear();
            voice_phrases.clear();
            pending_directions.clear();
            pending_staff_phrase_starts.clear();
            pending_staff_phrase_ends.clear();

            // Move to the new section
            unsafe {
              current_section_idx = new_section_idx;
              staff_phrases.push((
                Vec::from([master_section
                  .get_section_mut(current_section_idx)
                  .unwrap_unchecked()
                  .add_staff(&staff_name)
                  .add_phrase()
                  .get_id()]),
                Vec::new(),
              ));
            }
          }

          // Parse notes and chords and separate them by voice
          let mut items_by_voice =
            Self::get_musical_elements_by_voice(time_slice.notes, &time_slice.chord_modification);

          // Handle closing multivoices
          let latest_expected_timestamp_idx = multivoices
            .iter()
            .map(|(_, (_, idx, _))| idx[0])
            .max()
            .unwrap_or(time_slice_idx);
          if latest_expected_timestamp_idx != time_slice_idx {
            for (voice_id, (_, expected_next_timestamp_idx, _)) in &mut multivoices {
              if expected_next_timestamp_idx[0] == time_slice_idx && !items_by_voice.contains_key(voice_id) {
                let mut rest = MusicalItem::default();
                let duration = Self::convert_divisions_to_duration(
                  latest_expected_timestamp_idx - time_slice_idx,
                  divisions_per_quarter_note,
                  0,
                );
                rest.note = Some(Note::new(Pitch::new_rest(), duration, None));
                items_by_voice.insert(voice_id.clone(), rest);
                expected_next_timestamp_idx[0] = latest_expected_timestamp_idx;
              }
            }
          }
          multivoices.retain(|voice_id, (_, expected_next_timestamp_idx, _)| {
            expected_next_timestamp_idx[0] > time_slice_idx
              || (expected_next_timestamp_idx[0] == time_slice_idx && items_by_voice.contains_key(voice_id))
          });
          if !items_by_voice.keys().all(|voice| multivoices.contains_key(voice)) {
            multivoices.clear();
            voice_phrases.clear();
          }

          // Handle staff-wide phrase modification endings
          if voice_phrases.is_empty() {
            time_slice
              .phrase_modification_end
              .append(&mut pending_staff_phrase_ends);
            for item in time_slice.phrase_modification_end {
              let phrases_len = staff_phrases.len();
              if let Some((idx, (existing_mod_ids, _))) = staff_phrases
                .iter_mut()
                .enumerate()
                .find(|(_, (_, mod_types))| mod_types.contains(&item.modification))
              {
                existing_mod_ids.pop();
                if existing_mod_ids.is_empty() {
                  multivoices.clear();
                  let mut phrases_to_read = Vec::new();
                  for _ in (idx + 1)..phrases_len {
                    phrases_to_read.push(staff_phrases.pop().unwrap());
                  }
                  staff_phrases.pop();
                  while let Some(phrase) = phrases_to_read.pop() {
                    let new_phrase = master_section
                      .get_phrase_mut(*staff_phrases.last().unwrap().0.first().unwrap())
                      .unwrap()
                      .add_phrase();
                    for modification in &phrase.1 {
                      new_phrase.add_modification(*modification);
                    }
                    staff_phrases.push((Vec::from([new_phrase.get_id()]), phrase.1));
                  }
                }
              }
            }
          } else {
            pending_staff_phrase_ends.append(&mut time_slice.phrase_modification_end);
          }

          // Handle staff-wide directions
          if voice_phrases.is_empty() {
            time_slice.direction.append(&mut pending_directions);
            if !time_slice.direction.is_empty() {
              multivoices.clear();
              staff_phrases.clear();
              unsafe {
                let staff = master_section
                  .get_section_mut(current_section_idx)
                  .unwrap_unchecked()
                  .get_staff_mut_by_name(&staff_name)
                  .unwrap_unchecked();
                for direction in time_slice.direction {
                  staff.add_direction(direction);
                }
                staff_phrases.push((Vec::from([staff.add_phrase().get_id()]), Vec::new()));
              }
            }
          } else {
            pending_directions.append(&mut time_slice.direction);
          }

          // Handle new staff-wide phrase modifications
          let mut combined_indices = 0;
          time_slice
            .phrase_modification_start
            .append(&mut pending_staff_phrase_starts);
          for (mut idx, item) in time_slice.phrase_modification_start.iter().enumerate() {
            if combined_indices > 0 {
              combined_indices -= 1;
              continue;
            }
            if let Some((existing_mod_ids, _)) = staff_phrases
              .iter_mut()
              .find(|(_, mod_types)| mod_types.contains(&item.modification))
            {
              existing_mod_ids.push(0);
            } else if voice_phrases.is_empty() {
              while idx < time_slice.phrase_modification_start.len()
                && time_slice.phrase_modification_start[idx].combine_with_next
              {
                combined_indices += 1;
                idx += 1;
              }
              multivoices.clear();
              let new_phrase = master_section
                .get_phrase_mut(*staff_phrases.last().unwrap().0.first().unwrap())
                .unwrap()
                .add_phrase();
              let mut phrase_mod_types = Vec::new();
              for combined_idx in 0..=combined_indices {
                let mod_type = time_slice.phrase_modification_start[idx - combined_indices + combined_idx].modification;
                new_phrase.add_modification(mod_type);
                phrase_mod_types.push(mod_type);
              }
              staff_phrases.push((Vec::from([new_phrase.get_id()]), phrase_mod_types));
            } else {
              pending_staff_phrase_starts.push(item.clone());
            }
          }

          // Handle voice-wide musical contents
          for (voice, voice_items) in items_by_voice {
            // Retrieve or create a new phrase for the current voice
            let mut phrase = if let Some((_, expected_timestamp, phrase_ids)) = multivoices.get_mut(&voice) {
              expected_timestamp[0] = time_slice_idx + voice_items.divisions;
              master_section.get_phrase_mut(*phrase_ids.last().unwrap()).unwrap()
            } else if let Some((_, &(multivoice_id, _, _))) = multivoices.first_key_value() {
              let new_phrase = master_section.get_multivoice_mut(multivoice_id).unwrap().add_phrase();
              multivoices.insert(
                voice.clone(),
                (
                  multivoice_id,
                  [time_slice_idx + voice_items.divisions],
                  Vec::from([new_phrase.get_id()]),
                ),
              );
              new_phrase
            } else {
              let new_multivoice = master_section
                .get_phrase_mut(*staff_phrases.last().unwrap().0.first().unwrap())
                .unwrap()
                .add_multivoice();
              let multivoice_id = new_multivoice.get_id();
              let new_phrase = new_multivoice.add_phrase();
              multivoices.insert(
                voice.clone(),
                (
                  multivoice_id,
                  [time_slice_idx + voice_items.divisions],
                  Vec::from([new_phrase.get_id()]),
                ),
              );
              new_phrase
            };

            // Handle adding new phrase modifications
            if voice_items.ending_phrase_modifications.is_empty() {
              for new_mod in voice_items.new_phrase_modifications {
                phrase = phrase.add_phrase();
                phrase.add_modification(new_mod.modification);
                multivoices.get_mut(&voice).unwrap().2.push(phrase.get_id());
                if let Some(phrases) = voice_phrases.get_mut(&new_mod.modification) {
                  phrases.push(phrase.get_id());
                } else {
                  voice_phrases.insert(new_mod.modification, Vec::from([phrase.get_id()]));
                }
              }
            } else {
              // TODO: What if not empty? Convert to note_mod
            }

            // Add the current musical item to the phrase
            if let Some(note) = voice_items.note {
              phrase.claim_note(note);
            } else if let Some(chord) = voice_items.chord {
              phrase.claim_chord(chord);
            }

            // Handle removing ending phrase modifications
            for ending_mod in voice_items.ending_phrase_modifications {
              if let Some(phrases) = voice_phrases.get_mut(&ending_mod.modification) {
                if !phrases.is_empty() {
                  multivoices.get_mut(&voice).unwrap().2.pop();
                  phrases.pop();
                  if phrases.is_empty() {
                    voice_phrases.remove(&ending_mod.modification);
                  }
                }
              }
            }
          }
        }
      }

      // Simplify the part to remove any unnecessary nesting structures
      part.simplify();
    }

    Ok(composition)
  }
}

impl Load for MusicXmlConverter {
  fn load(path: &str) -> Result<Composition, String> {
    let score = musicxml::read_score_partwise(path)?;
    MusicXmlConverter::load_from_musicxml(&score)
  }

  fn load_data(data: &[u8]) -> Result<Composition, String> {
    let data = str::from_utf8(data).map_err(|err| err.to_string())?;
    let score = musicxml::parser::parse_from_xml_str(data).map_err(|err| err.to_string())?;
    MusicXmlConverter::load_from_musicxml(&score)
    // TODO: "Update MusicXML parser library to parse from raw data so we can do the partwise conversion if necessary"
  }
}
