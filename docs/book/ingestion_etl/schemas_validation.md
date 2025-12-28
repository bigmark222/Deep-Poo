# Schemas and validation

## Schema highlights
- Images: expected channel order/shape (e.g., 3xHxW), stored as tensors in shards.
- Labels: bounding boxes + masks (if present); normalized coordinates after transforms.
- Manifest: includes dataset root, transform config, version hash/recipe/code_version, thresholds, per-run + aggregate summaries, shard list with checksums/dtypes.

## Validation rules
- Enforce presence: images, labels, boxes/masks where required.
- Consistency: bounding boxes within image bounds after resizing/letterboxing; mask dimensions match target size.
- Thresholds: maximum invalid/missing/empty ratios fail the run; make thresholds explicit and configurable.
- Checksums: per-shard SHA256 recorded in manifest; reject shards with mismatched checksums.

## What to log
- Counts: total samples, per-class counts, invalid/missing/empty counts.
- Timing: per-stage ETL timings (decode, resize/letterbox, write shard).
- Version inputs: source root, transform config, code version (short SHA), prune config, max boxes, shard sizing.
