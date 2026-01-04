# colon_sim app (Deep Poo)

This repo now focuses on the colon simulation reference app and its binaries. It depends on the published `cortenforge-*` crates from crates.io for the substrate (simulation, vision, inference, capture).

## What’s here
- Single app crate at repo root
- Binaries: `src/bin/sim_view.rs`, `src/bin/inference_view.rs`, `src/bin/datagen_headless.rs`
- Key dependencies from crates.io: `cortenforge-sim-core`, `cortenforge-vision-core`, `cortenforge-vision-runtime`, `cortenforge-data-contracts`, `cortenforge-inference`, `cortenforge-capture-utils`, `cortenforge-models`, `cortenforge-burn-dataset`

## Quick start
- Interactive sim: `cargo run --bin sim_view --locked`
- Inference viewer: `cargo run --bin inference_view --locked` (pass detector weights as needed)
- Headless datagen smoke: `cargo run --bin datagen_headless --locked -- --max-frames 3 --headless --output-root /tmp/captures`

## Testing
- Smoke build against registry crates: `cargo test --locked -p colon_sim --test registry_smoke`

## Docs
- See `docs/app_guide.md` for layout, run commands, and local override notes.

## Developing against local crates (optional)
- Use registry crates by default (no patches).
- To point at local clones for debugging, run `scripts/dev-local.sh` to generate `.cargo/config.toml` with `[patch.crates-io]` entries (edit paths if your clones live elsewhere). Remove it with `scripts/dev-local.sh disable`.

## Helpers
- `just` commands: `just fmt`, `just run-sim`, `just run-infer`, `just smoke`, `just smoke-datagen` (add `ARGS='...'` to pass through flags).
 - Toolchain pinned via `rust-toolchain.toml` (Rust 1.91 + clippy + rustfmt).

## License
- Apache-2.0 by default; see `LICENSE` and `COMMERCIAL_LICENSE.md`.
