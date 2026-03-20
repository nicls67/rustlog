//!
//! Logging module for `rustlog` crate
//!

use chrono::Local;

use crate::{
    data::{get_log_config, write_to_log_file},
    LogSeverity, RustLogConfig,
};

/// Writes the given `p_text` to log.
///
/// Severity, caller name, and date will be added in the format _DATE-SEVERITY-CALLER-TEXT_.
/// If `p_text` contains multiple lines, each non-empty line is prefixed with the log header.
/// Empty text is silently ignored.
///
/// # Arguments
///
/// * `p_severity` - The severity level of the log (Verbose, Info, Warning, Error).
/// * `p_text` - The log message to be written. May contain multiple lines.
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
            l_disp_severity = p_severity >= l_min_severity;
        }

        // Log message
        if l_disp_severity && !p_text.is_empty() {
            let l_date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let mut l_log = generate_log(p_severity, p_text, p_caller, l_date, &l_config);

            if l_config.log_to_terminal {
                println!("{l_log}");
            }
            if l_config.log_to_file.is_some() {
                l_log.push('\n');
                let _ = write_to_log_file(l_log.as_bytes());
            }
        }
    }
}

/// Generates a formatted log string based on the provided configuration.
///
/// Builds a prefix from the date, severity, and caller according to `RustLogConfig`,
/// then prepends that prefix to every non-empty line of `p_text`.
/// Empty lines (including consecutive newlines) are filtered out.
/// Lines are joined with `\n` in the returned string.
///
/// # Arguments
///
/// * `p_severity` - The severity level of the log (Verbose, Info, Warning, Error).
/// * `p_text` - The log message to be written. May contain multiple lines.
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
    let mut l_prefix = String::new();
    if p_config.display_date {
        l_prefix.push_str(&p_date);
        l_prefix.push_str(" - ");
    }
    if p_config.display_severity.is_some() {
        l_prefix.push_str(&p_severity.to_string());
        l_prefix.push_str(" - ");
    }
    if p_config.display_caller {
        l_prefix.push_str(p_caller);
        l_prefix.push_str(" - ");
    }

    let mut l_output = String::new();
    let mut l_first = true;
    for l_line in p_text.lines().filter(|s| !s.is_empty()) {
        if !l_first {
            l_output.push('\n');
        }
        l_output.push_str(&l_prefix);
        l_output.push_str(l_line);
        l_first = false;
    }
    l_output
}

#[cfg(test)]
mod tests {
    use crate::{LogSeverity, RustLogConfig};
    use rusttests::{check_value, CheckType};

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

        check_value(
            (1, 1),
            &generate_log(LogSeverity::Info, l_text, l_caller, l_date, &l_config),
            &"Hello".to_string(),
            CheckType::Equal,
        )?;
        Ok(())
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

        check_value(
            (1, 1),
            &generate_log(LogSeverity::Error, l_text, l_caller, l_date, &l_config),
            &"Me - Hello".to_string(),
            CheckType::Equal,
        )?;
        Ok(())
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

        check_value(
            (1, 1),
            &generate_log(LogSeverity::Info, l_text, l_caller, l_date, &l_config),
            &"2024-01-01 12:15:32 - INFO - Me - Hello".to_string(),
            CheckType::Equal,
        )?;
        Ok(())
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

        check_value(
            (1, 1),
            &generate_log(LogSeverity::Info, l_text, l_caller, l_date, &l_config),
            &"2024-01-01 12:15:32 - INFO - Me - Hello".to_string(),
            CheckType::Equal,
        )?;
        Ok(())
    }

    #[test]
    fn formatting_multiline() -> Result<(), String> {
        let l_text = "Line 1\nLine 2\nLine 3";
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

        let expected = "2024-01-01 12:15:32 - INFO - Me - Line 1\n\
                        2024-01-01 12:15:32 - INFO - Me - Line 2\n\
                        2024-01-01 12:15:32 - INFO - Me - Line 3"
            .to_string();

        check_value(
            (1, 1),
            &generate_log(LogSeverity::Info, l_text, l_caller, l_date, &l_config),
            &expected,
            CheckType::Equal,
        )?;
        Ok(())
    }

    #[test]
    fn formatting_multiple_consecutive_newlines() -> Result<(), String> {
        let l_text = "Line 1\n\n\nLine 2\n\nLine 3";
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

        let expected = "2024-01-01 12:15:32 - INFO - Me - Line 1\n\
                        2024-01-01 12:15:32 - INFO - Me - Line 2\n\
                        2024-01-01 12:15:32 - INFO - Me - Line 3"
            .to_string();

        check_value(
            (1, 1),
            &generate_log(LogSeverity::Info, l_text, l_caller, l_date, &l_config),
            &expected,
            CheckType::Equal,
        )?;
        Ok(())
    }

    #[test]
    fn test_write_log_terminal() {
        crate::data::clear_log_config_and_file();
        crate::data::set_log_config(Some(RustLogConfig {
            log_to_terminal: true,
            log_to_file: None,
            append_to_file: false,
            display_date: false,
            display_caller: false,
            locked: false,
            display_severity: Some(LogSeverity::Warning),
        }));

        // Does not crash and prints to terminal
        super::write_log(LogSeverity::Error, "Test error message", "test");

        // This severity should be filtered out
        super::write_log(LogSeverity::Info, "Test info message", "test");
    }
}
