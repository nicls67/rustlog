//!
//! Integration tests for `rustlog` crate
//!

use rustlog::{self, configure_log, LogMethod, LogOptions};

#[test]
fn log_configuration() -> Result<(), String> {

    let method = LogMethod::ToFile("log.txt".to_string(), false);
    let options = LogOptions { add_date: false };

    // First call of config, shall return OK
    match configure_log(method, options) {
        Ok(_) => (),
        Err(_) => return Err("First call of configure_log should return Ok".to_string()),
    };

    let method = LogMethod::ToFile("log.txt".to_string(), false);
    let options = LogOptions { add_date: false };

    // Second call of config, shall return Err
    match configure_log(method, options) {
        Ok(_) => Err("Second call of configure_log should return Err".to_string()),
        Err(_) => Ok(()),
    }
}