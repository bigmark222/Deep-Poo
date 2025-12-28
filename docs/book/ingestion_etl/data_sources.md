# Data sources and capture/prune

## Capture inputs
- Primary source: filtered captures under `assets/datasets/captures_filtered` (pruning applied). Adjust path for your workspace.
- Raw captures: keep originals under a separate root; apply pruning/filters before ETL to avoid junk data entering the warehouse.

## Pruning and filters
- Filter out empties/invalids early (e.g., missing images, bad labels). Maintain a reproducible filter script/config and commit it.
- Common knobs to expose: allowed roots/globs, minimum image resolution, max invalid ratio per batch, and class/label allowlists.
- Record the prune config alongside the ETL invocation (e.g., in logs or manifest metadata) to keep runs reproducible.

## Pre-ETL checklist
- Paths exist and are readable; filtered root is non-empty.
- Expected label/image schema matches what ETL expects (see schemas/validation).
- Disk space is sufficient for shards + manifest + temporary scratch.
