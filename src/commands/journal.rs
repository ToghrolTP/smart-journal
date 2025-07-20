use crate::config::{Config, FileFormat};
use crate::utils::{self, get_string};
use colored::Colorize;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn exe_add_journal() {
    let config = match Config::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{} {}", "Failed to load config:".red(), e);
            return;
        }
    };

    let utc_date_time = time::UtcDateTime::now();
    let date = format!(
        "{}-{:02}-{:02}",
        utc_date_time.year(),
        utc_date_time.month() as u8,
        utc_date_time.day()
    );

    let file_path = config.get_journal_file_path(&date);
    let filename = file_path.file_name().unwrap().to_string_lossy();

    // Ensure directory exists
    if let Some(parent) = file_path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("{} {}", "Failed to create directory:".red(), e);
            return;
        }
    }

    let mut file = match OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&file_path)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{} {}", "Failed to open file:".red(), e);
            let choice = get_string("Do you want to append to an existing file? (y/n): ");
            match choice.as_str() {
                "y" | "Y" => append_to_existing_file(&file_path, &config),
                _ => write_to_existing_file(&file_path, &config),
            }
            return;
        }
    };

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let user_entry =
        utils::get_string(format!("Enter your journal entry for {}: ", filename).as_str());

    let processed_entry = match config.file_format {
        FileFormat::Markdown => {
            println!("{}", "Processing with the LLM...".yellow());
            match process_with_ollama(&user_entry) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("{} {}", "Failed to process entry with Ollama:".red(), e);
                    return;
                }
            }
        }
        FileFormat::PlainText => user_entry,
        FileFormat::Json => {
            match serde_json::to_string_pretty(&serde_json::json!({
                "date": date,
                "content": user_entry,
                "created_at": time::UtcDateTime::now().to_string()
            })) {
                Ok(json) => json,
                Err(e) => {
                    eprintln!("{} {}", "Failed to create JSON:".red(), e);
                    return;
                }
            }
        }
    };

    match write!(file, "{}", processed_entry) {
        Ok(_) => println!("{}", "Journal entry saved!".green()),
        Err(e) => eprintln!("{} {}", "Failed to write to file:".red(), e),
    }
}

fn append_to_existing_file(file_path: &std::path::Path, config: &Config) {
    let mut file = match OpenOptions::new().append(true).open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{} {}", "Failed to open file:".red(), e);
            return;
        }
    };

    let filename = file_path.file_name().unwrap().to_string_lossy();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let user_entry =
        utils::get_string(format!("Enter your journal entry for {}: ", filename).as_str());

    let processed_entry = match config.file_format {
        FileFormat::Markdown => {
            println!("{}", "Processing with the LLM...".yellow());
            match process_with_ollama(&user_entry) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("{} {}", "Failed to process entry with Ollama:".red(), e);
                    return;
                }
            }
        }
        FileFormat::PlainText => user_entry,
        FileFormat::Json => {
            let utc_date_time = time::UtcDateTime::now();
            let date = format!(
                "{}-{:02}-{:02}",
                utc_date_time.year(),
                utc_date_time.month() as u8,
                utc_date_time.day()
            );
            match serde_json::to_string_pretty(&serde_json::json!({
                "date": date,
                "content": user_entry,
                "created_at": time::UtcDateTime::now().to_string()
            })) {
                Ok(json) => json,
                Err(e) => {
                    eprintln!("{} {}", "Failed to create JSON:".red(), e);
                    return;
                }
            }
        }
    };

    match write!(file, "\n{}", processed_entry) {
        Ok(_) => println!("{}", "Journal entry appended!".green()),
        Err(e) => eprintln!("{} {}", "Failed to write to file:".red(), e),
    }
}

fn write_to_existing_file(file_path: &std::path::Path, config: &Config) {
    let mut file = match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)
    {
        Ok(file) => file,
        Err(e) => {
            println!("{} {}", "Error:".red(), e);
            return;
        }
    };

    let filename = file_path.file_name().unwrap().to_string_lossy();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let user_entry = utils::get_string(
        format!(
            "{} You are overwriting the existing file!\nEnter your journal entry for {}: ",
            "WARNING:".yellow(),
            filename
        )
        .as_str(),
    );

    let processed_entry = match config.file_format {
        FileFormat::Markdown => {
            println!("{}", "Processing with the LLM...".yellow());
            match process_with_ollama(&user_entry) {
                Ok(content) => content,
                Err(e) => {
                    println!("{} {}", "Error processing with LLM:".red(), e);
                    return;
                }
            }
        }
        FileFormat::PlainText => user_entry,
        FileFormat::Json => {
            let utc_date_time = time::UtcDateTime::now();
            let date = format!(
                "{}-{:02}-{:02}",
                utc_date_time.year(),
                utc_date_time.month() as u8,
                utc_date_time.day()
            );
            match serde_json::to_string_pretty(&serde_json::json!({
                "date": date,
                "content": user_entry,
                "created_at": time::UtcDateTime::now().to_string()
            })) {
                Ok(json) => json,
                Err(e) => {
                    eprintln!("{} {}", "Failed to create JSON:".red(), e);
                    return;
                }
            }
        }
    };

    match write!(file, "{}", processed_entry) {
        Ok(_) => println!("{}", "Journal entry saved!".green()),
        Err(e) => eprintln!("{} {}", "Failed to write to file:".red(), e),
    }
}

fn process_with_ollama(entry: &str) -> Result<String, std::io::Error> {
    let model_prompt = "Please just structure and organize the following journal entry into a beautiful and organized markdown format. You should only respond with the markdown result, do not add any additional details, you have to deal with the entry.";
    let mut child = Command::new("ollama")
        .args(["run", "llama3.1:8b", model_prompt])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(entry.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        eprintln!("{} {}", "Ollama Error:".red(), err);
        Err(std::io::Error::other("Ollama execution failed"))
    }
}

pub fn save_journal_entry_with_config(
    entry: &str,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let utc_date_time = time::UtcDateTime::now();
    let date = format!(
        "{}-{:02}-{:02}",
        utc_date_time.year(),
        utc_date_time.month() as u8,
        utc_date_time.day()
    );

    let file_path = config.get_journal_file_path(&date);

    // Ensure the journal directory exists
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let processed_entry = match config.file_format {
        FileFormat::Markdown => process_with_ollama(entry)?,
        FileFormat::PlainText => entry.to_string(),
        FileFormat::Json => serde_json::to_string_pretty(&serde_json::json!({
            "date": date,
            "content": entry,
            "processed_at": time::UtcDateTime::now().to_string()
        }))?,
    };

    let mut file = match std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&file_path)
    {
        Ok(file) => file,
        Err(_) => {
            // If file exists, append to it with a separator
            let mut append_file = std::fs::OpenOptions::new().append(true).open(&file_path)?;
            writeln!(append_file, "\n---\n")?;
            append_file
        }
    };

    writeln!(file, "{}", processed_entry)?;
    Ok(())
}

pub fn load_journal_entries_with_config(
    config: &Config,
) -> Result<Vec<crate::app::JournalEntry>, Box<dyn std::error::Error>> {
    let mut entries = Vec::new();

    if !config.journal_directory.exists() {
        return Ok(entries);
    }

    let extension = config.file_format.extension();

    for entry in std::fs::read_dir(&config.journal_directory)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(file_extension) = path.extension() {
            if file_extension == extension {
                if let Some(filename_str) = path.file_name().and_then(|n| n.to_str()) {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        // Check if filename matches date format (YYYY-MM-DD)
                        if stem.len() >= 10
                            && stem.chars().nth(4) == Some('-')
                            && stem.chars().nth(7) == Some('-')
                        {
                            let content = std::fs::read_to_string(&path)?;

                            // Process content based on file format
                            let processed_content = match config.file_format {
                                FileFormat::Json => {
                                    // For JSON files, try to extract the content field
                                    if let Ok(json_value) =
                                        serde_json::from_str::<serde_json::Value>(&content)
                                    {
                                        if let Some(content_field) = json_value.get("content") {
                                            content_field.as_str().unwrap_or(&content).to_string()
                                        } else {
                                            content
                                        }
                                    } else {
                                        content
                                    }
                                }
                                _ => content,
                            };

                            entries.push(crate::app::JournalEntry {
                                date: stem.to_string(),
                                content: processed_content,
                                filename: filename_str.to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    // Sort entries by date (newest first)
    entries.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(entries)
}

// Keep the original functions for backward compatibility
pub fn save_journal_entry(entry: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    save_journal_entry_with_config(entry, &config)
}

pub fn load_journal_entries() -> Result<Vec<crate::app::JournalEntry>, Box<dyn std::error::Error>> {
    let config = Config::load()?;
    load_journal_entries_with_config(&config)
}
