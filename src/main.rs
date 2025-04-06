mod application;
mod domain;
mod infrastructure;
mod presentation;

use anyhow::Result;
use clap::Parser;
use env_logger;
use log::{error, info};
use std::process;

use application::use_cases::CountTokensUseCase;
use infrastructure::config::FileConfigRepository;
use infrastructure::token_counter::HuggingFaceTokenizerService;
use presentation::cli::{Cli, CliHandler, Commands};

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting tokenizer application");

    let cli = Cli::parse();

    let (config_path, tokenizer_model) = match &cli.command {
        Commands::Count {
            config, tokenizer, ..
        } => (config.clone(), tokenizer.clone()),
    };

    info!("Using tokenizer model: {}", tokenizer_model);

    let config_repository = FileConfigRepository::new(config_path);
    let token_counter_service = HuggingFaceTokenizerService::with_model(&tokenizer_model)?;
    let count_tokens_use_case = CountTokensUseCase::new(config_repository, token_counter_service);

    let cli_handler = CliHandler::new(count_tokens_use_case);
    if let Err(e) = cli_handler.run(cli) {
        error!("Application error: {}", e);
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    Ok(())
}
