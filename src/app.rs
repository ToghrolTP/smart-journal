use crate::ui::render_ui;
use crate::{commands, config::Config};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{backend::Backend, Terminal};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
pub enum AppScreen {
    Main,
    AddJournal,
    ViewJournals,
    Settings,
    Help,
    Quit,
}

#[derive(Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub current_screen: AppScreen,
    pub input_mode: InputMode,
    pub current_input: String,
    pub cursor_position: usize,
    pub selected_menu_item: usize,
    pub message: Option<String>,
    pub message_type: MessageType,
    pub processing: bool,
    pub journal_entries: Vec<JournalEntry>,
    pub selected_journal: usize,
    pub config: Config,
    pub settings_selection: usize,
    pub settings_input: String,
    pub settings_editing: bool,
}

#[derive(Clone, PartialEq)]
pub enum MessageType {
    Success,
    Error,
    Info,
    Warning,
}

#[derive(Clone)]
pub struct JournalEntry {
    pub date: String,
    pub content: String,
    pub filename: String,
}

impl App {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;

        Ok(Self {
            current_screen: AppScreen::Main,
            input_mode: InputMode::Normal,
            current_input: String::new(),
            cursor_position: 0,
            selected_menu_item: 0,
            message: None,
            message_type: MessageType::Info,
            processing: false,
            journal_entries: Vec::new(),
            selected_journal: 0,
            config,
            settings_selection: 0,
            settings_input: String::new(),
            settings_editing: false,
        })
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|f| render_ui(f, self))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key_event(key)?;
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }

            if self.current_screen == AppScreen::Quit {
                break;
            }
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        match self.current_screen {
            AppScreen::Main => self.handle_main_input(key),
            AppScreen::AddJournal => self.handle_add_journal_input(key)?,
            AppScreen::ViewJournals => self.handle_view_journals_input(key),
            AppScreen::Settings => self.handle_settings_input(key)?,
            AppScreen::Help => self.handle_help_input(key),
            AppScreen::Quit => {}
        }
        Ok(())
    }

    fn handle_main_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.current_screen = AppScreen::Quit,
            KeyCode::Up => {
                if self.selected_menu_item > 0 {
                    self.selected_menu_item -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_menu_item < 4 {
                    self.selected_menu_item += 1;
                }
            }
            KeyCode::Enter => match self.selected_menu_item {
                0 => self.current_screen = AppScreen::AddJournal,
                1 => {
                    self.load_journal_entries();
                    self.current_screen = AppScreen::ViewJournals;
                }
                2 => self.current_screen = AppScreen::Settings,
                3 => self.current_screen = AppScreen::Help,
                4 => self.current_screen = AppScreen::Quit,
                _ => {}
            },
            _ => {}
        }
    }

    fn handle_add_journal_input(&mut self, key: KeyEvent) -> Result<()> {
        match self.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('e') => {
                    self.input_mode = InputMode::Editing;
                    self.message = Some("Start typing your journal entry...".to_string());
                    self.message_type = MessageType::Info;
                }
                KeyCode::Char('s') => {
                    if !self.current_input.is_empty() && !self.processing {
                        self.save_journal_entry()?;
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.reset_input_state();
                    self.current_screen = AppScreen::Main;
                }
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    self.message = Some("Press 's' to save or 'q' to quit".to_string());
                    self.message_type = MessageType::Info;
                }
                KeyCode::Char(c) => {
                    self.current_input.insert(self.cursor_position, c);
                    self.cursor_position += 1;
                }
                KeyCode::Backspace => {
                    if self.cursor_position > 0 {
                        self.cursor_position -= 1;
                        self.current_input.remove(self.cursor_position);
                    }
                }
                KeyCode::Left => {
                    if self.cursor_position > 0 {
                        self.cursor_position -= 1;
                    }
                }
                KeyCode::Right => {
                    if self.cursor_position < self.current_input.len() {
                        self.cursor_position += 1;
                    }
                }
                KeyCode::Home => self.cursor_position = 0,
                KeyCode::End => self.cursor_position = self.current_input.len(),
                _ => {}
            },
        }
        Ok(())
    }

    fn handle_view_journals_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.current_screen = AppScreen::Main,
            KeyCode::Up => {
                if self.selected_journal > 0 {
                    self.selected_journal -= 1;
                }
            }
            KeyCode::Down => {
                if !self.journal_entries.is_empty()
                    && self.selected_journal < self.journal_entries.len() - 1
                {
                    self.selected_journal += 1;
                }
            }
            KeyCode::Char('r') => self.load_journal_entries(),
            _ => {}
        }
    }

    fn handle_settings_input(&mut self, key: KeyEvent) -> Result<()> {
        if self.settings_editing {
            match key.code {
                KeyCode::Esc => {
                    self.settings_editing = false;
                    self.settings_input.clear();
                }
                KeyCode::Enter => {
                    self.apply_settings_change()?;
                    self.settings_editing = false;
                    self.settings_input.clear();
                }
                KeyCode::Char(c) => {
                    self.settings_input.push(c);
                }
                KeyCode::Backspace => {
                    self.settings_input.pop();
                }
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => self.current_screen = AppScreen::Main,
                KeyCode::Up => {
                    if self.settings_selection > 0 {
                        self.settings_selection -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.settings_selection < 2 {
                        self.settings_selection += 1;
                    }
                }
                KeyCode::Enter => {
                    self.start_editing_setting();
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_help_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.current_screen = AppScreen::Main,
            _ => {}
        }
    }

    fn save_journal_entry(&mut self) -> Result<()> {
        self.processing = true;
        self.message = Some("Processing with LLM...".to_string());
        self.message_type = MessageType::Info;

        match commands::save_journal_entry_with_config(&self.current_input, &self.config) {
            Ok(_) => {
                self.message = Some("Journal entry saved successfully!".to_string());
                self.message_type = MessageType::Success;
                self.reset_input_state();
            }
            Err(e) => {
                self.message = Some(format!("Failed to save journal: {}", e));
                self.message_type = MessageType::Error;
            }
        }

        self.processing = false;
        Ok(())
    }

    fn load_journal_entries(&mut self) {
        match commands::load_journal_entries_with_config(&self.config) {
            Ok(entries) => {
                self.journal_entries = entries;
                self.selected_journal = 0;
                self.message = Some(format!(
                    "Loaded {} journal entries",
                    self.journal_entries.len()
                ));
                self.message_type = MessageType::Success;
            }
            Err(e) => {
                self.message = Some(format!("Failed to load journals: {}", e));
                self.message_type = MessageType::Error;
            }
        }
    }

    fn start_editing_setting(&mut self) {
        self.settings_editing = true;
        self.settings_input = match self.settings_selection {
            0 => self.config.get_journal_directory_display(),
            1 => match self.config.file_format {
                crate::config::FileFormat::Markdown => "md".to_string(),
                crate::config::FileFormat::PlainText => "txt".to_string(),
                crate::config::FileFormat::Json => "json".to_string(),
            },
            _ => String::new(),
        };
    }

    fn apply_settings_change(&mut self) -> Result<()> {
        match self.settings_selection {
            0 => {
                // Change journal directory
                self.config.journal_directory = std::path::PathBuf::from(&self.settings_input);
            }
            1 => {
                // Change file format
                self.config.file_format = match self.settings_input.as_str() {
                    "txt" => crate::config::FileFormat::PlainText,
                    "json" => crate::config::FileFormat::Json,
                    _ => crate::config::FileFormat::Markdown,
                };
            }
            _ => {}
        }

        self.config.save()?;
        self.message = Some("Settings saved successfully!".to_string());
        self.message_type = MessageType::Success;
        Ok(())
    }

    fn reset_input_state(&mut self) {
        self.current_input.clear();
        self.cursor_position = 0;
        self.input_mode = InputMode::Normal;
        self.processing = false;
    }

    fn on_tick(&mut self) {
        // Handle any periodic updates here
    }
}
