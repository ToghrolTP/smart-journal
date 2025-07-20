mod screens;

use crate::app::{App, AppScreen};
use ratatui::Frame;

pub fn render_ui(f: &mut Frame, app: &App) {
    match app.current_screen {
        AppScreen::Main => screens::render_main_screen(f, app),
        AppScreen::AddJournal => screens::render_add_journal_screen(f, app),
        AppScreen::ViewJournals => screens::render_view_journals_screen(f, app),
        AppScreen::Settings => screens::render_settings_screen(f, app),
        AppScreen::Help => screens::render_help_screen(f, app),
        AppScreen::Quit => {}
    }
}
