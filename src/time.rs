use chrono::Local;

pub fn current_time() -> String {
//! Determines and gets the current time for PC
//! Nothing special here lol i just placed a 5 line code into different module
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

// wow separate module for 5 lines of code lmao