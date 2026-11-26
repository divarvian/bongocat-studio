use std::fmt;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use bongocat_core::Preset;
use ksni::blocking::TrayMethods;
use ksni::menu::{CheckmarkItem, MenuItem, StandardItem};
use tokio::sync::mpsc;
use tracing::debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayAction {
    ShowWindow,
    ApplyPreset(Preset),
    TogglePrivacy,
    CopyObsUrl,
    Quit,
}

pub struct TrayHandle {
    inner: Option<ksni::blocking::Handle<BongoTray>>,
    privacy_paused: Arc<AtomicBool>,
}

impl fmt::Debug for TrayHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TrayHandle")
            .field("active", &self.inner.is_some())
            .field(
                "privacy_paused",
                &self.privacy_paused.load(Ordering::Relaxed),
            )
            .finish()
    }
}

impl TrayHandle {
    pub fn set_privacy_paused(&self, paused: bool) {
        self.privacy_paused.store(paused, Ordering::Relaxed);
        if let Some(handle) = &self.inner {
            handle.update(|tray| {
                tray.privacy_paused = paused;
            });
        }
    }

    pub fn shutdown(&mut self) {
        if let Some(handle) = self.inner.take() {
            handle.shutdown();
        }
    }
}

impl Drop for TrayHandle {
    fn drop(&mut self) {
        self.shutdown();
    }
}

struct BongoTray {
    tx: mpsc::UnboundedSender<TrayAction>,
    privacy_paused: bool,
    icon: ksni::Icon,
}

impl ksni::Tray for BongoTray {
    fn id(&self) -> String {
        "bongocat-studio".to_owned()
    }

    fn title(&self) -> String {
        if self.privacy_paused {
            "BongoCat Studio (Privacy Paused)".to_owned()
        } else {
            "BongoCat Studio".to_owned()
        }
    }

    fn icon_pixmap(&self) -> Vec<ksni::Icon> {
        vec![self.icon.clone()]
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        let _ = self.tx.send(TrayAction::ShowWindow);
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        vec![
            MenuItem::Standard(StandardItem {
                label: "Open BongoCat Studio".to_owned(),
                activate: Box::new(|tray: &mut Self| {
                    let _ = tray.tx.send(TrayAction::ShowWindow);
                }),
                ..StandardItem::default()
            }),
            MenuItem::Separator,
            MenuItem::Standard(StandardItem {
                label: "Preset: Coding (Trackpad)".to_owned(),
                activate: Box::new(|tray: &mut Self| {
                    let _ = tray.tx.send(TrayAction::ApplyPreset(Preset::Coding));
                }),
                ..StandardItem::default()
            }),
            MenuItem::Standard(StandardItem {
                label: "Preset: Gaming (Mouse)".to_owned(),
                activate: Box::new(|tray: &mut Self| {
                    let _ = tray.tx.send(TrayAction::ApplyPreset(Preset::Gaming));
                }),
                ..StandardItem::default()
            }),
            MenuItem::Standard(StandardItem {
                label: "Preset: Minimal (Classic)".to_owned(),
                activate: Box::new(|tray: &mut Self| {
                    let _ = tray.tx.send(TrayAction::ApplyPreset(Preset::Minimal));
                }),
                ..StandardItem::default()
            }),
            MenuItem::Separator,
            MenuItem::Checkmark(CheckmarkItem {
                label: "Pause Privacy".to_owned(),
                checked: self.privacy_paused,
                activate: Box::new(|tray: &mut Self| {
                    let _ = tray.tx.send(TrayAction::TogglePrivacy);
                }),
                ..CheckmarkItem::default()
            }),
            MenuItem::Standard(StandardItem {
                label: "Copy OBS URL".to_owned(),
                activate: Box::new(|tray: &mut Self| {
                    let _ = tray.tx.send(TrayAction::CopyObsUrl);
                }),
                ..StandardItem::default()
            }),
            MenuItem::Separator,
            MenuItem::Standard(StandardItem {
                label: "Quit BongoCat Studio".to_owned(),
                activate: Box::new(|tray: &mut Self| {
                    let _ = tray.tx.send(TrayAction::Quit);
                }),
                ..StandardItem::default()
            }),
        ]
    }
}

pub fn spawn_tray(tx: mpsc::UnboundedSender<TrayAction>) -> TrayHandle {
    let privacy_paused = Arc::new(AtomicBool::new(false));
    let tray = BongoTray {
        tx,
        privacy_paused: false,
        icon: generate_tray_icon(),
    };

    let inner = match tray.spawn() {
        Ok(handle) => {
            debug!("SNI system tray registered");
            Some(handle)
        }
        Err(err) => {
            debug!(error = %err, "SNI system tray unavailable (continuing cleanly)");
            None
        }
    };

    TrayHandle {
        inner,
        privacy_paused,
    }
}

/// Builds a crisp 64x64 RGBA window icon for X11/Wayland compositors.
pub fn build_window_icon() -> Option<iced::window::Icon> {
    let rgba = generate_icon_rgba(64);
    iced::window::icon::from_rgba(rgba, 64, 64).ok()
}

/// Procedurally generates a crisp 32x32 ARGB32 Bongo Cat badge icon for SNI system trays.
fn generate_tray_icon() -> ksni::Icon {
    const SIZE: u32 = 32;
    let rgba = generate_icon_rgba(SIZE);
    let mut argb = vec![0_u8; rgba.len()];
    for (src, dst) in rgba
        .as_chunks::<4>()
        .0
        .iter()
        .zip(argb.as_chunks_mut::<4>().0.iter_mut())
    {
        // StatusNotifierItem expects network byte order: ARGB
        *dst = [src[3], src[0], src[1], src[2]];
    }
    ksni::Icon {
        width: SIZE as i32,
        height: SIZE as i32,
        data: argb,
    }
}

/// Renders a crisp rounded Bongo Cat app icon in RGBA at any square resolution.
fn generate_icon_rgba(size: u32) -> Vec<u8> {
    let mut data = vec![0_u8; (size * size * 4) as usize];
    let scale = size as f32 / 32.0;

    for y in 0..size {
        for x in 0..size {
            let fx = (x as f32 + 0.5) / scale;
            let fy = (y as f32 + 0.5) / scale;
            let dx = fx - 16.0;
            let dy = fy - 16.0;
            let r = dx.hypot(dy);

            let (red, green, blue, a) = if r > 15.2 {
                (0, 0, 0, 0)
            } else {
                let mut color = (15_u8, 23_u8, 42_u8, 255_u8);

                if r > 13.5 {
                    color = (56, 189, 248, 255);
                }

                // White chubby cat silhouette (dome + two pointed ears)
                let in_body = ((fx - 16.0) / 10.5).powi(2) + ((fy - 21.5) / 10.0).powi(2) <= 1.0
                    && fy <= 27.0;
                let in_left_ear = (7.0..=15.0).contains(&fy)
                    && fx >= 7.5 + (15.0 - fy) * 0.35
                    && fx <= 13.5 - (15.0 - fy) * 0.25;
                let in_right_ear = (7.0..=15.0).contains(&fy)
                    && fx >= 18.5 + (15.0 - fy) * 0.25
                    && fx <= 24.5 - (15.0 - fy) * 0.35;

                if in_body || in_left_ear || in_right_ear {
                    color = (255, 255, 255, 255);
                }

                // Cute dark eyes
                let left_eye = (fx - 12.0).hypot(fy - 18.5) <= 1.65;
                let right_eye = (fx - 20.0).hypot(fy - 18.5) <= 1.65;
                if left_eye || right_eye {
                    color = (15, 23, 42, 255);
                }

                // Pink cheek blushes
                let left_blush = ((fx - 9.2) / 1.8).powi(2) + ((fy - 20.8) / 1.1).powi(2) <= 1.0;
                let right_blush = ((fx - 22.8) / 1.8).powi(2) + ((fy - 20.8) / 1.1).powi(2) <= 1.0;
                if left_blush || right_blush {
                    color = (249, 168, 212, 255);
                }

                color
            };

            let idx = ((y * size + x) * 4) as usize;
            data[idx] = red;
            data[idx + 1] = green;
            data[idx + 2] = blue;
            data[idx + 3] = a;
        }
    }

    data
}
