# Tokenizer

A CLI tool to compute token lengths of various file types (txt, md, pdf, html) for different LLM models.

## Features

- Calculate token counts for various file types (Text, Markdown, PDF, HTML)
- Support for multiple LLM models (configurable via config.json)
- Display token usage as percentage of context window
- Powered by HuggingFace tokenizers library

## Installation

Clone the repository and build the project:

```bash
git clone https://github.com/deltartificial/tokenizer.git
cd tokenizer
cargo build --release
```

## Usage

```bash
# Count tokens in a file using the default config.json
./target/release/tokenizer count path/to/your/file.txt

# Count tokens using a custom config file
./target/release/tokenizer count path/to/your/file.txt -c custom-config.json

# Count tokens using a specific tokenizer model
./target/release/tokenizer count path/to/your/file.html -t roberta-base
```

## Configuration

The tool uses a `config.json` file to define models and their context lengths. The default file includes configurations for various models:

```json
{
  "models": [
    {
      "name": "gpt-3.5-turbo",
      "context_length": 16385,
      "encoding": "tiktoken"
    },
    {
      "name": "gpt-4",
      "context_length": 8192,
      "encoding": "tiktoken"
    },
    {
      "name": "bert-base",
      "context_length": 512,
      "encoding": "bert"
    },
    ...
  ]
}
```

You can customize this file to add or modify models as needed.

## Tokenization

This tool uses HuggingFace's tokenizers library, which provides high-performance implementations of various tokenization algorithms. The default tokenizer used is BERT, but the architecture is designed to be easily extended to support different tokenizers.

## Supported File Types

- `.txt` - Plain text files
- `.md` - Markdown files
- `.pdf` - PDF documents (basic implementation)
- `.html`/`.htm` - HTML files (tags are stripped for token counting)

## Project Structure

The project follows a clean architecture approach:

- `domain`: Core business logic and entities
- `application`: Use cases that orchestrate the domain logic
- `infrastructure`: External services implementation (file reading, tokenization)
- `presentation`: User interface (CLI)

## License

MIT
