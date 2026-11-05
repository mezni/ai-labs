use serde::Deserialize;

use super::database::DatabaseConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub app_env: String,

    pub database: DatabaseConfig,
}

impl Settings {
    pub fn validate(&self) -> Result<(), String> {
        if self.app_env.trim().is_empty() {
            return Err("APP_ENV cannot be empty".to_string());
        }

        self.database.validate()
    }

    pub fn is_production(&self) -> bool {
        self.app_env.eq_ignore_ascii_case("production")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::database::DatabaseConfig;

    #[test]
    fn production_environment_is_detected() {
        let settings = Settings {
            app_env: "production".to_string(),
            database: DatabaseConfig {
                url: "postgres://localhost/bio_com".to_string(),
                max_connections: 10,
            },
        };

        assert!(settings.is_production());
    }

    #[test]
    fn invalid_database_configuration_is_rejected() {
        let settings = Settings {
            app_env: "development".to_string(),
            database: DatabaseConfig {
                url: "".to_string(),
                max_connections: 10,
            },
        };

        assert!(settings.validate().is_err());
    }
}