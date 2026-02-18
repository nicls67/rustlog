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
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LogSeverity {
    Verbose,
    Info,
    Warning,
    Error,
}

impl LogSeverity {
    /// Returns the numeric value associated with each severity level.
    ///
    /// This is used internally for ordering comparisons.
    ///
    /// # Returns
    ///
    /// A `u8` value representing the severity level: `Verbose` = 0, `Info` = 1,
    /// `Warning` = 2, `Error` = 3.
    fn level(&self) -> u8 {
        match self {
            LogSeverity::Verbose => 0,
            LogSeverity::Info => 1,
            LogSeverity::Warning => 2,
            LogSeverity::Error => 3,
        }
    }
}

impl PartialOrd for LogSeverity {
    fn partial_cmp(&self, p_other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.level().cmp(&p_other.level()))
    }
}

impl fmt::Display for LogSeverity {
    fn fmt(&self, p_f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogSeverity::Verbose => write!(p_f, "VERB"),
            LogSeverity::Info => write!(p_f, "INFO"),
            LogSeverity::Warning => write!(p_f, "WARNING"),
            LogSeverity::Error => write!(p_f, "ERROR"),
        }
    }
}

pkg_infos!();
