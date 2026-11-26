use std::fs;
use std::path::PathBuf;

use thiserror::Error;

const AUTOSTART_FILE: &str = "bongocat-studio.desktop";

#[derive(Debug, Error)]
pub enum AutostartError {
    #[error("could not locate user config directory")]
    NoConfigDir,
    #[error("I/O error managing autostart file: {0}")]
    Io(#[from] std::io::Error),
}

pub fn autostart_path() -> Result<PathBuf, AutostartError> {
    let base = dirs::config_dir().ok_or(AutostartError::NoConfigDir)?;
    Ok(base.join("autostart").join(AUTOSTART_FILE))
}

pub fn is_enabled() -> bool {
    autostart_path().is_ok_and(|p| p.exists())
}

pub fn set_enabled(enabled: bool) -> Result<(), AutostartError> {
    let path = autostart_path()?;
    if !enabled {
        if path.exists() {
            fs::remove_file(path)?;
        }
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let exe = std::env::current_exe()
        .ok()
        .and_then(|p| p.into_os_string().into_string().ok())
        .unwrap_or_else(|| "bongocat-studio".to_owned());

    let entry = format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=BongoCat Studio\n\
         Comment=Bongo Cat desktop control center & OBS Browser Source server\n\
         Exec={exe} --minimized\n\
         Icon=bongocat-studio\n\
         Terminal=false\n\
         X-GNOME-Autostart-enabled=true\n"
    );

    fs::write(path, entry)?;
    Ok(())
}
