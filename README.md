# 🦀 Rusty Diary

A beautiful, terminal-based journal application written in Rust with an intuitive TUI interface and powerful features.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/Terminal-%23054020?style=for-the-badge&logo=gnu-bash&logoColor=white)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

## ✨ Features

- 🎨 **Beautiful TUI Interface** - Navigate with an intuitive terminal user interface
- 📝 **Multiple File Formats** - Support for Markdown, Plain Text, and JSON
- 🤖 **LLM Integration** - Automatic formatting with Ollama for beautiful markdown entries
- ⚙️ **Configurable Settings** - Customize journal directory, file formats, and more
- 📅 **Date-based Organization** - Automatic file naming with date stamps
- 🔍 **Browse & Search** - View and navigate through your journal entries
- 💾 **Persistent Configuration** - Settings saved in `~/.config/rusty_diary/config.toml`
- 🖥️ **Cross-platform** - Works on Linux, macOS, and Windows
- ⌨️ **CLI Mode** - Quick journal entry creation from command line

## 🚀 Installation

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Ollama](https://ollama.ai/) (optional, for LLM-powered markdown formatting)

### From Source

```bash
git clone https://github.com/yourusername/rusty_diary.git
cd rusty_diary
cargo build --release
cargo install --path .
```

### Using Cargo

```bash
cargo install rusty_diary
```

## 📖 Usage

### TUI Mode (Default)

Launch the interactive terminal interface:

```bash
rusty_diary
# or explicitly
rusty_diary --tui
```

### Command Line Mode

Quickly add a journal entry:

```bash
rusty_diary -aj
# or
rusty_diary --add-journal
```

### Help

```bash
rusty_diary --help
```

## 🎮 TUI Navigation

| Key | Action |
|-----|--------|
| `↑↓` | Navigate menus |
| `Enter` | Select option |
| `e` | Edit journal entry |
| `s` | Save journal entry |
| `r` | Refresh journal list |
| `q` / `Esc` | Go back / Quit |

## ⚙️ Configuration

Rusty Diary creates a configuration file at `~/.config/rusty_diary/config.toml`:

```toml
journal_directory = "/home/user/Documents/RustyDiary"
file_format = "Markdown"
date_format = "%Y-%m-%d"
auto_backup = false
editor_command = "vim"  # optional
```

### Configuration Options

- **`journal_directory`** - Where your journal files are stored
- **`file_format`** - Choose from "Markdown", "PlainText", or "Json"
- **`date_format`** - Date format for file naming
- **`auto_backup`** - Enable automatic backups (future feature)
- **`editor_command`** - External editor for advanced editing (future feature)

You can also modify settings through the **Settings** screen in the TUI.

## 📁 File Formats

### Markdown (.md)
- Processed with LLM for beautiful formatting
- Perfect for rich text journal entries
- Default format

### Plain Text (.txt)
- Simple, unprocessed text
- Fast and lightweight
- No LLM processing required

### JSON (.json)
- Structured data with metadata
- Includes timestamps and date information
- Great for programmatic access

## 🤖 LLM Integration

Rusty Diary integrates with [Ollama](https://ollama.ai/) to automatically format your journal entries into beautiful markdown. Make sure you have Ollama installed and the `llama3.1:8b` model available:

```bash
# Install Ollama (see ollama.ai for platform-specific instructions)
ollama pull llama3.1:8b
```

## 📂 Directory Structure

```
~/Documents/RustyDiary/           # Default journal directory
├── 2024-01-15.md               # Journal entries
├── 2024-01-16.md
└── 2024-01-17.md

~/.config/rusty_diary/           # Configuration
└── config.toml                 # Settings file
```

## 🛠️ Development

### Building

```bash
git clone https://github.com/yourusername/rusty_diary.git
cd rusty_diary
cargo build
```

### Running Tests

```bash
cargo test
```

### Development Dependencies

- `ratatui` - Terminal user interface
- `crossterm` - Cross-platform terminal manipulation
- `serde` - Serialization framework
- `toml` - Configuration file parsing
- `anyhow` - Error handling
- `time` - Date and time utilities
- `colored` - Terminal colors
- `dirs` - Platform-specific directories

## 🤝 Contributing

Contributions are welcome! Here are some ways you can help:

1. **Bug Reports** - Open an issue with details about the bug
2. **Feature Requests** - Suggest new features or improvements
3. **Code Contributions** - Submit pull requests with bug fixes or new features
4. **Documentation** - Help improve documentation and examples

### Development Setup

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests if applicable
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## 🗺️ Roadmap

- [ ] **Encryption** - Encrypt journal entries for privacy
- [ ] **Search Functionality** - Full-text search through entries
- [ ] **Export Options** - Export to PDF, HTML, etc.
- [ ] **Themes** - Customizable color themes
- [ ] **Plugins** - Plugin system for extensibility
- [ ] **Cloud Sync** - Optional cloud synchronization
- [ ] **Statistics** - Writing statistics and insights
- [ ] **Templates** - Journal entry templates

## ❓ FAQ

**Q: Do I need Ollama for the application to work?**
A: No, Ollama is optional. If not available, the app will save entries as plain text or you can choose Plain Text format.

**Q: Can I change where my journals are stored?**
A: Yes, either through the Settings screen in the TUI or by editing the config file directly.

**Q: Is my data safe?**
A: Your journals are stored locally on your machine. We recommend regular backups of your journal directory.

**Q: Can I use this on Windows?**
A: Yes! Rusty Diary works on Windows, macOS, and Linux.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [ratatui](https://github.com/ratatui-org/ratatui) - Amazing TUI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal library
- [Ollama](https://ollama.ai/) - Local LLM integration
- The Rust community for excellent crates and documentation

---

<div align="center">

**Made with ❤️ and 🦀 by the Rust community**

[Report Bug](https://github.com/yourusername/rusty_diary/issues) · [Request Feature](https://github.com/yourusername/rusty_diary/issues) · [Documentation](https://github.com/yourusername/rusty_diary/wiki)

</div>