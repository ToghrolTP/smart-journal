use crate::utils::{self, get_string};
use colored::Colorize;
use std::fs::OpenOptions;
use std::io::Write;
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

    match write!(file, "{}", utils::get_string("Enter your journal entry: ")) {
        Ok(_) => println!("{}", "Journal entry saved!".green()),
        Err(e) => eprintln!("{} {}", "Failed to write to file:".red(), e),
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

    match writeln!(file, "{}", utils::get_string("Enter your journal entry: ")) {
        Ok(_) => println!("{}", "Journal entry saved!".green()),
        Err(e) => eprintln!("{} {}", "Failed to write to file:".red(), e),
    }
}
