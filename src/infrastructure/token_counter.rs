use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use pdf::file::File as PdfFile;
use tiktoken_rs::cl100k_base;

use crate::domain::entities::{FileType, ModelTokenCount, TokenConfig, TokenCount};
use crate::domain::ports::TokenCounterService;

pub struct TiktokenCounterService {}

impl TiktokenCounterService {
    pub fn new() -> Self {
        Self {}
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
        let pdf = PdfFile::open(filepath)
            .with_context(|| format!("Failed to open PDF file: {}", filepath.display()))?;
        
        let mut content = String::new();
        
        for page_index in 0..pdf.num_pages() {
            if let Ok(page) = pdf.get_page(page_index) {
                if let Ok(text) = page.text() {
                    content.push_str(&text);
                    content.push('\n');
                }
            }
        }
        
        Ok(content)
    }

    fn count_content_tokens(&self, content: &str, config: &TokenConfig) -> Vec<ModelTokenCount> {
        let bpe = cl100k_base().expect("Failed to load cl100k_base tokenizer");
        let tokens = bpe.encode_with_special_tokens(content);
        let token_count = tokens.len();
        
        config
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
            .collect()
    }
}

impl TokenCounterService for TiktokenCounterService {
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
        
        let token_counts = self.count_content_tokens(&content, config);
        
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