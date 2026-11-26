use std::sync::Mutex;
use std::time::{Duration, Instant};

use bongocat_core::{
    AppConfig, AppTheme, CatState, DeskTheme, DeviceInfo, DisplayMode, EyeDirection, EyeStyle,
    InputEvent, PointerDevice, PointerDirection, Preset, PrivacyMode, RightPawMode,
};
use bongocat_server::{ServerHandle, ServerHub};
use iced::widget::{column, container, row};
use iced::{Element, Length, Size, Subscription, Task, Theme, clipboard, time, window};
use tokio::sync::{broadcast, mpsc};
use tracing::{info, warn};

use crate::autostart;
use crate::ipc::IpcCommand;
use crate::tray::{TrayAction, TrayHandle};
use crate::ui::header::view_header;
use crate::ui::preview::view_preview;
use crate::ui::settings::view_settings;
use crate::ui::status_bar::view_status_bar;
use crate::ui::theme::{StudioPalette, iced_theme, root_bg_style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsTab {
    Appearance,
    Behavior,
    System,
}

impl SettingsTab {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Appearance => "Appearance",
            Self::Behavior => "Behavior",
            Self::System => "Privacy & System",
        }
    }
}

pub struct BootContext {
    pub config: AppConfig,
    pub start_minimized: bool,
    pub hub: ServerHub,
    pub input_rx: Option<broadcast::Receiver<InputEvent>>,
    pub tray_handle: TrayHandle,
    pub tray_rx: Option<mpsc::UnboundedReceiver<TrayAction>>,
    pub ipc_rx: Option<mpsc::UnboundedReceiver<IpcCommand>>,
    pub server_error: Option<String>,
    pub server_handle: Option<ServerHandle>,
    pub runtime: tokio::runtime::Runtime,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Backend & lifecycle streams
    Input(InputEvent),
    Tray(TrayAction),
    Ipc(IpcCommand),
    Tick(Instant),
    WindowCloseRequested(window::Id),
    QuitApp,

    // Header & presets
    ApplyPreset(Preset),
    ToggleAppTheme,
    SelectTab(SettingsTab),

    // Quick actions
    CopyObsUrl,
    ClearCopiedFeedback,
    TogglePrivacyPause,
    SetPrivacyPause(bool),

    // Appearance settings
    SetDisplayMode(DisplayMode),
    SetPointerDevice(PointerDevice),
    SetDeskTheme(DeskTheme),
    SetEyesEnabled(bool),
    SetEyeStyle(EyeStyle),
    SetEyeDirection(EyeDirection),
    SetBounce(bool),
    SetShowKps(bool),
    SetShowHud(bool),
    SetShowCheckerboard(bool),

    // Behavior settings
    SetPointerDirection(PointerDirection),
    SetRightPawMode(RightPawMode),
    SetKeyHoldMs(u32),
    SetIdleRaiseSec(u32),
    SetSleepSec(u32),
    SetKpsThreshold(u32),

    // Privacy & System settings
    SetPrivacyMode(PrivacyMode),
    SetCloseToBackground(bool),
    SetAutostart(bool),
    ToggleFullLicense,
}

struct BackendReceivers {
    input: broadcast::Receiver<InputEvent>,
    tray: Option<mpsc::UnboundedReceiver<TrayAction>>,
    ipc: Option<mpsc::UnboundedReceiver<IpcCommand>>,
}

static BACKEND_RX: Mutex<Option<BackendReceivers>> = Mutex::new(None);

pub struct BongoCatApp {
    config: AppConfig,
    obs_url: String,
    cat: CatState,
    hub: ServerHub,
    tray: TrayHandle,
    main_window: Option<window::Id>,
    active_tab: SettingsTab,
    show_full_license: bool,
    devices: Vec<DeviceInfo>,
    server_error: Option<String>,
    input_error: Option<String>,
    copied_until: Option<Instant>,
    last_tick: Instant,
    _server_handle: Option<ServerHandle>,
    _runtime: tokio::runtime::Runtime,
}

impl BongoCatApp {
    pub fn new(mut boot: BootContext) -> (Self, Task<Message>) {
        let now = Instant::now();
        let obs_url = boot.config.obs_url();
        let cat = CatState::new(boot.config.overlay.clone(), now);

        if let Ok(mut guard) = BACKEND_RX.lock()
            && let Some(input) = boot.input_rx.take()
        {
            *guard = Some(BackendReceivers {
                input,
                tray: boot.tray_rx.take(),
                ipc: boot.ipc_rx.take(),
            });
        }

        let (main_window, open_task) = if boot.start_minimized {
            info!("started minimized in background for OBS Studio");
            (None, Task::none())
        } else {
            let (id, task) = open_main_window();
            (Some(id), task)
        };

        let initial_devices = boot.hub.status().devices;
        let app = Self {
            config: boot.config,
            obs_url,
            cat,
            hub: boot.hub,
            tray: boot.tray_handle,
            main_window,
            active_tab: SettingsTab::Appearance,
            show_full_license: false,
            devices: initial_devices,
            server_error: boot.server_error,
            input_error: None,
            copied_until: None,
            last_tick: now,
            _server_handle: boot.server_handle,
            _runtime: boot.runtime,
        };

        (app, open_task)
    }

    pub fn title(&self, _window: window::Id) -> String {
        if self.cat.privacy_paused {
            "BongoCat Studio — Privacy Paused".to_owned()
        } else {
            "BongoCat Studio".to_owned()
        }
    }

    pub fn theme(&self, _window: window::Id) -> Theme {
        iced_theme(self.config.app_theme)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let now = Instant::now();

        match message {
            Message::Input(ev) => {
                match &ev {
                    InputEvent::DevicesUpdated(list) => {
                        self.devices.clone_from(list);
                    }
                    InputEvent::InputError(msg) => {
                        self.input_error = if msg.is_empty() {
                            None
                        } else {
                            Some(msg.clone())
                        };
                    }
                    _ => self.cat.handle_event(&ev, now),
                }
                Task::none()
            }
            Message::Tray(action) => match action {
                TrayAction::ShowWindow => self.show_or_focus_window(),
                TrayAction::ApplyPreset(preset) => {
                    self.config.apply_preset(preset);
                    self.sync_overlay_and_save();
                    Task::none()
                }
                TrayAction::TogglePrivacy => {
                    let next = !self.cat.privacy_paused;
                    self.apply_privacy_pause(next);
                    Task::none()
                }
                TrayAction::CopyObsUrl => self.copy_obs_url(),
                TrayAction::Quit => iced::exit(),
            },
            Message::Ipc(cmd) => match cmd {
                IpcCommand::ShowWindow => self.show_or_focus_window(),
                IpcCommand::Quit => iced::exit(),
            },
            Message::Tick(tick_now) => {
                let dt = tick_now
                    .saturating_duration_since(self.last_tick)
                    .as_secs_f32();
                self.last_tick = tick_now;
                self.cat.tick(tick_now, dt);
                if let Some(until) = self.copied_until
                    && tick_now >= until
                {
                    self.copied_until = None;
                }
                Task::none()
            }
            Message::WindowCloseRequested(id) => {
                if self.config.close_to_background {
                    info!("closing control center window to background (OBS server stays live)");
                    if self.main_window == Some(id) {
                        self.main_window = None;
                    }
                    window::close(id)
                } else {
                    iced::exit()
                }
            }
            Message::QuitApp => iced::exit(),

            Message::ApplyPreset(preset) => {
                self.config.apply_preset(preset);
                self.sync_overlay_and_save();
                Task::none()
            }
            Message::ToggleAppTheme => {
                self.config.app_theme = match self.config.app_theme {
                    AppTheme::Dark => AppTheme::Light,
                    AppTheme::Light => AppTheme::Dark,
                };
                self.save_config();
                Task::none()
            }
            Message::SelectTab(tab) => {
                self.active_tab = tab;
                Task::none()
            }

            Message::CopyObsUrl => self.copy_obs_url(),
            Message::ClearCopiedFeedback => {
                self.copied_until = None;
                Task::none()
            }
            Message::TogglePrivacyPause => {
                let next = !self.cat.privacy_paused;
                self.apply_privacy_pause(next);
                Task::none()
            }
            Message::SetPrivacyPause(paused) => {
                self.apply_privacy_pause(paused);
                Task::none()
            }

            Message::SetDisplayMode(mode) => {
                self.config.overlay.mode = mode;
                self.sync_overlay_and_save();
                Task::none()
            }
            Message::SetPointerDevice(ptr) => {
                self.config.overlay.pointer = ptr;
                self.sync_overlay_and_save();
                Task::none()
            }
            Message::SetDeskTheme(dt) => {
                self.config.overlay.desk_theme = dt;
                self.sync_overlay_and_save();
                Task::none()
            }
            Message::SetEyesEnabled(enabled) => {
                self.config.overlay.eyes_enabled = enabled;
                self.sync_overlay_and_save();
                Task::none()
            }
            Message::SetEyeStyle(style) => {
                self.config.overlay.eye_style = style;
                self.sync_overlay_and_save();
                Task::none()
            }
            Message::SetEyeDirection(dir) => {
                self.config.overlay.eye_dir = dir;
                self.sync_overlay_and_save();
                Task::none()
            }
            Message::SetBounce(v) => {
                self.config.overlay.bounce = v;
                self.sync_overlay_and_save();
                Task::none()
            }
            Message::SetShowKps(v) => {
                self.config.overlay.show_kps = v;
                self.sync_overlay_and_save();
                Task::none()
            }
            Message::SetShowHud(v) => {
                self.config.overlay.show_hud = v;
                self.sync_overlay_and_save();
                Task::none()
            }
            Message::SetShowCheckerboard(v) => {
                self.config.show_checkerboard = v;
                self.save_config();
                Task::none()
            }

            Message::SetPointerDirection(dir) => {
                self.config.overlay.ptr_dir = dir;
                self.sync_overlay_and_save();
                Task::none()
            }
            Message::SetRightPawMode(rp) => {
                self.config.overlay.right_paw = rp;
                self.sync_overlay_and_save();
                Task::none()
            }
            Message::SetKeyHoldMs(ms) => {
                self.config.overlay.key_hold_ms = ms;
                self.sync_overlay_and_save();
                Task::none()
            }
            Message::SetIdleRaiseSec(sec) => {
                self.config.overlay.idle_raise_sec = sec;
                self.sync_overlay_and_save();
                Task::none()
            }
            Message::SetSleepSec(sec) => {
                self.config.overlay.sleep_sec = sec;
                self.sync_overlay_and_save();
                Task::none()
            }
            Message::SetKpsThreshold(kps) => {
                self.config.overlay.kps_happy_threshold = kps;
                self.sync_overlay_and_save();
                Task::none()
            }

            Message::SetPrivacyMode(pm) => {
                self.config.overlay.privacy_mode = pm;
                self.sync_overlay_and_save();
                Task::none()
            }
            Message::SetCloseToBackground(v) => {
                self.config.close_to_background = v;
                self.save_config();
                Task::none()
            }
            Message::SetAutostart(enabled) => {
                match autostart::set_enabled(enabled) {
                    Ok(()) => {
                        self.config.autostart = enabled;
                        self.save_config();
                    }
                    Err(err) => {
                        warn!(error = %err, "failed to update autostart desktop entry");
                    }
                }
                Task::none()
            }
            Message::ToggleFullLicense => {
                self.show_full_license = !self.show_full_license;
                Task::none()
            }
        }
    }

    pub fn view(&self, _window: window::Id) -> Element<'_, Message> {
        let palette = StudioPalette::for_theme(self.config.app_theme);

        let header = view_header(
            palette,
            self.config.app_theme,
            self.config.active_preset(),
            self.cat.privacy_paused,
            self.server_error.is_none(),
        );

        let left_pane = column![
            view_preview(palette, &self.cat, self.config.show_checkerboard),
            view_status_bar(
                palette,
                &self.obs_url,
                self.copied_until.is_some(),
                self.hub.client_count(),
                self.devices.len(),
                self.config.overlay.mode,
                self.cat.privacy_paused,
                self.server_error.as_deref(),
                self.input_error.as_deref(),
            ),
        ]
        .spacing(12)
        .width(Length::FillPortion(13))
        .height(Length::Fill);

        let right_pane = container(view_settings(
            palette,
            self.active_tab,
            &self.config,
            self.cat.privacy_paused,
            &self.devices,
            self.show_full_license,
        ))
        .width(Length::FillPortion(10))
        .height(Length::Fill);

        let workspace = row![left_pane, right_pane]
            .spacing(14)
            .padding(14)
            .width(Length::Fill)
            .height(Length::Fill);

        container(
            column![header, workspace]
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(root_bg_style(palette))
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let mut subs = vec![
            window::close_requests().map(Message::WindowCloseRequested),
            Subscription::run(backend_event_stream),
        ];

        // Adaptive tick cadence: 60fps only when visible & animating; low-power when idle or hidden!
        if self.main_window.is_some() {
            let interval = if self.cat.is_animating() {
                Duration::from_millis(16)
            } else {
                Duration::from_millis(350)
            };
            subs.push(time::every(interval).map(Message::Tick));
        }

        Subscription::batch(subs)
    }

    fn show_or_focus_window(&mut self) -> Task<Message> {
        if let Some(id) = self.main_window {
            window::gain_focus(id)
        } else {
            let (id, task) = open_main_window();
            self.main_window = Some(id);
            task
        }
    }

    fn copy_obs_url(&mut self) -> Task<Message> {
        self.copied_until = Some(Instant::now() + Duration::from_millis(1800));
        let write_clip = clipboard::write(self.obs_url.clone());
        let reset_badge = Task::perform(
            async {
                tokio::time::sleep(Duration::from_millis(1850)).await;
            },
            |()| Message::ClearCopiedFeedback,
        );
        Task::batch([write_clip, reset_badge])
    }

    fn apply_privacy_pause(&mut self, paused: bool) {
        self.cat.set_privacy_paused(paused);
        self.hub.set_privacy_paused(paused);
        self.tray.set_privacy_paused(paused);
    }

    fn sync_overlay_and_save(&mut self) {
        self.config.clamp();
        self.cat.set_config(self.config.overlay.clone());
        self.hub.update_config(self.config.overlay.clone());
        self.save_config();
    }

    fn save_config(&self) {
        if let Err(err) = self.config.save_default() {
            warn!(error = %err, "failed to save config.toml");
        }
    }
}

fn open_main_window() -> (window::Id, Task<Message>) {
    let (id, open) = window::open(window::Settings {
        size: Size::new(1080.0, 690.0),
        min_size: Some(Size::new(900.0, 580.0)),
        exit_on_close_request: false,
        icon: crate::tray::build_window_icon(),
        platform_specific: window::settings::PlatformSpecific {
            // Matches bongocat-studio.desktop so Wayland/GNOME/KDE/Ubuntu taskbars
            // display the crisp SVG app icon and hover title "BongoCat Studio"!
            application_id: "bongocat-studio".to_owned(),
            override_redirect: false,
        },
        ..window::Settings::default()
    });
    (id, open.discard())
}

fn backend_event_stream() -> impl iced::futures::Stream<Item = Message> {
    iced::stream::channel(128, |mut output| async move {
        use iced::futures::SinkExt;

        let receivers = BACKEND_RX.lock().ok().and_then(|mut g| g.take());
        let Some(mut rx) = receivers else {
            std::future::pending::<()>().await;
            return;
        };

        loop {
            tokio::select! {
                res = rx.input.recv() => {
                    match res {
                        Ok(ev) => {
                            let _ = output.send(Message::Input(ev)).await;
                        }
                        Err(broadcast::error::RecvError::Lagged(_)) => {}
                        Err(broadcast::error::RecvError::Closed) => break,
                    }
                }
                Some(action) = async {
                    match &mut rx.tray {
                        Some(r) => r.recv().await,
                        None => std::future::pending().await,
                    }
                } => {
                    let _ = output.send(Message::Tray(action)).await;
                }
                Some(cmd) = async {
                    match &mut rx.ipc {
                        Some(r) => r.recv().await,
                        None => std::future::pending().await,
                    }
                } => {
                    let _ = output.send(Message::Ipc(cmd)).await;
                }
            }
        }
    })
}
