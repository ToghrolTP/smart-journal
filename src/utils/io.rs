use std::io::{self, Write, stdout};

pub fn get_string(prompt: &str) -> String {
    print!("{prompt}");
    stdout().flush().expect("Failed to flush standard output");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}
