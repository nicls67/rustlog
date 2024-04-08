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
                    Err(e) => {
                        Err(format!("{e}"))
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;

    use crate::{configure_log, LogMethod, LogOptions};

    #[test]
    fn log_already_configured() {
        // Set config to Some
        unsafe { crate::data::LOG_OPTIONS = Some(LogOptions { add_date: true }) }

        let method = LogMethod::ToTerminal;
        let options = LogOptions { add_date: false };
        let result = configure_log(method, options);
        assert!(result.is_err());
    }

    #[test]
    fn log_not_configured_terminal() {
        // Set config to None
        unsafe { crate::data::LOG_OPTIONS = None }
        unsafe { crate::data::LOG_FILE = None }

        let method = LogMethod::ToTerminal;
        let options = LogOptions { add_date: false };
        let result = configure_log(method, options);
        assert!(result.is_ok());
        assert!(unsafe { crate::data::LOG_FILE.is_none() });
        assert!(unsafe { crate::data::LOG_OPTIONS.is_some() });
    }

    #[test]
    fn log_not_configured_file() {
        // Set config to None
        unsafe { crate::data::LOG_OPTIONS = None; }
        unsafe { crate::data::LOG_FILE = None; }

        remove_file("log.txt").unwrap_or(());

        let method = LogMethod::ToFile("log.txt".to_string(), false);
        let options = LogOptions { add_date: false };
        let result = configure_log(method, options);
        println!("{:?}", result);
        assert!(result.is_ok());
        assert!(unsafe { crate::data::LOG_FILE.is_some() });

        remove_file("log.txt").unwrap_or(());
    }

    #[test]
    fn log_not_configured_both() {
        // Set config to None
        unsafe { crate::data::LOG_OPTIONS = None; }
        unsafe { crate::data::LOG_FILE = None; }

        remove_file("log.txt").unwrap_or(());

        let method = LogMethod::Both("log.txt".to_string(), true);
        let options = LogOptions { add_date: false };
        let result = configure_log(method, options);
        println!("{:?}", result);
        assert!(result.is_ok());
        assert!(unsafe { crate::data::LOG_FILE.is_some() });

        remove_file("log.txt").unwrap_or(());
    }
}
