mod commands;
mod utils;

use std::env;
use colored::Colorize;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{}", "No argument provided.".red());
        println!("See {} for command arguments.", "-h".yellow());
        return;
    }

    match args[1].as_str() {
        "-h" | "--help" => {
            commands::exe_help();
        }
        "-aj" | "--add-journal" => {
            commands::exe_add_journal();
        }
        _ => {
            println!("{}", "Unknown command.".red());
            println!("See {} for command arguments.", "-h".yellow());
        }
    }
}
