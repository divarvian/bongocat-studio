use crate::config::{DisplayMode, PointerDevice, PointerDirection};
use crate::keymap::{KEY_ROWS, KeySpec};

/// 2D point in logical scene coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pt {
    pub x: f32,
    pub y: f32,
}

impl Pt {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn lerp(self, other: Self, factor: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * factor,
            y: self.y + (other.y - self.y) * factor,
        }
    }

    pub fn distance(self, other: Self) -> f32 {
        (other.x - self.x).hypot(other.y - self.y)
    }
}

/// Perspective quadrilateral defined in clockwise order: top-left, top-right, bottom-right, bottom-left.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad {
    pub tl: Pt,
    pub tr: Pt,
    pub br: Pt,
    pub bl: Pt,
}

impl Quad {
    pub fn point(&self, u: f32, v: f32) -> Pt {
        let tx = self.tl.x + (self.tr.x - self.tl.x) * u;
        let ty = self.tl.y + (self.tr.y - self.tl.y) * u;
        let bx = self.bl.x + (self.br.x - self.bl.x) * u;
        let by = self.bl.y + (self.br.y - self.bl.y) * u;
        Pt::new(tx + (bx - tx) * v, ty + (by - ty) * v)
    }

    pub fn sub_quad(&self, u0: f32, v0: f32, u1: f32, v1: f32) -> Self {
        Self {
            tl: self.point(u0, v0),
            tr: self.point(u1, v0),
            br: self.point(u1, v1),
            bl: self.point(u0, v1),
        }
    }

    pub const fn corners(&self) -> [Pt; 4] {
        [self.tl, self.tr, self.br, self.bl]
    }
}

/// Seamless curved Bongo Cat paw spline with anatomically fixed shoulder anchors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArmSpline {
    pub s_outer: Pt,
    pub c_outer: Pt,
    pub t_outer: Pt,
    pub tip: Pt,
    pub t_inner: Pt,
    pub c_inner: Pt,
    pub s_inner: Pt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenSide {
    Left,
    Right,
}

pub fn build_seamless_arm(
    s_outer: Pt,
    s_inner: Pt,
    target: Pt,
    side: ScreenSide,
    tip_half: f32,
) -> ArmSpline {
    let sx = f32::midpoint(s_outer.x, s_inner.x);
    let sy = f32::midpoint(s_outer.y, s_inner.y);
    let dx = target.x - sx;
    let dy = target.y - sy;
    let dist = dx.hypot(dy).max(1.0);
    let ux = dx / dist;
    let uy = dy / dist;
    let nx = -uy;
    let ny = ux;

    let out_n = match side {
        ScreenSide::Left => 1.0,
        ScreenSide::Right => -1.0,
    };

    let t_outer = Pt::new(
        target.x + nx * out_n * tip_half,
        target.y + ny * out_n * tip_half,
    );
    let t_inner = Pt::new(
        target.x - nx * out_n * tip_half,
        target.y - ny * out_n * tip_half,
    );

    let bow_out = (dist * 0.13).min(18.0);
    let bow_in = (dist * 0.06).min(8.0);
    let c_outer = Pt::new(
        f32::midpoint(s_outer.x, t_outer.x) + nx * out_n * bow_out,
        f32::midpoint(s_outer.y, t_outer.y) + ny * out_n * bow_out,
    );
    let c_inner = Pt::new(
        f32::midpoint(s_inner.x, t_inner.x) - nx * out_n * bow_in,
        f32::midpoint(s_inner.y, t_inner.y) - ny * out_n * bow_in,
    );

    let tip = Pt::new(
        target.x + ux * (tip_half * 1.18),
        target.y + uy * (tip_half * 1.18),
    );

    ArmSpline {
        s_outer,
        c_outer,
        t_outer,
        tip,
        t_inner,
        c_inner,
        s_inner,
    }
}

// ============================================================================
// Iconic Slanted Mode (~10° diagonal, viewBox 640x400)
// ============================================================================
pub const SLANTED_WIDTH: f32 = 640.0;
pub const SLANTED_HEIGHT: f32 = 400.0;
pub const SLANTED_DESK_Y0: f32 = 136.0;
pub const SLANTED_DESK_SLOPE: f32 = 0.175;

pub const SLANTED_LAPTOP_QUAD: Quad = Quad {
    tl: Pt::new(154.0, 166.0),
    tr: Pt::new(518.0, 230.0),
    br: Pt::new(492.0, 388.0),
    bl: Pt::new(92.0, 318.0),
};

pub const SLANTED_MOUSE_PAD_QUAD: Quad = Quad {
    tl: Pt::new(122.0, 162.0),
    tr: Pt::new(248.0, 184.0),
    br: Pt::new(188.0, 294.0),
    bl: Pt::new(54.0, 270.0),
};

pub const SLANTED_MOUSE_KB_QUAD: Quad = Quad {
    tl: Pt::new(262.0, 188.0),
    tr: Pt::new(586.0, 245.0),
    br: Pt::new(538.0, 372.0),
    bl: Pt::new(192.0, 311.0),
};

pub fn slanted_cat_shift(pointer: PointerDevice) -> Pt {
    let dx = match pointer {
        PointerDevice::Trackpad => 0.0,
        PointerDevice::Mouse => 42.0,
    };
    Pt::new(dx, dx * SLANTED_DESK_SLOPE)
}

pub fn slanted_quads(pointer: PointerDevice) -> (Quad, Quad) {
    match pointer {
        PointerDevice::Trackpad => (
            SLANTED_LAPTOP_QUAD.sub_quad(0.28, 0.04, 0.72, 0.31),
            SLANTED_LAPTOP_QUAD.sub_quad(0.04, 0.35, 0.96, 0.95),
        ),
        PointerDevice::Mouse => (SLANTED_MOUSE_PAD_QUAD, SLANTED_MOUSE_KB_QUAD),
    }
}

pub fn slanted_shoulders(pointer: PointerDevice) -> ((Pt, Pt), (Pt, Pt)) {
    let shift = slanted_cat_shift(pointer);
    let right_arm = (
        Pt::new(170.0 + shift.x, 156.0 + shift.y),
        Pt::new(230.0 + shift.x, 168.0 + shift.y),
    );
    let left_arm = (
        Pt::new(468.0 + shift.x, 198.0 + shift.y),
        Pt::new(412.0 + shift.x, 186.0 + shift.y),
    );
    (right_arm, left_arm)
}

/// Natural palm-rest standby points `(right_hand_screen_left, left_hand_screen_right)`
/// where paws rest briefly along the lower keyboard / laptop palm-rest before raising.
pub fn slanted_palm_rests(pointer: PointerDevice) -> (Pt, Pt) {
    match pointer {
        PointerDevice::Trackpad => (
            SLANTED_LAPTOP_QUAD.point(0.15, 0.22),
            SLANTED_LAPTOP_QUAD.point(0.85, 0.22),
        ),
        PointerDevice::Mouse => (
            SLANTED_MOUSE_KB_QUAD.point(0.24, 0.07),
            SLANTED_MOUSE_KB_QUAD.point(0.78, 0.07),
        ),
    }
}

// ============================================================================
// Flat Desk Mode (top-down cat POV, viewBox 640x380)
// ============================================================================
pub const FLAT_WIDTH: f32 = 640.0;
pub const FLAT_HEIGHT: f32 = 380.0;
pub const FLAT_DESK_Y: f32 = 166.0;

pub const FLAT_LAPTOP_CHASSIS: Quad = Quad {
    tl: Pt::new(112.0, 174.0),
    tr: Pt::new(528.0, 174.0),
    br: Pt::new(528.0, 370.0),
    bl: Pt::new(112.0, 370.0),
};

pub const FLAT_TRACKPAD_QUAD: Quad = Quad {
    tl: Pt::new(238.0, 182.0),
    tr: Pt::new(402.0, 182.0),
    br: Pt::new(402.0, 232.0),
    bl: Pt::new(238.0, 232.0),
};

pub const FLAT_LAPTOP_KB_QUAD: Quad = Quad {
    tl: Pt::new(124.0, 240.0),
    tr: Pt::new(516.0, 240.0),
    br: Pt::new(516.0, 358.0),
    bl: Pt::new(124.0, 358.0),
};

pub const FLAT_MOUSE_PAD_QUAD: Quad = Quad {
    tl: Pt::new(28.0, 196.0),
    tr: Pt::new(172.0, 196.0),
    br: Pt::new(172.0, 336.0),
    bl: Pt::new(28.0, 336.0),
};

pub const FLAT_MOUSE_KB_QUAD: Quad = Quad {
    tl: Pt::new(190.0, 200.0),
    tr: Pt::new(610.0, 200.0),
    br: Pt::new(610.0, 336.0),
    bl: Pt::new(190.0, 336.0),
};

pub fn flat_cat_shift_x(pointer: PointerDevice) -> f32 {
    match pointer {
        PointerDevice::Trackpad => 0.0,
        PointerDevice::Mouse => 62.0,
    }
}

pub fn flat_quads(pointer: PointerDevice) -> (Quad, Quad) {
    match pointer {
        PointerDevice::Trackpad => (FLAT_TRACKPAD_QUAD, FLAT_LAPTOP_KB_QUAD),
        PointerDevice::Mouse => (FLAT_MOUSE_PAD_QUAD, FLAT_MOUSE_KB_QUAD),
    }
}

pub fn flat_shoulders(pointer: PointerDevice) -> ((Pt, Pt), (Pt, Pt)) {
    let sx = flat_cat_shift_x(pointer);
    let right_arm = (Pt::new(212.0 + sx, 160.0), Pt::new(260.0 + sx, 160.0));
    let left_arm = (Pt::new(428.0 + sx, 160.0), Pt::new(380.0 + sx, 160.0));
    (right_arm, left_arm)
}

/// Natural palm-rest standby points `(right_hand_screen_left, left_hand_screen_right)` for Flat Desk.
pub fn flat_palm_rests(pointer: PointerDevice) -> (Pt, Pt) {
    match pointer {
        PointerDevice::Trackpad => (Pt::new(176.0, 208.0), Pt::new(464.0, 208.0)),
        PointerDevice::Mouse => (Pt::new(316.0, 204.0), Pt::new(492.0, 204.0)),
    }
}

/// Precomputed keycap polygon & paw hit targets for a given mode & pointer layout.
#[derive(Debug, Clone, Copy)]
pub struct KeyLayoutCell {
    pub spec: &'static KeySpec,
    pub quad: Quad,
    pub center: Pt,
    pub left_thumb: Pt,
    pub right_thumb: Pt,
}

/// Computes all 63 keycap polygons oriented toward the cat's POV (Spacebar closest to cat,
/// Esc/Numbers furthest from cat, left-hand keys on screen-right, right-hand keys on screen-left).
pub fn compute_keyboard_layout(kb_quad: Quad) -> Vec<KeyLayoutCell> {
    let pad_u = 0.022_f32;
    let pad_v = 0.045_f32;
    let gap_u = 0.0055_f32;
    let gap_v = 0.022_f32;
    let usable_u = 1.0 - pad_u * 2.0;
    let usable_v = 1.0 - pad_v * 2.0;
    let row_v = (usable_v - gap_v * 4.0) / 5.0;
    let unit_u = (usable_u - gap_u * 14.0) / 15.0;

    let mut cells = Vec::with_capacity(63);

    for (r_idx, row) in KEY_ROWS.iter().rev().enumerate() {
        let v0 = pad_v + (r_idx as f32) * (row_v + gap_v);
        let v1 = v0 + row_v;
        let vc = f32::midpoint(v0, v1);
        let mut u_cur = pad_u;

        for spec in row.iter().rev() {
            let u0 = u_cur;
            let u1 = u0 + spec.width * unit_u + (spec.width - 1.0) * gap_u;
            let quad = kb_quad.sub_quad(u0, v0, u1, v1);
            let center = kb_quad.point(f32::midpoint(u0, u1), vc);
            let left_thumb = kb_quad.point(u0 + (u1 - u0) * 0.68, vc);
            let right_thumb = kb_quad.point(u0 + (u1 - u0) * 0.32, vc);

            cells.push(KeyLayoutCell {
                spec,
                quad,
                center,
                left_thumb,
                right_thumb,
            });
            u_cur = u1 + gap_u;
        }
    }

    cells
}

/// Maps normalized pointer `(0..1, 0..1)` to logical scene coordinates on the active pad.
pub fn pointer_pad_point(
    mode: DisplayMode,
    pointer: PointerDevice,
    dir: PointerDirection,
    nx: f32,
    ny: f32,
) -> Pt {
    let natural = dir == PointerDirection::Natural;
    let fx = if natural { 1.0 - nx } else { nx };
    let fy = if natural { 1.0 - ny } else { ny };

    match mode {
        DisplayMode::Slanted => {
            let (pad, _) = slanted_quads(pointer);
            let (u, v) = match pointer {
                PointerDevice::Mouse => (0.24 + fx * 0.56, 0.22 + fy * 0.50),
                PointerDevice::Trackpad => (0.16 + fx * 0.68, 0.34 + fy * 0.52),
            };
            pad.point(u, v)
        }
        DisplayMode::Flat | DisplayMode::Classic => {
            let (pad, _) = flat_quads(pointer);
            let (u, v) = match pointer {
                PointerDevice::Mouse => (0.22 + fx * 0.56, 0.22 + fy * 0.56),
                PointerDevice::Trackpad => (0.14 + fx * 0.72, 0.26 + fy * 0.58),
            };
            pad.point(u, v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyboard_layout_has_63_cells_inside_canvas() {
        let (_, kb) = slanted_quads(PointerDevice::Trackpad);
        let cells = compute_keyboard_layout(kb);
        assert_eq!(cells.len(), 63);
        for cell in &cells {
            assert!(cell.center.x > 0.0 && cell.center.x < SLANTED_WIDTH);
            assert!(cell.center.y > 0.0 && cell.center.y < SLANTED_HEIGHT);
        }
    }

    #[test]
    fn slanted_cat_stays_centered_over_keyboard_in_mouse_mode() {
        let (_, kb) = slanted_quads(PointerDevice::Mouse);
        let kb_center = kb.point(0.5, 0.5);
        let shift = slanted_cat_shift(PointerDevice::Mouse);
        let cat_center_x = 340.0 + shift.x;
        assert!((cat_center_x - kb_center.x).abs() < 16.0);
    }

    #[test]
    fn slanted_shoulders_sit_above_desk_line() {
        for ptr in PointerDevice::ALL {
            let ((r_out, r_in), (l_out, l_in)) = slanted_shoulders(ptr);
            for pt in [r_out, r_in, l_out, l_in] {
                let desk_y = SLANTED_DESK_Y0 + SLANTED_DESK_SLOPE * pt.x;
                assert!(pt.y < desk_y, "shoulder {pt:?} below desk y={desk_y}");
            }
        }
    }
}
