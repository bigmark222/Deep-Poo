# Pointing at a manifest (CLI/env)

Set the manifest path via env or CLI. Use the versioned manifest produced by ETL.

## Env-based
```bash
TENSOR_WAREHOUSE_MANIFEST=artifacts/tensor_warehouse/v<version>/manifest.json \
cargo train_hp -- \
  --batch-size 64 \
  --epochs 20 \
  --status-file logs/train_status.json
```

## CLI flag
```bash
cargo train_hp -- \
  --tensor-warehouse artifacts/tensor_warehouse/v<version>/manifest.json \
  --batch-size 64 \
  --epochs 20 \
  --warehouse-store memory
```

## Batch/epoch math
- Epoch size = `manifest.sample_count / batch_size` (rounded up). Training uses manifest counts to compute progress.
- Adjust `batch_size` based on GPU memory; scale `log_every` to keep logs readable.

## Common flags
- `--tensor-warehouse <path>` — manifest path (required if not set via env).
- `--warehouse-store <memory|mmap|stream>` — shard loading mode (defaults to memory unless overridden by env).
- `--batch-size`, `--epochs`, `--log-every`, `--status-file` — core training controls.
