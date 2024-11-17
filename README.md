# amm-sdk

Abstract Music Manipulation (AMM) SDK

Under heavy development - Updates forthcoming

## Under Development

* Finish `iter_timeslices()` to work with playback
  * Make `get_slices_for_playback()`: Create implicit slices for things like glissandos and mordents
    * Determines that fastest possible timeslice and use that as the time quantization level
    * Can also select ranges of timeslices
* Implement `get_pcm_samples()` on `Timeslice` to create audio buffer for note + mods to use in direct playback
* Finish MIDI Reader Implementation
* Make fully `no_std` compatible
* Create WASM build

* Add a test containing Glissandos and/or multi-note tremolos and/or implicit + explicit tempo changes
* Finish MusicXML Reader Implementation
  * Take into account `time-only` attributes
  * Scan text attributes for common items (rall., etc.)
  * Scan `sound` attributes for items maybe not recognized otherwise (rall., etc.)
* Remove `pub(crate)` from Phrase and make `MultiVoice` okay with this
* Make composition timeslice iterator a real iterator (and check that staff timeslices line up correctly)
