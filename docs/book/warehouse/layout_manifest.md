# Layout, versioning, and manifest fields

## Directory layout
- Root: `artifacts/tensor_warehouse/v<version>/`
- Files: `manifest.json`, `shard_*.bin` (f32 tensors; header + payload).
- Optional extras: logs/trace outputs if enabled (per ETL config).

## Versioning
- Version key = SHA256 of (source root + cacheable transform config + max_boxes + skip_empty + code_version).
- Bump when any cacheable transform changes, prune config changes, schema changes, or code affecting ETL outputs changes.
- Store `code_version` (short git SHA) and prune/transform config alongside version in the manifest.

## Manifest fields (high-level)
| Field | Meaning |
| --- | --- |
| `dataset_root` | Source data root used for this build. |
| `transform` | Cacheable transforms applied (resize mode/size, normalize, max_boxes, skip_empty, etc.). |
| `version` | Derived version key (see above). |
| `code_version` | Short git SHA of the ETL code used. |
| `thresholds` | Validation thresholds (max invalid/missing/empty ratios). |
| `summaries` | Per-run + aggregate stats (counts, invalid/missing/empty, timings). |
| `shards` | List of shard metadata: filename, checksum (SHA256), dtype, counts, offsets. |

## Checksums and integrity
- Each shard records SHA256 in the manifest; loaders should verify before use.
- Manifest should include a checksum for itself (optional) if external distribution is expected.
- Validate shard header (magic/version/dtype/shape/offsets) on load; reject mismatches.

## When to bump versions
- Transform config change (target size, resize mode, normalization, max boxes).
- Schema/label changes (new fields, format shifts).
- Prune/filter config change that affects included samples.
- Code changes that alter tensor contents or shard layout.
- Upstream data change (new captures) even if config is identical.
