use colored::Colorize;

pub fn exe_help() {
    println!("{}", "Usage: rusty_diary <command> [options]".green());
    println!("{}", "Commands:".blue());
    println!("  -h,  --help           Show this help message");
    println!("  -aj, --add-journal    Show this help message");
}
