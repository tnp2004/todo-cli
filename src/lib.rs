pub mod command;
pub mod action;
pub mod file;
pub mod error;
pub mod status;
pub mod config;
pub mod utils;

pub use error::Error;
pub use utils::time;

pub type Result<T> = std::result::Result<T, Error>;