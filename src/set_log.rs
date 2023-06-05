use crate::config::{LogVariables, LogVariablesImpl, LogLevel};
use crate::log_error;
use crate::LOG_PATH;
use std::{
    path::{Path, PathBuf},
    process,
    thread,
    time::Duration,
};
pub fn set_log_level(level: LogLevel) {
//! Sets the log level to the provided value.
//!
//! # Examples
//!
//! ```
//! use logger_rust::LogLevel;
//! // Set the log level to INFO
//! set_log_level(LogLevel::Info);
//! ```
    let log_variables = LogVariablesImpl;
    let mut log_level = log_variables.log_level().lock().unwrap();
    *log_level = level;
}

pub fn set_log_path<P: AsRef<Path>>(path: P) {
//! Sets the path to the log file.
//!
//! If the provided path is not valid, an error message will be logged and the
//! current thread will sleep for 10 seconds before exiting with an error code.
//!
//! # Examples
//!
//! Set the path to the log file
//! ```
//! use logger_rust::set_log_path;
//! set_log_path("C:/Users/qruie/Desktop");
//! ```
    let path = path.as_ref();
    match (
        path.exists(),
        path.is_dir(),
        path.metadata().map(|m| m.permissions().readonly()),
    ) {
        (false, _, _) => {
            log_error!("Path is not correct: {}", path.display());
            thread::sleep(Duration::from_secs(10));
            process::exit(1);
        }
        (_, false, _) => {
            log_error!("Path is not a directory: {}", path.display());
            thread::sleep(Duration::from_secs(10));
            process::exit(1);
        }
        (_, _, Err(e)) => {
            log_error!("Failed to get metadata for path {}: {}", path.display(), e);
            thread::sleep(Duration::from_secs(10));
            process::exit(1);
        }
        (_, _, Ok(false)) => {
            log_error!("Not enough permissions to access the path: '{}'", path.display());
            thread::sleep(Duration::from_secs(10));
            process::exit(1);
        }
        _ => {}
    }
    let mut log_path = LOG_PATH.lock().unwrap();
    *log_path = PathBuf::from(path);
}
