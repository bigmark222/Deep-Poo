# Setup and fmt/clippy/test expectations

## Prerequisites
- Rust toolchain (stable), `cargo` available.
- mdBook if building docs locally (`cargo install mdbook`), optional `mdbook test`.
- GPU drivers/runtime appropriate for your hardware (DX12/Vulkan).

## Workflow
- Fetch deps: `cargo fetch`
- Format: `cargo fmt --all`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`
- Test: `cargo test --all`
- Docs: `mdbook build` (new book under `docs/new_mdbook`), run `mdbook test` if available.

## Expectations
- No warnings in clippy; keep formatting clean.
- Add/update tests when changing ETL, warehouse layout, or training loaders.
- Update docs alongside interface/config changes.
