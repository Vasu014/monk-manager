# Monk Manager - AI Programming Agent Specifications

## Overview
Monk Manager is an AI-powered programming assistant that helps developers understand, explain, and work with code. It uses advanced tracing and telemetry to provide insights into its operations and performance.

## Specifications Index

| Specification | Description |
|---------------|-------------|
| [Architecture](specs/architecture.md) | Overall system architecture and design |
| [Tracing](specs/tracing.md) | Logging, metrics, and telemetry implementation |
| [CLI Interface](specs/cli.md) | Command-based CLI interface (legacy) |
| [Interactive CLI](specs/cli-2.md) | Primary interactive chat interface |
| [Explain Command](specs/explain-command.md) | Implementation of the `monkexplain` command |
| [Edit Command](specs/edit-command.md) | Implementation of the `monk edit` command for file modifications |
| [AI Integration](specs/ai-integration.md) | AI model integration and interaction patterns |
| [Error Handling](specs/error-handling.md) | Error handling and reporting strategies |
| [Configuration](specs/configuration.md) | Configuration management and environment setup |
| [Testing](specs/testing.md) | Testing strategy and implementation guidelines |

## Getting Started
1. Clone the repository
2. Install dependencies with `cargo build`
3. Run `monk-manager --help` to see available commands

## Development Status
🚧 Under Development

## Contributing
Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests. 