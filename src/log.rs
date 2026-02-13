//!
//! Logging module for `rustlog` crate
//!

use std::io::Write;

use chrono::Local;

use crate::{
    data::{get_log_config, G_LOG_FILE},
    LogSeverity, RustLogConfig,
};

/// Writes the given `p_text` to log.
///
/// Severity, caller name, and date will be added in the format _DATE-SEVERITY-CALLER-TEXT_.
///
/// # Arguments
///
/// * `p_severity` - The severity level of the log (Verbose, Info, Warning, Error).
/// * `p_text` - The log message to be written.
/// * `p_caller` - The name of the function or module that is writing the log.
///
/// # Returns
///
/// Nothing.
///
/// # Error handling
///
/// This function does not return any error.
///
/// # Panicking
///
/// This function will never panic.
pub fn write_log(p_severity: LogSeverity, p_text: &str, p_caller: &str) {
    // Get config, if config is None, do nothing
    if let Some(l_config) = get_log_config() {
        let mut l_disp_severity = true;

        if let Some(l_min_severity) = l_config.display_severity {
            // Check if message should be logged according to severity
            l_disp_severity = match l_min_severity {
                LogSeverity::Verbose => true,
                LogSeverity::Info => p_severity != LogSeverity::Verbose,
                LogSeverity::Warning => {
                    matches!(p_severity, LogSeverity::Error | LogSeverity::Warning)
                }
                LogSeverity::Error => p_severity == LogSeverity::Error,
            };
        }

        // Log message
        if l_disp_severity {
            let l_date = format!("{} - ", Local::now().format("%Y-%m-%d %H:%M:%S"));
            let l_log = generate_log(p_severity, p_text, p_caller, l_date, &l_config);

            if l_config.log_to_terminal {
                println!("{l_log}");
            }
            if l_config.log_to_file.is_some() {
                if let Some(l_f) = G_LOG_FILE.lock().unwrap().as_mut() {
                    l_f.write_all(format!("{l_log}\n").as_bytes()).unwrap_or(());
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
/// * `p_severity` - The severity level of the log (Verbose, Info, Warning, Error).
/// * `p_text` - The log message to be written.
/// * `p_caller` - The name of the function or module that is writing the log.
/// * `p_date` - The current date and time as a string.
/// * `p_config` - A reference to the configuration settings for the logger.
///
/// # Returns
///
/// A `String` containing the formatted log message.
///
/// # Error handling
///
/// This function does not return any error.
///
/// # Panicking
///
/// This function will never panic.
fn generate_log(
    p_severity: LogSeverity,
    p_text: &str,
    p_caller: &str,
    p_date: String,
    p_config: &RustLogConfig,
) -> String {
    let mut l_output = String::new();
    if p_config.display_date {
        l_output = l_output + &p_date + " - ";
    }
    if p_config.display_severity.is_some() {
        let l_sev_str = match p_severity {
            LogSeverity::Verbose => "VERB",
            LogSeverity::Info => "INFO",
            LogSeverity::Warning => "WARNING",
            LogSeverity::Error => "ERROR",
        };
        l_output = l_output + l_sev_str + " - ";
    }
    if p_config.display_caller {
        l_output = l_output + p_caller + " - ";
    }
    l_output += p_text;
    l_output
}

#[cfg(test)]
mod tests {
    use crate::{LogSeverity, RustLogConfig};

    use super::generate_log;

    #[test]
    fn formatting_1() -> Result<(), String> {
        let l_text = "Hello";
        let l_caller = "Me";
        let l_date = "2024-01-01 12:15:32".to_string();
        let l_config = RustLogConfig {
            log_to_terminal: true,
            log_to_file: None,
            append_to_file: false,
            display_date: false,
            display_caller: false,
            locked: false,
            display_severity: None,
        };

        match generate_log(LogSeverity::Info, l_text, l_caller, l_date, &l_config).as_str() {
            "Hello" => Ok(()),
            s => Err(format!("Wrong log string received : {s}")),
        }
    }

    #[test]
    fn formatting_2() -> Result<(), String> {
        let l_text = "Hello";
        let l_caller = "Me";
        let l_date = "2024-01-01 12:15:32".to_string();
        let l_config = RustLogConfig {
            log_to_terminal: true,
            log_to_file: None,
            append_to_file: false,
            display_date: false,
            display_caller: true,
            locked: false,
            display_severity: None,
        };

        match generate_log(LogSeverity::Error, l_text, l_caller, l_date, &l_config).as_str() {
            "Me - Hello" => Ok(()),
            s => Err(format!("Wrong log string received : {s}")),
        }
    }

    #[test]
    fn formatting_3() -> Result<(), String> {
        let l_text = "Hello";
        let l_caller = "Me";
        let l_date = "2024-01-01 12:15:32".to_string();
        let l_config = RustLogConfig {
            log_to_terminal: true,
            log_to_file: None,
            append_to_file: false,
            display_date: true,
            display_caller: true,
            locked: false,
            display_severity: Some(LogSeverity::Info),
        };

        match generate_log(LogSeverity::Info, l_text, l_caller, l_date, &l_config).as_str() {
            "2024-01-01 12:15:32 - INFO - Me - Hello" => Ok(()),
            s => Err(format!("Wrong log string received : {s}")),
        }
    }

    #[test]
    fn formatting_4() -> Result<(), String> {
        let l_text = "Hello";
        let l_caller = "Me";
        let l_date = "2024-01-01 12:15:32".to_string();
        let l_config = RustLogConfig {
            log_to_terminal: true,
            log_to_file: None,
            append_to_file: false,
            display_date: true,
            display_caller: true,
            locked: false,
            display_severity: Some(LogSeverity::Warning),
        };

        match generate_log(LogSeverity::Info, l_text, l_caller, l_date, &l_config).as_str() {
            "2024-01-01 12:15:32 - INFO - Me - Hello" => Ok(()),
            s => Err(format!("Wrong log string received : {s}")),
        }
    }
}
