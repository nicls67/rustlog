//!
//! RustLog configuration module
//!

use std::fs::{self, File};

use chrono::Local;

use crate::data::is_log_configured;
use crate::{
    data::{
        clear_log_config_and_file, get_log_config, set_log_config, set_log_file, write_to_log_file,
    },
    LogSeverity,
};

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
    ///
    /// # Error handling
    ///
    /// This function does not return any error.
    ///
    /// # Panicking
    ///
    /// This function will never panic.
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
    /// This function does not panic.
    pub fn clear_config() {
        if let Some(l_config) = get_log_config() {
            if !l_config.locked {
                clear_log_config_and_file();
            }
        }
    }

    /// Locks the configuration to prevent further modifications.
    ///
    /// Once this method is called, the configuration cannot be cleared or modified.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `RustLogConfig` instance, allowing method chaining.
    ///
    /// # Error handling
    ///
    /// This function does not return any error.
    ///
    /// # Panicking
    ///
    /// This function will never panic.
    pub fn lock(&mut self) -> &mut RustLogConfig {
        self.locked = true;
        self
    }

    /// Enables logging to the terminal.
    ///
    /// This method sets the configuration to enable logging to the terminal.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `RustLogConfig` instance, allowing method chaining.
    ///
    /// # Error handling
    ///
    /// This function does not return any error.
    ///
    /// # Panicking
    ///
    /// This function will never panic.
    pub fn enable_terminal(&mut self) -> &mut RustLogConfig {
        if !is_log_configured() {
            self.log_to_terminal = true;
        }
        self
    }

    /// Disables logging to the terminal.
    ///
    /// This method sets the configuration to disable logging to the terminal.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `RustLogConfig` instance, allowing method chaining.
    ///
    /// # Error handling
    ///
    /// This function does not return any error.
    ///
    /// # Panicking
    ///
    /// This function will never panic.
    pub fn disable_terminal(&mut self) -> &mut RustLogConfig {
        if !is_log_configured() {
            self.log_to_terminal = false;
        }
        self
    }

    /// Enables logging to the specified file.
    ///
    /// This method sets the configuration to enable logging to the specified file.
    /// If `p_append` is `true`, new log entries will be added to the end of the file;
    /// otherwise, the file will be overwritten.
    ///
    /// # Parameters
    ///
    /// * `p_log_file` - A string slice that holds the name of the file to log to.
    /// * `p_append` - A boolean value indicating whether to append to the file or overwrite it.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `RustLogConfig` instance, allowing method chaining.
    ///
    /// # Error handling
    ///
    /// This function does not return any error.
    ///
    /// # Panicking
    ///
    /// This function will never panic.
    pub fn enable_file(&mut self, p_log_file: &'static str, p_append: bool) -> &mut RustLogConfig {
        if !is_log_configured() {
            self.log_to_file = Some(p_log_file);
            self.append_to_file = p_append;
        }
        self
    }

    /// Disables logging to the file.
    ///
    /// This method sets the configuration to disable logging to the file
    /// and stops appending new log messages.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `RustLogConfig` instance, allowing method chaining.
    ///
    /// # Error handling
    ///
    /// This function does not return any error.
    ///
    /// # Panicking
    ///
    /// This function will never panic.
    pub fn disable_file(&mut self) -> &mut RustLogConfig {
        if !is_log_configured() {
            self.log_to_file = None;
            self.append_to_file = false;
        }
        self
    }

    /// Enables date display for each log entry
    ///
    /// # Parameters
    ///
    /// * `p_disp_date` - A boolean indicating whether to display the date for each log entry.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `RustLogConfig` instance, allowing method chaining.
    ///
    /// # Error handling
    ///
    /// This function does not return any error.
    ///
    /// # Panicking
    ///
    /// This function will never panic.
    pub fn display_date(&mut self, p_disp_date: bool) -> &mut RustLogConfig {
        if !is_log_configured() {
            self.display_date = p_disp_date;
        }
        self
    }

    /// Enables caller display for each log entry
    ///
    /// # Parameters
    ///
    /// * `p_disp_caller` - A boolean indicating whether to display the caller for each log entry.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `RustLogConfig` instance, allowing method chaining.
    ///
    /// # Error handling
    ///
    /// This function does not return any error.
    ///
    /// # Panicking
    ///
    /// This function will never panic.
    pub fn display_caller(&mut self, p_disp_caller: bool) -> &mut RustLogConfig {
        if !is_log_configured() {
            self.display_caller = p_disp_caller;
        }
        self
    }

    /// Enables severity display for each log entry.
    ///
    /// # Parameters
    ///
    /// * `p_disp_severity` - `None` to disable severity display. `Some` to enable severity display, with
    ///   minimal displayed level given in variant.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `RustLogConfig` instance, allowing method chaining.
    ///
    /// # Error handling
    ///
    /// This function does not return any error.
    ///
    /// # Panicking
    ///
    /// This function will never panic.
    pub fn display_severity(&mut self, p_disp_severity: Option<LogSeverity>) -> &mut RustLogConfig {
        if !is_log_configured() {
            self.display_severity = p_disp_severity;
        }
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
    ///
    /// # Error handling
    ///
    /// Returns an error string if:
    /// * Logging is already configured.
    /// * All log destinations are disabled.
    /// * File creation/writing fails.
    ///
    /// # Panicking
    ///
    /// This function will never panic.
    pub fn configure(&self) -> Result<(), String> {
        // Logging is already configured, return Err
        if is_log_configured() {
            Err(String::from("Logging already configured"))
        } else {
            // At least one log destination must be selected
            if !self.log_to_terminal && self.log_to_file.is_none() {
                return Err("All log destinations are disabled".to_string());
            }

            // Save configuration
            set_log_config(Some(*self));

            // Create log file
            if let Some(l_log_file) = self.log_to_file {
                match File::options()
                    .create(true)
                    .write(true)
                    .append(self.append_to_file)
                    .open(l_log_file)
                {
                    Ok(l_f) => {
                        set_log_file(Some(l_f));

                        let l_date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

                        // Check if file is empty
                        match fs::read_to_string(l_log_file) {
                            Ok(l_s) => {
                                if !l_s.is_empty() && self.append_to_file {
                                    write_to_log_file("\n".as_bytes())?;
                                }
                            }
                            Err(l_e) => return Err(format!("{l_e}")),
                        };

                        // Write date on 1st line
                        write_to_log_file(format!("Log start on {l_date}\n").as_bytes())?;
                    }
                    Err(l_e) => return Err(format!("{l_e}")),
                };
            }

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        data::{clear_log_config_and_file, get_log_file, set_log_config},
        RustLogConfig,
    };
    use rusttests::{check_result, check_value, CheckType};
    use serial_test::serial;
    use std::fs::remove_file;

    fn force_clear_config() {
        clear_log_config_and_file();
    }

    #[test]
    #[serial]
    fn log_already_configured() -> Result<(), String> {
        force_clear_config();
        // Set config to Some
        set_log_config(Some(RustLogConfig {
            log_to_terminal: true,
            log_to_file: None,
            append_to_file: false,
            display_date: false,
            display_caller: false,
            locked: false,
            display_severity: Some(crate::LogSeverity::Info),
        }));

        check_result((1, 1), RustLogConfig::new_config().configure(), false)?;

        Ok(())
    }

    #[test]
    #[serial]
    fn log_already_configured_with_new_config() -> Result<(), String> {
        // Set config to None
        force_clear_config();

        let mut l_config = RustLogConfig::new_config();
        l_config.enable_terminal();
        l_config.configure()?;
        l_config.disable_terminal();

        check_value((1, 1), &l_config.log_to_terminal, &true, CheckType::Equal)?;

        Ok(())
    }

    #[test]
    #[serial]
    fn log_not_configured_terminal() -> Result<(), String> {
        // Set config to None
        force_clear_config();

        // Function shall return Ok
        // log file shall be None
        // Log options shall be Some
        RustLogConfig::new_config().enable_terminal().configure()?;

        if get_log_file().is_some() {
            return Err("LOG_FILE should be None".to_string());
        };

        match crate::data::get_log_config() {
            Some(_) => Ok(()),
            None => Err("LOG_OPTIONS should be Some".to_string()),
        }
    }

    #[test]
    #[serial]
    fn log_not_configured_file() -> Result<(), String> {
        // Set config to None
        force_clear_config();

        remove_file("log.txt").unwrap_or(());

        // Function shall return Ok
        // log file shall be Some
        // Log options shall be Some
        RustLogConfig::new_config()
            .disable_terminal()
            .enable_file("log.txt", true)
            .configure()?;

        remove_file("log.txt").unwrap_or(());

        match get_log_file().as_ref() {
            Some(_) => (),
            None => return Err("LOG_FILE should be Some".to_string()),
        };
        match crate::data::get_log_config() {
            Some(_) => Ok(()),
            None => Err("LOG_OPTIONS should be Some".to_string()),
        }
    }

    #[test]
    #[serial]
    fn log_not_configured_both() -> Result<(), String> {
        // Set config to None
        force_clear_config();

        remove_file("log.txt").unwrap_or(());

        // Function shall return Ok
        // log file shall be Some
        // Log options shall be Some
        RustLogConfig::new_config()
            .enable_terminal()
            .enable_file("log.txt", true)
            .configure()?;

        remove_file("log.txt").unwrap_or(());

        match get_log_file().as_ref() {
            Some(_) => (),
            None => return Err("LOG_FILE should be Some".to_string()),
        };

        match crate::data::get_log_config() {
            Some(_) => Ok(()),
            None => Err("LOG_OPTIONS should be Some".to_string()),
        }
    }

    #[test]
    #[serial]
    fn log_not_configured_all_disabled() -> Result<(), String> {
        // Set config to None
        force_clear_config();

        remove_file("log.txt").unwrap_or(());

        // Function shall return Err
        // log file shall be None
        // Log options shall be None
        let l_result = RustLogConfig::new_config().configure();
        remove_file("log.txt").unwrap_or(());

        match l_result {
            Ok(_) => Err("configure_log should return Err variant".to_string()),
            Err(_) => {
                // Verify that config was not saved before validation
                match crate::data::get_log_config() {
                    Some(_) => Err("LOG_CONFIG should be None after failed configure".to_string()),
                    None => Ok(()),
                }
            }
        }
    }

    #[test]
    #[serial]
    fn enable_terminal() -> Result<(), String> {
        force_clear_config();
        let mut l_binding = RustLogConfig::new_config();
        let l_config = l_binding.enable_terminal();

        match l_config.log_to_terminal {
            true => Ok(()),
            false => Err("log_to_terminal should be TRUE".to_string()),
        }
    }

    #[test]
    #[serial]
    fn enable_file() -> Result<(), String> {
        force_clear_config();
        let mut l_binding = RustLogConfig::new_config();
        let l_config = l_binding.enable_file("log.txt", true);

        match l_config.log_to_file {
            Some(_) => match l_config.append_to_file {
                true => Ok(()),
                false => Err("append_to_file should be TRUE".to_string()),
            },
            None => Err("log_to_file should be Some".to_string()),
        }
    }

    #[test]
    #[serial]
    fn clear_config_unlocked() -> Result<(), String> {
        // Set config to None
        force_clear_config();

        // New config unlocked by default
        RustLogConfig::new_config().enable_terminal().configure()?;

        // Try to clear config
        RustLogConfig::clear_config();

        match crate::data::get_log_config() {
            Some(_) => Err("Configuration should be cleared".to_string()),
            None => Ok(()),
        }
    }

    #[test]
    #[serial]
    fn clear_config_locked() -> Result<(), String> {
        // Set config to None
        force_clear_config();

        // New locked config
        RustLogConfig::new_config()
            .enable_terminal()
            .lock()
            .configure()?;

        // Try to clear config
        RustLogConfig::clear_config();

        match crate::data::get_log_config() {
            Some(_) => Ok(()),
            None => Err("Configuration should be locked".to_string()),
        }
    }
}
