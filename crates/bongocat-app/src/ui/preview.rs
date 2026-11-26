use bongocat_core::geometry::{
    ArmSpline, FLAT_DESK_Y, FLAT_HEIGHT, FLAT_LAPTOP_CHASSIS, FLAT_WIDTH, SLANTED_DESK_SLOPE,
    SLANTED_DESK_Y0, SLANTED_HEIGHT, SLANTED_LAPTOP_QUAD, SLANTED_WIDTH, ScreenSide,
    build_seamless_arm, flat_cat_shift_x, flat_quads, flat_shoulders, pointer_pad_point,
    slanted_cat_shift, slanted_quads, slanted_shoulders,
};
use bongocat_core::{
    CatState, ClassicFrame, DeskTheme, DisplayMode, EyeStyle, HudPillKind, MouseButton,
    PointerDevice, PrivacyMode, Pt, Quad,
};
use iced::widget::canvas::{self, Frame, Geometry, LineCap, LineJoin, Path, Stroke, Text};
use iced::widget::{Canvas, Space, column, container, row, svg, text};
use iced::{Alignment, Color, Element, Length, Point, Rectangle, Renderer, Size, Theme, mouse};

use crate::app::Message;
use crate::ui::theme::{StudioPalette, badge_style, card_style};

const CLASSIC_BOTH_UP: &[u8] = include_bytes!("../../../../assets/svg/bongo-both-up.svg");
const CLASSIC_LEFT_DOWN: &[u8] = include_bytes!("../../../../assets/svg/bongo-left-down.svg");
const CLASSIC_RIGHT_DOWN: &[u8] = include_bytes!("../../../../assets/svg/bongo-right-down.svg");
const CLASSIC_BOTH_DOWN: &[u8] = include_bytes!("../../../../assets/svg/bongo-both-down.svg");
const CLASSIC_SLEEPING: &[u8] = include_bytes!("../../../../assets/svg/bongo-sleeping.svg");

const INK: Color = Color::from_rgb(0.067, 0.094, 0.153);
const WHITE: Color = Color::WHITE;
const BLUSH: Color = Color::from_rgba(0.976, 0.659, 0.831, 0.50);
const BEAN_PINK: Color = Color::from_rgba(0.984, 0.443, 0.522, 0.82);
const ACCENT: Color = Color::from_rgb(0.220, 0.741, 0.973);
const RIGHT_ACCENT: Color = Color::from_rgb(0.984, 0.443, 0.522);
const MIDDLE_ACCENT: Color = Color::from_rgb(0.655, 0.545, 0.980);
const WHEEL_ACCENT: Color = Color::from_rgb(0.204, 0.827, 0.600);

pub fn view_preview<'a>(
    palette: StudioPalette,
    cat: &'a CatState,
    show_checkerboard: bool,
) -> Element<'a, Message> {
    let stage: Element<'a, Message> = match cat.config.mode {
        DisplayMode::Classic => view_classic_stage(palette, cat, show_checkerboard),
        DisplayMode::Slanted | DisplayMode::Flat => Canvas::new(DeskCanvasProgram {
            cat,
            palette,
            show_checkerboard,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into(),
    };

    let mut preview_col = column![container(stage).width(Length::Fill).height(Length::Fill)]
        .spacing(8)
        .align_x(Alignment::Center);

    if cat.config.show_kps || (cat.config.show_hud && cat.config.mode != DisplayMode::Classic) {
        preview_col = preview_col.push(view_hud_strip(palette, cat));
    }

    container(preview_col)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding([12, 14])
        .style(card_style(palette))
        .into()
}

fn view_classic_stage(
    palette: StudioPalette,
    cat: &CatState,
    show_checkerboard: bool,
) -> Element<'_, Message> {
    let bytes = match cat.classic_frame {
        ClassicFrame::BothUp => CLASSIC_BOTH_UP,
        ClassicFrame::LeftDown => CLASSIC_LEFT_DOWN,
        ClassicFrame::RightDown => CLASSIC_RIGHT_DOWN,
        ClassicFrame::BothDown => CLASSIC_BOTH_DOWN,
        ClassicFrame::Sleeping => CLASSIC_SLEEPING,
    };

    let bounce_pad = ((1.0 - cat.bounce_scale_y()) * 140.0).max(0.0);
    let handle = svg::Handle::from_memory(bytes);
    let cat_svg = svg(handle)
        .width(Length::Fill)
        .height(Length::Fill)
        .content_fit(iced::ContentFit::Contain);

    let bg = if show_checkerboard {
        palette.checker_a
    } else {
        palette.surface_alt
    };

    container(
        column![Space::with_height(Length::Fixed(bounce_pad)), cat_svg,]
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding([24, 36])
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(move |_| iced::widget::container::Style {
        background: Some(iced::Background::Color(bg)),
        border: iced::Border {
            color: palette.border,
            width: 1.0,
            radius: iced::border::Radius::new(10.0),
        },
        ..iced::widget::container::Style::default()
    })
    .into()
}

fn view_hud_strip(palette: StudioPalette, cat: &CatState) -> Element<'_, Message> {
    let mut strip = row![].spacing(8).align_y(Alignment::Center);

    if cat.config.show_kps {
        let kps_text = if cat.privacy_paused {
            "PAUSED".to_owned()
        } else {
            format!("{} KPS", cat.kps.round() as u32)
        };
        let dot_color = if cat.privacy_paused {
            palette.warning
        } else {
            palette.success
        };
        let fill_w = (cat.kps / 12.0).clamp(0.0, 1.0) * 48.0;

        let meter = container(
            container(Space::with_width(Length::Fixed(fill_w)))
                .height(Length::Fixed(6.0))
                .style(move |_| iced::widget::container::Style {
                    background: Some(iced::Background::Color(ACCENT)),
                    border: iced::Border {
                        radius: iced::border::Radius::new(999.0),
                        ..iced::Border::default()
                    },
                    ..iced::widget::container::Style::default()
                }),
        )
        .width(Length::Fixed(48.0))
        .height(Length::Fixed(6.0))
        .style(move |_| iced::widget::container::Style {
            background: Some(iced::Background::Color(Color::from_rgba(
                1.0, 1.0, 1.0, 0.16,
            ))),
            border: iced::Border {
                radius: iced::border::Radius::new(999.0),
                ..iced::Border::default()
            },
            ..iced::widget::container::Style::default()
        });

        let kps_pill = container(
            row![
                text("●").size(10).color(dot_color),
                text(kps_text)
                    .size(12.5)
                    .font(iced::Font::MONOSPACE)
                    .color(WHITE),
                meter,
            ]
            .spacing(7)
            .align_y(Alignment::Center),
        )
        .padding([5, 11])
        .style(badge_style(
            Color::from_rgba(0.059, 0.071, 0.098, 0.88),
            Color::from_rgba(1.0, 1.0, 1.0, 0.18),
            WHITE,
        ));
        strip = strip.push(kps_pill);
    }

    if cat.config.show_hud && cat.config.mode != DisplayMode::Classic {
        for pill in cat.hud_pills() {
            let border = match pill.kind {
                HudPillKind::Key | HudPillKind::LeftClick => ACCENT,
                HudPillKind::RightClick => RIGHT_ACCENT,
                HudPillKind::MiddleClick => MIDDLE_ACCENT,
                HudPillKind::Wheel => WHEEL_ACCENT,
            };
            strip = strip.push(
                container(
                    text(pill.text)
                        .size(12)
                        .font(iced::Font::MONOSPACE)
                        .color(WHITE),
                )
                .padding([4, 9])
                .style(badge_style(
                    Color::from_rgba(0.059, 0.071, 0.098, 0.88),
                    border,
                    WHITE,
                )),
            );
        }
    }

    container(strip)
        .width(Length::Fill)
        .height(Length::Fixed(32.0))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

struct DeskCanvasProgram<'a> {
    cat: &'a CatState,
    palette: StudioPalette,
    show_checkerboard: bool,
}

impl canvas::Program<Message> for DeskCanvasProgram<'_> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());

        // 1. Subtle checkerboard backdrop representing OBS alpha transparency
        draw_backdrop(
            &mut frame,
            bounds.size(),
            self.palette,
            self.show_checkerboard,
        );

        // 2. Fit logical scene viewBox neatly inside preview bounds with subtle bounce
        let (scene_w, scene_h) = match self.cat.config.mode {
            DisplayMode::Slanted => (SLANTED_WIDTH, SLANTED_HEIGHT),
            DisplayMode::Flat | DisplayMode::Classic => (FLAT_WIDTH, FLAT_HEIGHT),
        };

        let pad = 10.0;
        let avail_w = (bounds.width - pad * 2.0).max(10.0);
        let avail_h = (bounds.height - pad * 2.0).max(10.0);
        let scale = (avail_w / scene_w).min(avail_h / scene_h);
        let bounce_y = self.cat.bounce_scale_y();
        let bounce_shift_y = (1.0 - bounce_y) * scene_h * 0.14;

        let offset_x = (bounds.width - scene_w * scale) * 0.5;
        let offset_y = (bounds.height - scene_h * scale) * 0.5 + bounce_shift_y * scale;

        frame.with_save(|f| {
            f.translate(iced::Vector::new(offset_x, offset_y));
            f.scale_nonuniform(iced::Vector::new(scale, scale * bounce_y));

            match self.cat.config.mode {
                DisplayMode::Slanted => draw_slanted_scene(f, self.cat),
                DisplayMode::Flat | DisplayMode::Classic => draw_flat_scene(f, self.cat),
            }
        });

        vec![frame.into_geometry()]
    }
}

fn draw_backdrop(frame: &mut Frame, size: Size, palette: StudioPalette, checker: bool) {
    let bg_rect = Path::rounded_rectangle(Point::ORIGIN, size, 10.0.into());
    frame.fill(&bg_rect, palette.checker_a);

    if checker {
        let step = 18.0;
        let cols = (size.width / step).ceil() as usize;
        let rows = (size.height / step).ceil() as usize;
        for r in 0..rows {
            for c in 0..cols {
                if (r + c) % 2 == 1 {
                    let x = c as f32 * step;
                    let y = r as f32 * step;
                    let w = step.min(size.width - x);
                    let h = step.min(size.height - y);
                    frame.fill_rectangle(Point::new(x, y), Size::new(w, h), palette.checker_b);
                }
            }
        }
    }

    frame.stroke(
        &bg_rect,
        Stroke::default().with_color(palette.border).with_width(1.0),
    );
}

// ============================================================================
// Iconic Slanted Mode (~10° diagonal) Native Rust Vector Renderer
// ============================================================================
fn draw_slanted_scene(frame: &mut Frame, cat: &CatState) {
    let pointer = cat.config.pointer;
    let is_mouse = pointer == PointerDevice::Mouse;
    let is_dark = cat.config.desk_theme == DeskTheme::Dark;
    let shift = slanted_cat_shift(pointer);
    let (pad_quad, kb_quad) = slanted_quads(pointer);

    let y_left = SLANTED_DESK_Y0;
    let y_right = SLANTED_DESK_Y0 + SLANTED_DESK_SLOPE * SLANTED_WIDTH;

    // 1. Iconic chubby slanted Bongo Cat silhouette & face
    frame.with_save(|f| {
        f.translate(iced::Vector::new(shift.x, shift.y));
        draw_slanted_cat_body(f, cat);
    });

    // 2. Diagonal desk tabletop surface (covers lower torso cleanly along the slope!)
    if cat.config.desk_theme != DeskTheme::Transparent {
        let desk_fill = match cat.config.desk_theme {
            DeskTheme::Dark => Color::from_rgb(0.094, 0.118, 0.169),
            DeskTheme::Pastel => Color::from_rgb(0.992, 0.949, 0.973),
            DeskTheme::White | DeskTheme::Transparent => WHITE,
        };
        let desk_poly = Path::new(|b| {
            b.move_to(Point::new(0.0, y_left));
            b.line_to(Point::new(SLANTED_WIDTH, y_right));
            b.line_to(Point::new(SLANTED_WIDTH, SLANTED_HEIGHT));
            b.line_to(Point::new(0.0, SLANTED_HEIGHT));
            b.close();
        });
        frame.fill(&desk_poly, desk_fill);
    }

    let desk_line = Path::line(Point::new(0.0, y_left), Point::new(SLANTED_WIDTH, y_right));
    frame.stroke(
        &desk_line,
        Stroke::default()
            .with_color(INK)
            .with_width(5.8)
            .with_line_cap(LineCap::Round),
    );

    // 3. Unified Slanted Laptop Chassis (in Trackpad mode)
    if !is_mouse {
        draw_quad_bevel(
            frame,
            SLANTED_LAPTOP_QUAD,
            7.0,
            if is_dark {
                Color::from_rgb(0.059, 0.090, 0.165)
            } else {
                Color::from_rgb(0.553, 0.588, 0.639)
            },
        );
        let chassis_fill = match cat.config.desk_theme {
            DeskTheme::Dark => Color::from_rgb(0.165, 0.212, 0.294),
            DeskTheme::Pastel => Color::from_rgb(0.969, 0.922, 0.996),
            DeskTheme::White | DeskTheme::Transparent => Color::from_rgb(0.831, 0.859, 0.902),
        };
        fill_and_stroke_quad(frame, SLANTED_LAPTOP_QUAD, chassis_fill, INK, 3.2);
        let hinge = SLANTED_LAPTOP_QUAD.sub_quad(0.16, 0.965, 0.84, 1.0);
        fill_quad(frame, hinge, Color::from_rgb(0.118, 0.161, 0.231));
    }

    // 4. Slanted Trackpad (centered below Spacebar) OR Tucked Slanted Mousepad + Mouse
    let pad_pt = pointer_pad_point(
        DisplayMode::Slanted,
        pointer,
        cat.config.ptr_dir,
        cat.cur_ptr_x,
        cat.cur_ptr_y,
    );
    if is_mouse {
        draw_slanted_mousepad(frame, pad_quad, pad_pt, cat, is_dark);
    } else {
        draw_slanted_trackpad(frame, pad_quad, pad_pt, cat, is_dark);
    }

    // 5. Full 63-key 3D Perspective QWERTY Keyboard
    draw_perspective_keyboard(frame, kb_quad, cat, is_dark, is_mouse);

    // 6. Anatomically Fixed Seamless Chubby Paws (Active -> Resting on Palm-Rest -> Raised Both!)
    let ((r_out, r_in), (l_out, l_in)) = slanted_shoulders(pointer);

    let r_arm_alpha = (1.0 - cat.right_raise).clamp(0.0, 1.0);
    if r_arm_alpha > 0.02 {
        let right_arm = build_seamless_arm(r_out, r_in, cat.cur_right_paw, ScreenSide::Left, 20.0);
        draw_arm_spline(frame, right_arm, 5.4, r_arm_alpha);
    }
    if cat.right_raise > 0.02 {
        let lift = (1.0 - cat.right_raise) * 14.0;
        frame.with_save(|f| {
            f.translate(iced::Vector::new(shift.x, shift.y + lift));
            draw_slanted_raised_right_paw(f, cat.right_raise);
        });
    }

    let l_arm_alpha = (1.0 - cat.left_raise).clamp(0.0, 1.0);
    if l_arm_alpha > 0.02 {
        let left_arm = build_seamless_arm(l_out, l_in, cat.cur_left_paw, ScreenSide::Right, 19.5);
        draw_arm_spline(frame, left_arm, 5.4, l_arm_alpha);
    }
    if cat.left_raise > 0.02 {
        let lift = (1.0 - cat.left_raise) * 14.0;
        frame.with_save(|f| {
            f.translate(iced::Vector::new(shift.x, shift.y + lift));
            draw_slanted_raised_left_paw(f, cat.left_raise);
        });
    }
}

fn draw_slanted_cat_body(frame: &mut Frame, cat: &CatState) {
    let body = Path::new(|b| {
        b.move_to(Point::new(156.0, 178.0));
        b.bezier_curve_to(
            Point::new(162.0, 106.0),
            Point::new(218.0, 54.0),
            Point::new(276.0, 40.0),
        );
        b.bezier_curve_to(
            Point::new(286.0, 22.0),
            Point::new(298.0, 10.0),
            Point::new(308.0, 9.0),
        );
        b.bezier_curve_to(
            Point::new(318.0, 11.0),
            Point::new(325.0, 31.0),
            Point::new(329.0, 44.0),
        );
        b.bezier_curve_to(
            Point::new(374.0, 51.0),
            Point::new(416.0, 68.0),
            Point::new(452.0, 91.0),
        );
        b.bezier_curve_to(
            Point::new(471.0, 80.0),
            Point::new(490.0, 73.0),
            Point::new(499.0, 76.0),
        );
        b.bezier_curve_to(
            Point::new(504.0, 86.0),
            Point::new(497.0, 105.0),
            Point::new(488.0, 119.0),
        );
        b.bezier_curve_to(
            Point::new(506.0, 152.0),
            Point::new(518.0, 196.0),
            Point::new(524.0, 244.0),
        );
        b.line_to(Point::new(150.0, 182.0));
        b.close();
    });
    frame.fill(&body, WHITE);
    frame.stroke(
        &body,
        Stroke::default()
            .with_color(INK)
            .with_width(5.8)
            .with_line_cap(LineCap::Round)
            .with_line_join(LineJoin::Round),
    );

    // Tilted 10° face group with Live2D face parallax + eye tracking!
    frame.with_save(|f| {
        f.translate(iced::Vector::new(326.0, 138.0));
        f.rotate(10.0_f32.to_radians());

        let (fx, fy, px, py) = if cat.config.eyes_enabled && !cat.sleeping {
            (
                cat.cur_eye_x * 0.42,
                cat.cur_eye_y * 0.36,
                cat.cur_eye_x * 0.58,
                cat.cur_eye_y * 0.58,
            )
        } else {
            (0.0, 0.0, 0.0, 0.0)
        };
        f.translate(iced::Vector::new(fx, fy));

        // Cheek blushes
        fill_ellipse(f, -78.0, 14.0, 13.5, 7.2, BLUSH);
        fill_ellipse(f, 70.0, 14.0, 13.5, 7.2, BLUSH);

        // Eyes (Awake / Blink / Sleep)
        if cat.sleeping {
            let sleep_stroke = Stroke::default()
                .with_color(INK)
                .with_width(4.5)
                .with_line_cap(LineCap::Round);
            let e_l = Path::new(|b| {
                b.move_to(Point::new(-72.0, -2.0));
                b.quadratic_curve_to(Point::new(-60.0, 9.0), Point::new(-48.0, -2.0));
            });
            let e_r = Path::new(|b| {
                b.move_to(Point::new(38.0, 0.0));
                b.quadratic_curve_to(Point::new(50.0, 11.0), Point::new(62.0, 0.0));
            });
            f.stroke(&e_l, sleep_stroke);
            f.stroke(&e_r, sleep_stroke);
            f.fill_text(Text {
                content: "Z z".to_owned(),
                position: Point::new(118.0, -76.0),
                color: ACCENT,
                size: 22.0.into(),
                ..Text::default()
            });
        } else {
            draw_cat_eye(f, -60.0, -2.0, px, py, cat.config.eye_style, cat.blinking);
            draw_cat_eye(f, 50.0, 0.0, px, py, cat.config.eye_style, cat.blinking);
        }

        // Classic cute ω mouth
        let mouth = Path::new(|b| {
            b.move_to(Point::new(-31.0, 10.0));
            b.bezier_curve_to(
                Point::new(-28.0, 23.0),
                Point::new(-11.0, 21.0),
                Point::new(-6.0, 10.0),
            );
            b.bezier_curve_to(
                Point::new(-1.0, 21.0),
                Point::new(16.0, 23.0),
                Point::new(19.0, 10.0),
            );
        });
        frame_stroke_round(f, &mouth, INK, 5.0);

        if cat.fast_typing && !cat.sleeping {
            let smile = Path::new(|b| {
                b.move_to(Point::new(-15.0, 17.0));
                b.quadratic_curve_to(Point::new(-6.0, 30.0), Point::new(3.0, 17.0));
                b.close();
            });
            f.fill(&smile, RIGHT_ACCENT);
            frame_stroke_round(f, &smile, INK, 2.5);
        }
    });
}

fn draw_slanted_raised_right_paw(frame: &mut Frame, alpha: f32) {
    let fill = Color { a: alpha, ..WHITE };
    let ink = Color { a: alpha, ..INK };
    let pink = Color {
        a: BEAN_PINK.a * alpha,
        ..BEAN_PINK
    };

    let paw_fill = Path::new(|b| {
        b.move_to(Point::new(158.0, 164.0));
        b.bezier_curve_to(
            Point::new(154.0, 114.0),
            Point::new(176.0, 76.0),
            Point::new(204.0, 82.0),
        );
        b.bezier_curve_to(
            Point::new(232.0, 88.0),
            Point::new(236.0, 130.0),
            Point::new(224.0, 176.0),
        );
        b.close();
    });
    let paw_open = Path::new(|b| {
        b.move_to(Point::new(158.0, 164.0));
        b.bezier_curve_to(
            Point::new(154.0, 114.0),
            Point::new(176.0, 76.0),
            Point::new(204.0, 82.0),
        );
        b.bezier_curve_to(
            Point::new(232.0, 88.0),
            Point::new(236.0, 130.0),
            Point::new(224.0, 176.0),
        );
    });
    frame.fill(&paw_fill, fill);
    frame_stroke_round(frame, &paw_open, ink, 5.4);

    // 4 adorable pink paw beans mirroring the left raised paw
    fill_ellipse(frame, 194.0, 138.0, 12.0, 15.0, pink);
    fill_ellipse(frame, 177.0, 114.0, 5.5, 6.6, pink);
    fill_ellipse(frame, 195.0, 104.0, 5.8, 7.0, pink);
    fill_ellipse(frame, 212.0, 116.0, 5.8, 7.0, pink);
}

fn draw_slanted_raised_left_paw(frame: &mut Frame, alpha: f32) {
    let fill = Color { a: alpha, ..WHITE };
    let ink = Color { a: alpha, ..INK };
    let pink = Color {
        a: BEAN_PINK.a * alpha,
        ..BEAN_PINK
    };

    let paw_fill = Path::new(|b| {
        b.move_to(Point::new(404.0, 205.0));
        b.bezier_curve_to(
            Point::new(392.0, 158.0),
            Point::new(405.0, 116.0),
            Point::new(434.0, 120.0),
        );
        b.bezier_curve_to(
            Point::new(462.0, 124.0),
            Point::new(478.0, 166.0),
            Point::new(472.0, 218.0),
        );
        b.close();
    });
    let paw_open = Path::new(|b| {
        b.move_to(Point::new(404.0, 205.0));
        b.bezier_curve_to(
            Point::new(392.0, 158.0),
            Point::new(405.0, 116.0),
            Point::new(434.0, 120.0),
        );
        b.bezier_curve_to(
            Point::new(462.0, 124.0),
            Point::new(478.0, 166.0),
            Point::new(472.0, 218.0),
        );
    });
    frame.fill(&paw_fill, fill);
    frame_stroke_round(frame, &paw_open, ink, 5.4);

    // 4 adorable pink paw beans
    fill_ellipse(frame, 437.0, 180.0, 12.0, 15.0, pink);
    fill_ellipse(frame, 416.0, 158.0, 5.8, 7.0, pink);
    fill_ellipse(frame, 430.0, 144.0, 5.8, 7.0, pink);
    fill_ellipse(frame, 446.0, 152.0, 5.5, 6.6, pink);
}

fn draw_slanted_trackpad(frame: &mut Frame, q: Quad, pad_pt: Pt, cat: &CatState, is_dark: bool) {
    let pad_fill = if is_dark {
        Color::from_rgb(0.220, 0.275, 0.365)
    } else {
        Color::from_rgb(0.733, 0.765, 0.808)
    };
    fill_and_stroke_quad(
        frame,
        q,
        pad_fill,
        if is_dark {
            Color::from_rgb(0.012, 0.024, 0.090)
        } else {
            Color::from_rgb(0.392, 0.455, 0.545)
        },
        2.2,
    );

    let left_down = cat.buttons_down.contains(&MouseButton::Left)
        || cat.buttons_down.contains(&MouseButton::Middle);
    let right_down = cat.buttons_down.contains(&MouseButton::Right)
        || cat.buttons_down.contains(&MouseButton::Middle);

    // Cat POV: screen-LEFT half = Right Click, screen-RIGHT half = Left Click
    let r_zone = q.sub_quad(0.02, 0.04, 0.495, 0.32);
    let l_zone = q.sub_quad(0.505, 0.04, 0.98, 0.32);
    let idle_btn = Color::from_rgba(0.580, 0.639, 0.722, 0.16);
    fill_quad(
        frame,
        r_zone,
        if right_down { RIGHT_ACCENT } else { idle_btn },
    );
    fill_quad(frame, l_zone, if left_down { ACCENT } else { idle_btn });

    let div_color = if is_dark {
        Color::from_rgba(1.0, 1.0, 1.0, 0.18)
    } else {
        Color::from_rgba(0.059, 0.090, 0.165, 0.20)
    };
    let d_l = q.point(0.03, 0.33);
    let d_r = q.point(0.97, 0.33);
    let m_t = q.point(0.5, 0.04);
    let m_b = q.point(0.5, 0.33);
    frame.stroke(
        &Path::line(Point::new(d_l.x, d_l.y), Point::new(d_r.x, d_r.y)),
        Stroke::default().with_color(div_color).with_width(1.3),
    );
    frame.stroke(
        &Path::line(Point::new(m_t.x, m_t.y), Point::new(m_b.x, m_b.y)),
        Stroke::default().with_color(div_color).with_width(1.3),
    );

    // Touch / 2-finger glow ripples
    let active = cat.ptr_fingers > 0 || !cat.buttons_down.is_empty();
    if active {
        let is_mid = cat.buttons_down.contains(&MouseButton::Middle) || cat.ptr_fingers >= 3;
        let is_right = cat.buttons_down.contains(&MouseButton::Right) || cat.ptr_fingers >= 2;
        let color = if is_mid {
            MIDDLE_ACCENT
        } else if is_right {
            RIGHT_ACCENT
        } else {
            ACCENT
        };
        let r = if cat.buttons_down.is_empty() {
            8.5
        } else {
            11.5
        };
        let c1 = Path::circle(Point::new(pad_pt.x, pad_pt.y), r);
        frame.fill(&c1, Color { a: 0.28, ..color });
        frame.stroke(&c1, Stroke::default().with_color(color).with_width(2.0));
        if is_right {
            let c2 = Path::circle(Point::new(pad_pt.x - 14.0, pad_pt.y - 2.5), 7.5);
            frame.fill(&c2, Color { a: 0.24, ..color });
            frame.stroke(&c2, Stroke::default().with_color(color).with_width(1.8));
        }
    }

    if cat.wheel_dir != 0 {
        let sc = q.point(0.12, 0.66);
        frame.fill_text(Text {
            content: if cat.wheel_dir > 0 { "▲" } else { "▼" }.to_owned(),
            position: Point::new(sc.x - 5.0, sc.y - 8.0),
            color: ACCENT,
            size: 13.0.into(),
            ..Text::default()
        });
    }
}

fn draw_slanted_mousepad(frame: &mut Frame, q: Quad, pad_pt: Pt, cat: &CatState, is_dark: bool) {
    draw_quad_bevel(
        frame,
        q,
        6.0,
        if is_dark {
            Color::from_rgb(0.059, 0.090, 0.165)
        } else {
            Color::from_rgb(0.553, 0.584, 0.627)
        },
    );
    let pad_fill = if is_dark {
        Color::from_rgb(0.200, 0.255, 0.345)
    } else {
        Color::from_rgb(0.745, 0.776, 0.816)
    };
    fill_and_stroke_quad(
        frame,
        q,
        pad_fill,
        if is_dark {
            Color::from_rgb(0.012, 0.024, 0.090)
        } else {
            Color::from_rgb(0.482, 0.518, 0.569)
        },
        2.4,
    );

    // Angled cartoon mouse following pointer position
    let left_down = cat.buttons_down.contains(&MouseButton::Left)
        || cat.buttons_down.contains(&MouseButton::Middle);
    let right_down = cat.buttons_down.contains(&MouseButton::Right)
        || cat.buttons_down.contains(&MouseButton::Middle);
    let mid_down = cat.buttons_down.contains(&MouseButton::Middle);

    frame.with_save(|f| {
        f.translate(iced::Vector::new(pad_pt.x, pad_pt.y));
        f.rotate(202.0_f32.to_radians());

        fill_ellipse(
            f,
            0.0,
            4.0,
            30.0,
            23.0,
            Color::from_rgba(0.059, 0.090, 0.165, 0.18),
        );
        fill_and_stroke_ellipse(f, 0.0, 0.0, 28.0, 22.0, WHITE, INK, 4.4);

        // Left & right mouse button highlights
        let l_btn = Path::new(|b| {
            b.move_to(Point::new(-24.0, -2.0));
            b.quadratic_curve_to(Point::new(-16.0, -19.0), Point::new(-1.0, -19.0));
            b.line_to(Point::new(-1.0, -2.0));
            b.close();
        });
        let r_btn = Path::new(|b| {
            b.move_to(Point::new(1.0, -19.0));
            b.quadratic_curve_to(Point::new(16.0, -19.0), Point::new(24.0, -2.0));
            b.line_to(Point::new(1.0, -2.0));
            b.close();
        });
        if left_down {
            f.fill(&l_btn, ACCENT);
        }
        if right_down {
            f.fill(&r_btn, RIGHT_ACCENT);
        }

        f.stroke(
            &Path::line(Point::new(-25.0, -2.0), Point::new(25.0, -2.0)),
            Stroke::default().with_color(INK).with_width(2.6),
        );
        f.stroke(
            &Path::line(Point::new(0.0, -20.0), Point::new(0.0, -2.0)),
            Stroke::default().with_color(INK).with_width(2.6),
        );

        let wheel_y = match cat.wheel_dir.cmp(&0) {
            std::cmp::Ordering::Greater => -18.0,
            std::cmp::Ordering::Less => -14.0,
            std::cmp::Ordering::Equal => -16.0,
        };
        let wheel_color = if cat.wheel_dir != 0 {
            ACCENT
        } else if mid_down {
            MIDDLE_ACCENT
        } else {
            Color::from_rgb(0.392, 0.455, 0.545)
        };
        let wheel =
            Path::rounded_rectangle(Point::new(-3.0, wheel_y), Size::new(6.0, 10.5), 3.0.into());
        f.fill(&wheel, wheel_color);
        f.stroke(&wheel, Stroke::default().with_color(INK).with_width(1.6));
    });
}

fn draw_perspective_keyboard(
    frame: &mut Frame,
    kb_quad: Quad,
    cat: &CatState,
    is_dark: bool,
    with_bevel: bool,
) {
    if with_bevel {
        draw_quad_bevel(
            frame,
            kb_quad,
            6.0,
            if is_dark {
                Color::from_rgb(0.008, 0.024, 0.090)
            } else {
                Color::from_rgb(0.200, 0.255, 0.333)
            },
        );
    }

    let kb_base = if is_dark {
        Color::from_rgb(0.059, 0.090, 0.165)
    } else if with_bevel {
        Color::from_rgb(0.580, 0.639, 0.722)
    } else {
        Color::from_rgb(0.118, 0.161, 0.231)
    };
    fill_and_stroke_quad(
        frame,
        kb_quad,
        kb_base,
        INK,
        if with_bevel { 3.0 } else { 2.4 },
    );

    let cap_idle = if is_dark {
        Color::from_rgb(0.200, 0.255, 0.333)
    } else {
        Color::from_rgb(0.945, 0.961, 0.976)
    };
    let cap_stroke = if is_dark {
        Color::from_rgb(0.008, 0.024, 0.090)
    } else {
        Color::from_rgb(0.059, 0.090, 0.165)
    };
    let label_idle = if is_dark {
        Color::from_rgb(0.886, 0.910, 0.941)
    } else {
        Color::from_rgb(0.118, 0.161, 0.231)
    };
    let hide_labels = cat.config.privacy_mode == PrivacyMode::Zones;

    for cell in cat.key_cells() {
        let pressed = cat.is_key_pressed(cell.spec.code);
        fill_and_stroke_quad(
            frame,
            cell.quad,
            if pressed { ACCENT } else { cap_idle },
            cap_stroke,
            1.15,
        );

        if !hide_labels && cell.spec.label.len() <= 5 {
            let font_size = if cell.spec.label.len() > 3 {
                5.8
            } else if cell.spec.label.len() > 1 {
                6.8
            } else {
                8.2
            };
            let angle = (cell.quad.tr.y - cell.quad.tl.y).atan2(cell.quad.tr.x - cell.quad.tl.x)
                + std::f32::consts::PI;
            frame.with_save(|f| {
                f.translate(iced::Vector::new(cell.center.x, cell.center.y));
                f.rotate(angle);
                f.fill_text(Text {
                    content: cell.spec.label.to_owned(),
                    position: Point::ORIGIN,
                    color: if pressed { INK } else { label_idle },
                    size: font_size.into(),
                    horizontal_alignment: iced::alignment::Horizontal::Center,
                    vertical_alignment: iced::alignment::Vertical::Center,
                    ..Text::default()
                });
            });
        }
    }
}

// ============================================================================
// Flat Desk Mode Native Rust Vector Renderer
// ============================================================================
fn draw_flat_scene(frame: &mut Frame, cat: &CatState) {
    let pointer = cat.config.pointer;
    let is_mouse = pointer == PointerDevice::Mouse;
    let is_dark = cat.config.desk_theme == DeskTheme::Dark;
    let shift_x = flat_cat_shift_x(pointer);
    let (pad_quad, kb_quad) = flat_quads(pointer);

    // 1. Rounded cat head & ears behind desk
    frame.with_save(|f| {
        f.translate(iced::Vector::new(shift_x, 0.0));
        draw_flat_cat_head(f, cat);
    });

    // 2. Flat Desk Surface
    if cat.config.desk_theme == DeskTheme::Transparent {
        frame.stroke(
            &Path::line(
                Point::new(14.0, FLAT_DESK_Y),
                Point::new(FLAT_WIDTH - 14.0, FLAT_DESK_Y),
            ),
            Stroke::default().with_color(INK).with_width(4.2),
        );
    } else {
        let desk_fill = match cat.config.desk_theme {
            DeskTheme::Dark => Color::from_rgb(0.118, 0.149, 0.212),
            DeskTheme::Pastel => Color::from_rgb(0.988, 0.906, 0.953),
            DeskTheme::White | DeskTheme::Transparent => Color::from_rgb(0.945, 0.961, 0.976),
        };
        let desk_rect = Path::rounded_rectangle(
            Point::new(14.0, FLAT_DESK_Y),
            Size::new(FLAT_WIDTH - 28.0, FLAT_HEIGHT - FLAT_DESK_Y - 8.0),
            18.0.into(),
        );
        frame.fill(&desk_rect, desk_fill);
        frame.stroke(
            &desk_rect,
            Stroke::default().with_color(INK).with_width(4.2),
        );
    }

    // 3. Laptop base or standalone side mousepad
    if !is_mouse {
        let chassis_fill = if is_dark {
            Color::from_rgb(0.165, 0.212, 0.294)
        } else {
            Color::from_rgb(0.847, 0.878, 0.918)
        };
        fill_and_stroke_quad(frame, FLAT_LAPTOP_CHASSIS, chassis_fill, INK, 3.2);
    }

    let pad_pt = pointer_pad_point(
        DisplayMode::Flat,
        pointer,
        cat.config.ptr_dir,
        cat.cur_ptr_x,
        cat.cur_ptr_y,
    );
    if is_mouse {
        draw_slanted_mousepad(frame, pad_quad, pad_pt, cat, is_dark);
    } else {
        draw_slanted_trackpad(frame, pad_quad, pad_pt, cat, is_dark);
    }

    // 4. Flat 63-key keyboard
    draw_perspective_keyboard(frame, kb_quad, cat, is_dark, false);

    // 5. Smooth curved paws (Active -> Resting on palm-rest -> Raised both!)
    let ((r_out, r_in), (l_out, l_in)) = flat_shoulders(pointer);

    let r_arm_alpha = (1.0 - cat.right_raise).clamp(0.0, 1.0);
    if r_arm_alpha > 0.02 {
        let right_arm = build_seamless_arm(r_out, r_in, cat.cur_right_paw, ScreenSide::Left, 18.0);
        draw_arm_spline(frame, right_arm, 4.6, r_arm_alpha);
    }
    if cat.right_raise > 0.02 {
        let lift = (1.0 - cat.right_raise) * 12.0;
        draw_flat_raised_paw(
            frame,
            r_out.x + 16.0,
            FLAT_DESK_Y - 8.0 + lift,
            cat.right_raise,
        );
    }

    let l_arm_alpha = (1.0 - cat.left_raise).clamp(0.0, 1.0);
    if l_arm_alpha > 0.02 {
        let left_arm = build_seamless_arm(l_out, l_in, cat.cur_left_paw, ScreenSide::Right, 18.0);
        draw_arm_spline(frame, left_arm, 4.6, l_arm_alpha);
    }
    if cat.left_raise > 0.02 {
        let lift = (1.0 - cat.left_raise) * 12.0;
        draw_flat_raised_paw(
            frame,
            l_out.x - 16.0,
            FLAT_DESK_Y - 8.0 + lift,
            cat.left_raise,
        );
    }
}

fn draw_flat_raised_paw(frame: &mut Frame, cx: f32, cy: f32, alpha: f32) {
    let fill = Color { a: alpha, ..WHITE };
    let ink = Color { a: alpha, ..INK };
    let pink = Color {
        a: BEAN_PINK.a * alpha,
        ..BEAN_PINK
    };
    fill_and_stroke_ellipse(frame, cx, cy, 22.0, 26.0, fill, ink, 4.4);
    fill_ellipse(frame, cx, cy + 4.0, 8.0, 7.0, pink);
    fill_ellipse(frame, cx - 7.5, cy - 6.0, 3.5, 4.0, pink);
    fill_ellipse(frame, cx, cy - 8.5, 3.6, 4.2, pink);
    fill_ellipse(frame, cx + 7.5, cy - 6.0, 3.5, 4.0, pink);
}

fn draw_flat_cat_head(frame: &mut Frame, cat: &CatState) {
    let head = Path::new(|b| {
        b.move_to(Point::new(172.0, 174.0));
        b.bezier_curve_to(
            Point::new(168.0, 122.0),
            Point::new(190.0, 80.0),
            Point::new(220.0, 66.0),
        );
        b.bezier_curve_to(
            Point::new(222.0, 36.0),
            Point::new(236.0, 18.0),
            Point::new(252.0, 18.0),
        );
        b.bezier_curve_to(
            Point::new(268.0, 22.0),
            Point::new(282.0, 38.0),
            Point::new(292.0, 52.0),
        );
        b.bezier_curve_to(
            Point::new(310.0, 47.0),
            Point::new(330.0, 47.0),
            Point::new(348.0, 52.0),
        );
        b.bezier_curve_to(
            Point::new(358.0, 38.0),
            Point::new(372.0, 22.0),
            Point::new(388.0, 18.0),
        );
        b.bezier_curve_to(
            Point::new(404.0, 18.0),
            Point::new(418.0, 36.0),
            Point::new(420.0, 66.0),
        );
        b.bezier_curve_to(
            Point::new(450.0, 80.0),
            Point::new(472.0, 122.0),
            Point::new(468.0, 174.0),
        );
        b.close();
    });
    frame.fill(&head, WHITE);
    frame_stroke_round(frame, &head, INK, 4.6);

    // Inner pink ears
    let ear_l = Path::new(|b| {
        b.move_to(Point::new(236.0, 62.0));
        b.line_to(Point::new(251.0, 34.0));
        b.line_to(Point::new(274.0, 56.0));
        b.close();
    });
    let ear_r = Path::new(|b| {
        b.move_to(Point::new(404.0, 62.0));
        b.line_to(Point::new(389.0, 34.0));
        b.line_to(Point::new(366.0, 56.0));
        b.close();
    });
    frame.fill(&ear_l, BLUSH);
    frame.fill(&ear_r, BLUSH);

    let (fx, fy, px, py) = if cat.config.eyes_enabled && !cat.sleeping {
        (
            cat.cur_eye_x * 0.35,
            cat.cur_eye_y * 0.30,
            cat.cur_eye_x * 0.55,
            cat.cur_eye_y * 0.55,
        )
    } else {
        (0.0, 0.0, 0.0, 0.0)
    };

    frame.with_save(|f| {
        f.translate(iced::Vector::new(fx, fy));
        fill_ellipse(f, 232.0, 126.0, 14.0, 7.5, BLUSH);
        fill_ellipse(f, 408.0, 126.0, 14.0, 7.5, BLUSH);

        if cat.sleeping {
            let e_l = Path::new(|b| {
                b.move_to(Point::new(255.0, 108.0));
                b.quadratic_curve_to(Point::new(268.0, 118.0), Point::new(281.0, 108.0));
            });
            let e_r = Path::new(|b| {
                b.move_to(Point::new(359.0, 108.0));
                b.quadratic_curve_to(Point::new(372.0, 118.0), Point::new(385.0, 108.0));
            });
            frame_stroke_round(f, &e_l, INK, 4.2);
            frame_stroke_round(f, &e_r, INK, 4.2);
        } else {
            draw_cat_eye(f, 268.0, 106.0, px, py, cat.config.eye_style, cat.blinking);
            draw_cat_eye(f, 372.0, 106.0, px, py, cat.config.eye_style, cat.blinking);
        }

        let mouth = Path::new(|b| {
            b.move_to(Point::new(301.0, 122.0));
            b.bezier_curve_to(
                Point::new(304.0, 132.0),
                Point::new(317.0, 131.0),
                Point::new(320.0, 122.0),
            );
            b.bezier_curve_to(
                Point::new(323.0, 131.0),
                Point::new(336.0, 132.0),
                Point::new(339.0, 122.0),
            );
        });
        frame_stroke_round(f, &mouth, INK, 4.2);

        if cat.fast_typing && !cat.sleeping {
            let smile = Path::new(|b| {
                b.move_to(Point::new(311.0, 127.0));
                b.quadratic_curve_to(Point::new(320.0, 139.0), Point::new(329.0, 127.0));
                b.close();
            });
            f.fill(&smile, RIGHT_ACCENT);
            frame_stroke_round(f, &smile, INK, 2.4);
        }
    });
}

// ============================================================================
// Shared Vector Primitives
// ============================================================================
fn draw_cat_eye(
    frame: &mut Frame,
    cx: f32,
    cy: f32,
    px: f32,
    py: f32,
    style: EyeStyle,
    blinking: bool,
) {
    if blinking {
        frame_stroke_round(
            frame,
            &Path::line(Point::new(cx - 9.0, cy), Point::new(cx + 9.0, cy)),
            INK,
            3.8,
        );
        return;
    }

    if style == EyeStyle::Expressive {
        fill_and_stroke_ellipse(frame, cx, cy, 13.0, 14.0, WHITE, INK, 2.4);
        fill_ellipse(frame, cx + px, cy + py, 8.2, 9.5, INK);
        frame.fill(
            &Path::circle(Point::new(cx + px - 2.5, cy + py - 3.0), 2.7),
            WHITE,
        );
        frame.fill(
            &Path::circle(Point::new(cx + px + 2.5, cy + py + 2.6), 1.3),
            Color { a: 0.85, ..WHITE },
        );
    } else {
        fill_ellipse(frame, cx + px, cy + py, 8.8, 9.8, INK);
        frame.fill(
            &Path::circle(Point::new(cx + px - 2.6, cy + py - 3.0), 2.1),
            Color { a: 0.45, ..WHITE },
        );
    }
}

fn draw_arm_spline(frame: &mut Frame, arm: ArmSpline, width: f32, alpha: f32) {
    let fill = Color { a: alpha, ..WHITE };
    let ink = Color { a: alpha, ..INK };
    let open = Path::new(|b| {
        b.move_to(Point::new(arm.s_outer.x, arm.s_outer.y));
        b.quadratic_curve_to(
            Point::new(arm.c_outer.x, arm.c_outer.y),
            Point::new(arm.t_outer.x, arm.t_outer.y),
        );
        b.quadratic_curve_to(
            Point::new(arm.tip.x, arm.tip.y),
            Point::new(arm.t_inner.x, arm.t_inner.y),
        );
        b.quadratic_curve_to(
            Point::new(arm.c_inner.x, arm.c_inner.y),
            Point::new(arm.s_inner.x, arm.s_inner.y),
        );
    });
    let closed = Path::new(|b| {
        b.move_to(Point::new(arm.s_outer.x, arm.s_outer.y));
        b.quadratic_curve_to(
            Point::new(arm.c_outer.x, arm.c_outer.y),
            Point::new(arm.t_outer.x, arm.t_outer.y),
        );
        b.quadratic_curve_to(
            Point::new(arm.tip.x, arm.tip.y),
            Point::new(arm.t_inner.x, arm.t_inner.y),
        );
        b.quadratic_curve_to(
            Point::new(arm.c_inner.x, arm.c_inner.y),
            Point::new(arm.s_inner.x, arm.s_inner.y),
        );
        b.close();
    });
    frame.fill(&closed, fill);
    frame_stroke_round(frame, &open, ink, width);
}

fn draw_quad_bevel(frame: &mut Frame, q: Quad, depth: f32, fill: Color) {
    let bevel = Path::new(|b| {
        b.move_to(Point::new(q.bl.x, q.bl.y));
        b.line_to(Point::new(q.br.x, q.br.y));
        b.line_to(Point::new(q.tr.x, q.tr.y));
        b.line_to(Point::new(q.tr.x, q.tr.y + depth));
        b.line_to(Point::new(q.br.x, q.br.y + depth));
        b.line_to(Point::new(q.bl.x, q.bl.y + depth));
        b.close();
    });
    frame.fill(&bevel, fill);
    frame_stroke_round(frame, &bevel, INK, 2.0);
}

fn fill_quad(frame: &mut Frame, q: Quad, fill: Color) {
    let p = quad_path(q);
    frame.fill(&p, fill);
}

fn fill_and_stroke_quad(frame: &mut Frame, q: Quad, fill: Color, stroke: Color, width: f32) {
    let p = quad_path(q);
    frame.fill(&p, fill);
    frame_stroke_round(frame, &p, stroke, width);
}

fn quad_path(q: Quad) -> Path {
    let [tl, tr, br, bl] = q.corners();
    Path::new(|builder| {
        builder.move_to(Point::new(tl.x, tl.y));
        builder.line_to(Point::new(tr.x, tr.y));
        builder.line_to(Point::new(br.x, br.y));
        builder.line_to(Point::new(bl.x, bl.y));
        builder.close();
    })
}

fn fill_ellipse(frame: &mut Frame, cx: f32, cy: f32, rx: f32, ry: f32, color: Color) {
    frame.with_save(|f| {
        f.translate(iced::Vector::new(cx, cy));
        f.scale_nonuniform(iced::Vector::new(rx, ry));
        f.fill(&Path::circle(Point::ORIGIN, 1.0), color);
    });
}

#[allow(clippy::too_many_arguments)]
fn fill_and_stroke_ellipse(
    frame: &mut Frame,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    fill: Color,
    stroke: Color,
    width: f32,
) {
    frame.with_save(|f| {
        f.translate(iced::Vector::new(cx, cy));
        f.scale_nonuniform(iced::Vector::new(rx, ry));
        let unit = Path::circle(Point::ORIGIN, 1.0);
        f.fill(&unit, fill);
        let avg = f32::midpoint(rx, ry).max(1.0);
        f.stroke(
            &unit,
            Stroke::default().with_color(stroke).with_width(width / avg),
        );
    });
}

fn frame_stroke_round(frame: &mut Frame, path: &Path, color: Color, width: f32) {
    frame.stroke(
        path,
        Stroke::default()
            .with_color(color)
            .with_width(width)
            .with_line_cap(LineCap::Round)
            .with_line_join(LineJoin::Round),
    );
}
