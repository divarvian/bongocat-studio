use bongocat_core::DisplayMode;
use iced::widget::{Space, button, column, container, row, text};
use iced::{Alignment, Element, Length};

use crate::app::Message;
use crate::ui::theme::{
    ActionEmphasis, StudioPalette, action_button_style, badge_style, card_style, subtle_panel_style,
};

#[allow(clippy::too_many_arguments)]
pub fn view_status_bar<'a>(
    palette: StudioPalette,
    obs_url: &'a str,
    copied_feedback: bool,
    obs_clients: usize,
    device_count: usize,
    mode: DisplayMode,
    privacy_paused: bool,
    server_error: Option<&'a str>,
    input_error: Option<&'a str>,
) -> Element<'a, Message> {
    let (rec_w, rec_h) = mode.recommended_obs_size();

    let url_box = container(
        text(obs_url)
            .size(13)
            .font(iced::Font::MONOSPACE)
            .color(palette.text),
    )
    .padding([7, 11])
    .width(Length::Fill)
    .style(subtle_panel_style(palette));

    let copy_label = if copied_feedback {
        "Copied ✓"
    } else {
        "Copy URL"
    };

    let privacy_label = if privacy_paused {
        "Resume Input"
    } else {
        "Pause Privacy"
    };

    let top_row = row![
        url_box,
        button(text(copy_label).size(12.5))
            .padding([7, 14])
            .style(action_button_style(palette, ActionEmphasis::Primary))
            .on_press(Message::CopyObsUrl),
        button(text(privacy_label).size(12.5))
            .padding([7, 14])
            .style(action_button_style(
                palette,
                if privacy_paused {
                    ActionEmphasis::Danger
                } else {
                    ActionEmphasis::Neutral
                },
            ))
            .on_press(Message::TogglePrivacyPause),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let obs_color = if obs_clients > 0 {
        palette.success
    } else {
        palette.text_muted
    };
    let dev_color = if device_count > 0 {
        palette.accent
    } else {
        palette.warning
    };

    let meta_row = row![
        chip(
            palette,
            format!(
                "OBS: {} {}",
                obs_clients,
                if obs_clients == 1 {
                    "client"
                } else {
                    "clients"
                }
            ),
            obs_color,
        ),
        chip(
            palette,
            format!(
                "Input: {} {}",
                device_count,
                if device_count == 1 {
                    "device"
                } else {
                    "devices"
                }
            ),
            dev_color,
        ),
        Space::with_width(Length::Fill),
        text(format!("OBS Browser Size: {rec_w}×{rec_h}  ·  Live Sync"))
            .size(11.5)
            .color(palette.text_muted),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let mut content = column![
        text("OBS Browser Source (Fixed URL — Auto Live Sync)")
            .size(12)
            .color(palette.text_muted),
        top_row,
        meta_row,
    ]
    .spacing(9);

    if let Some(err) = server_error {
        content = content.push(warning_banner(palette, err));
    }
    if let Some(err) = input_error {
        content = content.push(warning_banner(palette, err));
    }

    container(content)
        .width(Length::Fill)
        .padding([13, 15])
        .style(card_style(palette))
        .into()
}

fn chip<'a>(palette: StudioPalette, label: String, color: iced::Color) -> Element<'a, Message> {
    let bg = iced::Color { a: 0.12, ..color };
    container(text(label).size(11.5).color(color))
        .padding([3, 9])
        .style(badge_style(bg, palette.border, color))
        .into()
}

fn warning_banner(palette: StudioPalette, msg: &str) -> Element<'_, Message> {
    container(text(msg).size(11.5).color(palette.danger))
        .width(Length::Fill)
        .padding([8, 10])
        .style(badge_style(
            palette.danger_soft,
            palette.danger,
            palette.danger,
        ))
        .into()
}
