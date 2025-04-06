use anyhow::{Context as AnyhowContext, Result};
use html2text::from_read;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};
use tokenizers::tokenizer::Tokenizer;

use crate::domain::entities::{FileType, ModelTokenCount, TokenConfig, TokenCount};
use crate::domain::ports::TokenCounterService;

pub struct HuggingFaceTokenizerService {
    tokenizer: Tokenizer,
}

impl HuggingFaceTokenizerService {
    pub fn new() -> Result<Self> {
        Self::with_model("bert-base-uncased")
    }

    pub fn with_model(model_name: &str) -> Result<Self> {
        let tokenizer = Tokenizer::from_pretrained(model_name, None)
            .map_err(|e| anyhow::anyhow!("Failed to load {} tokenizer: {}", model_name, e))?;

        Ok(Self { tokenizer })
    }

    fn get_file_extension(filepath: &Path) -> Option<FileType> {
        filepath
            .extension()
            .and_then(|ext| ext.to_str())
            .map(FileType::from)
    }

    fn read_text_file(&self, filepath: &Path) -> Result<String> {
        fs::read_to_string(filepath)
            .with_context(|| format!("Failed to read text file: {}", filepath.display()))
    }

    fn read_pdf_file(&self, filepath: &Path) -> Result<String> {
        let content = format!("PDF content from: {}", filepath.display());

        Ok(content)
    }

    fn read_html_file(&self, filepath: &Path) -> Result<String> {
        let html_content = fs::read_to_string(filepath)
            .with_context(|| format!("Failed to read HTML file: {}", filepath.display()))?;

        let text_content = from_read(html_content.as_bytes(), 80);

        Ok(text_content)
    }

    fn count_content_tokens(
        &self,
        content: &str,
        config: &TokenConfig,
    ) -> Result<Vec<ModelTokenCount>> {
        let encoding = self
            .tokenizer
            .encode(content, false)
            .map_err(|e| anyhow::anyhow!("Failed to encode content with tokenizer: {}", e))?;

        let token_count = encoding.get_tokens().len();

        let model_token_counts = config
            .models
            .iter()
            .map(|model| {
                let percentage = token_count as f64 / model.context_length as f64 * 100.0;
                ModelTokenCount {
                    model_name: model.name.clone(),
                    token_count,
                    percentage_of_context: percentage,
                }
            })
            .collect();

        Ok(model_token_counts)
    }
}

impl TokenCounterService for HuggingFaceTokenizerService {
    fn count_tokens(&self, filepath: &Path, config: &TokenConfig) -> Result<TokenCount> {
        let start_time = Instant::now();
        let file_type = Self::get_file_extension(filepath).unwrap_or(FileType::Unknown);

        let filename = filepath
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");

        let pb = ProgressBar::new(100);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {msg}")
                .unwrap()
                .progress_chars("█▇▆▅▄▃▂▁  "),
        );
        pb.set_message(format!("Processing {}...", filename));
        pb.set_position(10);

        let content = match file_type {
            FileType::Text | FileType::Markdown => {
                pb.set_message(format!("Reading {} as text...", filename));
                pb.set_position(20);
                let content = self.read_text_file(filepath)?;
                pb.set_position(40);
                content
            }
            FileType::Html => {
                pb.set_message(format!("Reading {} as HTML...", filename));
                pb.set_position(20);
                let content = self.read_html_file(filepath)?;
                pb.set_position(40);
                content
            }
            FileType::Pdf => {
                pb.set_message(format!("Reading {} as PDF...", filename));
                pb.set_position(20);
                let content = self.read_pdf_file(filepath)?;
                pb.set_position(40);
                content
            }
            FileType::Unknown => {
                pb.finish_with_message(format!("Unknown file type: {}", filename));
                return Err(anyhow::anyhow!(
                    "Unsupported file type: {}",
                    filepath.display()
                ));
            }
        };

        pb.set_message(format!("Tokenizing {} content...", filename));
        pb.set_position(60);
        let token_counts = self.count_content_tokens(&content, config)?;
        pb.set_position(90);

        pb.set_message(format!("Finalizing results for {}...", filename));
        let elapsed = start_time.elapsed();
        pb.finish_with_message(format!(
            "Processed {} in {}",
            filename,
            format_duration(elapsed)
        ));

        Ok(TokenCount {
            filename: filename.to_string(),
            file_type,
            token_counts,
            processing_time: elapsed,
        })
    }
}

pub fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs_f64();
    if secs < 1.0 {
        format!("{} ms", (secs * 1000.0).round() as u64)
    } else if secs < 60.0 {
        format!("{:.2} s", secs)
    } else {
        let minutes = (secs / 60.0).floor() as u64;
        let remaining_secs = secs - (minutes * 60) as f64;
        format!("{}m {:.2}s", minutes, remaining_secs)
    }
}
