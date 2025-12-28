# WGPU env quick reference

Set these env vars before running inference/training tools when you need explicit GPU/backend selection.

- `WGPU_BACKEND`: backend (`dx12` on Windows, `vulkan` on Linux; metal on macOS if applicable).
- `WGPU_ADAPTER_NAME`: GPU adapter name (set when multiple GPUs exist).
- `WGPU_POWER_PREF`: typically `high-performance`.
- `RUST_LOG`: logging level, e.g., `info,wgpu_core=info` or `info,wgpu_core=warn`.

Examples:
- NVIDIA/Windows (DX12): `WGPU_BACKEND=dx12 WGPU_ADAPTER_NAME=NVIDIA WGPU_POWER_PREF=high-performance`
- AMD/Linux (Vulkan): `WGPU_BACKEND=vulkan WGPU_ADAPTER_NAME=AMD WGPU_POWER_PREF=high-performance`

Notes:
- If only one GPU is present, you can omit `WGPU_ADAPTER_NAME`.
- Set explicitly for reproducibility; defaults can vary by platform/driver.
