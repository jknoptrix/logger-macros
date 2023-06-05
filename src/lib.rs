use colored::Colorize;
use chrono::Local;

pub fn error(now: &str, message: &str) {
    // define error function for printing error messages
    eprintln!("{} {} {}", now.bold().dimmed(), "[ERROR]".bold().red(), message.red()); // print formatted error message to stderr
}

pub fn warn(now: &str, message: &str) {
    // define warn function for printing warning messages
    eprintln!("{} {} {}", now.bold().dimmed(), "[WARN]".bold().yellow(), message.yellow()); // print formatted warning message to stderr
}

pub fn info(now: &str, message: &str) {
    // define info function for printing info messages
    println!("{} {} {}", now.bold().dimmed(), "[INFO]".bold().cyan(), message.cyan()); // print formatted info message to stdout
}

pub fn current_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[macro_export]
macro_rules! log_error {
    // define log_error macro for logging error messages
    ($($arg:tt)*) => {{
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        error(&now, &format!($($arg)*)); // call error function with formatted arguments
    }}
}

#[macro_export]
macro_rules! log_warn {
    // define log_warn macro for logging warning messages
    ($($arg:tt)*) => {{
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        warn(&now, &format!($($arg)*)); // call warn function with formatted arguments
    }}
}

#[macro_export]
macro_rules! log_info {
    // define log_info macro for logging info messages
    ($($arg:tt)*) => {{
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        info(&now, &format!($($arg)*)); // call info function with formatted arguments
    }}
}

