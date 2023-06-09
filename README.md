# Logger macro
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
logger-rust = "0.2.13"
```
Then, import the crate:
```rust
use logger_rust::*;
```
You can now use the `log_error!`, `log_warn!`, `log_info!`, `log_debug` and `log_trace!` macros to log messages of different types:
```rust
fn main() {
    log_error!("An error occurred: {}", "Something went wrong");
    log_warn!("A warning occurred: {}", "Something might be wrong");
    log_info!("An info message: {}", "Something happened");
    log_debug!("A debug message: {}", "Something happened in detail");
}
```
log_trace! example:
```rust

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let my_struct = MyStruct;
    my_struct.do_something();
    log_trace!(my_struct.do_something());
    
    let x = 5;
    log_trace!(x);

    let person = Person { name: "Alice".to_string(), age: 20 };
    log_trace!(person, person.name);
}
```
Output:
```
2023-06-09 15:06:46 [TRACE] src\main.rs:L23/C5 - used: my_struct.do_something() ->> (()): () | Type: <()> | ThreadId(1) ->> Timestamp: UN1686305206652692IX | Module: debug
2023-06-09 15:06:46 [TRACE] src\main.rs:L26/C5 - used: x ->> (5): 5 | Type: <i32> | ThreadId(1) ->> Timestamp: UN1686305206653039IX | Module: debug
2023-06-09 15:06:46 [TRACE] src\main.rs:L29/C5 - used: person ->> (Person { name: "Alice", age: 20 }): Person { name: "Alice", age: 20 } | Type: <debug::Person> | ThreadId(1) ->> Timestamp: UN1686305206653281IX ->> Context: <Alice> | Module: debug
```
By default, log messages are printed to the console. You can use the `set_log_level` function to specify where log messages should be written:
```rust
use logger_rust::*;

fn main() {
    set_log_level(LogLevel::Both);

    // ...
}
```

The `set_log_level` function takes a LogLevel as an argument. You can pass one of the following variants to specify where log messages should be written:

- `LogLevel::Console`: Log messages are printed to the console only.
- `LogLevel::File`: Log messages are written to a file only.
- `LogLevel::Both`: Log messages are printed to the console and written to a file.
> When logging messages to a file, the crate will automatically create a new file with a name based on the current date (e.g. 2023-06-04.log) or append to an existing file with the same name. If an error occurs while writing to the file (e.g. if the file is not accessible), an error message will be printed to the console.

Also, you can set custom log path. The default path is the same as the path where is your `Cargo.toml` file located.
The `set_log_path` function takes a `string` as an argument. You can pass one of the following variants to specify where log messages should be written:
```rust
use logger_rust::*;

fn main() {
    set_log_path(LogConfig::Path(LogPath::from("/path/to/log/dir")));
}
```
Will create a log file on directory that you specified.
> Note that if you use "." as dir (which is really not necessary lol), you will get an error message because directory is already busy.

## Log rotation
From version 1.0.39, you can create a `log rotator` instance which allows you to split logs by their size and duration.
- log_path: path to log directory;
- max_size (`u64`): maximum size of log file;
- max_life (`std::time::Duration`): maximum lifetime of log file;

Here's an example:
```rust
use logger_rust::*; // importing logger
use std::{
    path::PathBuf,
    time::Duration
};

fn main() {
    set_log_path(LogConfig::Rotator(LogRotatorConfig::new(
        PathBuf::from("C:/Users/JK/Desktop"), // path to the log directory
        5 * 1024 * 1024, // 5 MB
        Duration::from_secs(3600), // duration
    )));
}
```
### Note that you **SHOULD NOT** use LogRotator and LogPath in single instance. You will block the log file.

# Examples
Here’s an example that shows how to use this crate in a Rust project:
```rust
use logger_rust::*;

fn main() {
    set_log_level(LogLevel::Both);
    log_error!("An error occurred: {}", "Something went wrong");
    log_warn!("A warning occurred: {}", "Something might be wrong");
    log_info!("An info message: {}", "Something happened");
    log_debug!("A debug message: {}", "Something happened in detail");
    // ...
}
```
Also if you want, you can add `set_log_path(LogConfig::Path(LogPath::From("string")` method:
```rust
use logger_rust::*;

fn main() {
    set_log_level(LogLevel::Both);
    set_log_path(LogConfig::Path(LogPath::from("C:/Users/JK/Desktop"))); // will output logs on desktop

    log_error!("An error occurred: {}", "Something went wrong");
    log_warn!("A warning occurred: {}", "Something might be wrong");
    log_info!("An info message: {}", "Something happened");
    log_debug!("A debug message: {}", "Something happened in detail");
}
```
Also you can use this:
```rust
    error(&current_time(), "An error message");
    warn(&current_time(), "A warning message");
    info(&current_time(), "An info message");
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

# Custom logging
After 0.1.2 version, you can create a custom logging function with macros.
1. Create a `class` named as function that you needed (e.g. `custom`):
```rust
pub fn custom(now: &str, message: &str) {
    log_message("TRACE", now, message);
}
```
Note that log_message functions requires 3 arguments: `now`, `message`, but you can use whatever you want.
2. Create a macros for method which you created:
```rust
#[macro_export]
macro_rules! log_custom {
    ($($arg:tt)*) => {{
        let now = $crate::current_time(); // create a timestamp
        $crate::trace(&now, &format!($($arg)*)); // output the timestamp with message
    }}
}
```
Note that `arg` requires 2 arguments.
3. Use it:
```rust
fn main() {
    set_log_level(LogLevel::Both);
    log_custom!("A custom log macros");
}
```
Since you have `fn custom`, you can also use this:
```rust
    custom(&current_time(), "A custom log");
```
It does the same, but you can provide a value instead of `&current_time()`. But it requires 2 arguments:
1. An `&str` *`now` in our contex*
2. A `string` value that contains message.