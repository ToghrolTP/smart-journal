# Changelog

All notable changes to Rusty Diary will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Search functionality across journal entries
- Export options (PDF, HTML, etc.)
- Encryption support for journal entries
- Custom themes and color schemes
- Plugin system for extensibility
- Cloud synchronization options
- Writing statistics and insights
- Journal entry templates

## [0.1.0] - 2024-07-20

### Added
- 🎨 Beautiful terminal user interface (TUI) with ratatui
- 📝 Multiple file format support (Markdown, Plain Text, JSON)
- 🤖 LLM integration with Ollama for automatic markdown formatting
- ⚙️ Comprehensive configuration system with TOML config files
- 📅 Automatic date-based file organization (YYYY-MM-DD format)
- 🔍 Journal entry browser with content preview
- 💾 Persistent configuration in `~/.config/rusty_diary/config.toml`
- 🖥️ Cross-platform support (Linux, macOS, Windows)
- ⌨️ Command-line interface for quick journal entry creation
- 📂 Configurable journal directory (default: `~/Documents/RustyDiary/`)
- 🎮 Intuitive keyboard navigation in TUI mode
- 🔧 Interactive settings screen for configuration management
- 📋 Comprehensive help system with usage examples
- 🛠️ Backup and recovery options for existing files
- ✨ Smart file handling with append/overwrite options

### Features
- **TUI Mode**: Interactive terminal interface with menu navigation
- **CLI Mode**: Quick journal entry creation from command line
- **Settings Management**: Real-time configuration through TUI or config file
- **Multiple Formats**: 
  - Markdown with LLM processing for beautiful formatting
  - Plain text for simple, fast entries
  - JSON for structured data with metadata
- **Date Organization**: Automatic file naming with YYYY-MM-DD format
- **Directory Management**: Configurable storage location with auto-creation
- **LLM Processing**: Integration with Ollama for enhanced markdown formatting
- **Cross-Platform**: Native support for Linux, macOS, and Windows
- **Keyboard Navigation**: Full keyboard control in TUI mode

### Technical Details
- Built with Rust 2021 edition
- Uses ratatui for terminal user interface
- Crossterm for cross-platform terminal handling
- Serde for configuration serialization
- TOML for human-readable configuration files
- Anyhow for error handling
- Time crate for date/time operations
- Colored output for enhanced CLI experience
- Dirs crate for platform-specific directory handling

### Documentation
- Comprehensive README with installation and usage instructions
- Contributing guidelines for developers
- MIT license for open source distribution
- Example configuration file with detailed comments
- GitHub Actions CI/CD pipeline for automated testing and releases
- Issue templates for bug reports and feature requests

### Known Limitations
- Requires Ollama installation for LLM features (optional)
- No search functionality yet
- No export options yet
- No encryption support yet
- Limited to local storage only

[Unreleased]: https://github.com/yourusername/rusty_diary/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/rusty_diary/releases/tag/v0.1.0