//! Settings for a small backup daemon.

pub mod config;
pub mod duration;

pub use config::Config;
pub use duration::parse_duration;
