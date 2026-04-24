pub mod fs_ops;
pub mod models;

use anyhow::Result;
use fs_ops::get_config_dir;
use models::IconThreshold;
use serde::Deserialize;
use std::{fs::create_dir_all, path::PathBuf};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub use_percent: bool,

    #[serde(default)]
    pub thresholds: Vec<IconThreshold>,

    #[serde(skip)]
    pub config_file: PathBuf,
}

impl Config {
    pub fn setup() -> Result<Config> {
        let config_dir_path = get_config_dir()?;
        let default_config = Config::default();

        create_dir_all(config_dir_path)?;

        Ok(default_config)
    }

    fn save(&self) {}
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
        }
    }
}
