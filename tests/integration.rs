//!
//! Integration tests for `rustlog` crate
//!

use std::fs::{self, remove_file};

use rustlog::{write_log, RustLogConfig};
use rusttests::{check_result, check_value, CheckType};
use serial_test::serial;

#[test]
#[serial]
fn log_configuration() -> Result<(), String> {
    RustLogConfig::clear_config();

    // First call of config, shall return OK
    check_result(
        (1, 1),
        RustLogConfig::new_config().enable_terminal().configure(),
        true,
    )?;

    // Second call of config, shall return Err
    check_result(
        (2, 1),
        RustLogConfig::new_config()
            .enable_file("log.txt", true)
            .configure(),
        false,
    )?;

    Ok(())
}

#[test]
#[serial]
fn write_1() -> Result<(), String> {
    RustLogConfig::clear_config();

    // Create dummy file
    fs::write("log1.txt", "Dummy logging 1").unwrap();

    // Write some log
    RustLogConfig::new_config()
        .enable_file("log1.txt", false)
        .display_caller(true)
        .configure()?;

    write_log(rustlog::LogSeverity::Info, "Hello world !", "MyModule");

    // Get log file content
    let l_logfile = fs::read_to_string("log1.txt").unwrap();
    remove_file("log1.txt").unwrap_or(());

    // Compare content
    let mut l_lines: Vec<&str> = l_logfile.split('\n').collect();

    // Skip last line
    l_lines.pop().unwrap();

    check_value(
        (1, 1),
        &l_lines.pop().unwrap().to_string(),
        &"INFO - MyModule - Hello world !".to_string(),
        CheckType::Equal,
    )?;

    Ok(())
}

#[test]
#[serial]
fn write_2() -> Result<(), String> {
    RustLogConfig::clear_config();

    // Create dummy file
    fs::write("log2.txt", "Dummy logging 2").unwrap();

    // Write some log
    RustLogConfig::new_config()
        .enable_file("log2.txt", true)
        .display_caller(false)
        .configure()?;

    write_log(rustlog::LogSeverity::Error, "Hello world !", "MyModule");

    // Get log file content
    let l_logfile = fs::read_to_string("log2.txt").unwrap();
    remove_file("log2.txt").unwrap_or(());

    // Compare content
    let mut l_lines: Vec<&str> = l_logfile.split('\n').collect();

    // Skip last line
    l_lines.pop().unwrap();

    check_value(
        (1, 1),
        &l_lines.pop().unwrap().to_string(),
        &"ERROR - Hello world !".to_string(),
        CheckType::Equal,
    )?;

    // Skip 2nd line
    l_lines.pop().unwrap();

    check_value(
        (2, 1),
        &l_lines.pop().unwrap().to_string(),
        &"Dummy logging 2".to_string(),
        CheckType::Equal,
    )?;

    Ok(())
}

#[test]
#[serial]
fn write_3() -> Result<(), String> {
    RustLogConfig::clear_config();

    // Create dummy file
    fs::write("log3.txt", "Dummy logging 3").unwrap();

    // Write some log
    RustLogConfig::new_config()
        .enable_file("log3.txt", true)
        .display_caller(false)
        .display_severity(None)
        .configure()?;

    write_log(rustlog::LogSeverity::Error, "Hello world !", "MyModule");

    // Get log file content
    let l_logfile = fs::read_to_string("log3.txt").unwrap();
    remove_file("log3.txt").unwrap_or(());

    // Compare content
    let mut l_lines: Vec<&str> = l_logfile.split('\n').collect();

    // Skip last line
    l_lines.pop().unwrap();

    check_value(
        (1, 1),
        &l_lines.pop().unwrap().to_string(),
        &"Hello world !".to_string(),
        CheckType::Equal,
    )?;
    // Skip 2nd line
    l_lines.pop().unwrap();

    check_value(
        (2, 1),
        &l_lines.pop().unwrap().to_string(),
        &"Dummy logging 3".to_string(),
        CheckType::Equal,
    )?;

    Ok(())
}

#[test]
#[serial]
fn write_4() -> Result<(), String> {
    RustLogConfig::clear_config();

    // Create dummy file
    fs::write("log4.txt", "Dummy logging 4").unwrap();

    // Write some log
    RustLogConfig::new_config()
        .enable_file("log4.txt", true)
        .display_caller(false)
        .display_severity(Some(rustlog::LogSeverity::Warning))
        .configure()?;

    write_log(rustlog::LogSeverity::Info, "Hello world !", "MyModule");
    write_log(
        rustlog::LogSeverity::Error,
        "Very bad mistake !",
        "MyModule",
    );

    // Get log file content
    let l_logfile = fs::read_to_string("log4.txt").unwrap();
    remove_file("log4.txt").unwrap_or(());

    // Compare content
    let mut l_lines: Vec<&str> = l_logfile.split('\n').collect();

    // Skip last line
    l_lines.pop().unwrap();

    check_value(
        (1, 1),
        &l_lines.pop().unwrap().to_string(),
        &"ERROR - Very bad mistake !".to_string(),
        CheckType::Equal,
    )?;
    // Skip 2nd line
    l_lines.pop().unwrap();

    check_value(
        (2, 1),
        &l_lines.pop().unwrap().to_string(),
        &"Dummy logging 4".to_string(),
        CheckType::Equal,
    )?;

    Ok(())
}
