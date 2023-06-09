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
//! logger-rust = "0.2.12"
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
pub mod tracer_config;
pub use crate::set_log::*;
pub use crate::log_rotator::*;
pub use crate::tracer_config::*;
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

pub fn trace(now: &str, message: &str) {
//! # Debug
//! Outputs an debug message via `log_debug` macros.
//! You can also use just an `debug` method instead, but it requires you to define any variable as `&str`
//! > E.g:
//! ```rust
//! use logger_rust::debug;
//! use logger_rust::current_time;
//! debug(&current_time(), "A debug message");
//! ```
    log_message("TRACE", now, message);
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

#[macro_export]
/// `log_trace!` is a macro that logs trace-level messages.
///
/// ## Usage
///
/// The `log_trace!` macro can be used in three different ways:
///
/// 1. `log_trace!(debug_object)`: This logs the debug representation of the `debug_object` along with its type name and location information (file, line, column, module path).
///
/// 2. `log_trace!(debug_object, context)`: This logs the same information as the first form, but also includes a context string that can provide additional information about the log message.
///
/// 3. `log_trace!(format_string, args...)`: This logs a formatted message using the given format string and arguments. The format string should follow the same syntax as the standard `format!` macro.
///
/// ## Examples
///
/// ```rust
/// use logger_rust::*;
/// let x: i32 = 42;
/// log_trace!(x); // Logs: "TRACE 2023-06-09 14:57:47 [TRACE] src\<module>:L29/C1 - used: x ->> (42): 42 | Type: <i32> | ThreadId(4) ->> Timestamp: UN1686304667694020IX | Module <module>"
///
/// let y = "Hello, world!";
/// log_trace!(y, "greeting"); // Logs: "2023-06-09 14:57:47 [TRACE] src\<module>:L32/C1 - used: y ->> ("Hello, world!"): "Hello, world!" | Type: <&str> | ThreadId(4) ->> Timestamp: UN1686304667694335IX ->> Context: <greeting> | Module: <module>"
///
/// log_trace!(x, "{}"); // Logs: "TRACE used: x ->> (42): 42 | Type: <i32> ... <context is empty>"
/// ```
macro_rules! log_trace {
    ($debug_object:expr) => {{
        log_trace!($debug_object, "");
    }};
    ($debug_object:expr, $context:expr) => {{
        let now = $crate::current_time();
        let line = line!();
        let file = file!();
        let module_path = module_path!();
        let thread_id = std::thread::current().id();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros();
        let context_str = if $context.is_empty() {
            String::new()
        } else {
            format!("\x1b[36m ->> Context: \x1b[0m\x1b[1m<{}>", $context)
        };
        let type_name = $debug_object.type_name();
        let column = column!();
        let debug_info = format!(
            "\x1b[34m{}:L{}/C{} - used: \x1b[32m{}\x1b[36m ->> ({:?}): \x1b[31m{:?}\x1b[36m | \x1b[32mType: \x1b[0m\x1B[1m<{}>\x1b[0m | \x1b[32m{:?} \x1b[36m->> \x1b[34mTimestamp: UN{}IX{}\x1b[0m\x1b[36m |\x1b[33m Module: \x1b[0m{}",
            file, line,
            column,
            stringify!($debug_object), 
            &$debug_object, 
            $debug_object,
            type_name,
            thread_id,
            timestamp,
            context_str,
            module_path,
        );
        $crate::log_message("TRACE", &now, &debug_info);
    }};
    ($($arg:tt)*) => {{
        let now = $crate::current_time();
        $crate::log_message("TRACE", &now, &format!($($arg)*));
    }}
}