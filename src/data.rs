use std::fs::File;

use crate::log_config::RustLogConfig;

pub static mut LOG_CONFIG: Option<RustLogConfig> = None;
pub static mut LOG_FILE: Option<File> = None;

/// Returns log configuration with `unsafe` wrapping
pub fn get_log_config() -> Option<&'static RustLogConfig> {
    unsafe{LOG_CONFIG.as_ref()}
}

/// Returns log file with `unsafe` wrapping
pub fn get_log_file() -> Option<&'static File> {
    unsafe{LOG_FILE.as_ref()}
}