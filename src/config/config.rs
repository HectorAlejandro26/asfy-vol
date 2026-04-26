use crate::config::{DEFAULT_STYLE, models::IconThreshold};
use anyhow::Result;
use dirs::config_dir;
use gtk4::CssProvider;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::PathBuf,
};

/// Devuelve un `PathBuf` que apunta hacia `$XDG_CONFIG_HOME/asfy/asfy-vol`, sin importar si existe
/// o no
pub fn get_config_dir() -> Result<PathBuf> {
    let config_dir_path = config_dir();
    let app_config_dir_path = config_dir_path.unwrap().join("asfy").join("asfy-vol");

    Ok(app_config_dir_path)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Config {
    pub use_percent: bool,

    #[serde(default)]
    pub thresholds: Vec<IconThreshold>,

    style_path: Option<PathBuf>,

    /// `None` se asume la ruta predeterminada (`$XDG_CONFIG_HOME/asfy/asfy-vol/config.toml`)
    #[serde(skip)]
    config_file: Option<PathBuf>,
}

impl Config {
    pub fn setup(config_file: Option<PathBuf>) -> Result<Self> {
        if let Some(path) = config_file {
            return Self::load(&path);
        }

        let default_dir = get_config_dir()?;
        let default_file = default_dir.join("config.toml");
        let style_path = default_dir.join("style.css");

        if default_file.exists() {
            Self::load(&default_file)
        } else {
            let mut config = Self::default();
            if let Err(e) = config.init_default_files(&default_file) {
                eprintln!(
                    "Warning: Could not create default configuration file: {}",
                    e
                );
            }

            config.style_path = if style_path.exists() && style_path.is_file() {
                Some(style_path)
            } else {
                None
            };

            Ok(config)
        }
    }

    /// Pasamos la ruta exacta donde queremos guardar
    fn init_default_files(&self, path: &PathBuf) -> Result<()> {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                create_dir_all(parent)?;
            }

            // Intentamos crear el archivo CSS por defecto
            let style_path = parent.join("style.css");
            if let Err(e) = write(&style_path, DEFAULT_STYLE) {
                eprintln!("Warning: Could not create default style.css: {}", e);
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

        // Siempre buscar el style.css en el mismo directorio que el config cargado
        if let Some(parent) = file.parent() {
            let style = parent.join("style.css");
            if style.exists() && style.is_file() {
                config.style_path = Some(style);
            }
        }

        Ok(config)
    }

    pub fn get_file_path(&self) -> Option<PathBuf> {
        self.config_file.clone()
    }

    pub fn map_vol_icon(&self, val: f64) -> String {
        let mut threshold = 0_f64;
        for icon in &self.thresholds {
            threshold += icon.level;
            if val <= threshold {
                return icon.icon.to_string();
            }
        }
        self.thresholds
            .last()
            .map(|i| i.icon)
            .unwrap_or(' ')
            .to_string()
    }

    pub fn get_css_provider(&self) -> CssProvider {
        let provider = CssProvider::new();

        provider.load_from_string(DEFAULT_STYLE);

        if let Some(path) = &self.style_path {
            if path.exists() && path.is_file() {
                provider.load_from_path(path);
            }
        }

        provider
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
            style_path: None,
        }
    }
}
