use bongocat_core::{AppTheme, Preset};
use iced::widget::{Space, button, container, row, text};
use iced::{Alignment, Element, Length};

use crate::app::Message;
use crate::ui::theme::{
    ActionEmphasis, StudioPalette, action_button_style, badge_style, header_bar_style,
    pill_button_style,
};

pub fn view_header<'a>(
    palette: StudioPalette,
    app_theme: AppTheme,
    active_preset: Option<Preset>,
    privacy_paused: bool,
    server_ok: bool,
) -> Element<'a, Message> {
    let brand = row![
        text("BongoCat Studio")
            .size(17)
            .color(palette.text)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..iced::Font::DEFAULT
            }),
        status_pill(palette, privacy_paused, server_ok),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let mut preset_row = row![text("Presets").size(12).color(palette.text_muted)]
        .spacing(6)
        .align_y(Alignment::Center);

    for preset in Preset::ALL {
        let is_active = active_preset == Some(preset);
        preset_row = preset_row.push(
            button(text(preset.label()).size(12.5))
                .padding([5, 12])
                .style(pill_button_style(palette, is_active))
                .on_press(Message::ApplyPreset(preset)),
        );
    }

    if active_preset.is_none() {
        preset_row = preset_row.push(container(text("Custom").size(11.5)).padding([4, 9]).style(
            badge_style(palette.surface_alt, palette.border, palette.text_muted),
        ));
    }

    let theme_label = match app_theme {
        AppTheme::Dark => "Light",
        AppTheme::Light => "Dark",
    };

    let right_controls = row![
        button(text(theme_label).size(12.5))
            .padding([6, 12])
            .style(action_button_style(palette, ActionEmphasis::Neutral))
            .on_press(Message::ToggleAppTheme),
        button(text("Quit").size(12.5))
            .padding([6, 12])
            .style(action_button_style(palette, ActionEmphasis::Danger))
            .on_press(Message::QuitApp),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let bar = row![
        brand,
        Space::with_width(Length::Fill),
        preset_row,
        Space::with_width(Length::Fill),
        right_controls,
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    container(bar)
        .width(Length::Fill)
        .padding([11, 18])
        .style(header_bar_style(palette))
        .into()
}

fn status_pill<'a>(
    palette: StudioPalette,
    privacy_paused: bool,
    server_ok: bool,
) -> Element<'a, Message> {
    let (label, color) = if !server_ok {
        ("Port Busy", palette.danger)
    } else if privacy_paused {
        ("Privacy Paused", palette.warning)
    } else {
        ("Live", palette.success)
    };

    let bg = iced::Color { a: 0.16, ..color };
    container(text(format!("● {label}")).size(11.5))
        .padding([3, 9])
        .style(badge_style(bg, color, color))
        .into()
}
