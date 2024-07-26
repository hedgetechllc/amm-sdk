# amm-sdk
Abstract Music Manipulation (AMM) SDK


-Make get_slices_for_playback(): Create implicit slices for things like glissandos and mordents
  -Determines that fastest possible timeslice and use that as the time quantization level
  -Can also select ranges of timeslices
-Make get_audio_buffer(): Create PCM audio buffer for note + mods for use in direct playback
-MusicXML parser: Take into account "time-only" attributes
