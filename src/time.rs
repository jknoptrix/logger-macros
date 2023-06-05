use chrono::Local;

pub fn current_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

// wow separate module for 5 lines of code lmao