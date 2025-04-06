use anyhow::{Context, Result};
use serde_json;
use std::fs;

use crate::domain::entities::TokenConfig;
use crate::domain::ports::ConfigRepository;

pub struct FileConfigRepository {
    config_path: String,
}

impl FileConfigRepository {
    pub fn new(config_path: String) -> Self {
        Self { config_path }
    }
}

impl ConfigRepository for FileConfigRepository {
    fn load_config(&self) -> Result<TokenConfig> {
        let config_file = fs::read_to_string(&self.config_path)
            .with_context(|| format!("Failed to read config file: {}", self.config_path))?;

        let config: TokenConfig = serde_json::from_str(&config_file)
            .with_context(|| format!("Failed to parse config file: {}", self.config_path))?;

        Ok(config)
    }
}
