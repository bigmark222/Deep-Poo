# App Guide (colon_sim)

This repo is a single Rust crate that builds the colon simulation app and its binaries on top of published `cortenforge-*` crates from crates.io.

## Layout
- `src/`: app library and modules (`cli`, `app/*`, `vision`, `sim`).
- `src/bin/`: entrypoints (`sim_view`, `inference_view`, `datagen_headless`).
- `assets/`: app assets.
- `tests/`: integration tests (including the headless registry smoke test).
- `scripts/`: helper scripts (e.g., `dev-local.sh` for local crate overrides).

## Running
- Interactive sim: `cargo run --bin sim_view --locked`
- Inference viewer: `cargo run --bin inference_view --locked -- --detector-weights <path>`
- Headless datagen smoke: `cargo run --bin datagen_headless --locked -- --max-frames 3 --headless --output-root /tmp/captures`

## Local crate overrides (optional)
- By default, dependencies resolve from crates.io.
- To develop against local `cortenforge-*` clones, run `scripts/dev-local.sh` to write `.cargo/config.toml` with `[patch.crates-io]` entries (edit paths as needed). Remove with `scripts/dev-local.sh disable`.

## Testing
- Registry smoke: `cargo test --locked --test registry_smoke`
- Optional: add more targeted integration tests under `tests/`.
