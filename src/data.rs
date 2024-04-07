use std::fs::File;

/// Logging method selection
/// 1. `ToTerminal` : log data to terminal
/// 2. `ToFile` : log data to the specified file. Variant uses tuple for configuration : 1st data of tuple is the file name,
/// if 2nd is `true`, new data will be added at the end of the file, if `false`, file will be overidden
/// 3. `Both` : log data to the specified file. Variant uses tuple for configuration : 1st data of tuple is the file name,
/// if 2nd is `true`, new data will be added at the end of the file, if `false`, file will be overidden
pub enum LogMethod {
    ToTerminal,
    ToFile(String, bool),
    Both(String, bool)
}

/// Logging options
/// * `add_date` : Add date before log message
pub struct LogOptions {
    pub add_date: bool
}

pub static mut LOG_OPTIONS: Option<LogOptions> = None;
pub static mut LOG_FILE: Option<File> = None;

/// Returns log configuration with `unsafe` wrapping
pub fn get_log_options() -> Option<&'static LogOptions> {
    unsafe{LOG_OPTIONS.as_ref()}
}

/// Returns log file with `unsafe` wrapping
pub fn get_log_file() -> Option<&'static File> {
    unsafe{LOG_FILE.as_ref()}
}