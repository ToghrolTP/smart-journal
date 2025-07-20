# Contributing to Rusty Diary

Thank you for your interest in contributing to Rusty Diary! We welcome contributions from everyone, whether you're fixing a bug, adding a feature, improving documentation, or suggesting new ideas.

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Git](https://git-scm.com/)
- [Ollama](https://ollama.ai/) (optional, for testing LLM features)

### Setting Up Development Environment

1. **Fork the repository** on GitHub
2. **Clone your fork locally:**
   ```bash
   git clone https://github.com/yourusername/rusty_diary.git
   cd rusty_diary
   ```

3. **Add the upstream repository:**
   ```bash
   git remote add upstream https://github.com/originalusername/rusty_diary.git
   ```

4. **Install dependencies and build:**
   ```bash
   cargo build
   ```

5. **Run the application:**
   ```bash
   cargo run
   ```

6. **Run tests:**
   ```bash
   cargo test
   ```

## 🛠️ Development Guidelines

### Code Style

- Follow [Rust's official style guidelines](https://doc.rust-lang.org/nightly/style-guide/)
- Use `cargo fmt` to format your code before committing
- Run `cargo clippy` to catch common mistakes and improve code quality
- Ensure `cargo test` passes before submitting

### Formatting and Linting

Before submitting your changes, please run:

```bash
# Format code
cargo fmt

# Check for common mistakes
cargo clippy -- -D warnings

# Run tests
cargo test

# Check for unused dependencies
cargo machete  # if you have cargo-machete installed
```

### Project Structure

```
src/
├── app.rs          # Main application state and logic
├── commands/       # Command handlers (CLI and TUI)
├── config/         # Configuration management
├── ui/             # TUI components and screens
├── utils/          # Utility functions
└── main.rs         # Entry point
```

## 📝 Types of Contributions

### 🐛 Bug Reports

When reporting bugs, please include:

- **Description:** Clear description of the bug
- **Steps to reproduce:** Detailed steps to reproduce the issue
- **Expected behavior:** What you expected to happen
- **Actual behavior:** What actually happened
- **Environment:** OS, Rust version, terminal emulator
- **Screenshots:** If applicable

Use the bug report template when creating issues.

### 🌟 Feature Requests

When suggesting features:

- **Use case:** Explain why this feature would be useful
- **Description:** Detailed description of the proposed feature
- **Implementation ideas:** If you have thoughts on implementation
- **Alternatives:** Alternative solutions you've considered

### 🔧 Code Contributions

1. **Check existing issues** to see if someone is already working on it
2. **Create an issue** for discussion before starting major features
3. **Follow the development setup** outlined above
4. **Create a feature branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```
5. **Make your changes** following the coding guidelines
6. **Add tests** for new functionality
7. **Update documentation** if needed
8. **Commit your changes** with descriptive commit messages
9. **Push to your fork** and create a Pull Request

### 📚 Documentation

Help improve documentation by:

- Fixing typos or unclear explanations
- Adding examples and use cases
- Improving code comments
- Translating documentation
- Creating tutorials or guides

## 💬 Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
type(scope): description

[optional body]

[optional footer]
```

### Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples:
```
feat(ui): add settings screen for configuration
fix(config): handle missing config file gracefully
docs(readme): update installation instructions
test(journal): add tests for file format handling
```

## 🔄 Pull Request Process

1. **Update your fork:**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Ensure your code:**
   - Follows the style guidelines
   - Includes appropriate tests
   - Passes all existing tests
   - Has clear commit messages

3. **Create a Pull Request:**
   - Use a descriptive title
   - Reference related issues
   - Include a detailed description of changes
   - Add screenshots for UI changes

4. **Respond to feedback:**
   - Address reviewer comments promptly
   - Update your branch as needed
   - Be open to suggestions

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Writing Tests

- Add unit tests for new functions
- Add integration tests for major features
- Test both success and error cases
- Use descriptive test names

Example test structure:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loading() {
        // Test implementation
    }
}
```

## 🎯 Areas We Need Help With

- **Testing:** More comprehensive test coverage
- **Documentation:** User guides and examples
- **Features:** See our [roadmap](README.md#-roadmap)
- **Performance:** Optimization and profiling
- **Accessibility:** Making the TUI more accessible
- **Internationalization:** Multi-language support

## ❓ Questions and Support

- **GitHub Issues:** For bugs and feature requests
- **GitHub Discussions:** For general questions and ideas
- **Discord/Chat:** [Link if available]

## 📋 Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By participating, you agree to uphold this code.

## 🏆 Recognition

Contributors will be:

- Listed in the project's contributors section
- Mentioned in release notes for significant contributions
- Eligible for contributor badges (if implemented)

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Rusty Diary! Your efforts help make this project better for everyone. 🦀❤️