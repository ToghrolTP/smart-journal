mod app;
mod commands;
mod config;
mod ui;
mod utils;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{env, io};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // Check for command line arguments for backward compatibility
    if args.len() >= 2 {
        match args[1].as_str() {
            "-h" | "--help" => {
                commands::exe_help();
                return Ok(());
            }
            "-aj" | "--add-journal" => {
                commands::exe_add_journal();
                return Ok(());
            }
            "--tui" | "-t" => {
                // Explicit TUI mode
            }
            _ => {
                println!("Unknown command. Use --tui for TUI mode or -h for help.");
                return Ok(());
            }
        }
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run TUI
    let mut app = App::new()?;
    let result = app.run(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}
