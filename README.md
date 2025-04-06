# Tokenizer

A CLI tool to compute token lengths of various file types (txt, md, pdf) for different LLM models.

## Features

- Calculate token counts for various file types (Text, Markdown, PDF)
- Support for multiple LLM models (configurable via config.json)
- Display token usage as percentage of context window

## Installation

Clone the repository and build the project:

```bash
git clone https://github.com/yourusername/tokenizer.git
cd tokenizer
cargo build --release
```

## Usage

```bash
# Count tokens in a file using the default config.json
./target/release/tokenizer count path/to/your/file.txt

# Count tokens using a custom config file
./target/release/tokenizer count path/to/your/file.txt -c custom-config.json
```

## Configuration

The tool uses a `config.json` file to define models and their context lengths. The default file looks like this:

```json
{
  "models": [
    {
      "name": "gpt-3.5-turbo",
      "context_length": 16385,
      "encoding": "cl100k_base"
    },
    {
      "name": "gpt-4",
      "context_length": 8192,
      "encoding": "cl100k_base"
    },
    ...
  ]
}
```

You can customize this file to add or modify models as needed.

## Project Structure

The project follows a clean architecture approach:

- `domain`: Core business logic and entities
- `application`: Use cases that orchestrate the domain logic
- `infrastructure`: External services implementation (file reading, tokenization)
- `presentation`: User interface (CLI)

## License

MIT
