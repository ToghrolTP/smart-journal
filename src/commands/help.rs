use colored::Colorize;

pub fn exe_help() {
    println!(
        "{}",
        "🦀 Rusty Diary - A beautiful journal application"
            .green()
            .bold()
    );
    println!();
    println!("{}", "Usage: rusty_diary [command] [options]".green());
    println!();
    println!("{}", "Commands:".blue().bold());
    println!("  (no args)             Start interactive TUI mode (default)");
    println!("  -t,  --tui            Start interactive TUI mode explicitly");
    println!("  -aj, --add-journal    Add a journal entry (command line mode)");
    println!("  -h,  --help           Show this help message");
    println!();
    println!("{}", "Features:".blue().bold());
    println!("  • Beautiful terminal user interface (TUI)");
    println!("  • Multiple file formats: Markdown, Plain Text, JSON");
    println!("  • Configurable journal directory");
    println!("  • LLM processing with Ollama for markdown formatting");
    println!("  • Date-based file organization");
    println!("  • Browse and view existing journal entries");
    println!();
    println!("{}", "TUI Navigation:".blue().bold());
    println!("  • Use ↑↓ arrow keys to navigate menus");
    println!("  • Press Enter to select options");
    println!("  • Press 'q' or Esc to go back/quit");
    println!("  • Press 'e' to edit in Add Journal screen");
    println!("  • Press 's' to save journal entries");
    println!("  • Press 'r' to refresh journal list");
    println!();
    println!("{}", "Configuration:".blue().bold());
    println!("  Configuration file: ~/.config/rusty_diary/config.toml");
    println!("  Default journal directory: ~/Documents/RustyDiary/");
    println!("  Use the Settings screen in TUI to customize your setup");
    println!();
    println!("{}", "Examples:".blue().bold());
    println!("  rusty_diary                    # Start TUI mode");
    println!("  rusty_diary --tui              # Start TUI mode explicitly");
    println!("  rusty_diary -aj                # Quick add journal entry");
    println!("  rusty_diary --help             # Show this help");
}
