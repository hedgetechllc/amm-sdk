use super::Convert;
use crate::Composition;
use alloc::string::String;
use amm_internal::{JsonDeserializer, JsonSerializer};
use core::str;
use std::fs;

pub struct JsonConverter;

impl JsonConverter {
  fn load_from_json(data: &[u8]) -> Result<Composition, String> {
    let json = str::from_utf8(data).map_err(|err| err.to_string())?;
    Composition::deserialize_json(json)
  }

  fn save_to_json(composition: &Composition) -> String {
    composition.serialize_json()
  }
}

impl Convert for JsonConverter {
  fn load(path: &str) -> Result<Composition, String> {
    let data = fs::read(path).map_err(|err| err.to_string())?;
    JsonConverter::load_from_json(&data)
  }

  fn load_data(data: &[u8]) -> Result<Composition, String> {
    JsonConverter::load_from_json(data)
  }

  fn save(path: &str, composition: &Composition) -> Result<usize, String> {
    let json = JsonConverter::save_to_json(composition);
    fs::write(path, json.as_bytes()).map_err(|err| err.to_string())?;
    Ok(json.as_bytes().len())
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::*;

  #[test]
  fn test_json_serialization_direct() {
    let mut composition = Composition::new(
      "Test Composition",
      Some(Tempo::new(Duration::new(DurationType::Half, 1), 118)),
      Some(Key::from_fifths(5, Some(KeyMode::Minor))),
      Some(TimeSignature::new_explicit(3, 8)),
    );
    composition.set_copyright("Copyright Vandy");
    composition.set_publisher("Publisher Example");
    composition.add_arranger("Arranger Name1");
    composition.add_arranger("Arranger Name2");
    composition.add_composer("Composer Name1");
    composition.add_lyricist("Lyricist Name1");
    composition.add_lyricist("Lyricist Name2");
    composition.add_metadata("TestKey1", "TestValue1");
    composition.add_metadata("TestKey2", "TestValue2");
    let part = composition.add_part("Guitar");
    let section = part.add_section("Intro");
    let subsection = section.borrow_mut().add_section("Subsection");
    let staff = subsection.borrow_mut().add_staff("Staff1", None, None, None);
    staff.borrow_mut().add_direction(DirectionType::AccordionRegistration {
      high: true,
      middle: 2,
      low: true,
    });
    staff.borrow_mut().add_direction(DirectionType::BreathMark);
    staff.borrow_mut().add_direction(DirectionType::Caesura);
    staff.borrow_mut().add_direction(DirectionType::ClefChange {
      clef: Clef::new(ClefType::Alto, None),
    });
    staff.borrow_mut().add_direction(DirectionType::ClefChange {
      clef: Clef::new(ClefType::Baritone, None),
    });
    staff.borrow_mut().add_direction(DirectionType::ClefChange {
      clef: Clef::new(ClefType::Bass, None),
    });
    staff.borrow_mut().add_direction(DirectionType::ClefChange {
      clef: Clef::new(ClefType::FrenchViolin, None),
    });
    staff.borrow_mut().add_direction(DirectionType::ClefChange {
      clef: Clef::new(ClefType::MezzoSoprano, None),
    });
    staff.borrow_mut().add_direction(DirectionType::ClefChange {
      clef: Clef::new(ClefType::Soprano, None),
    });
    staff.borrow_mut().add_direction(DirectionType::ClefChange {
      clef: Clef::new(ClefType::Subbass, None),
    });
    staff.borrow_mut().add_direction(DirectionType::ClefChange {
      clef: Clef::new(ClefType::Tenor, None),
    });
    staff.borrow_mut().add_direction(DirectionType::ClefChange {
      clef: Clef::new(ClefType::Treble, None),
    });
    staff.borrow_mut().add_direction(DirectionType::Dynamic {
      dynamic: Dynamic::new(DynamicMarking::None, 0),
    });
    staff.borrow_mut().add_direction(DirectionType::Dynamic {
      dynamic: Dynamic::new(DynamicMarking::Piano, 2),
    });
    staff.borrow_mut().add_direction(DirectionType::Dynamic {
      dynamic: Dynamic::new(DynamicMarking::Forte, 4),
    });
    staff.borrow_mut().add_direction(DirectionType::Dynamic {
      dynamic: Dynamic::new(DynamicMarking::MezzoPiano, 1),
    });
    staff.borrow_mut().add_direction(DirectionType::Dynamic {
      dynamic: Dynamic::new(DynamicMarking::MezzoForte, 1),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::AFlat, KeyMode::Major),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::AFlat, KeyMode::Minor),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::A, KeyMode::Major),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::A, KeyMode::Minor),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::ASharp, KeyMode::Minor),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::BFlat, KeyMode::Major),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::BFlat, KeyMode::Minor),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::B, KeyMode::Major),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::B, KeyMode::Minor),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::CFlat, KeyMode::Major),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::C, KeyMode::Major),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::C, KeyMode::Minor),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::CSharp, KeyMode::Major),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::CSharp, KeyMode::Minor),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::DFlat, KeyMode::Major),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::D, KeyMode::Major),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::D, KeyMode::Minor),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::DSharp, KeyMode::Minor),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::EFlat, KeyMode::Major),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::EFlat, KeyMode::Minor),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::E, KeyMode::Major),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::E, KeyMode::Minor),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::F, KeyMode::Major),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::F, KeyMode::Minor),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::FSharp, KeyMode::Major),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::FSharp, KeyMode::Minor),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::GFlat, KeyMode::Major),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::G, KeyMode::Major),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::G, KeyMode::Minor),
    });
    staff.borrow_mut().add_direction(DirectionType::KeyChange {
      key: Key::new(KeySignature::GSharp, KeyMode::Minor),
    });
    staff.borrow_mut().add_direction(DirectionType::StringMute { on: true });
    staff.borrow_mut().add_direction(DirectionType::TimeSignatureChange {
      time_signature: TimeSignature::new_explicit(3, 8),
    });
    staff.borrow_mut().add_direction(DirectionType::TimeSignatureChange {
      time_signature: TimeSignature::new(TimeSignatureType::CutTime),
    });
    staff.borrow_mut().add_direction(DirectionType::TimeSignatureChange {
      time_signature: TimeSignature::new(TimeSignatureType::None),
    });
    staff.borrow_mut().add_direction(DirectionType::TimeSignatureChange {
      time_signature: TimeSignature::new(TimeSignatureType::CommonTime),
    });
    staff.borrow_mut().add_note(
      Pitch::new_rest(),
      Duration::new(DurationType::Maxima, 0),
      Some(Accidental::None),
    );
    staff.borrow_mut().add_note(
      Pitch::new(PitchName::A, 2),
      Duration::new(DurationType::Long, 1),
      Some(Accidental::Sharp),
    );
    staff.borrow_mut().add_note(
      Pitch::new(PitchName::B, 2),
      Duration::new(DurationType::Breve, 2),
      Some(Accidental::DoubleSharp),
    );
    staff.borrow_mut().add_note(
      Pitch::new(PitchName::C, 2),
      Duration::new(DurationType::Whole, 0),
      Some(Accidental::Flat),
    );
    staff.borrow_mut().add_note(
      Pitch::new(PitchName::D, 2),
      Duration::new(DurationType::Half, 1),
      Some(Accidental::DoubleFlat),
    );
    staff.borrow_mut().add_note(
      Pitch::new(PitchName::E, 2),
      Duration::new(DurationType::Quarter, 2),
      Some(Accidental::None),
    );
    staff.borrow_mut().add_note(
      Pitch::new(PitchName::F, 2),
      Duration::new(DurationType::Eighth, 0),
      Some(Accidental::Sharp),
    );
    staff.borrow_mut().add_note(
      Pitch::new(PitchName::G, 2),
      Duration::new(DurationType::Sixteenth, 1),
      Some(Accidental::DoubleSharp),
    );
    staff.borrow_mut().add_note(
      Pitch::new(PitchName::A, 4),
      Duration::new(DurationType::ThirtySecond, 2),
      Some(Accidental::Flat),
    );
    staff.borrow_mut().add_note(
      Pitch::new(PitchName::B, 4),
      Duration::new(DurationType::SixtyFourth, 0),
      Some(Accidental::DoubleFlat),
    );
    staff.borrow_mut().add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::OneHundredTwentyEighth, 1),
      Some(Accidental::None),
    );
    staff.borrow_mut().add_note(
      Pitch::new(PitchName::D, 4),
      Duration::new(DurationType::TwoHundredFiftySixth, 2),
      Some(Accidental::Sharp),
    );
    staff.borrow_mut().add_note(
      Pitch::new(PitchName::E, 4),
      Duration::new(DurationType::FiveHundredTwelfth, 0),
      Some(Accidental::DoubleSharp),
    );
    staff.borrow_mut().add_note(
      Pitch::new(PitchName::F, 4),
      Duration::new(DurationType::OneThousandTwentyFourth, 1),
      Some(Accidental::Flat),
    );
    staff.borrow_mut().add_note(
      Pitch::new(PitchName::G, 4),
      Duration::new(DurationType::TwoThousandFortyEighth, 2),
      Some(Accidental::DoubleFlat),
    );
    let note = staff.borrow_mut().add_note(
      Pitch::new(PitchName::C, 4),
      Duration::new(DurationType::Quarter, 0),
      Some(Accidental::None),
    );
    note.borrow_mut().add_modification(NoteModificationType::Accent);
    note.borrow_mut().add_modification(NoteModificationType::BrassBend);
    note.borrow_mut().add_modification(NoteModificationType::DetachedLegato);
    note.borrow_mut().add_modification(NoteModificationType::Doit);
    note.borrow_mut().add_modification(NoteModificationType::DoubleTongue);
    note.borrow_mut().add_modification(NoteModificationType::DownBow);
    note.borrow_mut().add_modification(NoteModificationType::Dynamic {
      dynamic: Dynamic::new(DynamicMarking::Piano, 1),
    });
    note.borrow_mut().add_modification(NoteModificationType::Falloff);
    note.borrow_mut().add_modification(NoteModificationType::Fermata);
    note.borrow_mut().add_modification(NoteModificationType::Fingernails);
    note.borrow_mut().add_modification(NoteModificationType::Flip);
    note.borrow_mut().add_modification(NoteModificationType::Glissando {
      from_current: false,
      going_up: true,
    });
    note.borrow_mut().add_modification(NoteModificationType::Golpe);
    note.borrow_mut().add_modification(NoteModificationType::Grace {
      acciaccatura: true,
      note_value: 45,
    });
    note.borrow_mut().add_modification(NoteModificationType::HalfMuted);
    note.borrow_mut().add_modification(NoteModificationType::Handbell {
      technique: HandbellTechnique::Belltree,
    });
    note.borrow_mut().add_modification(NoteModificationType::Handbell {
      technique: HandbellTechnique::Damp,
    });
    note.borrow_mut().add_modification(NoteModificationType::Handbell {
      technique: HandbellTechnique::Echo,
    });
    note.borrow_mut().add_modification(NoteModificationType::Handbell {
      technique: HandbellTechnique::Gyro,
    });
    note.borrow_mut().add_modification(NoteModificationType::Handbell {
      technique: HandbellTechnique::HandMartellato,
    });
    note.borrow_mut().add_modification(NoteModificationType::Handbell {
      technique: HandbellTechnique::MalletLift,
    });
    note.borrow_mut().add_modification(NoteModificationType::Handbell {
      technique: HandbellTechnique::MalletTable,
    });
    note.borrow_mut().add_modification(NoteModificationType::Handbell {
      technique: HandbellTechnique::Martellato,
    });
    note.borrow_mut().add_modification(NoteModificationType::Handbell {
      technique: HandbellTechnique::MartellatoLift,
    });
    note.borrow_mut().add_modification(NoteModificationType::Handbell {
      technique: HandbellTechnique::MutedMartellato,
    });
    note.borrow_mut().add_modification(NoteModificationType::Handbell {
      technique: HandbellTechnique::PluckLift,
    });
    note.borrow_mut().add_modification(NoteModificationType::Handbell {
      technique: HandbellTechnique::Swing,
    });
    note
      .borrow_mut()
      .add_modification(NoteModificationType::HarmonMute { open: true, half: true });
    note.borrow_mut().add_modification(NoteModificationType::Haydn);
    note.borrow_mut().add_modification(NoteModificationType::Heel);
    note.borrow_mut().add_modification(NoteModificationType::Hole {
      open: true,
      half: false,
    });
    note.borrow_mut().add_modification(NoteModificationType::Marcato);
    note
      .borrow_mut()
      .add_modification(NoteModificationType::Mordent { upper: true });
    note.borrow_mut().add_modification(NoteModificationType::Open);
    note.borrow_mut().add_modification(NoteModificationType::Pizzicato);
    note.borrow_mut().add_modification(NoteModificationType::Plop);
    note.borrow_mut().add_modification(NoteModificationType::Portamento {
      from_current: true,
      going_up: false,
    });
    note.borrow_mut().add_modification(NoteModificationType::Schleifer);
    note.borrow_mut().add_modification(NoteModificationType::Scoop);
    note.borrow_mut().add_modification(NoteModificationType::Sforzando);
    note.borrow_mut().add_modification(NoteModificationType::Shake);
    note.borrow_mut().add_modification(NoteModificationType::Smear);
    note.borrow_mut().add_modification(NoteModificationType::SoftAccent);
    note.borrow_mut().add_modification(NoteModificationType::Spiccato);
    note.borrow_mut().add_modification(NoteModificationType::Staccatissimo);
    note.borrow_mut().add_modification(NoteModificationType::Staccato);
    note.borrow_mut().add_modification(NoteModificationType::Stopped);
    note.borrow_mut().add_modification(NoteModificationType::Stress);
    note.borrow_mut().add_modification(NoteModificationType::Tap);
    note.borrow_mut().add_modification(NoteModificationType::Tenuto);
    note.borrow_mut().add_modification(NoteModificationType::ThumbPosition);
    note.borrow_mut().add_modification(NoteModificationType::Tie);
    note.borrow_mut().add_modification(NoteModificationType::Toe);
    note
      .borrow_mut()
      .add_modification(NoteModificationType::Tremolo { relative_speed: 2 });
    note
      .borrow_mut()
      .add_modification(NoteModificationType::Trill { upper: true });
    note.borrow_mut().add_modification(NoteModificationType::TripleTongue);
    note.borrow_mut().add_modification(NoteModificationType::Turn {
      upper: false,
      delayed: false,
      vertical: true,
    });
    note.borrow_mut().add_modification(NoteModificationType::Unstress);
    note.borrow_mut().add_modification(NoteModificationType::UpBow);
    let chord = staff.borrow_mut().add_chord();
    chord.borrow_mut().add_note(
      Pitch::new(PitchName::C, 3),
      Duration::new(DurationType::Quarter, 0),
      None,
    );
    chord.borrow_mut().add_note(
      Pitch::new(PitchName::E, 3),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    chord.borrow_mut().add_note(
      Pitch::new(PitchName::G, 3),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    chord.borrow_mut().add_modification(ChordModificationType::Accent);
    chord.borrow_mut().add_modification(ChordModificationType::Arpeggiate);
    chord
      .borrow_mut()
      .add_modification(ChordModificationType::DetachedLegato);
    chord.borrow_mut().add_modification(ChordModificationType::DownBow);
    chord.borrow_mut().add_modification(ChordModificationType::Dynamic {
      dynamic: Dynamic::new(DynamicMarking::Forte, 3),
    });
    chord.borrow_mut().add_modification(ChordModificationType::Fermata);
    chord.borrow_mut().add_modification(ChordModificationType::Fingernails);
    chord.borrow_mut().add_modification(ChordModificationType::HalfMuted);
    chord
      .borrow_mut()
      .add_modification(ChordModificationType::HarmonMute { open: true, half: true });
    chord.borrow_mut().add_modification(ChordModificationType::Heel);
    chord.borrow_mut().add_modification(ChordModificationType::Marcato);
    chord
      .borrow_mut()
      .add_modification(ChordModificationType::NonArpeggiate);
    chord.borrow_mut().add_modification(ChordModificationType::Open);
    chord.borrow_mut().add_modification(ChordModificationType::Pizzicato);
    chord.borrow_mut().add_modification(ChordModificationType::Sforzando);
    chord.borrow_mut().add_modification(ChordModificationType::Smear);
    chord.borrow_mut().add_modification(ChordModificationType::SoftAccent);
    chord.borrow_mut().add_modification(ChordModificationType::Spiccato);
    chord
      .borrow_mut()
      .add_modification(ChordModificationType::Staccatissimo);
    chord.borrow_mut().add_modification(ChordModificationType::Staccato);
    chord.borrow_mut().add_modification(ChordModificationType::Stress);
    chord.borrow_mut().add_modification(ChordModificationType::Tenuto);
    chord.borrow_mut().add_modification(ChordModificationType::Tie);
    chord.borrow_mut().add_modification(ChordModificationType::Toe);
    chord
      .borrow_mut()
      .add_modification(ChordModificationType::Tremolo { relative_speed: 1 });
    chord.borrow_mut().add_modification(ChordModificationType::Unstress);
    chord.borrow_mut().add_modification(ChordModificationType::UpBow);
    let phrase = staff.borrow_mut().add_phrase();
    phrase.borrow_mut().add_note(
      Pitch::new(PitchName::C, 3),
      Duration::new(DurationType::Quarter, 0),
      None,
    );
    let chord = phrase.borrow_mut().add_chord();
    chord.borrow_mut().add_note(
      Pitch::new(PitchName::C, 3),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    chord.borrow_mut().add_note(
      Pitch::new(PitchName::E, 3),
      Duration::new(DurationType::Eighth, 0),
      None,
    );
    let subphrase = phrase.borrow_mut().add_phrase();
    subphrase.borrow_mut().add_note(
      Pitch::new(PitchName::C, 3),
      Duration::new(DurationType::Quarter, 0),
      None,
    );
    subphrase.borrow_mut().add_note(
      Pitch::new(PitchName::D, 3),
      Duration::new(DurationType::Quarter, 0),
      None,
    );
    subphrase.borrow_mut().add_note(
      Pitch::new(PitchName::E, 3),
      Duration::new(DurationType::Quarter, 0),
      None,
    );
    subphrase
      .borrow_mut()
      .add_modification(PhraseModificationType::Crescendo {
        final_dynamic: Dynamic::new(DynamicMarking::None, 0),
      });
    subphrase
      .borrow_mut()
      .add_modification(PhraseModificationType::Decrescendo {
        final_dynamic: Dynamic::new(DynamicMarking::Piano, 3),
      });
    subphrase
      .borrow_mut()
      .add_modification(PhraseModificationType::Glissando);
    subphrase
      .borrow_mut()
      .add_modification(PhraseModificationType::Hairpin {
        maximum_dynamic: Dynamic::new(DynamicMarking::Forte, 4),
      });
    subphrase.borrow_mut().add_modification(PhraseModificationType::Legato);
    subphrase
      .borrow_mut()
      .add_modification(PhraseModificationType::OctaveShift { num_octaves: -1 });
    subphrase.borrow_mut().add_modification(PhraseModificationType::Pedal {
      pedal_type: PedalType::Soft,
    });
    subphrase.borrow_mut().add_modification(PhraseModificationType::Pedal {
      pedal_type: PedalType::Sustain,
    });
    subphrase.borrow_mut().add_modification(PhraseModificationType::Pedal {
      pedal_type: PedalType::Sostenuto,
    });
    subphrase
      .borrow_mut()
      .add_modification(PhraseModificationType::Portamento);
    subphrase
      .borrow_mut()
      .add_modification(PhraseModificationType::Tremolo { relative_speed: 3 });
    subphrase.borrow_mut().add_modification(PhraseModificationType::Tuplet {
      num_beats: 3,
      into_beats: 2,
    });
    section
      .borrow_mut()
      .add_modification(SectionModificationType::Accelerando);
    section
      .borrow_mut()
      .add_modification(SectionModificationType::OnlyPlay {
        iterations: vec![1, 2, 4],
      });
    section
      .borrow_mut()
      .add_modification(SectionModificationType::Rallentando);
    section
      .borrow_mut()
      .add_modification(SectionModificationType::Repeat { num_times: 2 });
    section
      .borrow_mut()
      .add_modification(SectionModificationType::Ritardando);
    section.borrow_mut().add_modification(SectionModificationType::Ritenuto);
    section
      .borrow_mut()
      .add_modification(SectionModificationType::Stringendo);
    section
      .borrow_mut()
      .add_modification(SectionModificationType::TempoExplicit {
        tempo: Tempo::new(Duration::new(DurationType::Whole, 0), 10),
      });
    section
      .borrow_mut()
      .add_modification(SectionModificationType::TempoImplicit {
        tempo: TempoSuggestion::new(TempoMarking::Allegretto),
      });
    let serialized = composition.serialize_json();
    match JsonConverter::load_data(serialized.as_bytes()) {
      Ok(ref loaded) => {
        let reserialized = loaded.serialize_json();
        assert_eq!(serialized, reserialized);
      }
      Err(error) => assert!(false, "{}", error),
    }
  }

  #[test]
  fn test_json_serialization_fs() {
    let mut composition = Storage::MusicXML.load("examples/Grande Valse Brillante.musicxml");
    match composition {
      Ok(ref mut composition) => match Storage::JSON.save("../target/test_out.json", composition) {
        Ok(size) => {
          println!("Successfully stored JSON file containing {size} bytes");
          match Storage::JSON.load("../target/test_out.json") {
            Ok(ref mut loaded) => {
              println!("Re-imported file from JSON representation, comparing to original...");
              let original = composition.serialize_json();
              let new = loaded.serialize_json();
              assert_eq!(original, new);
            }
            Err(error) => assert!(false, "{}", error),
          }
        }
        Err(error) => assert!(false, "{}", error),
      },
      Err(error) => assert!(false, "{}", error),
    }
  }
}
