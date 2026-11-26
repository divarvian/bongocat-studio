use bongocat_core::{
    AppConfig, DeskTheme, DeviceInfo, DisplayMode, EyeDirection, EyeStyle, PointerDevice,
    PointerDirection, PrivacyMode, RightPawMode,
};
use iced::widget::{Space, button, column, container, row, scrollable, slider, text, toggler};
use iced::{Alignment, Element, Length};

use crate::app::{Message, SettingsTab};
use crate::ui::theme::{
    ActionEmphasis, StudioPalette, action_button_style, badge_style, card_style,
    segment_button_style, subtle_panel_style,
};

const EMBEDDED_LICENSE: &str = include_str!("../../../../LICENSE");

pub fn view_settings<'a>(
    palette: StudioPalette,
    active_tab: SettingsTab,
    config: &'a AppConfig,
    privacy_paused: bool,
    devices: &'a [DeviceInfo],
    show_full_license: bool,
) -> Element<'a, Message> {
    let tabs = row![
        tab_btn(palette, SettingsTab::Appearance, active_tab),
        tab_btn(palette, SettingsTab::Behavior, active_tab),
        tab_btn(palette, SettingsTab::System, active_tab),
    ]
    .spacing(6);

    let tab_body = match active_tab {
        SettingsTab::Appearance => view_appearance(palette, config),
        SettingsTab::Behavior => view_behavior(palette, config),
        SettingsTab::System => {
            view_system(palette, config, privacy_paused, devices, show_full_license)
        }
    };

    column![
        tabs,
        scrollable(tab_body.width(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill),
    ]
    .spacing(12)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn tab_btn<'a>(
    palette: StudioPalette,
    tab: SettingsTab,
    active: SettingsTab,
) -> Element<'a, Message> {
    button(
        text(tab.label())
            .size(13)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center),
    )
    .width(Length::Fill)
    .padding([8, 10])
    .style(segment_button_style(palette, tab == active))
    .on_press(Message::SelectTab(tab))
    .into()
}

fn view_appearance(
    palette: StudioPalette,
    config: &AppConfig,
) -> iced::widget::Column<'_, Message> {
    let ov = &config.overlay;
    let is_classic = ov.mode == DisplayMode::Classic;

    let mut mode_row = row![].spacing(6);
    for mode in DisplayMode::ALL {
        mode_row = mode_row.push(
            button(
                text(mode.short_label())
                    .size(12.5)
                    .width(Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center),
            )
            .width(Length::Fill)
            .padding([7, 8])
            .style(segment_button_style(palette, ov.mode == mode))
            .on_press(Message::SetDisplayMode(mode)),
        );
    }

    let mut col = column![section_card(palette, "Display Mode", column![mode_row])]
        .spacing(11)
        .padding(iced::Padding {
            top: 0.0,
            right: 6.0,
            bottom: 8.0,
            left: 0.0,
        });

    if !is_classic {
        let mut ptr_row = row![].spacing(6);
        for ptr in PointerDevice::ALL {
            ptr_row = ptr_row.push(
                button(
                    text(ptr.label())
                        .size(12.5)
                        .width(Length::Fill)
                        .align_x(iced::alignment::Horizontal::Center),
                )
                .width(Length::Fill)
                .padding([7, 8])
                .style(segment_button_style(palette, ov.pointer == ptr))
                .on_press(Message::SetPointerDevice(ptr)),
            );
        }
        let ptr_hint = match (ov.mode, ov.pointer) {
            (DisplayMode::Slanted, PointerDevice::Trackpad) => {
                "Unified slanted laptop — Trackpad centered directly below Spacebar."
            }
            (DisplayMode::Slanted, PointerDevice::Mouse) => {
                "Mousepad tucked on cat's right while cat stays centered on keyboard."
            }
            (DisplayMode::Flat, PointerDevice::Trackpad) => {
                "Top-down laptop desk with centered palm-rest Trackpad."
            }
            _ => "Top-down keyboard + mousepad on cat's right hand.",
        };
        col = col.push(section_card(
            palette,
            "Pointer Device",
            column![ptr_row, text(ptr_hint).size(11.5).color(palette.text_muted)].spacing(7),
        ));

        let mut theme_row = row![].spacing(6);
        for dt in DeskTheme::ALL {
            theme_row = theme_row.push(
                button(
                    text(dt.label())
                        .size(12)
                        .width(Length::Fill)
                        .align_x(iced::alignment::Horizontal::Center),
                )
                .width(Length::Fill)
                .padding([6, 6])
                .style(segment_button_style(palette, ov.desk_theme == dt))
                .on_press(Message::SetDeskTheme(dt)),
            );
        }
        col = col.push(section_card(palette, "Desk Surface", column![theme_row]));

        let mut eye_style_row = row![].spacing(6);
        for es in EyeStyle::ALL {
            eye_style_row = eye_style_row.push(
                button(
                    text(es.label())
                        .size(12)
                        .width(Length::Fill)
                        .align_x(iced::alignment::Horizontal::Center),
                )
                .width(Length::Fill)
                .padding([6, 8])
                .style(segment_button_style(palette, ov.eye_style == es))
                .on_press(Message::SetEyeStyle(es)),
            );
        }

        let mut eye_dir_row = row![].spacing(6);
        for ed in EyeDirection::ALL {
            eye_dir_row = eye_dir_row.push(
                button(
                    text(ed.label())
                        .size(12)
                        .width(Length::Fill)
                        .align_x(iced::alignment::Horizontal::Center),
                )
                .width(Length::Fill)
                .padding([6, 8])
                .style(segment_button_style(palette, ov.eye_dir == ed))
                .on_press(Message::SetEyeDirection(ed)),
            );
        }

        col = col.push(section_card(
            palette,
            "Eyes & Face Parallax",
            column![
                toggle_row(
                    palette,
                    "Cursor Eye Tracking",
                    ov.eyes_enabled,
                    Message::SetEyesEnabled,
                ),
                eye_style_row,
                eye_dir_row,
            ]
            .spacing(8),
        ));
    }

    col = col.push(section_card(
        palette,
        "Overlay Visuals",
        column![
            toggle_row(
                palette,
                "Bounce Effect on Hit",
                ov.bounce,
                Message::SetBounce
            ),
            toggle_row(palette, "Show KPS Meter", ov.show_kps, Message::SetShowKps),
            toggle_row(
                palette,
                "Show Active Key / Click HUD",
                ov.show_hud,
                Message::SetShowHud,
            ),
            toggle_row(
                palette,
                "Preview Alpha Checkerboard",
                config.show_checkerboard,
                Message::SetShowCheckerboard,
            ),
        ]
        .spacing(8),
    ));

    col
}

fn view_behavior(palette: StudioPalette, config: &AppConfig) -> iced::widget::Column<'_, Message> {
    let ov = &config.overlay;

    let mut ptr_dir_row = row![].spacing(6);
    for pd in PointerDirection::ALL {
        ptr_dir_row = ptr_dir_row.push(
            button(
                text(pd.label())
                    .size(12)
                    .width(Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center),
            )
            .width(Length::Fill)
            .padding([7, 8])
            .style(segment_button_style(palette, ov.ptr_dir == pd))
            .on_press(Message::SetPointerDirection(pd)),
        );
    }

    let mut paw_row = row![].spacing(6);
    for rp in RightPawMode::ALL {
        paw_row = paw_row.push(
            button(
                text(rp.label())
                    .size(12)
                    .width(Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center),
            )
            .width(Length::Fill)
            .padding([7, 8])
            .style(segment_button_style(palette, ov.right_paw == rp))
            .on_press(Message::SetRightPawMode(rp)),
        );
    }

    let sleep_label = if ov.sleep_sec == 0 {
        "Off".to_owned()
    } else {
        format!("{} s", ov.sleep_sec)
    };

    column![
        section_card(palette, "Pointer Axis Direction", column![ptr_dir_row]),
        section_card(
            palette,
            "Right Paw Priority",
            column![
                paw_row,
                text("Auto types right-hand keys (Y-P, H-L, N-M) and returns to pointer; Stay on Pointer is great for FPS gaming.")
                    .size(11.5)
                    .color(palette.text_muted),
            ]
            .spacing(7),
        ),
        section_card(
            palette,
            "Animation Timing",
            column![
                slider_row(
                    palette,
                    "Key Hold Duration",
                    format!("{} ms", ov.key_hold_ms),
                    50..=400,
                    ov.key_hold_ms,
                    Message::SetKeyHoldMs,
                ),
                slider_row(
                    palette,
                    "Idle Paw Raise Delay",
                    format!("{} s", ov.idle_raise_sec),
                    1..=15,
                    ov.idle_raise_sec,
                    Message::SetIdleRaiseSec,
                ),
                slider_row(
                    palette,
                    "Idle Sleep Timeout",
                    sleep_label,
                    0..=300,
                    ov.sleep_sec,
                    Message::SetSleepSec,
                ),
                slider_row(
                    palette,
                    "Happy Smile Threshold",
                    format!("{} KPS", ov.kps_happy_threshold),
                    2..=15,
                    ov.kps_happy_threshold,
                    Message::SetKpsThreshold,
                ),
            ]
            .spacing(12),
        ),
    ]
    .spacing(11)
    .padding(iced::Padding {
        top: 0.0,
        right: 6.0,
        bottom: 8.0,
        left: 0.0,
    })
}

fn view_system<'a>(
    palette: StudioPalette,
    config: &'a AppConfig,
    privacy_paused: bool,
    devices: &'a [DeviceInfo],
    show_full_license: bool,
) -> iced::widget::Column<'a, Message> {
    let ov = &config.overlay;

    let mut priv_row = row![].spacing(6);
    for pm in PrivacyMode::ALL {
        priv_row = priv_row.push(
            button(
                text(pm.label())
                    .size(11.5)
                    .width(Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center),
            )
            .width(Length::Fill)
            .padding([7, 6])
            .style(segment_button_style(palette, ov.privacy_mode == pm))
            .on_press(Message::SetPrivacyMode(pm)),
        );
    }

    let priv_desc = match ov.privacy_mode {
        PrivacyMode::Full => {
            "Shows all 63 keys on the virtual keyboard & HUD. Raw keycodes are never logged."
        }
        PrivacyMode::Gaming => {
            "Only WASD/arrows/modifiers/1-6 are shown accurately; all other typing is masked into generic zones."
        }
        PrivacyMode::Zones => {
            "Maximum privacy — hides key labels and masks every keypress into generic left/right hand zones."
        }
    };

    let mut dev_list = column![].spacing(6);
    if devices.is_empty() {
        dev_list = dev_list.push(
            text("No active /dev/input devices detected yet.")
                .size(12)
                .color(palette.text_muted),
        );
    } else {
        for dev in devices {
            dev_list = dev_list.push(
                container(
                    row![
                        text(&dev.name).size(12).color(palette.text),
                        Space::with_width(Length::Fill),
                        container(text(dev.kind.label()).size(11).color(palette.accent))
                            .padding([2, 8])
                            .style(badge_style(
                                palette.accent_soft,
                                palette.border,
                                palette.accent,
                            )),
                    ]
                    .align_y(Alignment::Center),
                )
                .width(Length::Fill)
                .padding([6, 10])
                .style(subtle_panel_style(palette)),
            );
        }
    }

    column![
        section_card(
            palette,
            "Key Privacy Mode",
            column![
                priv_row,
                text(priv_desc).size(11.5).color(palette.text_muted),
                toggle_row(
                    palette,
                    "Pause Privacy (Freeze Key Input)",
                    privacy_paused,
                    Message::SetPrivacyPause,
                ),
            ]
            .spacing(8),
        ),
        section_card(
            palette,
            "Background & Autostart",
            column![
                toggle_row(
                    palette,
                    "Close Window to Background (Keep OBS Live)",
                    config.close_to_background,
                    Message::SetCloseToBackground,
                ),
                toggle_row(
                    palette,
                    "Start Automatically on Login (Minimized)",
                    config.autostart,
                    Message::SetAutostart,
                ),
                text(format!(
                    "OBS server bound strictly to 127.0.0.1:{} (localhost only).",
                    config.port
                ))
                .size(11.5)
                .color(palette.text_muted),
            ]
            .spacing(8),
        ),
        section_card(palette, "Detected Input Devices", dev_list),
        view_about_card(palette, show_full_license),
    ]
    .spacing(11)
    .padding(iced::Padding {
        top: 0.0,
        right: 6.0,
        bottom: 8.0,
        left: 0.0,
    })
}

fn view_about_card<'a>(palette: StudioPalette, show_full_license: bool) -> Element<'a, Message> {
    let version = env!("CARGO_PKG_VERSION");

    let mut content = column![
        row![
            text(format!("BongoCat Studio v{version}"))
                .size(13)
                .color(palette.text)
                .font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..iced::Font::DEFAULT
                }),
            Space::with_width(Length::Fill),
            container(text("MIT License").size(11).color(palette.accent))
                .padding([2, 8])
                .style(badge_style(
                    palette.accent_soft,
                    palette.border,
                    palette.accent,
                )),
        ]
        .align_y(Alignment::Center),
        text("Copyright (c) 2026 Arvian")
            .size(12)
            .color(palette.text_muted),
        text(env!("CARGO_PKG_REPOSITORY"))
            .size(11)
            .font(iced::Font::MONOSPACE)
            .color(palette.accent),
        text("Third-Party Attributions:")
            .size(11.5)
            .color(palette.text)
            .font(iced::Font {
                weight: iced::font::Weight::Semibold,
                ..iced::Font::DEFAULT
            }),
        text("• Classic SVGs: wayland-bongocat (MIT, Copyright (c) 2025 Saatvik Sharma)")
            .size(11)
            .color(palette.text_muted),
        text("• Rust Crates: iced, axum, tokio (MIT); evdev, serde, clap, toml, dirs, tracing (MIT/Apache-2.0); ksni (Unlicense)")
            .size(11)
            .color(palette.text_muted),
        text("• Embedded SVG engine (MPL-2.0, unmodified): resvg, usvg, option-ext")
            .size(11)
            .color(palette.text_muted),
        text("Fan-made open-source project inspired by the Bongo Cat meme (StrayRogue / DitzyFlama). Not affiliated with any commercial Bongo Cat brand.")
            .size(11)
            .color(palette.text_muted),
        button(
            text(if show_full_license {
                "Hide MIT License Text"
            } else {
                "View MIT License Text"
            })
            .size(11.5),
        )
        .padding([5, 10])
        .style(action_button_style(palette, ActionEmphasis::Neutral))
        .on_press(Message::ToggleFullLicense),
    ]
    .spacing(6);

    if show_full_license {
        content = content.push(
            container(
                text(EMBEDDED_LICENSE.trim())
                    .size(10.5)
                    .font(iced::Font::MONOSPACE)
                    .color(palette.text_muted),
            )
            .width(Length::Fill)
            .padding([8, 10])
            .style(subtle_panel_style(palette)),
        );
    }

    section_card(palette, "About & Licenses", content)
}

fn section_card<'a>(
    palette: StudioPalette,
    title: &'a str,
    body: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let inner = column![
        text(title)
            .size(12.5)
            .color(palette.text_muted)
            .font(iced::Font {
                weight: iced::font::Weight::Semibold,
                ..iced::Font::DEFAULT
            }),
        body.into(),
    ]
    .spacing(8);

    container(inner)
        .width(Length::Fill)
        .padding([12, 14])
        .style(card_style(palette))
        .into()
}

fn toggle_row(
    palette: StudioPalette,
    label: &str,
    value: bool,
    on_toggle: fn(bool) -> Message,
) -> Element<'_, Message> {
    row![
        text(label).size(13).color(palette.text),
        Space::with_width(Length::Fill),
        toggler(value).on_toggle(on_toggle).size(18),
    ]
    .align_y(Alignment::Center)
    .into()
}

fn slider_row(
    palette: StudioPalette,
    label: &str,
    value_text: String,
    range: std::ops::RangeInclusive<u32>,
    value: u32,
    on_change: fn(u32) -> Message,
) -> Element<'_, Message> {
    column![
        row![
            text(label).size(12.5).color(palette.text),
            Space::with_width(Length::Fill),
            text(value_text)
                .size(12)
                .font(iced::Font::MONOSPACE)
                .color(palette.accent),
        ]
        .align_y(Alignment::Center),
        slider(range, value, on_change).step(1_u32),
    ]
    .spacing(4)
    .into()
}
