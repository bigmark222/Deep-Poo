use bevy::prelude::Resource;

#[derive(Resource, Default, Clone)]
pub struct CaptureLimit {
    pub max_frames: Option<u32>,
}
