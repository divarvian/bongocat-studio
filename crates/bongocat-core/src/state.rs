use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};

use crate::config::{DisplayMode, EyeDirection, OverlayConfig, PointerDevice, RightPawMode};
use crate::event::{InputEvent, MouseButton};
use crate::geometry::{
    KeyLayoutCell, Pt, compute_keyboard_layout, flat_palm_rests, flat_quads, pointer_pad_point,
    slanted_palm_rests, slanted_quads,
};
use crate::keymap::{KeyHand, key_spec, resolve_key};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PawPose {
    /// Actively pressing a key, touching trackpad, or clicking/moving mouse.
    Active,
    /// Brief natural standby resting along the lower keyboard / palm-rest or pointer.
    Resting,
    /// Deep idle / sleep: paw raised beside the cheek showing pink paw beans.
    Raised,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassicFrame {
    BothUp,
    LeftDown,
    RightDown,
    BothDown,
    Sleeping,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HudPillKind {
    Key,
    LeftClick,
    RightClick,
    MiddleClick,
    Wheel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HudPill {
    pub text: String,
    pub kind: HudPillKind,
}

/// Deterministic real-time animation & kinematics state driving the Desktop 60fps Preview.
#[derive(Debug, Clone)]
pub struct CatState {
    pub config: OverlayConfig,
    pub privacy_paused: bool,

    // Cached keycap cells for current mode + pointer layout
    layout_mode: DisplayMode,
    layout_pointer: PointerDevice,
    key_cells: Vec<KeyLayoutCell>,
    key_index: HashMap<u16, usize>,

    // Keyboard pressed tracking
    raw_to_visual: HashMap<u16, (u16, bool)>,
    visual_ref_counts: HashMap<u16, usize>,
    last_key_hand: KeyHand,

    // Pointer & cursor tracking
    pub ptr_x: f32,
    pub ptr_y: f32,
    pub ptr_fingers: u8,
    pub buttons_down: HashSet<MouseButton>,
    pub wheel_dir: i8,
    wheel_until: Option<Instant>,

    pub cursor_x: f32,
    pub cursor_y: f32,

    // Smooth interpolated visuals
    pub cur_ptr_x: f32,
    pub cur_ptr_y: f32,
    pub cur_eye_x: f32,
    pub cur_eye_y: f32,
    eye_target_x: f32,
    eye_target_y: f32,

    pub left_active_target: Option<Pt>,
    pub right_active_target: Option<Pt>,
    left_hold_until: Option<Instant>,
    right_hold_until: Option<Instant>,

    // Independent per-hand posture lifecycle (Active -> Resting on palm-rest -> Raised)
    last_left_activity: Option<Instant>,
    last_right_key_activity: Option<Instant>,
    last_pointer_activity: Option<Instant>,
    pub left_pose: PawPose,
    pub right_pose: PawPose,
    /// Smooth 0.0 (down/arm) .. 1.0 (raised pink-bean paw) transition blend.
    pub left_raise: f32,
    /// Smooth 0.0 (down/arm) .. 1.0 (raised pink-bean paw) transition blend.
    pub right_raise: f32,

    pub cur_right_paw: Pt,
    pub cur_left_paw: Pt,

    // Classic frame + bounce + KPS + sleep + blink
    pub classic_frame: ClassicFrame,
    classic_next_hand: KeyHand,
    classic_until: Option<Instant>,

    pub bounce_phase: f32,
    kps_timestamps: VecDeque<Instant>,
    pub kps: f32,
    pub fast_typing: bool,

    pub sleeping: bool,
    last_activity: Instant,

    pub blinking: bool,
    next_blink_at: Instant,
    blink_end_at: Option<Instant>,
    blink_seed: u32,
}

impl CatState {
    pub fn new(config: OverlayConfig, now: Instant) -> Self {
        let mode = config.mode;
        let pointer = config.pointer;
        let (key_cells, key_index) = build_key_lookup(mode, pointer);
        let (init_right, init_left) = active_palm_rests(mode, pointer);

        Self {
            config,
            privacy_paused: false,
            layout_mode: mode,
            layout_pointer: pointer,
            key_cells,
            key_index,
            raw_to_visual: HashMap::new(),
            visual_ref_counts: HashMap::new(),
            last_key_hand: KeyHand::Left,
            ptr_x: 0.5,
            ptr_y: 0.5,
            ptr_fingers: 0,
            buttons_down: HashSet::new(),
            wheel_dir: 0,
            wheel_until: None,
            cursor_x: 0.5,
            cursor_y: 0.5,
            cur_ptr_x: 0.5,
            cur_ptr_y: 0.5,
            cur_eye_x: 0.0,
            cur_eye_y: 0.0,
            eye_target_x: 0.0,
            eye_target_y: 0.0,
            left_active_target: None,
            right_active_target: None,
            left_hold_until: None,
            right_hold_until: None,
            last_left_activity: None,
            last_right_key_activity: None,
            last_pointer_activity: None,
            left_pose: PawPose::Raised,
            right_pose: PawPose::Raised,
            left_raise: 1.0,
            right_raise: 1.0,
            cur_right_paw: init_right,
            cur_left_paw: init_left,
            classic_frame: ClassicFrame::BothUp,
            classic_next_hand: KeyHand::Left,
            classic_until: None,
            bounce_phase: 0.0,
            kps_timestamps: VecDeque::new(),
            kps: 0.0,
            fast_typing: false,
            sleeping: false,
            last_activity: now,
            blinking: false,
            next_blink_at: now + Duration::from_millis(3600),
            blink_end_at: None,
            blink_seed: 0x51A7_E9C3,
        }
    }

    pub fn set_config(&mut self, config: OverlayConfig) {
        let layout_changed =
            self.layout_mode != config.mode || self.layout_pointer != config.pointer;
        self.config = config;
        if layout_changed {
            self.layout_mode = self.config.mode;
            self.layout_pointer = self.config.pointer;
            let (cells, index) = build_key_lookup(self.layout_mode, self.layout_pointer);
            self.key_cells = cells;
            self.key_index = index;
            self.left_active_target = None;
            self.right_active_target = None;
            let (rest_r, rest_l) = active_palm_rests(self.config.mode, self.config.pointer);
            self.cur_right_paw = rest_r;
            self.cur_left_paw = rest_l;
        }
        self.recompute_eye_target();
    }

    pub fn set_privacy_paused(&mut self, paused: bool) {
        self.privacy_paused = paused;
        if paused {
            self.raw_to_visual.clear();
            self.visual_ref_counts.clear();
            self.left_active_target = None;
            self.right_active_target = None;
        }
    }

    pub fn key_cells(&self) -> &[KeyLayoutCell] {
        &self.key_cells
    }

    pub fn is_key_pressed(&self, visual_code: u16) -> bool {
        self.visual_ref_counts.contains_key(&visual_code)
    }

    pub fn handle_event(&mut self, event: &InputEvent, now: Instant) {
        match event {
            InputEvent::Key { code, down } => {
                if self.privacy_paused {
                    return;
                }
                self.handle_key(*code, *down, now);
            }
            InputEvent::PointerMove {
                x,
                y,
                touch,
                fingers,
            } => {
                self.wake(now);
                self.ptr_x = x.clamp(0.0, 1.0);
                self.ptr_y = y.clamp(0.0, 1.0);
                self.ptr_fingers = if *fingers > 0 {
                    *fingers
                } else {
                    u8::from(*touch)
                };
                self.last_pointer_activity = Some(now);
                self.right_active_target = None;
                self.right_pose =
                    if self.ptr_fingers > 0 || self.config.pointer == PointerDevice::Mouse {
                        PawPose::Active
                    } else {
                        PawPose::Resting
                    };
            }
            InputEvent::PointerButton { button, down } => {
                self.wake(now);
                self.last_pointer_activity = Some(now);
                self.right_active_target = None;
                if *down {
                    self.buttons_down.insert(*button);
                    self.right_pose = PawPose::Active;
                    self.record_hit(now);
                    self.trigger_classic_hit(now);
                } else {
                    self.buttons_down.remove(button);
                }
            }
            InputEvent::Wheel { delta } => {
                self.wake(now);
                self.last_pointer_activity = Some(now);
                self.right_pose = PawPose::Active;
                self.wheel_dir = delta.signum();
                self.wheel_until = Some(now + Duration::from_millis(240));
            }
            InputEvent::Cursor { x, y } => {
                self.cursor_x = x.clamp(0.0, 1.0);
                self.cursor_y = y.clamp(0.0, 1.0);
                self.recompute_eye_target();
            }
            InputEvent::DevicesUpdated(_) | InputEvent::InputError(_) => {}
        }
    }

    fn handle_key(&mut self, raw_code: u16, down: bool, now: Instant) {
        if down {
            self.wake(now);
            self.record_hit(now);
            self.trigger_classic_hit(now);

            let resolved = resolve_key(raw_code, self.config.privacy_mode);
            if let Some((prev_visual, _)) = self
                .raw_to_visual
                .insert(raw_code, (resolved.visual_code, resolved.masked))
            {
                self.decrement_visual(prev_visual);
            }
            *self
                .visual_ref_counts
                .entry(resolved.visual_code)
                .or_insert(0) += 1;

            if let Some(&idx) = self.key_index.get(&resolved.visual_code) {
                let cell = self.key_cells[idx];
                let (hand, target) = self.pick_hand(cell);
                if cell.spec.hand != KeyHand::Space {
                    self.last_key_hand = hand;
                }
                let hold = Duration::from_millis(u64::from(self.config.key_hold_ms.max(90)));
                match hand {
                    KeyHand::Right => {
                        self.right_active_target = Some(target);
                        self.right_hold_until = Some(now + hold);
                        self.last_right_key_activity = Some(now);
                        self.right_pose = PawPose::Active;
                    }
                    KeyHand::Left | KeyHand::Space => {
                        self.left_active_target = Some(target);
                        self.left_hold_until = Some(now + hold);
                        self.last_left_activity = Some(now);
                        self.left_pose = PawPose::Active;
                    }
                }
            }
        } else {
            if let Some((visual, _)) = self.raw_to_visual.remove(&raw_code) {
                self.decrement_visual(visual);
            }
            let release_tail = Duration::from_millis(40);
            if !self.has_held_hand(KeyHand::Left) && self.left_active_target.is_some() {
                self.left_hold_until = Some(now + release_tail);
            }
            if !self.has_held_hand(KeyHand::Right) && self.right_active_target.is_some() {
                self.right_hold_until = Some(now + release_tail);
            }
        }
    }

    fn decrement_visual(&mut self, visual: u16) {
        if let Some(count) = self.visual_ref_counts.get_mut(&visual) {
            if *count <= 1 {
                self.visual_ref_counts.remove(&visual);
            } else {
                *count -= 1;
            }
        }
    }

    fn pick_hand(&self, cell: KeyLayoutCell) -> (KeyHand, Pt) {
        let right_busy =
            self.config.right_paw == RightPawMode::Pointer || !self.buttons_down.is_empty();
        match cell.spec.hand {
            KeyHand::Space => {
                if right_busy || self.last_key_hand == KeyHand::Right {
                    (KeyHand::Left, cell.left_thumb)
                } else {
                    (KeyHand::Right, cell.right_thumb)
                }
            }
            KeyHand::Right => {
                if right_busy {
                    (KeyHand::Left, cell.center)
                } else {
                    (KeyHand::Right, cell.center)
                }
            }
            KeyHand::Left => (KeyHand::Left, cell.center),
        }
    }

    fn has_held_hand(&self, hand: KeyHand) -> bool {
        self.visual_ref_counts.keys().any(|&visual| {
            key_spec(visual).is_some_and(|spec| {
                spec.hand == hand || (spec.hand == KeyHand::Space && hand == KeyHand::Left)
            })
        })
    }

    fn wake(&mut self, now: Instant) {
        self.last_activity = now;
        if self.sleeping {
            self.sleeping = false;
            self.classic_frame = ClassicFrame::BothUp;
        }
    }

    fn record_hit(&mut self, now: Instant) {
        self.kps_timestamps.push_back(now);
        self.prune_kps(now);
        if self.config.bounce {
            self.bounce_phase = 1.0;
        }
    }

    fn trigger_classic_hit(&mut self, now: Instant) {
        let both_down = self.kps >= self.config.kps_happy_threshold as f32;
        if both_down {
            self.classic_frame = ClassicFrame::BothDown;
        } else {
            self.classic_frame = match self.classic_next_hand {
                KeyHand::Left => {
                    self.classic_next_hand = KeyHand::Right;
                    ClassicFrame::LeftDown
                }
                KeyHand::Right | KeyHand::Space => {
                    self.classic_next_hand = KeyHand::Left;
                    ClassicFrame::RightDown
                }
            };
        }
        self.classic_until = Some(now + Duration::from_millis(u64::from(self.config.key_hold_ms)));
    }

    fn prune_kps(&mut self, now: Instant) {
        let cutoff = now.checked_sub(Duration::from_secs(1)).unwrap_or(now);
        while self.kps_timestamps.front().is_some_and(|&ts| ts < cutoff) {
            self.kps_timestamps.pop_front();
        }
        self.kps = self.kps_timestamps.len() as f32;
        self.fast_typing = !self.sleeping && self.kps >= self.config.kps_happy_threshold as f32;
    }

    fn recompute_eye_target(&mut self) {
        if !self.config.eyes_enabled {
            self.eye_target_x = 0.0;
            self.eye_target_y = 0.0;
            return;
        }
        let sx = match self.config.eye_dir {
            EyeDirection::Screen => self.cursor_x,
            EyeDirection::Mirror => 1.0 - self.cursor_x,
        };
        self.eye_target_x = (sx - 0.5) * 24.0;
        self.eye_target_y = (self.cursor_y - 0.5) * 14.0;
    }

    /// Advances timers, interpolation, blinking, KPS decay, and sleep detection.
    /// Returns `true` when visuals changed and the canvas should redraw.
    pub fn tick(&mut self, now: Instant, dt_secs: f32) -> bool {
        let dt = dt_secs.clamp(0.001, 0.1);
        let smooth = |rate: f32| 1.0 - (-rate * dt).exp();

        self.prune_kps(now);

        if let Some(until) = self.wheel_until
            && now >= until
        {
            self.wheel_dir = 0;
            self.wheel_until = None;
        }

        if let Some(until) = self.left_hold_until
            && now >= until
            && !self.has_held_hand(KeyHand::Left)
        {
            self.left_active_target = None;
            self.left_hold_until = None;
        }
        if let Some(until) = self.right_hold_until
            && now >= until
            && !self.has_held_hand(KeyHand::Right)
        {
            self.right_active_target = None;
            self.right_hold_until = None;
        }

        if let Some(until) = self.classic_until
            && now >= until
        {
            self.classic_until = None;
            if !self.sleeping {
                self.classic_frame = ClassicFrame::BothUp;
            }
        }

        if self.config.sleep_sec > 0
            && !self.sleeping
            && now.duration_since(self.last_activity)
                >= Duration::from_secs(u64::from(self.config.sleep_sec))
        {
            self.sleeping = true;
            self.fast_typing = false;
            self.left_active_target = None;
            self.right_active_target = None;
            self.classic_frame = ClassicFrame::Sleeping;
        }

        // Natural blinking cadence
        if self.sleeping {
            self.blinking = false;
        } else if let Some(end) = self.blink_end_at {
            if now >= end {
                self.blinking = false;
                self.blink_end_at = None;
                let jitter_ms = u64::from(self.next_prng() % 3200);
                self.next_blink_at = now + Duration::from_millis(3200 + jitter_ms);
            }
        } else if now >= self.next_blink_at {
            self.blinking = true;
            self.blink_end_at = Some(now + Duration::from_millis(115));
        }

        // Bounce impulse decay
        if self.bounce_phase > 0.001 {
            self.bounce_phase = (self.bounce_phase - dt * 7.5).max(0.0);
        } else {
            self.bounce_phase = 0.0;
        }

        // Pointer & eye smoothing
        let ptr_f = smooth(24.0);
        self.cur_ptr_x += (self.ptr_x - self.cur_ptr_x) * ptr_f;
        self.cur_ptr_y += (self.ptr_y - self.cur_ptr_y) * ptr_f;

        let eye_f = smooth(14.0);
        self.cur_eye_x += (self.eye_target_x - self.cur_eye_x) * eye_f;
        self.cur_eye_y += (self.eye_target_y - self.cur_eye_y) * eye_f;

        let raise_delay = Duration::from_secs(u64::from(self.config.idle_raise_sec.max(1)));
        let (palm_rest_r, palm_rest_l) = active_palm_rests(self.config.mode, self.config.pointer);

        // ---- Left hand posture (Active -> Resting on palm-rest -> Raised) ----
        let left_recent = self
            .last_left_activity
            .is_some_and(|t| now.saturating_duration_since(t) < raise_delay);
        self.left_pose = if self.sleeping {
            PawPose::Raised
        } else if self.left_active_target.is_some() || self.has_held_hand(KeyHand::Left) {
            PawPose::Active
        } else if left_recent {
            PawPose::Resting
        } else {
            PawPose::Raised
        };

        let left_target_raise = if self.left_pose == PawPose::Raised {
            1.0
        } else {
            0.0
        };
        let left_raise_rate = if left_target_raise > 0.5 { 11.0 } else { 28.0 };
        self.left_raise += (left_target_raise - self.left_raise) * smooth(left_raise_rate);
        if (self.left_raise - left_target_raise).abs() < 0.005 {
            self.left_raise = left_target_raise;
        }

        if let Some(left_target) = self.left_active_target {
            self.cur_left_paw = self.cur_left_paw.lerp(left_target, smooth(32.0));
        } else {
            self.cur_left_paw = self.cur_left_paw.lerp(palm_rest_l, smooth(14.0));
        }

        // ---- Right hand posture (Active -> Resting on palm-rest/pointer -> Raised) ----
        let right_key_recent = self
            .last_right_key_activity
            .is_some_and(|t| now.saturating_duration_since(t) < raise_delay);
        let pointer_recent = self
            .last_pointer_activity
            .is_some_and(|t| now.saturating_duration_since(t) < raise_delay);
        let pointer_active =
            !self.buttons_down.is_empty() || self.ptr_fingers > 0 || self.wheel_dir != 0;

        self.right_pose = if self.sleeping {
            PawPose::Raised
        } else if self.right_active_target.is_some()
            || self.has_held_hand(KeyHand::Right)
            || pointer_active
        {
            PawPose::Active
        } else if right_key_recent || pointer_recent {
            PawPose::Resting
        } else {
            PawPose::Raised
        };

        let right_target_raise = if self.right_pose == PawPose::Raised {
            1.0
        } else {
            0.0
        };
        let right_raise_rate = if right_target_raise > 0.5 { 11.0 } else { 28.0 };
        self.right_raise += (right_target_raise - self.right_raise) * smooth(right_raise_rate);
        if (self.right_raise - right_target_raise).abs() < 0.005 {
            self.right_raise = right_target_raise;
        }

        // Right paw target: active key, live pointer, or palm-rest standby!
        let pad = pointer_pad_point(
            self.config.mode,
            self.config.pointer,
            self.config.ptr_dir,
            self.cur_ptr_x,
            self.cur_ptr_y,
        );
        let typed_more_recently = match (self.last_right_key_activity, self.last_pointer_activity) {
            (Some(k), Some(p)) => k > p,
            (Some(_), None) => true,
            _ => false,
        };
        let rest_on_keyboard = !pointer_active
            && self.config.pointer != PointerDevice::Mouse
            && self.config.right_paw != RightPawMode::Pointer
            && typed_more_recently;

        let right_target = if let Some(key_pt) = self.right_active_target {
            key_pt
        } else if rest_on_keyboard {
            palm_rest_r
        } else {
            let dip = if self.buttons_down.is_empty() {
                0.0
            } else {
                4.5
            };
            let hover_lift = if self.config.pointer == PointerDevice::Trackpad
                && self.ptr_fingers == 0
                && dip == 0.0
            {
                -4.5
            } else {
                0.0
            };
            let mouse_y = if self.config.pointer == PointerDevice::Mouse {
                -10.0
            } else {
                0.0
            };
            let btn_x = if self.buttons_down.contains(&MouseButton::Right) {
                -4.5
            } else if self.buttons_down.contains(&MouseButton::Left) {
                3.5
            } else {
                0.0
            };
            Pt::new(pad.x + btn_x, pad.y + mouse_y + dip + hover_lift)
        };
        let r_speed = if self.right_pose == PawPose::Active {
            30.0
        } else {
            14.0
        };
        self.cur_right_paw = self.cur_right_paw.lerp(right_target, smooth(r_speed));

        true
    }

    /// Returns whether the preview needs continuous 60fps animation ticks right now.
    pub fn is_animating(&self) -> bool {
        let (palm_rest_r, palm_rest_l) = active_palm_rests(self.config.mode, self.config.pointer);
        let pad = pointer_pad_point(
            self.config.mode,
            self.config.pointer,
            self.config.ptr_dir,
            self.cur_ptr_x,
            self.cur_ptr_y,
        );
        let typed_more_recently = match (self.last_right_key_activity, self.last_pointer_activity) {
            (Some(k), Some(p)) => k > p,
            (Some(_), None) => true,
            _ => false,
        };
        let right_rest_target = if self.config.pointer != PointerDevice::Mouse
            && self.config.right_paw != RightPawMode::Pointer
            && typed_more_recently
        {
            palm_rest_r
        } else if self.config.pointer == PointerDevice::Mouse {
            Pt::new(pad.x, pad.y - 10.0)
        } else {
            Pt::new(pad.x, pad.y - 4.5)
        };

        let left_transitioning = self.left_raise > 0.005 && self.left_raise < 0.995;
        let right_transitioning = self.right_raise > 0.005 && self.right_raise < 0.995;
        let left_gliding =
            self.left_pose == PawPose::Resting && self.cur_left_paw.distance(palm_rest_l) > 1.5;
        let right_gliding = self.right_pose == PawPose::Resting
            && self.cur_right_paw.distance(right_rest_target) > 1.5;

        self.bounce_phase > 0.0
            || self.blinking
            || left_transitioning
            || right_transitioning
            || left_gliding
            || right_gliding
            || self.left_active_target.is_some()
            || self.right_active_target.is_some()
            || self.classic_until.is_some()
            || self.wheel_until.is_some()
            || !self.kps_timestamps.is_empty()
            || (self.cur_ptr_x - self.ptr_x).abs() > 0.002
            || (self.cur_ptr_y - self.ptr_y).abs() > 0.002
            || (self.cur_eye_x - self.eye_target_x).abs() > 0.08
            || (self.cur_eye_y - self.eye_target_y).abs() > 0.08
    }

    pub fn bounce_scale_y(&self) -> f32 {
        if !self.config.bounce || self.bounce_phase <= 0.0 {
            return 1.0;
        }
        1.0 - 0.045 * (self.bounce_phase * std::f32::consts::PI).sin()
    }

    pub fn hud_pills(&self) -> Vec<HudPill> {
        let mut pills = Vec::with_capacity(5);
        if !self.privacy_paused {
            let mut seen = HashSet::new();
            for &(visual, masked) in self.raw_to_visual.values() {
                if masked {
                    continue;
                }
                if let Some(spec) = key_spec(visual)
                    && seen.insert(spec.hud)
                {
                    pills.push(HudPill {
                        text: spec.hud.to_owned(),
                        kind: HudPillKind::Key,
                    });
                }
            }
        }
        for (btn, kind) in [
            (MouseButton::Left, HudPillKind::LeftClick),
            (MouseButton::Right, HudPillKind::RightClick),
            (MouseButton::Middle, HudPillKind::MiddleClick),
        ] {
            if self.buttons_down.contains(&btn) {
                pills.push(HudPill {
                    text: btn.hud_label().to_owned(),
                    kind,
                });
            }
        }
        if self.wheel_dir != 0 {
            pills.push(HudPill {
                text: if self.wheel_dir > 0 {
                    "Scroll ↑".to_owned()
                } else {
                    "Scroll ↓".to_owned()
                },
                kind: HudPillKind::Wheel,
            });
        }
        pills.truncate(5);
        pills
    }

    fn next_prng(&mut self) -> u32 {
        let mut x = self.blink_seed;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.blink_seed = x;
        x
    }
}

fn build_key_lookup(
    mode: DisplayMode,
    pointer: PointerDevice,
) -> (Vec<KeyLayoutCell>, HashMap<u16, usize>) {
    let (_, kb) = match mode {
        DisplayMode::Slanted => slanted_quads(pointer),
        DisplayMode::Flat | DisplayMode::Classic => flat_quads(pointer),
    };
    let cells = compute_keyboard_layout(kb);
    let mut index = HashMap::with_capacity(cells.len());
    for (i, cell) in cells.iter().enumerate() {
        index.insert(cell.spec.code, i);
    }
    (cells, index)
}

fn active_palm_rests(mode: DisplayMode, pointer: PointerDevice) -> (Pt, Pt) {
    match mode {
        DisplayMode::Slanted => slanted_palm_rests(pointer),
        DisplayMode::Flat | DisplayMode::Classic => flat_palm_rests(pointer),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::PrivacyMode;

    #[test]
    fn key_events_drive_kps_bounce_and_hud() {
        let t0 = Instant::now();
        let mut state = CatState::new(OverlayConfig::default(), t0);

        state.handle_event(
            &InputEvent::Key {
                code: 30,
                down: true,
            },
            t0,
        ); // A
        assert!(state.is_key_pressed(30));
        assert!(state.kps >= 1.0);
        assert!(state.bounce_phase > 0.5);
        assert!(state.left_active_target.is_some());
        assert_eq!(state.hud_pills()[0].text, "A");

        state.handle_event(
            &InputEvent::Key {
                code: 30,
                down: false,
            },
            t0 + Duration::from_millis(20),
        );
        assert!(!state.is_key_pressed(30));
    }

    #[test]
    fn privacy_pause_blocks_key_display() {
        let t0 = Instant::now();
        let mut state = CatState::new(OverlayConfig::default(), t0);
        state.set_privacy_paused(true);
        state.handle_event(
            &InputEvent::Key {
                code: 30,
                down: true,
            },
            t0,
        );
        assert!(!state.is_key_pressed(30));
        assert_eq!(state.hud_pills(), Vec::<HudPill>::new());
    }

    #[test]
    fn zones_mode_hides_key_label_in_hud() {
        let t0 = Instant::now();
        let cfg = OverlayConfig {
            privacy_mode: PrivacyMode::Zones,
            ..OverlayConfig::default()
        };
        let mut state = CatState::new(cfg, t0);
        state.handle_event(
            &InputEvent::Key {
                code: 30,
                down: true,
            },
            t0,
        );
        assert_eq!(state.hud_pills(), Vec::<HudPill>::new());
    }

    #[test]
    fn sleep_triggers_after_idle_timeout() {
        let t0 = Instant::now();
        let cfg = OverlayConfig {
            sleep_sec: 10,
            ..OverlayConfig::default()
        };
        let mut state = CatState::new(cfg, t0);
        assert!(!state.sleeping);

        state.tick(t0 + Duration::from_secs(11), 0.016);
        assert!(state.sleeping);
        assert_eq!(state.classic_frame, ClassicFrame::Sleeping);

        state.handle_event(
            &InputEvent::Key {
                code: 30,
                down: true,
            },
            t0 + Duration::from_secs(12),
        );
        assert!(!state.sleeping);
    }

    #[test]
    fn four_stage_posture_transitions_rest_then_both_raised() {
        let t0 = Instant::now();
        let cfg = OverlayConfig {
            idle_raise_sec: 4,
            sleep_sec: 60,
            ..OverlayConfig::default()
        };
        let mut state = CatState::new(cfg, t0);
        // Starts idle with both paws calmly raised!
        assert_eq!(state.left_pose, PawPose::Raised);
        assert_eq!(state.right_pose, PawPose::Raised);

        // Stage 1: Type with both hands ('A' left, 'J' right) -> Active
        state.handle_event(
            &InputEvent::Key {
                code: 30,
                down: true,
            },
            t0,
        );
        state.handle_event(
            &InputEvent::Key {
                code: 36,
                down: true,
            },
            t0,
        );
        assert_eq!(state.left_pose, PawPose::Active);
        assert_eq!(state.right_pose, PawPose::Active);

        // Release both keys
        let t_rel = t0 + Duration::from_millis(80);
        state.handle_event(
            &InputEvent::Key {
                code: 30,
                down: false,
            },
            t_rel,
        );
        state.handle_event(
            &InputEvent::Key {
                code: 36,
                down: false,
            },
            t_rel,
        );

        // Stage 2: At +1.5s (< 4s), both paws rest on the lower keyboard palm-rest!
        state.tick(t0 + Duration::from_millis(1500), 0.05);
        assert_eq!(state.left_pose, PawPose::Resting);
        assert_eq!(state.right_pose, PawPose::Resting);

        // Stage 3: At +4.5s (>= 4s), BOTH paws raise!
        state.tick(t0 + Duration::from_millis(4500), 0.05);
        assert_eq!(state.left_pose, PawPose::Raised);
        assert_eq!(state.right_pose, PawPose::Raised);

        // Stage 4: Sleep keeps both paws calmly raised
        state.tick(t0 + Duration::from_secs(65), 0.05);
        assert!(state.sleeping);
        assert_eq!(state.left_pose, PawPose::Raised);
        assert_eq!(state.right_pose, PawPose::Raised);
    }

    #[test]
    fn pointer_only_activity_raises_idle_left_paw_independently() {
        let t0 = Instant::now();
        let cfg = OverlayConfig {
            idle_raise_sec: 4,
            ..OverlayConfig::default()
        };
        let mut state = CatState::new(cfg, t0);

        state.handle_event(
            &InputEvent::Key {
                code: 30,
                down: true,
            },
            t0,
        );
        state.handle_event(
            &InputEvent::Key {
                code: 30,
                down: false,
            },
            t0 + Duration::from_millis(60),
        );

        // At +5s, user moves pointer while left hand has been idle > 4s
        let t5 = t0 + Duration::from_secs(5);
        state.handle_event(
            &InputEvent::PointerMove {
                x: 0.7,
                y: 0.4,
                touch: true,
                fingers: 1,
            },
            t5,
        );
        state.tick(t5 + Duration::from_millis(16), 0.016);
        assert_eq!(state.left_pose, PawPose::Raised);
        assert_eq!(state.right_pose, PawPose::Active);
    }
}
