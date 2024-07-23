# amm-sdk
Abstract Music Manipulation (AMM) SDK


-Make get_slices_for_playback(): Create implicit slices for things like glissandos and mordents
  -Determines that fastest possible timeslice and use that as the time quantization level
  -Can also select ranges of timeslices
-Make get_audio_buffer(): Create PCM audio buffer for note + mods for use in direct playback

-Get duration of entire staff (and every other structure type)
//pub fn duration(&self) -> f64 { self.content.iter().map(|slice| slice.duration()).sum() }


-Add way to retrieve any item within entire structure from top-level composition??
-MusicXML parser: Take into account "time-only" attributes
