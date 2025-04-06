use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::application::use_cases::CountTokensUseCase;
use crate::domain::entities::TokenCount;
use crate::domain::ports::{ConfigRepository, TokenCounterService};
use crate::infrastructure::token_counter::format_duration;

#[derive(Parser)]
#[command(name = "tokenizer")]
#[command(about = "A CLI tool to compute token lengths of various file types", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Count {
        #[arg(required = true)]
        file: PathBuf,

        #[arg(short, long, default_value = "config.json")]
        config: String,

        #[arg(short, long, default_value = "bert-base-uncased")]
        tokenizer: String,
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
        Self {
            count_tokens_use_case,
        }
    }

    pub fn run(&self, cli: Cli) -> Result<()> {
        match cli.command {
            Commands::Count {
                file, config: _, ..
            } => {
                let result = self.count_tokens_use_case.execute(&file)?;
                self.display_token_count(&result);
                Ok(())
            }
        }
    }

    fn display_token_count(&self, token_count: &TokenCount) {
        println!("\nResults for: {}", token_count.filename);
        println!("File type: {:?}", token_count.file_type);
        println!(
            "Processing time: {}",
            format_duration(token_count.processing_time)
        );
        println!("\nToken counts by model:");
        println!(
            "{:<20} {:<12} {:<20}",
            "Model", "Tokens", "% of Context Length"
        );
        println!("{}", "-".repeat(60));

        for model_count in &token_count.token_counts {
            println!(
                "{:<20} {:<12} {:.2}%",
                model_count.model_name, model_count.token_count, model_count.percentage_of_context
            );
        }
    }
}
