use crate::Error;

#[derive(Debug, Clone, Default)]
pub struct EnvConfig {
    pub server_url: String,
    pub database_url: String,
}

fn env_var(key: &'static str) -> Result<String, Error> {
    std::env::var(key).map_err(|e| Error::Env(key.to_string(), e))
}

impl EnvConfig {
    pub fn from_env() -> Result<Self, Error> {
        Ok(Self {
            server_url: env_var("SERVER_URL")?,
            database_url: env_var("DATABASE_URL")?,
        })
    }
}

#[derive(Debug, Default)]
pub struct AppConfig {
    pub env: EnvConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            env: EnvConfig::from_env()?,
        })
    }
}
