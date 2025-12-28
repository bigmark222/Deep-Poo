pub mod interfaces;
pub mod overlay;

mod vision_runtime;

pub use vision_runtime::{
    BurnDetector, BurnInferenceState, DefaultDetectorFactory, DetectionOverlayState, DetectorFactory,
    DetectorHandle, DetectorKind, FrontCamera, FrontCameraFrame, FrontCameraFrameBuffer,
    FrontCameraState, FrontCaptureCamera, FrontCaptureReadback, FrontCaptureTarget,
    InferenceThresholds, CapturePlugin, InferencePlugin, capture_front_camera_frame,
    on_front_capture_readback, poll_burn_inference, schedule_burn_inference, setup_front_capture,
    threshold_hotkeys, track_front_camera_state,
};
pub use ::vision_core::prelude::CaptureLimit;

pub mod prelude {
    pub use ::vision_core::prelude::{
        CaptureLimit, Detector, Frame, FrameRecord, Label, DetectionResult, FrameSource, Recorder,
        draw_rect, normalize_box,
    };
    pub use crate::vision::vision_runtime::{
        CapturePlugin, InferencePlugin, DefaultDetectorFactory, DetectorFactory, DetectorHandle,
        DetectorKind, InferenceThresholds,
    };
}
