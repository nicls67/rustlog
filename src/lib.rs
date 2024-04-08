use std::fs::File;

use data::{get_log_options, LOG_FILE, LOG_OPTIONS};
pub use data::{LogMethod, LogOptions};

#[doc = include_str!("../README.md")]
mod data;

/// ### Logging configuration
///
/// Configures logging according to given `method` and `options`.
/// This function can be called only once, if called a second time, will return `Err`
pub fn configure_log(method: LogMethod, options: LogOptions) -> Result<(), String> {
    // Logging is already configured, return Err
    if get_log_options().is_some() {
        Err(String::from("Logging already configured"))
    } else {
        unsafe {
            LOG_OPTIONS = Some(options);
        }
        match method {
            LogMethod::ToTerminal => Ok(()),
            LogMethod::ToFile(filename, append) | LogMethod::Both(filename, append) => {
                match File::options()
                    .create(true)
                    .append(append)
                    .write(true)
                    .open(filename)
                {
                    Ok(f) => unsafe {
                        LOG_FILE = Some(f);
                        Ok(())
                    },
                    Err(e) => Err(format!("{e}")),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;

    use crate::{
        configure_log,
        data::{get_log_file, get_log_options},
        LogMethod, LogOptions,
    };

    #[test]
    fn log_already_configured() -> Result<(), String> {
        // Set config to Some
        unsafe { crate::data::LOG_OPTIONS = Some(LogOptions { add_date: true }) }

        let method = LogMethod::ToTerminal;
        let options = LogOptions { add_date: false };

        // Function shall return Err
        match configure_log(method, options) {
            Ok(_) => Err("configure_log should return Err variant".to_string()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn log_not_configured_terminal() -> Result<(), String> {
        // Set config to None
        unsafe { crate::data::LOG_OPTIONS = None }
        unsafe { crate::data::LOG_FILE = None }

        let method = LogMethod::ToTerminal;
        let options = LogOptions { add_date: false };

        // Function shall return Ok
        // log file shall be None
        // Log options shall be Some
        configure_log(method, options)?;

        match get_log_file() {
            Some(_) => return Err("LOG_FILE should be None".to_string()),
            None => (),
        };

        match get_log_options() {
            Some(_) => Ok(()),
            None => Err("LOG_OPTIONS should be Some".to_string()),
        }
    }

    #[test]
    fn log_not_configured_file() -> Result<(), String> {
        // Set config to None
        unsafe {
            crate::data::LOG_OPTIONS = None;
        }
        unsafe {
            crate::data::LOG_FILE = None;
        }

        remove_file("log.txt").unwrap_or(());

        let method = LogMethod::ToFile("log.txt".to_string(), false);
        let options = LogOptions { add_date: false };

        // Function shall return Ok
        // log file shall be Some
        // Log options shall be Some
        configure_log(method, options)?;

        remove_file("log.txt").unwrap_or(());

        match get_log_file() {
            Some(_) => (),
            None => return Err("LOG_FILE should be Some".to_string()),
        };
        match get_log_options() {
            Some(_) => Ok(()),
            None => Err("LOG_OPTIONS should be Some".to_string()),
        }
    }

    #[test]
    fn log_not_configured_both() -> Result<(), String> {
        // Set config to None
        unsafe {
            crate::data::LOG_OPTIONS = None;
        }
        unsafe {
            crate::data::LOG_FILE = None;
        }

        remove_file("log.txt").unwrap_or(());

        let method = LogMethod::Both("log.txt".to_string(), true);
        let options = LogOptions { add_date: false };

        // Function shall return Ok
        // log file shall be Some
        // Log options shall be Some
        configure_log(method, options)?;

        remove_file("log.txt").unwrap_or(());

        match get_log_file() {
            Some(_) => (),
            None => return Err("LOG_FILE should be Some".to_string()),
        };
        match get_log_options() {
            Some(_) => Ok(()),
            None => Err("LOG_OPTIONS should be Some".to_string()),
        }
    }
}
