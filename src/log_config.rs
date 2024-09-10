//!
//! RustLog configuration module
//!

use std::{
    fs::{self, File},
    io::Write,
};

use chrono::Local;

use crate::{data::{get_log_config, get_log_file, LOG_CONFIG, LOG_FILE}, LogSeverity};

/// Log configuration structure
///
/// This structure holds the settings for logging configuration.
///
/// # Fields
///
/// * `log_to_terminal` - Whether to log messages to the terminal.
/// * `log_to_file` - Optionally, a file path where logs will be written. If `None`, logging to file is disabled.
/// * `append_to_file` - If `true`, new log messages will be appended to the specified file. If `false`, the file will be overwritten.
/// * `display_date` - Whether to display the date in log messages.
/// * `display_caller` - Whether to display the caller in log messages.
/// * `display_severity` - Optionally, the minimum severity level to display in log messages. If `None`, severity display is disabled.
/// * `locked` - If `true`, the configuration is locked and cannot be modified.
#[derive(Clone, Copy)]
pub struct RustLogConfig {
    pub(crate) log_to_terminal: bool,
    pub(crate) log_to_file: Option<&'static str>,
    pub(crate) append_to_file: bool,
    pub(crate) display_date: bool,
    pub(crate) display_caller: bool,
    pub(crate) display_severity: Option<LogSeverity>,
    pub(crate) locked: bool,
}

impl RustLogConfig {
    /// Creates a default log configuration
    ///
    /// This method returns an instance of `RustLogConfig` with the following default settings:
    /// * Logging to the terminal is disabled.
    /// * Logging to a file is disabled.
    /// * New log entries will not be appended to the file.
    /// * Date display for log entries is disabled.
    /// * Caller display for log entries is disabled.
    /// * The configuration is not locked, allowing modifications.
    /// * Log severity display is enabled with `INFO` as the minimum severity level.
    ///
    /// # Returns
    ///
    /// An instance of `RustLogConfig` with the default settings.
    pub fn new_config() -> RustLogConfig {
        RustLogConfig {
            log_to_terminal: false,
            log_to_file: None,
            append_to_file: false,
            display_date: false,
            display_caller: false,
            locked: false,
            display_severity: Some(LogSeverity::Info),
        }
    }

    /// Deletes the current log configuration.
    ///
    /// This function removes the current log configuration, including the log file reference.
    /// This operation can only be performed if the configuration is not locked.
    ///
    /// # Safety
    ///
    /// This function uses `unsafe` code to modify static mutable variables. Ensure that the
    /// configuration and log file are managed properly when calling this function to avoid
    /// potential data races or inconsistencies.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    ///
    pub fn clear_config() {
        if let Some(config) = get_log_config() {
            if !config.locked {
                unsafe {
                    LOG_CONFIG = None;
                    LOG_FILE = None;
                }
            }
        }
    }

    ///
    /// Once this method is called, the configuration cannot be cleared or modified.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `RustLogConfig` instance, allowing method chaining.
    ///
    pub fn lock(&mut self) -> &mut RustLogConfig {
        self.locked = true;
        self
    }

    ///
    /// This method sets the configuration to enable logging to the terminal.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `RustLogConfig` instance, allowing method chaining.
    pub fn enable_terminal(&mut self) -> &mut RustLogConfig {
        self.log_to_terminal = true;
        self
    }

    ///
    /// This method sets the configuration to disable logging to the terminal.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `RustLogConfig` instance, allowing method chaining.
    pub fn disable_terminal(&mut self) -> &mut RustLogConfig {
        self.log_to_terminal = false;
        self
    }

    /// Enables logging to the specified file.
    ///
    /// This method sets the configuration to enable logging to the specified file. 
    /// If `append` is `true`, new log entries will be added to the end of the file; 
    /// otherwise, the file will be overwritten.
    ///
    /// # Parameters
    ///
    /// * `log_file` - A string slice that holds the name of the file to log to.
    /// * `append` - A boolean value indicating whether to append to the file or overwrite it.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `RustLogConfig` instance, allowing method chaining.
    pub fn enable_file(&mut self, log_file: &'static str, append: bool) -> &mut RustLogConfig {
        self.log_to_file = Some(log_file);
        self.append_to_file = append;
        self
    }

    ///
    /// This method sets the configuration to disable logging to the file 
    /// and stops appending new log messages.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `RustLogConfig` instance, allowing method chaining.
    pub fn disable_file(&mut self) -> &mut RustLogConfig {
        self.log_to_file = None;
        self.append_to_file = false;
        self
    }

    /// Enables date display for each log entry
    ///
    /// # Parameters
    ///
    /// * `disp_date` - A boolean indicating whether to display the date for each log entry.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `RustLogConfig` instance, allowing method chaining.
    pub fn display_date(&mut self, disp_date: bool) -> &mut RustLogConfig {
        self.display_date = disp_date;
        self
    }

    /// Enables caller display for each log entry
    ///
    /// # Parameters
    ///
    /// * `disp_caller` - A boolean indicating whether to display the caller for each log entry.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `RustLogConfig` instance, allowing method chaining.
    pub fn display_caller(&mut self, disp_caller: bool) -> &mut RustLogConfig {
        self.display_caller = disp_caller;
        self
    }

    /// Enables severity display for each log entry.
    ///
    /// # Parameters
    ///
    /// * `disp_severity` - `None` to disable severity display. `Some` to enable severity display, with 
    ///    minimal displayed level given in variant.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `RustLogConfig` instance, allowing method chaining.
    pub fn display_severity(&mut self, disp_severity: Option<LogSeverity>) -> &mut RustLogConfig {
        self.display_severity = disp_severity;
        self
    }


    /// Configures the logging settings based on the current configuration.
    ///
    /// This method applies the current `RustLogConfig` instance's settings to configure
    /// logging destinations and options. Logging can be configured to terminal, file, or both.
    /// The configuration will be saved and can no longer be modified once applied.
    /// If logging is already configured, this method will return an error.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure:
    /// * `Ok(())` if logging is successfully configured.
    /// * `Err(String)` if logging is already configured, no log destinations are specified,
    ///   or if an error occurs while creating the log file.
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
            if !self.log_to_terminal && self.log_to_file.is_none() {
                return Err("All log destinations are disabled".to_string());
            }

            // Create log file
            if let Some(log_file) = self.log_to_file {
                match File::options()
                    .create(true)
                    .write(true)
                    .append(self.append_to_file)
                    .open(log_file)
                {
                    Ok(f) => {
                        unsafe {
                            LOG_FILE = Some(f);
                        };
                        let date = format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));

                        // Check if file is empty
                        match fs::read_to_string(log_file) {
                            Ok(s) => {
                                if !s.is_empty() && self.append_to_file {
                                    get_log_file().unwrap().write_all("\n".as_bytes()).unwrap();
                                }
                            }
                            Err(e) => return Err(format!("{e}")),
                        };

                        // Write date on 1st line
                        match get_log_file()
                            .unwrap()
                            .write_all(format!("Log start on {date}\n").as_bytes())
                        {
                            Ok(_) => (),
                            Err(e) => return Err(format!("{e}")),
                        }
                    }
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
                display_date: false,
                display_caller: false,
                locked: false,
                display_severity: Some(crate::LogSeverity::Info),
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

        if get_log_file().is_some() { return Err("LOG_FILE should be None".to_string()); };

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
        let result = RustLogConfig::new_config().configure();
        remove_file("log.txt").unwrap_or(());

        match result {
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

    #[test]
    fn clear_config_unlocked() -> Result<(), String> {
        // Set config to None
        unsafe {
            crate::data::LOG_CONFIG = None;
        }
        unsafe {
            crate::data::LOG_FILE = None;
        }

        // New config unlocked by default
        RustLogConfig::new_config().enable_terminal().configure()?;

        // Try to clear config
        RustLogConfig::clear_config();

        match get_log_config() {
            Some(_) => Err("Configuration should be cleared".to_string()),
            None => Ok(()),
        }
    }

    #[test]
    fn clear_config_locked() -> Result<(), String> {
        // Set config to None
        unsafe {
            crate::data::LOG_CONFIG = None;
        }
        unsafe {
            crate::data::LOG_FILE = None;
        }

        // New locked config
        RustLogConfig::new_config().enable_terminal().lock().configure()?;

        // Try to clear config
        RustLogConfig::clear_config();

        match get_log_config() {
            Some(_) => Ok(()),
            None => Err("Configuration should be locked".to_string()),
        }
    }
}
