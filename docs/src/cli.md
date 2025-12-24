# CLI usage

The simulator ships multiple binaries (interactive, headless data-gen, overlay tool) driven by a Clap CLI. This page lists every flag, defaults, and ready-to-run examples.

## Flags (all binaries)
- `--mode <sim|datagen>`: run interactively (`sim`, default) or headless data generation (`datagen`).
- `--seed <u64>`: optional seed override for reproducible polyp layouts; default is time-based.
- `--output-root <path>`: root for run folders. Default: `assets/datasets/captures`.
- `--max-frames <N>`: optional frame cap for data runs (stop after N frames).
- `--headless`: hide the main window / offscreen rendering (for datagen).

## Binaries
- `sim_view`: interactive/visible sim (also usable for visible datagen with `--mode datagen`).
- `datagen_headless`: headless data-gen runner.
- `overlay_labels`: draw bounding boxes onto captured frames.

## Runtime hotkeys (vision)
- `-`/`=`: decrease/increase objectness threshold.
- `[`/`]`: decrease/increase IoU threshold.
- `B`: toggle between Burn and heuristic detectors; HUD shows the active mode/box stats.
- Burn checkpoint: place model at `checkpoints/tinydet.bin` (runtime loads automatically). If missing or load fails, sim falls back to the heuristic detector and shows a fallback banner in the HUD.

## Command gallery (covers every flag)
1) **Interactive sim (defaults)**
   - Command: `cargo run --release --bin sim_view`
   - Flags: none
     - mode=`sim`, time-based seed, output root `assets/datasets/captures`
     - Recording is manual in this mode:
       - `C` — toggle camera; switch to probe POV until HUD shows `VISION :: cam=ON` (needed for POV captures).
       - `L` — start/stop recording manually; HUD shows `REC :: on` and frames/labels are written under `assets/datasets/captures/run_<timestamp>/`.
       - `O` — data-run shortcut: enables autopilot + probe POV; recording auto-starts after ~8s and auto-stops at tunnel end (return leg not recorded).

2) **Interactive sim with fixed seed**
   - Command: `cargo run --release --bin sim_view -- --seed 1234`
   - Flags: `--seed`

3) **Interactive datagen (visible) with frame cap**
   - Command: `cargo run --release --bin sim_view -- --mode datagen --max-frames 500`
   - Flags: `--mode datagen`, `--max-frames`
     - Recording: same hotkeys as sim (`C` POV toggle, `L` start/stop, `O` auto-run with auto-stop).

4) **Headless datagen with custom output + seed**
   - Command: `cargo run --release --bin datagen_headless -- --seed 42 --output-root /tmp/runs --max-frames 600`
   - Flags: `--headless` (implied by binary), `--seed`, `--output-root`, `--max-frames`
     - Recording: automatic in headless datagen; frames/labels written under the specified output root.

5) **Headless datagen using default output root**
   - Command: `cargo run --release --bin datagen_headless -- --mode datagen`
   - Flags: `--mode datagen` (explicit), other flags default
     - Recording: automatic in headless datagen; output under `assets/datasets/captures`.

6) **Headless datagen with explicit headless flag**
   - Command: `cargo run --release --bin sim_view -- --mode datagen --headless --max-frames 300`
   - Flags: `--mode datagen`, `--headless`, `--max-frames`
     - Recording: automatic in headless/datagen mode; respect frame cap.

7) **Overlay previously captured run**
   - Command: `cargo run --release --bin overlay_labels -- assets/datasets/captures/run_1234567890123`
   - Flags: positional path to run directory (no additional flags)
     - Output: writes overlays under `<run>/overlays` (or optional custom output dir).

8) **Run with alternate output root (visible sim)**
   - Command: `cargo run --release --bin sim_view -- --output-root /tmp/captures`
   - Flags: `--output-root`
     - Recording: manual via `C`/`L`/`O`; writes under the custom root.

9) **Headless datagen with only a frame cap**
   - Command: `cargo run --release --bin datagen_headless -- --max-frames 1000`
   - Flags: `--max-frames`
      - Recording: automatic in headless; stops at cap.

10) **Visible datagen with max frames and seed**
    - Command: `cargo run --release --bin sim_view -- --mode datagen --seed 9876 --max-frames 750`
    - Flags: `--mode datagen`, `--seed`, `--max-frames`
      - Recording: manual hotkeys; respects frame cap if recording is on.

11) **Headless datagen writing to default root (short form)**
    - Command: `cargo run --release --bin datagen_headless`
    - Flags: none (binary is headless; mode defaults to `sim` but headless path is implied)
     - Recording: automatic to `assets/datasets/captures` with time-based seed.

12) **Headless datagen with custom output and headless flag (redundant but explicit)**
    - Command: `cargo run --release --bin datagen_headless -- --output-root /data/runs --headless`
    - Flags: `--output-root`, `--headless`
     - Recording: automatic to `/data/runs` with time-based seed unless `--seed` is provided.
