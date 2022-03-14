use colored::*;

pub fn error(message: &str, filename: &str, line: u32) {
    eprintln!("{} {} {}", format!("{}:{}:", filename, line).bold(), "error:".red().bold(), message);
}

pub fn error_no_line(message: &str, filename: &str) {
    eprintln!("{} {} {}", format!("{}:", filename).bold(), "error:".red().bold(), message);
}
