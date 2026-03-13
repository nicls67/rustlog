use std::fs::File;
use std::io::Write;
use std::sync::{Mutex, MutexGuard};

use crate::log_config::RustLogConfig;

static G_LOG_CONFIG: Mutex<Option<RustLogConfig>> = Mutex::new(None);
static G_LOG_FILE: Mutex<Option<File>> = Mutex::new(None);

/// Locks the given mutex and returns the guard, handling poisoning by returning the inner guard.
fn lock_mutex<T>(p_mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    p_mutex.lock().unwrap_or_else(|l_e| l_e.into_inner())
}

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
    *lock_mutex(&G_LOG_CONFIG)
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
    lock_mutex(&G_LOG_CONFIG).is_some()
}

/// Sets the global log configuration.
///
/// This function stores the given log configuration into the global state.
///
/// # Parameters
///
/// * `p_config` - An `Option<RustLogConfig>` to set. Pass `Some` to store a configuration, or `None` to clear it.
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
pub fn set_log_config(p_config: Option<RustLogConfig>) {
    *lock_mutex(&G_LOG_CONFIG) = p_config;
}

/// Returns a lock guard to the global log file.
///
/// This function acquires the mutex lock on the global log file and returns
/// the `MutexGuard`, allowing the caller to read or write to the file.
///
/// # Returns
///
/// A `MutexGuard<Option<File>>` providing access to the global log file.
///
/// # Error handling
///
/// This function does not return any error.
///
/// # Panicking
///
/// This function will never panic.
#[cfg(test)]
pub fn get_log_file() -> MutexGuard<'static, Option<File>> {
    lock_mutex(&G_LOG_FILE)
}

/// Writes the given data to the global log file.
///
/// This function acquires the mutex lock on the global log file and writes the
/// provided byte slice to it. If the log file is not set, this function does nothing.
///
/// # Parameters
///
/// * `p_data` - The byte slice to write to the log file.
///
/// # Returns
///
/// A `Result` indicating success or failure:
/// * `Ok(())` if the write was successful or no log file is set.
/// * `Err(String)` if an I/O error occurred during writing.
///
/// # Error handling
///
/// Returns an error string if an I/O error occurs during writing.
///
/// # Panicking
///
/// This function will never panic.
pub fn write_to_log_file(p_data: &[u8]) -> Result<(), String> {
    if let Some(l_f) = lock_mutex(&G_LOG_FILE).as_mut() {
        l_f.write_all(p_data).map_err(|l_e| format!("{l_e}"))
    } else {
        Ok(())
    }
}

/// Sets the global log file.
///
/// This function stores the given file into the global state.
///
/// # Parameters
///
/// * `p_file` - An `Option<File>` to set. Pass `Some` to store a file, or `None` to clear it.
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
pub fn set_log_file(p_file: Option<File>) {
    *lock_mutex(&G_LOG_FILE) = p_file;
}

/// Clears both the global log configuration and the global log file.
///
/// This function sets both `G_LOG_CONFIG` and `G_LOG_FILE` to `None`.
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
pub fn clear_log_config_and_file() {
    *lock_mutex(&G_LOG_CONFIG) = None;
    *lock_mutex(&G_LOG_FILE) = None;
}
