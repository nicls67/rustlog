use std::fs::File;

use crate::log_config::RustLogConfig;

pub static mut LOG_CONFIG: Option<RustLogConfig> = None;
pub static mut LOG_FILE: Option<File> = None;

/// Returns log configuration with `unsafe` wrapping.
///
/// # Safety
///
/// This function performs an unsafe operation by dereferencing a raw pointer.
///
/// # Returns
///
/// An `Option` containing a reference to a `RustLogConfig` if the global `LOG_CONFIG` is set, or `None` if it is not.
pub fn get_log_config() -> Option<&'static RustLogConfig> {
    unsafe { LOG_CONFIG.as_ref() }
}

/// Checks if the global log configuration is set.
///
/// # Safety
///
/// This function performs an unsafe operation by dereferencing a raw pointer.
/// Make sure the global `LOG_CONFIG` has been properly initialized before
/// calling this function to avoid undefined behavior.
///
/// # Returns
///
/// `true` if the global log configuration is set, `false` otherwise.
pub fn is_log_configured() -> bool {
    unsafe { LOG_CONFIG }.is_some()
}

/// Returns a reference to the global log file if it is set.
///
/// # Safety
///
/// This function performs an unsafe operation by dereferencing a raw pointer.
/// Make sure the global `LOG_FILE` has been properly initialized before calling
/// this function to avoid undefined behavior.
///
/// # Returns
///
/// An `Option` containing a reference to a `File` if the global `LOG_FILE` is set, or `None` if it is not.
pub fn get_log_file() -> Option<&'static File> {
    unsafe { LOG_FILE.as_ref() }
}
