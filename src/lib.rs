#[doc = include_str!("../README.md")]
mod data;
mod log;
mod log_config;

pub use log::write_log;
pub use log_config::RustLogConfig;
use package_infos::pkg_infos;
use package_infos::PackageInfos;

use std::fmt;

/// Defines log message severity
///
/// `LogSeverity` is an enumeration that specifies the severity levels of log messages.
/// It includes four levels: `Verbose`, `Info`, `Warning`, and `Error`.
///
/// # Variants
///
/// * `Verbose` - Used for detailed debug-level messages.
/// * `Info` - Used for informational messages that highlight the progress of the application.
/// * `Warning` - Used for potentially harmful situations that might need attention.
/// * `Error` - Used for error events that might still allow the application to continue running.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum LogSeverity {
    Verbose,
    Info,
    Warning,
    Error,
}

impl fmt::Display for LogSeverity {
    fn fmt(&self, p_f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogSeverity::Verbose => p_f.write_str("VERB"),
            LogSeverity::Info => p_f.write_str("INFO"),
            LogSeverity::Warning => p_f.write_str("WARNING"),
            LogSeverity::Error => p_f.write_str("ERROR"),
        }
    }
}

pkg_infos!();

#[cfg(test)]
mod tests {
    use super::*;
    use rusttests::{check_value, CheckType};

    #[test]
    fn test_log_severity_partial_cmp() -> Result<(), String> {
        check_value(
            (1, 1),
            &(LogSeverity::Verbose < LogSeverity::Info),
            &true,
            CheckType::Equal,
        )?;
        check_value(
            (2, 1),
            &(LogSeverity::Info < LogSeverity::Warning),
            &true,
            CheckType::Equal,
        )?;
        check_value(
            (3, 1),
            &(LogSeverity::Warning < LogSeverity::Error),
            &true,
            CheckType::Equal,
        )?;
        check_value(
            (4, 1),
            &(LogSeverity::Verbose == LogSeverity::Verbose),
            &true,
            CheckType::Equal,
        )?;
        Ok(())
    }

    #[test]
    fn test_log_severity_display() -> Result<(), String> {
        check_value(
            (1, 1),
            &format!("{}", LogSeverity::Verbose),
            &"VERB".to_string(),
            CheckType::Equal,
        )?;
        check_value(
            (2, 1),
            &format!("{}", LogSeverity::Info),
            &"INFO".to_string(),
            CheckType::Equal,
        )?;
        check_value(
            (3, 1),
            &format!("{}", LogSeverity::Warning),
            &"WARNING".to_string(),
            CheckType::Equal,
        )?;
        check_value(
            (4, 1),
            &format!("{}", LogSeverity::Error),
            &"ERROR".to_string(),
            CheckType::Equal,
        )?;
        Ok(())
    }
}
