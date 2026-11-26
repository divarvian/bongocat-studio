use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use bongocat_core::{DeviceInfo, InputEvent, MouseButton};
use evdev::{AbsoluteAxisType, EventType, InputEventKind, Key, RelativeAxisType};
use tokio::runtime::Handle;
use tokio::sync::{broadcast, watch};
use tokio::task::JoinHandle;
use tracing::{debug, info, warn};

use crate::clickpad::{AbsRange, PointerProcessor};
use crate::discovery::{DiscoveredDevice, scan_input_devices};

/// Handle owning the async epoll-driven evdev supervisor. Stops cleanly on drop.
#[derive(Debug)]
pub struct InputHandle {
    stop_tx: watch::Sender<bool>,
    supervisor: JoinHandle<()>,
}

impl InputHandle {
    pub fn shutdown(self) {
        let _ = self.stop_tx.send(true);
        self.supervisor.abort();
    }
}

impl Drop for InputHandle {
    fn drop(&mut self) {
        let _ = self.stop_tx.send(true);
        self.supervisor.abort();
    }
}

#[derive(Debug, Clone, Copy)]
struct DeviceCaps {
    is_keyboard: bool,
    is_mouse: bool,
    is_touchpad: bool,
    has_dedicated_right_btn: bool,
    abs_x: Option<AbsRange>,
    abs_y: Option<AbsRange>,
}

struct ActiveTask {
    info: DeviceInfo,
    handle: JoinHandle<()>,
}

/// Spawns an async, zero-polling (`epoll`) evdev input supervisor with automatic hotplug support.
pub fn spawn_input_reader(rt: &Handle, tx: broadcast::Sender<InputEvent>) -> InputHandle {
    let (stop_tx, stop_rx) = watch::channel(false);
    let supervisor = rt.spawn(run_supervisor(tx, stop_rx));
    InputHandle {
        stop_tx,
        supervisor,
    }
}

async fn run_supervisor(tx: broadcast::Sender<InputEvent>, mut stop_rx: watch::Receiver<bool>) {
    let processor = Arc::new(Mutex::new(PointerProcessor::new()));
    let mut active: HashMap<PathBuf, ActiveTask> = HashMap::new();
    let mut last_error: Option<String> = None;

    let mut interval = tokio::time::interval(Duration::from_secs(3));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    loop {
        tokio::select! {
            _ = interval.tick() => {
                // Reap any finished (disconnected) device tasks first.
                let prev_count = active.len();
                active.retain(|_, task| !task.handle.is_finished());
                let mut changed = active.len() != prev_count;

                let scan = scan_input_devices();
                let mut seen_paths = Vec::with_capacity(scan.devices.len());

                for dev in scan.devices {
                    seen_paths.push(dev.path.clone());
                    if active.contains_key(&dev.path) {
                        continue;
                    }
                    let path = dev.path.clone();
                    let info = dev.info.clone();
                    let task = spawn_device_task(dev, tx.clone(), Arc::clone(&processor), stop_rx.clone());
                    if let Some(handle) = task {
                        active.insert(path, ActiveTask { info, handle });
                        changed = true;
                    }
                }

                // Cancel tasks for devices that disappeared from /dev/input.
                let removed: Vec<PathBuf> = active
                    .keys()
                    .filter(|p| !seen_paths.contains(p))
                    .cloned()
                    .collect();
                for path in removed {
                    if let Some(task) = active.remove(&path) {
                        task.handle.abort();
                        changed = true;
                    }
                }

                let next_error = if active.is_empty() && scan.permission_denied {
                    Some(
                        "Permission denied reading /dev/input/event*. Add your user to the 'input' group (sudo usermod -aG input $USER) and re-login."
                            .to_owned(),
                    )
                } else {
                    None
                };

                if next_error != last_error {
                    if let Some(msg) = &next_error {
                        warn!("{msg}");
                        let _ = tx.send(InputEvent::InputError(msg.clone()));
                    } else {
                        let _ = tx.send(InputEvent::InputError(String::new()));
                    }
                    last_error = next_error;
                }

                if changed {
                    let mut infos: Vec<DeviceInfo> =
                        active.values().map(|t| t.info.clone()).collect();
                    infos.sort_by(|a, b| a.name.cmp(&b.name));
                    info!(count = infos.len(), "active input devices updated");
                    let _ = tx.send(InputEvent::DevicesUpdated(infos));
                }
            }
            _ = stop_rx.changed() => {
                if *stop_rx.borrow() {
                    break;
                }
            }
        }
    }

    for (_, task) in active {
        task.handle.abort();
    }
}

fn spawn_device_task(
    dev: DiscoveredDevice,
    tx: broadcast::Sender<InputEvent>,
    processor: Arc<Mutex<PointerProcessor>>,
    mut stop_rx: watch::Receiver<bool>,
) -> Option<JoinHandle<()>> {
    let caps = DeviceCaps {
        is_keyboard: dev.is_keyboard,
        is_mouse: dev.is_mouse,
        is_touchpad: dev.is_touchpad,
        has_dedicated_right_btn: dev.has_dedicated_right_btn,
        abs_x: dev.abs_x,
        abs_y: dev.abs_y,
    };
    let name = dev.info.name.clone();
    let mut stream = match dev.device.into_event_stream() {
        Ok(s) => s,
        Err(err) => {
            debug!(device = %name, error = %err, "failed to create async evdev stream");
            return None;
        }
    };

    Some(tokio::spawn(async move {
        let mut rel_dx = 0_i32;
        let mut rel_dy = 0_i32;
        let mut abs_nx: Option<f32> = None;
        let mut abs_ny: Option<f32> = None;
        let mut out = Vec::with_capacity(8);

        loop {
            tokio::select! {
                res = stream.next_event() => {
                    let Ok(ev) = res else {
                        debug!(device = %name, "input device stream closed");
                        break;
                    };

                    let now = Instant::now();
                    match ev.kind() {
                        InputEventKind::Key(key) => {
                            // Privacy guarantee: NEVER log keycodes or key names!
                            if let Ok(mut proc) = processor.lock() {
                                handle_key_event(caps, key, ev.value(), now, &mut proc, &mut out);
                            }
                        }
                        InputEventKind::RelAxis(axis) if caps.is_mouse => match axis {
                            RelativeAxisType::REL_X => rel_dx += ev.value(),
                            RelativeAxisType::REL_Y => rel_dy += ev.value(),
                            RelativeAxisType::REL_WHEEL => {
                                if let Ok(mut proc) = processor.lock() {
                                    proc.on_mouse_wheel(ev.value(), &mut out);
                                }
                            }
                            _ => {}
                        },
                        InputEventKind::AbsAxis(axis) if caps.is_touchpad => match axis {
                            AbsoluteAxisType::ABS_X | AbsoluteAxisType::ABS_MT_POSITION_X => {
                                if let Some(range) = caps.abs_x {
                                    abs_nx = Some(range.normalize(ev.value()));
                                }
                            }
                            AbsoluteAxisType::ABS_Y | AbsoluteAxisType::ABS_MT_POSITION_Y => {
                                if let Some(range) = caps.abs_y {
                                    abs_ny = Some(range.normalize(ev.value()));
                                }
                            }
                            _ => {}
                        },
                        _ if ev.event_type() == EventType::SYNCHRONIZATION => {
                            if (rel_dx != 0 || rel_dy != 0 || abs_nx.is_some() || abs_ny.is_some())
                                && let Ok(mut proc) = processor.lock()
                            {
                                if rel_dx != 0 || rel_dy != 0 {
                                    proc.on_mouse_rel(rel_dx, rel_dy, now, &mut out);
                                    rel_dx = 0;
                                    rel_dy = 0;
                                }
                                if abs_nx.is_some() || abs_ny.is_some() {
                                    proc.on_touchpad_abs(abs_nx.take(), abs_ny.take(), now, &mut out);
                                }
                            }
                        }
                        _ => {}
                    }

                    for item in out.drain(..) {
                        let _ = tx.send(item);
                    }
                }
                _ = stop_rx.changed() => {
                    if *stop_rx.borrow() {
                        break;
                    }
                }
            }
        }
    }))
}

fn handle_key_event(
    caps: DeviceCaps,
    key: Key,
    value: i32,
    now: Instant,
    processor: &mut PointerProcessor,
    out: &mut Vec<InputEvent>,
) {
    if value != 0 && value != 1 {
        return;
    }
    let down = value == 1;

    match key {
        Key::BTN_LEFT => processor.on_button(
            MouseButton::Left,
            down,
            caps.is_touchpad,
            caps.has_dedicated_right_btn,
            out,
        ),
        Key::BTN_RIGHT => processor.on_button(
            MouseButton::Right,
            down,
            caps.is_touchpad,
            caps.has_dedicated_right_btn,
            out,
        ),
        Key::BTN_MIDDLE => processor.on_button(
            MouseButton::Middle,
            down,
            caps.is_touchpad,
            caps.has_dedicated_right_btn,
            out,
        ),
        Key::BTN_TOUCH if caps.is_touchpad => {
            processor.on_touchpad_touch(down, now, out);
        }
        Key::BTN_TOOL_FINGER if caps.is_touchpad => {
            processor.on_touchpad_finger_tool(1, down);
        }
        Key::BTN_TOOL_DOUBLETAP if caps.is_touchpad => {
            processor.on_touchpad_finger_tool(2, down);
        }
        Key::BTN_TOOL_TRIPLETAP if caps.is_touchpad => {
            processor.on_touchpad_finger_tool(3, down);
        }
        _ if caps.is_keyboard && key.code() < 256 => {
            out.push(InputEvent::Key {
                code: key.code(),
                down,
            });
        }
        _ => {}
    }
}
