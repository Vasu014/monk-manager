# Monk Manager

A high-performance CLI tool for AI-powered code explanation and documentation generation, built in Rust.

## Features

- 🤖 AI-powered code explanation using Claude
- 📝 Multiple output formats (Markdown, Plain Text)
- 🔍 Automatic language detection
- ⚡ High-performance implementation in Rust
- 🔒 Secure credential management
- 📊 Comprehensive logging and tracing
- 🎯 Flexible configuration system

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/monk-manager.git
cd monk-manager

# Build the project
cargo build --release

# Install globally
cargo install --path .
```

### From Binary

Download the latest release from the [releases page](https://github.com/yourusername/monk-manager/releases).

## Configuration

Monk Manager supports multiple configuration formats (TOML, JSON, YAML). Create a configuration file in one of these locations:

- `./monk.toml`
- `./monk.json`
- `./monk.yaml`
- `~/.config/monk-manager/config.yaml`

Example configuration (YAML):

```yaml
ai:
  provider: anthropic
  model_name: claude-3-sonnet-20240229
  api_key: your-api-key
  temperature: 0.7
  max_tokens: 1024

logging:
  level: info
  format: pretty
  output: stderr

commands:
  default_language: rust
  default_format: markdown
  timeout: 30
  explain:
    max_context_lines: 10
    language_detection: true

security:
  secrets_file: null
```

Environment variables can override configuration:
- `MONK_CONFIG`: Path to config file
- `ANTHROPIC_API_KEY`: AI API key
- `MONK_LOG_LEVEL`: Logging level

## Usage

### Explain Code

```bash
# Basic usage
monk explain src/main.rs

# Specify language
monk explain src/main.rs --language rust

# Custom output format
monk explain src/main.rs --format plain

# Include context lines
monk explain src/main.rs --context-lines 10
```

### Output Formats

#### Markdown
```markdown
# Code Explanation

## File: src/main.rs

## Language: rust

## Explanation

[AI-generated explanation]
```

#### Plain Text
```
File: src/main.rs
Language: rust

Explanation:
[AI-generated explanation]
```

## Development

### Prerequisites

- Rust 1.70 or higher
- Cargo
- Anthropic API key

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- explain src/main.rs
```

### Project Structure

```
monk-manager/
├── src/
│   ├── ai/           # AI integration
│   ├── cli/          # Command-line interface
│   ├── config/       # Configuration management
│   ├── error/        # Error handling
│   └── tracing/      # Logging and tracing
├── specs/            # Project specifications
└── tests/            # Integration tests
```

## Performance

Monk Manager is built in Rust for optimal performance:

- 🚀 Fast startup time (~10ms)
- 💾 Low memory usage (~5-10MB)
- ⚡ Efficient file processing
- 🔄 Optimized concurrent operations
- 🌐 Efficient network handling

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Claude](https://www.anthropic.com/) for AI capabilities
- [Rust](https://www.rust-lang.org/) for the programming language
- [Clap](https://github.com/clap-rs/clap) for CLI parsing
- [Tokio](https://tokio.rs/) for async runtime
- [Tracing](https://github.com/tokio-rs/tracing) for logging
- Geoffrey Huntley's article ["You are using Cursor AI incorrectly..."](https://ghuntley.com/stdlib/) for insights on effectively using AI coding assistants.

## Roadmap

- [ ] Support for multiple AI providers
- [ ] Interactive mode
- [ ] Code review assistance
- [ ] Documentation generation
- [ ] Integration with IDEs
- [ ] Web interface
- [ ] Plugin system

## Support

For support, please:
1. Check the [documentation](docs/)
2. Search [existing issues](https://github.com/yourusername/monk-manager/issues)
3. Create a new issue if needed

## Security

- API keys are stored securely
- No sensitive data is logged
- All dependencies are audited
- Regular security updates

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes.
