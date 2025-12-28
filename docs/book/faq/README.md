# FAQ / Troubleshooting

Common questions and quick fixes. Link back here from Warehouse/Training/Analytics pages.

## Version bumps
- Q: When do I need a new warehouse version?  
  A: Any change to cacheable transforms (resize/normalize/max_boxes), schema, prune config, or ETL code affecting outputs. Also when upstream data changes. Rebuild and record the new version hash.

- Q: Can I reuse an old manifest after code changes?  
  A: Only if the change doesn’t alter outputs. Otherwise rebuild; don’t mix mismatched code/manifest versions.

## Adapter/backend selection
- Q: Training picks the wrong GPU.  
  A: Set `WGPU_ADAPTER_NAME` to the exact adapter string and set `WGPU_BACKEND` (`dx12` on Windows, `vulkan` on Linux). Ensure drivers are current.

- Q: Do I always need `WGPU_ADAPTER_NAME`?  
  A: Set it when multiple GPUs exist; you can omit it on single-GPU machines.

## Store mode selection
- Q: Which store mode should I use?  
  A: memory for max speed and ample RAM; mmap for lower RAM with OS caching; stream for lowest RAM with more disk IO. For stream, set `WAREHOUSE_PREFETCH` (start with 2–4).

- Q: Streaming is slow.  
  A: Check disk IO; raise `WAREHOUSE_PREFETCH` if RAM allows; reduce other disk-heavy jobs; consider `mmap` if IO is the bottleneck.

## Training issues
- OOM (GPU): lower `--batch-size`; prefer `stream`; close other GPU jobs.
- OOM (host): prefer `mmap`/`stream`; lower `WAREHOUSE_PREFETCH`.
- Adapter mismatch: set `WGPU_BACKEND`/`WGPU_ADAPTER_NAME` explicitly.
- Wrong epoch math: verify manifest path/version matches the intended dataset.
- Noisy logs: increase `--log-every`; set `RUST_LOG=info,wgpu_core=warn`.

## ETL issues
- ETL fails mid-run: check input roots, thresholds (`BURN_DATASET_MAX_*`), disk space, and env toggles (`WAREHOUSE_CLEAR`, `WAREHOUSE_SKIP_IF_EXISTS`).
- Checksums mismatch: rerun ETL; ensure you’re not mixing manifests/shards from different versions.

## Where to look next
- Warehouse layout/versioning: `warehouse/layout_manifest.md`
- Store modes and commands: `warehouse/store_modes.md`
- WGPU envs: `training/wgpu_configs.md`
- Export/analytics: `analytics_ops/exports_queries.md`
