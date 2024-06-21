use super::Convert;
use crate::{
  Accidental, Clef, Composition, DisplayOptions, Duration, Key, KeyMode, NotationalItem, Note, NoteModification, Pitch, Stem, Tempo, TimeSignature, Voice,
};
use musicxml;
use std::collections::HashMap;

pub struct MusicXmlConverter;

impl Convert for MusicXmlConverter {
  fn load(path: &str) -> Result<Composition, String> {
    // Parse the MusicXML score representation
    let score = musicxml::read_score_partwise(path)?;

    // Generate the initial composition structure and fill in known attributes
    let mut beat_divisions: u32 = 4;
    let mut system_map: HashMap<String, String> = HashMap::new();
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
    if let Some(work) = &score.content.work {
      if let Some(work_number) = &work.content.work_number {
        composition.add_metadata("opus_number", work_number.content.as_str());
      }
    }
    if let Some(movement_number) = &score.content.movement_number {
      composition.add_metadata("movement_number", movement_number.content.as_str());
    }
    if let Some(movement_title) = &score.content.movement_title {
      composition.add_metadata("movement_title", movement_title.content.as_str());
    }
    if let Some(identification) = &score.content.identification {
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
    for part in &score.content.part_list.content.content {
      match part {
        musicxml::elements::PartListElement::ScorePart(score_part) => {
          system_map.insert(
            (*score_part.attributes.id).clone(),
            score_part.content.part_name.content.clone(),
          );
          composition.add_system(score_part.content.part_name.content.as_str(), None);
        }
        _ => (),
      }
    }
    if score.content.part.is_empty() {
      return Err(String::from("No parts found in the MusicXML score"));
    } else if score.content.part[0].content.is_empty() {
      return Err(String::from(
        "No measures found in the first part of the MusicXML score",
      ));
    }
    let mut tempo_found = false;
    for part in &score.content.part {
      let system_name = system_map
        .get(&*part.attributes.id)
        .expect("Unknown Part ID encountered");
      match &part.content[0] {
        musicxml::elements::PartElement::Measure(measure) => {
          for measure_element in &measure.content {
            match measure_element {
              musicxml::elements::MeasureElement::Attributes(attributes) => {
                for key_element in &attributes.content.key {
                  match &key_element.content {
                    musicxml::elements::KeyContents::Explicit(key) => {
                      let mode = match &key.mode {
                        Some(mode) => match mode.content {
                          musicxml::datatypes::Mode::Minor => KeyMode::Minor,
                          _ => KeyMode::Major,
                        },
                        None => KeyMode::Major,
                      };
                      composition.set_starting_key(Key::from_fifths(*key.fifths.content, Some(mode)));
                    }
                    _ => (),
                  };
                }
                if let Some(divisions_elements) = &attributes.content.divisions {
                  beat_divisions = *divisions_elements.content * 4;
                };
                for time_element in &attributes.content.time {
                  for beat_element in &time_element.content.beats {
                    composition.set_starting_time_signature(TimeSignature::new(
                      (*beat_element.beats.content).parse().unwrap(),
                      (*beat_element.beat_type.content).parse().unwrap(),
                    ));
                  }
                }
                if let Some(staff) = &attributes.content.staves {
                  for i in 1..=*staff.content {
                    composition.add_staff_to_system(system_name, i.to_string().as_str(), None);
                  }
                } else {
                  composition.add_staff_to_system(system_name, "1", None);
                };
                for clef in &attributes.content.clef {
                  let clef_type = match clef.content.sign.content {
                    musicxml::datatypes::ClefSign::G => Clef::Treble,
                    musicxml::datatypes::ClefSign::F => Clef::Bass,
                    musicxml::datatypes::ClefSign::C => Clef::Alto,
                    _ => Clef::Treble,
                  };
                  let staff_number = if let Some(number) = &clef.attributes.number {
                    **number
                  } else {
                    1
                  };
                  composition.add_staff_to_system(system_name, staff_number.to_string().as_str(), Some(clef_type));
                }
              }
              musicxml::elements::MeasureElement::Direction(direction) => {
                for direction_type in &direction.content.direction_type {
                  match &direction_type.content {
                    musicxml::elements::DirectionTypeContents::Metronome(metronome) => {
                      match &metronome.content {
                        musicxml::elements::MetronomeContents::BeatBased(beat_data) => {
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
                            musicxml::datatypes::NoteTypeValue::OneHundredTwentyEighth => {
                              Duration::OneHundredTwentyEighth(num_dots)
                            }
                            musicxml::datatypes::NoteTypeValue::TwoHundredFiftySixth => {
                              Duration::TwoHundredFiftySixth(num_dots)
                            }
                            musicxml::datatypes::NoteTypeValue::FiveHundredTwelfth => {
                              Duration::FiveHundredTwelfth(num_dots)
                            }
                            musicxml::datatypes::NoteTypeValue::OneThousandTwentyFourth => {
                              Duration::OneThousandTwentyFourth(num_dots)
                            }
                            _ => Duration::Quarter(num_dots),
                          };
                          match &beat_data.equals {
                            musicxml::elements::BeatEquation::BPM(per_minute) => {
                              composition.set_tempo(Tempo {
                                base_note,
                                beats_per_minute: per_minute.content.parse().unwrap(),
                              });
                              tempo_found = true;
                            }
                            _ => (),
                          };
                        }
                        _ => (),
                      };
                    }
                    _ => (),
                  };
                }
                if !tempo_found {
                  if let Some(sound) = &direction.content.sound {
                    if let Some(tempo) = &sound.attributes.tempo {
                      let bpm = **tempo as u16;
                      if bpm > 0 {
                        composition.set_tempo(Tempo {
                          base_note: Duration::Quarter(0),
                          beats_per_minute: bpm,
                        });
                      }
                    }
                  }
                }
              }
              _ => (),
            }
          }
        }
        _ => {
          return Err(String::from(
            "The only part elements in a MusicXML score should be measures",
          ))
        }
      }
    }
    if !tempo_found {
      composition.set_tempo(Tempo {
        base_note: Duration::Quarter(0),
        beats_per_minute: 120,
      });
    }

    // Parse the actual musical contents of the score
    for part in &score.content.part {
      let system_name = system_map
        .get(&*part.attributes.id)
        .expect("Unknown Part ID encountered");
      let mut system = composition.get_system(system_name).expect("Unknown system encountered");
      for element in &part.content {
        match element {
          musicxml::elements::PartElement::Measure(measure) => {
            for measure_element in &measure.content {
              match measure_element {
                musicxml::elements::MeasureElement::Note(note) => {
                  let staff_name = if let Some(staff) = &note.content.staff {
                    staff.content.to_string()
                  } else {
                    String::from("1")
                  };
                  let num_dots = note.content.dot.len() as u8;
                  let (mut duration, mut tied, chord, pitch) = match &note.content.info {
                    musicxml::elements::NoteType::Cue(_) => continue,
                    musicxml::elements::NoteType::Grace(_) => continue,
                    musicxml::elements::NoteType::Normal(normal) => (
                      match beat_divisions / *normal.duration.content {
                        1 => Duration::Whole(num_dots),
                        2 => Duration::Half(num_dots),
                        4 => Duration::Quarter(num_dots),
                        8 => Duration::Eighth(num_dots),
                        16 => Duration::Sixteenth(num_dots),
                        32 => Duration::ThirtySecond(num_dots),
                        64 => Duration::SixtyFourth(num_dots),
                        128 => Duration::OneHundredTwentyEighth(num_dots),
                        256 => Duration::TwoHundredFiftySixth(num_dots),
                        512 => Duration::FiveHundredTwelfth(num_dots),
                        1024 => Duration::OneThousandTwentyFourth(num_dots),
                        2048 => Duration::TwoThousandFortyEighth(num_dots),
                        _ => Duration::Breve(num_dots),
                      },
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
                        musicxml::elements::AudibleType::Unpitched(_) => continue,
                        musicxml::elements::AudibleType::Rest(_) => Pitch::Rest,
                      },
                    ),
                  };
                  let beamed = note
                    .content
                    .beam
                    .iter()
                    .any(|beam| beam.content != musicxml::datatypes::BeamValue::End);
                  let stem = note
                    .content
                    .stem
                    .as_ref()
                    .map(|value| match value.content {
                      musicxml::datatypes::StemValue::Up => Stem::Up,
                      musicxml::datatypes::StemValue::Down => Stem::Down,
                      _ => Stem::Default,
                    })
                    .unwrap_or(Stem::Default);
                  let voice = Voice::default(); // TODO: Parse from MusicXML string <voice>6</voice>
                  if let Some(note_type) = &note.content.r#type {
                    duration = match &note_type.content {
                      musicxml::datatypes::NoteTypeValue::Maxima => Duration::Maxima(num_dots),
                      musicxml::datatypes::NoteTypeValue::Long => Duration::Long(num_dots),
                      musicxml::datatypes::NoteTypeValue::Breve => Duration::Breve(num_dots),
                      musicxml::datatypes::NoteTypeValue::Whole => Duration::Whole(num_dots),
                      musicxml::datatypes::NoteTypeValue::Half => Duration::Half(num_dots),
                      musicxml::datatypes::NoteTypeValue::Eighth => Duration::Eighth(num_dots),
                      musicxml::datatypes::NoteTypeValue::Sixteenth => Duration::Sixteenth(num_dots),
                      musicxml::datatypes::NoteTypeValue::ThirtySecond => Duration::ThirtySecond(num_dots),
                      musicxml::datatypes::NoteTypeValue::SixtyFourth => Duration::SixtyFourth(num_dots),
                      musicxml::datatypes::NoteTypeValue::OneHundredTwentyEighth => {
                        Duration::OneHundredTwentyEighth(num_dots)
                      }
                      musicxml::datatypes::NoteTypeValue::TwoHundredFiftySixth => {
                        Duration::TwoHundredFiftySixth(num_dots)
                      }
                      musicxml::datatypes::NoteTypeValue::FiveHundredTwelfth => Duration::FiveHundredTwelfth(num_dots),
                      musicxml::datatypes::NoteTypeValue::OneThousandTwentyFourth => {
                        Duration::OneThousandTwentyFourth(num_dots)
                      }
                      _ => Duration::Quarter(num_dots),
                    };
                  };
                  let accidental = note
                    .content
                    .accidental
                    .as_ref()
                    .map(|accidental| match accidental.content {
                      musicxml::datatypes::AccidentalValue::Sharp
                      | musicxml::datatypes::AccidentalValue::NaturalSharp => Accidental::Sharp,
                      musicxml::datatypes::AccidentalValue::Flat
                      | musicxml::datatypes::AccidentalValue::NaturalFlat => Accidental::Flat,
                      musicxml::datatypes::AccidentalValue::Natural => Accidental::Natural,
                      musicxml::datatypes::AccidentalValue::DoubleSharp
                      | musicxml::datatypes::AccidentalValue::SharpSharp => Accidental::DoubleSharp,
                      musicxml::datatypes::AccidentalValue::FlatFlat => Accidental::DoubleFlat,
                      _ => Accidental::None,
                    });
                  let tuplet = note.content.time_modification.as_ref().map(|time_modification| {
                    let actual_notes = *time_modification.content.actual_notes.content;
                    let normal_notes = *time_modification.content.normal_notes.content;
                    (actual_notes, normal_notes)
                  });
                  system.get_staff(&staff_name).expect("Unknown staff encountered"); // TODO: Something with this staff
                  let mut new_note = Note::new(
                    pitch,
                    duration,
                    accidental,
                    Some(DisplayOptions { voice, stem, beamed }),
                  );
                  note.content.notations.iter().for_each(|notation| {
                    notation
                      .content
                      .notations
                      .iter()
                      .for_each(|notation_type| match notation_type {
                        musicxml::elements::NotationContentTypes::Tied(tie) => {
                          if tie.attributes.r#type == musicxml::datatypes::StartStopContinue::Start { tied = true; }
                        }
                        musicxml::elements::NotationContentTypes::Slur(slur) => {} // TODO: Something 
                        musicxml::elements::NotationContentTypes::Tuplet(tuplet) => {} // TODO: Something (can we ignore in favor of time-modification above?)
                        musicxml::elements::NotationContentTypes::Glissando(glissando) => {} // TODO: Something 
                        musicxml::elements::NotationContentTypes::Slide(slide) => {} // TODO: Something 
                        musicxml::elements::NotationContentTypes::Ornaments(ornaments) => {
                          let note_modifications = ornaments.content.ornaments.iter().filter_map(|ornament| {
                            match ornament {
                              musicxml::elements::OrnamentType::TrillMark(_trill_mark) => Some(NoteModification::Trill { upper: true }),
                              musicxml::elements::OrnamentType::Turn(_turn) => Some(NoteModification::Turn { upper: true, delayed: false, vertical: false }),
                              musicxml::elements::OrnamentType::DelayedTurn(_delayed_turn) => Some(NoteModification::Turn { upper: true, delayed: true, vertical: false }),
                              musicxml::elements::OrnamentType::InvertedTurn(_inverted_turn) => Some(NoteModification::Turn { upper: false, delayed: false, vertical: false }),
                              musicxml::elements::OrnamentType::DelayedInvertedTurn(_delayed_inverted_turn) => Some(NoteModification::Turn { upper: false, delayed: true, vertical: false }),
                              musicxml::elements::OrnamentType::VerticalTurn(_vertical_turn) => Some(NoteModification::Turn { upper: true, delayed: false, vertical: true }),
                              musicxml::elements::OrnamentType::InvertedVerticalTurn(_inverted_vertical_turn) => Some(NoteModification::Turn { upper: false, delayed: false, vertical: true }),
                              musicxml::elements::OrnamentType::Shake(_shake) => Some(NoteModification::Shake),
                              musicxml::elements::OrnamentType::WavyLine(_wavy_line) => Some(NoteModification::Trill { upper: true }),
                              musicxml::elements::OrnamentType::Mordent(_mordent) => Some(NoteModification::Mordent { upper: true }),
                              musicxml::elements::OrnamentType::InvertedMordent(_inverted_mordent) => Some(NoteModification::Mordent { upper: false }),
                              musicxml::elements::OrnamentType::Schleifer(_schleifer) => Some(NoteModification::Schleifer),
                              musicxml::elements::OrnamentType::Tremolo(tremolo) => {
                                if let Some(tremolo_type) = &tremolo.attributes.r#type {
                                  let relative_speed = *tremolo.content;
                                  match tremolo_type {
                                    musicxml::datatypes::TremoloType::Start => Some(NoteModification::DoubleNoteTremolo { second_note: 0, relative_speed }), // TODO: Parse second note in future somehow
                                    musicxml::datatypes::TremoloType::Single => Some(NoteModification::SingleNoteTremolo { relative_speed }),
                                    _ => None,
                                  }
                                } else {
                                  None
                                }
                              },
                              musicxml::elements::OrnamentType::Haydn(_haydn) => Some(NoteModification::Haydn),
                              musicxml::elements::OrnamentType::OtherOrnament(_) => None,
                            }
                          }).collect::<Vec<_>>();
                          for note_modification in note_modifications {
                            new_note.add_note_modification(note_modification);
                          }
                        }
                        musicxml::elements::NotationContentTypes::Technical(technicals) => {
                          let note_modifications = technicals.content.iter().filter_map(|technical| {
                            match technical {
                              musicxml::elements::TechnicalContents::UpBow(_up_bow) => Some(NoteModification::UpBow),
                              musicxml::elements::TechnicalContents::DownBow(_down_bow) => Some(NoteModification::DownBow),
                              musicxml::elements::TechnicalContents::Harmonic(_harmonic) => None,
                              musicxml::elements::TechnicalContents::OpenString(_open_string) => None,
                              musicxml::elements::TechnicalContents::ThumbPosition(_thumb_position) => None,
                              musicxml::elements::TechnicalContents::Fingering(_fingering) => None,
                              musicxml::elements::TechnicalContents::Pluck(_pluck) => None,
                              musicxml::elements::TechnicalContents::DoubleTongue(_double_tongue) => None,
                              musicxml::elements::TechnicalContents::TripleTongue(_triple_tongue) => None,
                              musicxml::elements::TechnicalContents::Stopped(_stopped) => None,
                              musicxml::elements::TechnicalContents::SnapPizzicato(_snap_pizzicato) => Some(NoteModification::Pizzicato),
                              musicxml::elements::TechnicalContents::Fret(_fret) => None,
                              musicxml::elements::TechnicalContents::StringNumber(_string) => None,
                              musicxml::elements::TechnicalContents::HammerOn(_hammer_on) => None,
                              musicxml::elements::TechnicalContents::PullOff(_pull_off) => None,
                              musicxml::elements::TechnicalContents::Bend(_bend) => None,
                              musicxml::elements::TechnicalContents::Tap(_tap) => None,
                              musicxml::elements::TechnicalContents::Heel(_heel) => Some(NoteModification::Heel),
                              musicxml::elements::TechnicalContents::Toe(_toe) => Some(NoteModification::Toe),
                              musicxml::elements::TechnicalContents::Fingernails(_fingernails) => Some(NoteModification::Fingernails),
                              musicxml::elements::TechnicalContents::Hole(_hole) => None,
                              musicxml::elements::TechnicalContents::Arrow(_arrow) => None,
                              musicxml::elements::TechnicalContents::Handbell(_handbell) => None,
                              musicxml::elements::TechnicalContents::BrassBend(_brass_bend) => Some(NoteModification::BrassBend),
                              musicxml::elements::TechnicalContents::Flip(_flip) => Some(NoteModification::Flip),
                              musicxml::elements::TechnicalContents::Smear(_smear) => Some(NoteModification::Smear),
                              musicxml::elements::TechnicalContents::Open(_open) => Some(NoteModification::Open),
                              musicxml::elements::TechnicalContents::HalfMuted(_half_muted) => Some(NoteModification::HalfMuted),
                              musicxml::elements::TechnicalContents::HarmonMute(harmon_mute) => match harmon_mute.content.harmon_closed.content {
                                musicxml::datatypes::HarmonClosedValue::No => Some(NoteModification::HarmonMute { open: true, half: false }),
                                musicxml::datatypes::HarmonClosedValue::Half => Some(NoteModification::HarmonMute { open: false, half: true }),
                                musicxml::datatypes::HarmonClosedValue::Yes => Some(NoteModification::HarmonMute { open: false, half: false }),
                              },
                              musicxml::elements::TechnicalContents::Golpe(_golpe) => Some(NoteModification::Golpe),
                              musicxml::elements::TechnicalContents::OtherTechnical(_) => None,
                            }
                          }).collect::<Vec<_>>();
                          for note_modification in note_modifications {
                            new_note.add_note_modification(note_modification);
                          }
                        }
                        musicxml::elements::NotationContentTypes::Articulations(articulations) => {
                          let mut notational_items: Vec<NotationalItem> = Vec::new();
                          let note_modifications = articulations.content.iter().filter_map(|articulation| {
                            match articulation {
                              musicxml::elements::ArticulationsType::Accent(_accent) => Some(NoteModification::Accent),
                              musicxml::elements::ArticulationsType::StrongAccent(_strong_accent) => Some(NoteModification::Marcato),
                              musicxml::elements::ArticulationsType::Staccato(_staccato) => Some(NoteModification::Staccato),
                              musicxml::elements::ArticulationsType::Tenuto(_tenuto) => Some(NoteModification::Tenuto),
                              musicxml::elements::ArticulationsType::DetachedLegato(_detached_legato) => Some(NoteModification::DetachedLegato),
                              musicxml::elements::ArticulationsType::Staccatissimo(_staccatissimo) => Some(NoteModification::Staccatissimo),
                              musicxml::elements::ArticulationsType::Spiccato(_spiccato) => Some(NoteModification::Spiccato),
                              musicxml::elements::ArticulationsType::Scoop(_scoop) => Some(NoteModification::Scoop),
                              musicxml::elements::ArticulationsType::Plop(_plop) => Some(NoteModification::Plop),
                              musicxml::elements::ArticulationsType::Doit(_doit) => Some(NoteModification::Doit),
                              musicxml::elements::ArticulationsType::Falloff(_falloff) => Some(NoteModification::Falloff),
                              musicxml::elements::ArticulationsType::BreathMark(_breath_mark) => {
                                notational_items.push(NotationalItem::BreathMark);
                                None
                              },
                              musicxml::elements::ArticulationsType::Caesura(_caesura) => {
                                notational_items.push(NotationalItem::Caesura);
                                None
                              },
                              musicxml::elements::ArticulationsType::Stress(_stress) => Some(NoteModification::Stress),
                              musicxml::elements::ArticulationsType::Unstress(_unstress) => Some(NoteModification::Unstress),
                              musicxml::elements::ArticulationsType::SoftAccent(_soft_accent) => Some(NoteModification::SoftAccent),
                              musicxml::elements::ArticulationsType::OtherArticulation(_) => None,
                            }
                          }).collect::<Vec<_>>();
                          for note_modification in note_modifications {
                            new_note.add_note_modification(note_modification);
                          }
                          // TODO: Something with notational_items
                        }
                        musicxml::elements::NotationContentTypes::Dynamics(dynamics) => {} // TODO: Something 
                        musicxml::elements::NotationContentTypes::Fermata(_fermata) => { new_note.add_note_modification(NoteModification::Fermata { relative_duration: 2 }); },
                        musicxml::elements::NotationContentTypes::Arpeggiate(arpeggiate) => {} // TODO: Something 
                        musicxml::elements::NotationContentTypes::NonArpeggiate(non_arpeggiate) => {} // TODO: Something 
                        musicxml::elements::NotationContentTypes::AccidentalMark(accidental_mark) => {} // TODO: Something 
                        musicxml::elements::NotationContentTypes::OtherNotation(other_notation) => {} // TODO: Something 
                      });
                  });
                  // TODO: Deal with tied, chord, tuplet
                }
                musicxml::elements::MeasureElement::Backup(backup) => {} // TODO: Something 
                musicxml::elements::MeasureElement::Forward(forward) => {} // TODO: Something 
                musicxml::elements::MeasureElement::Direction(direction) => {} // TODO: Something 
                musicxml::elements::MeasureElement::Attributes(attributes) => {} // TODO: Something 
                musicxml::elements::MeasureElement::Barline(barline) => {} // TODO: Something 
                _ => (),
              }
            }
          }
          _ => (),
        }
      }
    }

    // Return the composition
    Ok(composition)
  }

  fn save(path: &str, composition: &Composition) -> Result<usize, String> {
    //fs::write(path, contents)?;
    Ok(0)
  }
}
