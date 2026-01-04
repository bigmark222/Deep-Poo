# colon_sim app (Deep Poo)

This repo now focuses on the colon simulation reference app and its binaries. It depends on the published `cortenforge-*` crates from crates.io for the substrate (simulation, vision, inference, capture).

## What’s here
- App crate: `crates/colon_sim`
- Binaries: `apps/colon_sim/bin/sim_view.rs`, `apps/colon_sim/bin/inference_view.rs`
- Key dependencies from crates.io: `cortenforge-sim-core`, `cortenforge-vision-core`, `cortenforge-vision-runtime`, `cortenforge-data-contracts`, `cortenforge-inference`, `cortenforge-capture-utils`, `cortenforge-models`, `cortenforge-burn-dataset`

## Quick start
- Interactive sim: `cargo run --bin sim_view --locked`
- Inference viewer: `cargo run --bin inference_view --locked` (pass detector weights as needed)
- Headless datagen smoke: `cargo run --bin datagen_headless --locked -- --max-frames 3 --headless --output-root /tmp/captures`

## Testing
- Smoke build against registry crates: `cargo test --locked -p colon_sim --test registry_smoke`

## License
- Apache-2.0 by default; see `LICENSE` and `COMMERCIAL_LICENSE.md`.
