# Logging Macros
Project have logging macros for different log levels: `error`, `warn`, and `info`. These macros allow you to easily log messages with the current time and appropriate formatting for each log level.

## Usage
To use these macros, you need to declare the variable `now` using `let now = current_time();`. This is necessary because the macros require a reference to the current time to properly format the log messages.

You also need to add the following crate imports: `use crate::log::{<log type>}; use crate::{<method>};`. These imports are necessary to use the logging functions and the `current_time` function.

To log a message, use the appropriate macro for the desired log level. For example, to log an error message, use the `log_error!` macro like this: `log_error!(&now, "<message>");`. Note that you must pass a reference to `now` as the first argument to the macro.

### Example:
```rust
use logger_rust::{info, log_info};

fn main() {
    let now = current_time();
    log_info!(&now, "This is an informational message");
}
```
> The `log_info!` macro is defined using the macro_rules! macro. It takes two arguments: a reference to the current time `($now:expr)` and a format string with any additional arguments `($($arg:tt)*)`. The macro expands to a call to the info function with the current time and the formatted message as arguments.

The info function takes two arguments: 
- A reference to the current time `(now: &str)`;
- The message to log `(message: &str)`. 
It uses the colored crate to format the log message with appropriate colors and styles for an informational message. The formatted message is then printed to standard output using the println! macro.

## Time Module
The `current_time` function is defined in a separate module because it uses the `chrono` crate to get the current time. This function cannot be defined in the same module as the macros because Rust.
