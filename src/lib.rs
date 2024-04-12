
#[doc = include_str!("../README.md")]

mod data;
mod log_config;
mod log;

pub use log_config::RustLogConfig;
pub use log::write_log;