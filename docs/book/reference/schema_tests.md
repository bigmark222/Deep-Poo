# Data schema tables and test coverage map

## Data schema (high-level)
- Images: CHW, dtype `f32`, target size (e.g., 3x384x384), stored in shards.
- Boxes: normalized coordinates after transforms; padded to max_boxes; dtype `f32`.
- Masks (if present): aligned to target size; dtype `f32`.
- Manifest: dataset root, transform config, version hash, code_version, thresholds, summaries, shard list with checksums/dtypes.

## Test coverage map (current)
- Shard loader / store modes (feature `burn_runtime`):
  - `tests/warehouse_store.rs::store_modes_len_match` — memory/mmap/stream see same lengths.
  - `tests/warehouse_store.rs::streaming_vs_ram_throughput_smoke` — smoke throughput across modes.
  - `tests/warehouse_store.rs::streaming_bench_optional` — optional bench (gated by `STREAM_BENCH=1`).
- Warehouse command builder:
  - `tools/warehouse_commands/tests/builder_outputs.rs::{powershell_amd_matches_legacy,bash_amd_matches_legacy,powershell_nvidia_matches_legacy,bash_nvidia_matches_legacy,non_stream_omits_prefetch_and_allows_extra_args}` — exact one-liner outputs for presets and edge case.
- Training harness (feature `burn_runtime`):
  - `tests/train_harness.rs::train_harness_runs_multi_step_batch_gt1` — tiny synthetic dataset, two-step training loop sanity.
- Datagen smoke:
  - `tests/datagen_smoke.rs::datagen_headless_smoke` — runs headless datagen on a tiny frame cap (gated by `RUN_DATAGEN_SMOKE=1`).

Gaps to fill:
- ETL validation on tiny dataset (manifest + shard checksums).
- Schema evolution tests when adding fields (end-to-end ETL → loader).
- Additional training/loader assertions tied to manifest counts/epoch math.
