use crate::app::{App, InputMode, MessageType};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn render_main_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("🦀 Rusty Diary")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Menu
    let menu_items = vec![
        "📝 Add Journal Entry",
        "📖 View Journal Entries",
        "⚙️ Settings",
        "❓ Help",
        "🚪 Quit",
    ];

    let items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.selected_menu_item {
                Style::default()
                    .bg(Color::Yellow)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(*item).style(style)
        })
        .collect();

    let menu = List::new(items)
        .block(Block::default().title("Main Menu").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC));

    f.render_widget(menu, chunks[1]);

    // Instructions
    let instructions = Paragraph::new("Use ↑↓ to navigate, Enter to select, q to quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(instructions, chunks[2]);
}

pub fn render_add_journal_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("📝 Add Journal Entry")
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Input area
    let input_style = match app.input_mode {
        InputMode::Normal => Style::default(),
        InputMode::Editing => Style::default().fg(Color::Yellow),
    };

    let input_block = Block::default()
        .borders(Borders::ALL)
        .title(match app.input_mode {
            InputMode::Normal => "Journal Entry (Press 'e' to edit)",
            InputMode::Editing => "Journal Entry (Press Esc when done)",
        })
        .title_style(input_style);

    let input = Paragraph::new(app.current_input.as_str())
        .style(input_style)
        .block(input_block)
        .wrap(Wrap { trim: true });

    f.render_widget(input, chunks[1]);

    // Set cursor position when editing
    if app.input_mode == InputMode::Editing {
        f.set_cursor_position((
            chunks[1].x + (app.cursor_position % (chunks[1].width as usize - 2)) as u16 + 1,
            chunks[1].y + (app.cursor_position / (chunks[1].width as usize - 2)) as u16 + 1,
        ));
    }

    // Status/Message area
    if let Some(ref message) = app.message {
        let message_style = match app.message_type {
            MessageType::Success => Style::default().fg(Color::Green),
            MessageType::Error => Style::default().fg(Color::Red),
            MessageType::Warning => Style::default().fg(Color::Yellow),
            MessageType::Info => Style::default().fg(Color::Blue),
        };

        let status = Paragraph::new(message.as_str())
            .style(message_style)
            .block(Block::default().borders(Borders::ALL).title("Status"))
            .wrap(Wrap { trim: true });
        f.render_widget(status, chunks[2]);
    }

    // Instructions
    let instructions = match app.input_mode {
        InputMode::Normal => {
            if app.processing {
                "Processing..."
            } else if app.current_input.is_empty() {
                "Press 'e' to start editing, 'q' to quit"
            } else {
                "Press 's' to save, 'e' to edit, 'q' to quit"
            }
        }
        InputMode::Editing => "Type your journal entry, Press Esc when done",
    };

    let instruction_widget = Paragraph::new(instructions)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(instruction_widget, chunks[3]);

    // Show processing overlay if processing
    if app.processing {
        let area = centered_rect(50, 20, f.area());
        f.render_widget(Clear, area);
        let processing = Paragraph::new("🔄 Processing with LLM...")
            .style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Please Wait"));
        f.render_widget(processing, area);
    }
}

pub fn render_view_journals_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("📖 Journal Entries")
        .style(
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Journal list
    if app.journal_entries.is_empty() {
        let empty_message =
            Paragraph::new("No journal entries found.\n\nPress 'r' to refresh or 'q' to go back.")
                .style(Style::default().fg(Color::Gray))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("No Entries"));
        f.render_widget(empty_message, chunks[1]);
    } else {
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(chunks[1]);

        // Journal list
        let items: Vec<ListItem> = app
            .journal_entries
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let style = if i == app.selected_journal {
                    Style::default()
                        .bg(Color::Blue)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(format!("📅 {}", entry.date)).style(style)
            })
            .collect();

        let journal_list =
            List::new(items).block(Block::default().borders(Borders::ALL).title("Entries"));
        f.render_widget(journal_list, horizontal_chunks[0]);

        // Journal content preview
        if let Some(selected_entry) = app.journal_entries.get(app.selected_journal) {
            let content = Paragraph::new(selected_entry.content.as_str())
                .style(Style::default())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("Content - {}", selected_entry.date)),
                )
                .wrap(Wrap { trim: true });
            f.render_widget(content, horizontal_chunks[1]);
        }
    }

    // Instructions
    let instructions = if app.message.is_some() {
        app.message.as_ref().unwrap()
    } else {
        "Use ↑↓ to navigate, 'r' to refresh, 'q' to go back"
    };

    let instruction_style = match app.message_type {
        MessageType::Success => Style::default().fg(Color::Green),
        MessageType::Error => Style::default().fg(Color::Red),
        _ => Style::default().fg(Color::Gray),
    };

    let instruction_widget = Paragraph::new(instructions)
        .style(instruction_style)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(instruction_widget, chunks[2]);
}

pub fn render_help_screen(f: &mut Frame, _app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("❓ Help")
        .style(
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Help content
    let help_text = vec![
        Line::from(vec![
            Span::styled(
                "🦀 Rusty Diary",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" - A beautiful TUI journal application"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Main Menu:",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from("  • Use ↑↓ arrow keys to navigate"),
        Line::from("  • Press Enter to select an option"),
        Line::from("  • Press 'q' or Esc to quit"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Add Journal Entry:",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from("  • Press 'e' to start editing"),
        Line::from("  • Press Esc to stop editing"),
        Line::from("  • Press 's' to save your entry"),
        Line::from("  • Your entry will be processed with LLM for formatting"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "View Journal Entries:",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from("  • Use ↑↓ to browse entries"),
        Line::from("  • Press 'r' to refresh the list"),
        Line::from("  • View content preview on the right"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Features:",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from("  • Automatic date-based file naming"),
        Line::from("  • LLM processing with Ollama"),
        Line::from("  • Markdown formatting"),
        Line::from("  • Beautiful TUI interface"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Command Line:",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from("  • rusty_diary --tui    (Start TUI mode)"),
        Line::from("  • rusty_diary -aj      (Quick add journal)"),
        Line::from("  • rusty_diary -h       (Show help)"),
    ];

    let help_content = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(help_content, chunks[1]);

    // Instructions
    let instructions = Paragraph::new("Press 'q' or Esc to go back to main menu")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(instructions, chunks[2]);
}

// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn render_settings_screen(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("⚙️ Settings")
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Settings list
    let settings_items = vec![
        format!(
            "Journal Directory: {}",
            app.config.get_journal_directory_display()
        ),
        format!("File Format: {}", app.config.file_format.display_name()),
        format!("Date Format: {}", app.config.date_format),
    ];

    let items: Vec<ListItem> = settings_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.settings_selection {
                if app.settings_editing {
                    Style::default()
                        .bg(Color::Green)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                        .bg(Color::Yellow)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD)
                }
            } else {
                Style::default()
            };
            ListItem::new(item.as_str()).style(style)
        })
        .collect();

    let settings_list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Configuration"),
    );
    f.render_widget(settings_list, chunks[1]);

    // Show input box if editing
    if app.settings_editing {
        let area = centered_rect(80, 20, f.area());
        f.render_widget(Clear, area);

        let input_title = match app.settings_selection {
            0 => "Edit Journal Directory",
            1 => "Edit File Format (md/txt/json)",
            _ => "Edit Setting",
        };

        let input_widget = Paragraph::new(app.settings_input.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(input_title)
                    .title_style(Style::default().fg(Color::Yellow)),
            );
        f.render_widget(input_widget, area);
    }

    // Instructions
    let instructions = if app.settings_editing {
        "Enter new value, Press Enter to save, Esc to cancel"
    } else if let Some(ref message) = app.message {
        message.as_str()
    } else {
        "Use ↑↓ to navigate, Enter to edit, 'q' to go back"
    };

    let instruction_style = if app.message.is_some() {
        match app.message_type {
            MessageType::Success => Style::default().fg(Color::Green),
            MessageType::Error => Style::default().fg(Color::Red),
            _ => Style::default().fg(Color::Gray),
        }
    } else {
        Style::default().fg(Color::Gray)
    };

    let instruction_widget = Paragraph::new(instructions)
        .style(instruction_style)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(instruction_widget, chunks[2]);
}
