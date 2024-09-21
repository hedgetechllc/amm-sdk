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
    {
      let part = composition.add_part("Guitar");
      let section = part.add_section("Intro");
      let subsection = section.add_section("Subsection");
      let staff = subsection.add_staff("Staff1");
      staff.add_direction(DirectionType::AccordionRegistration {
        high: true,
        middle: 2,
        low: true,
      });
      staff.add_direction(DirectionType::BreathMark);
      staff.add_direction(DirectionType::Caesura);
      staff.add_direction(DirectionType::ClefChange {
        clef: Clef::new(ClefType::Alto, None),
      });
      staff.add_direction(DirectionType::ClefChange {
        clef: Clef::new(ClefType::Baritone, None),
      });
      staff.add_direction(DirectionType::ClefChange {
        clef: Clef::new(ClefType::Bass, None),
      });
      staff.add_direction(DirectionType::ClefChange {
        clef: Clef::new(ClefType::FrenchViolin, None),
      });
      staff.add_direction(DirectionType::ClefChange {
        clef: Clef::new(ClefType::MezzoSoprano, None),
      });
      staff.add_direction(DirectionType::ClefChange {
        clef: Clef::new(ClefType::Soprano, None),
      });
      staff.add_direction(DirectionType::ClefChange {
        clef: Clef::new(ClefType::Subbass, None),
      });
      staff.add_direction(DirectionType::ClefChange {
        clef: Clef::new(ClefType::Tenor, None),
      });
      staff.add_direction(DirectionType::ClefChange {
        clef: Clef::new(ClefType::Treble, None),
      });
      staff.add_direction(DirectionType::Dynamic {
        dynamic: Dynamic::Piano(2),
      });
      staff.add_direction(DirectionType::Dynamic {
        dynamic: Dynamic::Forte(4),
      });
      staff.add_direction(DirectionType::Dynamic {
        dynamic: Dynamic::MezzoPiano,
      });
      staff.add_direction(DirectionType::Dynamic {
        dynamic: Dynamic::MezzoForte,
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::AFlat, KeyMode::Major),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::AFlat, KeyMode::Minor),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::A, KeyMode::Major),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::A, KeyMode::Minor),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::ASharp, KeyMode::Minor),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::BFlat, KeyMode::Major),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::BFlat, KeyMode::Minor),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::B, KeyMode::Major),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::B, KeyMode::Minor),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::CFlat, KeyMode::Major),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::C, KeyMode::Major),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::C, KeyMode::Minor),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::CSharp, KeyMode::Major),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::CSharp, KeyMode::Minor),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::DFlat, KeyMode::Major),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::D, KeyMode::Major),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::D, KeyMode::Minor),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::DSharp, KeyMode::Minor),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::EFlat, KeyMode::Major),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::EFlat, KeyMode::Minor),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::E, KeyMode::Major),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::E, KeyMode::Minor),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::F, KeyMode::Major),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::F, KeyMode::Minor),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::FSharp, KeyMode::Major),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::FSharp, KeyMode::Minor),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::GFlat, KeyMode::Major),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::G, KeyMode::Major),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::G, KeyMode::Minor),
      });
      staff.add_direction(DirectionType::KeyChange {
        key: Key::new(KeySignature::GSharp, KeyMode::Minor),
      });
      staff.add_direction(DirectionType::StringMute { on: true });
      staff.add_direction(DirectionType::TimeSignatureChange {
        time_signature: TimeSignature::new_explicit(3, 8),
      });
      staff.add_direction(DirectionType::TimeSignatureChange {
        time_signature: TimeSignature::new(TimeSignatureType::CutTime),
      });
      staff.add_direction(DirectionType::TimeSignatureChange {
        time_signature: TimeSignature::new(TimeSignatureType::None),
      });
      staff.add_direction(DirectionType::TimeSignatureChange {
        time_signature: TimeSignature::new(TimeSignatureType::CommonTime),
      });
      staff.add_note(
        Pitch::new_rest(),
        Duration::new(DurationType::Maxima, 0),
        Some(Accidental::None),
      );
      staff.add_note(
        Pitch::new(PitchName::A, 2),
        Duration::new(DurationType::Long, 1),
        Some(Accidental::Sharp),
      );
      staff.add_note(
        Pitch::new(PitchName::B, 2),
        Duration::new(DurationType::Breve, 2),
        Some(Accidental::DoubleSharp),
      );
      staff.add_note(
        Pitch::new(PitchName::C, 2),
        Duration::new(DurationType::Whole, 0),
        Some(Accidental::Flat),
      );
      staff.add_note(
        Pitch::new(PitchName::D, 2),
        Duration::new(DurationType::Half, 1),
        Some(Accidental::DoubleFlat),
      );
      staff.add_note(
        Pitch::new(PitchName::E, 2),
        Duration::new(DurationType::Quarter, 2),
        Some(Accidental::None),
      );
      staff.add_note(
        Pitch::new(PitchName::F, 2),
        Duration::new(DurationType::Eighth, 0),
        Some(Accidental::Sharp),
      );
      staff.add_note(
        Pitch::new(PitchName::G, 2),
        Duration::new(DurationType::Sixteenth, 1),
        Some(Accidental::DoubleSharp),
      );
      staff.add_note(
        Pitch::new(PitchName::A, 4),
        Duration::new(DurationType::ThirtySecond, 2),
        Some(Accidental::Flat),
      );
      staff.add_note(
        Pitch::new(PitchName::B, 4),
        Duration::new(DurationType::SixtyFourth, 0),
        Some(Accidental::DoubleFlat),
      );
      staff.add_note(
        Pitch::new(PitchName::C, 4),
        Duration::new(DurationType::OneHundredTwentyEighth, 1),
        Some(Accidental::None),
      );
      staff.add_note(
        Pitch::new(PitchName::D, 4),
        Duration::new(DurationType::TwoHundredFiftySixth, 2),
        Some(Accidental::Sharp),
      );
      staff.add_note(
        Pitch::new(PitchName::E, 4),
        Duration::new(DurationType::FiveHundredTwelfth, 0),
        Some(Accidental::DoubleSharp),
      );
      staff.add_note(
        Pitch::new(PitchName::F, 4),
        Duration::new(DurationType::OneThousandTwentyFourth, 1),
        Some(Accidental::Flat),
      );
      staff.add_note(
        Pitch::new(PitchName::G, 4),
        Duration::new(DurationType::TwoThousandFortyEighth, 2),
        Some(Accidental::DoubleFlat),
      );
      let note = staff.add_note(
        Pitch::new(PitchName::C, 4),
        Duration::new(DurationType::Quarter, 0),
        Some(Accidental::None),
      );
      note.add_modification(NoteModificationType::Accent);
      note.add_modification(NoteModificationType::BrassBend);
      note.add_modification(NoteModificationType::DetachedLegato);
      note.add_modification(NoteModificationType::Doit);
      note.add_modification(NoteModificationType::DoubleTongue);
      note.add_modification(NoteModificationType::DownBow);
      note.add_modification(NoteModificationType::Dynamic {
        dynamic: Dynamic::Piano(1),
      });
      note.add_modification(NoteModificationType::Falloff);
      note.add_modification(NoteModificationType::Fermata);
      note.add_modification(NoteModificationType::Fingernails);
      note.add_modification(NoteModificationType::Flip);
      note.add_modification(NoteModificationType::Glissando {
        from_current: false,
        going_up: true,
      });
      note.add_modification(NoteModificationType::Golpe);
      note.add_modification(NoteModificationType::Grace {
        acciaccatura: true,
        note_value: 45,
      });
      note.add_modification(NoteModificationType::HalfMuted);
      note.add_modification(NoteModificationType::Handbell {
        technique: HandbellTechnique::Belltree,
      });
      note.add_modification(NoteModificationType::Handbell {
        technique: HandbellTechnique::Damp,
      });
      note.add_modification(NoteModificationType::Handbell {
        technique: HandbellTechnique::Echo,
      });
      note.add_modification(NoteModificationType::Handbell {
        technique: HandbellTechnique::Gyro,
      });
      note.add_modification(NoteModificationType::Handbell {
        technique: HandbellTechnique::HandMartellato,
      });
      note.add_modification(NoteModificationType::Handbell {
        technique: HandbellTechnique::MalletLift,
      });
      note.add_modification(NoteModificationType::Handbell {
        technique: HandbellTechnique::MalletTable,
      });
      note.add_modification(NoteModificationType::Handbell {
        technique: HandbellTechnique::Martellato,
      });
      note.add_modification(NoteModificationType::Handbell {
        technique: HandbellTechnique::MartellatoLift,
      });
      note.add_modification(NoteModificationType::Handbell {
        technique: HandbellTechnique::MutedMartellato,
      });
      note.add_modification(NoteModificationType::Handbell {
        technique: HandbellTechnique::PluckLift,
      });
      note.add_modification(NoteModificationType::Handbell {
        technique: HandbellTechnique::Swing,
      });
      note.add_modification(NoteModificationType::HarmonMute { open: true, half: true });
      note.add_modification(NoteModificationType::Haydn);
      note.add_modification(NoteModificationType::Heel);
      note.add_modification(NoteModificationType::Hole {
        open: true,
        half: false,
      });
      note.add_modification(NoteModificationType::Marcato);
      note.add_modification(NoteModificationType::Mordent { upper: true });
      note.add_modification(NoteModificationType::Open);
      note.add_modification(NoteModificationType::Pizzicato);
      note.add_modification(NoteModificationType::Plop);
      note.add_modification(NoteModificationType::Portamento {
        from_current: true,
        going_up: false,
      });
      note.add_modification(NoteModificationType::Schleifer);
      note.add_modification(NoteModificationType::Scoop);
      note.add_modification(NoteModificationType::Sforzando);
      note.add_modification(NoteModificationType::Shake);
      note.add_modification(NoteModificationType::Smear);
      note.add_modification(NoteModificationType::SoftAccent);
      note.add_modification(NoteModificationType::Spiccato);
      note.add_modification(NoteModificationType::Staccatissimo);
      note.add_modification(NoteModificationType::Staccato);
      note.add_modification(NoteModificationType::Stopped);
      note.add_modification(NoteModificationType::Stress);
      note.add_modification(NoteModificationType::Tap);
      note.add_modification(NoteModificationType::Tenuto);
      note.add_modification(NoteModificationType::ThumbPosition);
      note.add_modification(NoteModificationType::Tie);
      note.add_modification(NoteModificationType::Toe);
      note.add_modification(NoteModificationType::Tremolo { relative_speed: 2 });
      note.add_modification(NoteModificationType::Trill { upper: true });
      note.add_modification(NoteModificationType::TripleTongue);
      note.add_modification(NoteModificationType::Turn {
        upper: false,
        delayed: false,
        vertical: true,
      });
      note.add_modification(NoteModificationType::Unstress);
      note.add_modification(NoteModificationType::UpBow);
      let mut chord = staff.add_chord();
      chord.add_note(
        Pitch::new(PitchName::C, 3),
        Duration::new(DurationType::Quarter, 0),
        None,
      );
      chord.add_note(
        Pitch::new(PitchName::E, 3),
        Duration::new(DurationType::Eighth, 0),
        None,
      );
      chord.add_note(
        Pitch::new(PitchName::G, 3),
        Duration::new(DurationType::Eighth, 0),
        None,
      );
      chord.add_modification(ChordModificationType::Accent);
      chord.add_modification(ChordModificationType::Arpeggiate);
      chord.add_modification(ChordModificationType::DetachedLegato);
      chord.add_modification(ChordModificationType::DownBow);
      chord.add_modification(ChordModificationType::Dynamic {
        dynamic: Dynamic::Forte(3),
      });
      chord.add_modification(ChordModificationType::Fermata);
      chord.add_modification(ChordModificationType::Fingernails);
      chord.add_modification(ChordModificationType::HalfMuted);
      chord.add_modification(ChordModificationType::HarmonMute { open: true, half: true });
      chord.add_modification(ChordModificationType::Heel);
      chord.add_modification(ChordModificationType::Marcato);
      chord.add_modification(ChordModificationType::NonArpeggiate);
      chord.add_modification(ChordModificationType::Open);
      chord.add_modification(ChordModificationType::Pizzicato);
      chord.add_modification(ChordModificationType::Sforzando);
      chord.add_modification(ChordModificationType::Smear);
      chord.add_modification(ChordModificationType::SoftAccent);
      chord.add_modification(ChordModificationType::Spiccato);
      chord.add_modification(ChordModificationType::Staccatissimo);
      chord.add_modification(ChordModificationType::Staccato);
      chord.add_modification(ChordModificationType::Stress);
      chord.add_modification(ChordModificationType::Tenuto);
      chord.add_modification(ChordModificationType::Tie);
      chord.add_modification(ChordModificationType::Toe);
      chord.add_modification(ChordModificationType::Tremolo { relative_speed: 1 });
      chord.add_modification(ChordModificationType::Unstress);
      chord.add_modification(ChordModificationType::UpBow);
      let phrase = staff.add_phrase();
      phrase.add_note(
        Pitch::new(PitchName::C, 3),
        Duration::new(DurationType::Quarter, 0),
        None,
      );
      chord = phrase.add_chord();
      chord.add_note(
        Pitch::new(PitchName::C, 3),
        Duration::new(DurationType::Eighth, 0),
        None,
      );
      chord.add_note(
        Pitch::new(PitchName::E, 3),
        Duration::new(DurationType::Eighth, 0),
        None,
      );
      let subphrase = phrase.add_phrase();
      subphrase.add_note(
        Pitch::new(PitchName::C, 3),
        Duration::new(DurationType::Quarter, 0),
        None,
      );
      subphrase.add_note(
        Pitch::new(PitchName::D, 3),
        Duration::new(DurationType::Quarter, 0),
        None,
      );
      subphrase.add_note(
        Pitch::new(PitchName::E, 3),
        Duration::new(DurationType::Quarter, 0),
        None,
      );
      subphrase.add_modification(PhraseModificationType::Crescendo { final_dynamic: None });
      subphrase.add_modification(PhraseModificationType::Decrescendo {
        final_dynamic: Some(Dynamic::Piano(3)),
      });
      subphrase.add_modification(PhraseModificationType::Glissando);
      subphrase.add_modification(PhraseModificationType::Hairpin {
        maximum_dynamic: Some(Dynamic::Forte(4)),
      });
      subphrase.add_modification(PhraseModificationType::Legato);
      subphrase.add_modification(PhraseModificationType::OctaveShift { num_octaves: -1 });
      subphrase.add_modification(PhraseModificationType::Pedal {
        pedal_type: PedalType::Soft,
      });
      subphrase.add_modification(PhraseModificationType::Pedal {
        pedal_type: PedalType::Sustain,
      });
      subphrase.add_modification(PhraseModificationType::Pedal {
        pedal_type: PedalType::Sostenuto,
      });
      subphrase.add_modification(PhraseModificationType::Portamento);
      subphrase.add_modification(PhraseModificationType::Tremolo { relative_speed: 3 });
      subphrase.add_modification(PhraseModificationType::Tuplet {
        num_beats: 3,
        into_beats: 2,
      });
      section.add_modification(SectionModificationType::Accelerando);
      section.add_modification(SectionModificationType::OnlyPlay {
        iterations: vec![1, 2, 4],
      });
      section.add_modification(SectionModificationType::Rallentando);
      section.add_modification(SectionModificationType::Repeat { num_times: 2 });
      section.add_modification(SectionModificationType::Ritardando);
      section.add_modification(SectionModificationType::Ritenuto);
      section.add_modification(SectionModificationType::Stringendo);
      section.add_modification(SectionModificationType::TempoExplicit {
        tempo: Tempo::new(Duration::new(DurationType::Whole, 0), 10),
      });
      section.add_modification(SectionModificationType::TempoImplicit {
        tempo: TempoSuggestion::new(TempoMarking::Allegretto),
      });
    }
    let serialized = composition.serialize_json();
    match JsonConverter::load_data(serialized.as_bytes()) {
      Ok(ref loaded) => {
        let reserialized = loaded.serialize_json();
        assert_eq!(composition, *loaded);
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
              assert_eq!(*composition, *loaded);
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
