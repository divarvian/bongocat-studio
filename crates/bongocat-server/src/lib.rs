//! Built-in loopback HTTP & SSE server serving the embedded OBS Studio Browser Source
//! overlay with live configuration synchronization.

pub mod hub;
pub mod server;

pub use hub::ServerHub;
pub use server::{ServerError, ServerHandle, spawn_server};
