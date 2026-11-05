pub mod database;
pub mod settings;

use config::{Config, ConfigError, Environment, File};
use thiserror::Error;

use settings::Settings;

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("Configuration loading failed: {0}")]
    Load(#[from] ConfigError),

    #[error("Invalid configuration: {0}")]
    Validation(String),
}

pub fn load() -> Result<Settings, SettingsError> {
    let _ = dotenvy::dotenv();

    let config = Config::builder()
        .add_source(
            File::with_name("config/default")
                .required(false),
        )
        .add_source(
            Environment::default()
                .separator("__")
                .try_parsing(true),
        )
        .build()?;

    let settings: Settings = config.try_deserialize()?;

    settings
        .validate()
        .map_err(SettingsError::Validation)?;

    Ok(settings)
}