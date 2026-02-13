use std::fs::File;
use std::sync::Mutex;

use crate::log_config::RustLogConfig;

pub static G_LOG_CONFIG: Mutex<Option<RustLogConfig>> = Mutex::new(None);
pub static G_LOG_FILE: Mutex<Option<File>> = Mutex::new(None);

/// Returns log configuration.
///
/// This function retrieves the current log configuration from the global state.
///
/// # Returns
///
/// An `Option` containing a `RustLogConfig` if the global `G_LOG_CONFIG` is set, or `None` if it is not.
///
/// # Error handling
///
/// This function does not return any error.
///
/// # Panicking
///
/// This function will never panic.
pub fn get_log_config() -> Option<RustLogConfig> {
    *G_LOG_CONFIG.lock().unwrap()
}

/// Checks if the global log configuration is set.
///
/// This function checks if the global log configuration has been initialized.
///
/// # Returns
///
/// `true` if the global log configuration is set, `false` otherwise.
///
/// # Error handling
///
/// This function does not return any error.
///
/// # Panicking
///
/// This function will never panic.
pub fn is_log_configured() -> bool {
    G_LOG_CONFIG.lock().unwrap().is_some()
}
