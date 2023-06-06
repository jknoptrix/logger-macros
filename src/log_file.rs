pub use crate::config::{
    LogVariables, 
    LogVariablesImpl, 
    LogLevel,
    LOG_PATH, LOG_ROTATOR_CONFIG
};
use std::{
    fs,
    path::Path,
    fs::OpenOptions,
    io::{self, Write},

};
use chrono::Local;

pub fn log_to_file(now: &str, message: &str) -> io::Result<()> {
//! # log_to_file
//! The `log_to_file` function takes two arguments: `now` and `message`. 
//! - The `now` argument is a string representing the current time and the message argument is the message to be logged. 
//! The function checks if the log path is empty. If it is, it creates a new filename using the current date. If the log path is not empty, 
//! it joins the log path with the filename. 
//! The function then creates a new file at the specified path using the `OpenOptions` struct and writes the message to the file.
    let log_path = LOG_PATH.lock().unwrap();
    
    let filename = if log_path.as_os_str().is_empty() {
        format!("{}.log", Local::now().format("%Y-%m-%d"))
    } else {
        log_path.join(format!("{}.log", Local::now().format("%Y-%m-%d"))).to_string_lossy().into_owned()
    };
    
    let path = Path::new(&filename);
    
    // Check if we need to rotate the logs
    if let Some(log_rotator_config) = &*LOG_ROTATOR_CONFIG.lock().unwrap() {
        // Check if the current log file has exceeded the maximum size or lifetime
        if let Ok(metadata) = fs::metadata(&path) {
            if metadata.len() > log_rotator_config.max_size
                || metadata.modified()?.elapsed().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
                    > log_rotator_config.max_time
            {
                // Rotate the logs
                let mut i = 1;
                loop {
                    let rotated_filename = format!("{}_rot-{}.log", filename.trim_end_matches(".log"), i);
                    let rotated_path = Path::new(&rotated_filename);
                    if !rotated_path.exists() {
                        fs::rename(&path, rotated_path)?;
                        break;
                    }
                    i += 1;
                }
            }
        }
    }
    
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{} {}", now, message)?;
    Ok(())
}

pub fn log_message(level: &str, now: &str, message: &str) {
//! # log_message
//! The log_message function takes three arguments: level, now, and message. 
//! - The `level` argument is a string representing the log level *(e.g. “ERROR”, “WARN”, “INFO”, “DEBUG”)*. 
//! - The `now` argument is a string representing the current time and the message argument is the message to be logged. 
//! - The `function` matches the log level with a color code and formats the message with the color code and log level. 
//! It then checks the current log level and logs the message to either the console, a file, or both depending on the current log level.
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
