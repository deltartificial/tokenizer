use std::path::PathBuf;
use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::application::use_cases::CountTokensUseCase;
use crate::domain::entities::TokenCount;
use crate::domain::ports::{ConfigRepository, TokenCounterService};

#[derive(Parser)]
#[command(name = "tokenizer")]
#[command(about = "A CLI tool to compute token lengths of various file types", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Count tokens in a file
    Count {
        /// Path to the file to analyze
        #[arg(required = true)]
        file: PathBuf,
        
        /// Path to config file (defaults to config.json in the current directory)
        #[arg(short, long, default_value = "config.json")]
        config: String,
    },
}

pub struct CliHandler<R, S>
where
    R: ConfigRepository,
    S: TokenCounterService,
{
    count_tokens_use_case: CountTokensUseCase<R, S>,
}

impl<R, S> CliHandler<R, S>
where
    R: ConfigRepository,
    S: TokenCounterService,
{
    pub fn new(count_tokens_use_case: CountTokensUseCase<R, S>) -> Self {
        Self { count_tokens_use_case }
    }
    
    pub fn run(&self, cli: Cli) -> Result<()> {
        match cli.command {
            Commands::Count { file, config: _ } => {
                let result = self.count_tokens_use_case.execute(&file)?;
                self.display_token_count(&result);
                Ok(())
            }
        }
    }
    
    fn display_token_count(&self, token_count: &TokenCount) {
        println!("File: {}", token_count.filename);
        println!("Type: {:?}", token_count.file_type);
        println!("\nToken counts by model:");
        println!("{:<20} {:<12} {:<20}", "Model", "Tokens", "% of Context Length");
        println!("{}", "-".repeat(60));
        
        for model_count in &token_count.token_counts {
            println!(
                "{:<20} {:<12} {:.2}%",
                model_count.model_name,
                model_count.token_count,
                model_count.percentage_of_context
            );
        }
    }
} 