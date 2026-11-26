use bongocat_core::AppTheme;
use iced::border::Radius;
use iced::widget::{button, container};
use iced::{Background, Border, Color, Shadow, Theme, Vector};

#[derive(Debug, Clone, Copy)]
pub struct StudioPalette {
    pub bg: Color,
    pub surface: Color,
    pub surface_alt: Color,
    pub border: Color,
    pub border_strong: Color,
    pub text: Color,
    pub text_muted: Color,
    pub accent: Color,
    pub accent_soft: Color,
    pub accent_text: Color,
    pub danger: Color,
    pub danger_soft: Color,
    pub success: Color,
    pub warning: Color,
    pub checker_a: Color,
    pub checker_b: Color,
}

impl StudioPalette {
    pub const fn for_theme(theme: AppTheme) -> Self {
        match theme {
            AppTheme::Dark => Self {
                bg: Color::from_rgb(0.043, 0.067, 0.125),          // #0b1120
                surface: Color::from_rgb(0.075, 0.110, 0.192),     // #131c31
                surface_alt: Color::from_rgb(0.118, 0.161, 0.231), // #1e293b
                border: Color::from_rgb(0.149, 0.204, 0.294),      // #26344b
                border_strong: Color::from_rgb(0.220, 0.298, 0.420),
                text: Color::from_rgb(0.973, 0.980, 0.988), // #f8fafc
                text_muted: Color::from_rgb(0.580, 0.639, 0.722), // #94a3b8
                accent: Color::from_rgb(0.220, 0.741, 0.973), // #38bdf8
                accent_soft: Color::from_rgba(0.220, 0.741, 0.973, 0.16),
                accent_text: Color::from_rgb(0.031, 0.071, 0.145),
                danger: Color::from_rgb(0.984, 0.443, 0.522), // #fb7185
                danger_soft: Color::from_rgba(0.984, 0.443, 0.522, 0.18),
                success: Color::from_rgb(0.290, 0.871, 0.502), // #4ade80
                warning: Color::from_rgb(0.984, 0.749, 0.141), // #fbbf24
                checker_a: Color::from_rgb(0.094, 0.133, 0.212),
                checker_b: Color::from_rgb(0.122, 0.165, 0.251),
            },
            AppTheme::Light => Self {
                bg: Color::from_rgb(0.945, 0.961, 0.976),          // #f1f5f9
                surface: Color::from_rgb(1.0, 1.0, 1.0),           // #ffffff
                surface_alt: Color::from_rgb(0.945, 0.961, 0.976), // #f1f5f9
                border: Color::from_rgb(0.847, 0.882, 0.922),      // #d8e1eb
                border_strong: Color::from_rgb(0.741, 0.800, 0.867),
                text: Color::from_rgb(0.059, 0.090, 0.165), // #0f172a
                text_muted: Color::from_rgb(0.392, 0.455, 0.545), // #64748b
                accent: Color::from_rgb(0.008, 0.518, 0.780), // #0284c7
                accent_soft: Color::from_rgba(0.008, 0.518, 0.780, 0.13),
                accent_text: Color::from_rgb(1.0, 1.0, 1.0),
                danger: Color::from_rgb(0.882, 0.114, 0.282), // #e11d48
                danger_soft: Color::from_rgba(0.882, 0.114, 0.282, 0.14),
                success: Color::from_rgb(0.086, 0.639, 0.290), // #16a34a
                warning: Color::from_rgb(0.851, 0.467, 0.024), // #d97706
                checker_a: Color::from_rgb(0.945, 0.957, 0.973),
                checker_b: Color::from_rgb(0.894, 0.914, 0.941),
            },
        }
    }
}

pub fn iced_theme(app_theme: AppTheme) -> Theme {
    let p = StudioPalette::for_theme(app_theme);
    let name = match app_theme {
        AppTheme::Dark => "Studio Dark",
        AppTheme::Light => "Studio Light",
    };
    Theme::custom(
        name.to_owned(),
        iced::theme::Palette {
            background: p.bg,
            text: p.text,
            primary: p.accent,
            success: p.success,
            danger: p.danger,
        },
    )
}

pub fn root_bg_style(p: StudioPalette) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(p.bg)),
        text_color: Some(p.text),
        ..container::Style::default()
    }
}

pub fn header_bar_style(p: StudioPalette) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(p.surface)),
        text_color: Some(p.text),
        border: Border {
            color: p.border,
            width: 1.0,
            radius: Radius::new(0.0),
        },
        ..container::Style::default()
    }
}

pub fn card_style(p: StudioPalette) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(p.surface)),
        text_color: Some(p.text),
        border: Border {
            color: p.border,
            width: 1.0,
            radius: Radius::new(14.0),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.14),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 14.0,
        },
    }
}

pub fn subtle_panel_style(p: StudioPalette) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(p.surface_alt)),
        text_color: Some(p.text),
        border: Border {
            color: p.border,
            width: 1.0,
            radius: Radius::new(10.0),
        },
        ..container::Style::default()
    }
}

pub fn badge_style(bg: Color, border: Color, text: Color) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(bg)),
        text_color: Some(text),
        border: Border {
            color: border,
            width: 1.0,
            radius: Radius::new(999.0),
        },
        ..container::Style::default()
    }
}

pub fn segment_button_style(
    p: StudioPalette,
    active: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_, status| {
        let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
        if active {
            button::Style {
                background: Some(Background::Color(p.accent)),
                text_color: p.accent_text,
                border: Border {
                    color: p.accent,
                    width: 1.0,
                    radius: Radius::new(9.0),
                },
                ..button::Style::default()
            }
        } else {
            button::Style {
                background: Some(Background::Color(if hovered {
                    p.border
                } else {
                    p.surface_alt
                })),
                text_color: p.text,
                border: Border {
                    color: if hovered { p.border_strong } else { p.border },
                    width: 1.0,
                    radius: Radius::new(9.0),
                },
                ..button::Style::default()
            }
        }
    }
}

pub fn pill_button_style(
    p: StudioPalette,
    active: bool,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_, status| {
        let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
        if active {
            button::Style {
                background: Some(Background::Color(p.accent_soft)),
                text_color: p.accent,
                border: Border {
                    color: p.accent,
                    width: 1.2,
                    radius: Radius::new(999.0),
                },
                ..button::Style::default()
            }
        } else {
            button::Style {
                background: Some(Background::Color(if hovered {
                    p.surface_alt
                } else {
                    Color::TRANSPARENT
                })),
                text_color: if hovered { p.text } else { p.text_muted },
                border: Border {
                    color: if hovered { p.border_strong } else { p.border },
                    width: 1.0,
                    radius: Radius::new(999.0),
                },
                ..button::Style::default()
            }
        }
    }
}

pub fn action_button_style(
    p: StudioPalette,
    emphasis: ActionEmphasis,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_, status| {
        let hovered = matches!(status, button::Status::Hovered);
        let pressed = matches!(status, button::Status::Pressed);
        match emphasis {
            ActionEmphasis::Primary => button::Style {
                background: Some(Background::Color(if pressed {
                    Color {
                        a: 0.85,
                        ..p.accent
                    }
                } else {
                    p.accent
                })),
                text_color: p.accent_text,
                border: Border {
                    color: p.accent,
                    width: 1.0,
                    radius: Radius::new(9.0),
                },
                ..button::Style::default()
            },
            ActionEmphasis::Danger => button::Style {
                background: Some(Background::Color(if hovered || pressed {
                    p.danger
                } else {
                    p.danger_soft
                })),
                text_color: if hovered || pressed {
                    Color::WHITE
                } else {
                    p.danger
                },
                border: Border {
                    color: p.danger,
                    width: 1.0,
                    radius: Radius::new(9.0),
                },
                ..button::Style::default()
            },
            ActionEmphasis::Neutral => button::Style {
                background: Some(Background::Color(if hovered {
                    p.border
                } else {
                    p.surface_alt
                })),
                text_color: p.text,
                border: Border {
                    color: p.border_strong,
                    width: 1.0,
                    radius: Radius::new(9.0),
                },
                ..button::Style::default()
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionEmphasis {
    Primary,
    Danger,
    Neutral,
}
