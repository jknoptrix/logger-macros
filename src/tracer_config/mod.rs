//! It is not already used, it just an template for future using in log_trace! macro.
/// I'll be used as a tracing configuration:
/// ```rust
/// log_trace_conf(LogTrace::TracerConfiguration::new(
///   Context: enabled;
///   Timestamp: enabled;
///   Timestamp_Type: chrono;
///   File: enabled;
///   Line: enabled;
///   Format: "{}:{} - used: {} ->> ({:?}): {:?} ->> Thread ID: {:?} Timestamp: {}{}";
/// ));
/// ```
use std::sync::RwLock;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;

use crate::current_time;

lazy_static::lazy_static! {
    /// A global instance of `TracerConfiguration` that can be accessed and modified using the `set_tracer_config` and `get_tracer_config` functions.
    static ref TRACER_CONFIG: RwLock<TracerConfiguration> = RwLock::new(Default::default());
}

/// Sets the global tracer configuration to the given value.
///
/// # Arguments
///
/// * `config` - The new tracer configuration.
pub fn set_tracer_config(config: TracerConfiguration) {
    *TRACER_CONFIG.write().unwrap() = config;
}

/// Returns a copy of the current global tracer configuration.
pub fn get_tracer_config() -> TracerConfiguration {
    (*TRACER_CONFIG.read().unwrap()).clone()
}

/// An enum representing the different types of timestamps that can be used in log messages.
#[derive(Clone, Copy)]
pub enum TimestampType {
    /// A timestamp in the format "YYYY-MM-DD HH:MM:SS".
    Chrono,
    /// A Unix timestamp (the number of seconds since January 1, 1970).
    Unix,
}

/// A struct representing the configuration options for the tracer.
#[derive(Clone, Default)]
pub struct TracerConfiguration {
    /// Whether to include context information in log messages. If `None`, the default behavior is used.
    pub context_enabled: Option<bool>,
    /// Whether to include a timestamp in log messages. If `None`, the default behavior is used.
    pub timestamp_enabled: Option<bool>,
    /// The type of timestamp to use in log messages. If `None`, the default behavior is used.
    pub timestamp_type: Option<TimestampType>,
    /// Whether to include the file name in log messages. If `None`, the default behavior is used.
    pub file_enabled: Option<bool>,
    /// Whether to include the line number in log messages. If `None`, the default behavior is used.
    pub line_enabled: Option<bool>,
    /// The format string to use when generating log messages. If `None`, the default behavior is used.
    pub format: Option<String>,
}

impl TracerConfiguration {
    /// Creates a new `TracerConfiguration` with default values.
    pub fn new() -> Self {
        Default::default()
    }
}

/// A trait for types that can be logged using the tracer.
pub trait Loggable {
    /// Returns a string representation of the value that can be included in a log message.
    fn log_behavior(&self) -> String;
}

/// A trait for types that have a name that can be included in log messages.
pub trait TypeName {
    /// Returns the name of the type as a string.
    fn type_name(&self) -> &'static str;
}

impl<T: 'static> TypeName for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

/// A trait for accessing information about the current state of the program that can be included in log messages.
pub trait TracerConfig {
    /// Returns a string representation of the current time.
    fn now() -> String;
    /// Returns the current line number.
    fn line() -> u32;
    /// Returns the name of the current file.
    fn file() -> &'static str;
    /// Returns the path of the current module.
    fn module_path() -> &'static str;
    /// Returns the ID of the current thread.
    fn thread_id() -> std::thread::ThreadId;
    /// Returns a timestamp representing the current time.
    fn timestamp() -> u128;
}

impl TracerConfig for () {
    fn now() -> String {
        current_time()
    }

    fn line() -> u32 {
        line!()
    }

    fn file() -> &'static str {
        file!()
    }

    fn module_path() -> &'static str {
        module_path!()
    }

    fn thread_id() -> std::thread::ThreadId {
        std::thread::current().id()
    }

    fn timestamp() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros()
    }
}
