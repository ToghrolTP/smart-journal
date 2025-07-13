use crate::utils::{self, get_string};
use colored::Colorize;
use std::fs::OpenOptions;
use std::io::{Read, Write, pipe};
use std::process::{Command, Stdio};
use std::thread::spawn;
use time;

pub fn exe_add_journal() {
    let utc_date_time = time::UtcDateTime::now();
    let date = format!(
        "{}-{}-{}",
        utc_date_time.year(),
        utc_date_time.month(),
        utc_date_time.day()
    );
    let filename = format!("{date}.md");

    let mut file = match OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&filename)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{} {}", "Failed to open file:".red(), e);
            let choice = get_string("Do you want to append to an existing file? (y/n): ");
            match choice.as_str() {
                "y" | "Y" => append_to_existing_file(&filename.as_str()),
                _ => {
                    return;
                }
            }
            return;
        }
    };

    let user_entry = utils::get_string("Enter your journal entry: ");

    println!("{}", "Processing with the LLM...".yellow());
    match process_with_ollama(&user_entry) {
        Ok(processed_entry) => match write!(file, "{}", processed_entry) {
            Ok(_) => println!("{}", "Journal entry saved!".green()),
            Err(e) => eprintln!("{} {}", "Failed to write to file:".red(), e),
        },
        Err(e) => {
            eprintln!("{} {}", "Failed to process entry with Ollama:".red(), e);
        }
    }
}

fn append_to_existing_file(filename: &str) {
    let mut file = match OpenOptions::new().append(true).open(filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{} {}", "Failed to open file:".red(), e);
            get_string("Do you want to append to an existing file? (y/n): ");
            return;
        }
    };
    let user_entry = utils::get_string("Enter your journal entry: ");

    println!("{}", "Processing with the LLM...".yellow());
    match process_with_ollama(&user_entry) {
        Ok(processed_entry) => match write!(file, "{}", processed_entry) {
            Ok(_) => println!("{}", "Journal entry saved!".green()),
            Err(e) => eprintln!("{} {}", "Failed to write to file:".red(), e),
        },
        Err(e) => {
            eprintln!("{} {}", "Failed to process entry with Ollama:".red(), e);
        }
    }
}

fn process_with_ollama(entry: &str) -> Result<String, std::io::Error> {
    let model_prompt = "Please just structure and organize the following journal entry into a beautiful and organized markdown format. You should only respond with the markdown result, do not add any additional details, you have to deal with the entry.";
    let mut child = Command::new("ollama")
        .args(&["run", "llama3.1:8b", model_prompt])
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
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Ollama execution failed",
        ))
    }
}
