# RustLog

A logging library for Rust projects

## How it works ?

**RustLog** logs data from main application or from another library either on terminal or in a specific file.
It can be configured only once, meaning the first call of `configure_log` will define once for all how the logging will work.

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
