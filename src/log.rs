//!
//! Logging module for `rustlog` crate
//!

extern crate chrono;
use std::io::Write;

use chrono::Local;

use crate::{
    data::{get_log_config, get_log_file},
    LogSeverity, RustLogConfig,
};

/// Writes the given `text` to log.
///
/// Severity, caller name, and date will be added in the format _DATE-SEVERITY-CALLER-TEXT_.
///
/// # Arguments
///
/// * `severity` - The severity level of the log (Verbose, Info, Warning, Error).
/// * `text` - The log message to be written.
/// * `caller` - The name of the function or module that is writing the log.
pub fn write_log(severity: LogSeverity, text: &String, caller: &String) {
    // Get config, if config is None, do nothing
    if let Some(config) = get_log_config() {
        let mut disp_severity = true;

        if let Some(min_severity) = config.display_severity {
            // Check if message should be logged according to severity
            disp_severity = match min_severity {
                LogSeverity::Verbose => true,
                LogSeverity::Info => severity != LogSeverity::Verbose,
                LogSeverity::Warning => {
                    matches!(severity, LogSeverity::Error | LogSeverity::Warning)
                }
                LogSeverity::Error => severity == LogSeverity::Error,
            };
        }

        // Log message
        if disp_severity {
            let date = format!("{} - ", Local::now().format("%Y-%m-%d %H:%M:%S"));
            let log = generate_log(severity, text, caller, date, config);

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
}

/// Generates a formatted log string based on the provided configuration.
///
/// Combines the date, severity, caller, and the log message into a single string.
/// The format of the log string depends on the `RustLogConfig` settings.
///
/// # Arguments
///
/// * `severity` - The severity level of the log (Verbose, Info, Warning, Error).
/// * `text` - The log message to be written.
/// * `caller` - The name of the function or module that is writing the log.
/// * `date` - The current date and time as a string.
/// * `config` - A reference to the configuration settings for the logger.
///
/// # Returns
///
/// A `String` containing the formatted log message.
fn generate_log(
    severity: LogSeverity,
    text: &String,
    caller: &String,
    date: String,
    config: &RustLogConfig,
) -> String {
    let mut output = String::new();
    if config.display_date {
        output = output + &date + " - ";
    }
    if config.display_severity.is_some() {
        let sev_str = match severity {
            LogSeverity::Verbose => "VERB",
            LogSeverity::Info => "INFO",
            LogSeverity::Warning => "WARNING",
            LogSeverity::Error => "ERROR",
        };
        output = output + sev_str + " - ";
    }
    if config.display_caller {
        output = output + caller + " - ";
    }
    output += text;
    output
}

#[cfg(test)]
mod tests {
    use crate::{LogSeverity, RustLogConfig};

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
            display_severity: None,
        };

        match generate_log(LogSeverity::Info, &text, &caller, date, &config).as_str() {
            "Hello" => Ok(()),
            s => Err(format!("Wrong log string received : {s}")),
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
            display_severity: None,
        };

        match generate_log(LogSeverity::Error, &text, &caller, date, &config).as_str() {
            "Me - Hello" => Ok(()),
            s => Err(format!("Wrong log string received : {s}")),
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
            display_severity: Some(LogSeverity::Info),
        };

        match generate_log(LogSeverity::Info, &text, &caller, date, &config).as_str() {
            "2024-01-01 12:15:32 - INFO - Me - Hello" => Ok(()),
            s => Err(format!("Wrong log string received : {s}")),
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
            display_caller: true,
            locked: false,
            display_severity: Some(LogSeverity::Warning),
        };

        match generate_log(LogSeverity::Info, &text, &caller, date, &config).as_str() {
            "2024-01-01 12:15:32 - INFO - Me - Hello" => Ok(()),
            s => Err(format!("Wrong log string received : {s}")),
        }
    }
}
