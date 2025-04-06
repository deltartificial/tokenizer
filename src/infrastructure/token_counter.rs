use anyhow::{Context as AnyhowContext, Result};
use std::fs;
use std::path::Path;
use tokenizers::tokenizer::Tokenizer;

use crate::domain::entities::{FileType, ModelTokenCount, TokenConfig, TokenCount};
use crate::domain::ports::TokenCounterService;

pub struct HuggingFaceTokenizerService {
    tokenizer: Tokenizer,
}

impl HuggingFaceTokenizerService {
    pub fn new() -> Result<Self> {
        // Use BERT as default model
        Self::with_model("bert-base-uncased")
    }

    pub fn with_model(model_name: &str) -> Result<Self> {
        // Initialize a tokenizer with the specified model
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
        // Simple implementation to read PDF text
        // For a more robust implementation, consider using a dedicated PDF text extraction library
        let content = format!("PDF content from: {}", filepath.display());

        // This is a placeholder - in a real implementation, you would extract
        // text from the PDF using a more sophisticated approach.
        Ok(content)
    }

    fn count_content_tokens(
        &self,
        content: &str,
        config: &TokenConfig,
    ) -> Result<Vec<ModelTokenCount>> {
        // Use the HuggingFace tokenizer to count tokens
        let encoding = self
            .tokenizer
            .encode(content, false)
            .map_err(|e| anyhow::anyhow!("Failed to encode content with tokenizer: {}", e))?;

        let token_count = encoding.get_tokens().len();

        // Map the token count to each model in the config
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
        let file_type = Self::get_file_extension(filepath).unwrap_or(FileType::Unknown);

        let content = match file_type {
            FileType::Text | FileType::Markdown => self.read_text_file(filepath)?,
            FileType::Pdf => self.read_pdf_file(filepath)?,
            FileType::Unknown => {
                return Err(anyhow::anyhow!(
                    "Unsupported file type: {}",
                    filepath.display()
                ))
            }
        };

        let token_counts = self.count_content_tokens(&content, config)?;

        Ok(TokenCount {
            filename: filepath
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown")
                .to_string(),
            file_type,
            token_counts,
        })
    }
}
