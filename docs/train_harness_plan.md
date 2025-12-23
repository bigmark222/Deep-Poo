Recommended steps to flesh out the training harness:

1) Reproducible data pipeline
- Add a seed field to `DatasetConfig`/`BatchIter` and use it for shuffling/splits.
- Keep val split without augmentation.

2) Training loop skeleton
- Extend `train.rs` to iterate epochs and batches.
- Add AdamW optimizer (fixed LR initially) and apply optimizer steps each batch.
- Log loss/IoU every N steps.

3) Validation pass
- Add a val `BatchIter` (no aug); per epoch, compute mean IoU on val and log it.

4) Checkpointing
- Save model/optimizer state every M steps/epochs to `checkpoints/`.
- Load the last checkpoint if present.

5) Config polish
- Pull batch size, epochs, LR, checkpoint dir, seed, train/val ratios from a small CLI/config.

6) Optional next
- Add LR decay and optional early stop on val metric.
