use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub struct StatusBar;

impl StatusBar {
    pub fn render(title: &str, message: &str, style: Style) -> Paragraph {
        Paragraph::new(message)
            .style(style)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .title_style(style)
            )
    }
}

pub struct TitleBar;

impl TitleBar {
    pub fn render(title: &str, subtitle: Option<&str>, color: Color) -> Paragraph {
        let mut lines = vec![
            Line::from(vec![
                Span::styled(title, Style::default().fg(color).add_modifier(Modifier::BOLD))
            ])
        ];
        
        if let Some(sub) = subtitle {
            lines.push(Line::from(vec![
                Span::styled(sub, Style::default().fg(Color::Gray))
            ]));
        }

        Paragraph::new(lines)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL))
    }
}

pub struct InputField;

impl InputField {
    pub fn render(content: &str, is_focused: bool, title: &str) -> Paragraph {
        let style = if is_focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let block_title = if is_focused {
            format!("{} (Editing...)", title)
        } else {
            title.to_string()
        };

        Paragraph::new(content)
            .style(style)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(block_title)
                    .title_style(style)
            )
    }
}