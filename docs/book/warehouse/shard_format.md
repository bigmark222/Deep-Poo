# Shard format and checksums

## Shard structure (conceptual)
- Header: magic (`TWH1`), shard version (1), dtype (`f32`), channels (3), target size (HxW), counts/offsets for images/boxes/masks.
- Payload: contiguous regions for images (CHW), padded boxes, masks (if present).
- Default shard size target: ~128–256 MB per shard (tunable via `--shard-samples`).

## Checksums
- Per-shard SHA256 stored in the manifest; recompute and compare before use.
- Consider embedding a header checksum if shard format evolves (future-proofing).

## Notes for implementers
- Maintain little-endian encoding for portability.
- Preserve alignment/padding rules if you adjust header fields.
- Keep dtype/channel order stable (`f32`, CHW) unless you plan a format version bump.
