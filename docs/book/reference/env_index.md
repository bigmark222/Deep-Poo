# Env var index (with defaults)

| Env var | Default | Purpose |
| --- | --- | --- |
| `TENSOR_WAREHOUSE_MANIFEST` | (none) | Path to manifest; required unless provided via CLI. |
| `WAREHOUSE_STORE` | `memory` | Shard loading mode (`memory`, `mmap`, `stream`). |
| `WAREHOUSE_PREFETCH` | `2` (typical) | Prefetch depth for streaming store. |
| `WAREHOUSE_CLEAR` | unset | If `1`, clear computed version dir before writing (ETL). |
| `WAREHOUSE_SKIP_IF_EXISTS` | unset | If `1`, skip ETL if versioned manifest already exists. |
| `CODE_VERSION` | git short SHA or empty | Embed code version into manifest. |
| `WGPU_POWER_PREF` | unset | WGPU power preference (set to `high-performance`). |
| `WGPU_BACKEND` | platform default | WGPU backend (`dx12`, `vulkan`, etc.). |
| `WGPU_ADAPTER_NAME` | unset | GPU adapter name; set when multiple GPUs exist. |
| `RUST_LOG` | `info` | Logging level; often `info,wgpu_core=info`. |
| `BURN_DATASET_MAX_INVALID` | tool default | Validation threshold for invalid samples (ETL). |
| `BURN_DATASET_MAX_MISSING` | tool default | Validation threshold for missing samples (ETL). |
| `BURN_DATASET_MAX_EMPTY_RATIO` | tool default | Max empty ratio (ETL). |
