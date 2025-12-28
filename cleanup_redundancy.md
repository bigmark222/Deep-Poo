# Cleanup targets (redundancy / mislinks)


- WGPU/env guidance (currently repeated across training, inference, single_infer):
  1) Add a shared snippet/page with WGPU env vars, defaults, and usage.
  2) Update training/inference/single_infer docs to link to the shared snippet and trim duplicate text. ✅
- Gate detector init:
  1) Only construct `DetectorHandle`/insert detection systems when `RunMode::Inference` (skip in sim/datagen). ✅
  2) Verify sim_view runs without model init; inference_view still initializes and runs detection. ✅ (cargo check both bins)
- Verify Inference pages link to the correct overlay doc; Data Capture pages should no longer reference the removed training overlay page. ✅
