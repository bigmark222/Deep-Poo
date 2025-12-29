# colon_sim_app

Reference app crate built on the substrate crates. It holds domain-specific systems (world/entities, HUD, controls/autopilot) and recorder world-state updates.

Integration points:
- Systems: wired via `AppSystemsPlugin`; bins add this alongside `sim_core::SimPlugin`/`SimRuntimePlugin` and vision/inference plugins as needed.
- Recorder: update `sim_core::recorder_meta::RecorderWorldState` (e.g., head_z, stop flag) via your app systems; the substrate recorder installs a default sink (`JsonRecorder`) but you can inject custom sinks.

Note: Keep core crates detector-free and domain-agnostic; app crates supply the concrete systems and world logic.
