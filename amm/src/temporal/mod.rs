mod timeslice;

pub use timeslice::{PartTimeslice, Timeslice, TimesliceContent, TimesliceContext, TimeslicePhraseDetails};

pub(crate) fn place_and_merge_part_timeslice(
  part_name: &str,
  timeslices: &mut Vec<(f64, PartTimeslice)>,
  slice: Timeslice,
  mut index: usize,
  curr_time: f64,
) -> (usize, f64) {
  let beat_base_note = crate::note::Duration::new(crate::note::DurationType::TwoThousandFortyEighth, 0);
  let slice_duration = slice.get_beats(&beat_base_note);
  if let Some(slice_details) = timeslices.get_mut(index) {
    let (mut slice_time, mut existing_slice) = (slice_details.0, &mut slice_details.1);
    while curr_time > slice_time && curr_time - slice_time > 0.000_001 {
      index += 1;
      (slice_time, existing_slice) = if let Some((start_time, slice)) = timeslices.get_mut(index) {
        (*start_time, slice)
      } else {
        timeslices.push((curr_time, PartTimeslice::default()));
        let (start_time, slice) = unsafe { timeslices.last_mut().unwrap_unchecked() };
        (*start_time, slice)
      };
    }
    if (slice_time - curr_time).abs() < 0.000_001 {
      existing_slice.add_timeslice(part_name, slice);
    } else {
      timeslices.insert(index, (curr_time, PartTimeslice::from(part_name, slice)));
    }
  } else {
    timeslices.push((curr_time, PartTimeslice::from(part_name, slice)));
  }
  (index + 1, curr_time + slice_duration)
}
