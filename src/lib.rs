pub mod command;
pub mod action;
pub mod file;
pub mod error;
pub mod status;
pub mod config;

pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;