# RustLog

A logging library for Rust projects

## How it works ?

**RustLog** logs data from main application or from another library either on terminal or in a specific file.
It can be configured only once, meaning the first call of `configure_log` will define once for all how the logging will work.

## Usage

### Configure logging

1. Define log method
2. Define log options
3. Call `configure_log`

```rust
use rustlog::{LogOptions, LogMethod, configure_log};


let method1 = LogMethod::ToTerminal;
let method2 = LogMethod::ToFile("log.txt".to_string(), true);

let options = LogOptions {
    add_date: true
};

configure_log(method1, options).unwrap();
```
