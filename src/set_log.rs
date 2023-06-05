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
    let log_variables = LogVariablesImpl;
    let mut log_level = log_variables.log_level().lock().unwrap();
    *log_level = level;
}

pub fn set_log_path<P: AsRef<Path>>(path: P) {
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
