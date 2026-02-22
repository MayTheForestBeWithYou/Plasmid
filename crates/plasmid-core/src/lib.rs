#![forbid(unsafe_code)]

pub mod config;
pub mod planner;

pub use config::loader;

#[cfg(any(test, feature = "mock"))]
pub use config::mock::MockConfigReader;
