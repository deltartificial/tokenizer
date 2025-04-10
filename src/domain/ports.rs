use anyhow::Result;
use std::path::Path;

use crate::domain::entities::{TokenConfig, TokenCount};

pub trait ConfigRepository {
    fn load_config(&self) -> Result<TokenConfig>;
}

pub trait TokenCounterService {
    fn count_tokens(&self, filepath: &Path, config: &TokenConfig) -> Result<TokenCount>;
}
