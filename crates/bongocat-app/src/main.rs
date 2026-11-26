mod app;
mod autostart;
mod ipc;
mod tray;
mod ui;

use std::fs;
use std::path::PathBuf;

use bongocat_core::{AppConfig, InputEvent};
use bongocat_input::spawn_input_reader;
use bongocat_server::{ServerHub, spawn_server};
use clap::Parser;
use tokio::sync::{broadcast, mpsc};
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

use crate::app::{BongoCatApp, BootContext};

#[derive(Debug, Parser)]
#[command(
    name = "bongocat-studio",
    version,
    about = "Pure Rust desktop control center & built-in OBS Browser Source server for Bongo Cat"
)]
struct Cli {
    /// Start silently in the background (ideal for login autostart).
    #[arg(long)]
    minimized: bool,

    /// Override the loopback HTTP/SSE port for OBS Studio.
    #[arg(long)]
    port: Option<u16>,
}

struct SocketCleanup(Option<PathBuf>);

impl Drop for SocketCleanup {
    fn drop(&mut self) {
        if let Some(path) = self.0.take() {
            let _ = fs::remove_file(path);
        }
    }
}

fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::new("bongocat_studio=info,bongocat_server=info,bongocat_input=info,warn")
        }))
        .with_target(false)
        .init();

    let cli = Cli::parse();

    // Single-instance guard: relaunching brings the existing background window straight to front!
    if ipc::notify_existing_instance(cli.minimized) {
        info!("another BongoCat Studio instance is already running; brought it to front");
        return Ok(());
    }

    let mut config = match AppConfig::load_default() {
        Ok(cfg) => cfg,
        Err(err) => {
            warn!(error = %err, "failed to load config; using defaults");
            AppConfig::default()
        }
    };
    if let Some(port) = cli.port {
        config.port = port;
        config.clamp();
    }
    config.autostart = autostart::is_enabled();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .thread_name("bongocat-rt")
        .build()
        .expect("initialize Tokio runtime");

    let (input_tx, ui_input_rx) = broadcast::channel::<InputEvent>(256);
    let server_rx = input_tx.subscribe();
    let _input_handle = spawn_input_reader(runtime.handle(), input_tx.clone());

    let hub = ServerHub::new(config.port, config.overlay.clone());
    let (server_handle, server_error) =
        match runtime.block_on(spawn_server(config.port, hub.clone(), server_rx)) {
            Ok(handle) => (Some(handle), None),
            Err(err) => {
                warn!(error = %err, "OBS HTTP server could not bind");
                (None, Some(err.to_string()))
            }
        };

    let (tray_tx, tray_rx) = mpsc::unbounded_channel();
    let tray_handle = tray::spawn_tray(tray_tx);

    let (ipc_tx, ipc_rx) = mpsc::unbounded_channel();
    let _socket_guard = SocketCleanup(ipc::spawn_ipc_listener(runtime.handle(), ipc_tx));

    let boot = BootContext {
        config,
        start_minimized: cli.minimized,
        hub,
        input_rx: Some(ui_input_rx),
        tray_handle,
        tray_rx: Some(tray_rx),
        ipc_rx: Some(ipc_rx),
        server_error,
        server_handle,
        runtime,
    };

    iced::daemon(BongoCatApp::title, BongoCatApp::update, BongoCatApp::view)
        .subscription(BongoCatApp::subscription)
        .theme(BongoCatApp::theme)
        .run_with(move || BongoCatApp::new(boot))
}
