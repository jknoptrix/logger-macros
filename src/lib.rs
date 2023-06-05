use chrono::Local;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref LOG_LEVEL: Mutex<LogLevel> = Mutex::new(LogLevel::Console);
}

#[derive(Copy, Clone)]
pub enum LogLevel {
    Console,
    File,
    Both,
}

pub fn set_log_level(level: LogLevel) {
    let mut log_level = LOG_LEVEL.lock().unwrap();
    *log_level = level;
}

fn log_to_file(now: &str, message: &str) -> io::Result<()> {
    let filename = format!("{}.log", Local::now().format("%Y-%m-%d"));
    let path = Path::new(&filename);
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{} {}", now, message)?;
    Ok(())
}

pub fn error(now: &str, message: &str) {
    let message = format!("\x1b[2m\x1b[1m{}\x1b[0m \x1b[31m\x1b[1m[ERROR]\x1b[0m \x1b[31m{}\x1b[0m", now, message);
    let log_level = LOG_LEVEL.lock().unwrap();
    match *log_level {
        LogLevel::Console => eprintln!("{}", message),
        LogLevel::File => {
            log_to_file(now, &message).unwrap_or_else(|e| eprintln!("Failed to write to log file: {}", e));
        }
        LogLevel::Both => {
            eprintln!("{}", message);
            log_to_file(now, &message).unwrap_or_else(|e| eprintln!("Failed to write to log file: {}", e));
        }
    }
}

pub fn warn(now: &str, message: &str) {
    // define warn function for printing warning messages
    eprintln!("\x1b[2m\x1b[1m{}\x1b[0m \x1b[33m\x1b[1m[WARN]\x1b[0m \x1b[33m{}\x1b[0m", now, message); // print formatted warning message to stderr
    let log_level = LOG_LEVEL.lock().unwrap();
    match *log_level {
        LogLevel::Console => eprintln!("{}", message),
        LogLevel::File => {
            log_to_file(now, &message).unwrap_or_else(|e| eprintln!("Failed to write to log file: {}", e));
        }
        LogLevel::Both => {
            eprintln!("{}", message);
            log_to_file(now, &message).unwrap_or_else(|e| eprintln!("Failed to write to log file: {}", e));
        }
    }
}

pub fn info(now: &str, message: &str) {
    // define info function for printing info messages
    println!("\x1b[2m\x1b[1m{}\x1b[0m \x1b[36m\x1b[1m[INFO]\x1b[0m \x1b[36m{}\x1b[0m", now, message); // print formatted info message to stdout
    let log_level = LOG_LEVEL.lock().unwrap();
    match *log_level {
        LogLevel::Console => eprintln!("{}", message),
        LogLevel::File => {
            log_to_file(now, &message).unwrap_or_else(|e| eprintln!("Failed to write to log file: {}", e));
        }
        LogLevel::Both => {
            eprintln!("{}", message);
            log_to_file(now, &message).unwrap_or_else(|e| eprintln!("Failed to write to log file: {}", e));
        }
    }
}

pub fn debug(now: &str, message: &str) {
    // define debug function for printing debug messages
    println!("\x1b[2m\x1b[1m{}\x1b[0m \x1b[34m\x1b[1m[DEBUG]\x1b[0m \x1b[34m{}\x1b[0m", now, message); // print formatted debug message to stdout
    let log_level = LOG_LEVEL.lock().unwrap();
    match *log_level {
        LogLevel::Console => eprintln!("{}", message),
        LogLevel::File => {
            log_to_file(now, &message).unwrap_or_else(|e| eprintln!("Failed to write to log file: {}", e));
        }
        LogLevel::Both => {
            eprintln!("{}", message);
            log_to_file(now, &message).unwrap_or_else(|e| eprintln!("Failed to write to log file: {}", e));
        }
    }
}

pub fn current_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[macro_export]
macro_rules! log_error {
    // define log_error macro for logging error messages
    ($($arg:tt)*) => {{
        let now = $crate::current_time();
        $crate::error(&now, &format!($($arg)*));
    }}
}

#[macro_export]
macro_rules! log_warn {
    // define log_warn macro for logging warning messages
    ($($arg:tt)*) => {{
        let now = $crate::current_time();
        $crate::warn(&now, &format!($($arg)*)); // call warn function with formatted arguments
    }}
}

#[macro_export]
macro_rules! log_info {
    // define log_info macro for logging info messages
    ($($arg:tt)*) => {{
        let now = $crate::current_time();
        $crate::info(&now, &format!($($arg)*)); // call info function with formatted arguments
    }}
}

#[macro_export]
macro_rules! log_debug {
    // define log_debug macro for logging info messages
    ($($arg:tt)*) => {{
        let now = $crate::current_time();
        $crate::debug(&now, &format!($($arg)*)); // call info function with formatted arguments
    }}
}