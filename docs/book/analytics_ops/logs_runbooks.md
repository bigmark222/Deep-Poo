# Logs, metrics, checks, and runbooks

## Logs & metrics
- ETL: stdout/stderr; optional trace file if enabled. Check for validation failures and timings.
- Training: stdout/stderr; optional status file (e.g., `logs/train_status.json`) for progress/resume.
- Export: stdout/stderr from `warehouse_export`; confirm Parquet written.
- Metrics: if available, note Grafana/Prometheus dashboards here (add links once set).

## Routine checks
- Before training: manifest path/version, checksum validation, WGPU envs set.
- During training: throughput vs expected, batch/epoch counts, memory usage.
- After training: checkpoint presence, status file updated, logs free of repeated warnings.

## Runbook snippets
- Training slow: verify store mode (memory/mmap/stream), storage IO, and prefetch; adjust `WAREHOUSE_PREFETCH` for stream.
- Adapter/driver issues: set `WGPU_BACKEND` and `WGPU_ADAPTER_NAME` explicitly; confirm driver versions; try fallback backend if available.
- Export/ETL failure: rerun on a tiny subset; inspect first failing sample; check disk space and permissions; confirm env toggles (`WAREHOUSE_CLEAR`, `WAREHOUSE_SKIP_IF_EXISTS`).

## Escalation
- Gather: command used, manifest version, env vars, hardware (GPU/driver/OS), and relevant log excerpts.
- Reproduce: attempt with a small dataset/shard; note whether the issue is data-specific or systemic.
- Report: file an issue with the above details and steps taken; include any traces/metrics if available.
