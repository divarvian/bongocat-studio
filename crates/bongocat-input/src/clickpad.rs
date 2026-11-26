use std::time::{Duration, Instant};

use bongocat_core::{InputEvent, MouseButton};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbsRange {
    pub min: i32,
    pub max: i32,
}

impl AbsRange {
    pub const fn new(min: i32, max: i32) -> Self {
        Self { min, max }
    }

    pub fn normalize(self, val: i32) -> f32 {
        let span = (self.max - self.min).max(1) as f32;
        ((val - self.min) as f32 / span).clamp(0.0, 1.0)
    }
}

/// Stateful processor translating raw mouse + touchpad/clickpad evdev streams into
/// clean, unified pointer, multi-finger click, 2-finger scroll, and virtual screen cursor events.
#[derive(Debug, Clone)]
pub struct PointerProcessor {
    // Shared virtual screen cursor (0..1, 0..1) driving natural eye tracking
    cursor_x: f32,
    cursor_y: f32,

    // Mouse virtual pad position (0..1, 0..1)
    mouse_x: f32,
    mouse_y: f32,

    // Touchpad/clickpad live state
    tp_x: f32,
    tp_y: f32,
    tp_touch: bool,
    tp_fingers: u8,
    tp_has_prev: bool,
    tp_prev_x: f32,
    tp_prev_y: f32,
    tp_scroll_accum: f32,
    active_clickpad_btn: Option<MouseButton>,

    // Rate limiting for high-frequency move/cursor events (keys/buttons are never throttled)
    last_move_emit: Option<Instant>,
    last_cursor_emit: Option<Instant>,
}

impl Default for PointerProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl PointerProcessor {
    const MOVE_INTERVAL: Duration = Duration::from_millis(16);
    const SCROLL_STEP: f32 = 0.045;

    pub const fn new() -> Self {
        Self {
            cursor_x: 0.5,
            cursor_y: 0.5,
            mouse_x: 0.5,
            mouse_y: 0.5,
            tp_x: 0.5,
            tp_y: 0.5,
            tp_touch: false,
            tp_fingers: 0,
            tp_has_prev: false,
            tp_prev_x: 0.5,
            tp_prev_y: 0.5,
            tp_scroll_accum: 0.0,
            active_clickpad_btn: None,
            last_move_emit: None,
            last_cursor_emit: None,
        }
    }

    pub const fn cursor(&self) -> (f32, f32) {
        (self.cursor_x, self.cursor_y)
    }

    pub fn on_mouse_rel(&mut self, dx: i32, dy: i32, now: Instant, out: &mut Vec<InputEvent>) {
        if dx == 0 && dy == 0 {
            return;
        }
        let fdx = dx as f32;
        let fdy = dy as f32;

        self.mouse_x = (self.mouse_x + fdx / 380.0).clamp(0.0, 1.0);
        self.mouse_y = (self.mouse_y + fdy / 380.0).clamp(0.0, 1.0);

        self.cursor_x = (self.cursor_x + fdx / 920.0).clamp(0.0, 1.0);
        self.cursor_y = (self.cursor_y + fdy / 640.0).clamp(0.0, 1.0);

        self.maybe_emit_move(self.mouse_x, self.mouse_y, false, 0, now, out);
        self.maybe_emit_cursor(now, out);
    }

    pub fn on_mouse_wheel(&mut self, delta: i32, out: &mut Vec<InputEvent>) {
        if delta != 0 {
            out.push(InputEvent::Wheel {
                delta: delta.signum() as i8,
            });
        }
    }

    pub fn on_touchpad_abs(
        &mut self,
        nx: Option<f32>,
        ny: Option<f32>,
        now: Instant,
        out: &mut Vec<InputEvent>,
    ) {
        let new_x = nx.unwrap_or(self.tp_x).clamp(0.0, 1.0);
        let new_y = ny.unwrap_or(self.tp_y).clamp(0.0, 1.0);

        if self.tp_touch && self.tp_has_prev {
            let dx = new_x - self.tp_prev_x;
            let dy = new_y - self.tp_prev_y;

            if self.tp_fingers >= 2 {
                // Two-finger vertical scroll gesture on clickpads
                self.tp_scroll_accum += dy;
                while self.tp_scroll_accum.abs() >= Self::SCROLL_STEP {
                    let dir = if self.tp_scroll_accum > 0.0 { -1 } else { 1 };
                    self.tp_scroll_accum -= self.tp_scroll_accum.signum() * Self::SCROLL_STEP;
                    out.push(InputEvent::Wheel { delta: dir });
                }
            } else if dx.abs() < 0.35 && dy.abs() < 0.35 {
                // Delta-accumulated virtual cursor resists finger-lift jumps!
                self.cursor_x = (self.cursor_x + dx * 1.65).clamp(0.0, 1.0);
                self.cursor_y = (self.cursor_y + dy * 1.65).clamp(0.0, 1.0);
                self.maybe_emit_cursor(now, out);
            }
        }

        self.tp_x = new_x;
        self.tp_y = new_y;
        self.tp_prev_x = new_x;
        self.tp_prev_y = new_y;
        self.tp_has_prev = self.tp_touch;

        self.maybe_emit_move(
            self.tp_x,
            self.tp_y,
            self.tp_touch,
            self.tp_fingers,
            now,
            out,
        );
    }

    pub fn on_touchpad_touch(&mut self, down: bool, now: Instant, out: &mut Vec<InputEvent>) {
        self.tp_touch = down;
        if down {
            if self.tp_fingers == 0 {
                self.tp_fingers = 1;
            }
            // Wait for first abs sample before computing deltas after finger touchdown
            self.tp_has_prev = false;
            self.tp_scroll_accum = 0.0;
        } else {
            self.tp_fingers = 0;
            self.tp_has_prev = false;
            self.tp_scroll_accum = 0.0;
        }
        out.push(InputEvent::PointerMove {
            x: self.tp_x,
            y: self.tp_y,
            touch: self.tp_touch,
            fingers: self.tp_fingers,
        });
        self.last_move_emit = Some(now);
    }

    pub fn on_touchpad_finger_tool(&mut self, fingers: u8, active: bool) {
        if active {
            self.tp_fingers = self.tp_fingers.max(fingers);
            if fingers >= 2 {
                self.tp_scroll_accum = 0.0;
            }
        } else if self.tp_fingers == fingers {
            self.tp_fingers = u8::from(self.tp_touch);
        }
    }

    pub fn on_button(
        &mut self,
        raw_button: MouseButton,
        down: bool,
        is_clickpad: bool,
        has_dedicated_right_btn: bool,
        out: &mut Vec<InputEvent>,
    ) {
        if is_clickpad && raw_button == MouseButton::Left {
            if down {
                let logical = if self.tp_fingers >= 3 {
                    MouseButton::Middle
                } else if self.tp_fingers == 2
                    || (!has_dedicated_right_btn && self.tp_x >= 0.62 && self.tp_y >= 0.72)
                {
                    MouseButton::Right
                } else {
                    MouseButton::Left
                };
                self.active_clickpad_btn = Some(logical);
                out.push(InputEvent::PointerButton {
                    button: logical,
                    down: true,
                });
            } else {
                let logical = self.active_clickpad_btn.take().unwrap_or(MouseButton::Left);
                out.push(InputEvent::PointerButton {
                    button: logical,
                    down: false,
                });
            }
        } else {
            out.push(InputEvent::PointerButton {
                button: raw_button,
                down,
            });
        }
    }

    fn maybe_emit_move(
        &mut self,
        x: f32,
        y: f32,
        touch: bool,
        fingers: u8,
        now: Instant,
        out: &mut Vec<InputEvent>,
    ) {
        let ready = self
            .last_move_emit
            .is_none_or(|prev| now.duration_since(prev) >= Self::MOVE_INTERVAL);
        if ready {
            self.last_move_emit = Some(now);
            out.push(InputEvent::PointerMove {
                x,
                y,
                touch,
                fingers,
            });
        }
    }

    fn maybe_emit_cursor(&mut self, now: Instant, out: &mut Vec<InputEvent>) {
        let ready = self
            .last_cursor_emit
            .is_none_or(|prev| now.duration_since(prev) >= Self::MOVE_INTERVAL);
        if ready {
            self.last_cursor_emit = Some(now);
            out.push(InputEvent::Cursor {
                x: self.cursor_x,
                y: self.cursor_y,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clickpad_two_finger_click_maps_to_right_and_releases_right() {
        let mut p = PointerProcessor::new();
        let mut out = Vec::new();
        let now = Instant::now();

        p.on_touchpad_touch(true, now, &mut out);
        p.on_touchpad_finger_tool(2, true);
        out.clear();

        p.on_button(MouseButton::Left, true, true, false, &mut out);
        assert_eq!(
            out,
            vec![InputEvent::PointerButton {
                button: MouseButton::Right,
                down: true,
            }]
        );

        // Even if user lifts second finger before releasing click, release matches Right!
        p.on_touchpad_finger_tool(2, false);
        out.clear();
        p.on_button(MouseButton::Left, false, true, false, &mut out);
        assert_eq!(
            out,
            vec![InputEvent::PointerButton {
                button: MouseButton::Right,
                down: false,
            }]
        );
    }

    #[test]
    fn clickpad_bottom_right_corner_maps_to_right_click() {
        let mut p = PointerProcessor::new();
        let mut out = Vec::new();
        let now = Instant::now();

        p.on_touchpad_touch(true, now, &mut out);
        p.on_touchpad_abs(
            Some(0.85),
            Some(0.88),
            now + Duration::from_millis(20),
            &mut out,
        );
        out.clear();

        p.on_button(MouseButton::Left, true, true, false, &mut out);
        assert_eq!(
            out,
            vec![InputEvent::PointerButton {
                button: MouseButton::Right,
                down: true,
            }]
        );
    }

    #[test]
    fn touchpad_finger_lift_does_not_reset_virtual_cursor() {
        let mut p = PointerProcessor::new();
        let mut out = Vec::new();
        let t0 = Instant::now();

        // Swipe right from 0.2 -> 0.5
        p.on_touchpad_touch(true, t0, &mut out);
        p.on_touchpad_abs(
            Some(0.2),
            Some(0.5),
            t0 + Duration::from_millis(20),
            &mut out,
        );
        p.on_touchpad_abs(
            Some(0.5),
            Some(0.5),
            t0 + Duration::from_millis(40),
            &mut out,
        );
        let after_swipe = p.cursor().0;
        assert!(after_swipe > 0.85);

        // Lift finger and place back at left side (0.1) — cursor must NOT jump left!
        p.on_touchpad_touch(false, t0 + Duration::from_millis(60), &mut out);
        p.on_touchpad_touch(true, t0 + Duration::from_millis(80), &mut out);
        p.on_touchpad_abs(
            Some(0.1),
            Some(0.5),
            t0 + Duration::from_millis(100),
            &mut out,
        );
        assert!((p.cursor().0 - after_swipe).abs() < 0.001);
    }

    #[test]
    fn two_finger_drag_emits_scroll_wheel_events() {
        let mut p = PointerProcessor::new();
        let mut out = Vec::new();
        let t0 = Instant::now();

        p.on_touchpad_touch(true, t0, &mut out);
        p.on_touchpad_finger_tool(2, true);
        p.on_touchpad_abs(
            Some(0.5),
            Some(0.4),
            t0 + Duration::from_millis(20),
            &mut out,
        );
        out.clear();

        p.on_touchpad_abs(
            Some(0.5),
            Some(0.52),
            t0 + Duration::from_millis(40),
            &mut out,
        );
        assert!(
            out.iter()
                .any(|e| matches!(e, InputEvent::Wheel { delta: -1 }))
        );
    }
}
