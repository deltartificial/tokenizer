use anyhow::Result;
use std::path::Path;

use crate::domain::entities::TokenCount;
use crate::domain::ports::{ConfigRepository, TokenCounterService};

pub struct CountTokensUseCase<R, S> {
    config_repository: R,
    token_counter_service: S,
}

impl<R, S> CountTokensUseCase<R, S>
where
    R: ConfigRepository,
    S: TokenCounterService,
{
    pub fn new(config_repository: R, token_counter_service: S) -> Self {
        Self {
            config_repository,
            token_counter_service,
        }
    }

    pub fn execute(&self, filepath: &Path) -> Result<TokenCount> {
        let config = self.config_repository.load_config()?;
        self.token_counter_service.count_tokens(filepath, &config)
    }
}
