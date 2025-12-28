# Store modes and tuning (memory / mmap / stream)

Pick based on memory vs IO trade-offs. Set via `WAREHOUSE_STORE` (or CLI `--warehouse-store`). For streaming, tune `WAREHOUSE_PREFETCH`.

## Decision guidance
- Use **memory** when RAM is plentiful and you want fastest iteration; loads all shards upfront.
- Use **mmap** when RAM is tighter but you still want OS-managed caching; maps shard files on demand.
- Use **stream** when RAM is constrained or shards are large; relies on bounded prefetch and disk IO.

## Performance notes
- memory: highest RAM, lowest per-batch latency after warmup.
- mmap: lower RAM, performance depends on OS page cache and storage speed.
- stream: lowest RAM, more disk seeks; prefetch depth (`WAREHOUSE_PREFETCH`, default 2–4) controls buffering.

## Commands/examples
Assume `TENSOR_WAREHOUSE_MANIFEST=artifacts/tensor_warehouse/v<version>/manifest.json`.

**memory (default)**:
```bash
TENSOR_WAREHOUSE_MANIFEST=artifacts/tensor_warehouse/v<version>/manifest.json \
WAREHOUSE_STORE=memory \
cargo train_hp
```

**mmap**:
```bash
TENSOR_WAREHOUSE_MANIFEST=artifacts/tensor_warehouse/v<version>/manifest.json \
cargo run --features "burn_runtime,burn_wgpu" --bin train -- \
  --tensor-warehouse artifacts/tensor_warehouse/v<version>/manifest.json \
  --warehouse-store mmap \
  --batch-size 64
```

**stream with prefetch**:
```bash
TENSOR_WAREHOUSE_MANIFEST=artifacts/tensor_warehouse/v<version>/manifest.json \
WAREHOUSE_PREFETCH=4 \
cargo run --features "burn_runtime,burn_wgpu" --bin train -- \
  --tensor-warehouse artifacts/tensor_warehouse/v<version>/manifest.json \
  --warehouse-store stream \
  --batch-size 64 \
  --epochs 20
```

## Troubleshooting
- Verify `WAREHOUSE_STORE` and `WAREHOUSE_PREFETCH` are set as intended.
- For stream, start with prefetch 2–4; increase if IO allows, decrease if RAM is tight.
- If mmap/stream is slow, check storage throughput and concurrent IO load.
