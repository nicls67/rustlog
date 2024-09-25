#[doc = include_str!("../README.md")]
mod data;
mod log;
mod log_config;

pub use log::write_log;
pub use log_config::RustLogConfig;
use package_infos::pkg_infos;
use package_infos::PackageInfos;

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
#[derive(Clone, Copy, PartialEq)]
pub enum LogSeverity {
    Verbose,
    Info,
    Warning,
    Error,
}

pkg_infos!();
