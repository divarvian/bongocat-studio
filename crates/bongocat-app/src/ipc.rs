use std::fs;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream as StdUnixStream;
use std::path::PathBuf;
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixListener;
use tokio::sync::mpsc;
use tracing::{debug, warn};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcCommand {
    ShowWindow,
    Quit,
}

pub fn socket_path() -> PathBuf {
    let base = dirs::runtime_dir().unwrap_or_else(std::env::temp_dir);
    base.join("bongocat-studio.sock")
}

/// Attempts to contact an already-running instance.
/// Returns `true` if an existing instance handled our request (caller should exit cleanly).
pub fn notify_existing_instance(minimized: bool) -> bool {
    let path = socket_path();
    let Ok(mut stream) = StdUnixStream::connect(&path) else {
        return false;
    };
    let _ = stream.set_read_timeout(Some(Duration::from_millis(400)));
    let _ = stream.set_write_timeout(Some(Duration::from_millis(400)));

    let cmd = if minimized { b"PING\n" } else { b"SHOW\n" };
    if stream.write_all(cmd).is_err() {
        return false;
    }

    let mut reply = [0_u8; 8];
    matches!(stream.read(&mut reply), Ok(n) if n > 0)
}

/// Binds the single-instance Unix socket listener on the Tokio runtime and forwards commands.
pub fn spawn_ipc_listener(
    rt: &tokio::runtime::Handle,
    tx: mpsc::UnboundedSender<IpcCommand>,
) -> Option<PathBuf> {
    let path = socket_path();
    if path.exists() {
        let _ = fs::remove_file(&path);
    }

    let _guard = rt.enter();
    let listener = match UnixListener::bind(&path) {
        Ok(l) => l,
        Err(err) => {
            warn!(error = %err, "failed to bind single-instance IPC socket");
            return None;
        }
    };

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&path, fs::Permissions::from_mode(0o600));
    }

    debug!(path = %path.display(), "single-instance IPC socket ready");

    rt.spawn(async move {
        loop {
            let Ok((stream, _)) = listener.accept().await else {
                break;
            };
            let tx_conn = tx.clone();
            tokio::spawn(async move {
                let (read_half, mut write_half) = stream.into_split();
                let mut reader = BufReader::new(read_half);
                let mut line = String::new();
                if reader.read_line(&mut line).await.is_ok() {
                    match line.trim() {
                        "SHOW" => {
                            let _ = tx_conn.send(IpcCommand::ShowWindow);
                        }
                        "QUIT" => {
                            let _ = tx_conn.send(IpcCommand::Quit);
                        }
                        _ => {}
                    }
                    let _ = write_half.write_all(b"OK\n").await;
                }
            });
        }
    });

    Some(path)
}
