#[doc = include_str!("../README.md")]
mod data;
mod log;
mod log_config;

pub use log::write_log;
pub use log_config::RustLogConfig;

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

/// Retrieves the library version and the authors as defined in the package metadata.
///
/// # Returns
///
/// A tuple containing:
/// - The version of the library as a `&'static str`.
/// - The authors of the library as a `&'static str`.
pub fn get_lib_infos() -> (&'static str, &'static str) {
    (env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"))
}
