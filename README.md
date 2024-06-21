# amm-sdk
Abstract Music Manipulation (AMM) SDK


-For playback/editing, have concept of "cursor" that moves along timeslices (can be placed at beginning/middle/end of timeslice)
  -Can also select ranges of timeslices
-Make "optimize" function that determines that fastest possible timeslice and use that as the time quantization level
  -Auto call this on load and save, undo on edit?
-Add "validate" function that ensures that there are measure bars in the right place (parameter to auto-add?)
-Add option of half/whole steps to trills/turns/mordents/etc


-Make get_slices_for_playback(): Create implicit slices for things like glissandos and mordents
-Make get_audio_buffer(): Create PCM audio buffer for note + mods for use in direct playback


-Make TimeSignature an enum, add common_time, cut_time
-What exactly is notational::wavy_line
