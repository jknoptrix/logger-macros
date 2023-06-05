use crate::config::{LogVariables, LogVariablesImpl, LogLevel};
use crate::config::LOG_PATH;
use chrono::Local;

use std::{
    path::Path,
    fs::OpenOptions,
    io::{self, Write},

};

pub fn log_to_file(now: &str, message: &str) -> io::Result<()> {
    let log_path = LOG_PATH.lock().unwrap();
    let filename = if log_path.as_os_str().is_empty() {
        format!("{}.log", Local::now().format("%Y-%m-%d"))
    } else {
        log_path.join(format!("{}.log", Local::now().format("%Y-%m-%d"))).to_string_lossy().into_owned()
    };
    let path = Path::new(&filename);
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{} {}", now, message)?;
    Ok(())
}

pub fn log_message(level: &str, now: &str, message: &str) {
    let color_code = match level {
        "ERROR" => "\x1b[31m", // red
        "WARN" => "\x1b[33m",  // yellow
        "INFO" => "\x1b[36m",  // cyan
        "DEBUG" => "\x1b[34m", // blue
        _ => "\x1b[0m",        // reset
    };
    let message = format!(
        "\x1b[1m\x1b[37m{}\x1b[0m {}[{}]\x1b[0m {}{}\x1b[0m",
        now, color_code, level, color_code, message
    );
    let log_variables = LogVariablesImpl;
    let log_level = log_variables.log_level().lock().unwrap();
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
