use std::path::PathBuf;

use anyhow::Result;
use dirs::config_dir;

/// Devuelve un `PathBuf` que apunta hacia `$XDG_CONFIG_HOME/asfy/asfy-vol`, sin importar si existe
/// o no
pub fn get_config_dir() -> Result<PathBuf> {
    let config_dir_path = config_dir();
    let app_config_dir_path = config_dir_path.unwrap().join("asfy").join("asfy-vol");

    Ok(app_config_dir_path)
}
