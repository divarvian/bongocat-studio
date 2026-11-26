use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

use bongocat_core::keymap::resolve_key;
use bongocat_core::{DeviceInfo, InputEvent, OverlayConfig, ServerStatus, WireEvent};
use tokio::sync::broadcast;

/// Shared state & SSE broadcast fan-out hub coordinating the Desktop App and OBS clients.
#[derive(Debug, Clone)]
pub struct ServerHub {
    inner: Arc<HubInner>,
}

#[derive(Debug)]
struct HubInner {
    port: u16,
    config: RwLock<OverlayConfig>,
    privacy_paused: AtomicBool,
    client_count: AtomicUsize,
    devices: RwLock<Vec<DeviceInfo>>,
    input_error: RwLock<Option<String>>,
    wire_tx: broadcast::Sender<String>,
}

impl ServerHub {
    pub fn new(port: u16, initial_config: OverlayConfig) -> Self {
        let (wire_tx, _) = broadcast::channel(512);
        Self {
            inner: Arc::new(HubInner {
                port,
                config: RwLock::new(initial_config),
                privacy_paused: AtomicBool::new(false),
                client_count: AtomicUsize::new(0),
                devices: RwLock::new(Vec::new()),
                input_error: RwLock::new(None),
                wire_tx,
            }),
        }
    }

    pub fn port(&self) -> u16 {
        self.inner.port
    }

    pub fn client_count(&self) -> usize {
        self.inner.client_count.load(Ordering::Relaxed)
    }

    pub fn privacy_paused(&self) -> bool {
        self.inner.privacy_paused.load(Ordering::Relaxed)
    }

    pub fn current_config(&self) -> OverlayConfig {
        self.inner
            .config
            .read()
            .map_or_else(|_| OverlayConfig::default(), |c| c.clone())
    }

    pub fn update_config(&self, config: OverlayConfig) {
        if let Ok(mut guard) = self.inner.config.write() {
            *guard = config.clone();
        }
        self.broadcast_wire(&WireEvent::Config {
            config: Box::new(config),
        });
    }

    pub fn set_privacy_paused(&self, paused: bool) {
        self.inner.privacy_paused.store(paused, Ordering::Relaxed);
        self.broadcast_wire(&WireEvent::Privacy { paused });
    }

    /// Processes a local `InputEvent`, enforcing privacy masking before emitting over SSE.
    pub fn publish_input(&self, event: &InputEvent) {
        match event {
            InputEvent::Key { code, down } => {
                if self.privacy_paused() {
                    return;
                }
                // Mask sensitive keycodes in Rust before they ever cross the loopback SSE wire.
                let privacy = self.current_config().privacy_mode;
                let resolved = resolve_key(*code, privacy);
                self.broadcast_wire(&WireEvent::Key {
                    c: resolved.visual_code,
                    d: *down,
                });
            }
            InputEvent::PointerMove {
                x,
                y,
                touch,
                fingers,
            } => {
                self.broadcast_wire(&WireEvent::Move {
                    x: (*x * 1000.0).round() / 1000.0,
                    y: (*y * 1000.0).round() / 1000.0,
                    touch: *touch,
                    fingers: *fingers,
                });
            }
            InputEvent::PointerButton { button, down } => {
                self.broadcast_wire(&WireEvent::Button {
                    b: button.as_u8(),
                    d: *down,
                });
            }
            InputEvent::Wheel { delta } => {
                self.broadcast_wire(&WireEvent::Wheel { d: *delta });
            }
            InputEvent::Cursor { x, y } => {
                self.broadcast_wire(&WireEvent::Cursor {
                    x: (*x * 1000.0).round() / 1000.0,
                    y: (*y * 1000.0).round() / 1000.0,
                });
            }
            InputEvent::DevicesUpdated(list) => {
                if let Ok(mut guard) = self.inner.devices.write() {
                    guard.clone_from(list);
                }
            }
            InputEvent::InputError(msg) => {
                if let Ok(mut guard) = self.inner.input_error.write() {
                    *guard = if msg.is_empty() {
                        None
                    } else {
                        Some(msg.clone())
                    };
                }
            }
        }
    }

    pub fn subscribe(&self) -> (ClientGuard, broadcast::Receiver<String>) {
        self.inner.client_count.fetch_add(1, Ordering::Relaxed);
        let rx = self.inner.wire_tx.subscribe();
        (
            ClientGuard {
                inner: Arc::clone(&self.inner),
            },
            rx,
        )
    }

    pub fn initial_snapshot_payloads(&self) -> Vec<String> {
        let cfg = WireEvent::Config {
            config: Box::new(self.current_config()),
        };
        let priv_ev = WireEvent::Privacy {
            paused: self.privacy_paused(),
        };
        let mut out = Vec::with_capacity(2);
        if let Ok(s) = serde_json::to_string(&cfg) {
            out.push(s);
        }
        if let Ok(s) = serde_json::to_string(&priv_ev) {
            out.push(s);
        }
        out
    }

    pub fn status(&self) -> ServerStatus {
        let devices = self
            .inner
            .devices
            .read()
            .map_or_else(|_| Vec::new(), |d| d.clone());
        let input_error = self.inner.input_error.read().ok().and_then(|e| e.clone());

        ServerStatus {
            ok: true,
            port: self.inner.port,
            clients: self.client_count(),
            devices,
            privacy_paused: self.privacy_paused(),
            input_error,
        }
    }

    fn broadcast_wire(&self, ev: &WireEvent) {
        if self.inner.wire_tx.receiver_count() == 0 {
            return;
        }
        if let Ok(json) = serde_json::to_string(ev) {
            let _ = self.inner.wire_tx.send(json);
        }
    }
}

/// RAII guard decrementing the active SSE client counter when an OBS client disconnects.
#[derive(Debug)]
pub struct ClientGuard {
    inner: Arc<HubInner>,
}

impl Drop for ClientGuard {
    fn drop(&mut self) {
        self.inner.client_count.fetch_sub(1, Ordering::Relaxed);
    }
}
