//!
//! Integration tests for `rustlog` crate
//!

use rustlog::RustLogConfig;



#[test]
fn log_configuration() -> Result<(), String> {

    // First call of config, shall return OK
    match RustLogConfig::new_config().enable_terminal().configure() {
        Ok(_) => (),
        Err(_) => return Err("First call of configure_log should return Ok".to_string()),
    };

    // Second call of config, shall return Err
    match RustLogConfig::new_config().enable_file("log.txt", true).configure() {
        Ok(_) => Err("Second call of configure_log should return Err".to_string()),
        Err(_) => Ok(()),
    }
}