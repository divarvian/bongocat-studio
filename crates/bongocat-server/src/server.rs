use std::convert::Infallible;
use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

use axum::extract::{Request, State};
use axum::http::header::{CACHE_CONTROL, CONTENT_TYPE, HOST};
use axum::http::{HeaderValue, StatusCode};
use axum::middleware::{self, Next};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use bongocat_core::InputEvent;
use futures_core::Stream;
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, oneshot};
use tokio::task::JoinHandle;
use tracing::{info, warn};

use crate::hub::ServerHub;

const INDEX_HTML: &str = include_str!("../assets/index.html");
const OVERLAY_JS: &str = include_str!("../assets/overlay.js");
const FLAT_DESK_JS: &str = include_str!("../assets/flat_desk.js");
const SLANTED_DESK_JS: &str = include_str!("../assets/slanted_desk.js");

const SVG_BOTH_UP: &str = include_str!("../../../assets/svg/bongo-both-up.svg");
const SVG_LEFT_DOWN: &str = include_str!("../../../assets/svg/bongo-left-down.svg");
const SVG_RIGHT_DOWN: &str = include_str!("../../../assets/svg/bongo-right-down.svg");
const SVG_BOTH_DOWN: &str = include_str!("../../../assets/svg/bongo-both-down.svg");
const SVG_SLEEPING: &str = include_str!("../../../assets/svg/bongo-sleeping.svg");

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("failed to bind loopback HTTP server on 127.0.0.1:{port}: {source}")]
    Bind {
        port: u16,
        #[source]
        source: std::io::Error,
    },
}

#[derive(Debug)]
pub struct ServerHandle {
    pub addr: SocketAddr,
    shutdown_tx: Option<oneshot::Sender<()>>,
    server_task: JoinHandle<()>,
    bridge_task: JoinHandle<()>,
}

impl ServerHandle {
    pub fn shutdown(mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        self.bridge_task.abort();
        self.server_task.abort();
    }
}

impl Drop for ServerHandle {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        self.bridge_task.abort();
    }
}

/// Validates that the HTTP `Host` header targets localhost/loopback only (anti-DNS-rebinding).
pub fn is_allowed_host(host: &str) -> bool {
    let stripped = host
        .split(':')
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    matches!(stripped.as_str(), "127.0.0.1" | "localhost" | "[::1]")
}

async fn loopback_host_guard(req: Request, next: Next) -> Response {
    if let Some(host_hdr) = req.headers().get(HOST) {
        let allowed = host_hdr.to_str().is_ok_and(is_allowed_host);
        if !allowed {
            return (StatusCode::FORBIDDEN, "Loopback access only").into_response();
        }
    }
    next.run(req).await
}

pub fn build_router(hub: ServerHub) -> Router {
    Router::new()
        .route("/", get(serve_index))
        .route("/obs", get(serve_index))
        .route("/assets/overlay.js", get(serve_overlay_js))
        .route("/assets/flat_desk.js", get(serve_flat_desk_js))
        .route("/assets/slanted_desk.js", get(serve_slanted_desk_js))
        .route("/assets/bongo-both-up.svg", get(serve_svg_both_up))
        .route("/assets/bongo-left-down.svg", get(serve_svg_left_down))
        .route("/assets/bongo-right-down.svg", get(serve_svg_right_down))
        .route("/assets/bongo-both-down.svg", get(serve_svg_both_down))
        .route("/assets/bongo-sleeping.svg", get(serve_svg_sleeping))
        .route("/events", get(serve_sse))
        .route("/api/status", get(serve_status))
        .layer(middleware::from_fn(loopback_host_guard))
        .with_state(hub)
}

/// Binds the OBS HTTP + SSE server strictly to `127.0.0.1:<port>` and bridges local `InputEvent`s.
pub async fn spawn_server(
    port: u16,
    hub: ServerHub,
    mut input_rx: broadcast::Receiver<InputEvent>,
) -> Result<ServerHandle, ServerError> {
    let bind_addr = SocketAddr::from((Ipv4Addr::LOCALHOST, port));
    let listener = TcpListener::bind(bind_addr)
        .await
        .map_err(|source| ServerError::Bind { port, source })?;
    let actual_addr = listener
        .local_addr()
        .map_err(|source| ServerError::Bind { port, source })?;

    info!(addr = %actual_addr, "OBS HTTP/SSE overlay server listening");

    let hub_bridge = hub.clone();
    let bridge_task = tokio::spawn(async move {
        loop {
            match input_rx.recv().await {
                Ok(ev) => hub_bridge.publish_input(&ev),
                Err(broadcast::error::RecvError::Lagged(skipped)) => {
                    warn!(skipped, "OBS bridge lagged behind input stream");
                }
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    });

    let app = build_router(hub);
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    let server_task = tokio::spawn(async move {
        let server = axum::serve(listener, app).with_graceful_shutdown(async move {
            let _ = shutdown_rx.await;
        });
        if let Err(err) = server.await {
            warn!(error = %err, "OBS HTTP server stopped with error");
        }
    });

    Ok(ServerHandle {
        addr: actual_addr,
        shutdown_tx: Some(shutdown_tx),
        server_task,
        bridge_task,
    })
}

fn static_response(content_type: &'static str, body: &'static str) -> Response {
    let mut res = body.into_response();
    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static(content_type));
    res.headers_mut()
        .insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    res
}

async fn serve_index() -> Response {
    static_response("text/html; charset=utf-8", INDEX_HTML)
}

async fn serve_overlay_js() -> Response {
    static_response("application/javascript; charset=utf-8", OVERLAY_JS)
}

async fn serve_flat_desk_js() -> Response {
    static_response("application/javascript; charset=utf-8", FLAT_DESK_JS)
}

async fn serve_slanted_desk_js() -> Response {
    static_response("application/javascript; charset=utf-8", SLANTED_DESK_JS)
}

async fn serve_svg_both_up() -> Response {
    static_response("image/svg+xml", SVG_BOTH_UP)
}

async fn serve_svg_left_down() -> Response {
    static_response("image/svg+xml", SVG_LEFT_DOWN)
}

async fn serve_svg_right_down() -> Response {
    static_response("image/svg+xml", SVG_RIGHT_DOWN)
}

async fn serve_svg_both_down() -> Response {
    static_response("image/svg+xml", SVG_BOTH_DOWN)
}

async fn serve_svg_sleeping() -> Response {
    static_response("image/svg+xml", SVG_SLEEPING)
}

async fn serve_status(State(hub): State<ServerHub>) -> Json<bongocat_core::ServerStatus> {
    Json(hub.status())
}

async fn serve_sse(
    State(hub): State<ServerHub>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let (guard, mut rx) = hub.subscribe();
    let initial = hub.initial_snapshot_payloads();

    let (tx, stream_rx) = tokio::sync::mpsc::channel::<Result<Event, Infallible>>(128);

    tokio::spawn(async move {
        let _keep_client_counted = guard;
        for payload in initial {
            if tx.send(Ok(Event::default().data(payload))).await.is_err() {
                return;
            }
        }
        loop {
            match rx.recv().await {
                Ok(payload) => {
                    if tx.send(Ok(Event::default().data(payload))).await.is_err() {
                        break;
                    }
                }
                Err(broadcast::error::RecvError::Lagged(_)) => {}
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(stream_rx);
    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("ping"),
    )
}

#[cfg(test)]
mod tests {
    use bongocat_core::{DisplayMode, OverlayConfig, PrivacyMode};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    use super::*;

    #[test]
    fn host_header_validation_blocks_dns_rebinding() {
        assert!(is_allowed_host("127.0.0.1:17890"));
        assert!(is_allowed_host("localhost:17890"));
        assert!(is_allowed_host("127.0.0.1"));
        assert!(!is_allowed_host("evil.example.com:17890"));
        assert!(!is_allowed_host("192.168.1.50:17890"));
    }

    #[tokio::test]
    async fn server_serves_overlay_status_and_rejects_external_host() {
        let hub = ServerHub::new(0, OverlayConfig::default());
        let (tx, rx) = broadcast::channel(32);
        let handle = spawn_server(0, hub.clone(), rx)
            .await
            .expect("spawn test server");
        let addr = handle.addr;

        // 1. Valid localhost request to /api/status
        let mut s = TcpStream::connect(addr).await.unwrap();
        s.write_all(
            format!(
                "GET /api/status HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nConnection: close\r\n\r\n",
                addr.port()
            )
            .as_bytes(),
        )
        .await
        .unwrap();
        let mut resp = String::new();
        s.read_to_string(&mut resp).await.unwrap();
        assert!(resp.starts_with("HTTP/1.1 200 OK"));
        assert!(resp.contains("\"ok\":true"));

        // 2. DNS rebinding attempt rejected with 403
        let mut bad = TcpStream::connect(addr).await.unwrap();
        bad.write_all(
            b"GET /api/status HTTP/1.1\r\nHost: evil.attacker\r\nConnection: close\r\n\r\n",
        )
        .await
        .unwrap();
        let mut bad_resp = String::new();
        bad.read_to_string(&mut bad_resp).await.unwrap();
        assert!(bad_resp.starts_with("HTTP/1.1 403"));

        // 3. Privacy masking & live config sync over hub broadcast
        let cfg = OverlayConfig {
            privacy_mode: PrivacyMode::Zones,
            mode: DisplayMode::Classic,
            ..OverlayConfig::default()
        };
        let (_guard, mut sub) = hub.subscribe();
        assert_eq!(hub.client_count(), 1);
        hub.update_config(cfg);
        let cfg_msg = sub.recv().await.unwrap();
        assert!(cfg_msg.contains("\"t\":\"config\""));
        assert!(cfg_msg.contains("\"mode\":\"classic\""));

        // Sending 'P' (25) in Zones mode must NOT emit raw code 25!
        tx.send(InputEvent::Key {
            code: 25,
            down: true,
        })
        .unwrap();
        let key_msg = tokio::time::timeout(Duration::from_secs(1), sub.recv())
            .await
            .unwrap()
            .unwrap();
        assert!(key_msg.contains("\"t\":\"k\""));
        assert!(!key_msg.contains("\"c\":25"));

        handle.shutdown();
    }
}
