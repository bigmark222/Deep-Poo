# High-performance training helper for Windows/NVIDIA.
# Sets wgpu env vars to prefer the discrete GPU, then calls the cargo alias from repo root.

param(
    # Optional adapter name (e.g., "NVIDIA", "AMD", "Intel"). If omitted, wgpu will auto-pick.
    [string] $Adapter,
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]] $Args
)

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\\..")
Push-Location $repoRoot
Write-Host "Working dir:" (Get-Location)

$env:WGPU_POWER_PREF = "high-performance"
$env:WGPU_BACKEND = "dx12"
if ($Adapter) {
    $env:WGPU_ADAPTER_NAME = $Adapter
}
Write-Host "WGPU_POWER_PREF=$env:WGPU_POWER_PREF WGPU_BACKEND=$env:WGPU_BACKEND WGPU_ADAPTER_NAME=$env:WGPU_ADAPTER_NAME"

# Ensure logs directory exists for status writes.
if (-not (Test-Path "logs")) {
    New-Item -ItemType Directory -Path "logs" | Out-Null
}

$inputRoot = "assets/datasets/captures_filtered"
if (-not (Test-Path $inputRoot)) {
    Write-Host "Input root not found at '$inputRoot'. Override via '-- --input-root <path>' if needed."
}

cargo train_hp -- @Args

Pop-Location
