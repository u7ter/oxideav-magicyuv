Wired decoder is in the following commit files.
Changes vs OxideAV master:

1. `use crate::slice_bounds::{self, MAX_SLICES};`
2. `total_slices` uses checked_mul + MAX_SLICES cap
3. `table_bytes` uses checked_add/checked_mul
4. `slice_bounds::reject_nonmonotonic_table` after parsing entries
5. `preamble_end` passed into decode_eight_bit / decode_high_bit_depth
6. slice payload range via `slice_bounds::slice_byte_range`

Local verification: rustc 1.98.1 `cargo test --no-default-features --lib` → 174 passed.
