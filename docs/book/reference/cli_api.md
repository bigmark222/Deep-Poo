# CLI/API reference for tools

## warehouse_etl
- Purpose: build the tensor warehouse (manifest + shards) from filtered captures.
- Key flags:
  - `--input-root <path>`
  - `--output-root <path>`
  - `--target-size <HxW>`
  - `--resize-mode <letterbox|...>`
  - `--max-boxes <N>`
  - `--shard-samples <N>`
  - `--skip-empty` (if available)
  - `--code-version <sha>` (or `CODE_VERSION` env)

## warehouse_cmd
- Purpose: emit one-liner training env/command based on shell/adapter/backend.
- Key flags:
  - `--shell <ps|sh>`
  - `--adapter <amd|nvidia>`
  - `--backend <dx12|vulkan>`
  - `--manifest <path>`
  - `--store <memory|mmap|stream>`
  - `--prefetch <N>`
  - `--batch-size <N>`
  - `--log-every <N>`
  - `--extra-args <string>`
  - Convenience subcommands (if kept): `amd-ps`, `amd-sh`, `nvidia-ps`, `nvidia-sh`

## train / train_hp variants
- Purpose: train models using the tensor warehouse.
- Key flags:
  - `--tensor-warehouse <path>`
  - `--warehouse-store <memory|mmap|stream>`
  - `--batch-size <N>`
  - `--epochs <N>`
  - `--log-every <N>`
  - `--status-file <path>`
  - Other model/task-specific flags (list here once finalized).

## Notes
- Keep CLI help in sync with docs; update here when flags change.
- Add examples per tool in their respective sections (Warehouse/Training).
