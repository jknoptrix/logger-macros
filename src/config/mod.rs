pub use crate::log_rotator::LogRotatorConfig;

use std::{
    sync::Mutex,
    path::PathBuf
};

lazy_static::lazy_static! {
    pub static ref LOG_LEVEL: Mutex<LogLevel> = Mutex::new(LogLevel::Console);
    pub static ref LOG_PATH: Mutex<PathBuf> = Mutex::new(PathBuf::new());
    pub static ref LOG_MUTEX: Mutex<()> = Mutex::new(());
    pub static ref LOG_ROTATOR_CONFIG: Mutex<Option<LogRotatorConfig>> = Mutex::new(None);
}

#[derive(Copy, Clone)]
pub enum LogLevel {
/// # LogLevel enum
/// Defines the logger levels parameters: `Console`, `File`, `Both`
/// - The `Console` level will output logs only into console buffer
/// - The `File` level will place logs only into files, including existing logs
/// - The `Both` level **does the both**.
    Console,
    File,
    Both
}

pub trait LogVariables {
/// Trait that defines a method for accessing the current log level.
///
/// This trait defines a `log_level` method that returns a reference to a `Mutex<LogLevel>`
/// that contains the current log level. The log level determines where log messages are written:
/// to the console, to a file, or both.
/// `fn log_level(&self)` - Returns a reference to a `Mutex<LogLevel>` that contains the current log level.
    fn log_level(&self) -> &Mutex<LogLevel>;
}

pub struct LogVariablesImpl;
/// Implementation of the `LogVariables` trait for the `LogVariablesImpl` struct.
impl LogVariables for LogVariablesImpl {
/// Implementation of the `LogVariables` trait for the `LogVariablesImpl` struct.
///
/// This implementation provides a `log_level` method that returns a reference to the
/// `LOG_LEVEL` static variable, which contains the current log level.
/// 
    fn log_level(&self) -> &Mutex<LogLevel> {
//! Returns a reference to the `LOG_LEVEL` static variable, which contains the current
//! log level.
        &LOG_LEVEL
    }
}
