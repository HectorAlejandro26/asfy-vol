pub mod fs_ops;
pub mod models;

use anyhow::Result;
use fs_ops::get_config_dir;
use models::IconThreshold;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    pub use_percent: bool,

    #[serde(default)]
    pub thresholds: Vec<IconThreshold>,

    /// `None` se asume la ruta predeterminada (`$XDG_CONFIG_HOME/asfy/asfy-vol/config.toml`)
    #[serde(skip)]
    config_file: Option<PathBuf>,
}

impl Config {
    pub fn setup(config_file: Option<PathBuf>) -> Result<Config> {
        if let Some(path) = config_file {
            return Self::load(&path);
        }

        // Buscamos la ruta por defecto
        let default_dir = get_config_dir()?;
        let default_file = default_dir.join("config.toml");

        if default_file.exists() {
            Self::load(&default_file)
        } else {
            // Si no existe, creamos los valores por defecto y guardamos el archivo
            let config = Self::default();
            if let Err(e) = config.init_default_file(&default_file) {
                eprintln!(
                    "Warning: Could not create default configuration file: {}",
                    e
                );
            }
            Ok(config)
        }
    }

    /// Pasamos la ruta exacta donde queremos guardar
    fn init_default_file(&self, path: &PathBuf) -> Result<()> {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                create_dir_all(parent)?;
            }
        }

        let content = toml::to_string_pretty(self)?;
        write(path, content)?;

        Ok(())
    }

    fn load(file: &PathBuf) -> Result<Self> {
        let content = read_to_string(file)?;
        let mut config: Config = toml::from_str(&content)?;
        config.config_file = Some(file.clone());
        Ok(config)
    }

    pub fn get_file_path(&self) -> Option<PathBuf> {
        self.config_file.clone()
    }
}

impl Default for Config {
    fn default() -> Self {
        let thresholds = vec![
            IconThreshold {
                icon: '\u{f026}',
                level: 0.15,
            },
            IconThreshold {
                icon: '\u{f027}',
                level: 0.425,
            },
            IconThreshold {
                icon: '\u{f028}',
                level: 0.425,
            },
        ];

        Self {
            thresholds,
            use_percent: false,
            config_file: None,
        }
    }
}
