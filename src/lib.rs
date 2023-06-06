//! # Logger macro
//! This crate provides a simple and flexible logging system for Rust projects. It allows you to log messages of different types (error, warn, info, debug) using macros and choose whether to log messages to the console, a file, or both.
//!
//! # Features
//! - Log messages of different types (error, warn, info, debug) using macros
//! - Choose whether to log messages to the console, a file, or both
//! - Set the log level at runtime
//! - Automatically create and append to log files based on the current date
//! - Handle errors when writing to log files
//!
//! ## Usage
//! To use this crate in your project, add it as a dependency in your `Cargo.toml` file:
//!
//! ```env
//! [dependencies]
//! logger-rust = "0.1.44"
//! ```
//! Then, import the crate:
//! ```rust
//! use logger_rust::*;
//! ```
//! If you want to use them separately:
//! ```rust
//! // all possible crates
//! use logger_rust::log_debug;
//! use logger_rust::log_info;
//! use logger_rust::log_warn;
//! use logger_rust::log_error;
//! use logger_rust::set_log_path;
//! use logger_rust::set_log_level;
//! use logger_rust::LOG_PATH;
//! use logger_rust::LogLevel;
//! use logger_rust::current_time;
//! ```
//!
//! You can now use the `log_error!`, `log_warn!`, `log_info!`, and `log_debug!` macros to log messages of different types:
//!
//! ```rust
//! use logger_rust::*;
//! 
//! fn main() {
//!     log_error!("An error occurred: {}", "Something went wrong");
//!     log_warn!("A warning occurred: {}", "Something might be wrong");
//!     log_info!("An info message: {}", "Something happened");
//!     log_debug!("A debug message: {}", "Something happened in detail");
//! }
//! ```
//!
//!
//! Also you can use this:
//! ```rust
//! use logger_rust::*;
//! 
//!     error(&current_time(), "An error message");
//!     warn(&current_time(), "A warning message");
//!     info(&current_time(), "An info message");
//! ```
//! Output:
//! ```diff
//! - 2023-06-05 12:23:25 [ERROR] An error occurred: Something went wrong
//! - 2023-06-05 12:23:25 [WARN] A warning occurred: Something might be wrong
//! A warning occurred: Something might be wrong
//! + 2023-06-05 12:23:25 [INFO] An info message: Something happened
//! An info message: Something happened
//! + 2023-06-05 12:23:25 [DEBUG] A debug message: Something happened in detail
//! A debug message: Something happened in detail
//! ```
//!
//! # Custom logging
//! After 0.1.2 version, you can create a custom logging function with macros.
//!
//! 1. Create a `class` named as function that you needed (e.g. `trace`):
//!
//! ```rust
//! use logger_rust::*;
//! 
//! pub fn trace(now: &str, message: &str) {
//!     log_message("TRACE", now, message);
//! }
//! ```
//!
//! 
//!
pub mod time;
pub mod config;
pub mod set_log;
pub mod log_file;
pub mod log_rotator;
pub use crate::set_log::*;
pub use crate::log_rotator::*;
pub use crate::config::LOG_PATH;
pub use crate::time::current_time;
pub use crate::log_file::log_message;
pub use crate::set_log::{set_log_level, set_log_path};
pub use crate::config::{LogVariables, LogVariablesImpl, LogLevel};
pub fn error(now: &str, message: &str) {
//! # Error
//! Outputs an error message via `log_error!` macros.
//! You can also use just an `error` method instead, but it requires you to define any variable as `&str`
//! > E.g:
//! ```rust
//! use logger_rust::error;
//! use logger_rust::current_time;
//! error(&current_time(), "An error message");
//! ```
    log_message("ERROR", now, message);
}

pub fn warn(now: &str, message: &str) {
//! # Warn
//! Outputs an warn message via `log_warn!` macros.
//! You can also use just an `warn` method instead, but it requires you to define any variable as `&str`
//! > E.g:
//! ```rust
//! use logger_rust::warn;
//! use logger_rust::current_time;
//! warn(&current_time(), "A warn message");
//! ```
    log_message("WARN", now, message);
}

pub fn info(now: &str, message: &str) {
//! # Info
//! Outputs an info message via `log_info!` macros.
//! You can also use just an `info` method instead, but it requires you to define any variable as `&str`
//! > E.g:
//! ```rust
//! use logger_rust::info;
//! use logger_rust::current_time;
//! info(&current_time(), "An info message");
//! ```
    log_message("INFO", now, message);
}

pub fn debug(now: &str, message: &str) {
//! # Debug
//! Outputs an debug message via `log_debug` macros.
//! You can also use just an `debug` method instead, but it requires you to define any variable as `&str`
//! > E.g:
//! ```rust
//! use logger_rust::debug;
//! use logger_rust::current_time;
//! debug(&current_time(), "A debug message");
//! ```
    log_message("DEBUG", now, message);
}

#[macro_export]
/// ## Macro rules - log_error!
/// The log_error macro takes any number of arguments and formats them using the format! macro. 
/// It then gets the current time using the current_time function from the crate and calls the error function from the crate with the current time and formatted message.
macro_rules! log_error {
    ($($arg:tt)*) => {{
        let now = $crate::current_time();
        $crate::error(&now, &format!($($arg)*));
    }}
}

#[macro_export]
/// ## Macro rules - log_warn!
/// The log_warn macro takes any number of arguments and formats them using the format! macro. 
/// It then gets the current time using the current_time function from the crate and calls the warn function from the crate with the current time and formatted message.
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        let now = $crate::current_time();
        $crate::warn(&now, &format!($($arg)*));
    }}
}

#[macro_export]
/// ## Macro rules - log_info!
/// The log_info macro takes any number of arguments and formats them using the format! macro. 
/// It then gets the current time using the current_time function from the crate and calls the info function from the crate with the current time and formatted message.
macro_rules! log_info {
    ($($arg:tt)*) => {{
        let now = $crate::current_time();
        $crate::info(&now, &format!($($arg)*));
    }}
}

#[macro_export]
/// ## Macro rules - log_debug!
/// The log_debug macro takes any number of arguments and formats them using the format! macro. 
/// It then gets the current time using the current_time function from the crate and calls the debug function from the crate with the current time and formatted message.
macro_rules! log_debug {
    ($($arg:tt)*) => {{
        let now = $crate::current_time();
        $crate::debug(&now, &format!($($arg)*));
    }}
}