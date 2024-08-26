use amm_sdk::Storage;

fn main() {
  let mut composition = Storage::MusicXML.load("./tests/Grande Valse Brillante.musicxml");
  match composition {
    Ok(ref mut composition) => {
      let mut composition = composition.restructure_staves_as_parts();
      println!("{}", composition);
      for part_name in &composition.get_part_names() {
        let part = unsafe { composition.get_part_by_name(part_name).unwrap_unchecked() };
        println!("\nPart {part_name}:");
        //part.iter_timeslices().into_iter().for_each(|timeslice| {
        //  println!("  {}", timeslice);
        //});
        println!("{}\n\n\n", part);
      }
    }
    Err(error) => println!("{}", error),
  }
}
