use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,

    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

fn default_max_connections() -> u32 {
    10
}

impl DatabaseConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.url.trim().is_empty() {
            return Err("DATABASE_URL cannot be empty".to_string());
        }

        if self.max_connections == 0 {
            return Err(
                "DATABASE_MAX_CONNECTIONS must be greater than zero".to_string()
            );
        }

        Ok(())
    }
}