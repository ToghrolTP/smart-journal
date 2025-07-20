use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub journal_directory: PathBuf,
    pub file_format: FileFormat,
    pub date_format: String,
    pub auto_backup: bool,
    pub editor_command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileFormat {
    Markdown,
    PlainText,
    Json,
}

impl FileFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            FileFormat::Markdown => "md",
            FileFormat::PlainText => "txt",
            FileFormat::Json => "json",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            FileFormat::Markdown => "Markdown (.md)",
            FileFormat::PlainText => "Plain Text (.txt)",
            FileFormat::Json => "JSON (.json)",
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let journal_dir = home_dir.join("Documents").join("RustyDiary");

        Self {
            journal_directory: journal_dir,
            file_format: FileFormat::Markdown,
            date_format: "%Y-%m-%d".to_string(),
            auto_backup: false,
            editor_command: None,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_file_path()?;

        if config_path.exists() {
            let config_content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&config_content)?;
            Ok(config)
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_file_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let config_content = toml::to_string_pretty(self)?;
        fs::write(&config_path, config_content)?;

        // Also ensure journal directory exists
        fs::create_dir_all(&self.journal_directory)?;

        Ok(())
    }

    fn config_file_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
            .unwrap_or_else(|| PathBuf::from("."));

        Ok(config_dir.join("rusty_diary").join("config.toml"))
    }

    pub fn get_journal_file_path(&self, date: &str) -> PathBuf {
        let filename = format!("{}.{}", date, self.file_format.extension());
        self.journal_directory.join(filename)
    }

    pub fn get_journal_directory_display(&self) -> String {
        self.journal_directory.display().to_string()
    }
}
