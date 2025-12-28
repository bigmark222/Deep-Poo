use std::path::PathBuf;

use bevy::math::UVec2;
use bevy::prelude::{Resource, Timer, TimerMode};

// Re-export recorder systems that currently live in the vision module.
pub use crate::vision::{
    auto_start_recording, auto_stop_recording_on_cecum, datagen_failsafe_recording,
    finalize_datagen_run, record_front_camera_metadata, recorder_toggle_hotkey,
};

#[derive(Resource)]
pub struct RecorderConfig {
    pub output_root: PathBuf,
    pub capture_interval: Timer,
    pub resolution: UVec2,
    pub prune_empty: bool,
    pub prune_output_root: Option<PathBuf>,
}

impl Default for RecorderConfig {
    fn default() -> Self {
        Self {
            output_root: PathBuf::from("assets/datasets/captures"),
            capture_interval: Timer::from_seconds(0.33, TimerMode::Repeating),
            resolution: UVec2::new(640, 360),
            prune_empty: false,
            prune_output_root: None,
        }
    }
}

#[derive(Resource)]
pub struct RecorderState {
    pub enabled: bool,
    pub session_dir: PathBuf,
    pub frame_idx: u64,
    pub last_toggle: f64,
    pub last_image_ok: bool,
    pub paused: bool,
    pub overlays_done: bool,
    pub prune_done: bool,
    pub initialized: bool,
    pub manifest_written: bool,
}

impl Default for RecorderState {
    fn default() -> Self {
        Self {
            enabled: false,
            session_dir: PathBuf::from("assets/datasets/captures/unsynced"),
            frame_idx: 0,
            last_toggle: 0.0,
            last_image_ok: false,
            paused: false,
            overlays_done: false,
            prune_done: false,
            initialized: false,
            manifest_written: false,
        }
    }
}

#[derive(Resource)]
pub struct AutoRecordTimer {
    pub timer: Timer,
}

impl Default for AutoRecordTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(30.0, TimerMode::Once),
        }
    }
}

#[derive(Resource, Default)]
pub struct RecorderMotion {
    pub last_head_z: Option<f32>,
    pub cumulative_forward: f32,
    pub started: bool,
}
