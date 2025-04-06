use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenModel {
    pub name: String,
    pub context_length: usize,
    pub encoding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConfig {
    pub models: Vec<TokenModel>,
}

#[derive(Debug, Clone)]
pub struct TokenCount {
    pub filename: String,
    pub file_type: FileType,
    pub token_counts: Vec<ModelTokenCount>,
}

#[derive(Debug, Clone)]
pub struct ModelTokenCount {
    pub model_name: String,
    pub token_count: usize,
    pub percentage_of_context: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    Text,
    Markdown,
    Pdf,
    Unknown,
}

impl From<&str> for FileType {
    fn from(extension: &str) -> Self {
        match extension.to_lowercase().as_str() {
            "txt" => FileType::Text,
            "md" => FileType::Markdown,
            "pdf" => FileType::Pdf,
            _ => FileType::Unknown,
        }
    }
}
