use bevy::prelude::*;
use bevy_rapier3d::prelude::PhysicsSet;

use sim_core::ModeSet;

use crate::balloon_control::{
    balloon_body_update, balloon_control_input, balloon_marker_update, spawn_balloon_body,
    spawn_balloon_marker,
};
use crate::hud::{update_controls_ui, spawn_controls_ui, spawn_detection_overlay};
use crate::polyp::{apply_detection_votes, polyp_detection_system, polyp_removal_system, spawn_polyps};
use crate::probe::{distributed_thrust, peristaltic_drive, spawn_probe};
use crate::sim::autopilot::{auto_inchworm, auto_toggle, data_run_toggle, datagen_autostart};
use crate::sim::recorder::{
    auto_start_recording, auto_stop_recording_on_cecum, datagen_failsafe_recording,
    finalize_datagen_run, record_front_camera_metadata, recorder_toggle_hotkey,
};
use crate::tunnel::{
    cecum_detection, setup_tunnel, start_detection, tunnel_expansion_system,
};
use crate::vision::{CapturePlugin, poll_burn_inference};
use sim_core::camera::{camera_controller, pov_toggle_system, setup_camera};
use sim_core::controls::control_inputs_and_apply;

/// Plugin that wires the existing autopilot/recorder and sim systems into ModeSet schedules.
pub struct SimSystemsPlugin;

impl Plugin for SimSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (ModeSet::Common, ModeSet::SimDatagen, ModeSet::Inference))
            .add_systems(
                Startup,
                (
                    setup_camera,
                    setup_tunnel,
                    spawn_probe,
                    spawn_balloon_body,
                    spawn_balloon_marker,
                    spawn_polyps,
                    spawn_controls_ui,
                    spawn_detection_overlay,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    balloon_control_input,
                    balloon_body_update,
                    camera_controller,
                    pov_toggle_system,
                    apply_detection_votes
                        .after(polyp_detection_system)
                        .after(poll_burn_inference),
                )
                    .in_set(ModeSet::Common),
            )
            .add_systems(
                Update,
                (
                    datagen_autostart,
                    data_run_toggle,
                    auto_toggle,
                    auto_inchworm,
                    balloon_marker_update,
                    recorder_toggle_hotkey,
                    auto_start_recording,
                    auto_stop_recording_on_cecum,
                    finalize_datagen_run.after(auto_stop_recording_on_cecum),
                    datagen_failsafe_recording,
                    record_front_camera_metadata,
                    control_inputs_and_apply,
                    update_controls_ui,
                    cecum_detection,
                    start_detection,
                    tunnel_expansion_system,
                    polyp_detection_system,
                    polyp_removal_system.after(polyp_detection_system),
                )
                    .in_set(ModeSet::SimDatagen),
            )
            .add_systems(
                FixedUpdate,
                (
                    peristaltic_drive,
                    distributed_thrust.before(PhysicsSet::SyncBackend),
                ),
            )
            .add_plugins(CapturePlugin);
    }
}
