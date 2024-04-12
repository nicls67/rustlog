//!
//! Integration tests for `rustlog` crate
//!

use std::fs::{self, remove_file};

use rustlog::{write_log, RustLogConfig};

#[test]
fn log_configuration() -> Result<(), String> {
    RustLogConfig::clear_config();

    // First call of config, shall return OK
    match RustLogConfig::new_config().enable_terminal().configure() {
        Ok(_) => (),
        Err(_) => return Err("First call of configure_log should return Ok".to_string()),
    };

    // Second call of config, shall return Err
    match RustLogConfig::new_config()
        .enable_file("log.txt", true)
        .configure()
    {
        Ok(_) => Err("Second call of configure_log should return Err".to_string()),
        Err(_) => Ok(()),
    }
}

#[test]
fn write_1() -> Result<(), String> {
    RustLogConfig::clear_config();

    // Create dummy file
    fs::write("log1.txt", "Dummy logging 1").unwrap();

    // Write some log
    RustLogConfig::new_config()
        .enable_file("log1.txt", false)
        .display_caller(true)
        .configure().unwrap();

    write_log("Hello world !".to_string(), "MyModule".to_string());

    // Get log file content
    let logfile = fs::read_to_string("log1.txt").unwrap();
    remove_file("log1.txt").unwrap_or(());

    // Compare content
    let mut lines: Vec<&str> = logfile.split("\n").collect();

    // Skip last line
    lines.pop().unwrap();
    match lines.pop().unwrap() {
        "MyModule - Hello world !" => Ok(()),
        s => Err(format!("Wrong log file content : {s}"))
    }
}

#[test]
fn write_2() -> Result<(), String> {
    RustLogConfig::clear_config();

    // Create dummy file
    fs::write("log2.txt", "Dummy logging 2").unwrap();

    // Write some log
    RustLogConfig::new_config()
        .enable_file("log2.txt", true)
        .display_caller(false)
        .configure().unwrap();

    write_log("Hello world !".to_string(), "MyModule".to_string());

    // Get log file content
    let logfile = fs::read_to_string("log2.txt").unwrap();
    remove_file("log2.txt").unwrap_or(());

    // Compare content
    let mut lines: Vec<&str> = logfile.split("\n").collect();

    // Skip last line
    lines.pop().unwrap();

    match lines.pop().unwrap() {
        "Hello world !" => (),
        s => return Err(format!("Wrong log file content on line 3 : {s}"))
    };
    // Skip 2nd line
    lines.pop().unwrap();

    match lines.pop().unwrap() {
        "Dummy logging 2" => Ok(()),
        s => Err(format!("Wrong log file content on line 1 : {s}"))
    }
}
