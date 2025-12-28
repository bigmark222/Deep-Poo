//! vision_core: shared detector/capture/overlay interfaces.

pub mod interfaces;
pub mod overlay;
pub mod capture;

pub mod prelude {
    pub use crate::interfaces::*;
    pub use crate::overlay::*;
    pub use crate::capture::CaptureLimit;
}
