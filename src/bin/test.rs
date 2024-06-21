use amm_sdk::Composition;
use amm_sdk::Storage;
use amm_sdk::{Accidental, Duration, Note, Pitch};
use amm_sdk::{Clef, Key, Tempo, TimeSignature};
use amm_sdk::{NoteModification, PhraseModification};

fn main() {
  let composition = Storage::MusicXML.load("./tests/Grande Valse Brillante.musicxml");
  println!("{}", composition.unwrap());

  /*
  // Create a new composition complete with metadata, a piano system, and a left- and right-hand staff
  let mut composition = Composition::new(
    "Grande Valse Brillante",
    Some(Tempo::new(Duration::Quarter(0), 220)),
    Some(Key::EFlatMajor),
    Some(TimeSignature::new(3, 4)),
  );
  composition
    .update_copyright("1992 Alfred Publishing Co., Inc.")
    .update_publisher("Alfred Publishing Co., Inc.")
    .add_composer("Frédéric Chopin")
    .add_metadata("Opus", "18");
  let piano_system = composition.add_system("Piano");
  let right_hand = piano_system.borrow_mut().add_staff("Right Hand", Some(Clef::Treble));
  let left_hand = piano_system.borrow_mut().add_staff("Left Hand", Some(Clef::Bass));

  //
  right_hand
    .borrow_mut()
    .add_musical_slice()
    .add_note(Note::new(Pitch::B(4), Duration::Half(0), None))
    .add_note_modification(NoteModification::Accent);
  right_hand
    .borrow_mut()
    .add_musical_slice()
    .add_note(Note::new(Pitch::B(4), Duration::Eighth(0), None))
    .add_note_modification(NoteModification::Staccato);
  */
}

/*
fn main() {
  println!("Accidentals:");
  for accidental in Accidental::iter() {
    println!("   {accidental}: {} ", accidental.value());
  }

  println!("Durations:");
  for duration in Duration::iter() {
    println!("   {duration}: {} ", duration.value());
  }

  let base_beat_duration = Duration::Quarter(0).value();
  let key_signature = Key::FMajor.accidentals();
  let note = Note::new(
    Pitch::C(4),
    Duration::Quarter(2),
    Some(Accidental::DoubleFlat),
  );
  println!(
    "{note}: Pitch = {} Hz, Midi = {}, Beats = {}",
    note.pitch(&key_signature, None),
    note.midi_number(&key_signature),
    note.beats(base_beat_duration)
  );

  let rest = Rest::new(Duration::Half(1));
  println!("{rest}: Beats = {}", rest.beats(base_beat_duration));

  let mut staff = Staff::new("Piano");
  staff.add_musical_node(Pitch::A(5), Duration::Quarter(1), None);
  staff.add_rest_node(Duration::Quarter(0));
  println!("{} Staff:", staff.name);
  for node in &staff {
    println!("   {node}");
  }
}
*/
