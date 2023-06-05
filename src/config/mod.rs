use std::{
    sync::Mutex,
    path::PathBuf
};

lazy_static::lazy_static! {
    pub static ref LOG_LEVEL: Mutex<LogLevel> = Mutex::new(LogLevel::Console);
    pub static ref LOG_PATH: Mutex<PathBuf> = Mutex::new(PathBuf::new());
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
    fn log_level(&self) -> &Mutex<LogLevel>;
}

pub struct LogVariablesImpl;

impl LogVariables for LogVariablesImpl {
    fn log_level(&self) -> &Mutex<LogLevel> {
        &LOG_LEVEL
    }
}
