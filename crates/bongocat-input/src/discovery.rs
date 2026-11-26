use std::fmt;
use std::fs;
use std::io::ErrorKind;
use std::os::unix::fs::FileTypeExt;
use std::path::{Path, PathBuf};

use bongocat_core::{DeviceInfo, DeviceKind};
use evdev::{AbsoluteAxisType, Device, Key, RelativeAxisType};

use crate::clickpad::AbsRange;

pub struct DiscoveredDevice {
    pub path: PathBuf,
    pub info: DeviceInfo,
    pub device: Device,
    pub is_keyboard: bool,
    pub is_mouse: bool,
    pub is_touchpad: bool,
    pub has_dedicated_right_btn: bool,
    pub abs_x: Option<AbsRange>,
    pub abs_y: Option<AbsRange>,
}

impl fmt::Debug for DiscoveredDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DiscoveredDevice")
            .field("path", &self.path)
            .field("info", &self.info)
            .field("is_keyboard", &self.is_keyboard)
            .field("is_mouse", &self.is_mouse)
            .field("is_touchpad", &self.is_touchpad)
            .field("has_dedicated_right_btn", &self.has_dedicated_right_btn)
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
pub struct ScanResult {
    pub devices: Vec<DiscoveredDevice>,
    pub permission_denied: bool,
}

/// Validates that `name` strictly matches `event<digits>` to prevent path traversal or
/// opening unrelated device nodes outside `/dev/input/`.
pub fn is_valid_event_filename(name: &str) -> bool {
    let Some(suffix) = name.strip_prefix("event") else {
        return false;
    };
    !suffix.is_empty() && suffix.bytes().all(|b| b.is_ascii_digit())
}

pub fn scan_input_devices() -> ScanResult {
    let input_dir = Path::new("/dev/input");
    let Ok(entries) = fs::read_dir(input_dir) else {
        return ScanResult {
            devices: Vec::new(),
            permission_denied: false,
        };
    };

    let mut paths: Vec<PathBuf> = entries
        .flatten()
        .filter_map(|entry| {
            let file_name = entry.file_name();
            let name = file_name.to_str()?;
            if !is_valid_event_filename(name) {
                return None;
            }
            let path = entry.path();
            let meta = fs::symlink_metadata(&path).ok()?;
            if meta.file_type().is_char_device() {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    paths.sort();

    let mut devices = Vec::new();
    let mut permission_denied = false;

    for path in paths {
        match Device::open(&path) {
            Ok(device) => {
                if let Some(discovered) = classify_device(path, device) {
                    devices.push(discovered);
                }
            }
            Err(err) if err.kind() == ErrorKind::PermissionDenied => {
                permission_denied = true;
            }
            Err(_) => {}
        }
    }

    ScanResult {
        devices,
        permission_denied,
    }
}

fn classify_device(path: PathBuf, device: Device) -> Option<DiscoveredDevice> {
    let keys = device.supported_keys();
    let rels = device.supported_relative_axes();
    let abs = device.supported_absolute_axes();

    let is_keyboard = keys.is_some_and(|k| {
        k.contains(Key::KEY_A) && k.contains(Key::KEY_Z) && k.contains(Key::KEY_SPACE)
    });

    let has_left_btn = keys.is_some_and(|k| k.contains(Key::BTN_LEFT));
    let has_dedicated_right_btn = keys.is_some_and(|k| k.contains(Key::BTN_RIGHT));
    let has_touch =
        keys.is_some_and(|k| k.contains(Key::BTN_TOUCH) || k.contains(Key::BTN_TOOL_FINGER));

    let has_abs_xy = abs.is_some_and(|a| {
        (a.contains(AbsoluteAxisType::ABS_X) && a.contains(AbsoluteAxisType::ABS_Y))
            || (a.contains(AbsoluteAxisType::ABS_MT_POSITION_X)
                && a.contains(AbsoluteAxisType::ABS_MT_POSITION_Y))
    });

    let has_rel_xy = rels.is_some_and(|r| {
        r.contains(RelativeAxisType::REL_X) && r.contains(RelativeAxisType::REL_Y)
    });

    let is_touchpad = has_abs_xy && has_touch;
    let is_mouse = !is_touchpad && has_rel_xy && has_left_btn;

    if !is_keyboard && !is_mouse && !is_touchpad {
        return None;
    }

    let kind = match (is_keyboard, is_touchpad, is_mouse) {
        (true, true, _) | (true, _, true) => DeviceKind::Combo,
        (true, false, false) => DeviceKind::Keyboard,
        (false, true, _) => DeviceKind::Touchpad,
        (false, false, true) => DeviceKind::Mouse,
        (false, false, false) => unreachable!(),
    };

    let (abs_x, abs_y) = if is_touchpad {
        read_abs_ranges(&device)
    } else {
        (None, None)
    };

    let name = device.name().unwrap_or("Input Device").trim().to_owned();

    Some(DiscoveredDevice {
        path,
        info: DeviceInfo { name, kind },
        device,
        is_keyboard,
        is_mouse,
        is_touchpad,
        has_dedicated_right_btn,
        abs_x,
        abs_y,
    })
}

fn read_abs_ranges(device: &Device) -> (Option<AbsRange>, Option<AbsRange>) {
    let Ok(state) = device.get_abs_state() else {
        return (None, None);
    };

    let extract = |axis: AbsoluteAxisType| -> Option<AbsRange> {
        let info = state.get(axis.0 as usize)?;
        if info.maximum > info.minimum {
            Some(AbsRange::new(info.minimum, info.maximum))
        } else {
            None
        }
    };

    let x =
        extract(AbsoluteAxisType::ABS_MT_POSITION_X).or_else(|| extract(AbsoluteAxisType::ABS_X));
    let y =
        extract(AbsoluteAxisType::ABS_MT_POSITION_Y).or_else(|| extract(AbsoluteAxisType::ABS_Y));
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_strict_event_filenames() {
        assert!(is_valid_event_filename("event0"));
        assert!(is_valid_event_filename("event14"));
        assert!(!is_valid_event_filename("event"));
        assert!(!is_valid_event_filename("event1a"));
        assert!(!is_valid_event_filename("../event0"));
        assert!(!is_valid_event_filename("mice"));
        assert!(!is_valid_event_filename("mouse0"));
    }
}
