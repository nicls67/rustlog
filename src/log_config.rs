//!
//! RustLog configuration module
//!

use std::fs::File;

use crate::data::{get_log_config, LOG_CONFIG, LOG_FILE};

/// Log configuration structure
#[derive(Clone, Copy)]
pub struct RustLogConfig {
    log_to_terminal: bool,
    log_to_file: Option<&'static str>,
    append_to_file: bool,
    add_date: bool,
}

impl RustLogConfig {
    /// Creates default log configuration :
    /// * All log destinations disabled
    /// * No date added
    pub fn new_config() -> RustLogConfig {
        RustLogConfig {
            log_to_terminal: false,
            log_to_file: None,
            append_to_file: false,
            add_date: false,
        }
    }

    /// Enables logging to terminal
    pub fn enable_terminal(&mut self) -> &mut RustLogConfig {
        self.log_to_terminal = true;
        self
    }

    /// Disables logging to terminal
    pub fn disable_terminal(&mut self) -> &mut RustLogConfig {
        self.log_to_terminal = false;
        self
    }

    /// Enables logging to file `log_file`
    /// If `append` is `True`, new content will be added at the end of the selected file, else file will be overriden
    pub fn enable_file(&mut self, log_file: &'static str, append: bool) -> &mut RustLogConfig {
        self.log_to_file = Some(log_file);
        self.append_to_file = append;
        self
    }

    /// Disables logging to file
    pub fn disable_file(&mut self) -> &mut RustLogConfig {
        self.log_to_file = None;
        self.append_to_file = false;
        self
    }

    /// Enables date display for each log entry
    pub fn add_date(&mut self, add_date: bool) -> &mut RustLogConfig {
        self.add_date = add_date;
        self
    }

    /// ## Logging configuration
    ///
    /// Configures logging according to the selected configuration
    /// This function can be called only once, if called a second time, will return `Err`
    ///
    /// ### Returns
    /// Returns `Err` variant in the following cases :
    /// * When called twice (even by multiple crates)
    /// * If all logging destination are disabled
    pub fn configure(&self) -> Result<(), String> {
        // Logging is already configured, return Err
        if get_log_config().is_some() {
            Err(String::from("Logging already configured"))
        } else {
            // Save configuration
            unsafe {
                LOG_CONFIG = Some(*self);
            }

            // At least one log destination must be selected
            if self.log_to_terminal == false && self.log_to_file.is_none() {
                return Err("All log destinations are disabled".to_string());
            }

            // Create log file
            if let Some(log_file) = self.log_to_file {
                match File::options()
                    .create(true)
                    .append(self.append_to_file)
                    .write(true)
                    .open(log_file)
                {
                    Ok(f) => unsafe {
                        LOG_FILE = Some(f);
                    },
                    Err(e) => return Err(format!("{e}")),
                };
            }

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;

    use crate::{
        data::{get_log_config, get_log_file},
        RustLogConfig,
    };

    #[test]
    fn log_already_configured() -> Result<(), String> {
        // Set config to Some
        unsafe {
            crate::data::LOG_CONFIG = Some(RustLogConfig {
                log_to_terminal: true,
                log_to_file: None,
                append_to_file: false,
                add_date: false,
            });
        }

        match RustLogConfig::new_config().configure() {
            Ok(_) => Err("configure_log should return Err variant".to_string()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn log_not_configured_terminal() -> Result<(), String> {
        // Set config to None
        unsafe { crate::data::LOG_CONFIG = None }
        unsafe { crate::data::LOG_FILE = None }

        // Function shall return Ok
        // log file shall be None
        // Log options shall be Some
        RustLogConfig::new_config().enable_terminal().configure()?;

        match get_log_file() {
            Some(_) => return Err("LOG_FILE should be None".to_string()),
            None => (),
        };

        match get_log_config() {
            Some(_) => Ok(()),
            None => Err("LOG_OPTIONS should be Some".to_string()),
        }
    }

    #[test]
    fn log_not_configured_file() -> Result<(), String> {
        // Set config to None
        unsafe {
            crate::data::LOG_CONFIG = None;
        }
        unsafe {
            crate::data::LOG_FILE = None;
        }

        remove_file("log.txt").unwrap_or(());

        // Function shall return Ok
        // log file shall be Some
        // Log options shall be Some
        RustLogConfig::new_config()
            .disable_terminal()
            .enable_file("log.txt", true)
            .configure()?;

        remove_file("log.txt").unwrap_or(());

        match get_log_file() {
            Some(_) => (),
            None => return Err("LOG_FILE should be Some".to_string()),
        };
        match get_log_config() {
            Some(_) => Ok(()),
            None => Err("LOG_OPTIONS should be Some".to_string()),
        }
    }

    #[test]
    fn log_not_configured_both() -> Result<(), String> {
        // Set config to None
        unsafe {
            crate::data::LOG_CONFIG = None;
        }
        unsafe {
            crate::data::LOG_FILE = None;
        }

        remove_file("log.txt").unwrap_or(());

        // Function shall return Ok
        // log file shall be Some
        // Log options shall be Some
        RustLogConfig::new_config()
            .enable_terminal()
            .enable_file("log.txt", true)
            .configure()?;

        remove_file("log.txt").unwrap_or(());

        match get_log_file() {
            Some(_) => (),
            None => return Err("LOG_FILE should be Some".to_string()),
        };
        match get_log_config() {
            Some(_) => Ok(()),
            None => Err("LOG_OPTIONS should be Some".to_string()),
        }
    }

    #[test]
    fn log_not_configured_all_disabled() -> Result<(), String> {
        // Set config to None
        unsafe {
            crate::data::LOG_CONFIG = None;
        }
        unsafe {
            crate::data::LOG_FILE = None;
        }

        remove_file("log.txt").unwrap_or(());

        // Function shall return Err
        // log file shall be None
        // Log options shall be None
        match RustLogConfig::new_config().configure() {
            Ok(_) => Err("configure_log should return Err variant".to_string()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn enable_terminal() -> Result<(), String> {
        let mut binding = RustLogConfig::new_config();
        let config = binding.enable_terminal();

        match config.log_to_terminal {
            true => Ok(()),
            false => Err("log_to_terminal should be TRUE".to_string()),
        }
    }

    #[test]
    fn enable_file() -> Result<(), String> {
        let mut binding = RustLogConfig::new_config();
        let config = binding.enable_file("log.txt", true);

        match config.log_to_file {
            Some(_) => match config.append_to_file {
                true => Ok(()),
                false => Err("append_to_file should be TRUE".to_string()),
            },
            None => Err("log_to_file should be Some".to_string()),
        }
    }
}
