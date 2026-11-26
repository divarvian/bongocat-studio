use serde::{Deserialize, Serialize};

use crate::config::OverlayConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum MouseButton {
    Left = 1,
    Right = 2,
    Middle = 3,
}

impl MouseButton {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::Left),
            2 => Some(Self::Right),
            3 => Some(Self::Middle),
            _ => None,
        }
    }

    pub const fn hud_label(self) -> &'static str {
        match self {
            Self::Left => "L-Click",
            Self::Right => "R-Click",
            Self::Middle => "M-Click",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceKind {
    Keyboard,
    Mouse,
    Touchpad,
    Combo,
}

impl DeviceKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Keyboard => "Keyboard",
            Self::Mouse => "Mouse",
            Self::Touchpad => "Touchpad",
            Self::Combo => "Combo",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub name: String,
    pub kind: DeviceKind,
}

/// Sanitized local input event flowing from `bongocat-input` to the Desktop Preview & OBS Server.
/// Raw keycodes are never logged anywhere in the application.
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    Key {
        code: u16,
        down: bool,
    },
    PointerMove {
        x: f32,
        y: f32,
        touch: bool,
        fingers: u8,
    },
    PointerButton {
        button: MouseButton,
        down: bool,
    },
    Wheel {
        delta: i8,
    },
    Cursor {
        x: f32,
        y: f32,
    },
    DevicesUpdated(Vec<DeviceInfo>),
    InputError(String),
}

/// Compact SSE payload transmitted exclusively over `127.0.0.1` to OBS Browser Source clients.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t")]
pub enum WireEvent {
    #[serde(rename = "k")]
    Key { c: u16, d: bool },
    #[serde(rename = "m")]
    Move {
        x: f32,
        y: f32,
        #[serde(rename = "tc")]
        touch: bool,
        #[serde(rename = "f")]
        fingers: u8,
    },
    #[serde(rename = "b")]
    Button { b: u8, d: bool },
    #[serde(rename = "w")]
    Wheel { d: i8 },
    #[serde(rename = "c")]
    Cursor { x: f32, y: f32 },
    #[serde(rename = "config")]
    Config { config: Box<OverlayConfig> },
    #[serde(rename = "privacy")]
    Privacy { paused: bool },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerStatus {
    pub ok: bool,
    pub port: u16,
    pub clients: usize,
    pub devices: Vec<DeviceInfo>,
    pub privacy_paused: bool,
    pub input_error: Option<String>,
}
