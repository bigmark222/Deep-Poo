use bevy::prelude::*;

use crate::balloon_control::BalloonControl;
use crate::polyp::PolypRemoval;
use crate::probe::StretchState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoStage {
    AnchorTail,
    Extend,
    AnchorHead,
    ReleaseTail,
    Contract,
    ReleaseHead,
}

#[derive(Resource)]
pub struct AutoDrive {
    pub enabled: bool,
    pub stage: AutoStage,
    pub timer: f32,
    pub extend: bool,
    pub retract: bool,
}

impl Default for AutoDrive {
    fn default() -> Self {
        Self {
            enabled: false,
            stage: AutoStage::AnchorTail,
            timer: 0.0,
            extend: false,
            retract: false,
        }
    }
}

pub fn auto_toggle(keys: Res<ButtonInput<KeyCode>>, mut auto: ResMut<AutoDrive>) {
    if keys.just_pressed(KeyCode::KeyP) {
        auto.enabled = !auto.enabled;
        auto.stage = AutoStage::AnchorTail;
        auto.timer = 0.2;
        auto.extend = false;
        auto.retract = false;
    }
}

pub fn auto_inchworm(
    time: Res<Time>,
    removal: Res<PolypRemoval>,
    mut auto: ResMut<AutoDrive>,
    mut balloon: ResMut<BalloonControl>,
    stretch: Res<StretchState>,
) {
    // If autopilot off, do nothing.
    if !auto.enabled {
        auto.extend = false;
        auto.retract = false;
        return;
    }

    // Pause during removal dwell.
    if removal.in_progress {
        auto.extend = false;
        auto.retract = false;
        return;
    }

    auto.timer += time.delta_secs();
    auto.extend = false;
    auto.retract = false;

    match auto.stage {
        AutoStage::AnchorTail => {
            balloon.tail_inflated = true;
            balloon.head_inflated = false;
            if auto.timer > 0.1 {
                auto.stage = AutoStage::Extend;
                auto.timer = 0.0;
            }
        }
        AutoStage::Extend => {
            balloon.tail_inflated = true;
            balloon.head_inflated = false;
            auto.extend = true;
            if stretch.factor >= crate::probe::MAX_STRETCH - 0.02 {
                auto.stage = AutoStage::AnchorHead;
                auto.timer = 0.0;
            }
        }
        AutoStage::AnchorHead => {
            balloon.tail_inflated = true;
            balloon.head_inflated = true;
            if auto.timer > 0.2 {
                auto.stage = AutoStage::ReleaseTail;
                auto.timer = 0.0;
            }
        }
        AutoStage::ReleaseTail => {
            balloon.tail_inflated = false;
            balloon.head_inflated = true;
            if auto.timer > 0.1 {
                auto.stage = AutoStage::Contract;
                auto.timer = 0.0;
            }
        }
        AutoStage::Contract => {
            balloon.tail_inflated = false;
            balloon.head_inflated = true;
            auto.retract = true;
            if stretch.factor <= crate::probe::MIN_STRETCH + 0.02 {
                auto.stage = AutoStage::ReleaseHead;
                auto.timer = 0.0;
            }
        }
        AutoStage::ReleaseHead => {
            balloon.tail_inflated = false;
            balloon.head_inflated = false;
            if auto.timer > 0.1 {
                auto.stage = AutoStage::AnchorTail;
                auto.timer = 0.0;
            }
        }
    }
}
