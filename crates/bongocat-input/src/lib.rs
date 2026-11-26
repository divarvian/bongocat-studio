//! Pure-Rust Linux `evdev` input discovery, clickpad gesture processing, and
//! privacy-safe event reader.

pub mod clickpad;
pub mod discovery;
pub mod reader;

pub use clickpad::{AbsRange, PointerProcessor};
pub use discovery::{DiscoveredDevice, scan_input_devices};
pub use reader::{InputHandle, spawn_input_reader};
