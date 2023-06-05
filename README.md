# Log Crate
This crate provides a simple and flexible logging system for Rust projects. It allows you to log messages of different types (error, warn, info, debug) using macros and choose whether to log messages to the console, a file, or both.

# Features
- Log messages of different types (error, warn, info, debug) using macros
- Choose whether to log messages to the console, a file, or both
- Set the log level at runtime
- Automatically create and append to log files based on the current date
- Handle errors when writing to log files

## Usage
To use this crate in your project, add it as a dependency in your `Cargo.toml` file:

```env
[dependencies]
logger-rust = "0.1.0"
````
Then, import the crate:
```rust
#[macro_use]
extern crate logger_rust;
use logger_rust::{set_log_level, LogLevel};
```
You can now use the `log_error!`, `log_warn!`, `log_info!`, and `log_debug!` macros to log messages of different types:
```rust
fn main() {
    log_error!("An error occurred: {}", "Something went wrong");
    log_warn!("A warning occurred: {}", "Something might be wrong");
    log_info!("An info message: {}", "Something happened");
    log_debug!("A debug message: {}", "Something happened in detail");
}
```
By default, log messages are printed to the console. You can use the `set_log_level` function to specify where log messages should be written:
```rust
use log_crate::{set_log_level, LogLevel};

fn main() {
    set_log_level(LogLevel::Both);

    // ...
}
```

The `set_log_level` function takes a LogLevel as an argument. You can pass one of the following variants to specify where log messages should be written:

- LogLevel::Console: Log messages are printed to the console only.
- LogLevel::File: Log messages are written to a file only.
- LogLevel::Both: Log messages are printed to the console and written to a file.
> When logging messages to a file, the crate will automatically create a new file with a name based on the current date (e.g. 2023-06-04.log) or append to an existing file with the same name. If an error occurs while writing to the file (e.g. if the file is not accessible), an error message will be printed to the console.

## Examples
Here’s an example that shows how to use this crate in a Rust project:
```rust
#[macro_use]
extern crate logger_rust;
use log_crate::{set_log_level, LogLevel};

fn main() {
    set_log_level(LogLevel::Both);

    log_error!("An error occurred: {}", "Something went wrong");
    log_warn!("A warning occurred: {}", "Something might be wrong");
    log_info!("An info message: {}", "Something happened");
    log_debug!("A debug message: {}", "Something happened in detail");
}
```
Output:
```diff
- 2023-06-05 12:23:25 [ERROR] An error occurred: Something went wrong
- 2023-06-05 12:23:25 [WARN] A warning occurred: Something might be wrong
A warning occurred: Something might be wrong
+ 2023-06-05 12:23:25 [INFO] An info message: Something happened
An info message: Something happened
+ 2023-06-05 12:23:25 [DEBUG] A debug message: Something happened in detail
A debug message: Something happened in detail
```
This code sets the log level to LogLevel::Both, which means that log messages will be printed to the console and written to a file. It then uses the logging macros to log messages of different types.
