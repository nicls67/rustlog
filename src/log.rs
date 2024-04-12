//!
//! Logging module for `rustlog` crate
//!

extern crate chrono;
use std::io::Write;

use chrono::Local;

use crate::{data::{get_log_config, get_log_file}, RustLogConfig};

/// Writes the given `text` to log.
///
/// Caller name and date will be added in format _DATE-CALLER-TEXT_
pub fn write_log(text: String, caller: String) {
    // Get config, if config is None, do nothing
    if let Some(config) = get_log_config() {
        let date = format!("{} - ", Local::now().format("%Y-%m-%d %H:%M:%S"));
        let log = generate_log(text, caller, date, config);

        if config.log_to_terminal {
            println!("{log}");
        }
        if config.log_to_file.is_some() {
            if let Some(f) = get_log_file().as_mut() {
                f.write_all(format!("{log}\n").as_bytes()).unwrap_or(());
            }
        }
    }
}

/// Generates log string
fn generate_log(text: String, caller: String, date: String, config: &RustLogConfig) -> String {
    let mut output = String::new();
    if config.display_date {
        output = output + &date + " - ";
    }
    if config.display_caller {
        output = output + &caller + " - ";
    }
    output += &text;
    output
}


#[cfg(test)]
mod tests {
    use crate::RustLogConfig;

    use super::generate_log;

    #[test]
    fn formatting_1() -> Result<(), String> {
        let text = "Hello".to_string();
        let caller = "Me".to_string();
        let date = "2024-01-01 12:15:32".to_string();
        let config = RustLogConfig {
            log_to_terminal: true,
            log_to_file: None,
            append_to_file: false,
            display_date: false,
            display_caller: false,
            locked: false,
        };

        match generate_log(text, caller, date, &config).as_str() {
            "Hello" => Ok(()),
            s => Err(format!("Wrong log string received : {s}"))
        }
    }

    #[test]
    fn formatting_2() -> Result<(), String> {
        let text = "Hello".to_string();
        let caller = "Me".to_string();
        let date = "2024-01-01 12:15:32".to_string();
        let config = RustLogConfig {
            log_to_terminal: true,
            log_to_file: None,
            append_to_file: false,
            display_date: false,
            display_caller: true,
            locked: false,
        };

        match generate_log(text, caller, date, &config).as_str() {
            "Me - Hello" => Ok(()),
            s => Err(format!("Wrong log string received : {s}"))
        }
    }

    #[test]
    fn formatting_3() -> Result<(), String> {
        let text = "Hello".to_string();
        let caller = "Me".to_string();
        let date = "2024-01-01 12:15:32".to_string();
        let config = RustLogConfig {
            log_to_terminal: true,
            log_to_file: None,
            append_to_file: false,
            display_date: true,
            display_caller: true,
            locked: false,
        };

        match generate_log(text, caller, date, &config).as_str() {
            "2024-01-01 12:15:32 - Me - Hello" => Ok(()),
            s => Err(format!("Wrong log string received : {s}"))
        }
    }

    #[test]
    fn formatting_4() -> Result<(), String> {
        let text = "Hello".to_string();
        let caller = "Me".to_string();
        let date = "2024-01-01 12:15:32".to_string();
        let config = RustLogConfig {
            log_to_terminal: true,
            log_to_file: None,
            append_to_file: false,
            display_date: true,
            display_caller: false,
            locked: false,
        };

        match generate_log(text, caller, date, &config).as_str() {
            "2024-01-01 12:15:32 - Hello" => Ok(()),
            s => Err(format!("Wrong log string received : {s}"))
        }
    }
}