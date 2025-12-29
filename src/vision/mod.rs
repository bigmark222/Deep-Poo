pub mod interfaces;
pub mod overlay;

pub use vision_runtime::{
    BurnDetector, BurnDetectionResult, BurnInferenceState, CapturePlugin, DetectorHandle,
    DetectorKind, DetectionOverlayState, FrontCameraFrame, FrontCameraFrameBuffer,
    FrontCameraState, InferencePlugin, InferenceThresholds, recorder_draw_rect,
    schedule_burn_inference, threshold_hotkeys,
};
pub use ::vision_core::prelude::{
    CaptureLimit, FrontCamera, FrontCaptureCamera, FrontCaptureReadback, FrontCaptureTarget,
};

pub mod prelude {
    pub use ::vision_core::prelude::{
        CaptureLimit, Detector, Frame, FrameRecord, Label, DetectionResult, FrameSource, Recorder,
        draw_rect, normalize_box,
    };
    pub use crate::vision::{
        CapturePlugin, DetectorHandle, DetectorKind, DetectionOverlayState, InferencePlugin,
        InferenceThresholds,
    };
    pub use ::vision_core::prelude::{
        FrontCamera, FrontCaptureCamera, FrontCaptureReadback, FrontCaptureTarget,
    };
}
