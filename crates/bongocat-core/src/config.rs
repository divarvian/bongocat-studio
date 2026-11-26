use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const DEFAULT_PORT: u16 = 17890;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to determine configuration directory")]
    NoConfigDir,
    #[error("I/O error accessing config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse TOML configuration: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("failed to serialize TOML configuration: {0}")]
    Serialize(#[from] toml::ser::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DisplayMode {
    #[default]
    Slanted,
    Flat,
    Classic,
}

impl DisplayMode {
    pub const ALL: [Self; 3] = [Self::Slanted, Self::Flat, Self::Classic];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Slanted => "Slanted (~10°)",
            Self::Flat => "Flat Desk",
            Self::Classic => "Classic Bongo",
        }
    }

    pub const fn short_label(self) -> &'static str {
        match self {
            Self::Slanted => "Slanted",
            Self::Flat => "Flat",
            Self::Classic => "Classic",
        }
    }

    pub const fn recommended_obs_size(self) -> (u32, u32) {
        match self {
            Self::Slanted => (640, 450),
            Self::Flat => (640, 430),
            Self::Classic => (420, 260),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum PointerDevice {
    #[default]
    Trackpad,
    Mouse,
}

impl PointerDevice {
    pub const ALL: [Self; 2] = [Self::Trackpad, Self::Mouse];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Trackpad => "Laptop Trackpad",
            Self::Mouse => "Mouse + Pad",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DeskTheme {
    #[default]
    White,
    Dark,
    Pastel,
    Transparent,
}

impl DeskTheme {
    pub const ALL: [Self; 4] = [Self::White, Self::Dark, Self::Pastel, Self::Transparent];

    pub const fn label(self) -> &'static str {
        match self {
            Self::White => "White",
            Self::Dark => "Dark",
            Self::Pastel => "Pastel",
            Self::Transparent => "Transparent",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum EyeStyle {
    #[default]
    Classic,
    Expressive,
}

impl EyeStyle {
    pub const ALL: [Self; 2] = [Self::Classic, Self::Expressive];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Classic => "Classic Dots",
            Self::Expressive => "Expressive",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum EyeDirection {
    #[default]
    Screen,
    Mirror,
}

impl EyeDirection {
    pub const ALL: [Self; 2] = [Self::Screen, Self::Mirror];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Screen => "Follow Cursor",
            Self::Mirror => "Mirror (Cat POV)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum PointerDirection {
    #[default]
    Natural,
    Screen,
}

impl PointerDirection {
    pub const ALL: [Self; 2] = [Self::Natural, Self::Screen];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Natural => "Cat POV (Natural)",
            Self::Screen => "Match Screen",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum RightPawMode {
    #[default]
    Auto,
    Pointer,
}

impl RightPawMode {
    pub const ALL: [Self; 2] = [Self::Auto, Self::Pointer];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Auto => "Auto (Type + Pointer)",
            Self::Pointer => "Stay on Pointer",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum PrivacyMode {
    #[default]
    Full,
    Gaming,
    Zones,
}

impl PrivacyMode {
    pub const ALL: [Self; 3] = [Self::Full, Self::Gaming, Self::Zones];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Full => "Full (63 Keys)",
            Self::Gaming => "Gaming (WASD Only)",
            Self::Zones => "Zones (Private)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum AppTheme {
    #[default]
    Dark,
    Light,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Preset {
    Coding,
    Gaming,
    Minimal,
}

impl Preset {
    pub const ALL: [Self; 3] = [Self::Coding, Self::Gaming, Self::Minimal];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Coding => "Coding",
            Self::Gaming => "Gaming",
            Self::Minimal => "Minimal",
        }
    }

    pub const fn overlay(self) -> OverlayConfig {
        match self {
            Self::Coding => OverlayConfig {
                mode: DisplayMode::Slanted,
                pointer: PointerDevice::Trackpad,
                desk_theme: DeskTheme::White,
                eye_style: EyeStyle::Classic,
                eye_dir: EyeDirection::Screen,
                ptr_dir: PointerDirection::Natural,
                right_paw: RightPawMode::Auto,
                privacy_mode: PrivacyMode::Full,
                eyes_enabled: true,
                show_hud: true,
                show_kps: true,
                bounce: true,
                key_hold_ms: 140,
                idle_raise_sec: 4,
                sleep_sec: 60,
                kps_happy_threshold: 5,
            },
            Self::Gaming => OverlayConfig {
                mode: DisplayMode::Slanted,
                pointer: PointerDevice::Mouse,
                desk_theme: DeskTheme::Dark,
                eye_style: EyeStyle::Expressive,
                eye_dir: EyeDirection::Screen,
                ptr_dir: PointerDirection::Natural,
                right_paw: RightPawMode::Pointer,
                privacy_mode: PrivacyMode::Gaming,
                eyes_enabled: true,
                show_hud: true,
                show_kps: true,
                bounce: true,
                key_hold_ms: 120,
                idle_raise_sec: 4,
                sleep_sec: 90,
                kps_happy_threshold: 6,
            },
            Self::Minimal => OverlayConfig {
                mode: DisplayMode::Classic,
                pointer: PointerDevice::Trackpad,
                desk_theme: DeskTheme::Transparent,
                eye_style: EyeStyle::Classic,
                eye_dir: EyeDirection::Screen,
                ptr_dir: PointerDirection::Natural,
                right_paw: RightPawMode::Auto,
                privacy_mode: PrivacyMode::Zones,
                eyes_enabled: false,
                show_hud: false,
                show_kps: true,
                bounce: true,
                key_hold_ms: 130,
                idle_raise_sec: 4,
                sleep_sec: 60,
                kps_happy_threshold: 5,
            },
        }
    }
}

/// Visual & behavioral configuration shared live between Desktop Preview and OBS Browser Source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct OverlayConfig {
    pub mode: DisplayMode,
    pub pointer: PointerDevice,
    pub desk_theme: DeskTheme,
    pub eye_style: EyeStyle,
    pub eye_dir: EyeDirection,
    pub ptr_dir: PointerDirection,
    pub right_paw: RightPawMode,
    pub privacy_mode: PrivacyMode,
    pub eyes_enabled: bool,
    pub show_hud: bool,
    pub show_kps: bool,
    pub bounce: bool,
    pub key_hold_ms: u32,
    pub idle_raise_sec: u32,
    pub sleep_sec: u32,
    pub kps_happy_threshold: u32,
}

impl Default for OverlayConfig {
    fn default() -> Self {
        Preset::Coding.overlay()
    }
}

impl OverlayConfig {
    pub fn clamp(&mut self) {
        self.key_hold_ms = self.key_hold_ms.clamp(50, 400);
        self.idle_raise_sec = self.idle_raise_sec.clamp(1, 15);
        self.sleep_sec = self.sleep_sec.clamp(0, 600);
        self.kps_happy_threshold = self.kps_happy_threshold.clamp(2, 20);
    }
}

/// Complete persisted application settings stored at `~/.config/bongocat-studio/config.toml`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub overlay: OverlayConfig,
    pub app_theme: AppTheme,
    pub port: u16,
    pub close_to_background: bool,
    pub autostart: bool,
    pub show_checkerboard: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            overlay: OverlayConfig::default(),
            app_theme: AppTheme::Dark,
            port: DEFAULT_PORT,
            close_to_background: true,
            autostart: false,
            show_checkerboard: true,
        }
    }
}

impl AppConfig {
    pub fn clamp(&mut self) {
        self.overlay.clamp();
        if self.port < 1024 {
            self.port = DEFAULT_PORT;
        }
    }

    pub fn apply_preset(&mut self, preset: Preset) {
        self.overlay = preset.overlay();
    }

    pub fn active_preset(&self) -> Option<Preset> {
        Preset::ALL
            .into_iter()
            .find(|preset| self.overlay == preset.overlay())
    }

    pub fn obs_url(&self) -> String {
        format!("http://127.0.0.1:{}/", self.port)
    }

    pub fn default_path() -> Result<PathBuf, ConfigError> {
        let base = dirs::config_dir().ok_or(ConfigError::NoConfigDir)?;
        Ok(base.join("bongocat-studio").join("config.toml"))
    }

    pub fn load_default() -> Result<Self, ConfigError> {
        let path = Self::default_path()?;
        Self::load_from(&path)
    }

    pub fn load_from(path: &Path) -> Result<Self, ConfigError> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = fs::read_to_string(path)?;
        let mut cfg: Self = toml::from_str(&raw)?;
        cfg.clamp();
        Ok(cfg)
    }

    pub fn save_default(&self) -> Result<(), ConfigError> {
        let path = Self::default_path()?;
        self.save_to(&path)
    }

    pub fn save_to(&self, path: &Path) -> Result<(), ConfigError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut normalized = self.clone();
        normalized.clamp();
        let body = toml::to_string_pretty(&normalized)?;

        // Atomic write via sibling temp file + rename so config is never half-written.
        let tmp_path = path.with_extension("toml.tmp");
        {
            let mut file = fs::File::create(&tmp_path)?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = file.set_permissions(fs::Permissions::from_mode(0o600));
            }
            file.write_all(body.as_bytes())?;
            file.sync_all()?;
        }
        fs::rename(&tmp_path, path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_matches_coding_preset() {
        let cfg = AppConfig::default();
        assert_eq!(cfg.active_preset(), Some(Preset::Coding));
        assert_eq!(cfg.overlay.mode, DisplayMode::Slanted);
        assert_eq!(cfg.overlay.pointer, PointerDevice::Trackpad);
        assert_eq!(cfg.obs_url(), "http://127.0.0.1:17890/");
    }

    #[test]
    fn apply_preset_updates_active_preset() {
        let mut cfg = AppConfig::default();
        cfg.apply_preset(Preset::Gaming);
        assert_eq!(cfg.active_preset(), Some(Preset::Gaming));
        assert_eq!(cfg.overlay.pointer, PointerDevice::Mouse);
        assert_eq!(cfg.overlay.privacy_mode, PrivacyMode::Gaming);

        cfg.overlay.bounce = false;
        assert_eq!(cfg.active_preset(), None);
    }

    #[test]
    fn atomic_toml_round_trip() {
        let dir = std::env::temp_dir().join("bongocat-studio-core-test");
        let _ = fs::remove_dir_all(&dir);
        let path = dir.join("config.toml");

        let mut cfg = AppConfig::default();
        cfg.apply_preset(Preset::Minimal);
        cfg.app_theme = AppTheme::Light;
        cfg.port = 18000;
        cfg.save_to(&path).expect("save config");

        let loaded = AppConfig::load_from(&path).expect("load config");
        assert_eq!(loaded, cfg);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn clamps_invalid_values() {
        let mut cfg = AppConfig {
            port: 80,
            overlay: OverlayConfig {
                key_hold_ms: 5,
                kps_happy_threshold: 99,
                ..OverlayConfig::default()
            },
            ..AppConfig::default()
        };
        cfg.clamp();
        assert_eq!(cfg.port, DEFAULT_PORT);
        assert_eq!(cfg.overlay.key_hold_ms, 50);
        assert_eq!(cfg.overlay.kps_happy_threshold, 20);
    }
}
