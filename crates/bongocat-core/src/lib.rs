//! Core domain types, configuration, ergonomic 63-key mapping, perspective geometry,
//! and deterministic animation state for `BongoCat Studio`.

pub mod config;
pub mod event;
pub mod geometry;
pub mod keymap;
pub mod state;

pub use config::{
    AppConfig, AppTheme, ConfigError, DeskTheme, DisplayMode, EyeDirection, EyeStyle,
    OverlayConfig, PointerDevice, PointerDirection, Preset, PrivacyMode, RightPawMode,
};
pub use event::{DeviceInfo, DeviceKind, InputEvent, MouseButton, ServerStatus, WireEvent};
pub use geometry::{ArmSpline, Pt, Quad};
pub use keymap::{KeyHand, KeySpec, ResolvedKey};
pub use state::{CatState, ClassicFrame, HudPill, HudPillKind, PawPose};
