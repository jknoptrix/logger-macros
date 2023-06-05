mod time;
mod config;
mod set_log;
mod log_file;
pub use crate::config::LOG_PATH;
pub use crate::time::current_time;
pub use crate::log_file::log_message;
pub use crate::config::{LogVariables, LogVariablesImpl, LogLevel};
pub use crate::set_log::{set_log_level, set_log_path};

pub fn error(now: &str, message: &str) {
    log_message("ERROR", now, message);
}

pub fn warn(now: &str, message: &str) {
    log_message("WARN", now, message);
}

pub fn info(now: &str, message: &str) {
    log_message("INFO", now, message);
}

pub fn debug(now: &str, message: &str) {
    log_message("DEBUG", now, message);
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