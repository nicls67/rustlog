# RustLog

A logging library for Rust projects

## How it works ?

**RustLog** logs data from main application or from another library either on terminal or in a specific file.
It can be configured only once, meaning the first call of `configure_log` will define once for all how the logging will work.

To log some text, use `write`.
Calls of `write` method will never panick or return an error to avoid any perturbation in main program.

## Usage

### Configure logging

Implementation of `RustLogConfig` must be used to generate log configuration.

`RustLogConfig::new_config` will generate a default configuration will all outputs disabled. Then use `RustLogConfig` methods to configure logging and enable outputs.

`RustLogConfig::configure` will configure and start logging with the desired configuration. `RustLogConfig::configure` will return an `Err` variant if called twice, with no impact on the current logging configuration.

```rust
use rustlog::RustLogConfig;

// To enable logging on terminal
RustLogConfig::new_config().enable_terminal().configure().unwrap();
```

```rust
use rustlog::RustLogConfig;

// To enable logging on file
RustLogConfig::new_config().enable_file("log.txt", true).configure().unwrap();
```

### Configure severity

Message severity can be added to log message. Four severities exists :

* `VERB` : Verbose mode
* `INFO` : Information message
* `WARNING` : Warning message
* `ERROR` : Error message

Log configuration contains the minimal severity level which will be displayed. For example, if `INFO` is selected as minimal level, verbose messages will not be displayed, if `ERROR` is selected, only erorr messages will be displayed.

`display_severity` method shall be used to configure enable or disable severity display, and configure minimal displayed level.

By default, when calling `RustLogConfig::new_config`, logging is configured to display message severity before the log message itself, with `INFO` as minimal log display.

```rust
use rustlog::RustLogConfig;

// Disable severity display inside the message, but all messages will still be displayed
RustLogConfig::new_config().display_severity(None);
```

```rust
use rustlog::RustLogConfig;
use rustlog::LogSeverity;

// Enable severity display inside the message, only messages with severity higher than WARNING will be displayed
RustLogConfig::new_config().display_severity(Some(LogSeverity::Warning));
```

### Log text

Call `write_log` to generate a log entry. Format of log entry is _DATE-SEVERITY-CALLER-TEXT_, _DATE_, _SEVERITY_ and _CALLER_ might be added or not depending of the selected configuration.

```rust
use rustlog::write_log;
use rustlog::LogSeverity;

write_log(LogSeverity::Info, &"New log text".to_string(), &"From function".to_string());
```
