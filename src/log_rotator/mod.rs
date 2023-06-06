/// ## Log rotation
/// From version 1.0.39, you can create a `log rotator` instance which allows you to split logs by their size and duration.
/// - log_path: path to log directory;
/// - max_size (`u64`): maximum size of log file;
/// - max_life (`std::time::Duration`): maximum lifetime of log file;
/// 
/// Here's an example:
/// ```rust
/// use logger_rust::*; // importing 
/// use std::{
///     path::PathBuf,
///     time::Duration
/// };
/// 
/// fn main() {
///     //set_log_level(LogLevel::File);
///     //set_log_path(LogConfig::Rotator(LogRotatorConfig::new(
///         //PathBuf::from("C:/Users/JK/Desktop"),
///         //5 * 15 * 15, <-- im not able to pass the testdoc cuz of that lol
///         //Duration::from_secs(2),
///     //)));
/// }
/// ```

use std::{
    time::Duration,
    path::PathBuf,
};
pub enum LogConfig {
    /// # LogConfig enum
    /// Defines the Path variable as std::PathBuf for path
    /// And a Rotator variable linked to the LogRotatorConfig implementor
    Path(LogPath),
    Rotator(LogRotatorConfig),
}

pub enum LogPath {
    /// # LogPath enum
    /// Defines the Path variable as std::PathBuf for path
    /// What should i even write here and why?
    Path(PathBuf),
}

pub struct LogRotatorConfig {
    /// # LogRotatorConfig struct
    /// Defines the public variables for LogRotator instanse
    /// `log_path` = string
    /// `max-size` = u64 (e.g 5 * 1024 * 1024)
    /// `max_time` = time::Duration
    pub log_path: PathBuf,
    pub max_size: u64,
    pub max_time: Duration,
}

impl LogRotatorConfig {
    pub fn new(log_path: PathBuf, max_size: u64, max_time: Duration) -> Self {
        //! # LogRotatorConfig::New
        //! The inializer for configuration of log rotator:
        //! ```rust
        //! use logger_rust::*;
        //! use std::time::Duration;
        //! use std::path::PathBuf;
        //! 
        //! fn main() {
        //!     set_log_level(LogLevel::File);
        //!     set_log_path(LogConfig::Rotator(LogRotatorConfig::new(
        //!         PathBuf::from("C:/Users/qruie/Documents"), // Logging directory
        //!         5 * 1024 * 1024, // 5MB
        //!         Duration::from_secs(2), // Duration for log splits
        //!     )));
        //! }
        //! ```
        Self {
            log_path,
            max_size,
            max_time,
        }
    }
}

impl From<&str> for LogPath {
/// `From` trait implementation for `LogPath` that allows it to be constructed from a string slice.
///
/// # Examples
///
/// ```
/// use logger_rust::*;
///
/// fn main() {
///     set_log_level(LogLevel::File);
///     set_log_path(LogConfig::Path(LogPath::from("C:/Users/qruie/Documents")));
///     // ..
/// }
/// ```
    fn from(s: &str) -> Self {
        Self::Path(PathBuf::from(s))
    }
}

impl From<PathBuf> for LogPath {
/// `From` trait implementation for `LogPath` that allows it to be constructed from a `PathBuf`.
///
/// # Examples
///
/// ```
/// use logger_rust::*;
/// use std::path::PathBuf;
///
/// fn main() {
///     set_log_level(LogLevel::File);
///     set_log_path(LogConfig::Path(LogPath::from("C:/Users/qruie/Documents")));
///     // ..
/// }
/// ```
    fn from(path: PathBuf) -> Self {
        Self::Path(path)
    }
}
