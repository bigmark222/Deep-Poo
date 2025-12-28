# WGPU configs per vendor

Set backend/adapter env vars before training. Match backend to OS/GPU; set adapter name when multiple GPUs exist.

## NVIDIA (Windows, DX12)
```pwsh
$env:WGPU_POWER_PREF="high-performance"
$env:WGPU_BACKEND="dx12"
$env:WGPU_ADAPTER_NAME="NVIDIA"
$env:RUST_LOG="info,wgpu_core=info"
```

## NVIDIA (Linux, Vulkan)
```bash
WGPU_POWER_PREF=high-performance \
WGPU_BACKEND=vulkan \
WGPU_ADAPTER_NAME=NVIDIA \
RUST_LOG=info,wgpu_core=info
```

## AMD (Windows, DX12)
```pwsh
$env:WGPU_POWER_PREF="high-performance"
$env:WGPU_BACKEND="dx12"
$env:WGPU_ADAPTER_NAME="AMD"
$env:RUST_LOG="info,wgpu_core=info"
```

## AMD (Linux, Vulkan)
```bash
WGPU_POWER_PREF=high-performance \
WGPU_BACKEND=vulkan \
WGPU_ADAPTER_NAME=AMD \
RUST_LOG=info,wgpu_core=info
```

## Notes
- Backend defaults may vary; set explicitly for reproducibility.
- Omit `WGPU_ADAPTER_NAME` only if you have a single GPU; include it to avoid mismatches on multi-GPU systems.
- Keep `RUST_LOG` tuned to your noise tolerance; `info,wgpu_core=info` is a good default.
