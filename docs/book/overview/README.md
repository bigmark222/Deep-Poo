# Overview

This page orients newcomers to the full data/training pipeline and gives a 10-minute quickstart. Replace the diagram placeholder once a visual is ready.

![Pipeline diagram — replace with final graphic](../media/pipeline_overview.png)

## Lifecycle at a glance

| Stage | What happens | Key artifacts/inputs | Primary commands |
| --- | --- | --- | --- |
| Ingest | Capture and prune raw data | Raw captures, filters | `cargo run --bin data_ingest …` |
| ETL | Validate, transform, shard | `manifest.json`, `shard_*.bin` | `cargo run --bin warehouse_etl …` |
| Warehouse | Store versioned tensors | `artifacts/tensor_warehouse/v<version>/` | n/a (consumed by training) |
| Train | Read warehouse, train model | checkpoints, logs | `cargo train_hp -- --tensor-warehouse …` |
| Evaluate | Analyze outputs/metrics | metrics, Parquet exports | `cargo run --bin warehouse_export …` |

## 10-minute quickstart

Follow these steps end-to-end; swap paths to match your machine.

1) Ingest or prepare filtered data roots (see ingestion chapter for capture/prune commands).  
2) Build the warehouse:  
```bash
CODE_VERSION=$(git rev-parse --short HEAD 2>/dev/null || echo "") \
cargo run --bin warehouse_etl -- \
  --input-root assets/datasets/captures_filtered \
  --output-root artifacts/tensor_warehouse \
  --target-size 384x384 \
  --resize-mode letterbox \
  --max-boxes 16 \
  --shard-samples 1024
```
3) Train from the manifest:  
```bash
cargo train_hp -- \
  --tensor-warehouse artifacts/tensor_warehouse/v<version>/manifest.json \
  --batch-size 64 \
  --epochs 20 \
  --status-file logs/train_status.json
```
Set WGPU env vars if needed (see training section).  
4) Inspect outputs: logs, checkpoints, and optional Parquet export:  
```bash
cargo run --bin warehouse_export -- \
  --manifest artifacts/tensor_warehouse/v<version>/manifest.json \
  --out logs/warehouse_summary.parquet
```
5) Troubleshoot with the FAQ if anything looks off, then iterate.
