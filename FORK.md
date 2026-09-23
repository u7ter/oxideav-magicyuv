# Fork: u7ter/oxideav-magicyuv

Fork of [OxideAV/oxideav-magicyuv](https://github.com/OxideAV/oxideav-magicyuv).

Branch `harden-bounds` adds slice-table checks for the same attack class as FFmpeg CVE-2026-8461 (PixelSmash):

- `MAX_SLICES` cap
- `checked_mul` / `checked_add` on table size and slice offsets
- reject non-monotonic offset table
- reject slice start before preamble end
- new errors: `TooManySlices`, `BadSliceTable`
