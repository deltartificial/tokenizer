mod domain;
mod application;
mod infrastructure;
mod presentation;

use std::process;
use anyhow::Result;
use clap::Parser;
use env_logger;
use log::{error, info};

use application::use_cases::CountTokensUseCase;
use infrastructure::config::FileConfigRepository;
use infrastructure::token_counter::TiktokenCounterService;
use presentation::cli::{Cli, CliHandler, Commands};

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting tokenizer application");
    
    let cli = Cli::parse();
    
    // Extract config path from CLI
    let config_path = if let Commands::Count { config, .. } = &cli.command {
        config.clone()
    } else {
        "config.json".to_string()
    };
    
    // Set up dependencies
    let config_repository = FileConfigRepository::new(config_path);
    let token_counter_service = TiktokenCounterService::new();
    let count_tokens_use_case = CountTokensUseCase::new(config_repository, token_counter_service);
    
    // Create CLI handler and run
    let cli_handler = CliHandler::new(count_tokens_use_case);
    if let Err(e) = cli_handler.run(cli) {
        error!("Application error: {}", e);
        eprintln!("Error: {}", e);
        process::exit(1);
    }
    
    Ok(())
}
