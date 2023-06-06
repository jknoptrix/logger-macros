pub use crate::config::{LogVariables, LogVariablesImpl, LogLevel, LOG_ROTATOR_CONFIG, LOG_MUTEX};
pub use crate::{log_error, LOG_PATH};
pub use crate::log_rotator::{
    LogPath,
    LogConfig,
    LogRotatorConfig,

};
use std::{
    path::PathBuf,
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
//! use logger_rust::*;
//! 
//! fn main() {
//! // Set the log level to File
//!   set_log_level(LogLevel::File);
//! }
//! ```
//! 
    let log_variables = LogVariablesImpl;
    let mut log_level = log_variables.log_level().lock().unwrap();
    *log_level = level;
}

pub fn set_log_path(config: LogConfig) {
    //! Sets the log path and, optionally, the log rotator configuration.
    //!
    //! This function takes a `LogConfig` argument that specifies the log path and, optionally,
    //! the log rotator configuration. If a log rotator configuration is provided, the log rotator
    //! will automatically rotate the logs when necessary based on the provided configuration.
    //!
    //! # Arguments
    //!
    //! * `config` - The log configuration. Can be either a `LogConfig::Path` variant that specifies
    //!   the log path or a `LogConfig::Rotator` variant that specifies the log path and the log
    //!   rotator configuration.
    //!
    //! # Examples
    //! ### Set the log path without a log rotator:
    //! ```
    //! use logger_rust::*;
    //! use std::time::Duration;
    //! use std::path::PathBuf;
    //!
    //! 
    //! set_log_path(LogConfig::Path(LogPath::from("C:/Users/qruie/Documents")));
    //! ```
    //! 
    //! ### Set the log path with a log rotator
    //! ```
    //! use logger_rust::*;
    //! use std::time::Duration;
    //! use std::path::PathBuf;
    //!
    //! set_log_path(LogConfig::Rotator(LogRotatorConfig::new(
    //!     PathBuf::from("C:/Users/qruie/Documents"),
    //!     5 * 1024 * 1024,
    //!     Duration::from_secs(2),
    //! )));
    //! ```
    let log_variables = LogVariablesImpl;
    let log_level = log_variables.log_level().lock().unwrap();
    if *log_level != LogLevel::File && *log_level != LogLevel::Both {
        panic!("
        Cannot call set_log_path when log level is no set to `LogLevel::File` or `LogLevel::Both`
        Please, specify the `LogLevel::Console` or `LogLevel::Path` if you want to use `set_log_path`
        `set_log_level(LogLevel::File);` // OR `LogLevel::Both` <──────────────────────────────┘
        ---------------------------------------\n");
    }
    let handle = thread::spawn(move || {
        let path = match config {
            LogConfig::Path(LogPath::Path(path)) => path,
            LogConfig::Rotator(rotator_config) => {
                let log_path = rotator_config.log_path.clone();
                let mut log_rotator_config = LOG_ROTATOR_CONFIG.lock().unwrap();
                *log_rotator_config = Some(rotator_config);
                log_path
            }
        };
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
    });
    handle.join().unwrap();
}