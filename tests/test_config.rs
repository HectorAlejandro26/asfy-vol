use std::path::Path;

use anyhow::Result;
use asfy_vol::config::Config;

/// Setup para usuario nuevo (sin configuracion en `XDG_CONFIG_HOME`).
/// Es necesario borrar el archivo de configuracion por defecto para
/// que el test no de errores.
#[test]
fn test_setup_config() -> Result<()> {
    let config_res = Config::setup(None);
    assert!(config_res.is_ok());

    let config = config_res?;
    let default_config = Config::default();
    // dbg!(&config);

    assert_eq!(config.use_percent, default_config.use_percent);
    assert_eq!(config.thresholds, default_config.thresholds);
    assert_eq!(config.get_file_path(), default_config.get_file_path());

    Ok(())
}

#[test]
fn test_specific_config() -> Result<()> {
    let custom_file = Path::new("./tests/custom_config.toml").to_path_buf();
    dbg!(&custom_file.exists());
    let config_res = Config::setup(Some(custom_file));
    assert!(config_res.is_ok());

    let config = config_res?;
    let default_config = Config::default();

    // Cambiamos el campo `use_percent`
    assert_ne!(config.use_percent, default_config.use_percent);
    assert_eq!(config.thresholds, default_config.thresholds);
    assert_ne!(config.get_file_path(), default_config.get_file_path());

    Ok(())
}
