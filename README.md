# amm-sdk

Abstract Music Manipulation (AMM) SDK

## Under Development

* Finish `iter_timeslices()` to work with playback
  * Make `get_slices_for_playback()`: Create implicit slices for things like glissandos and mordents
    * Determines that fastest possible timeslice and use that as the time quantization level
    * Can also select ranges of timeslices
* Implement `get_pcm_samples()` on `Timeslice` to create audio buffer for note + mods to use in direct playback
* Add option to `get_duration` to take into account tempo adjustments (e.g. accelerando, fermata, etc.) but ONLY when requested since internally this function is used to combine multivoice parts
* Finish MusicXML Reader Implementation
  * Take into account `time-only` attributes
  * Scan text attributes for common items (rall., etc.)
  * Scan `sound` attributes for items maybe not recognized otherwise (rall., etc.)
* Finish MusicXML Writer Implementation
* Finish MIDI Reader Implementation
* Finish MIDI Writer Implementation
* Finish AMM Reader Implementation
* Finish AMM Writer Implementation
* Make fully `no_std` compatible
