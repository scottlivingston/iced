//! Build gesture events.
use crate::Point;

/// A trackpad/touchscreen gesture.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// A pinch gesture (two-finger zoom on trackpad).
    Pinch {
        /// The phase of the gesture.
        phase: Phase,
        /// The magnification delta; positive for zoom in, negative for zoom out.
        delta: f32,
    },

    /// A pan gesture (multi-finger drag).
    Pan {
        /// The phase of the gesture.
        phase: Phase,
        /// The change in position since the last event.
        delta: Point,
    },

    /// A rotation gesture (two-finger rotation on trackpad).
    Rotation {
        /// The phase of the gesture.
        phase: Phase,
        /// The change in rotation in degrees.
        delta: f32,
    },

    /// A double-tap gesture on the trackpad (smart zoom).
    DoubleTap,
}

/// The phase of an ongoing gesture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// The gesture has started.
    Started,
    /// The gesture parameters have changed.
    Changed,
    /// The gesture has ended.
    Ended,
    /// The system cancelled the gesture.
    Cancelled,
}
